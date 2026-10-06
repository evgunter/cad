---
id: the-lift-center-writer-loses-the-stored-radius-bits
kind: issue
title: a pinned lift writes arc_to(Center), whose lowering stores the radius as the rim at the start, so a half-disc's lift fell from bit-identical to value-equal
status: open
opened: 2026-10-06
---


`lift::chain_form` (`crates/profile/src/lift.rs`) writes an undeclared
arc as `Step::ArcTo(ArcData::Center { c, winding, target })` about its
stored centre. Since 5b (#3774), the `Center` mode's one conversion
(`path::center_arc`) stores the radius as `‖a − c‖`, the rim at the
arc's start, and the sweep as `4·atan(σh/(r + σp))` off the chord. That
is the authored shape of a `Center` arc, but it is not the stored
radius bit for bit. So a table whose stored radius is not exactly that
rim replays value-equal.

`crates/profile/tests/lift_census.rs` moved two loops, `half_disc` and
`half_disc_undeclared`, from `Bits` to `Value`, and the tally of
bit-identical lifts fell from 7 to 5.
`an_undeclared_cocircular_run_lifts_as_the_declared_joint` now reads
`Fidelity::ValueEqual`.

Before 5b these were bit-identical because the lift's replay
re-lowered from the chord, as the table's arcs had been. A lossless writer needs a
program step that carries the radius, such as a `Center` spelling with
the radius as data. That is a question for the program vocabulary
(unit 6, `pncad-surface-for-canonical-segments`, owns the surface) and
not a fix pass.
