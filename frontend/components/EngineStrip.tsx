"use client";

import { useEffect, useState } from "react";
import { getJSON } from "@/lib/api";
import type { HealthResponse } from "@/lib/types";

type Dot = "live" | "warn" | "off";

function Dot({ tone }: { tone: Dot }) {
  return <span className={`dot dot-${tone}`} aria-hidden="true" />;
}

export function EngineStrip({ paymentRail }: { paymentRail?: string }) {
  const [health, setHealth] = useState<HealthResponse | null>(null);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    let active = true;
    getJSON<HealthResponse>("/api/health")
      .then((data) => {
        if (active) setHealth(data);
      })
      .catch(() => {
        if (active) setFailed(true);
      });
    return () => {
      active = false;
    };
  }, []);

  const engines = health?.engines;
  const rail = paymentRail ?? engines?.payment_rail;

  return (
    <div className="engine-strip" aria-live="polite">
      <div className="engine-item">
        <span className="engine-name">SERV Reasoning</span>
        <span className="engine-state">
          <Dot tone={failed ? "off" : engines ? (engines.serv === "live" ? "live" : "warn") : "warn"} />
          {failed ? "API offline" : engines ? (engines.serv === "live" ? `live · ${engines.serv_model}` : "key required") : "checking"}
        </span>
      </div>
      <div className="engine-item">
        <span className="engine-name">Reppo Eval</span>
        <span className="engine-state">
          <Dot
            tone={
              failed
                ? "off"
                : engines
                  ? engines.reppo === "live_lite"
                    ? "live"
                    : engines.reppo === "sandbox"
                      ? "warn"
                      : "warn"
                  : "warn"
            }
          />
          {failed
            ? "API offline"
            : engines
              ? engines.reppo === "live_lite"
                ? "live · x402 credits"
                : engines.reppo === "sandbox"
                  ? "sandbox verdict"
                  : "not configured"
              : "checking"}
        </span>
      </div>
      <div className="engine-item">
        <span className="engine-name">Settlement</span>
        <span className="engine-state">
          <Dot
            tone={
              failed
                ? "off"
                : engines
                  ? rail === "kite_passport"
                    ? "live"
                    : "warn"
                  : "warn"
            }
          />
          {failed
            ? "API offline"
            : engines
              ? rail === "kite_passport"
                ? "Kite Passport"
                : "demo ledger"
              : "checking"}
        </span>
      </div>
      <div className="engine-item">
        <span className="engine-name">Script checks</span>
        <span className="engine-state">
          <Dot tone={failed ? "off" : "live"} />
          {failed ? "API offline" : "deterministic"}
        </span>
      </div>
    </div>
  );
}
