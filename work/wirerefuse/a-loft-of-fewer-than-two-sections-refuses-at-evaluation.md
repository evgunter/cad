---
id: a-loft-of-fewer-than-two-sections-refuses-at-evaluation
kind: issue
title: A loft of fewer than two sections refuses at evaluation though its section count is authored data the doors hold
status: open
opened: 2026-10-20
priority: P3
cost: E
---


## Finding

Found by the sweep of WIRE's `wire/empty-blend-at-the-door` unit,
which moved an empty blend selection's refusal from evaluation to the
edit and load doors (`Select::fault`) on the ground that how many
names a selection holds is authored data. The sweep looked for other
counts evaluation refuses that are readable off the document as
written.

`crates/editor-core/src/eval/wire.rs` `wire_loft` refuses
`NodeErrorKind::Skin(SkinError::TooFewSections { have, need: 2 })`
when `profiles.len() < 2`, before anything is skinned. A loft's
section list is `Node::Loft`'s own operand list, fixed when the node
is written, so the count is as much the doors' fact as a blend's edge
count; `Node::input_fault` (`crates/editor-core/src/node.rs`) and the
load door's walk over it (`persist/check.rs`) are where both doors
already ask a node's operand shape. Python reaches it with
`Node.loft([], 2)` or one section.

Not the same as a union's members: `wire_union` gives an empty
member list a meaning (the typed empty value), so its count has no
bound to hold.

## What is owed

A ruling on whether the edit and load doors refuse a loft of fewer
than two sections (an `InputFault` arm, with `SkinError::TooFewSections`
then unreachable from a document), or the evaluation refusal stays and
says why a door does not hold it.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).

## Re-checked and fixed (2026-11-02, wire-refuse1, branch `wirerefuse/loft-count-at-load`)

Re-check: since this row was filed, the edit doors refuse a one-section loft
(`EditError::LoftShape { fault: LoftFault::TooFewSections }`,
`edit::check_loft_shape`, asked by `check_written_node` and the slot door).
The load door did not ask: `persist/check.rs` `validate_snapshot` asks
`Node::input_fault` but no loft shape, so a saved file whose loft had one
section, or a section with no station, loaded and then refused at evaluation.

Ruling taken: both doors refuse the count, and evaluation does not.
- `edit::loft_structure_fault` is the part of the loft's shape that no value
  moves: one section, or an empty station. `check_loft_shape` asks it first.
  The load door asks it too, as the new `SnapshotError::LoftShape { node, fault }`,
  whose sentence is `LoftFault::problem`, the one both doors forward. The arms
  that depend on values (a station in the first plane, the degree) stay with the
  edit doors and evaluation, because a `SetVarValue` may reach them legitimately.
- `wire_loft`'s `SkinError::TooFewSections` refusal is now `unreachable!` (D9
  row 4). Row 0 does not apply: a non-empty `then` type would reach `Node::Loft`'s
  public shape, the wire format and pncad-py.
- Witness: `intent_loft_set_sections::a_saved_loft_of_one_section_or_an_empty_station_refuses_at_load`.

Sweep (evaluation refusals keyed on an operand list's length or emptiness,
`if .*(\.len\(\) <|==|…)|\.is_empty\(\)` in `eval/`, and every
`Err(NodeErrorKind::…)` construction in `eval/` read for a fact the doors
hold): the blend's empty selection is already `unreachable!`;
`with_replaced`, the union's `images` and the mate checks are not refusals
of authored counts. One sibling was found and filed:
`an-empty-pick-path-refuses-at-evaluation-though-both-doors-hold-it`.
Blind spot: a door fact that evaluation reads other than through a length,
for example an operand kind. The second pass over the `Err(NodeErrorKind::…)`
list covers `eval/` only, not the kernel crates' own count refusals behind
evaluation (`sweep::skin`'s `TooFewSections` stays: a kernel door).

Review fix pass (2026-11-02): the witness now drops the dropped
station's anonymous variable with it, so the doctored files break in
the loft alone (with the load check disabled they load, as on base);
`SnapshotError::LoftShape` ends with `Recourse(REGENERATE_RECOURSE)`.
`loft_structure_fault` stays beside `check_loft_shape` rather than in
`Node::input_fault`: an `InputFault` renders at the edit door as
`EditError::IndexedRead` ("cannot read that member") with that
recourse, while a loft's shape has its own variant, sentence and
per-arm recourse in one `LoftFault` whose value arms cannot live in
`input_fault`; folding would split one enum across two homes.
