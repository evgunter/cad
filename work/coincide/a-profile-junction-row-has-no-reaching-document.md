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

## The same-carrier relation too, since the shorter arm (2026-10-19)

#5377's fix pass levers the lattice's junction turn by the SHORTER of
the two legs' arms, once the leaving leg is stored, and validation's
joint pass reads the same arm. The arm's other reaching row,
`a_profile_junction_decided_on_one_carrier_is_one_same_oriented_row`
(a 1 mm leg rising `ε/10` off a 10 m line: a corner at the 10 m
arriving arm, one carrier continuing at validation), now refuses at the
door, `JunctionTangent` at step 2 (`0.1ε` at the 1 mm arm), and is
`a_profile_junction_on_one_carrier_refuses_at_the_door`. Validation's
same-carrier read (`chord_side`, `crates/profile/src/seg.rs`, in
`joint_tangency`) puts the leaving leg's far end against the arriving
leg's line, `φ` times the leaving leg's length, at least `φ` times the
shorter arm, so a junction the lattice builds as a corner cannot read
as one carrier either. So neither of the
`ProfileJunction` arm's relations, `Tangent` nor `SameOriented`, has a
reaching document, and its "continues on one carrier with" sentence
joins "continues tangent into" without a test that speaks it.

## Python too, and probed (2026-10-20)

`crates/pncad-py/tests/test_checks.py` carried the same two rows
(the `sqrt(ε)` line–arc `tangent` row and the 1 mm `same_oriented`
row). Both went red with #5377 and are re-pinned to the door
(`paths/python-junction-main-red`): a corner with no row, and
`junction_tangent` at the verb. A probe looked for any lattice program
that reaches a decided junction under the shorter-arm rule. It ran
line–arc, line–line and arc–arc junctions across the band, at `f64`
and `Interval`, at coordinates up to `1e9` m, and on legs a later
fillet trims. None reached one: every case refused at the door or
validated with `decided_joints()` empty
(`work/paths/a-python-junction-test-builds-a-junction-the-door-now-refuses.md`,
Outcome). So the `ProfileJunction` arm's rows are unreachable from
every document surface, Rust and Python. Its text in
`pncad.pyi` (`Coincidence.relation`/`site`) and in
`crates/pncad-py/src/py/value.rs` and `py/checks.rs` names
`profile_junction` rows that no document can produce, and goes with
whatever the open question decides.

