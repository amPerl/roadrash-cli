use binrw::{BinRead, BinReaderExt, NullString};
use clap::Clap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Clap, Debug)]
pub struct ExtractRsrcOpts {
    #[clap(short = 'i', long, about = "input file")]
    input_path: String,
    #[clap(short = 'o', long, about = "output directory")]
    output_dir: String,
}

#[derive(BinRead, Debug)]
#[br(magic = b"CRSR")]
struct RsrcHeader {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub entries_offset: u32,
    pub data_offset: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
}

#[derive(BinRead, Debug)]
struct RsrcEntries {
    #[br(map = |x: [u8; 4]| String::from_utf8_lossy(&x).to_string().chars().rev().collect())]
    pub kind: String,
    unk1: u32,
    pub entry_count: u32,
    pub unk3: u32,
    #[br(count = entry_count)]
    pub sub_entries: Vec<RsrcEntry>,
}

#[derive(BinRead, Debug)]
struct RsrcEntry {
    #[br(map = |x: [u8; 4]| String::from_utf8_lossy(&x).to_string().chars().rev().collect())]
    pub kind: String,
    pub unk1: u32,
    pub data_offset: u32,
    pub data_size: u32,
    pub runtime_fields: (u32, u32, u32, u32),
}

pub fn extract_rsrc(opts: ExtractRsrcOpts) -> anyhow::Result<()> {
    let mut file = File::open(&opts.input_path)?;

    let rsrc_header = file.read_le::<RsrcHeader>()?;
    eprintln!("{:#?}", &rsrc_header);

    let rsrc_entries = file.read_le::<RsrcEntries>()?;
    eprintln!("{:?}", &rsrc_entries);

    for entry in rsrc_entries.sub_entries {
        eprintln!("{:?}", &entry);
    }

    Ok(())
}
