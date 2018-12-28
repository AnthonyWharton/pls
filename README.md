# pls - 0.1.0

A command line utility that repeatedly runs a command until some criteria are met.

## Build Instructions

1) Ensure you have Rust, you can install everything you need very easily [from here](https://rustup.rs/).
2) Clone the repository.
3) From the root directory in the repository run `cargo build --release`
4) You're done! The resulting executable will be located at `./target/release/pls`.

## Flags

The flags you can use to operate `pls` are documented, and can be found by typing `pls --help`, which will give the following output:

```
pls 0.1.0
Anthony W. <a.wharton.2015@bristol.ac.uk>
Repeatedly runs a command until some criteria are met. The default criteria is
if the command exits with a non-zero status code.

USAGE:
    pls [FLAGS] [OPTIONS] <COMMAND>...

FLAGS:
    -h, --help
            Prints help information

    -n, --negate
            Repeats the command if the exit status was 0 (successful).

    -q, --quiet
            Suppresses the pls messages.

    -s, --silent
            Suppresses the output of the command.

    -V, --version
            Prints version information


OPTIONS:
        --gen-completions <SHELL>
            Generates autocompletion file for specified terminal. [possible
            values: bash, fish, zsh, powershell,
            elvish]

ARGS:
    <COMMAND>...
            The command to be run until specified criteria are met.


Insanity is doing the same thing over and over again and expecting different
results.
```

## TODO

- [ ] Create installation scripts to install `pls` to system.
- [ ] Add option for adding a delay between execution(s).
- [ ] Add option for custom status exit criteria.
- [ ] This is a work in progress, think of anything else? Let me know!

## License

This project is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.
