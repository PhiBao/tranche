export function Mark({ className = "" }: { className?: string }) {
  return (
    <svg
      className={className}
      viewBox="0 0 32 32"
      fill="none"
      aria-hidden="true"
      xmlns="http://www.w3.org/2000/svg"
    >
      <rect x="3" y="6" width="19" height="3.6" fill="currentColor" />
      <rect x="3" y="14.2" width="14" height="3.6" fill="currentColor" />
      <rect x="3" y="22.4" width="9" height="3.6" fill="currentColor" />
      <rect x="25" y="3" width="3.4" height="26" fill="var(--green)" />
    </svg>
  );
}

export function Wordmark({ compact = false }: { compact?: boolean }) {
  return (
    <span className="brand">
      <Mark className="brand-mark" />
      <span className="brand-word">{compact ? "TRANCHE" : "TRANCHE"}</span>
    </span>
  );
}
