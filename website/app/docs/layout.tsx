import "../globals.css";
import Link from "next/link";
import type { ReactNode } from "react";
import { DocsSidebar } from "../../components/DocsSidebar";
import { DocsSearchLauncher } from "../../components/DocsSearchLauncher";
import { DocsSidebarMobile } from "../../components/DocsSidebarMobile";
import { buildSidebar, buildSearchIndex } from "../../lib/docs";

export default function DocsLayout({ children }: { children: ReactNode }) {
  const sidebar = buildSidebar();
  const searchEntries = buildSearchIndex();

  return (
    <div className="min-h-screen bg-[#05070d] text-slate-100">
      <header className="sticky top-0 z-20 backdrop-blur border-b border-white/5 bg-black/30">
        <div className="mx-auto flex max-w-6xl items-center justify-between px-4 sm:px-6 py-4">
          <div className="flex items-center gap-3">
            <Link href="/" className="flex items-center gap-3">
              <img src="/logo_light.png" alt="Piramid" className="h-9 w-9" />
              <div className="flex flex-col leading-tight">
                <span className="text-lg font-semibold tracking-wide">piramid</span>
              </div>
            </Link>
          </div>
          <div className="flex items-center gap-3 text-sm text-slate-300">
            <DocsSearchLauncher entries={searchEntries} />
          </div>
        </div>
      </header>
      <main className="mx-auto flex max-w-6xl flex-col gap-6 px-4 sm:px-6 py-8 lg:flex-row lg:gap-8 lg:py-10">
        <DocsSidebarMobile sections={sidebar} />
        <aside className="hidden lg:block w-64 flex-shrink-0">
          <DocsSidebar sections={sidebar} />
        </aside>
        <article className="flex-1">{children}</article>
      </main>
    </div>
  );
}
