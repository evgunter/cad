---
id: the-creation-tools-commit-buttons-are-live-over-a-pick-their-door-refuses-by-kind
kind: issue
title: Extrude, the seven seated-tool commits and Commit blend are drawn live over a pick require_kind refuses
status: open
opened: 2026-09-28
priority: P3
cost: M
design: true
refs: [the-mirror-class-is-unswept-outside-the-properties-pane]
---


Found by the census in `the-mirror-class-is-unswept-outside-the-properties-pane`
(VNEWS), at merge base `f4e9aa68b`. Filed here because
`crates/viewer/src/pane/create.rs` and the doors are VSEAM's.

## What happens

Nine commit controls in `pane/create.rs` are live whenever their seats
are filled (or, for Extrude, whenever anything is selected), and every
one of their doors in `crates/viewer/src/session.rs` runs
`DocSession::require_kind` first, refusing `Refusal::WrongNodeKind`:

| control | gate | door and kind it requires | a state it is live in and refused |
|---|---|---|---|
| Extrude (`ViewerBehavior::extrude_ui`, ~:1172) | `selection().node().is_some()` | `add_extrude`: Profile | an extrude or datum selected in the tree, or a face picked on an extrude |
| Commit revolve (`tool_commit_row` call ~:1231) | seats filled | `add_revolve`: Profile, SketchAxis | a world axis datum picked as the axis |
| Commit boolean (~:1270) | seats filled | `add_boolean`: Body x2 | a profile, split or pattern picked as an operand |
| Commit split (~:1289) | seats filled | `add_split`: Body, Plane | a pick that fits neither seat (a profile) |
| Commit transform (~:1329) | seats filled | `add_transform`: Body | a click on a split's drawn half (seat node is the Split) |
| Commit pattern (~:1412) | seats filled | `add_pattern`: Body, Axis when circular | a pattern root as the body |
| Commit projection (~:1470) | seats filled | `add_part`: Split or Instances by selector | an extrude in the seat with "Half" chosen |
| Commit duplicate (~:1496) | seats filled | `add_duplicate`: Body | a split half or pattern picked |
| Commit blend (`blend_commit_row`, ~:1618) | none; `BlendTool::fillet_op`/`chamfer_op` (blend.rs) check only that edges are held | `add_blend`: Body | edges picked on a split's or pattern's body |

`tool_commit_row` (~:1668) draws a bare `ui.button(label)`; the tool's
own `op(drafts)` answers only `CommitFault`s (empty seat, draft value),
which go to notices and push no op.

## Why this carries `design: true`

The behaviour is stated as intended. `crates/viewer/src/seats.rs`'s
module doc, *"Kinds ROUTE a pick; they still do not judge one"*: no tool
decides whether a seat's contents are legal, so *"a wrong-kind pick
still refuses typed at the commit rather than being silently ignored at
pick time, and the verdict lives in one place"*. Extrude's doc says the
same (*"A selection that is not a profile refuses typed at the door"*).

That argument is against IGNORING a pick, not against drawing the
button disabled with the door's own sentence, which is what the hide
toggle and the unit picker now do. The classifier is already shared
(`crate::session::admits`, the one `require_kind` calls, and
`Seat::wants`), so gating each seat on `admits(committed_doc().node(held),
seat.wants())` and carrying `Refusal::WrongNodeKind`'s words keeps the
verdict in one place. The question is whether that reverses the
seats.rs decision or completes it; weigh it before building.

## Also chrome-computable here, but late

Two refusals of the same tools arrive at `commit` or evaluation, not at
admission, and the same seat judgement could answer them: a boolean with
one node in both seats (`DuplicateInput`), and a projection's instance
index past the landed pattern's count.
