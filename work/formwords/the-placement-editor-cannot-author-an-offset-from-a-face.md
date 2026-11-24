---
id: the-placement-editor-cannot-author-an-offset-from-a-face
kind: issue
title: The placement editor never writes a slid far side, so no body can be placed at an offset from a face in the GUI
status: closed
opened: 2026-11-22
priority: P0
cost: M
refs: [a-plane-other-than-a-face-cannot-be-authored-in-the-viewer]
branch: formwords/face-offset
pr: 7380
closed: 2026-11-22
---


Found by the FORMWORDS plane-door designers (`[ev]` 7367, 2026-11-22).
Read from code; not driven.

DESIGN `[placement.mates]`: "a mate holds no number of its own … a
number a kind's own equation fixes (a standoff along a plane's normal)
is a construction on the target, never a value". So a body standing
5 mm off a face is a plane mate whose far side is the face's plane slid
along its own normal (`PoseDef::Slide`).
`crates/editor-core/tests/place_mate_frame_offset.rs` exercises that
spelling through the API.

The viewer never writes `PoseDef::Slide` (grep `crates/viewer/src`).
The placement editor's mate sides (`crates/viewer/src/matetool.rs`:
`side`, `turned`, `toggled`) write the face's plane and its `Flip`, but
no slide. A value cannot stand in for it: `mate::pin::Placed::freedoms`
leaves a plane mate only the in-plane slides and the spin. Equating
normals and sliding along a target axis measures the slide from the
space's origin, not from the face, so the body stays put when the face
moves.

So a standoff from a face cannot be authored in the GUI at all, which
puts it in P0 (`work/README.md`, Priority). Before building, drive it
once in the viewer to confirm.

The fix both designers named: the editor's far side gets an "offset"
row, an ordinary length row that takes formulas. It wraps the far
side's plane in `Slide` along its normal, and clearing it unwraps.
FORMWORDS' datum design (`[ev]` 7367, `[placement.datums]`) reads the
same row for a datum offset from a face, so whichever lands second
reuses it.

(FORMWORDS orchestrator)

Claimed by FORMWORDS on 2026-11-22, by `git mv`: the datum build (`[ev]` 7367) reads the same far-side offset row, and DOORS has no orchestrator on it. Dispatched as `formwords/face-offset`.

## Closed (2026-11-22)

PR 7380. A mate row holds an offset: one length row, a number or a formula. It writes the far side slid along the outward normal of the face the side reads (`matetool::face_pose`), whatever the side's spelling. An offset on a foreign slide sits beside it, and setting it again replaces it, never stacking. One buffered field serves the value rows and the offset row, keyed by (placement, row). Edges left: `face_pose` peels slides and flips but not spins, and a named pose variable is not unfolded.
