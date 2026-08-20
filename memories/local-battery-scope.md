---
name: local-battery-scope
description: Local testing is an iteration-speed tool, not a gate — scope it by expected time-to-signal; hosted CI is the only gate
metadata:
  type: feedback
---

**The principle (Evan, 2026-08-01, stated after two boundary
cases in one day): "there's never any need for a hard and fast
rule on local CI. local testing is only useful insofar as it
speeds up iteration compared to waiting on CI."**

Hosted CI is THE gate — the merge gate is hosted Actions (the
nextest build-once/sharded matrix), `local-scripts/ci-local.sh`
is its mirror and `gate.sh` a billing-outage fallback only.

**It is also the CHEAP option, and that is the point.** The hosted
matrix runs its rows in parallel on GitHub hardware; the same rows
locally are serialized behind one box's cores, and a cold local
build alone can outlast the whole hosted matrix. Spare the local
machine
wherever practical: push and let the gate run. A local run is
justified exactly when it is likely to surface a failure faster
than pushing — that is the whole calculus. Corollaries:

- Feature work: the touched-crate suites at default ε (+ the
  Interval lane when the change is scalar-generic) usually pay
  for themselves — failures there are likely and the runs are
  minutes. That is the standard implementer brief.
- Cross-cutting mechanical sweeps: "touched crates" ≈ the whole
  workspace, the failure probability per suite is tiny, and the
  runs are hours — workspace `cargo check` + the unit's own
  self-test + a spot suite per converted-site class is the
  right buy. (Evan caught a lane over-running this live,
  2026-08-01.)
- A KNOWN gate failure: always reproduce locally first — the
  red→fix→re-push loop is fast precisely because the local
  reproduction is targeted.
- Standing pre-push CI mimicry (full-matrix or full-lint local
  rows "to be safe"): never — it was proposed after PR #152's
  triple red and declined ("doing ci locally was extremely
  slow"). Red gates are cheap; the gate exists to catch what
  narrowed runs miss.

**How to apply:** when writing an implementer brief, pick the
local scope by asking what failures are LIKELY for this change
shape and what runs surface them in minutes — write that, and
say "hosted CI proves the rest." Do not enumerate rules; apply
the principle. Related: [[agent-lane-operations]],
[[cad-working-style]].
