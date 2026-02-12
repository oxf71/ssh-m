import { useState, useEffect, useCallback } from "react";

export type ThemeMode = "light" | "dark" | "system";

const STORAGE_KEY = "ssh-m:theme";

function getSystemTheme(): "light" | "dark" {
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

function getResolvedTheme(mode: ThemeMode): "light" | "dark" {
  return mode === "system" ? getSystemTheme() : mode;
}

export function useTheme() {
  const [mode, setMode] = useState<ThemeMode>(
    () => (localStorage.getItem(STORAGE_KEY) as ThemeMode) || "system",
  );
  const [resolved, setResolved] = useState<"light" | "dark">(() =>
    getResolvedTheme(
      (localStorage.getItem(STORAGE_KEY) as ThemeMode) || "system",
    ),
  );

  // Apply theme to <html> element
  const applyTheme = useCallback((theme: "light" | "dark") => {
    const root = document.documentElement;
    root.classList.remove("light", "dark");
    root.classList.add(theme);
    setResolved(theme);
  }, []);

  // Listen for system theme changes
  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = () => {
      if (mode === "system") {
        applyTheme(getSystemTheme());
      }
    };
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, [mode, applyTheme]);

  // Apply theme on mode change
  useEffect(() => {
    applyTheme(getResolvedTheme(mode));
  }, [mode, applyTheme]);

  const setTheme = useCallback((newMode: ThemeMode) => {
    localStorage.setItem(STORAGE_KEY, newMode);
    setMode(newMode);
  }, []);

  return { mode, resolved, setTheme };
}
