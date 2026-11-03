export type Status = "pending" | "running" | "pass" | "fail" | "warn" | "skipped" | "blocked";

export function StatusMark({ status }: { status: Status }) {
  if (status === "running") {
    return (
      <span className="status-mark running" aria-label="Running">
        <span className="spinner" />
      </span>
    );
  }
  if (status === "pass") {
    return (
      <span className="status-mark pass" aria-label="Passed">
        <svg viewBox="0 0 12 12" aria-hidden="true">
          <path d="M2 6.2 4.8 9 10 3.4" fill="none" stroke="currentColor" strokeWidth="1.8" />
        </svg>
      </span>
    );
  }
  if (status === "fail" || status === "blocked") {
    return (
      <span className={`status-mark ${status}`} aria-label={status}>
        <svg viewBox="0 0 12 12" aria-hidden="true">
          <path d="M3 3l6 6M9 3 3 9" fill="none" stroke="currentColor" strokeWidth="1.8" />
        </svg>
      </span>
    );
  }
  if (status === "warn") {
    return (
      <span className="status-mark warn" aria-label="Needs attention">
        <svg viewBox="0 0 12 12" aria-hidden="true">
          <path d="M6 2v5" fill="none" stroke="currentColor" strokeWidth="1.8" />
          <circle cx="6" cy="9.2" r="1" fill="currentColor" />
        </svg>
      </span>
    );
  }
  return (
    <span className="status-mark pending" aria-label="Pending">
      <span className="dot" />
    </span>
  );
}

export function statusText(status: Status): string {
  switch (status) {
    case "pass":
      return "passed";
    case "fail":
      return "failed";
    case "blocked":
      return "blocked";
    case "warn":
      return "attention";
    case "running":
      return "running";
    case "skipped":
      return "skipped";
    default:
      return "waiting";
  }
}
