---
id: the-villarceau-strip-oracle-finds-no-family-circle-for-seed-3d14ecd0
kind: issue
title: torus_region's Villarceau strip fuzz row panics in its own oracle (no family circle found) at seed 0x3d14ecd04e99c48b
status: closed
opened: 2026-10-19
closed: 2026-10-19
priority: P2
pr: 5363
---


`topo boolean::torus_region::tests::a_villarceau_strip_reads_its_side_at_random_points`
(`crates/topo/src/boolean/torus_region.rs`) panics at its own oracle's
assert (`assert!(best.0 < 1e-9, "oracle {best:?}")`, ~line 1166) with
`oracle (inf, 0.0)` at fuzz seed `0x3d14ecd04e99c48b`: the `fam` closure
scans 20000 brackets of φ for a sign change of `g(φ) = rot(φ, n0)·w` and
finds none, so no family circle is located for the sampled point. The
kernel is not reached; the failure is the test's oracle.

- Seen on hosted CI on PR 5333 (FLUX, run 39229239643, `[eps 1e-6]`),
  whose diff does not touch `topo::boolean`.
- Reproduces on main alone (`origin/main` at filing):
  `CAD_FUZZ_SEED=0x3d14ecd04e99c48b cargo nextest run -p topo --lib
  --ignore-default-filter -E 'test(a_villarceau_strip_reads_its_side_at_random_points)'`.
- Likely cause, unverified: a sampled point where `g` touches zero
  without changing sign (a double root, the point on the strip's
  boundary circle or at a tangency), or a point the sampler places off
  every family circle. Either the sampler must keep its points where the
  oracle's bracket scan is valid, or the oracle must find a tangential
  root (minimise `on` over φ rather than bracket `g`).

## Closed

PR 5363 fixes the same oracle defect as `villarceau-strip-fuzz-oracle-misses-a-root`. `fam` now computes the family parameter in closed form instead of scanning brackets for a sign change. Seed `0x3d14ecd04e99c48b` passes on that branch at ε 1e-9 and 1e-6 (replayed by the RESTREAD orchestrator).
