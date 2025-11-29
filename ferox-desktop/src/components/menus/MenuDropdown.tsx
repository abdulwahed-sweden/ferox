import { ReactNode, useRef, useEffect, useState } from "react";
import { clsx } from "clsx";
import { motion, AnimatePresence } from "framer-motion";

interface MenuItem {
  id: string;
  label: string;
  icon?: ReactNode;
  shortcut?: string;
  disabled?: boolean;
  danger?: boolean;
  separator?: boolean;
  onClick?: () => void;
}

interface MenuDropdownProps {
  label: string;
  items: MenuItem[];
  isOpen: boolean;
  onToggle: () => void;
  onClose: () => void;
}

export function MenuDropdown({
  label,
  items,
  isOpen,
  onToggle,
  onClose,
}: MenuDropdownProps) {
  const menuRef = useRef<HTMLDivElement>(null);
  const [position, setPosition] = useState({ top: 0, left: 0 });

  // Position menu below button
  useEffect(() => {
    if (isOpen && menuRef.current) {
      const button = menuRef.current.querySelector("button");
      if (button) {
        const rect = button.getBoundingClientRect();
        setPosition({ top: rect.bottom + 4, left: rect.left });
      }
    }
  }, [isOpen]);

  // Close on outside click
  useEffect(() => {
    const handleClick = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };

    if (isOpen) {
      // Delay adding listener to prevent immediate close
      setTimeout(() => {
        document.addEventListener("click", handleClick);
      }, 0);
    }

    return () => document.removeEventListener("click", handleClick);
  }, [isOpen, onClose]);

  const handleItemClick = (item: MenuItem) => {
    if (!item.disabled && item.onClick) {
      item.onClick();
      onClose();
    }
  };

  return (
    <div ref={menuRef} className="relative">
      <button
        onClick={(e) => {
          e.stopPropagation();
          onToggle();
        }}
        className={clsx(
          "px-3 py-1 rounded text-sm transition-colors",
          isOpen
            ? "bg-[var(--bg-hover)] text-[var(--text-primary)]"
            : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text-primary)]"
        )}
      >
        {label}
      </button>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, y: -4 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -4 }}
            transition={{ duration: 0.1 }}
            className="fixed z-50 rounded-lg"
            style={{
              top: position.top,
              left: position.left,
              minWidth: "220px",
              maxWidth: "320px",
              padding: "4px",
              backgroundColor: "var(--surface-primary)",
              border: "1px solid var(--border-primary)",
              boxShadow: "0 10px 25px -5px rgba(0, 0, 0, 0.3), 0 8px 10px -6px rgba(0, 0, 0, 0.2)",
            }}
          >
            {items.map((item, index) =>
              item.separator ? (
                <div
                  key={`sep-${index}`}
                  className="h-px my-1"
                  style={{ backgroundColor: "var(--border-primary)" }}
                />
              ) : (
                <button
                  key={item.id}
                  onClick={() => handleItemClick(item)}
                  disabled={item.disabled}
                  className={clsx(
                    "w-full flex items-center px-3 py-2 rounded transition-colors",
                    item.disabled
                      ? "cursor-not-allowed"
                      : item.danger
                        ? "hover:bg-red-500/10"
                        : "hover:bg-[var(--bg-hover)]"
                  )}
                  style={{
                    fontSize: "13px",
                    lineHeight: "1.4",
                    whiteSpace: "nowrap",
                    color: item.disabled
                      ? "var(--text-muted)"
                      : item.danger
                        ? "#f87171"
                        : "var(--text-secondary)",
                  }}
                >
                  <span className="flex items-center gap-2 flex-shrink-0">
                    {item.icon}
                    <span>{item.label}</span>
                  </span>
                  {item.shortcut && (
                    <span
                      className="ml-auto pl-4 flex-shrink-0"
                      style={{
                        fontSize: "11px",
                        color: "var(--text-muted)",
                        opacity: 0.7,
                      }}
                    >
                      {item.shortcut}
                    </span>
                  )}
                </button>
              )
            )}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
