---
id: step-adopt-reads-a-plane-origin-against-another-planes-normal
kind: issue
title: step-import's coincident_surfaces reads one plane's stored origin against another's normal after an unlevered parallel gate
status: open
opened: 2026-10-07
priority: P3
cost: E
refs: [cylinder-offsets-read-at-a-stored-origin-off-the-reach]
---


## What

`coincident_surfaces` (`crates/step-import/src/adopt.rs`), the plane
pair: `n1.cross(n2).norm() <= eps && (o2 - o1).dot(n1).abs() <= eps`.
Two PLANE records carry arbitrary origins, so `(o2 − o1)·n1` reads plane
2's stored origin against plane 1's normal: a tilt just under `eps`
moves it by the tilt times the origins' separation, which nothing
bounds. Two planes coplanar at the faces read apart when their origins
are stored far apart, and two that diverge at the faces can read
coincident. Neither gate is a named, margined decision, and the tilt is
not levered.

Fix: decide both through named margins, read the offset at the faces
(the foot of a face point on each plane), and lever the tilt at the
faces' extent from there.

Found by `tang/cylinder-offsets-at-the-reach`'s sweep. Read, not probed.
