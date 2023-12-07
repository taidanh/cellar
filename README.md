![cellar_banner](https://github.com/taidanh/cellar/assets/65222208/6224f5df-1c1d-4ce7-9ce7-6ba1aac91be0)
Cellar is a simple script manager for Unix based systems.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/taidanh/cellar
   ```
2. Build:
   ```bash
   cargo build --release
   ```
3. Initialize:
   ```bash
   target/release/cellar init
   ```

## Usage
```bash
target/debug/cellar --help
```

```
Usage: cellar [PATH] <COMMAND>

Commands:
  add     Adds a new executable
  run     Runs a specified command
  remove  Removes an existing command
  init    Initializes the configuration directory
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  [default: ]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
