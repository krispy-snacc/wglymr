import Link from "next/link";
import Image from "next/image";
import { Navbar } from "@/components/navbar/Navbar";
import { Sparkles, Workflow, Zap, Code2, Cpu } from "lucide-react";

export default function HomePage() {
  return (
    <div className="w-full text-white flex flex-col" style={{ minHeight: 'calc(var(--vh, 1vh) * 100)' }}>
      <Navbar />

      {/* Hero Section */}
      <main className="flex flex-col items-center px-4 sm:px-6 py-8 sm:py-12">
        <div className="max-w-5xl w-full space-y-5 sm:space-y-6">
          {/* Hero Content */}
          <div className="text-center space-y-3 sm:space-y-4">
            <div className="flex justify-center pt-4 sm:pt-8">
              <Image
                src="/wglymr_logo_full.svg"
                alt="wglymr logo"
                width={140}
                height={36}
                className="h-8 sm:h-9 w-auto opacity-90"
              />
            </div>
            <h1 className="text-2xl sm:text-4xl md:text-5xl font-bold tracking-tight px-4">
              <span className="bg-linear-to-r from-white via-gray-200 to-gray-400 bg-clip-text text-transparent">
                Visual Shader Editor
              </span>
            </h1>
            <p className="text-sm sm:text-base text-gray-400 max-w-2xl mx-auto px-4">
              Build stunning GPU shaders with an intuitive node-based interface.
              Real-time preview, WebGPU powered, and built for creative coding.
            </p>

            {/* CTA Buttons */}
            <div className="flex flex-col sm:flex-row items-center justify-center gap-3 pt-3 px-4">
              <Link
                href="/view/default"
                className="w-full sm:w-auto group flex items-center justify-center gap-2 px-5 sm:px-6 py-2.5 sm:py-3 bg-accent hover:bg-accent/90 rounded-lg text-sm sm:text-base font-semibold text-white transition-all hover:scale-105"
              >
                <Sparkles className="w-4 h-4" />
                <span>Start Creating</span>
              </Link>
              <Link
                href="/docs"
                className="w-full sm:w-auto flex items-center justify-center gap-2 px-5 sm:px-6 py-2.5 sm:py-3 border border-white/20 hover:border-white/40 rounded-lg text-sm sm:text-base font-semibold text-gray-300 hover:text-white transition-colors"
              >
                <Code2 className="w-4 h-4" />
                <span>Learn More</span>
              </Link>
            </div>
          </div>

          {/* Features Grid */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4 pt-3 sm:pt-4 px-4">
            <FeatureCard
              icon={<Workflow className="w-5 h-5 text-accent" />}
              title="Node-Based Editing"
              description="Intuitive visual programming with a powerful node graph editor"
            />
            <FeatureCard
              icon={<Zap className="w-5 h-5 text-accent" />}
              title="Real-Time Preview"
              description="See your shaders come to life instantly with WebGPU acceleration"
            />
            <FeatureCard
              icon={<Cpu className="w-5 h-5 text-accent" />}
              title="GPU Accelerated"
              description="Harness the full power of modern graphics APIs for maximum performance"
            />
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="mt-auto border-t border-white/10 py-4 px-6 text-center text-xs sm:text-sm text-gray-500">
        <p>wglymr - Visual Shader Editor</p>
      </footer>
    </div>
  );
}

function FeatureCard({
  icon,
  title,
  description,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
}) {
  return (
    <div className="group p-4 sm:p-5 rounded-lg border border-white/10 bg-white/5 hover:border-accent/50 hover:bg-white/10 transition-all">
      <div className="flex items-start gap-3">
        <div className="shrink-0 p-1.5 rounded-lg bg-accent/10 group-hover:bg-accent/20 transition-colors">
          {icon}
        </div>
        <div className="space-y-1">
          <h3 className="text-sm sm:text-base font-semibold text-white">{title}</h3>
          <p className="text-xs sm:text-sm text-gray-400">{description}</p>
        </div>
      </div>
    </div>
  );
}
