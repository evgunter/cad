---
id: a-spline-boundary-edge-drops-a-faces-depth-cover
kind: issue
title: census::depth_gap::face_read gives a spline boundary edge no piece, so a face bounded by one has no boundary cover at all and the census's depth gap reads only its carrier
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by SECT step 1b's sweep for box rules that treat a spline's control
hull as no claim (PR 5894, 2026-10-28), and filed at the dual review's
ask.

## What

`topo::census::depth_gap::face_read` builds a face's boundary cover
piece by piece from each edge's box rule. `EdgeBoxRule::ControlHull`
(a spline edge) lands in the `NoSoundBox` arm and gives `None`, which
sets the whole boundary to `None`, so the face keeps only its
carrier's depth. A spline edge's locus lies in its control hull (its
weights are positive), so a piece exists. Step 1b gave the axial reader
the same hull (`boolean::boxes::AxialCarrier::Hull`) after the merge's
`face_reach` refused every union whose cylinder face a carried chord
bounds.

Not reached by SECT's rows: the cylinder × sphere booleans that build
chords refuse at the result gate's volume before the census reads a
body (`spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`).
Not examined: whether the lost cover ever costs the census a decision,
or only a looser gap.

## Done when

A spline edge contributes its control hull as a boundary piece (or the
reason it cannot is stated at the site), and a row reads the depth gap
of a face bounded by one.
