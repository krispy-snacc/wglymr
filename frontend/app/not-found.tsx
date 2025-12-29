import Link from "next/link";
import { Navbar } from "@/ui/Navbar";
import { Home, ArrowLeft, Search } from "lucide-react";

export default function NotFound() {
    return (
        <div className="w-full flex flex-col bg-black text-white overflow-x-hidden" style={{ minHeight: 'calc(var(--vh, 1vh) * 100)' }}>
            <Navbar />

            <main className="flex-1 flex items-center justify-center px-4 sm:px-6">
                <div className="max-w-2xl w-full text-center space-y-6 sm:space-y-8">
                    {/* 404 Display */}
                    <div className="space-y-3 sm:space-y-4">
                        <h1 className="text-7xl sm:text-9xl font-bold bg-linear-to-r from-accent via-pink-500 to-purple-500 bg-clip-text text-transparent">
                            404
                        </h1>
                        <h2 className="text-2xl sm:text-3xl font-semibold text-gray-200">
                            Shader Not Found
                        </h2>
                        <p className="text-base sm:text-lg text-gray-400 px-4">
                            Looks like this shader graph got lost in the pipeline.
                        </p>
                    </div>

                    {/* Decorative Element */}
                    <div className="flex items-center justify-center gap-2 py-8">
                        <div className="w-2 h-2 rounded-full bg-accent animate-pulse" />
                        <div className="w-2 h-2 rounded-full bg-accent animate-pulse [animation-delay:0.2s]" />
                        <div className="w-2 h-2 rounded-full bg-accent animate-pulse [animation-delay:0.4s]" />
                    </div>

                    {/* Action Buttons */}
                    <div className="flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4 px-4">
                        <Link
                            href="/"
                            className="w-full sm:w-auto flex items-center justify-center gap-2 px-6 py-3 bg-accent hover:bg-accent/90 rounded-lg text-sm font-medium text-white transition-colors"
                        >
                            <Home className="w-4 h-4" />
                            <span>Go Home</span>
                        </Link>
                        <Link
                            href="/glym/default"
                            className="w-full sm:w-auto flex items-center justify-center gap-2 px-6 py-3 border border-white/20 hover:border-white/40 rounded-lg text-sm font-medium text-gray-300 hover:text-white transition-colors"
                        >
                            <Search className="w-4 h-4" />
                            <span>Open Editor</span>
                        </Link>
                    </div>

                    {/* Helpful Links */}
                    <div className="pt-8 text-sm text-gray-500">
                        <p>Need help? Check out our <Link href="/docs" className="text-accent hover:underline">documentation</Link></p>
                    </div>
                </div>
            </main>

            <footer className="border-t border-white/10 py-8 px-6 text-center text-sm text-gray-500">
                <p>wglymr - Visual Shader Editor</p>
            </footer>
        </div>
    );
}
