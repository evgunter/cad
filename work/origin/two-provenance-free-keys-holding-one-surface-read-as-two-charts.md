---
id: two-provenance-free-keys-holding-one-surface-read-as-two-charts
kind: issue
title: the loop-re-parenting doors read two keys holding one surface as two charts when no GeomSource ties them, and drop rows that were correct
status: open
opened: 2026-09-14
refs: [loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart]
priority: P0
cost: H
---

Found by both reviewers of PR 2549 (R1 by execution as a MAJOR, R2 as
a MINOR, on the same probe) and left open by that PR's fix pass, which
narrowed it.

`Body::same_chart` (`crates/topo/src/euler_ring.rs`) decides whether a
loop-re-parenting door carries or drops the moved loop's pcurve rows.
Its rungs are the merge door's two hard ones
(`Body::planes_declared_equal`, `crates/topo/src/merge_faces.rs`): one
surface key, or two keys carrying one `GeomSource` — plus the shared
`Arc` spelling of the second for the two payload kinds. Two keys
holding an EQUAL surface that no `GeomSource` ties together therefore
read as two charts, and the door drops rows that were correct.

**Measured** (PR 2549's head, the fixture of
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`, `validate_pcurves`
at `Band::linear(Tol::witness())`): splitting the sheet's two curved
panels onto two keys both holding `Surface::Cylinder` with the same
six scalars, then `kfmrh(low, up)` — `(4, 4)` and four
`MissingCache`, where the same move under one key is `(8, 0)` and
`[]`. `mint_pcurves` restores it to `(8, 0)`, `[]`. The row
`two_keys_holding_one_surface_with_no_provenance_read_as_two_charts`
pins exactly this, so the cost is measured and not a surprise.

**It is a cost, never a wrong row.** The door answers "not the same
chart" when it cannot see that they are, which drops rows that were
true; answering the other way when it is wrong would KEEP a row about
another surface, which is the defect the unit closed. Absent evidence,
this is the safe direction to be wrong in, and what it costs is a
re-mint.

**Why the fix pass did not close it, measured.** Deciding those two
keys equal means reading the two `Surface` values' scalars
structurally. This tree has exactly one production spelling of that
— `chart_region::surface_bits_equal`, the bracket-exact C6 read — and
it needs `geom_core::Bounds`, which `Decide` does not imply; the
retired bit-identity channel (`geom_core::bit_identity`, N6) is fenced
out of production by `scripts/gates/bit-identity-consumer.sh` and
would need a DESIGN.md revision. Widening the three doors to
`Decide + Bounds` was applied and followed with `cargo check`: it
reaches **20 signatures inside `topo`** — including the public
`splitting::split`, `splitting::section::plane_section` and
`Body::merge_coplanar_faces` — and then **20 more across `sweep` and
`editor-core`** (`extrude`, `revolve`, `loft`, the tube doors, the
wire evaluators) without converging after 20 rounds, with
`step-import`, `pncad`, `viewer` and the demo roots still downstream
of those. That is a cross-crate public-API bound change through four
programs' territory, and it is a decision to take on its own evidence
rather than a rider on a fix pass.

**What would close it**, in rough order of cost: (a) record a
`GeomSource` wherever a producer mints a surface it knows the recipe
for — today only `editor-core` does, so every `sweep` and
`step-import` body reaches these doors with no provenance at all, and
this would pay off in the merge door's second rung too; (b) the bound
widening above, priced by whoever wants it; (c) nothing, with the
re-mint as the documented price — which is what the doors say today.
