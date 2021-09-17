use binrw::BinReaderExt;
use clap::Clap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::parsers::{Palette, RsrcAnimEntry, RsrcAnimEntryData};

#[derive(Clap, Debug)]
pub struct UnpackOpts {
    #[clap(short = 'i', long, about = "input file")]
    input_path: String,
    #[clap(short = 'p', long, about = "palette file")]
    palette_path: String,
    #[clap(short = 'o', long, about = "output directory")]
    output_dir: String,
}

pub fn unpack_anim(opts: UnpackOpts) -> anyhow::Result<()> {
    let palette = Palette::from_path(&opts.palette_path)?;

    let mut file = File::open(&opts.input_path)?;

    let mut next_entry_offset = 0;

    loop {
        let current_entry_offset = file.seek(SeekFrom::Start(next_entry_offset as u64))? as u32;

        if let Ok(entry) = file.read_le::<RsrcAnimEntry>() {
            next_entry_offset = current_entry_offset + entry.next_offset;
            let entry_kind_str = String::from_utf8_lossy(&entry.kind)
                .to_string()
                .chars()
                .rev()
                .collect::<String>();
            dbg!(&current_entry_offset, &entry_kind_str);

            match entry.data {
                RsrcAnimEntryData::Rran { three_more_offsets } => {
                    eprintln!("rran extra offsets: {:?}", three_more_offsets);
                }
                RsrcAnimEntryData::Rrfd { unk } => {
                    eprintln!("rrfd data: {:?}", unk);
                }
                RsrcAnimEntryData::Anim {
                    unk1,
                    rrpd_count,
                    unk2,
                } => {
                    eprintln!("anim data: {:?} {:?} {:?}", unk1, rrpd_count, unk2);
                }
                RsrcAnimEntryData::Plut { unk } => {
                    eprintln!("plut data: {:?}", unk);
                }
                RsrcAnimEntryData::Rrpd { unk, data } => {
                    eprintln!("rrpd data: {:?} {:#?}", unk, data);
                    if let Some(data) = data {
                        let mip_offset_sub = data.mip_count * 12 + 14 * 4;
                        dbg!(&entry.next_offset, mip_offset_sub);

                        let to_height_mul = data.mip1_height as f32 / data.mip1_width as f32;

                        // let mip_data_sizes = Vec::new();
                        for (mip_size, _wat, mip_offset) in data.mip_headers {
                            let mip_width = mip_size + 1;
                            let mip_height = (mip_width as f32 * to_height_mul).ceil() as u32;
                            let mip_size = mip_width * mip_height;
                            let mip_data_begin = current_entry_offset + 36 + mip_offset;
                            let mut mip_buffer = vec![0; mip_size as usize];
                            file.seek(SeekFrom::Start(mip_data_begin as u64))?;
                            file.read_exact(&mut mip_buffer)?;

                            std::fs::create_dir_all(&opts.output_dir)?;

                            let image_path = Path::new(&opts.output_dir)
                                .join(format!("{}_{}.bmp", &data.name, mip_width));

                            let mut image_buffer = image::ImageBuffer::new(mip_width, mip_height);

                            for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
                                let mip_buffer_index = y * mip_width + x;
                                let palette_idx = mip_buffer[mip_buffer_index as usize];
                                let palette_color = palette.colors[palette_idx as usize];
                                *pixel =
                                    image::Rgb([palette_color.0, palette_color.1, palette_color.2]);
                            }

                            image_buffer.save(&image_path)?;
                        }
                    }
                }
                _ => {}
            };
        } else {
            break;
        }
    }

    Ok(())
}
