// ferox-desktop/src/components/ui/Tooltip.tsx
// Animated tooltip component with Framer Motion

import { useState, ReactNode } from "react";
import { motion, AnimatePresence } from "framer-motion";

type TooltipPosition = "top" | "bottom" | "left" | "right";

interface TooltipProps {
  content: ReactNode;
  children: ReactNode;
  position?: TooltipPosition;
  delay?: number;
  className?: string;
}

const positionStyles: Record<TooltipPosition, string> = {
  top: "bottom-full left-1/2 -translate-x-1/2 mb-2",
  bottom: "top-full left-1/2 -translate-x-1/2 mt-2",
  left: "right-full top-1/2 -translate-y-1/2 mr-2",
  right: "left-full top-1/2 -translate-y-1/2 ml-2",
};

const arrowStyles: Record<TooltipPosition, string> = {
  top: "top-full left-1/2 -translate-x-1/2 border-l-transparent border-r-transparent border-b-transparent",
  bottom:
    "bottom-full left-1/2 -translate-x-1/2 border-l-transparent border-r-transparent border-t-transparent",
  left: "left-full top-1/2 -translate-y-1/2 border-t-transparent border-b-transparent border-r-transparent",
  right:
    "right-full top-1/2 -translate-y-1/2 border-t-transparent border-b-transparent border-l-transparent",
};

const animationVariants = {
  top: {
    initial: { opacity: 0, y: 5, scale: 0.95 },
    animate: { opacity: 1, y: 0, scale: 1 },
    exit: { opacity: 0, y: 5, scale: 0.95 },
  },
  bottom: {
    initial: { opacity: 0, y: -5, scale: 0.95 },
    animate: { opacity: 1, y: 0, scale: 1 },
    exit: { opacity: 0, y: -5, scale: 0.95 },
  },
  left: {
    initial: { opacity: 0, x: 5, scale: 0.95 },
    animate: { opacity: 1, x: 0, scale: 1 },
    exit: { opacity: 0, x: 5, scale: 0.95 },
  },
  right: {
    initial: { opacity: 0, x: -5, scale: 0.95 },
    animate: { opacity: 1, x: 0, scale: 1 },
    exit: { opacity: 0, x: -5, scale: 0.95 },
  },
} as const;

export function Tooltip({
  content,
  children,
  position = "top",
  delay = 200,
  className = "",
}: TooltipProps) {
  const [isVisible, setIsVisible] = useState(false);
  const [timeoutId, setTimeoutId] = useState<NodeJS.Timeout | null>(null);

  const handleMouseEnter = () => {
    const id = setTimeout(() => setIsVisible(true), delay);
    setTimeoutId(id);
  };

  const handleMouseLeave = () => {
    if (timeoutId) {
      clearTimeout(timeoutId);
      setTimeoutId(null);
    }
    setIsVisible(false);
  };

  const variants = animationVariants[position];

  return (
    <div
      className={`relative inline-flex ${className}`}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {children}
      <AnimatePresence>
        {isVisible && (
          <motion.div
            className={`absolute z-50 ${positionStyles[position]}`}
            initial={variants.initial}
            animate={variants.animate}
            exit={variants.exit}
            transition={{ duration: 0.15, ease: "easeOut" }}
          >
            <div className="px-2 py-1 text-xs font-medium text-white bg-dark-900 rounded shadow-lg border border-dark-600 whitespace-nowrap">
              {content}
            </div>
            {/* Arrow */}
            <div
              className={`absolute w-0 h-0 border-4 border-dark-900 ${arrowStyles[position]}`}
            />
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

// Info tooltip variant with icon
interface InfoTooltipProps {
  content: ReactNode;
  position?: TooltipPosition;
}

export function InfoTooltip({ content, position = "top" }: InfoTooltipProps) {
  return (
    <Tooltip content={content} position={position}>
      <span className="inline-flex items-center justify-center w-4 h-4 text-xs text-text-muted hover:text-text-secondary cursor-help rounded-full border border-dark-600">
        ?
      </span>
    </Tooltip>
  );
}

// Badge tooltip for status indicators
interface StatusTooltipProps {
  status: "success" | "warning" | "error" | "info";
  message: string;
  children: ReactNode;
}

const statusColors = {
  success: "border-green-400/50",
  warning: "border-yellow-400/50",
  error: "border-red-400/50",
  info: "border-cyan-400/50",
};

export function StatusTooltip({
  status,
  message,
  children,
}: StatusTooltipProps) {
  return (
    <Tooltip
      content={
        <div className={`border-l-2 pl-2 ${statusColors[status]}`}>
          {message}
        </div>
      }
    >
      {children}
    </Tooltip>
  );
}
