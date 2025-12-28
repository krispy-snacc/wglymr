import type { Config } from "tailwindcss";
import { THEME } from "./app/theme";

const config: Config = {
    content: [
        "./app/**/*.{js,ts,jsx,tsx}",
        "./components/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                accent: THEME.accent,
                glass: "rgba(255,255,255,0.06)",
            },
        },
    },
    plugins: [],
};

export default config;
