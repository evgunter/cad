---
id: main-red-the-copysign-census-since-the-offset-conic-arc
kind: issue
title: Main red: the copysign census row since PR 5692's offset conic arc added three unregistered copysign sites in intersect/conic_arc.rs
status: closed
pr: 5874
opened: 2026-10-26
closed: 2026-10-27
priority: P1
cost: E
refs: [main-red-the-copysign-census-since-the-apex-tangent-generator]
---


Found by the SPLICE nightly triage of run 40274924857 (main
`38f1a24ed3`). `geom-core sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these`
(`crates/geom-core/tests/sym_rule_f_rows.rs`) is red at every ε row.
It was green on 2026-10-25 (run 40165703340):

    the copysign mint sites moved: a site appeared or went.
      left: {…, "crates/geom-brep/src/intersect/conic_arc.rs": 3, …}
     right: {… no conic_arc.rs entry …}

**Bisected.** The row passes at `d7bdade36b^1` and fails at
`d7bdade36b`, the merge of #5692 (`offset/conic-arc-and-axial-section`).
That PR adds `crates/geom-brep/src/intersect/conic_arc.rs` with three
`.copysign(` sites:
- `delta_sigma: delta * T::one().copysign(tilt.c)`;
- `… * T::one().copysign(self.delta_sigma)` in the seed's side;
- `root.copysign(self.l)` in the stable quadratic root.

Neither the census row's expected table nor
`crates/geom-core/src/sym/manifest.rs`'s header list names them.

**Owed** (the row's own remedy, the same as the closed
`main-red-the-copysign-census-since-the-apex-tangent-generator`):
re-ask the census (`m10_10_evidence_interval`'s
`sym12_the_copysign_census_at_the_nominal`) on the measured
documents, then move the table and the manifest's list together.
First read each site, per `work/germ/log.md`'s 2026-09-26 class
note: at `Interval`, `tilt.c`, `delta_sigma` and `l` can each straddle
zero, and wherever the sign picks a branch it may want a decision
rather than a `copysign`. Until this lands, every PR whose gate runs
geom-core reds on this one row.

## Closed

The three sites are read and registered: `conic_arc.rs` enters the
census row's expected table at 3, and `sym/manifest.rs`'s header list
names each site. None is a defect. `Interval::copysign` hulls a sign
that straddles zero, so no site can name a branch on an undecided
sign. A straddle only widens:
- `δσ` names one of two mirror-image vertices near. Either sign of a
  zero `c` serves the same pair of branches.
- The seed's side is decided after the transfer, so a straddling `δσ`
  refuses.
- The stable root's sign is that of the branch's own `L`.

At `f64` each sign is exact, and each site reads the same `δσ` as the
vertex placement. The new `plane_cone_arc` row
`a_seed_names_its_own_nappes_branch_on_either_lean` pins the
consistency across a lean of either sign. It is red under two mutants
(the side signed by `δ`, `σ` dropped), which the rest of the file
passed.

Census re-asked on the seven measured documents, rules shipped and
`no_f`: no `copysign` atom on any. The manifest's early-form counts
moved to the measured ones (the plate 8, the bracket 46, the link 44,
the D-tabs 68 and 72).

