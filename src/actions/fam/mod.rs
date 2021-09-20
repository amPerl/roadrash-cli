use clap::Clap;

mod unpack;
pub use unpack::*;

#[derive(Clap)]
pub enum Command {
    #[clap(about = "Decompress the FAM")]
    Unpack(UnpackOpts),
}

impl Command {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            Command::Unpack(opts) => unpack::unpack_fam(opts),
        }
    }
}
