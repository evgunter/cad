# M5 S7 — CI/docs hygiene sweep (binding spec; Evan-directed 2026-07-30)

Three Evan rulings executed together. Branch `ev/m5-s7-ci-docs`
from main. Small unit; touches ci.yml (workflow scope at merge is
available).

## 1. Retire the stale gmp/LGPL language

The kernel's interval backend is in-house; gmp/inari/LGPL is no
longer a live concern and the "gmp-free"/"copyleft-free"
trumpeting reads as noise. Sweep:
- CI job name "interval backend crate (gmp-free)" → drop the
  parenthetical.
- DESIGN.md / other docs: phrases specifically asserting
  lgpl-free / gmp-free / copyleft-free status as a selling point
  are removed or reduced to plain statements of what the code IS
  (present-tense doc discipline; history stays in logs). The
  factual record (interval-transcendentals keeps its gmp-backed
  dev-oracle in its own excluded workspace, never in kernel
  builds) stays stated ONCE where it matters — the crate-table
  row / quarantine note — without the campaign language.
- Grep-verify: no remaining "gmp-free"/"LGPL-free"/"copyleft-
  free" outside logs (docs/M*-LOG.md are the historical record —
  untouched).

## 2. Drop the ε = 1e-9 CI rows (ruling: 1e-6 and 1e-12 straddle it)

- Remove eps=1e-9 from every hosted matrix (test, band 4 corpus,
  persistence) and from ci-local.sh's row sets.
- Sync the battery-convention text: "3ε" language in DESIGN.md
  (convention row) and any spec/doc boilerplate becomes the
  two-ε set {1e-6, 1e-12} (+ default ε row where the default is
  distinct); note the ruling date. Do NOT rewrite historical
  logs/specs.
- The tier-aware filter/floors need no row-count changes (the
  watcher semantics are no-fail + filter-pass), but update any
  comment naming the old row counts.

## 3. Interval-lane job cost

- Keep the `interval` feature flag and the separate job (a
  different build graph is the point — the lane carries the
  certification-heavy suites).
- Reduce its wall-clock where cheap and safe: verify the
  rust-cache key separates feature sets (a shared key thrashes;
  a missing key rebuilds cold every run) and fix if wrong;
  OPTIONALLY split the job like the ε matrix if the split is a
  pure YAML change (no new scripts). Report measured/expected
  effect; do not restructure beyond this.

## 4. Acceptance

- Hosted CI green on the PR with the new row set (this PR's own
  run demonstrates the 1e-9 removal).
- ci-local.sh --full agrees with the hosted set (shared
  classifier untouched).
- Greps in §1 clean; convention text synced; no semantic code
  change anywhere (docs + YAML + ci-local only).

## 5. Process

Standard orchestration rules (foreground, push per unit, OUTPUT
DISCIPLINE ≤30 lines). Review: lightweight — this is docs/CI
hygiene; one adversarial pass focused on (a) accidental semantic
deletions in swept docs, (b) matrix-row correctness, (c) cache-
key correctness.
