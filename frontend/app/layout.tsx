import type { Metadata, Viewport } from "next";
import "./globals.css";
import { SiteFooter, SiteHeader } from "@/components/SiteChrome";

export const metadata: Metadata = {
  title: "Tranche — Pay when it's proven",
  description:
    "Proof-gated milestone payments. Tranche compiles acceptance criteria into an executable graph, verifies evidence with SERV Reasoning and Reppo, and releases funds through Kite only when the proof passes.",
  metadataBase: new URL("https://tranche-serv.fly.dev"),
  openGraph: {
    title: "Tranche — Pay when it's proven",
    description:
      "Proof-gated milestone payments for grants, bounties, and hackathons. Built on SERV Reasoning, Reppo, and Kite Agent Passport.",
    type: "website",
  },
};

export const viewport: Viewport = {
  themeColor: "#f3f0e7",
  colorScheme: "light",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>
        <SiteHeader />
        <main>{children}</main>
        <SiteFooter />
      </body>
    </html>
  );
}
