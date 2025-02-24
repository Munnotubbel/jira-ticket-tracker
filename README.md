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

### Debian/Ubuntu
```bash
# Installation
sudo dpkg -i ticket-tracker.deb

# Update auf neue Version
sudo dpkg -i ticket-tracker-new.deb

# Deinstallation
sudo dpkg -r ticket-tracker
```

### Fedora/RHEL
```bash
# Installation
sudo rpm -i ticket-tracker.rpm

# Update auf neue Version
sudo rpm -U ticket-tracker-new.rpm

# Deinstallation
sudo rpm -e ticket-tracker
```

### Windows
```powershell
# Installation
ticket-tracker-setup.exe

# Update
# Einfach neue Version installieren, überschreibt automatisch

# Deinstallation
# Über Windows-Einstellungen > Apps > Apps & Features
# ODER
Control Panel > Programs > Uninstall a Program
```

### macOS
```bash
# Installation
# DMG öffnen und in Applications ziehen

# Update
# Neue Version in Applications ziehen, bestätigen dass alte überschrieben wird

# Deinstallation
# App aus Applications in den Papierkorb ziehen
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
├── releases/         # Build artifacts
│   ├── linux/        # Linux binaries
│   ├── windows/      # Windows binaries
│   ├── macos/        # macOS binaries
│   └── README.md     # Release instructions
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

