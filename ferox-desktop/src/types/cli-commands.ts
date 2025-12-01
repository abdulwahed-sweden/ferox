// src/types/cli-commands.ts
// Type definitions for CLI command display feature

export interface CLICommand {
  id: string;
  module: string;
  category: ModuleCategory;
  command: string;
  description: string;
  parameters: CommandParameter[];
  examples: CommandExample[];
  output?: string;
  tags: string[];
}

export interface CommandParameter {
  name: string;
  flag: string;
  type: "string" | "number" | "boolean" | "enum";
  required: boolean;
  default?: string | number | boolean;
  description: string;
  options?: string[]; // For enum type
}

export interface CommandExample {
  title: string;
  command: string;
  description: string;
  output?: string;
}

export type ModuleCategory =
  | "reconnaissance"
  | "exploitation"
  | "post-exploitation"
  | "persistence"
  | "evasion"
  | "payload"
  | "c2"
  | "utility";

export interface Alert {
  id: string;
  type: "warning" | "info" | "success" | "danger";
  title: string;
  message: string;
  timestamp?: Date;
  dismissible?: boolean;
}
