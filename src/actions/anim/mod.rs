use clap::Clap;

mod unpack;
pub use unpack::*;

#[derive(Clap)]
pub enum Command {
    #[clap(about = "Unpack the images into individual paletted gifs")]
    Unpack(UnpackOpts),
}

impl Command {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            Command::Unpack(opts) => unpack::unpack_anim(opts),
        }
    }
}
