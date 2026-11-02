---
id: axial-frame-extent-reads-vertices-not-arcs
kind: issue
title: offset_axial's frame.extent is the scope's farthest vertex from the axis foot, short of a meridian arc that bulges past its ends
status: open
opened: 2026-11-02
priority: P2
cost: E
---


Found by the `onehome/one-extent` unit (2026-11-02), which typed the
frame's extent a `Lever`. `topo::offset_axial`'s `axial_frame` measures
`Frame::extent` as the scope's farthest **vertex** from the axis foot
(an `ExtentBall::point(..).lever_from(..)` fold). A meridian arc that
bulges past its ends (a torus's outer equator) stands farther than
that, so the lever the frame's sine closures, pole station, carrier
mint and latitude posture read (`offset_axial` `classify`,
`solve_corner`, `mint_carrier`, `latitude_posture`) is short of the
consumed region: the wrong-answer direction. Same class as
`work/charttail/chart-region-carrier-gates-read-vertices-not-arcs.md`;
the edge extent's home is `geom_brep::extent` (`edge_reach_from`).
