/**
 * This script syncs theme values from theme.ts to globals.css
 * Run this whenever you update theme.ts values
 */

import { readFileSync, writeFileSync } from "fs";
import { THEME } from "../app/theme";

const cssPath = "./app/globals.css";
const css = readFileSync(cssPath, "utf-8");

// Replace CSS custom properties with theme values
const updatedCss = css.replace(
    /:root\s*{[^}]*}/,
    `:root {
    /* Theme colors - auto-synced from theme.ts */
    --background: ${THEME.background};
    --foreground: ${THEME.foreground};
    --accent: ${THEME.accent};
    --accent-hover: ${THEME.accentHover};
    --border: ${THEME.border};
    --border-hover: ${THEME.borderHover};
    --panel-bg: ${THEME.panelBg};
    --text-secondary: ${THEME.textSecondary};
    --text-muted: ${THEME.textMuted};
    
    /* Mobile viewport height fix */
    --vh: 1vh;
}`
);

writeFileSync(cssPath, updatedCss);
console.log("✅ Theme synced to globals.css");
