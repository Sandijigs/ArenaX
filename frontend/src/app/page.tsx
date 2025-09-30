"use client";

import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { ArrowRight, Menu } from "lucide-react";
import Image from "next/image";
import { useState } from "react";

export default function Home() {
  const [menuOpen, setMenuOpen] = useState(false);

  return (
    <div className="min-h-screen bg-black text-white flex flex-col">
      {/* Header */}
      <header className="w-full border-b border-green-500/30">
        <nav className="container mx-auto flex items-center justify-between py-4 px-4">
          {/* Logo */}
          <div className="flex items-center gap-2">
            <Image
              src="/icon-192x192.png"
              alt="ArenaX Logo"
              width={36}
              height={36}
            />
            <span className="text-xl md:text-2xl font-bold text-green-500">
              ArenaX
            </span>
          </div>

          {/* Desktop Nav */}
          <div className="hidden md:flex gap-4">
            <Button variant="ghost" className="text-white hover:text-green-400">
              Features
            </Button>
            <Button variant="ghost" className="text-white hover:text-green-400">
              Pricing
            </Button>
            <Button variant="ghost" className="text-white hover:text-green-400">
              About
            </Button>
            <Button className="bg-green-500 hover:bg-green-600 text-black font-semibold">
              Get Started
            </Button>
          </div>

          {/* Mobile Menu Toggle */}
          <button
            className="md:hidden p-2 text-green-500"
            onClick={() => setMenuOpen(!menuOpen)}
          >
            <Menu className="h-6 w-6" />
          </button>
        </nav>

        {/* Mobile Menu */}
        {menuOpen && (
          <div className="md:hidden flex flex-col px-4 pb-4 gap-2 border-t border-green-500/30 bg-black">
            <Button variant="ghost" className="text-white hover:text-green-400">
              Features
            </Button>
            <Button variant="ghost" className="text-white hover:text-green-400">
              Pricing
            </Button>
            <Button variant="ghost" className="text-white hover:text-green-400">
              About
            </Button>
            <Button className="bg-green-500 hover:bg-green-600 text-black font-semibold">
              Get Started
            </Button>
          </div>
        )}
      </header>

      {/* Hero Section */}
      <section className="flex-1 flex items-center justify-center text-center px-6 py-12">
        <div className="max-w-3xl">
          <h1 className="text-4xl md:text-6xl font-extrabold text-green-500 mb-6 leading-tight">
            Enter the Future of Competitive Gaming
          </h1>
          <p className="text-base md:text-lg text-gray-300 mb-8">
            ArenaX is your all-in-one esports platform for tournaments, rewards,
            and community. Compete, climb, and conquer with style.
          </p>
          <Button
            size="lg"
            className="bg-green-500 hover:bg-green-600 text-black font-bold w-full md:w-auto"
          >
            Join ArenaX <ArrowRight className="ml-2 h-5 w-5" />
          </Button>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 bg-zinc-950 px-6">
        <div className="container mx-auto grid grid-cols-1 md:grid-cols-3 gap-8">
          <Card className="bg-black border-green-500/20">
            <CardContent className="p-6 text-center">
              <h3 className="text-lg md:text-xl font-bold text-green-500 mb-3">
                Tournaments
              </h3>
              <p className="text-gray-400 text-sm md:text-base">
                Join exciting esports events, battle players worldwide, and win
                big rewards.
              </p>
            </CardContent>
          </Card>

          <Card className="bg-black border-green-500/20">
            <CardContent className="p-6 text-center">
              <h3 className="text-lg md:text-xl font-bold text-green-500 mb-3">
                Rewards
              </h3>
              <p className="text-gray-400 text-sm md:text-base">
                Earn tokens, prizes, and bragging rights as you climb the ArenaX
                ladder.
              </p>
            </CardContent>
          </Card>

          <Card className="bg-black border-green-500/20">
            <CardContent className="p-6 text-center">
              <h3 className="text-lg md:text-xl font-bold text-green-500 mb-3">
                Community
              </h3>
              <p className="text-gray-400 text-sm md:text-base">
                Connect with gamers, join teams, and build your legacy in the
                ArenaX world.
              </p>
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-green-500/30 py-6 text-center text-gray-400 text-sm md:text-base">
        <p>© {new Date().getFullYear()} ArenaX. All rights reserved.</p>
      </footer>
    </div>
  );
}
