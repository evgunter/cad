---
id: the-tours-k-probe-brackets-suite-runs-in-no-ci-row
kind: issue
title: demos/tour's k_probe_brackets suite is gated on the probe feature and no CI row builds the tour with it, so it runs nowhere; its 1e-12 row is red on main (lily's stem refuses inside plant at Probe)
status: open
opened: 2026-10-29
priority: P1
cost: E
---


Found by SPLICE's lily-one-loft lane (2026-10-29), which ran the suite
locally because its change moves the lily's long leaf.

## What

`demos/tour/tests/k_probe_brackets.rs` opens with
`#![cfg(feature = "probe")]`. The per-PR demos row and nightly's
`demos tour suite` step (`cd demos/tour && cargo nextest run --release
--no-fail-fast`) build the tour without `--features probe`, so the suite
compiles to nothing. Nightly's `dev-probe` row lists probe-gated test
targets only for the crates `scripts/gates/probe-suite-census.sh
--crates` names, and its sweep step runs `scripts/k_probe_sweep.sh`, not
this suite. So nothing runs it.

Run by hand (`cd demos/tour && cargo test --release --features probe
--test all k_probe_brackets`):

- `k_probe_brackets_the_lofted_leaf_and_the_sepals_at_eps_1e_12` is red
  on main, independent of the lily change: `probe.rs` sweeps
  `lily::plant::<Probe>` with no `lily::stem_frontier` check, and at
  1e-12 the lower stem's tube refuses as a kernel limit inside `plant`
  (`stem tube arc builds: ... AnalyticRung3(KernelLimit { operand:
  Torus, .. })`), so `demo-tour k-probe` exits 101.
- The 1e-9 and 1e-6 rows pass once the lily change re-pins its bracket
  set (`lily_leaf_a` now reads as a bracket there).

## What it needs

A row that builds the tour with `--features probe` and runs the suite,
and the 1e-12 row brought to the lily's declared frontier: the probe
sweep reads `stem_frontier` before building the plant, or the row stops
expecting the lily's bodies at 1e-12.
