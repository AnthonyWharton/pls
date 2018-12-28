use std::process::Command;

use term::{color, stdout, Attr, StdoutTerminal};

use crate::options::Options;

mod options;

/// Alias for a TerminalOutput used for the coloured pls outputs.
type TerminalOutput = Box<StdoutTerminal>;

/// Program Main, Contains the main pls loop.
fn main() {
    let options = Options::new();
    let mut t = stdout().expect("Unable to open terminal");

    loop {
        let output = Command::new(&options.command).args(&options.args).output();
        match output {
            Ok(output) => {
                let success = output.status.success();

                if !options.silent {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }

                if success != options.loop_on_success {
                    break;
                }

                no_pls(&options, &mut t).unwrap()
            }
            Err(_) => exit(1, "Command completely failed to run.", t),
        }
    }
    ok(&options, &mut t).unwrap()
}

/// Prints an _"ok"_ message to the terminal if not suppressed by the options.
fn ok(options: &Options, t: &mut TerminalOutput) -> term::Result<()> {
    if !options.quiet {
        println!();

        if t.supports_reset() {
            if t.supports_attr(Attr::Bold) {
                t.attr(Attr::Bold)?;
            }
            if t.supports_color() {
                t.fg(color::GREEN)?;
            }
        }
        writeln!(t, "ok")?;

        if t.supports_reset() {
            t.reset()?;
        }
    }
    Ok(())
}

/// Prints a _"no, pls"_ message to the terminal if not suppressed by the
/// options.
fn no_pls(options: &Options, t: &mut TerminalOutput) -> term::Result<()> {
    println!();
    if !options.quiet {
        if t.supports_reset() {
            if t.supports_attr(Attr::Blink) {
                t.attr(Attr::Blink)?;
            }
            if t.supports_attr(Attr::Bold) {
                t.attr(Attr::Bold)?;
            }
            if t.supports_color() {
                t.fg(color::RED)?;
            }
        }
        writeln!(t, "no, pls")?;

        if t.supports_reset() {
            t.reset()?;
        }
        println!();
    }
    Ok(())
}

/// Prints the given message and then exits the program with the given status
/// code.
#[allow(unused_must_use)]
pub fn exit(code: i32, message: &'static str, mut t: TerminalOutput) -> ! {
    if code != 0 {
        if t.supports_reset() {
            t.attr(Attr::Bold);
            if t.supports_color() {
                t.fg(color::RED);
            }
        }
    }

    if message != "" {
        writeln!(t, "{}", message);
    }

    if t.supports_reset() {
        t.reset();
    }

    std::process::exit(code)
}
