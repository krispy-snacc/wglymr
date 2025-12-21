import type { Metadata, Viewport } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import { AnimatedBackground } from "@/components/layout/AnimatedBackground";
import { BACKGROUND_COLOR, THEME } from "./theme";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "wglymr - Shader Editor",
  description: "Web-based shader editor with visual node programming",
  icons: {
    icon: [
      { url: "/wglymr_logo_short.svg", type: "image/svg+xml" },
    ],
    apple: "/wglymr_logo_short.svg",
  },
};

export const viewport: Viewport = {
  themeColor: BACKGROUND_COLOR,
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        <link rel="icon" href="/wglymr_logo_short.svg" type="image/svg+xml" />
        <meta name="theme-color" content={BACKGROUND_COLOR} />
        <style dangerouslySetInnerHTML={{
          __html: `
            :root {
              --background: ${THEME.background};
              --foreground: ${THEME.foreground};
              --accent: ${THEME.accent};
              --accent-hover: ${THEME.accentHover};
              --border: ${THEME.border};
              --border-hover: ${THEME.borderHover};
              --panel-bg: ${THEME.panelBg};
              --text-secondary: ${THEME.textSecondary};
              --text-muted: ${THEME.textMuted};
            }
          `
        }} />
        <script
          dangerouslySetInnerHTML={{
            __html: `
              function setVH() {
                const vh = window.innerHeight * 0.01;
                document.documentElement.style.setProperty('--vh', vh + 'px');
              }
              setVH();
              window.addEventListener('resize', setVH);
              window.addEventListener('orientationchange', setVH);
            `,
          }}
        />
      </head>
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
        style={{ backgroundColor: BACKGROUND_COLOR }}
        suppressHydrationWarning
      >
        <AnimatedBackground />
        {children}
      </body>
    </html>
  );
}
