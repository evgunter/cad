---
id: a-spline-faces-cover-is-its-whole-control-net
kind: issue
title: A spline face's cover is its whole control net, so the session box refuses as a limit a spline-walled body that lies inside it
status: open
opened: 2026-11-21
priority: P3
cost: M
---


Found by `oneround/box-region` (PR 7225), measured.

`topo::cover::face_cover`'s spline arm is the carrier's cover, its whole control net (`Surface::Nurbs | Surface::Approx => carrier`), and `cover::face_cover_unproven` reads the same. Since PR 7225 the session box reads every face's region by its cover (`topo::session_box`), and a body whose cover alone reaches past the box refuses as the limit `ValidationError::SessionBoxUnshown`.

**Measured.** Across the workspace's fast set at ε = 1e-9, 1e-6 and 1e-12 and its slow set at ε = 1e-9, one fixture outside that PR's own rows is refused this way: the exhausting arc prism of `crates/sweep/tests/tcost_k3_certificate.rs` (`EXHAUSTING`), lofted between two arc sections, at ε = 1e-9. It is scaled to reach 900 m (`FAR_END`, its bulge's crest), and its walls' net reaches 1272.8 m, `√2` times as far: a rational arc's middle control point stands at the corner of its tangents. So its rows (`a_multi_solid_certificate_split_across_rounds_continues_to_the_measurement`, `a_refusing_arm_returns_no_properties_through_either_door`) now assert the limit at 1e-9 and run their exhausting arm at 1e-12 only.

**The shape of a fix** (precision, not soundness): a spline face's cover bounded by the part of its net the face's trim spans (the net clipped to the face's parameter window, or refined there), or met with its boundary's hull where the face is a ruled loft. A tighter cover admits such a body; the box's verdict on a body past it is unchanged, since that is read off points on it.
