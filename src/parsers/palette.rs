use anyhow::Context;
use binrw::{BinRead, BinReaderExt};
use std::{fs::File, path::Path};

#[derive(BinRead, Debug)]
pub struct Palette {
    #[br(count = 256)]
    pub colors: Vec<(u8, u8, u8, u8)>,
}

impl Palette {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut palette_file = File::open(path).context("Failed to open palette file")?;
        palette_file
            .read_le::<Palette>()
            .context("Failed to parse palette")
    }
}
