use clap::Clap;

mod extract_rsrc;
pub use extract_rsrc::*;

#[derive(Clap)]
pub enum Action {
    ExtractRsrc(ExtractRsrcOpts),
}

pub fn process_action(action: Action) -> anyhow::Result<()> {
    match action {
        Action::ExtractRsrc(opts) => extract_rsrc(opts),
    }
}
