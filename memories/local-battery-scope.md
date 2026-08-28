---
name: local-battery-scope
description: Local testing is an iteration-speed tool, not a gate — hosted CI is the only gate and the cheap option, and the only producer of committed measurements
metadata:
  type: feedback
---

**The principle (Evan): "there's never any need for a hard and fast
rule on local CI. local testing is only useful insofar as it speeds up
iteration compared to waiting on CI."**

Hosted Actions is THE merge gate (the nextest build-once/sharded
matrix); `local-scripts/ci-local.sh` mirrors it and `gate.sh` is a
billing-outage fallback only. **It is also the CHEAP option**: the
hosted matrix runs its rows in parallel on GitHub hardware, while the
same rows locally serialize behind one box's cores and a cold local
build alone can outlast the whole hosted matrix. A local run is
justified exactly when it is likely to surface a failure faster than
pushing — that is the whole calculus.

- Feature work: the touched-crate suites at default ε (plus the
  Interval lane when the change is scalar-generic). That is the
  standard implementer brief.
- Cross-cutting mechanical sweeps: "touched crates" ≈ the workspace and
  per-suite failure probability is tiny — workspace `cargo check` + the
  unit's own self-test + a spot suite per converted-site class.
- A KNOWN gate failure: always reproduce locally first; the red→fix→
  re-push loop is fast precisely because the reproduction is targeted.
- Standing pre-push CI mimicry "to be safe": never (Evan declined it —
  "doing ci locally was extremely slow"). Red gates are cheap; the gate
  exists to catch what narrowed runs miss.

**Committed measurements come from hosted CI only.** A timing or a
rendered frame is worth nothing without the box that produced it, so
one reproducible box class produces every committed number and every
sample carries its own environment block; local runs read that history
and report against it, and are never committed
([[freecad-render-lane]]). Where an exact deterministic counter exists
(predicate decision counts, cell counts), reach for it before a
stopwatch.

**How to apply:** when writing an implementer brief, pick the local
scope by asking what failures are LIKELY for this change shape and what
runs surface them in minutes — write that, and say "hosted CI proves
the rest." Do not enumerate rules; apply the principle. Related:
[[agent-lane-operations]], [[cad-working-style]].
