---
id: replace-face-writes-a-loop-edges-reading-back-as-its-stored-carrier
kind: issue
title: replace_face re-attaches an untouched loop edge on its reading, so the edge stores a clamped net and its arcs no longer rejoin
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found at PR 7052's review, with four sibling sites that were fixed
there (`boolean/combine.rs`'s graft, `merge_faces.rs` and `seqgen.rs`
test support) or need nothing (`shell.rs`'s `EdgeArc::Conic`, reached
by a circle or an ellipse only).

`crates/topo/src/replace_face.rs`, the `Incident` built for an edge the
move does not re-derive (`carrier: curve.carrier().clone()`, line 4857
at `ce33e72ce3`), and the plan it feeds, carry the edge's *reading*. For
an edge on a periodic cubic that is the clamped view over its span, so
the re-attached edge stores a clamped net. `edge_join::crease_at` then
reads that net's clamped ends as creases, and two such arcs of one loop
never rejoin: the maximal-edge complex depends on whether a face offset
ran, which tier 3 cannot see.

Why it was not switched in place: the plan's readers take a spline
through the foot lane on `Curve3::Nurbs` (`read_ends`, `plan_reanchors`,
`solve_corners`), and a `Curve3::Periodic` there refuses
`NurbsLaneUnsupported`. Keeping the stored loop needs those readers to
read a loop's view while the plan keeps the loop and the span.

Not measured: no face offset of a body with a loop edge was run.
