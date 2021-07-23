use clap::ArgMatches;

use crate::{command_follow::follow, config::Config};

pub struct Monitor {}

impl Monitor {
    pub async fn run(
        &self,
        args: &ArgMatches<'_>,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match args.subcommand() {
            ("list", args) => Ok(()),
            ("follow", args) => follow(args.unwrap(), config).await,
            _ => Ok(()),
        }
    }
}
