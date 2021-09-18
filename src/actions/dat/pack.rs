use anyhow::Context;
use clap::Clap;
use image::GenericImageView;
use std::collections::HashMap;
use std::path::Path;
use std::{fs::File, io::Write};

use crate::parsers::{DatTexture, Palette};

#[derive(Clap, Debug)]
pub struct PackOpts {
    #[clap(short = 'i', long, about = "input directory")]
    input_dir: String,
    #[clap(short = 'p', long, about = "palette file")]
    palette_path: String,
    #[clap(short = 'o', long, about = "output file")]
    output_file: String,
}

pub fn pack_dat(opts: PackOpts) -> anyhow::Result<()> {
    let palette = Palette::from_path(&opts.palette_path)?;
    let palette_indices_by_color = palette
        .colors
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i as u8))
        .collect::<HashMap<(u8, u8, u8, u8), u8>>();

    let mut textures: Vec<DatTexture> = Vec::new();

    let input_dir_path = Path::new(&opts.input_dir);
    for dir_entry in std::fs::read_dir(input_dir_path)?.flatten() {
        if let Ok(img) = image::open(dir_entry.path()) {
            let mut palette_indices = Vec::new();
            let image_rgb8 = img.to_rgb8();
            for pixel in image_rgb8.pixels() {
                let color_tuple = (pixel.0[0], pixel.0[1], pixel.0[2], 0u8);
                let palette_index = palette_indices_by_color
                    .get(&color_tuple)
                    .context("Could not find color in palette")?;
                palette_indices.push(*palette_index);
            }

            textures.push(DatTexture {
                width: img.width() as u8,
                height: img.height() as u8,
                palette_indices,
            });
        } else {
            textures.push(DatTexture {
                width: 0u8,
                height: 0u8,
                palette_indices: Vec::new(),
            });
        }
    }

    let mut out_file = File::create(&opts.output_file)?;
    for texture in textures {
        out_file.write_all(&[texture.width])?;
        out_file.write_all(&[texture.height])?;
        out_file.write_all(&texture.palette_indices)?;
    }

    Ok(())
}
