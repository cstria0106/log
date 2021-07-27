use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn get_arguments() -> ArgMatches<'static> {
    App::new("Log Monitor")
        .settings(&[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])
        .version("0.0.1")
        .about("An easy log monitor tool")
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .value_name("PATH")
                .help("Specifies config file path")
                .default_value(".monitorrc.toml")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("follow")
                .setting(AppSettings::ColoredHelp)
                .about("Follow and print realtime logs"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .setting(AppSettings::ColoredHelp)
                .visible_alias("ls")
                .about("List logged dates or logs of certain date")
                .arg(
                    Arg::with_name("date")
                        .value_name("DATE")
                        .help("Specified date to query")
                        .takes_value(true)
                        .index(1),
                ),
        )
        .get_matches()
}
