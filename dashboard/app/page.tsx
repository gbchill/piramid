"use client";

import Image from 'next/image';

export default function Dashboard() {
  return (
    <div className="min-h-screen bg-[var(--bg-primary)] text-[var(--text-primary)] flex items-center justify-center">
      <div className="text-center max-w-2xl px-6">
        <div className="mb-8">
          <Image 
            src="/logo_dark.png" 
            alt="Piramid Logo" 
            width={400} 
            height={100}
            priority
            className="mx-auto"
          />
        </div>
        <p className="text-xl text-[var(--text-secondary)] mb-8">
          Coming Soon
        </p>
        <div className="bg-[var(--bg-secondary)] border border-[var(--border-color)] rounded-lg p-6 text-left">
          <p className="text-[var(--text-secondary)] mb-4">
            The dashboard is currently under development while we focus on completing the core Rust engine.
          </p>
          <p className="text-[var(--text-secondary)]">
            In the meantime, you can interact with Piramid via the REST API at{' '}
            <code className="bg-[var(--bg-tertiary)] px-2 py-1 rounded text-[var(--accent)]">
              http://localhost:6333/api
            </code>
          </p>
        </div>
      </div>
    </div>
  );
}
