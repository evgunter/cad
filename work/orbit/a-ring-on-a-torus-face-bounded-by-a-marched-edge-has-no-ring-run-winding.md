---
id: a-ring-on-a-torus-face-bounded-by-a-marched-edge-has-no-ring-run-winding
kind: issue
title: A ring on a torus face bounded by a marched plane × torus edge refuses EdgeCarrierUnsupported at the ring run
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [a-torus-near-a-tilted-cut-stops-at-the-extent-scan]
---


Found by `orbit/tilted-torus-rim` (2026-11-23), once the section value
marches a plane tilted against a torus's axis.

`crates/sweep/tests/germ_plane_torus_frame.rs`
`a_general_pose_builds_to_its_volume`, pose B3 (the 0.02-wide bar
through `(0, 0.25, 2)` along `(0, 0.3, 1)`): its tilted faces cut the
donut's tube in null loops, the marched loops leave a ring on the torus
face, and the join's ring-run winding refuses

```
EdgeCarrierUnsupported { operand: A, edge: …, face: …, face_of: A, site: RingRun }
```

`topo::boolean::join::ring_run_ccw` refuses a spline run edge on a
wall's or a quadric's closing before the winding is read: a torus face
is closed by its curve (`IslandClosing::Quadric`), whose path winding
reads no spline edge. The row pins the refusal for B3.

Settles when the quadric closing winds a run with a marched edge (the
path parity reading the spline edge's crossings), and B3 builds to its
volume.
