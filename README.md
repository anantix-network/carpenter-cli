# 🔧 Carpet CLI

Carpet CLI (Cargo Project Scripts) is an interactive script runner for your projects, inspired by `nps` (Node Project Scripts) but built for the Rust ecosystem.

## ✨ Features

- 🎯 **Interactive Menu**: Select and run scripts with an easy-to-use interface
- 📦 **Config-based**: Define your scripts in `carpet.toml` or `carpet.yaml`
- 🌍 **Environment Support**: Set environment variables per script
- 📂 **Working Directory**: Specify working directory for each script
- 🔄 **Multiple Formats**: Supports both TOML and YAML configuration
- 📝 **Script Descriptions**: Document your scripts with descriptions

## 🚀 Quick Start

### Installation

There are several ways to install Carpet CLI:

#### 1. Using Install Script (Recommended)

The easiest way to install is using our install script:

```bash
curl -fsSL https://raw.githubusercontent.com/anantix-network/carpenter-cli/refs/heads/production/install.sh | bash
```

This will:
- Clone the repository
- Build from source
- Install the binary to `/usr/local/bin/carpet`

#### 2. Manual Installation

If you prefer to install manually:

```bash
# Clone the repository
git clone https://github.com/anantix-network/carpenter-cli
cd carpet-cli

# Build and install
cargo install --path .
```

#### Prerequisites

- Git
- Rust and Cargo (Install from https://rustup.rs)

#### Verifying Installation

After installation, verify it works by running:

```bash
carpet --version
```

### Initialize Configuration

Create a new configuration file in your project:

```bash
# Create carpet.toml (default)
carpet init

# Or create carpet.yaml
carpet init --yaml
```

This will create a config file with some example scripts.

### Running Scripts

```bash
# Interactive mode - shows menu of available scripts
carpet run

# Direct mode - run a specific script
carpet run build
```

## ⚙️ Configuration

### TOML Format (carpet.toml)

```toml
version = "0.1.0"

[scripts.build]
command = "cargo build"
description = "Build the project"

[scripts.test]
command = "cargo test"
description = "Run tests"
env = { RUST_BACKTRACE = "1" }

[scripts.run]
command = "cargo run"
description = "Run the project"
working_dir = "target/debug"
```

### YAML Format (carpet.yaml)

```yaml
version: 0.1.0
scripts:
  build:
    command: cargo build
    description: Build the project
  
  test:
    command: cargo test
    description: Run tests
    env:
      RUST_BACKTRACE: "1"
  
  run:
    command: cargo run
    description: Run the project
    working_dir: target/debug
```

## 📖 Command Reference

### Global Options

- `-c, --config <PATH>`: Specify config file path
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Commands

#### `carpet init`
Initialize a new configuration file

Options:
- `-f, --force`: Force creation even if file exists
- `-y, --yaml`: Use YAML format instead of TOML

#### `carpet run [SCRIPT]`
Run a script from configuration

Arguments:
- `SCRIPT`: Optional script name to run directly

## 🧩 Script Configuration

Each script can have the following properties:

- `command`: The command to execute (required)
- `description`: A description of what the script does (optional)
- `env`: Environment variables for the script (optional)
- `working_dir`: Working directory to run the script in (optional)

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Submit a pull request

## 📝 License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

## 🔍 Project Structure

```
src/
├── commands/       # Command implementations
│   ├── init.rs    # Initialize config
│   ├── run.rs     # Run scripts
│   └── mod.rs     # Commands module
├── config/        # Configuration handling
│   ├── mod.rs     # Config structures
│   └── loader.rs  # Config loading
├── env/           # Environment handling
├── script/        # Script execution
├── utils/         # Utilities
└── main.rs        # Entry point
```

## 🙏 Acknowledgments

- Inspired by [nps](https://github.com/sezna/nps) for Node.js
- Built with [Rust](https://www.rust-lang.org/)

## Star History

<a href="https://www.star-history.com/#anantix-network/carpenter-cli&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=anantix-network/carpenter-cli&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=anantix-network/carpenter-cli&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=anantix-network/carpenter-cli&type=date&legend=top-left" />
 </picture>
</a>