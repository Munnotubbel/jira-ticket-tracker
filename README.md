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

```bash
# Windows: Visit https://rustup.rs and run the installer
# Linux/macOS:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Additional Dependencies

```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install build-essential pkg-config libx11-dev libxcb-render0-dev \
  libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev

# Fedora
sudo dnf group install "C Development Tools and Libraries"
sudo dnf install pkg-config libX11-devel libxcb-devel alsa-lib-devel

# macOS
xcode-select --install

# Windows
# Install Visual Studio Build Tools with C++ support
```

## Installation & Updates

### Pre-built Binaries

Download the latest release from our [GitHub Releases page](https://github.com/munnotubbel/jira-ticket-tracker/releases).

#### Debian/Ubuntu
```bash
# Install
sudo dpkg -i ticket-tracker_[version].deb

# Update
sudo dpkg -i ticket-tracker_[new-version].deb

# Uninstall
sudo dpkg -r ticket-tracker
```

#### Fedora/RHEL
```bash
# Install
sudo rpm -i ticket-tracker-[version].rpm

# Update
sudo rpm -U ticket-tracker-[new-version].rpm

# Uninstall
sudo rpm -e ticket-tracker
```

#### Windows
```powershell
# Install/Update
# Run the installer from GitHub Releases
ticket-tracker-setup.exe

# Uninstall
# Via Windows Settings > Apps > Apps & Features
# OR
Control Panel > Programs > Uninstall a Program
```

#### macOS
```bash
# Install
# Download .dmg, open and drag to Applications

# Update
# Drag new version to Applications, confirm overwrite

# Uninstall
# Drag from Applications to Trash
```

### Building from Source

1. Install Rust and dependencies (see System Requirements)

2. Clone and build:
```bash
# Clone repository
git clone https://github.com/munnotubbel/jira-ticket-tracker.git
cd jira-ticket-tracker

# Build for your system
make build-linux    # For Linux
make build-windows  # For Windows
make build-macos    # For macOS

# Or build for all platforms
make build-all
```

3. Install (Linux/macOS):
```bash
# Install system-wide (requires sudo)
sudo make install

# Install to custom location
make install PREFIX=~/.local

# Uninstall
sudo make uninstall
```

## Usage

```bash
# Start the app
ticket-tracker

# Install autostart
ticket-tracker --install

# Remove from autostart
ticket-tracker --uninstall

# Quit all instances
ticket-tracker --quit
```

### Ticket Entry
1. Click in the input field
2. Enter ticket number (PROJ-123)
3. Press Enter to save

### File Locations

```bash
# Tickets Excel file
Windows: %USERPROFILE%\Documents\tickets.xlsx
Linux/macOS: ~/Documents/tickets.xlsx

# Configuration
Windows: %USERPROFILE%\Documents\tickets.config
Linux/macOS: ~/Documents/tickets.config
```

## Project Structure
```tree 
ticket-tracker/
├── src/              # Source code
├── assets/           # Resources
│   ├── face_*.png    # Face expressions
│   └── yeah.wav      # Success sound
├── .github/          # GitHub Actions workflows
├── Cargo.toml        # Dependencies
└── README.md         # Documentation
```

## Troubleshooting

### Common Issues
- **App doesn't start**: Check dependencies and permissions
- **No sound**: Verify system audio and app permissions
- **Build fails**: Ensure all development dependencies are installed

### Debug Output
Run from terminal to see error messages: `ticket-tracker --debug`

## License

[MIT](LICENSE)

## Author

Marcus Weissohn Eede - [@munnotubbel](https://twitter.com/munnotubbel) - marcus.weissohn@gmail.com

Project Link: [https://github.com/munnotubbel/jira-ticket-tracker](https://github.com/munnotubbel/jira-ticket-tracker)

