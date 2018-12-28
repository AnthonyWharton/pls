use std::io::stdout;

use clap::{App, AppSettings, Arg, Shell};

/// Struct to encapsulate the command line options specified.
#[derive(Debug, Default)]
pub struct Options {
    /// The command to be run.
    pub command: String,
    /// The arguments for the command to be run.
    pub args: Vec<String>,
    /// Whether or not pls outputs should be suppressed.
    pub quiet: bool,
    /// Whether or not the command output should be suppressed.
    pub silent: bool,
    /// Whether pls should loop on a success exit status code from the command,
    /// or a non-successful exit status code, i.e. zero or non-zero status
    /// code respectively.
    pub loop_on_success: bool,
}

impl Options {
    /// Generates a new Options object, using the command line arguments
    /// from execution.
    pub fn new() -> Options {
        let matches = build_cli().get_matches();
        let mut options = Options::default();

        if let Some(c) = matches.value_of("gen-completions") {
            let mut cli = build_cli();
            match c.to_lowercase().as_str() {
                "bash" => cli.gen_completions_to("pls", Shell::Bash, &mut stdout()),
                "fish" => cli.gen_completions_to("pls", Shell::Fish, &mut stdout()),
                "zsh" => cli.gen_completions_to("pls", Shell::Zsh, &mut stdout()),
                "powershell" => cli.gen_completions_to("pls", Shell::PowerShell, &mut stdout()),
                "elvish" => cli.gen_completions_to("pls", Shell::Elvish, &mut stdout()),
                _ => (),
            }
            std::process::exit(0);
        }

        options.quiet = matches.is_present("quiet");
        options.silent = matches.is_present("silent");
        options.loop_on_success = matches.is_present("negate");
        if let Some(ms) = matches.values_of("command") {
            let mut command: Vec<String> = ms.map(|s| String::from(s)).collect();
            options.command = command.remove(0);
            options.args = command;
        }
        options
    }
}

/// Builds a new clap command line argument parser.
pub fn build_cli() -> App<'static, 'static> {
    App::new("pls")
            .version("0.1.0")
            .author("Anthony W. <a.wharton.2015@bristol.ac.uk>")
            .about(
                "Repeatedly runs a command until some criteria are met. The default criteria is if the command exits with a non-zero status code.",
            )
            .after_help(
                "Insanity is doing the same thing over and over again and expecting different results.",
            )
            .set_term_width(80)
            .max_term_width(80)
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::TrailingVarArg)
            .arg(
                Arg::with_name("quiet")
                    .short("q")
                    .long("quiet")
                    .help("Suppresses the pls messages."),
            )
            .arg(
                Arg::with_name("silent")
                    .short("s")
                    .long("silent")
                    .help("Suppresses the output of the command."),
            )
            .arg(
                Arg::with_name("negate")
                    .short("n")
                    .long("negate")
                    .help("Repeats the command if the exit status was 0 (successful)."),
            )
            .arg(
                Arg::with_name("gen-completions")
                    .long("gen-completions")
                    .help("Generates autocompletion file for specified terminal.")
                    .takes_value(true)
                    .value_name("SHELL")
                    .multiple(false)
                    .possible_values(&["bash", "fish", "zsh", "powershell", "elvish"])
                    .case_insensitive(true)
                    .hidden_short_help(true)
            )
            .arg(
                Arg::with_name("command")
                    .help("The command to be run until specified criteria are met.")
                    .takes_value(true)
                    .value_name("COMMAND")
                    .multiple(true)
                    .use_delimiter(false)
                    .required_unless("gen-completions")
            )
}
