use clap::ArgMatches;

use crate::config::Config;

pub struct Monitor {}

impl Monitor {
    pub async fn run(
        &self,
        args: &ArgMatches<'_>,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match args.subcommand() {
            ("list", args) => crate::command_list::list(args.unwrap(), config).await,
            ("follow", args) => crate::command_follow::follow(args.unwrap(), config).await,
            _ => Ok(()),
        }
    }
}
