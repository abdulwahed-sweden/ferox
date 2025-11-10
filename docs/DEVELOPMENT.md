# 🦊 Ferox Framework - Development Guide

## Quick Start

### Build and Run
```bash
# Build the project
cargo build --release

# Run Ferox
./target/release/ferox

# Or in development mode
cargo run
```

### Using Ferox CLI

```bash
# Start Ferox
ferox

# List available modules
ferox> modules

# Use port scanner
ferox> use scanner/port_scanner
ferox(scanner/port_scanner)> set RHOSTS 127.0.0.1
ferox(scanner/port_scanner)> set PORTS 1-100
ferox(scanner/port_scanner)> run

# Use subdomain enumeration
ferox> use recon/subdomain_enum
ferox(recon/subdomain_enum)> set RHOSTS example.com
ferox(recon/subdomain_enum)> check
ferox(recon/subdomain_enum)> run

# Use exploit module (safe skeleton)
ferox> use exploit/example/example_exploit
ferox(exploit/example/example_exploit)> set RHOSTS target.local
ferox(exploit/example/example_exploit)> set LHOST 192.168.1.100
ferox(exploit/example/example_exploit)> check
ferox(exploit/example/example_exploit)> run
```

## Available Commands

- `help` - Show all commands
- `modules` - List available modules
- `use <module>` - Select a module
- `options` - Show module options
- `set <opt> <val>` - Set option value
- `check` - Run safe check (non-destructive)
- `run` - Execute module
- `sessions` - Manage sessions
- `clear` - Clear screen
- `exit` - Exit Ferox

## Development

### Running Tests
```bash
cargo test
```

### Code Quality
```bash
# Check code
cargo clippy

# Format code
cargo fmt
```

### Adding a New Module

1. Create file in `src/modules/<category>/<name>.rs`
2. Implement the `Module` trait
3. Register in `src/main.rs`

Example:
```rust
use crate::core::module::*;
use async_trait::async_trait;

pub struct MyModule {
    options: HashMap<String, String>,
}

#[async_trait]
impl Module for MyModule {
    fn info(&self) -> ModuleInfo { /* ... */ }
    fn options(&self) -> Vec<ModuleOption> { /* ... */ }
    async fn check(&self) -> Result<CheckResult> { /* ... */ }
    async fn run(&mut self) -> Result<ModuleResult> { /* ... */ }
}
```

## Project Structure

```
ferox/
├── Cargo.toml              # Dependencies
├── README.md               # Documentation
├── wordlist.txt            # Sample wordlist
├── setup.sh                # Setup script
├── DEVELOPMENT.md          # This file
└── src/
    ├── main.rs             # Entry point
    ├── cli/                # CLI interface
    │   ├── app.rs          # REPL
    │   └── theme.rs        # Styling
    ├── core/               # Core engine
    │   ├── module.rs       # Module trait
    │   ├── session.rs      # Session manager
    │   └── payload.rs      # Payload framework
    └── modules/            # Security modules
        ├── scanner/        # Port scanner
        ├── recon/          # Subdomain enum
        └── exploit/        # Exploit modules
```

## Troubleshooting

### Build Errors
- Ensure Rust 1.70+ is installed
- Run `cargo clean` then `cargo build`

### Runtime Errors
- Check DNS resolver configuration
- Verify wordlist file exists
- Ensure proper permissions

## Safety & Legal

⚠️ **IMPORTANT**: Ferox is for authorized testing only.
- Only test systems you own or have permission to test
- Use `check` before `run` for exploits
- Exploits require explicit confirmation
- All actions are logged

## Performance Tips

- Adjust THREADS for your network
- Use appropriate TIMEOUT values
- Start with small port ranges
- Use JSON output for automation

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Support

- GitHub: https://github.com/abdulwahed-sweden/ferox
- Email: abdulwahed.mansour@gmail.com

---

🦊 **Fast. Fierce. Fearless.**
