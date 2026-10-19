---
id: a-profile-junction-row-has-no-reaching-document
kind: issue
title: The coincidence door's ProfileJunction arm has no reaching document since the joint pass reads the path door's turn
status: open
opened: 2026-10-19
priority: P3
cost: M
design: true
---


Found by `paths/joint-reads-its-turn` (Ev's ruling on #5354).

`editor-core`'s coincidence door turns each `ValidatedLoop::decided_joints`
row into a `NamedCoincidence` at `topo::DecisionSite::ProfileJunction`
(`crates/editor-core/src/coincide.rs`, the `decided_joints()` loop; the
"a profile junction no constructor made" and "continues tangent into"
sentences). Its one reaching row,
`crates/editor-core/tests/coincidence_door.rs`'s former
`a_profile_junction_decided_tangent_is_one_unproven_row_on_its_profile`,
reached it through a disagreement that PR removed: a line into an arc
at a turn of `√ε`, which the path lattice read as a corner and
validation's carrier clearance (`r(1 − cos φ) ≈ ε/2`) read as Zero.

Validation now decides a joint by the same turn the lattice reads
(`seg::junction_turn`), levered by the longer leg's arm, which is at
least the lattice's arriving arm. A junction validation reads Zero the
lattice reads Zero too, and the lattice refuses an undeclared Zero
junction (`PathError::JunctionTangent` / `JunctionCusp`) while a
declared one is constructed, not decided. A document's profiles are all
lattice programs (`LoopProgram::Chain`, `Circle`, `CircleSplit`), so no
document reaches a decided junction except at the rounding boundary
between the lattice's authored direction and the stored segment's
heading. The row is now
`a_profile_junction_the_lattice_reads_as_a_corner_is_no_row`, which
asserts the corner and no row.

**The open part** (`design: true`): whether the `ProfileJunction` arm
should stay as the home for a decided junction a future door may mint
(a raw table, an import), with a unit-level row that builds a
`ValidatedLoop` from a raw `Profile` to keep its sentences reaching, or
whether it retires with its reachability. Until one or the other, its
speaking text has no test that reaches it.
