/**
 * Logo component - Theme-aware Ferox fox logo
 * Uses the new minimalist fox silhouette design with negative space details
 */

import { useTheme } from "../../hooks/useTheme";

export type LogoColor = "orange" | "white" | "dark" | "blue" | "auto";

interface LogoProps {
  variant?: "full" | "icon";
  className?: string;
  size?: "sm" | "md" | "lg" | "xl";
  color?: LogoColor;
}

const sizes = {
  sm: { full: "h-6", icon: "h-6 w-6" },
  md: { full: "h-8", icon: "h-8 w-8" },
  lg: { full: "h-10", icon: "h-10 w-10" },
  xl: { full: "h-14", icon: "h-14 w-14" },
};

const colors: Record<LogoColor, { fill: string; bg: string }> = {
  orange: { fill: "#E07B2E", bg: "#12161F" },
  white: { fill: "#FFFFFF", bg: "#12161F" },
  dark: { fill: "#12161F", bg: "#F0F2F5" },
  blue: { fill: "#3B82F6", bg: "#12161F" },
  auto: { fill: "", bg: "" }, // Will be determined by theme
};

// Fox head SVG path - main outline
const FOX_HEAD_PATH = `M50 5
  L20 25
  L15 20
  L10 35
  L5 55
  L15 70
  L25 80
  L35 88
  L50 95
  L65 88
  L75 80
  L85 70
  L95 55
  L90 35
  L85 20
  L80 25
  L50 5
  Z`;

// Negative space paths
const LEFT_EAR_PATH = "M25 28 L22 38 L28 45 L32 35 L25 28 Z";
const RIGHT_EAR_PATH = "M75 28 L78 38 L72 45 L68 35 L75 28 Z";
const LEFT_EYE_PATH = "M30 50 L25 55 L30 60 L38 55 L30 50 Z";
const RIGHT_EYE_PATH = "M70 50 L75 55 L70 60 L62 55 L70 50 Z";
const LEFT_CHEEK_PATH = "M18 58 L22 65 L28 62 L24 55 L18 58 Z";
const RIGHT_CHEEK_PATH = "M82 58 L78 65 L72 62 L76 55 L82 58 Z";

export function Logo({
  variant = "full",
  className = "",
  size = "md",
  color = "auto",
}: LogoProps) {
  const { theme } = useTheme();
  const isDark = theme === "dark";
  const sizeClass = sizes[size][variant];

  // Determine colors based on theme or explicit color choice
  let fillColor: string;
  let bgColor: string;

  if (color === "auto") {
    fillColor = isDark ? "#E07B2E" : "#12161F";
    bgColor = isDark ? "#12161F" : "#F0F2F5";
  } else {
    fillColor = colors[color].fill;
    bgColor = colors[color].bg;
  }

  if (variant === "icon") {
    return (
      <svg
        viewBox="0 0 100 100"
        className={`${sizeClass} ${className}`}
        xmlns="http://www.w3.org/2000/svg"
        aria-label="Ferox Logo"
      >
        {/* Main fox head outline */}
        <path d={FOX_HEAD_PATH} fill={fillColor} />

        {/* Negative space details */}
        <path d={LEFT_EAR_PATH} fill={bgColor} />
        <path d={RIGHT_EAR_PATH} fill={bgColor} />
        <path d={LEFT_EYE_PATH} fill={bgColor} />
        <path d={RIGHT_EYE_PATH} fill={bgColor} />
        <path d={LEFT_CHEEK_PATH} fill={bgColor} />
        <path d={RIGHT_CHEEK_PATH} fill={bgColor} />
      </svg>
    );
  }

  // Full logo with text
  const textColor = isDark ? "#F0F2F5" : "#111827";

  return (
    <svg
      viewBox="0 0 280 100"
      className={`${sizeClass} ${className}`}
      xmlns="http://www.w3.org/2000/svg"
      aria-label="Ferox - Penetration Testing"
    >
      {/* Fox Head Icon */}
      <g transform="translate(0, 0)">
        <path d={FOX_HEAD_PATH} fill={fillColor} />
        <path d={LEFT_EAR_PATH} fill={bgColor} />
        <path d={RIGHT_EAR_PATH} fill={bgColor} />
        <path d={LEFT_EYE_PATH} fill={bgColor} />
        <path d={RIGHT_EYE_PATH} fill={bgColor} />
        <path d={LEFT_CHEEK_PATH} fill={bgColor} />
        <path d={RIGHT_CHEEK_PATH} fill={bgColor} />
      </g>

      {/* FEROX Text */}
      <text
        x="110"
        y="68"
        fontFamily="Inter, system-ui, -apple-system, sans-serif"
        fontSize="56"
        fontWeight="700"
        fill={textColor}
        letterSpacing="-1"
      >
        FEROX
      </text>
    </svg>
  );
}

export default Logo;
