use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn get_arguments() -> ArgMatches<'static> {
    App::new("Log Server")
        .settings(&[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])
        .version("0.0.1")
        .about("An easy log server")
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .value_name("PATH")
                .help("Specifies config file path")
                .default_value(".loggerrc.toml")
                .takes_value(true),
        )
        .get_matches()
}
