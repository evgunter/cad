---
id: the-from-f64-gate-reads-every-generic-crate
kind: issue
title: The from_f64 audit and its gate cover sweep/topo/geom/geom-brep only; profile, editor-core and geom-core also run generic code at Sym
status: parked
opened: 2026-10-10
priority: P2
cost: M
blocked_on: [a-computed-value-re-enters-as-a-constant]
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
