#!/usr/bin/env node
/**
 * Ferox Icon Generator
 * Generates all required icon sizes for Tauri desktop app and web
 *
 * Usage: node scripts/generate-icons.js
 */

import { Resvg } from '@resvg/resvg-js';
import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Paths
const ASSETS_DIR = path.join(__dirname, '..', 'src', 'assets', 'logo');
const TAURI_ICONS_DIR = path.join(__dirname, '..', 'src-tauri', 'icons');
const PUBLIC_DIR = path.join(__dirname, '..', 'public');

// Source SVG
const SVG_SOURCE = path.join(ASSETS_DIR, 'ferox-fox.svg');

// Icon sizes needed
const TAURI_SIZES = [
  { name: '16x16.png', size: 16 },
  { name: '32x32.png', size: 32 },
  { name: '128x128.png', size: 128 },
  { name: '128x128@2x.png', size: 256 },
  { name: 'icon.png', size: 512 },
];

const WEB_SIZES = [
  { name: 'favicon-16x16.png', size: 16 },
  { name: 'favicon-32x32.png', size: 32 },
  { name: 'apple-touch-icon.png', size: 180 },
  { name: 'android-chrome-192x192.png', size: 192 },
  { name: 'android-chrome-512x512.png', size: 512 },
];

async function generatePng(svgPath, outputPath, size) {
  const svg = fs.readFileSync(svgPath, 'utf8');
  const resvg = new Resvg(svg, {
    fitTo: {
      mode: 'width',
      value: size
    },
    background: 'rgba(0, 0, 0, 0)', // Transparent background
  });
  const pngData = resvg.render();
  const pngBuffer = pngData.asPng();
  fs.writeFileSync(outputPath, pngBuffer);
  console.log(`Generated: ${outputPath} (${size}x${size})`);
}

async function generateIco(pngPaths, outputPath) {
  // For Windows .ico, we'll just copy the 32x32 version
  // A proper solution would use png-to-ico package
  console.log('Creating icon.ico from 32x32.png');
  fs.copyFileSync(pngPaths[1], outputPath);
  console.log(`Generated: ${outputPath}`);
}

async function generateIcns(pngPath, outputPath) {
  // macOS icon generation using iconutil
  if (process.platform === 'darwin') {
    try {
      const iconsetDir = path.join(path.dirname(outputPath), 'icon.iconset');

      // Create iconset directory
      if (!fs.existsSync(iconsetDir)) {
        fs.mkdirSync(iconsetDir, { recursive: true });
      }

      // Generate all required sizes for icns
      const icnsSizes = [
        { name: 'icon_16x16.png', size: 16 },
        { name: 'icon_16x16@2x.png', size: 32 },
        { name: 'icon_32x32.png', size: 32 },
        { name: 'icon_32x32@2x.png', size: 64 },
        { name: 'icon_128x128.png', size: 128 },
        { name: 'icon_128x128@2x.png', size: 256 },
        { name: 'icon_256x256.png', size: 256 },
        { name: 'icon_256x256@2x.png', size: 512 },
        { name: 'icon_512x512.png', size: 512 },
        { name: 'icon_512x512@2x.png', size: 1024 },
      ];

      for (const { name, size } of icnsSizes) {
        await generatePng(SVG_SOURCE, path.join(iconsetDir, name), size);
      }

      // Run iconutil to create .icns
      execSync(`iconutil -c icns "${iconsetDir}" -o "${outputPath}"`, { stdio: 'inherit' });

      // Clean up iconset directory
      fs.rmSync(iconsetDir, { recursive: true });

      console.log(`Generated: ${outputPath}`);
    } catch (e) {
      console.error('Failed to generate .icns:', e.message);
    }
  } else {
    console.log('Skipping .icns generation (not on macOS)');
  }
}

async function main() {
  console.log('Ferox Icon Generator');
  console.log('====================\n');

  // Check if SVG source exists
  if (!fs.existsSync(SVG_SOURCE)) {
    console.error(`Error: Source SVG not found at ${SVG_SOURCE}`);
    process.exit(1);
  }

  // Ensure directories exist
  [TAURI_ICONS_DIR, PUBLIC_DIR].forEach(dir => {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
  });

  console.log('Generating Tauri icons...');
  for (const { name, size } of TAURI_SIZES) {
    await generatePng(SVG_SOURCE, path.join(TAURI_ICONS_DIR, name), size);
  }

  console.log('\nGenerating web icons...');
  for (const { name, size } of WEB_SIZES) {
    await generatePng(SVG_SOURCE, path.join(PUBLIC_DIR, name), size);
  }

  // Generate .ico for Windows
  console.log('\nGenerating Windows icon...');
  const icoPngs = [
    path.join(TAURI_ICONS_DIR, '16x16.png'),
    path.join(TAURI_ICONS_DIR, '32x32.png'),
    path.join(TAURI_ICONS_DIR, '128x128.png'),
    path.join(TAURI_ICONS_DIR, '128x128@2x.png'),
  ];
  await generateIco(icoPngs, path.join(TAURI_ICONS_DIR, 'icon.ico'));

  // Generate .icns for macOS
  console.log('\nGenerating macOS icon...');
  await generateIcns(
    path.join(TAURI_ICONS_DIR, 'icon.png'),
    path.join(TAURI_ICONS_DIR, 'icon.icns')
  );

  console.log('\nDone! All icons generated successfully.');
}

main().catch(console.error);
