---
id: rim-level-rule-manufactures-its-error-by-feeding-nan-into-classify
kind: issue
title: The rim-level rule's structurally-impossible arm throws by feeding f64::NAN into classify, and unreachable_zero returns a 4-tuple of NaNs into live flux arithmetic
status: open
opened: 2026-09-11
refs: [877, S40]
---


## Carved out of `S40` (2026-09-11, by the WIRE orchestrator)

`S40` ("Residue and editing artifacts", accepted by Ev 2026-08-18)
was one file carrying four unrelated residues on three programs'
territory. WIRE inherited it whole in the cut of 2026-09-11 and owns
only one of the four; the rest are filed where the code lives, per
`work/README.md` ("when the owning program is clear, file the item
straight onto that program's slate"). `S40` keeps its id and its
`WitnessSlot` row; this is the second of its four bullets, verbatim
below. **Nothing about it was re-judged in the move** — the citations
are `S40`'s, written against a tree `#877` has since moved, and the row
was last read on 2026-08-18.

## Finding (`S40` bullet 2, verbatim)

- **Confidence**: sure

The rim-level rule's structurally-impossible arm manufactures its error
by feeding `f64::NAN` into `classify` and letting the funnel escalate —
a decision predicate used as a `throw`; `unreachable_zero` returns a
4-tuple of NaNs into live flux arithmetic (`props/curved.rs`,
`mixed_levels` and `unreachable_zero` — cited by target name per
**S176(a)**; **`same_level` no longer exists**, the two rim-level
spellings having been unified into `level_coincides` by **#877 / S81**,
and this bullet named it). **STILL OPEN** — the idiom survived the
unification unchanged, it is now at one site instead of two, and it is
D2 (bug-vs-invalid-state) territory.

## Why it is PROPS's

`crates/geom-brep/src/props/*` is PROPS's glob and no other open
program claims it. `S40`'s own verdict scoped this row out in 2026-08-19
as "a design call or belonging to a later wave"; the wave it named was
`S40`'s own, and `S40`'s program is gone. The D2 taxonomy is
`docs/DESIGN.md`'s (the bug-vs-invalid-state addendum, ~line 846), so
the call is PROPS's to make against it, not a ruling to escalate.
