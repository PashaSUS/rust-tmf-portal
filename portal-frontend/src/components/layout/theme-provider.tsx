import { createContext, useContext, useEffect, useState, type ReactNode } from "react";

type Theme = "dark" | "light" | "system";

interface ThemeCtx {
  theme: Theme;
  setTheme: (t: Theme) => void;
}

const ThemeContext = createContext<ThemeCtx>({ theme: "system", setTheme: () => {} });

export function ThemeProvider({ children, defaultTheme = "system", storageKey = "tmf-portal-theme" }: {
  children: ReactNode;
  defaultTheme?: Theme;
  storageKey?: string;
}) {
  const [theme, setTheme] = useState<Theme>(
    () => (localStorage.getItem(storageKey) as Theme) || defaultTheme
  );

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove("light", "dark");

    if (theme === "system") {
      const sys = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
      root.classList.add(sys);
    } else {
      root.classList.add(theme);
    }
  }, [theme]);

  return (
    <ThemeContext.Provider value={{ theme, setTheme: (t) => { localStorage.setItem(storageKey, t); setTheme(t); } }}>
      {children}
    </ThemeContext.Provider>
  );
}

export const useTheme = () => useContext(ThemeContext);
