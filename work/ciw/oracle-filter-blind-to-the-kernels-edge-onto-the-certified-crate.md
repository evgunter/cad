---
id: oracle-filter-blind-to-the-kernels-edge-onto-the-certified-crate
kind: issue
title: the inari/MPFR oracle's path filter cannot see a change that makes the backend an unconditional kernel dependency
status: open
opened: 2026-09-21
priority: P3
cost: E
refs: [ring-1-interval-type-ungated]
---


## What

`interval oracle (certify vs inari+MPFR)` is keyed on paths, not on
tier: `ORACLE_PATHS` in `scripts/ci-filter.py` is
`interval-transcendentals/{src/,tests/,Cargo.toml,Cargo.lock}`, and the
reason is costed in the comment above it — the job builds GMP and MPFR
from C source, ~234 s of its ~250 s, so it fires on the certified code
and its dependency pinning and nothing else (2 of the last 400
first-parent merges).

RING-1 (PR #2971) changed `crates/geom-core/Cargo.toml` so that
`interval-transcendentals` is a NORMAL, unconditional dependency of the
kernel's bottom crate where it had been optional plus a dev entry. The
oracle job skipped on that run (35555608396 and 35559102712, both
`skipped`), correctly per the filter as written — no path under the
backend moved.

The question this row is for: should the filter fire on a change to
the EDGE, as opposed to a change to the certified code? Two readings,
both defensible:

- **No.** The filter's subject is "did the certified code or its
  pinning move", and a dependent's manifest cannot change what the
  backend computes. The oracle would re-certify identical code at
  C-build cost.
- **Yes, for this shape.** What changed is WHO ships the crate: it is
  now in every default kernel build rather than an opt-in one, so the
  blast radius of an oracle-detectable defect grew even though the
  code did not. A run at the moment the edge becomes unconditional is
  one run, not a new standing cost.

Deciding it needs CIW's view on what the filter's subject is; the fix
if the answer is "yes" is a narrow arm on the dependent manifests that
declare the edge (`crates/geom-core/Cargo.toml`, root
`Cargo.toml`'s `[workspace.dependencies]` entry), not a tier change.

## Where

- `scripts/ci-filter.py`, `ORACLE_PATHS` and `_touches_oracle`
- `.github/workflows/ci.yml`, the `interval oracle (certify vs
  inari+MPFR)` job
- found by RING-1's review (R2 NOTE-6)
