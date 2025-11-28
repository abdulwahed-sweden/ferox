# Claude Code Prompt - Apply Official Ferox Logo

## 🎯 Task

Replace all existing logo files with the official traced SVG logos.

---

## 📁 SVG Files Ready (Copy from outputs)

The following SVG files are ready in `/mnt/user-data/outputs/`:

| File | Description |
|------|-------------|
| `ferox-logo.svg` | Base logo (uses `currentColor` - theme adaptive) |
| `ferox-logo-orange.svg` | Orange `#FF7F00` (Primary brand color) |
| `ferox-logo-white.svg` | White `#FFFFFF` (for dark backgrounds) |
| `ferox-logo-dark.svg` | Dark `#12161F` (for light backgrounds) |
| `ferox-logo-blue.svg` | Blue `#3B82F6` (accent variant) |

---

## 📋 Implementation Steps

### Step 1: Copy SVG Files

```bash
# Create logo directory if not exists
mkdir -p src/assets/logo

# Copy all SVG files
cp /mnt/user-data/outputs/ferox-logo.svg src/assets/logo/
cp /mnt/user-data/outputs/ferox-logo-orange.svg src/assets/logo/
cp /mnt/user-data/outputs/ferox-logo-white.svg src/assets/logo/
cp /mnt/user-data/outputs/ferox-logo-dark.svg src/assets/logo/
cp /mnt/user-data/outputs/ferox-logo-blue.svg src/assets/logo/
```

### Step 2: Update Favicon

```bash
# Copy orange logo as favicon
cp /mnt/user-data/outputs/ferox-logo-orange.svg public/favicon.svg
```

### Step 3: Update Logo.tsx Component

Replace the existing Logo component with one that imports the new SVGs:

```tsx
import { useTheme } from '@/hooks/useTheme';

// Import SVGs
import LogoBase from '@/assets/logo/ferox-logo.svg?react';
import LogoOrange from '@/assets/logo/ferox-logo-orange.svg?react';
import LogoWhite from '@/assets/logo/ferox-logo-white.svg?react';
import LogoDark from '@/assets/logo/ferox-logo-dark.svg?react';
import LogoBlue from '@/assets/logo/ferox-logo-blue.svg?react';

interface LogoProps {
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
  color?: 'auto' | 'orange' | 'white' | 'dark' | 'blue';
  className?: string;
}

const sizes = {
  xs: 16,
  sm: 24,
  md: 32,
  lg: 48,
  xl: 64,
};

export function Logo({ 
  size = 'md', 
  color = 'auto',
  className = ''
}: LogoProps) {
  const { theme } = useTheme();
  const dimension = sizes[size];
  
  // Select logo based on color prop
  const LogoComponent = (() => {
    switch (color) {
      case 'orange': return LogoOrange;
      case 'white': return LogoWhite;
      case 'dark': return LogoDark;
      case 'blue': return LogoBlue;
      case 'auto':
      default:
        return theme === 'dark' ? LogoWhite : LogoDark;
    }
  })();
  
  return (
    <LogoComponent 
      width={dimension} 
      height={dimension}
      className={className}
    />
  );
}
```

### Step 4: Generate PNG Files (for Tauri icons)

```bash
# Install sharp if not available
npm install sharp --save-dev

# Create script to generate PNGs
cat > scripts/generate-icons.js << 'EOF'
const sharp = require('sharp');
const fs = require('fs');
const path = require('path');

const sizes = [16, 32, 64, 128, 256, 512, 1024];
const inputSvg = 'src/assets/logo/ferox-logo-orange.svg';
const outputDir = 'src-tauri/icons';

// Ensure output directory exists
if (!fs.existsSync(outputDir)) {
  fs.mkdirSync(outputDir, { recursive: true });
}

async function generateIcons() {
  const svgBuffer = fs.readFileSync(inputSvg);
  
  for (const size of sizes) {
    await sharp(svgBuffer)
      .resize(size, size)
      .png()
      .toFile(path.join(outputDir, `${size}x${size}.png`));
    console.log(`Generated ${size}x${size}.png`);
  }
  
  // Copy 512 as main icon
  fs.copyFileSync(
    path.join(outputDir, '512x512.png'),
    path.join(outputDir, 'icon.png')
  );
  
  // Copy 256 as retina icon
  fs.copyFileSync(
    path.join(outputDir, '256x256.png'),
    path.join(outputDir, '128x128@2x.png')
  );
  
  console.log('All icons generated!');
}

generateIcons().catch(console.error);
EOF

node scripts/generate-icons.js
```

### Step 5: Verify

```bash
# Check all files exist
ls -la src/assets/logo/
ls -la src-tauri/icons/
ls -la public/favicon.svg

# Build to verify
npm run build
npm test
```

---

## ✅ Verification Checklist

- [ ] All 5 SVG variants in src/assets/logo/
- [ ] favicon.svg updated in public/
- [ ] Logo.tsx component updated
- [ ] Tauri icons generated (if needed)
- [ ] Build passes
- [ ] Tests pass

---

## 📝 Commit Message

```
feat: apply official Ferox fox logo

- Replace traced SVG with professionally designed fox logo
- Add color variants: orange, white, dark, blue
- Update Logo.tsx component with theme-aware selection
- Update favicon
- Generate Tauri app icons

Design: Official fox head silhouette from approved design
Primary color: #FF7F00 (Orange)
```
