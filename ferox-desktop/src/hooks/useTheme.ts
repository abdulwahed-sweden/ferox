import { useState, useEffect, useCallback } from 'react';

export type Theme = 'light' | 'dark';

interface UseThemeReturn {
  theme: Theme;
  toggleTheme: () => void;
  setLightTheme: () => void;
  setDarkTheme: () => void;
  isDark: boolean;
  isLight: boolean;
}

const THEME_STORAGE_KEY = 'ferox-theme';

/**
 * Get initial theme from localStorage or system preference
 */
function getInitialTheme(): Theme {
  // Check localStorage first
  if (typeof window !== 'undefined') {
    const stored = localStorage.getItem(THEME_STORAGE_KEY) as Theme | null;
    if (stored === 'light' || stored === 'dark') {
      return stored;
    }

    // Check system preference
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      return 'dark';
    }
  }

  // Default to dark for Ferox aesthetic
  return 'dark';
}

/**
 * Hook for managing theme state and persistence
 */
export function useTheme(): UseThemeReturn {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);

  // Apply theme to document
  useEffect(() => {
    const root = document.documentElement;

    // Add transitioning class to prevent flash
    root.classList.add('theme-transitioning');

    // Set theme attribute
    root.setAttribute('data-theme', theme);

    // Store preference
    localStorage.setItem(THEME_STORAGE_KEY, theme);

    // Remove transitioning class after brief delay
    const timeout = setTimeout(() => {
      root.classList.remove('theme-transitioning');
    }, 50);

    return () => clearTimeout(timeout);
  }, [theme]);

  // Listen for system preference changes
  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

    const handleChange = (e: MediaQueryListEvent) => {
      // Only auto-switch if user hasn't set a preference
      const stored = localStorage.getItem(THEME_STORAGE_KEY);
      if (!stored) {
        setTheme(e.matches ? 'dark' : 'light');
      }
    };

    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, []);

  const toggleTheme = useCallback(() => {
    setTheme((prev) => (prev === 'light' ? 'dark' : 'light'));
  }, []);

  const setLightTheme = useCallback(() => setTheme('light'), []);
  const setDarkTheme = useCallback(() => setTheme('dark'), []);

  return {
    theme,
    toggleTheme,
    setLightTheme,
    setDarkTheme,
    isDark: theme === 'dark',
    isLight: theme === 'light',
  };
}

export default useTheme;
