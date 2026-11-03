import type { ReactNode } from "react";

export type StampTone = "green" | "red" | "amber" | "ink";

export function Stamp({
  tone = "green",
  children,
  rotate = true,
}: {
  tone?: StampTone;
  children: ReactNode;
  rotate?: boolean;
}) {
  return (
    <span className={`stamp stamp-${tone} ${rotate ? "stamp-tilt" : ""}`}>
      {children}
    </span>
  );
}

const stateMap: Record<string, { label: string; tone: StampTone }> = {
  draft: { label: "Draft", tone: "ink" },
  open: { label: "Open", tone: "ink" },
  submitted: { label: "Submitted", tone: "amber" },
  verifying: { label: "Verifying", tone: "amber" },
  needs_review: { label: "Review", tone: "amber" },
  released: { label: "Released", tone: "green" },
  bounced: { label: "Blocked", tone: "red" },
};

export function StateStamp({ state }: { state: string }) {
  const item = stateMap[state] ?? { label: state, tone: "ink" as StampTone };
  return (
    <Stamp tone={item.tone} rotate={false}>
      {item.label}
    </Stamp>
  );
}
