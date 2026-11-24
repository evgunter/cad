---
id: a-spline-faces-cover-is-its-whole-control-net
kind: issue
title: A spline face's cover is its whole control net, so the session box refuses as a limit a spline-walled body that lies inside it
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Found by `oneround/box-region` (PR 7225), measured.

`topo::cover::face_cover`'s spline arm is the carrier's cover, its whole control net (`Surface::Nurbs | Surface::Approx => carrier`), and `cover::face_cover_unproven` reads the same. Since PR 7225 the session box reads every face's region by its cover (`topo::session_box`), and a body whose cover alone reaches past the box refuses as the limit `ValidationError::SessionBoxUnshown`.

**Measured.** Across the workspace's fast set at ε = 1e-9, 1e-6 and 1e-12 and its slow set at ε = 1e-9, one fixture outside that PR's own rows is refused this way: the exhausting arc prism of `crates/sweep/tests/tcost_k3_certificate.rs` (`EXHAUSTING`), lofted between two arc sections, at ε = 1e-9. It is scaled to reach 900 m (`FAR_END`, its bulge's crest), and its walls' net reaches 1272.8 m, `√2` times as far: a rational arc's middle control point stands at the corner of its tangents. Its rows now stand the prism where its net fits the box, and the limit is a row of its own (`the_exhausting_prism_whose_net_alone_is_past_the_box_refuses_as_a_limit`).

**The shape of a fix** (precision, not soundness): a spline face's cover bounded by the part of its net the face's trim spans (the net clipped to the face's parameter window, or refined there), or met with its boundary's hull where the face is a ruled loft. A tighter cover admits such a body. It does not by itself refuse a spline body past the box as past it: no point of a spline face's interior is read (`work/boxes/a-spline-face-has-no-inside-read.md`).

**How wide the class is** (reviews of PR 7225, r1 NOTE F, r2 MAJOR 1). A rational arc's net overshoots the arc by at most `(√2 − 1)` of the arc's own radius, along the bisector of its end tangents, and by nothing along them; a 120° rational segment, its control point at twice the radius, by up to the radius. Distance from the origin enters only through where the fixture stands. Imported trimmed NURBS are the likelier source: the base surface's net can reach far past the trim, and the cover reads the whole net.

**Why P2.** The session box (PR 7225) refuses a valid body that main admits, wherever such a face's net reaches past the box. It needs a face whose net stands within its own overshoot of a box face, so not everyday parts, but site-scale models stand there by design.
