use anyhow::{bail, Result};
use clap::Parser;
use iptvrs::{
    cli::{hydrate, list, play, search},
    db::DB,
    settings::Settings,
};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    /// Hydrate the database with the contents of playlist
    Hydrate(hydrate::Cmd),
    /// Search the database for a channel
    Search(search::Cmd),
    /// Play selected channel
    Play(play::Cmd),
    /// List all channels
    List(list::Cmd),
}

impl Cmd {
    pub async fn run(self, settings: Settings) -> Result<()> {
        // setup tracing
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&settings.log))
            .with(tracing_logfmt::layer())
            .init();

        let db = DB::from_settings(&settings).await?;

        sqlx::migrate!("./migrations").run(&db.pool).await?;

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
