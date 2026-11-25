

# infoFetch

A unified command-line utility for Linux system information and management.

`infoFetch` aims to bring common system inspection and hardware controls under one interface, reducing the need to learn and manage multiple independent utilities.

## Motivation

Linux provides excellent tools for managing individual parts of a system. However, these tools often have different interfaces, commands, and conventions.

For users who prefer minimal Linux environments, managing basic system functionality can mean dealing with several independent utilities.

infoFetch aims to simplify this experience by providing a compact, consistent command-line interface for interacting with different system domains.

## Command Interface

infoFetch follows a short-flag-based command structure designed to keep commands concise and easy to remember.

### Battery

```bash
# Display battery information
inf -b

# Set maximum battery charge limit
inf -b -sl 80%
```

### Storage

```bash
# Display storage information
inf -s
```

### Display

```bash
# Display brightness information
inf -d

# Set brightness
inf -d -sb 70%
```

### Audio

```bash
# Display audio information
inf -a

# Set volume
inf -a -sv 50%
```

*The command examples describe the intended interface. Functionality is subject to implementation.*

## Planned Features

### System Information
- [ ] Battery information
- [ ] Battery health
- [ ] Storage information
- [ ] General system information
- [ ] Display information
- [ ] Audio information

### System Controls
- [ ] Battery charge limit
- [ ] Display brightness
- [ ] Audio volume
- [ ] Power management

### Future Possibilities
- [ ] Network information and management
- [ ] Hardware monitoring
- [ ] Power profiles
- [ ] Additional Linux system controls

## Architecture

The project follows a domain-oriented architecture.

Each domain is responsible for its own command handling and system operations, while the root CLI coordinates the available domains.

```text
src/
├── main.rs
├── cli/
│   ├── mod.rs
│   └── cli.rs
└── domain/
    ├── mod.rs
    ├── battery/
    │   ├── mod.rs
    │   ├── cli.rs
    │   ├── info.rs
    │   └── control.rs
    ├── storage/
    │   ├── mod.rs
    │   ├── cli.rs
    │   └── info.rs
    ├── display/
    │   ├── mod.rs
    │   ├── cli.rs
    │   └── brightness.rs
    └── audio/
        ├── mod.rs
        ├── cli.rs
        └── volume.rs
```

The structure may evolve as the project grows.

## Design Goals

- **Unified interface:** Manage different system domains through one CLI.
- **Minimalism:** Keep commands short, predictable, and easy to remember.
- **Modularity:** Keep each domain independent and responsible for its own operations.
- **Linux integration:** Interact with existing Linux interfaces and system facilities.
- **Extensibility:** Make adding new system capabilities straightforward.

## Tech Stack

- Rust
- Linux system interfaces
- CLI argument parsing

## Platform

Currently focused on Linux.

Available functionality may vary depending on hardware, kernel, drivers, and system configuration.

## Project Status

Early development.

Development begins with system information and will gradually expand toward hardware controls and broader system management.

## License

To be decided.
