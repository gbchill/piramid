"use client";

import { useEffect, useState } from "react";

type Heading = {
  id: string;
  text: string;
  level: number;
};

export function DocsToc({ headings }: { headings: Heading[] }) {
  const [activeId, setActiveId] = useState<string | null>(null);

  useEffect(() => {
    if (!headings || headings.length === 0) return;
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter((e) => e.isIntersecting)
          .sort((a, b) => (b.intersectionRatio || 0) - (a.intersectionRatio || 0));
        if (visible.length > 0) {
          setActiveId(visible[0].target.id);
        }
      },
      {
        rootMargin: "0px 0px -60% 0px",
        threshold: [0, 0.2, 0.4, 0.6, 0.8, 1],
      }
    );

    headings.forEach((h) => {
      const el = document.getElementById(h.id);
      if (el) observer.observe(el);
    });

    return () => observer.disconnect();
  }, [headings]);

  if (!headings || headings.length === 0) return null;

  return (
    <aside className="hidden xl:block w-64">
      <div className="sticky top-24 rounded-2xl border border-white/10 bg-white/5 p-4 shadow-lg shadow-slate-900/30 backdrop-blur space-y-3">
        <div className="text-xs font-semibold uppercase tracking-[0.18em] text-slate-400">On this page</div>
        <div className="space-y-1 text-sm">
          {headings.map((h) => (
            <a
              key={h.id}
              href={`#${h.id}`}
              className={`block rounded-lg px-2 py-1 transition ${
                h.level > 2 ? "pl-4 text-xs" : ""
              } ${
                activeId === h.id
                  ? "bg-indigo-500/15 text-white ring-1 ring-indigo-400/40"
                  : "text-slate-200 hover:bg-indigo-500/10 hover:text-white"
              }`}
            >
              {h.text}
            </a>
          ))}
        </div>
      </div>
    </aside>
  );
}
