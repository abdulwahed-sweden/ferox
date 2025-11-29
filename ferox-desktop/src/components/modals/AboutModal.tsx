import { Modal } from "./Modal";
import { Logo } from "../ui/Logo";
import { ExternalLink, Github, Mail, Shield } from "lucide-react";

interface AboutModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function AboutModal({ isOpen, onClose }: AboutModalProps) {
  return (
    <Modal isOpen={isOpen} onClose={onClose} title="About Ferox" size="md">
      <div className="flex flex-col items-center text-center">
        {/* Logo */}
        <div className="mb-4">
          <Logo variant="icon" size="xl" color="auto" />
        </div>

        {/* Name & Version */}
        <h3 className="text-xl font-bold text-[var(--text-primary)] mb-1">
          Ferox Desktop
        </h3>
        <p className="text-sm text-[var(--text-secondary)] mb-4">
          Version 4.0.0
        </p>

        {/* Description */}
        <p className="text-sm text-[var(--text-secondary)] mb-6 max-w-sm">
          Professional security assessment platform built with Rust and React.
          Fast, Fierce, Fearless.
        </p>

        {/* Features */}
        <div className="w-full grid grid-cols-2 gap-2 mb-6 text-xs">
          <div className="flex items-center gap-2 p-2 rounded bg-[var(--surface-secondary)]">
            <Shield size={14} className="text-[var(--color-primary)]" />
            <span className="text-[var(--text-secondary)]">
              Security Framework
            </span>
          </div>
          <div className="flex items-center gap-2 p-2 rounded bg-[var(--surface-secondary)]">
            <span className="text-[var(--text-secondary)]">
              Tauri + React + Rust
            </span>
          </div>
        </div>

        {/* Links */}
        <div className="flex gap-3 mb-6">
          <a
            href="https://github.com/abdulwahed-sweden/ferox"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-[var(--surface-secondary)] hover:bg-[var(--bg-hover)] text-sm text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors"
          >
            <Github size={14} />
            GitHub
            <ExternalLink size={10} />
          </a>
          <a
            href="mailto:abdulwahed.mansour@gmail.com"
            className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-[var(--surface-secondary)] hover:bg-[var(--bg-hover)] text-sm text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors"
          >
            <Mail size={14} />
            Contact
          </a>
        </div>

        {/* Copyright */}
        <p className="text-xs text-[var(--text-muted)]">
          &copy; 2024 Abdulwahed Mansour. MIT License.
        </p>
        <p className="text-xs text-[var(--text-muted)] mt-1">
          For authorized security testing only.
        </p>
      </div>
    </Modal>
  );
}
