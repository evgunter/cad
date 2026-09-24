---
id: count-continuous-arm-is-shadowed-by-the-display-unit-walk
kind: issue
title: SnapshotError::CountContinuous is unreachable: the display-unit walk refuses first
status: closed
pr: 2780
branch: edit/one-predicate-round-three
opened: 2026-09-16
refs: [three-door-predicates-are-hand-copied-not-shared]
closed: 2026-09-16
---


Measured by the `edit/one-predicate-round-two` unit while giving
`DocParam::is_continuous_count` one home, and recorded in that unit's
fresh census of `validate_document` (entry 9).

**The finding.** `validate_document` runs its walks in a fixed order:
float walk, distribution walk, **display-unit walk**, program walk,
`validate_snapshot`. The display-unit walk
(`persist/check.rs`'s `first_display_unit_fault`) refuses any
`DocParam::Continuous` whose `display_unit.measures()` is not its
declared `dim`. `UnitSym::measures` answers `Length`, `Angle` or
`Scalar` and nothing else — the table has no row for a count, and
`UnitSym::canonical_for` maps `Count` onto the dimensionless row on
purpose.

So a `Continuous` parameter declared `Count` fails the display-unit
walk **whatever notation it carries**, and `validate_snapshot` is never
reached for it. `SnapshotError::CountContinuous` is unreachable through
either persistence door.

**Measured, not inferred.**
`crates/editor-core/tests/edit_one_predicate.rs`'s
`a_continuous_parameter_declared_count_is_refused_at_both_doors_in_different_words` saves
a well-formed LENGTH parameter, retypes its `dim` to `Count` on the
wire, and reads back `PersistError::DisplayUnit { declared: Count, .. }`.
That row is green and is this finding's measurement; its doc says so
at the site.

**Why it is a row and not a deletion here.** Three readings are open
and the choice between them is a decision about what a file may carry:

- **Delete the arm.** It costs `SnapshotError` a variant and
  `pncad-py` a stable tag word (`count_continuous`), which is a Python
  API surface, so a retirement needs LIB's seat at the table.
- **Keep the arm and re-order the walks**, so the structural/continuous
  divide is reported before the notation. That changes which refusal a
  corrupt file gets, i.e. the diagnostics every existing corrupt-file
  row reads.
- **Keep the arm as a defence in depth** and say at the site that it is
  shadowed. Cheapest, and the honest version of the status quo — but a
  guard nothing can reach is documentation, and the implementer
  discipline's rule is that deleting it is then the repair.

The edit door's half is not in question: `SetDocParam` refuses
`ContinuousParamCannotBeCount` through the one shared predicate, and
that refusal is reachable and pinned.

## Dispatched (2026-09-16, EDIT orchestrator) — middle tier, with the slot-dimension row

Built by the unit `edit/one-predicate-round-three`, whose spec is the
`## Spec` section of `load-door-checks-slot-dimensions-for-profile-nodes-only`;
this row's decision is ruled there.

## Built (2026-09-16, `edit/one-predicate-round-three`)

`SnapshotError::CountContinuous` and the `validate_snapshot` walk that
raised it are deleted, and the `count_continuous` tag is retired —
the first of the three readings this row listed, ruled by the
implementer discipline's own rule that a guard nothing can reach is
documentation whose repair is deletion.

The row that measured the shadow keeps asserting what the load door
actually answers, `PersistError::DisplayUnit { declared: Count, .. }`,
and its doc now states the divide's one home rather than pointing at a
filed gap. It is named for what it asserts —
`edit_one_predicate::a_continuous_parameter_declared_count_is_refused_at_both_doors_in_different_words`
— because that is the property the deletion leaves: same verdict,
different word, and only the edit door can name the
structural/continuous divide as the reason. `write_doc_param`'s doc
says so at the site, where it previously claimed "the same fault
whichever door refuses it". The edit door's
half is untouched and still reachable: `DocParam::is_continuous_count`
is asked by `SetDocParam`, and its rustdoc says why that is the only
door where it can fire.

Outside EDIT's fence, mechanical: `crates/pncad-py/src/tags.rs` and
`src/tests.rs` lose the tag word. LIB's files.
## Closed (2026-09-16, EDIT orchestrator)

Built and merged as PR #2780; the record is on
`load-door-checks-slot-dimensions-for-profile-nodes-only`'s `## Closed`.
