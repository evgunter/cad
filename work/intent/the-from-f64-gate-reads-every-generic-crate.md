---
id: the-from-f64-gate-reads-every-generic-crate
kind: issue
title: The from_f64 audit and its gate cover sweep/topo/geom/geom-brep only; profile, editor-core and geom-core also run generic code at Sym
status: closed
opened: 2026-10-10
priority: P2
cost: M
closed: 2026-10-10
pr: 4596
branch: intent/from-f64-generic-crates
---


`scripts/gates/from-f64-allowlist.sh` scans `geom`, `geom-brep`, `sweep`
and `topo` (`FROM_F64_CRATES`), the four the audit was scoped to. Other
crates run generic code at `Sym` too and hold `from_f64` sites the audit
never read: `profile` (about 55 occurrences, the profile lowering the
symbolic lane registers arcs through), `editor-core` (about 35, the
evaluator and the driver) and `geom-core` (about 250, the spline algebra
and the `Real` impls themselves). Widen `FROM_F64_CRATES`, classify each
new site in `scripts/gates/from-f64-sites.tsv`, and file the owed ones
as this unit's fixes. Counts are `grep -w from_f64` over `src`, tests
included, at the merge of unit 1.

## Closed

PR #4596. `FROM_F64_CRATES` now reads `editor-core geom geom-brep geom-core pncad profile sweep topo verbs`. Every new site is classed in `scripts/gates/from-f64-sites.tsv`; geom-core's `Real` impls are classed `constant` as the door itself. What is owed has a row of its own:
- `editor-core-keeps-computed-values-in-t`: the mate lever arm, 6 sites.
- `the-pinned-profile-lift-launders-at-sym`: the pinned profile lift, 5 sites. That row also owes running the re-valuation rows under `Guided`.
- `nurbs-weights-and-knots-are-the-scalar`: geom-core's spline basis knot reads, 7 sites, classed `stored-carrier-data`.

The die P0 turned out to be a census pairing artefact, and the `revalue` instrument now keeps a mismatch after a sign flip apart (`after_flip`). `kitchen-sink-census-pairs-planes-out-of-order` carries the census reading the sign rule cannot see.
