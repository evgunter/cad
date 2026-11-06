---
id: a-corner-behind-the-incoming-rays-start-names-no-recourse
kind: issue
title: a fillet whose incoming leg ends at the corner refuses 'this corner lies behind the incoming ray's start' with no recourse, the first wall a newcomer hits once the lattice is satisfied
status: dispatched
opened: 2026-11-06
priority: P1
cost: E
branch: round/corner-behind-names-its-recourse
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
