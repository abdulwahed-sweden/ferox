# Ferox C2 Documentation

Welcome to the Ferox C2 Framework documentation.

## Quick Links

- [Installation Guide](INSTALLATION.md)
- [User Guide](user-guide/getting-started.md)
- [Developer Guide](developer/architecture.md)
- [API Reference](api/tauri-commands.md)

## Documentation Structure

```
docs/
├── user-guide/          # End-user documentation
│   ├── getting-started.md
│   ├── dashboard.md
│   ├── sessions.md
│   ├── modules.md
│   ├── reports.md       # NEW: Report generation (JSON, HTML, PDF)
│   ├── maintenance.md
│   └── troubleshooting.md
│
├── developer/           # Developer documentation
│   ├── architecture.md
│   ├── c2.md
│   ├── console.md
│   ├── examples.md      # NEW: Test examples documentation
│   └── memory-forensics.md
│
├── api/                 # API documentation
│   └── tauri-commands.md
│
├── design/              # Design documents
│   ├── smart-payload-system.md
│   └── launch-checklist.md
│
└── assets/              # Documentation assets
    └── images/
```

## Getting Started

1. **Installation**: See [INSTALLATION.md](INSTALLATION.md) for setup instructions
2. **First Steps**: Follow the [Getting Started Guide](user-guide/getting-started.md)
3. **Understanding Sessions**: Learn about [Session Management](user-guide/sessions.md)
4. **Post-Exploitation**: Explore [Available Modules](user-guide/modules.md)
5. **Report Generation**: Learn about [Report Export](user-guide/reports.md) (JSON, HTML, PDF)

## For Developers

- [Architecture Overview](developer/architecture.md) - System design and components
- [C2 Protocol](developer/c2.md) - Command and control communication
- [Memory Forensics](developer/memory-forensics.md) - Memory analysis capabilities
- [Test Examples](developer/examples.md) - Report generation test examples

## Support

For issues and feature requests, please use the [GitHub Issues](https://github.com/your-org/ferox/issues) page.
