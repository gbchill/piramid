import "../globals.css";
import Link from "next/link";
import type { ReactNode } from "react";

export default function DocsLayout({ children }: { children: ReactNode }) {
  return (
    <div className="min-h-screen bg-[#05070d] text-slate-100">
      <header className="sticky top-0 z-20 backdrop-blur border-b border-white/5 bg-black/30">
        <div className="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
          <div className="flex items-center gap-3">
            <Link href="/" className="flex items-center gap-3">
              <img src="/logo_light.png" alt="Piramid" className="h-9 w-9" />
              <div className="flex flex-col leading-tight">
                <span className="text-lg font-semibold tracking-wide">piramid</span>
                <span className="text-xs text-slate-400">Docs</span>
              </div>
            </Link>
          </div>
          <div className="flex items-center gap-3 text-sm text-slate-300">
            <Link href="/docs" className="hover:text-white transition">Docs home</Link>
            <a href="https://github.com/ashworks1706/piramid" className="hover:text-white transition">GitHub</a>
            <a href="https://crates.io/crates/piramid" className="hover:text-white transition">crates.io</a>
          </div>
        </div>
      </header>
      <main className="mx-auto max-w-4xl px-6 py-10 prose prose-invert prose-slate prose-headings:text-white prose-a:text-indigo-200">
        {children}
      </main>
    </div>
  );
}
