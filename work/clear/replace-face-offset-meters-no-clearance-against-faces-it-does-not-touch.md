---
id: replace-face-offset-meters-no-clearance-against-faces-it-does-not-touch
kind: issue
title: topo: replace_face_offset moves a face without metering it against the body's non-adjacent faces (measure first)
status: open
opened: 2026-10-06
priority: P2
cost: M
---


Found by the class sweep of the BAND lane that built the blend's reach
(`work/band/blend-material-is-never-checked-against-faces-that-are-not-its-supports.md`):
the blend built bodies every tier admits because it judged only the faces
it touched. The same shape is a candidate here, and it is not measured.

`replace_face_offset` / `replace_faces_offset` in
`crates/topo/src/replace_face.rs` move a face to its offset and
re-describe its boundary against its neighbours; the door validates the
clone before adopting it. Nothing in the door reads the body's other
faces, so a face offset far enough to pass through a face it does not
share an edge with (a pocket floor pushed through the opposite wall of a
thin part) would rely on the closing validation to refuse — and tier 3's
per-face checks do not see two faces crossing (the shell module's own
docs say so of the curved neck, `crates/topo/src/shell.rs` module docs,
window #1055; `validate_pseudomanifold`'s census does).

`shell` guards its own path with its wall-clearance gate; the question is
the door's other callers (`offset_together`, the restate paths) and the
door itself. Measure first: build the push-through on a plain block and
read which tier refuses it.

**Evidence from SHELL (PR 4311, 2026-10-08).** `topo::shell` now
reads every non-adjacent pair of transversal planar faces of one
solid on the cavity its offset doors built (`moved_walls_cross`,
`crates/topo/src/shell.rs`): each moved face is cut by the line the
two moved planes share and the two cuts must not overlap. That read
takes a body and a partition and nothing shell-specific, so it is a
candidate planar half of the meter this item asks for on
`replace_faces_offset` and the public `offset_planes_together` door.
The latter has the same gap: it validates tier 2 and nothing reads a
moved face against the faces it does not touch.
