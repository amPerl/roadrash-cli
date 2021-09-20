use anyhow::Context;
use binrw::BinReaderExt;
use clap::Clap;
use std::fs::File;
use std::path::Path;

use crate::parsers::Fam;

#[derive(Clap, Debug)]
pub struct UnpackOpts {
    #[clap(short = 'i', long, about = "input file")]
    input_path: String,
    #[clap(short = 'o', long, about = "output file")]
    output_path: String,
}

pub fn unpack_fam(opts: UnpackOpts) -> anyhow::Result<()> {
    let input_file_path = Path::new(&opts.input_path);
    let mut file = File::open(input_file_path)?;

    let fam: Fam = file.read_le()?;
    let decompressed = explode::explode(&fam.compressed_data).context("Failed to explode fam")?;

    std::fs::write(&opts.output_path, &decompressed)
        .context("Failed to write decompressed fam to file")?;

    Ok(())
}
