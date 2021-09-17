use binrw::BinReaderExt;
use clap::Clap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::parsers::{RsrcEntries, RsrcHeader};

#[derive(Clap, Debug)]
pub struct UnpackOpts {
    #[clap(short = 'i', long, about = "input file")]
    input_path: String,
    #[clap(short = 'o', long, about = "output directory")]
    output_dir: String,
}

pub fn unpack_rsrc(opts: UnpackOpts) -> anyhow::Result<()> {
    let mut file = File::open(&opts.input_path)?;

    let _rsrc_header = file.read_le::<RsrcHeader>()?;
    // eprintln!("{:#?}", &rsrc_header);

    let rsrc_entries = file.read_le::<RsrcEntries>()?;
    eprintln!("{:?}", &rsrc_entries);

    for entry in rsrc_entries.sub_entries {
        eprintln!("{:?}", &entry);

        let mut file = File::open(&opts.input_path)?;

        file.seek(SeekFrom::Start(entry.data_offset as u64))?;
        let mut data = vec![0; entry.data_size as usize];
        file.read_exact(&mut data)?;

        std::fs::create_dir_all(&opts.output_dir)?;

        let mut output_file = File::create(
            Path::new(&opts.output_dir).join(format!("{}.{}", entry.index, &entry.kind)),
        )?;
        output_file.write_all(&data)?;
    }

    Ok(())
}
