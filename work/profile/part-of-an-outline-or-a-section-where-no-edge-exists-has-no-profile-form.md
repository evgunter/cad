---
id: part-of-an-outline-or-a-section-where-no-edge-exists-has-no-profile-form
kind: issue
title: Part of a face's outline continued by drawn pieces, or a section where no edge exists, has no profile form: along a wall's trace, or a frozen run along picked edges
status: open
opened: 2026-11-22
priority: P2
cost: M
design: true
refs: [a-faces-loop-or-closed-trace-cannot-become-a-profile-loop]
---


Filed by PROFILE's cut of the face-loop ruling (PR 7284, 2026-11-22),
from the residue the two designers left open (fork log row 297, and
`work/faceloop/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`,
"Weighed": "What they did not settle is the second read"). Not cut: it
has no ruling.

## What the face loop does not cover

The ruled form (`docs/DESIGN.md` `[variables.kinds]`; FACELOOP's U2)
is a whole loop of the face the profile is drawn on. It does not
cover:

- **Part of an outline, continued by drawn pieces**: run along half a
  rim, then draw a chord.
- **A section where no edge exists**: a collar on a shaft, drawn on a
  plane through the shaft's axis, where the wall's cut is wanted and
  the plane holds no edge of the body.
- **A wall's plain section, wanted to ignore later edits to the cap**:
  the elbow kept at the wall's radius when the rim is later filleted.

Today a trace can be run **to** (`ProgramTarget::OnTrace`) and left
**tangent to** (`ProgramStep::AlongTrace`, a heading), and nothing runs
**along** one; a chain may not close on its own carrier (PQ4), so no
form spells a trace's whole closed circle
(`crates/editor-core/src/trace.rs`, module docs).

## The design question (`design`)

Two candidate reads, from the designers' texts:

- **Along a wall's trace.** A piece that runs along the trace it
  stands on to a target (the next trace's crossing, a written
  coordinate, the start), and a loop that is a whole closed trace,
  lowered to `line`, `arc_to(Center)` or `circle` off the carrier. It
  follows the wall and ignores edits to the cap. A filleted rim's wall
  is tangent to the cap, which the trace reading refuses today
  (`TraceFault::Grazing`), and a chain starting on the crossing of two
  traces needs a `Point2` read (SELFREF's ground).
- **A run along picked edges, frozen** as a fillet's selection is
  (`Node::Fillet`, "the selection freezes"; `names/select.rs`): the
  edge names stored, refusing when one is lost or split.

The designers' last rounds lean to the first and drop the second
(B's round 3: "No third, frozen-edge form"), with `[one-way]` kept at
the gesture: a click on an edge of the face sketched on writes the
face's loop, and latching another face writes a trace form. Not
weighed: renaming `AlongTrace` so that "along" means the new piece
(both designers' earlier rounds), and a coincident-wall statement in
the trace rung of `[coincidence.structural]` (a piece on G's trace
swept within G's symmetry sweeps G's own surface).

Weigh it with the designer pair before any lane builds it.

## Band

P2: neither case is what Ev asked for (the elbow is the face loop), and
both designers said not to block the elbow on it. Re-band P1 if a
collar or a part-outline is reported from the viewer. The trace door
is TRACE's ground (`crates/editor-core/src/trace.rs`); it is filed here
because the brief that cut the ruling asked for it on PROFILE, and
TRACE may claim it by `git mv`.
