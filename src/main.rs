use anyhow::{bail, Result};
use clap::Parser;
use iptvrs::{
    cli::{hydrate, list, play, search},
    settings::Settings,
};
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "iptvrs")]
pub struct Cli {
    #[clap(short = 'c')]
    config: Option<PathBuf>,

    #[clap(subcommand)]
    cmd: Cmd,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let settings = Settings::new(self.config)?;

        match self.cmd.run(settings).await {
            Ok(_) => Ok(()),
            Err(err) => {
                bail!("Error: {:?}", err)
            }
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum Cmd {
    Hydrate(hydrate::Cmd),
    Search(search::Cmd),
    Play(play::Cmd),
    List(list::Cmd),
}

impl Cmd {
    pub async fn run(self, settings: Settings) -> Result<()> {
        match self {
            Cmd::Hydrate(cmd) => cmd.run(&settings).await,
            Cmd::Search(cmd) => cmd.run(&settings).await,
            Cmd::Play(cmd) => cmd.run(&settings).await,
            Cmd::List(cmd) => cmd.run(&settings).await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await
}
