---
id: curved-face-placement-reads-no-spindle-face
kind: issue
title: curved_face_placement answers no verdict on a spindle face, so the session box has no point on one and a spindle body past the box is refused as a limit
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Found by the r1 review of PR 7225 (MINOR 2) and r2's NOTE 8, filed by that PR's lane.

**What.** `contain::curved_interior_placement` answers `Trim(None)` for a spindle face (`Surface::Plane | Surface::Spindle => Trim(None)`). The session box's inside read (`topo::cover::face_extreme_points`) keeps a candidate only where this door places it on the face, so it has no point on a spindle face, and a spindle body past the box only there is refused as the limit `SessionBoxUnshown`, against `crates/geom-core/README.md`'s "one with a point past it is refused as past the box".

**Measured.** r1's football (a spindle `R = 30`, `r = 50` between two cones) 30 m past the box is told it "may lie inside the box". In r2's differential, all 36 bodies truly past the box but refused as a limit have a spindle face (the apple 13, the lemon 8, the D bulge 7, the lens 6, the frustum's spindle cap 2).

**The shape of a fix.** A spindle face's placement (its carrier read as the torus's, and its trim read in its chart as a ring torus's is); with it, `face_extreme_points` takes a spindle's extremes as it takes a torus's (the same critical points). The cover's tube is a separate row (`work/boxes/a-spindle-faces-cover-is-its-whole-tube.md`).

**Why P2.** With no point placed on a spindle face, a body past the box by a spindle face alone is refused as a limit rather than as past it. Every one of the 36 past-the-box limits in PR 7225's differential has a spindle face. That is the right body refused for the wrong reason, and its recourse (move the geometry) is the same.
