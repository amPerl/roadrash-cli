use clap::Clap;

mod anim;
mod rsrc;

#[derive(Clap)]
pub enum FileTypeCommand {
    #[clap(about = "Actions for RSRC-type resource files (CRS, CAR, RSC)")]
    Rsrc {
        #[clap(subcommand, about = "subcommand to run")]
        cmd: rsrc::Command,
    },
    #[clap(about = "Actions for ANIM files (typically extracted from a RSRC)")]
    Anim {
        #[clap(subcommand, about = "subcommand to run")]
        cmd: anim::Command,
    },
}

impl FileTypeCommand {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            FileTypeCommand::Rsrc { cmd } => cmd.process(),
            FileTypeCommand::Anim { cmd } => cmd.process(),
        }
    }
}
