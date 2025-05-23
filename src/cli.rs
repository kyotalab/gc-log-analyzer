use clap::{Arg, ArgAction, Command};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn generate_cli_app() -> Command {
    Command::new(APP_NAME)
        .version(APP_VERSION)
        .about("A CLI tool to analyze Java GC logs and visualize heap and pause times")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .value_name("FILE")
                .help("Specify the GC log file to analyze")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("plot")
                .long("plot")
                .short('p')
                .value_name("FILE")
                .help("PNG file output destination for drawing results")
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("mode")
                .long("mode")
                .short('m')
                .value_name("MODE")
                .help("Rendering mode: heap, pause, or combined")
                .required(false)
                .default_value("combined")
                .value_parser(["heap", "pause", "combined"])
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("csv")
                .long("csv")
                .value_name("CSV_PATH")
                .help("Specify the path of the CSV file to be output (no output if omitted)")
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("summary")
                .long("summary")
                .help("Output the number of GC events by type")
                .required(false)
                .action(ArgAction::SetTrue),
        )
}
