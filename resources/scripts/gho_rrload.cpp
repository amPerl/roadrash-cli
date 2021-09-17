// By Gho: https://sourceforge.net/p/dxwnd/discussion/general/thread/e080095150/?page=1
#define _CRT_SECURE_NO_WARNINGS

#include <windows.h>
#include <d3d.h>
#include <stdio.h>

static bool TextureLoad(FILE *dat, char *path, int index)
{
    int w, h, OffBits;
    BYTE bw, bh;
    char pszFile[MAX_PATH];
    char *sExt;
    size_t wlen;
    char *linebuf;
    char bmppath[MAX_PATH];

    /*
    if(!fread(&w, 1, 1, bitmap)) return 0;
    if(!fread(&h, 1, 1, bitmap)) return 0;
    if((w==0) || (h==0)) {
        printf("void pic at %#x\n", ftell(bitmap));
        return 1;
    }
    */

    sprintf(bmppath, "%s.%d.bmp", path, index);

    FILE *bmp = fopen(bmppath, "rb");
    if (!bmp)
        return 0;

    BITMAPFILEHEADER hdr; // bitmap file-header
    BITMAPV4HEADER pbi;   // bitmap info-header

    fread(&hdr, 1, sizeof(BITMAPFILEHEADER), bmp);
    fread(&pbi, 1, sizeof(BITMAPV4HEADER), bmp);
    w = pbi.bV4Width;
    h = pbi.bV4Height;
    if (h < 0)
        h = -h;
    //fseek(bmp, 14, SEEK_SET);
    //if(!fread(&h, 1, sizeof(DWORD), bmp)) return 0;
    //if(!fread(&w, 1, sizeof(DWORD), bmp)) return 0;
    //if(!fread(&OffBits, 1, sizeof(DWORD), bmp)) return 0;
    printf("processing image %d wxh=%dx%d file=%s\n", index, w, h, bmppath);
    int iScanLineSize = ((w * 8 + 0x1F) & ~0x1F) / 8;

    OffBits = 0x47A;
    bw = (BYTE)w;
    bh = (BYTE)h;
    fwrite(&bw, 1, 1, dat);
    fwrite(&bh, 1, 1, dat);

    if (w && h)
    {
        fseek(bmp, OffBits, SEEK_SET);
        linebuf = (char *)malloc(iScanLineSize);
        for (; h; h--)
        {
            fread(linebuf, 1, iScanLineSize, bmp);
            fwrite(linebuf, 1, w, dat);
        }
        free(linebuf);
    }

    fclose(bmp);
    return 1;
}

int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        printf("syntax: rrload <file>.DAT\n");
        exit(0);
    }
    char *dat = argv[1];
    char newdat[MAX_PATH];
    sprintf(newdat, "%s.new", dat);

    FILE *f = fopen(newdat, "wb");
    if (!f)
    {
        printf("error: can't open %s err=%d\n", argv[1], GetLastError());
        exit(0);
    }

    for (int index = 0;; index++)
    {
        if (!TextureLoad(f, dat, index))
            break;
    }
}