---
id: ring-1-interval-type-ungated
kind: unit
title: RING-1: geom_core::interval compiles unconditionally; the feature gates only the instantiation
status: closed
opened: 2026-09-21
closed: 2026-09-21
branch: scalar/ring-1
pr: 2971
---

## What

The first cut of `H5`'s ruling 1 (PR 2701, `## RATIFIED`): the
`Interval` scalar's module, arithmetic and impls compile in every build;
`interval-transcendentals` becomes a normal dependency of `geom-core`;
the `interval` feature keeps gating only the kernel's instantiation at
`Interval` (the lane impls in other crates and the gated test files)
until RING-3 drops it. Measured free (107 s vs 106 s clean workspace
build, 5 files, +2/−9 lines). Spec: `docs/RING-1-SPEC.md` (deleted at
merge). Block SCALAR-B4 slot 2 (OPUS). Ground: PROPS (`geom-core`),
CIW, GUARD; announced.

## Closed (2026-09-21) — PR 2971

`geom_core::interval` and its `Real`/`Decide`/`Bounds`/
`CertifiedEnclosure`/`SpanLocate` impls compile in every build;
`interval-transcendentals` is a normal dependency of `geom-core` (the
duplicate dev-entry removed); `interval = []` stays declared and gates
the lane-trait impls above `geom-core` and the interval test files
until RING-3 drops it. Seven type gates gone in `geom-core`, nine
`cfg(test)` sites stay; 48 sites in 23 other `src` files and 126 gated
test files are RING-3's starting count. Pin
`crates/geom-core/tests/interval_type_default_build.rs`, seven rows
ungated (both reviewers' probe rows adopted). `DESIGN.md` Q1's phrase
and the crate-landscape row re-worded (naming only, CLAUDE.md's
carve-out); `docs/GENERICS-BUILD-COST.md` §9, hosted numbers only (the
local re-take, 153 s vs 152 s on the shared box, stays in the PR body).
Four excluded-root lockfiles regenerated (the one MAJOR, bilateral,
pre-fixed at 30ca6a8697). Workspace dependency count 84 → 85; default
`geom-core` rlib carries 44 `interval` symbols where it carried 0; the
K roster does not move (a runtime sweep). Reviews: dual, both APPROVE
WITH FIXES; fourteen items, thirteen taken, one declined in two parts
with reason. Carried to RING-3 and LANE-1..4: the lane traits, not the
feature, are the whole remaining gate on the kernel doors at
`Interval`. Rows: `gate-on-the-type-in-prose-outside-geom-core`
(scalar), CIW `oracle-filter-blind-to-the-kernels-edge-onto-the-certified-crate`,
TINT `geom-core-all-rs-has-a-sorted-half-and-an-unsorted-one`.
