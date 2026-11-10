---
id: the-large-k-pin-cannot-see-a-new-rule-1-family-inside-its-count
kind: issue
title: k-lint (dev-probe): the pinned large-K step cannot see a new rule-1 family land inside its 3997 in-band samples
status: open
opened: 2026-11-10
priority: P3
cost: M
design: true
refs: [k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs]
---


Found by INSTR's `instr/klint-large-k` lane while it routed the large-K
gate's flags.

## What

The nightly's `large-K lint` step is pinned red. It must exit 2, it
prints the per-ε counts by rule in a `::warning`, and it reds on exit 0
or 1 (`.github/workflows/nightly.yml`, `k-lint` job). Rule 1 stands at
3997 in-band samples, all in families that already have rows. A new
in-band landing elsewhere — the trigger E6 names for re-opening K —
would show only as a slightly larger count in that warning. Nothing
reds on it, and the CLI prints at most 200 flags per file
(`tools/k-lint/src/main.rs`, `CAP`), so even the log may not name the
new family.

## The question

One fix is a per-predicate flag tally, printed on every run the way
the zero-arm tally is, with the pin redding on any rule-1 family that
is not on the routed list. That conflicts with the pin's stated
posture: "the nightly's `large-K lint` step pins the gate's verdict,
not its counts" (the P1 row's "Pinned red" section). A family list in
the wiring is a pinned count in all but name, and a list of known
families sits close to the name-keyed exemption that
`k-lint-rule-1-prose-assumes-every-in-band-site-refuses` rules out for
rule 1.

Is the blind spot worth closing while the pin stands, and if so, should
the guard be a tally in the CLI, a family list in the wiring, or
something else? The question goes away when the row's closing
conditions land and the step becomes a plain gate again.
