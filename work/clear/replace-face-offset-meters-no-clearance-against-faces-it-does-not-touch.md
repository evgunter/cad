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
