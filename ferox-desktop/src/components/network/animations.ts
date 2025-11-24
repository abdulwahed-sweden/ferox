/**
 * Framer Motion Animation Configurations
 * For cyber-neon network visualization
 */

import { Variants } from 'framer-motion';

export const nodeVariants: Variants = {
  idle: {
    scale: 1,
    opacity: 0.7
  },
  active: {
    scale: [1, 1.05, 1],
    opacity: 1,
    transition: { duration: 2, repeat: Infinity, ease: 'easeInOut' }
  },
  scanning: {
    scale: [1, 1.1, 1],
    opacity: [0.7, 1, 0.7],
    transition: { duration: 1.5, repeat: Infinity, ease: 'easeInOut' }
  },
  exploited: {
    scale: [1, 1.08, 1],
    opacity: 1,
    transition: { duration: 0.8, repeat: Infinity, ease: 'easeInOut' }
  },
  offline: {
    scale: 1,
    opacity: 0.3
  }
};

export const pulseVariants: Variants = {
  pulse: {
    scale: [1, 1.8, 2.5],
    opacity: [0.6, 0.3, 0],
    transition: { duration: 2, repeat: Infinity, ease: 'easeOut' }
  }
};

export const glowVariants: Variants = {
  glow: {
    filter: [
      'drop-shadow(0 0 4px currentColor)',
      'drop-shadow(0 0 12px currentColor)',
      'drop-shadow(0 0 4px currentColor)'
    ],
    transition: { duration: 2, repeat: Infinity, ease: 'easeInOut' }
  }
};

export const edgeVariants: Variants = {
  active: {
    opacity: [0.4, 1, 0.4],
    transition: { duration: 2, repeat: Infinity, ease: 'easeInOut' }
  },
  idle: {
    opacity: 0.2
  }
};

export const dataFlowVariants: Variants = {
  flow: {
    offsetDistance: ['0%', '100%'],
    transition: { duration: 1.5, repeat: Infinity, ease: 'linear' }
  }
};

export const tooltipVariants: Variants = {
  hidden: {
    opacity: 0,
    scale: 0.9,
    y: 10
  },
  visible: {
    opacity: 1,
    scale: 1,
    y: 0,
    transition: { duration: 0.2 }
  },
  exit: {
    opacity: 0,
    scale: 0.9,
    y: 10,
    transition: { duration: 0.15 }
  }
};

export const legendVariants: Variants = {
  hidden: { opacity: 0, x: -20 },
  visible: {
    opacity: 1,
    x: 0,
    transition: { duration: 0.3, staggerChildren: 0.05 }
  }
};

export const legendItemVariants: Variants = {
  hidden: { opacity: 0, x: -10 },
  visible: { opacity: 1, x: 0 }
};
