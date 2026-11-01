---
id: edge-pose-reads-a-near-circular-ellipses-u-ref-off-its-semi-axes
kind: issue
title: edge_pose answers a near-circular ellipse's u_ref from major_frame, which rounding decides
status: open
opened: 2026-10-19
priority: P3
cost: E
design: true
---


Found by the readers audit of `circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band`
(PR 5156), which lets `Curve3::ellipse` mint near-circular ellipses.

`topo/src/readback.rs` `edge_pose` answers an ellipse edge's
`Pose.u_ref` as `Curve3::major_frame().major_dir`, which picks the
major direction by `|semi_v| − |semi_u|`. On a near-circular ellipse
(the section of a plane a definite but tiny tilt off square, `a − b`
far under ε) that sign is rounding, so the reported `u_ref` can turn a
quarter turn between two poses that differ by nothing a caller can
see. The pose's own major direction is well defined: the section
table sets the stored `u_ref` from the surface axis
(`geom_brep::tilted_section_major`), and `Curve3::section_frame(axis)` reads it
from the axis, never from the semi-axes.

No in-repo consumer reads `u_ref` of an ellipse pose today
(`wire/pose.rs`, `mate/pin.rs` read `origin` and `axis`); it is a
public API answer (`editor-core` `names/interrogate.rs`).

The question (why `design: true`): what `edge_pose` promises for an
ellipse. Either the stored `u_ref` verbatim (the seam, which the
section tables put on the major vertex from the pose; the doc's "its
seam may sit at either principal vertex" then names the struct-literal
case only), or the major direction read through an adjacent face's
surface axis (`section_frame`), which needs the face at the read.
