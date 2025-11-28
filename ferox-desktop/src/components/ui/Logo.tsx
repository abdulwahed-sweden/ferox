/**
 * Logo component - Theme-aware Ferox logo
 */

import { useTheme } from "../../hooks/useTheme";

interface LogoProps {
  variant?: "full" | "icon";
  className?: string;
  size?: "sm" | "md" | "lg";
}

const sizes = {
  sm: { full: "h-6", icon: "h-6 w-6" },
  md: { full: "h-8", icon: "h-8 w-8" },
  lg: { full: "h-10", icon: "h-10 w-10" },
};

export function Logo({
  variant = "full",
  className = "",
  size = "md",
}: LogoProps) {
  const { theme } = useTheme();
  const isDark = theme === "dark";
  const sizeClass = sizes[size][variant];

  if (variant === "icon") {
    return (
      <svg
        viewBox="0 0 100 100"
        className={`${sizeClass} ${className}`}
        xmlns="http://www.w3.org/2000/svg"
      >
        <defs>
          <linearGradient
            id="logoIconGradient"
            x1="0%"
            y1="0%"
            x2="100%"
            y2="100%"
          >
            <stop offset="0%" stopColor={isDark ? "#60A5FA" : "#3B82F6"} />
            <stop offset="100%" stopColor={isDark ? "#34D399" : "#10B981"} />
          </linearGradient>
        </defs>
        <circle cx="50" cy="50" r="48" fill={isDark ? "#12161F" : "#F4F5F7"} />
        <g transform="translate(50, 50)">
          <path
            d="M0 -30 L25 5 L18 35 L0 28 L-18 35 L-25 5 Z"
            fill="url(#logoIconGradient)"
          />
          <path d="M-25 5 L-32 -25 L-12 -10 Z" fill="url(#logoIconGradient)" />
          <path d="M25 5 L32 -25 L12 -10 Z" fill="url(#logoIconGradient)" />
          <ellipse
            cx="-10"
            cy="0"
            rx="5"
            ry="6"
            fill={isDark ? "#12161F" : "#F4F5F7"}
          />
          <ellipse
            cx="10"
            cy="0"
            rx="5"
            ry="6"
            fill={isDark ? "#12161F" : "#F4F5F7"}
          />
          <circle cx="-8" cy="-2" r="2" fill={isDark ? "#34D399" : "#10B981"} />
          <circle cx="12" cy="-2" r="2" fill={isDark ? "#34D399" : "#10B981"} />
        </g>
      </svg>
    );
  }

  // Full logo with text
  return (
    <svg
      viewBox="0 0 200 60"
      className={`${sizeClass} ${className}`}
      xmlns="http://www.w3.org/2000/svg"
    >
      <defs>
        <linearGradient
          id="logoFullGradient"
          x1="0%"
          y1="0%"
          x2="100%"
          y2="100%"
        >
          <stop offset="0%" stopColor={isDark ? "#60A5FA" : "#3B82F6"} />
          <stop offset="100%" stopColor={isDark ? "#34D399" : "#10B981"} />
        </linearGradient>
      </defs>
      <g transform="translate(5, 5)">
        <path
          d="M25 5 L40 25 L35 45 L25 40 L15 45 L10 25 Z"
          fill="url(#logoFullGradient)"
        />
        <path d="M10 25 L5 8 L18 18 Z" fill="url(#logoFullGradient)" />
        <path d="M40 25 L45 8 L32 18 Z" fill="url(#logoFullGradient)" />
        <circle cx="18" cy="25" r="3" fill={isDark ? "#12161F" : "#FFFFFF"} />
        <circle cx="32" cy="25" r="3" fill={isDark ? "#12161F" : "#FFFFFF"} />
        <circle cx="19" cy="24" r="1" fill={isDark ? "#34D399" : "#10B981"} />
        <circle cx="33" cy="24" r="1" fill={isDark ? "#34D399" : "#10B981"} />
        <path
          d="M25 32 L22 36 L28 36 Z"
          fill={isDark ? "#12161F" : "#FFFFFF"}
        />
      </g>
      <g transform="translate(60, 15)">
        <text
          x="0"
          y="30"
          fontFamily="Inter, system-ui, sans-serif"
          fontSize="32"
          fontWeight="700"
          fill={isDark ? "#F0F2F5" : "#111827"}
        >
          FEROX
        </text>
        <text
          x="2"
          y="45"
          fontFamily="Inter, system-ui, sans-serif"
          fontSize="8"
          fontWeight="400"
          fill={isDark ? "#9CA3AF" : "#6B7280"}
          letterSpacing="2"
        >
          PENETRATION TESTING
        </text>
      </g>
    </svg>
  );
}

export default Logo;
