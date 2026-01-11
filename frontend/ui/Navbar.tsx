"use client";

import Link from "next/link";
import Image from "next/image";
import { Github, BookOpen, Sparkles } from "lucide-react";

export function Navbar() {
    return (
        <nav className="h-12 border-b border-white/10 bg-black/30 backdrop-blur-xl px-4 md:px-6 flex items-center justify-between">
            {/* Logo */}
            <Link href="/" className="flex items-center gap-3 hover:opacity-80 transition-opacity shrink-0">
                <div className="relative h-5 md:h-6" style={{ width: "auto", aspectRatio: "624/238" }}>
                    <Image
                        src="/wglymr_logo_full.svg"
                        alt="wglymr"
                        fill
                        priority
                        className="object-contain"
                    />
                </div>
            </Link>

            {/* Right Side Links */}
            <div className="flex items-center gap-2 md:gap-3">
                <Link
                    href="/docs"
                    className="hidden sm:flex items-center gap-1.5 px-2.5 py-1 text-xs text-gray-400 hover:text-white transition-colors"
                >
                    <BookOpen className="w-3.5 h-3.5" />
                    <span>Docs</span>
                </Link>
                <a
                    href="https://github.com/krispy-snacc/wglymr"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hidden sm:flex items-center gap-1.5 px-2.5 py-1 text-xs text-gray-400 hover:text-white transition-colors"
                >
                    <Github className="w-3.5 h-3.5" />
                    <span className="hidden md:inline">GitHub</span>
                </a>
                <Link
                    href="/glym/default"
                    className="flex items-center gap-1.5 px-2.5 md:px-3 py-1.5 bg-accent hover:bg-accent/80 rounded-lg text-xs font-medium text-white transition-colors"
                >
                    <Sparkles className="w-3.5 h-3.5" />
                    <span className="hidden sm:inline">Launch Editor</span>
                    <span className="sm:hidden">Editor</span>
                </Link>
            </div>
        </nav>
    );
}
