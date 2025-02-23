# Tickettracker

A desktop application for easy JIRA ticket tracking. The app displays a small window with a reactive face that changes based on the time since the last recorded ticket.

## Features

- Simple ticket entry with JIRA format validation (PROJ-123)
- Reactive face for visual feedback
- Automatic saving in Excel format
- Autostart support for all major operating systems
- Sound feedback on ticket submission
- Always-on-top window
- Drag & drop positioning

## System Requirements

### Rust Installation

#### Windows
1. Visit [rustup.rs](https://rustup.rs)
2. Download and run the installer
3. Restart PowerShell/CMD

#### Linux/macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Additional Dependencies

#### Debian/Ubuntu

```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config libx11-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

#### Fedora

```bash
sudo dnf group install "C Development Tools and Libraries"
sudo dnf install pkg-config libX11-devel libxcb-devel
```

#### macOS

```bash
xcode-select --install
```

#### Windows
- Visual Studio Build Tools with C++ support

## Installation

### Method 1: Package Installation (Recommended)

#### Debian/Ubuntu
```bash
# Download the .deb package
wget https://github.com/munnotubbel/ticket-tracker/releases/latest/download/ticket-tracker.deb

# Install the package
sudo dpkg -i ticket-tracker.deb
```

#### Fedora/RHEL
```bash
# Download the .rpm package
wget https://github.com/munnotubbel/ticket-tracker/releases/latest/download/ticket-tracker.rpm

# Install the package
sudo rpm -i ticket-tracker.rpm
```

#### macOS
```bash
# Using Homebrew
brew tap munnotubbel/ticket-tracker
brew install ticket-tracker
```

#### Windows
- Download and run the installer (ticket-tracker-setup.exe) from the releases page

After installation, you can:
- Start the app by typing `ticket-tracker` in your terminal
- View the manual with `man ticket-tracker` (Linux/macOS)
- Find it in your start menu (Windows)

### Method 2: Building from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/ticket-tracker.git
cd ticket-tracker
```

2. Build the app:

For Linux:
```bash
# Install required build dependencies
sudo dnf install make gcc rpm-build rpmdevtools   # Fedora/RHEL
# or
sudo apt install make gcc rpm build-essential     # Debian/Ubuntu

# Build Linux packages
make clean-linux
make build-linux
```

For Windows:
```bash
# Ensure you have the Windows GNU toolchain
rustup target add x86_64-pc-windows-gnu

make clean-windows
make build-windows
```

For macOS:
```bash
# Note: macOS builds can only be created on macOS systems
# due to Apple's framework requirements and code signing

# On macOS:
make clean-macos
make build-macos
```

# Alternative for macOS users:
1. Clone the repository
2. Install Rust and dependencies
3. Run: cargo build --release
```

The compiled binaries will be available in:
- Linux: `releases/linux/`
- Windows: `releases/windows/`
- macOS: Build locally or download from releases page

## Usage

### Commands

```bash
# Start the app
./target/release/ticket-tracker

# Show help
./target/release/ticket-tracker --help

# Install app for autostart
./target/release/ticket-tracker --install

# Remove app from autostart
./target/release/ticket-tracker --uninstall

# Quit all running instances
./target/release/ticket-tracker --quit
```

### Ticket Entry

1. Click in the input field
2. Enter the ticket number in PROJ-123 format
3. Press Enter to save

Tickets are stored in an Excel file in your Documents folder:
- Windows: `%USERPROFILE%\Documents\tickets.xlsx`
- Linux/macOS: `~/Documents/tickets.xlsx`

## Configuration

The app stores its configuration in:
- Windows: `%USERPROFILE%\Documents\tickets.config`
- Linux/macOS: `~/Documents/tickets.config`

## Project Structure
```tree
ticket-tracker/
├── src/
│   ├── main.rs          # Main program
│   └── ...
├── assets/              # Images and sounds
│   ├── face_*.png       # Face expressions
│   └── yeah.wav         # Success sound
├── Cargo.toml           # Project dependencies
├── ticket-tracker.1      # Man page
└── README.md           # This file
```

## Troubleshooting

### Known Issues

1. **App doesn't start**: 
   - Check if all system dependencies are installed
   - Check executable permissions

2. **No sound**:
   - Ensure your system audio is working
   - Check app audio permissions

### Logs

The app outputs errors to the console. Start it from a terminal/command prompt to see the output.

## License

[MIT](LICENSE)

## Author

Marcus Weissohn Eede - [@yourtwitter](https://twitter.com/munnotubbel) - marcus.weissohn@gmail.com

Project Link: [https://github.com/munnotubbel/ticket-tracker](https://github.com/munnotubbel/ticket-tracker)

