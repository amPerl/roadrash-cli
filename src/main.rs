use clap::Clap;

mod actions;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "amPerl")]
struct Opts {
    #[clap(subcommand, about = "action to perform")]
    action: actions::Action,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    actions::process_action(opts.action)
}
