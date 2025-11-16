import Image from "next/image";

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col bg-black font-sans">
      {/* Navbar */}
      <nav className="fixed top-0 left-0 right-0 z-50 bg-black/80 backdrop-blur-sm border-b border-white/10">
        <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
          <Image
            src="/navbar.png"
            alt="Piramid"
            width={120}
            height={40}
            className="drop-shadow-lg"
          />
          
          <a
            href="https://github.com/ashworks1706/Piramid"
            target="_blank"
            rel="noopener noreferrer"
            className="text-sm font-regular text-gray-400 hover:text-white transition-colors"
          >
            GitHub
          </a>
        </div>
      </nav>

      {/* Main Content */}
      <main className="flex flex-1 flex-col items-center justify-center gap-8 px-6 pt-20">
        {/* Pyramid Logo */}
        <div className="flex flex-col items-center">
          <Image
            src="/logo.png"
            alt="Piramid Logo"
            width={240}
            height={280}
            priority
            className="drop-shadow-2xl"
          />
          
          <h1 className="text-6xl font-semibold tracking-wider text-white">
            piramid
          </h1>
        </div>

        {/* Coming Soon Text */}
        <div className="flex flex-col items-center gap-3 text-center">
          <p className="text-xl font-regular text-gray-300 tracking-wide">
            Rust Based Vector database for Agentic Applications
          </p>
          <p className="max-w-md text-sm font-regular text-gray-400 leading-relaxed">
            Coming Soon
          </p>
        </div>

        {/* CTA Buttons */}
        <div className="flex gap-4 mt-4">
          <a
            href="https://github.com/ashworks1706/Piramid"
            target="_blank"
            rel="noopener noreferrer"
            className="px-6 py-2.5 bg-white text-black text-sm font-normal rounded hover:bg-gray-200 transition-colors"
          >
            Get Started
          </a>
          <a
            href="https://github.com/ashworks1706/Piramid"
            target="_blank"
            rel="noopener noreferrer"
            className="px-6 py-2.5 border border-white/20 text-white text-sm font-light rounded hover:border-white/40 transition-colors"
          >
            Learn More
          </a>
        </div>
      </main>
    </div>
  );
}
