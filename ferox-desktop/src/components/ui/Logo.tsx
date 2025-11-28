/**
 * Logo component - Theme-aware Ferox fox logo
 * Official design from potrace vector trace
 */

import { useTheme } from "../../hooks/useTheme";

export type LogoColor = "orange" | "auto";

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
  auto: { fill: "", bg: "" },
};

// Official Ferox fox logo paths (from potrace)
const BACKGROUND_PATH =
  "M0 5120 l0 -5120 5120 0 5120 0 0 5120 0 5120 -5120 0 -5120 0 0 -5120z";

const FOX_HEAD_PATH =
  "M1809 9185 c16 -14 40 -42 52 -61 27 -45 207 -274 262 -334 22 -25 61 -68 86 -95 160 -177 318 -342 551 -575 615 -614 1015 -988 1047 -976 27 11 234 45 363 61 453 54 788 70 1201 55 161 -6 308 -12 324 -14 17 -2 64 -7 105 -11 80 -7 378 -43 420 -50 14 -3 72 -13 128 -24 l104 -19 71 60 c152 127 369 332 790 748 208 204 461 459 512 514 11 12 49 53 85 92 115 123 202 223 297 342 52 64 100 124 107 133 7 9 32 43 54 77 49 72 85 102 122 102 54 0 102 -94 125 -245 76 -510 77 -519 104 -850 23 -289 25 -609 7 -935 -17 -293 -19 -314 -47 -550 -51 -420 -96 -645 -222 -1100 -28 -102 -32 -130 -32 -245 0 -109 4 -142 23 -200 33 -100 121 -243 203 -332 38 -41 234 -223 252 -235 14 -8 21 -2 -88 -76 -49 -33 -115 -83 -145 -112 -30 -28 -72 -66 -93 -83 -22 -18 -51 -42 -65 -55 -67 -58 -326 -220 -687 -427 -223 -128 -213 -123 -340 -200 -60 -37 -126 -77 -145 -88 -166 -101 -444 -304 -564 -411 -373 -335 -565 -642 -725 -1158 -56 -183 -132 -309 -265 -443 -137 -138 -257 -212 -423 -261 -375 -111 -794 58 -1031 416 -55 82 -109 205 -150 342 -54 177 -192 476 -287 618 -273 411 -626 694 -1445 1158 -25 14 -78 45 -118 69 -41 24 -75 43 -77 43 -8 0 -337 197 -405 242 -76 51 -164 123 -233 190 -54 53 -173 148 -234 187 -29 18 -53 37 -53 41 0 4 17 22 38 40 95 82 268 262 313 328 141 204 170 396 99 657 -143 527 -232 1023 -255 1430 -3 61 -9 162 -14 225 -12 171 -13 603 -1 785 19 303 50 587 96 875 38 240 60 314 103 347 26 20 67 15 100 -12z";

const LEFT_EAR_PATH =
  "M1994 8215 c-9 -22 22 -338 46 -475 24 -137 42 -221 76 -355 14 -55 30 -117 35 -137 5 -20 27 -99 50 -175 22 -76 45 -151 49 -168 4 -16 20 -68 35 -115 59 -188 126 -402 165 -530 23 -74 54 -175 70 -225 16 -49 41 -129 55 -177 14 -49 29 -88 32 -88 32 0 111 223 142 400 28 164 34 285 22 445 -20 243 -70 432 -188 705 -48 110 -73 160 -168 335 -89 162 -380 575 -405 575 -6 0 -13 -7 -16 -15z";

const RIGHT_EAR_PATH =
  "M8195 8202 c-215 -292 -274 -380 -384 -577 -71 -126 -161 -310 -161 -327 0 -8 -4 -18 -9 -23 -5 -6 -19 -39 -31 -75 -12 -36 -26 -74 -30 -85 -19 -44 -50 -155 -69 -245 -29 -136 -41 -246 -41 -390 0 -229 31 -398 114 -624 41 -110 53 -112 79 -11 14 54 64 219 193 635 25 80 54 172 64 205 58 192 83 276 96 325 8 30 23 84 34 120 90 306 155 629 180 885 19 209 12 250 -35 187z";

const LEFT_FACE_PATH =
  "M2490 4751 c-17 -12 2 -54 81 -182 78 -127 127 -189 270 -340 108 -115 271 -189 544 -247 259 -56 333 -76 400 -110 165 -82 336 -241 488 -455 87 -122 90 -126 100 -120 12 7 -6 103 -64 338 -33 131 -108 328 -174 453 -126 238 -326 442 -508 520 -44 19 -149 54 -177 59 -14 2 -54 11 -89 19 -75 17 -233 40 -341 49 -41 4 -86 8 -100 11 -64 10 -416 15 -430 5z";

const RIGHT_FACE_PATH =
  "M7425 4753 c-284 -17 -610 -70 -755 -124 -208 -77 -382 -237 -543 -501 -106 -172 -197 -437 -252 -732 -22 -120 -10 -128 50 -34 110 170 276 350 405 439 138 95 198 117 533 194 106 24 206 49 222 55 17 7 38 13 48 15 10 2 56 23 103 46 72 36 101 58 184 143 113 115 185 206 256 321 55 88 84 153 77 173 -4 11 -177 15 -328 5z";

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
        viewBox="0 0 1024 1024"
        className={`${sizeClass} ${className}`}
        xmlns="http://www.w3.org/2000/svg"
        aria-label="Ferox Logo"
      >
        <g transform="translate(0,1024) scale(0.1,-0.1)">
          <path d={BACKGROUND_PATH} fill={fillColor} />
          <path d={FOX_HEAD_PATH} fill={bgColor} />
          <path d={LEFT_EAR_PATH} fill={bgColor} />
          <path d={RIGHT_EAR_PATH} fill={bgColor} />
          <path d={LEFT_FACE_PATH} fill={bgColor} />
          <path d={RIGHT_FACE_PATH} fill={bgColor} />
        </g>
      </svg>
    );
  }

  // Full logo with text
  const textColor = isDark ? "#F0F2F5" : "#111827";

  return (
    <svg
      viewBox="0 0 380 100"
      className={`${sizeClass} ${className}`}
      xmlns="http://www.w3.org/2000/svg"
      aria-label="Ferox - Penetration Testing"
    >
      {/* Fox Head Icon - scaled to fit */}
      <g transform="translate(0, 0) scale(0.0977)">
        <g transform="translate(0,1024) scale(0.1,-0.1)">
          <path d={BACKGROUND_PATH} fill={fillColor} />
          <path d={FOX_HEAD_PATH} fill={bgColor} />
          <path d={LEFT_EAR_PATH} fill={bgColor} />
          <path d={RIGHT_EAR_PATH} fill={bgColor} />
          <path d={LEFT_FACE_PATH} fill={bgColor} />
          <path d={RIGHT_FACE_PATH} fill={bgColor} />
        </g>
      </g>

      {/* FEROX Text */}
      <text
        x="115"
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
