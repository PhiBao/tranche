import Link from "next/link";
import { Wordmark } from "./Mark";

export function Ticker() {
  return (
    <div className="ticker">
      <div className="wrap ticker-inner">
        <span>
          <span className="ticker-dot" /> SERV Hackathon 02 · Kite Track
        </span>
        <span className="ticker-right">Nov 2–15, 2026 · Online</span>
      </div>
    </div>
  );
}

export function SiteHeader() {
  return (
    <>
      <Ticker />
      <header className="site-header">
        <div className="wrap header-inner">
          <Link href="/" aria-label="Tranche home">
            <Wordmark />
          </Link>
          <nav className="nav" aria-label="Primary">
            <Link href="/#how">How it works</Link>
            <Link href="/#stack">Stack</Link>
            <Link href="/console">Console</Link>
            <Link href="/new">New milestone</Link>
          </nav>
          <Link href="/console" className="button button-small header-cta">
            Open console <span aria-hidden="true">↗</span>
          </Link>
        </div>
      </header>
    </>
  );
}

export function SiteFooter() {
  return (
    <footer className="site-footer">
      <div className="wrap footer-grid">
        <div>
          <Wordmark />
          <p className="footer-note">
            Proof-gated milestone payments. Built on SERV Reasoning, Reppo, and
            Kite Agent Passport.
          </p>
        </div>
        <div className="footer-links">
          <div>
            <span className="footer-label">Product</span>
            <Link href="/console">Release room</Link>
            <Link href="/new">New milestone</Link>
            <Link href="/#how">How it works</Link>
          </div>
          <div>
            <span className="footer-label">Proof</span>
            <Link href="/#stack">Engine status</Link>
            <Link href="/#anatomy">Acceptance graph</Link>
            <Link href="/#blocked">Blocked payment</Link>
          </div>
          <div>
            <span className="footer-label">Hackathon</span>
            <a href="https://www.openserv.ai/hackathon-2" target="_blank" rel="noreferrer">
              SERV Hackathon 02 ↗
            </a>
            <a href="https://docs.openserv.ai" target="_blank" rel="noreferrer">
              SERV docs ↗
            </a>
            <a href="https://docs.gokite.ai/passport" target="_blank" rel="noreferrer">
              Kite Passport ↗
            </a>
          </div>
        </div>
      </div>
      <div className="wrap footer-bottom">
        <span>© 2026 Tranche</span>
        <span className="mono">Pay when it&apos;s proven.</span>
      </div>
    </footer>
  );
}
