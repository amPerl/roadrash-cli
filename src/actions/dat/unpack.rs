use binrw::BinReaderExt;
use clap::Clap;
use std::fs::File;
use std::path::Path;

use crate::parsers::{Dat, Palette};

#[derive(Clap, Debug)]
pub struct UnpackOpts {
    #[clap(short = 'i', long, about = "input file")]
    input_path: String,
    #[clap(short = 'p', long, about = "palette file")]
    palette_path: String,
    #[clap(short = 'o', long, about = "output directory")]
    output_dir: String,
}

pub fn unpack_dat(opts: UnpackOpts) -> anyhow::Result<()> {
    let palette = Palette::from_path(&opts.palette_path)?;

    let input_file_path = Path::new(&opts.input_path);
    let mut file = File::open(input_file_path)?;

    let dat: Dat = file.read_le()?;
    // eprintln!("Dat: {:?}", dat);

    std::fs::create_dir_all(&opts.output_dir)?;

    let base_output_filename = input_file_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('.')
        .next()
        .unwrap();

    let output_dir = Path::new(&opts.output_dir);

    for (i, texture) in dat.textures.into_iter().enumerate() {
        let path = output_dir.join(format!("{}_{:02}.bmp", base_output_filename, i));
        eprintln!(
            "Writing texture idx {} res {}x{} length {}",
            i,
            texture.width,
            texture.height,
            texture.palette_indices.len()
        );

        let mut image_buffer = image::ImageBuffer::new(texture.width as _, texture.height as _);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let pixel_index = y * texture.width as u32 + x;
            let pixel_palette_index = texture.palette_indices[pixel_index as usize];
            let palette_color = palette.colors[pixel_palette_index as usize];
            *pixel = image::Rgb([palette_color.0, palette_color.1, palette_color.2]);
        }

        image_buffer.save(&path)?;
    }

    Ok(())
}
