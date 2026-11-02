---
id: a-profile-pick-names-no-profile-piece
kind: issue
title: a viewport click on a profile selects the node and names no profile piece (ProfileEdgeRef)
status: open
opened: 2026-11-02
priority: P3
cost: M
---


Found by the lane that made a profile pickable in the viewer
(`a-profile-cannot-be-picked-in-the-viewer`), as a decision it left open
on purpose.

A click on a committed profile's drawn outline selects the profile node
(`Hovered::Profile` → `Selection::Node`, `crates/viewer/src/session/select.rs`).
It does not carry WHICH piece of the profile was hit. The hit is known
in the index (`PickIndex::profile_near`, `crates/viewer/src/pickindex.rs`:
the loop and the flattened segment), but turning that into the piece's
N1 name (`ProfileEdgeRef::Piece { step, role }`) needs the
authoring-step ↔ flattened-segment correspondence that
`marks::focus`'s doc (issue 1182) says is not established: one authored
step can lower to several segments, and an arc flattens to many.

Wanted when a tool takes a profile edge, or for the ruled in-profile
reads (`docs/PATHS-DESIGN.md` §1b). The selection would then need a
variant of its own (GQ7 single select; names only, G1), and the marks
a per-piece outline.
