import type { MDXComponents } from "mdx/types";

const Callout = ({ title, children }: { title: string; children: React.ReactNode }) => (
  <div className="rounded-xl border border-indigo-400/30 bg-indigo-500/10 px-4 py-3 text-slate-100 shadow-lg shadow-indigo-900/30">
    <div className="text-sm font-semibold text-indigo-200">{title}</div>
    <div className="mt-1 text-sm text-slate-200">{children}</div>
  </div>
);

export const mdxComponents: MDXComponents = {
  Callout,
  h1: (props) => <h1 className="text-4xl font-semibold tracking-tight text-white" {...props} />,
  h2: (props) => <h2 className="mt-10 text-3xl font-semibold tracking-tight text-white" {...props} />,
  h3: (props) => <h3 className="mt-8 text-2xl font-semibold text-slate-100" {...props} />,
  h4: (props) => <h4 className="mt-6 text-xl font-semibold text-slate-100" {...props} />,
  p: (props) => <p className="leading-7 text-slate-200" {...props} />,
  ul: (props) => <ul className="ml-6 list-disc space-y-1 text-slate-200" {...props} />,
  ol: (props) => <ol className="ml-6 list-decimal space-y-1 text-slate-200" {...props} />,
  li: (props) => <li className="leading-6 text-slate-200" {...props} />,
  a: (props) => (
    <a
      className="font-semibold text-indigo-300 underline decoration-indigo-400/60 underline-offset-4 hover:text-indigo-200 hover:decoration-indigo-200"
      {...props}
    />
  ),
  code: (props) => (
    <code
      className="rounded-md bg-slate-900/70 px-2 py-1 text-[13px] font-semibold text-indigo-100 ring-1 ring-white/10"
      {...props}
    />
  ),
  pre: (props) => (
    <pre
      className="overflow-x-auto rounded-2xl border border-white/10 bg-slate-950/80 p-4 text-sm text-slate-100 shadow-xl shadow-slate-900/40"
      {...props}
    />
  ),
  blockquote: (props) => (
    <blockquote
      className="border-l-4 border-indigo-400/50 bg-white/5 px-4 py-2 text-slate-100"
      {...props}
    />
  ),
  table: (props) => (
    <table
      className="w-full border-separate border-spacing-y-2 rounded-2xl border border-white/10 bg-white/5 text-sm text-slate-100"
      {...props}
    />
  ),
  th: (props) => (
    <th className="border-b border-white/10 px-3 py-2 text-left font-semibold text-white" {...props} />
  ),
  td: (props) => <td className="px-3 py-2 text-slate-200" {...props} />,
};
