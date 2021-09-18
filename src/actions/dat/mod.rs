use clap::Clap;

mod unpack;
pub use unpack::*;

mod pack;
pub use pack::*;

#[derive(Clap)]
pub enum Command {
    #[clap(about = "Unpack the DAT into individual bmp files")]
    Unpack(UnpackOpts),
    #[clap(about = "Repack invididual bmp files into a DAT")]
    Pack(PackOpts),
}

impl Command {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            Command::Unpack(opts) => unpack::unpack_dat(opts),
            Command::Pack(opts) => pack::pack_dat(opts),
        }
    }
}
