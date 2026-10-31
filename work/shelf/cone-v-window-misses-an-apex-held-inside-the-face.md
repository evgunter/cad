---
id: cone-v-window-misses-an-apex-held-inside-the-face
kind: issue
title: replace_face's cone v-window is its boundary's alone, so a cone face holding its apex inside, off its boundary, reads a window that misses the apex
status: open
opened: 2026-10-31
priority: P3
cost: E
---

Found by FRUSTUM's sweep (the shape: a cone face's `v` window and
whether it reaches the apex).

`crates/topo/src/replace_face.rs`, `group_cone_v_window`: the window
is the hull of the boundary edges' `v` ranges, read by the offset's
apex decision (`offset_apex_window`). A cone face can hold its apex
inside with none of its boundary on it: a cap bounded by one circle,
no seam, whose loop winds once about the axis. Then the window's near
end is the rim, not the apex, and the apex decision reads the wrong
end.

No row builds such a face through a kernel door (a revolve keeps an
apex vertex on a cone face that reaches its apex, and merging its
halves keeps the vertex and a seam, where the boundary's range takes
the apex's `0`; booleans were not checked), but the
construction API does: `clearance::cone_window`'s `cone_cap` fixture
is one. `clearance::misses_apex` is the decision that tells the two
apart (the boundary's slant enclosure excludes `0`, and the loops'
joint elements compose to the identity with no reset); the fix is
that reading here, widening to the apex where it fails, plus a row
offsetting the cap.
