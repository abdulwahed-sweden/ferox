// src/lib/__tests__/validation.test.ts
import { describe, it, expect } from "vitest";
import {
  SessionIdSchema,
  IpAddressSchema,
  CidrSchema,
  HostnameSchema,
  PortSchema,
  PortRangeSchema,
  CreateSessionRequestSchema,
  CreateTerminalRequestSchema,
  WriteTerminalRequestSchema,
  PayloadConfigSchema,
  ValidationError,
  validate,
  validateOrNull,
} from "../validation";

describe("SessionIdSchema", () => {
  it("accepts valid UUIDs", () => {
    const result = SessionIdSchema.safeParse(
      "550e8400-e29b-41d4-a716-446655440000"
    );
    expect(result.success).toBe(true);
  });

  it("accepts non-empty strings", () => {
    const result = SessionIdSchema.safeParse("session-123");
    expect(result.success).toBe(true);
  });

  it("rejects empty strings", () => {
    const result = SessionIdSchema.safeParse("");
    expect(result.success).toBe(false);
  });
});

describe("IpAddressSchema", () => {
  it("accepts valid IPv4 addresses", () => {
    const validIPs = [
      "192.168.1.1",
      "10.0.0.1",
      "172.16.0.1",
      "255.255.255.255",
      "0.0.0.0",
    ];

    validIPs.forEach((ip) => {
      const result = IpAddressSchema.safeParse(ip);
      expect(result.success, `${ip} should be valid`).toBe(true);
    });
  });

  it("rejects invalid IPv4 addresses", () => {
    const invalidIPs = [
      "256.1.1.1",
      "192.168.1",
      "192.168.1.1.1",
      "192.168.1.a",
      "not-an-ip",
      "",
    ];

    invalidIPs.forEach((ip) => {
      const result = IpAddressSchema.safeParse(ip);
      expect(result.success, `${ip} should be invalid`).toBe(false);
    });
  });
});

describe("CidrSchema", () => {
  it("accepts valid CIDR notation", () => {
    const validCIDRs = [
      "192.168.1.0/24",
      "10.0.0.0/8",
      "172.16.0.0/16",
      "192.168.0.1/32",
      "0.0.0.0/0",
    ];

    validCIDRs.forEach((cidr) => {
      const result = CidrSchema.safeParse(cidr);
      expect(result.success, `${cidr} should be valid`).toBe(true);
    });
  });

  it("rejects invalid CIDR notation", () => {
    const invalidCIDRs = [
      "192.168.1.0",
      "192.168.1.0/33",
      "192.168.1.0/-1",
      "256.168.1.0/24",
      "192.168.1/24",
      "not-a-cidr/24",
    ];

    invalidCIDRs.forEach((cidr) => {
      const result = CidrSchema.safeParse(cidr);
      expect(result.success, `${cidr} should be invalid`).toBe(false);
    });
  });
});

describe("HostnameSchema", () => {
  it("accepts valid hostnames", () => {
    const validHostnames = [
      "localhost",
      "server1",
      "web-server",
      "mail.example.com",
      "sub.domain.example.org",
    ];

    validHostnames.forEach((hostname) => {
      const result = HostnameSchema.safeParse(hostname);
      expect(result.success, `${hostname} should be valid`).toBe(true);
    });
  });

  it("rejects invalid hostnames", () => {
    const invalidHostnames = [
      "",
      "-invalid",
      "invalid-",
      ".invalid",
      "invalid..host",
      "host_name", // underscore not allowed
    ];

    invalidHostnames.forEach((hostname) => {
      const result = HostnameSchema.safeParse(hostname);
      expect(result.success, `${hostname} should be invalid`).toBe(false);
    });
  });
});

describe("PortSchema", () => {
  it("accepts valid port numbers", () => {
    const validPorts = [1, 80, 443, 8080, 65535];

    validPorts.forEach((port) => {
      const result = PortSchema.safeParse(port);
      expect(result.success, `${port} should be valid`).toBe(true);
    });
  });

  it("rejects invalid port numbers", () => {
    const invalidPorts = [0, -1, 65536, 100000, 1.5];

    invalidPorts.forEach((port) => {
      const result = PortSchema.safeParse(port);
      expect(result.success, `${port} should be invalid`).toBe(false);
    });
  });
});

describe("PortRangeSchema", () => {
  it("accepts valid port range strings", () => {
    const validRanges = ["80", "80,443", "80-90", "80,443,8080-8090"];

    validRanges.forEach((range) => {
      const result = PortRangeSchema.safeParse(range);
      expect(result.success, `${range} should be valid`).toBe(true);
    });
  });

  it("rejects invalid port range strings", () => {
    const invalidRanges = ["", "abc", "80-", "-80", "80,"];

    invalidRanges.forEach((range) => {
      const result = PortRangeSchema.safeParse(range);
      expect(result.success, `${range} should be invalid`).toBe(false);
    });
  });
});

describe("CreateSessionRequestSchema", () => {
  it("accepts valid session creation request", () => {
    const validRequest = {
      hostname: "workstation1",
      ip_address: "192.168.1.100",
      os: "windows",
      username: "admin",
      privileges: "administrator",
    };

    const result = CreateSessionRequestSchema.safeParse(validRequest);
    expect(result.success).toBe(true);
  });

  it("accepts request with optional parent_id", () => {
    const validRequest = {
      hostname: "workstation1",
      ip_address: "192.168.1.100",
      os: "linux",
      username: "root",
      privileges: "root",
      parent_id: "550e8400-e29b-41d4-a716-446655440000",
    };

    const result = CreateSessionRequestSchema.safeParse(validRequest);
    expect(result.success).toBe(true);
  });

  it("rejects request with invalid os", () => {
    const invalidRequest = {
      hostname: "workstation1",
      ip_address: "192.168.1.100",
      os: "invalid-os",
      username: "admin",
      privileges: "user",
    };

    const result = CreateSessionRequestSchema.safeParse(invalidRequest);
    expect(result.success).toBe(false);
  });

  it("rejects request with invalid IP address", () => {
    const invalidRequest = {
      hostname: "workstation1",
      ip_address: "not-an-ip",
      os: "windows",
      username: "admin",
      privileges: "user",
    };

    const result = CreateSessionRequestSchema.safeParse(invalidRequest);
    expect(result.success).toBe(false);
  });
});

describe("CreateTerminalRequestSchema", () => {
  it("accepts valid terminal creation request", () => {
    const validRequest = {
      session_id: "550e8400-e29b-41d4-a716-446655440000",
      rows: 24,
      cols: 80,
      shell: "/bin/bash",
    };

    const result = CreateTerminalRequestSchema.safeParse(validRequest);
    expect(result.success).toBe(true);
  });

  it("accepts minimal request with only session_id", () => {
    const minimalRequest = {
      session_id: "session-123",
    };

    const result = CreateTerminalRequestSchema.safeParse(minimalRequest);
    expect(result.success).toBe(true);
  });

  it("rejects invalid rows/cols values", () => {
    const invalidRequest = {
      session_id: "session-123",
      rows: 0, // Must be at least 1
      cols: 600, // Must be at most 500
    };

    const result = CreateTerminalRequestSchema.safeParse(invalidRequest);
    expect(result.success).toBe(false);
  });
});

describe("WriteTerminalRequestSchema", () => {
  it("accepts valid write request", () => {
    const validRequest = {
      terminal_id: "term-123",
      data: "ls -la\n",
    };

    const result = WriteTerminalRequestSchema.safeParse(validRequest);
    expect(result.success).toBe(true);
  });

  it("rejects empty terminal_id", () => {
    const invalidRequest = {
      terminal_id: "",
      data: "command",
    };

    const result = WriteTerminalRequestSchema.safeParse(invalidRequest);
    expect(result.success).toBe(false);
  });
});

describe("PayloadConfigSchema", () => {
  it("accepts valid payload configuration", () => {
    const validConfig = {
      payload_type: "reverse_shell",
      format: "exe",
      lhost: "192.168.1.10",
      lport: 4444,
      staged: true,
      obfuscation: true,
      encryption: "aes256",
      sleep_time: 5,
      jitter: 20,
    };

    const result = PayloadConfigSchema.safeParse(validConfig);
    expect(result.success).toBe(true);
  });

  it("accepts minimal payload configuration", () => {
    const minimalConfig = {
      payload_type: "bind_shell",
      format: "dll",
      lhost: "attacker.example.com",
      lport: 8080,
    };

    const result = PayloadConfigSchema.safeParse(minimalConfig);
    expect(result.success).toBe(true);
  });

  it("rejects invalid port", () => {
    const invalidConfig = {
      payload_type: "reverse_shell",
      format: "exe",
      lhost: "192.168.1.10",
      lport: 70000, // Invalid port
    };

    const result = PayloadConfigSchema.safeParse(invalidConfig);
    expect(result.success).toBe(false);
  });

  it("rejects invalid jitter value", () => {
    const invalidConfig = {
      payload_type: "reverse_shell",
      format: "exe",
      lhost: "192.168.1.10",
      lport: 4444,
      jitter: 150, // Must be 0-100
    };

    const result = PayloadConfigSchema.safeParse(invalidConfig);
    expect(result.success).toBe(false);
  });
});

describe("ValidationError", () => {
  it("creates error with formatted message", () => {
    const issues = [
      {
        path: ["ip_address"],
        message: "Invalid IP address format",
        code: "custom" as const,
      },
      {
        path: ["port"],
        message: "Must be between 1 and 65535",
        code: "custom" as const,
      },
    ];

    const error = new ValidationError(issues);

    expect(error.name).toBe("ValidationError");
    expect(error.message).toContain("ip_address: Invalid IP address format");
    expect(error.message).toContain("port: Must be between 1 and 65535");
    expect(error.issues).toEqual(issues);
  });
});

describe("validate helper", () => {
  it("returns parsed data on success", () => {
    const result = validate(IpAddressSchema, "192.168.1.1");
    expect(result).toBe("192.168.1.1");
  });

  it("throws ValidationError on failure", () => {
    expect(() => validate(IpAddressSchema, "not-an-ip")).toThrow(
      ValidationError
    );
  });
});

describe("validateOrNull helper", () => {
  it("returns parsed data on success", () => {
    const result = validateOrNull(IpAddressSchema, "192.168.1.1");
    expect(result).toBe("192.168.1.1");
  });

  it("returns null on failure", () => {
    const result = validateOrNull(IpAddressSchema, "not-an-ip");
    expect(result).toBeNull();
  });
});
