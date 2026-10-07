---
id: blend-reach-meters-a-nurbs-face-on-its-box-alone
kind: issue
title: blend: the reach meter has no surface distance for a NURBS face, so a loft wall whose box meets a band refuses uncertified however far its surface lies
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by the lane that built predicate 2's reach
(`blend-material-is-never-checked-against-faces-that-are-not-its-supports`).

`face_bound` in `crates/sweep/src/blend/reach.rs` clears a cell when
either the face's own surface distance (`surface_range`) or one of the
reach's bounds is positive over it. A NURBS or approximating face has no
closed-form surface distance (`surface_distance` answers `None`), so only
the reach's bounds can clear its cells: every cell of the face's control
hull box that the reach meets refuses `FaceClearance { bounded: true }`,
however far the actual surface lies from the band. That is an
over-refusal in the safe direction, for a loft or sweep wall standing
near a filleted edge.

The fix is a certified distance for a NURBS patch over a cell (its
control hull clipped to the cell, or a subdivided hull), which is the
same machinery the boolean's NURBS boxes use. No corpus body reaches it
today; the whole blend suite and the tour pass with the meter on.
