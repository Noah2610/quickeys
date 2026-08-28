# QuicKeys

A simple command-shortcut CLI app. Better version of and based on [ShortStrokes].
Configure keys mapped to commands. Run a command for a given key.

## Installation
### Install from [crates.io]
Download, build, and install with `cargo` from [crates.io]:
```bash
cargo install quickeys
```

### Install from source
Build and install from source with `cargo`:
```bash
cargo install --path ./path/to/quickeys
```

## Configuration
Create a configuration YAML file at `~/.config/quickeys/config.yml` (or platform equivalent, see [`dirs::config_dir`]).
You can get started with the example [`config.yml`] from the repository.  

```bash
mkdir -p ~/.config/quickeys &&
curl -Lo ~/.config/quickeys/config.yml https://raw.githubusercontent.com/Noah2610/quickeys/refs/heads/main/config.yml
```

Here's a quick overview of the config structure:
```yaml
# Most command-line arguments can be set here. CLI arguments have precedence.
# See `quickeys --help` for details on each config option listed here.
# Note that the special character "~" is expanded to the user's home directory.
config:
    shell:      sh
    background: false
    #stdout:     ~/.local/share/quickeys/stdout.log
    #stderr:     ~/.local/share/quickeys/stderr.log

# Constants are variables that can be used in keybindings' commands.
# Before running a command, all constants are replaced with their values.
# Reference a constant with a leading "@" (ex. "@browser") and optionally
# wrap the identifier name in "{}" (ex. "@{browser}").
constants:
    browser:  firefox
    terminal: alacritty

# Define your key/command mappings here.
# Keys are the literal strings you type to trigger the command.
# Any constants referenced in the command string are resolved
# to their respective values before running.
# Optionally you may set run arguments (cli options) for specific commands
# by specifying the command value as a list of 2 string values:
#   - cli options string
#   - actual command string
keybindings:
    ddg:  '@browser "https://start.duckduckgo.com"'
    # Run terminal command in background process with `--bg` flag,
    # overwriting the default config of `background: false`.
    term: ['--bg', '@terminal']
```

## Usage
```bash
# List and describe all command-line options and arguments.
quickeys --help

# Run an interactive prompt (default), running the command for the typed key,
# as soon as a valid key is typed (no Enter needed).
quickeys prompt
quickeys

# Run a single command for the given key.
quickeys run ddg

# Run a single command in a new background process,
# detached from the current terminal.
quickeys run term -b

# List all configured keybindings and their resolved command strings.
quickeys list
quickeys list --help # for more options
```

## License
Distributed under the MIT license. See [`./LICENSE`].

[ShortStrokes]: https://github.com/Noah2610/ShortStrokes
[crates.io]: https://crates.io/crates/quickeys
[`dirs::config_dir`]: https://docs.rs/dirs/6.0.0/dirs/fn.config_dir.html
[`config.yml`]: ./config.yml
[`./LICENSE`]: ./LICENSE
