---
id: the-face-boundary-walk-is-entered-around-its-home
kind: issue
title: The face-boundary walk's input is still assembled outside its home, and its doc claims readers it does not have
status: open
opened: 2026-11-17
priority: P3
refs: [6781]
---

From PR 6781's review (the containment move, step 3), which moved the face-boundary walk to `crate::face_boundary`.

- **The input is still built outside its home.** `carrier_cross.rs:201`, `read_boundary`, calls `curve_face::boundary(swept, &b.spans(), &b.lone_points(), band)` by hand, beside `FaceBoundary::read`/`located`. To keep it compiling, `spans` and `lone_points` were widened to `pub(crate)` (`face_boundary.rs:60, 68`). Fix: a `FaceBoundary::boundary` method, with those two private again.
- **The module doc claims every reader.** `face_boundary.rs:5-7` says every reader that asks where a curve meets a face's boundary walks through `face_boundary`. Two do not:
  - census's `edge_crossings`/`meetings` (`census.rs:4976, 5028`), through `splitting::containment::LoopEdge` + `curve_pair::meet`;
  - `reduce.rs:3640`, which reads meetings edge by edge about a vertex.

  Decide for each whether it is the same question, and so a second home to port, or a different one, and so the doc narrows.
- **The live-walk row pins little.** `face_boundary.rs:140-160` checks the edge count and that `lone` is empty. It does not check the walk order, `start`/`ends`, or the `Ok(None)` branch for an edge with no certified curve (`:115-117`).
- **Two `ends` orientations.** `live::FaceEdge.ends` runs along the half-edge and `face_boundary::BoundaryEdge.ends` along the span (he_plus), both built from `FaceView::boundary`. Settle one convention, or rename one of them.
- **Small items:**
  - `face_boundary.rs:96-100` duplicates `boundary_crossing`'s Panics text;
  - `From<FaceBoundaryError> for BooleanError` sits in carrier_cross.rs, though reduce and overlap use it;
  - the topo README's "Where in the code" table doesn't list `face_boundary.rs`.

**Done when:** the walk's input is built only inside `FaceBoundary`, the doc's claim is true, and a row pins order, ends and the uncertified-edge branch.
