export const THEME = {
    // Primary colors
    background: "#0d0410", // Main background color
    foreground: "#ffffff", // Main text color
    accent: "#ff5c8a", // Primary accent color (buttons, sliders, highlights)

    // Semantic colors
    accentHover: "#ff7ca3", // Accent hover state
    accentMuted: "#ff5c8a33", // Transparent accent for backgrounds

    // Grays
    border: "#ffffff1a", // Border color (white/10)
    borderHover: "#ffffff33", // Border hover (white/20)

    // Panel backgrounds
    panelBg: "#18090fe6", // Panel background (zinc-950/90)
    panelBgLight: "#ffffff0d", // Light panel overlay (white/5)

    // Text colors
    textPrimary: "#ffffff", // Primary text
    textSecondary: "#d1d5db", // Secondary text (gray-300)
    textMuted: "#9ca3af", // Muted text (gray-400)
    textDisabled: "#6b7280", // Disabled text (gray-500)
} as const;

// Export individual values for backward compatibility
export const BACKGROUND_COLOR = THEME.background;
export const ACCENT_COLOR = THEME.accent;
