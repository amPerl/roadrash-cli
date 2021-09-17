// By Gho: https://sourceforge.net/p/dxwnd/discussion/general/thread/e080095150/?page=1
#define _CRT_SECURE_NO_WARNINGS

#include <windows.h>
#include <d3d.h>
#include <stdio.h>

static bool TextureDump(FILE *bitmap, BYTE *palette, char *path, int index)
{
	BYTE w, h;
	char pszFile[MAX_PATH];
	char *sExt;
	size_t wlen;
	char *linebuf;
	char bmppath[MAX_PATH];

#if 0
	while(true){
		if(!fread(&w, 1, 1, bitmap)) return 0;
		if(!fread(&h, 1, 1, bitmap)) return 0;
		if(w && h) break;
		else {
			printf("void size at %#x\n", ftell(bitmap));
		}
	}
#else
	if (!fread(&w, 1, 1, bitmap))
		return 0;
	if (!fread(&h, 1, 1, bitmap))
		return 0;
#endif
	sprintf(bmppath, "%s.%d.bmp", path, index);
	printf("processing image %d@%#x wxh=%dx%d file=%s\n", index, ftell(bitmap), w, h, bmppath);

	FILE *bmp = fopen(bmppath, "wb");
	if (!bmp)
		return 0;

	BITMAPFILEHEADER hdr; // bitmap file-header
	BITMAPV4HEADER pbi;	  // bitmap info-header

	memset((void *)&pbi, 0, sizeof(BITMAPV4HEADER));
	pbi.bV4Size = sizeof(BITMAPV4HEADER);
	pbi.bV4Width = w;
	pbi.bV4Height = h;
	pbi.bV4BitCount = 8;
	pbi.bV4SizeImage = ((pbi.bV4Width * pbi.bV4BitCount + 0x1F) & ~0x1F) / 8 * pbi.bV4Height;
	pbi.bV4Height = -pbi.bV4Height;
	pbi.bV4Planes = 1;
	pbi.bV4V4Compression = BI_RGB;
	pbi.bV4XPelsPerMeter = 1;
	pbi.bV4YPelsPerMeter = 1;
	pbi.bV4ClrUsed = 0;
	pbi.bV4ClrUsed = 256;
	pbi.bV4ClrImportant = 0;
	pbi.bV4RedMask = 0;
	pbi.bV4GreenMask = 0;
	pbi.bV4BlueMask = 0;
	pbi.bV4AlphaMask = 0;
	pbi.bV4CSType = LCS_CALIBRATED_RGB;
	int iScanLineSize = ((pbi.bV4Width * pbi.bV4BitCount + 0x1F) & ~0x1F) / 8;

	hdr.bfType = 0x4d42; // 0x42 = "B" 0x4d = "M"
	// Compute the size of the entire file.
	hdr.bfSize = (DWORD)(sizeof(BITMAPFILEHEADER) + pbi.bV4Size + pbi.bV4ClrUsed * sizeof(RGBQUAD) + pbi.bV4SizeImage);
	hdr.bfReserved1 = 0;
	hdr.bfReserved2 = 0;

	// Compute the offset to the array of color indices.
	hdr.bfOffBits = (DWORD)sizeof(BITMAPFILEHEADER) + pbi.bV4Size + pbi.bV4ClrUsed * sizeof(RGBQUAD);

	// Copy the BITMAPFILEHEADER into the .BMP file.
	fwrite((LPVOID)&hdr, sizeof(BITMAPFILEHEADER), 1, bmp);

	// Copy the BITMAPINFOHEADER array into the file.
	fwrite((LPVOID)&pbi, sizeof(BITMAPV4HEADER), 1, bmp);

	// Copy the RGBQUAD array into the file.
	if (w && h)
		fwrite((LPVOID)palette, 1024, 1, bmp);

	linebuf = (char *)malloc(w);
	for (; h; h--)
	{
		fread(linebuf, 1, w, bitmap);
		fwrite(linebuf, 1, iScanLineSize, bmp);
	}
	free(linebuf);
	return 1;
}

void swap(BYTE *p, int i1, int i2)
{
	BYTE swap;
	swap = *(p + i1);
	*(p + i1) = *(p + i2);
	*(p + i2) = swap;
}

int main(int argc, char *argv[])
{
	BYTE palette[1024];
	if (argc < 2)
	{
		printf("syntax: rrdump <file>.DAT [-grayscale]\n");
		exit(0);
	}
	char *dat = argv[1];
	if ((argc > 2) && !strcmp(argv[2], "-grayscale"))
	{
		for (int i = 0; i < 256; i++)
		{
			BYTE *palentry = &palette[4 * i];
			palentry[0] = (BYTE)i;
			palentry[1] = (BYTE)i;
			palentry[2] = (BYTE)i;
			palentry[3] = (BYTE)i;
		}
	}
	else
	{
		FILE *p = fopen("PALETTE.RAW", "rb");
		if (!p)
		{
			printf("error: can't open PALETTE.RAW err=%d\n", GetLastError());
			exit(0);
		}
		if (fread(palette, 1, 1024, p) != 1024)
		{
			printf("error: can't read palette data err=%d\n", GetLastError());
			exit(0);
		}
		for (int i = 0; i < 256; i++)
		{
			BYTE *palentry = &palette[4 * i];
			swap(palentry, 0, 2);
			//swap(palentry, 1, 3);
			//swap(palentry, 2, 3);
		}
	}
	FILE *f = fopen(dat, "rb");
	if (!f)
	{
		printf("error: can't open %s err=%d\n", argv[1], GetLastError());
		exit(0);
	}
	for (int index = 0;; index++)
	{
		if (!TextureDump(f, palette, dat, index))
			break;
	}
}
