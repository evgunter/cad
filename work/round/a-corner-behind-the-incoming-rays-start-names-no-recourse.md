---
id: a-corner-behind-the-incoming-rays-start-names-no-recourse
kind: issue
title: a fillet whose incoming leg ends at the corner refuses 'this corner lies behind the incoming ray's start' with no recourse, the first wall a newcomer hits once the lattice is satisfied
status: closed
opened: 2026-11-06
priority: P1
cost: E
branch: round/corner-behind-names-its-recourse
closed: 2026-11-06
pr: 6556
---


**Found by PR 6549's style review (ROUND, 2026-11-06), by authoring.** The
reviewer wrote an L-bracket through editor-core's insert door with two
fillets and an inner arc, at naive coordinates: once the lattice
refusals had guided the step order (PR 6549), every lattice-legal
variant was refused at the fillet with "this corner lies behind the
incoming ray's start" (`CornerReason` `BehindIncomingRay`,
`crates/profile/src/path.rs`, the corner-reason `Display` arm; about
:725). The cause is the most natural authoring of all: the incoming leg
ends AT the corner, so the ray extension has no room. Nothing says the
leg must stop short of the corner, and the refusal names no recourse.

PR 6549's acceptance walk (`crates/editor-core/tests/fillet_step_walk.rs`)
cannot see it: its "naive" attempts already put every leg 2R short of
its corner. That premise excludes this failing mode.

Owed: the refusal names its recourse ("end the incoming leg short of the
corner: the fillet's corner is where the two carriers cross, and is
never authored"), at both the line and arc doors, and the walk gains an
attempt that ends a leg at its corner and is guided out by this
sentence. Same family: the shared `TipState::Angle` (reached by an entry
director and by a fillet's arrival director) offers `FarEndTo` after an
entry `Toward`, where the path then refuses `FarEndAnchorWithoutFillet`
— lattice-true, not acceptance-true.

## Closed (2026-11-06)

PR 6556. Both corner-window refusals name their lever: "start the
incoming side short of the corner (end its leg, or move its start)" and
"anchor the arrival side beyond the corner, within half a turn on an
arc", each ending in one `CORNER_IS_NEVER_AUTHORED` clause. A pair
whose corners need different levers names each beside its corner; the
pair's `.contains("anchor")` text match is gone for a private
`CornerLever` with an explicit `subsumes` rule (pinned both ways).
`crates/editor-core/tests/fillet_step_walk.rs` gains the leg-at-its-corner
and arrival-at-its-corner attempts and follows both to the pad. Single
STYLE review: APPROVE-WITH-FIXES, all taken. The shared `Angle` state's
`FarEndTo` offer this row's last paragraph named is
`an-entry-director-offers-far-end-to-the-path-then-refuses` (P3).
