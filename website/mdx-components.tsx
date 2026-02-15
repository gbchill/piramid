import type { MDXComponents } from "mdx/types";

const Callout = ({ title, children }: { title: string; children: React.ReactNode }) => (
  <div className="rounded-xl border border-indigo-400/30 bg-indigo-500/10 px-4 py-3 text-slate-100 shadow-lg shadow-indigo-900/30">
    <div className="text-sm font-semibold text-indigo-200">{title}</div>
    <div className="mt-1 text-sm text-slate-200">{children}</div>
  </div>
);

export const mdxComponents: MDXComponents = {
  Callout,
};
