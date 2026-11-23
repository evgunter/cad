---
id: profile-has-no-lever-door-a-2d-crate-can-call
kind: issue
title: profile's point and curvature reads have no lever door a 2D crate can call
status: open
opened: 2026-11-22
design: true
priority: P2
---

Found by `oneround/unsourced-unions` (PR 7294), which left `profile`'s six `// Unsourced lever:` pins in place (ONEROUND orchestrator, 2026-11-22: what `lever-doors.sh` holds for every crate is a design question of its own, not that unit's).

The six are three kinds, each checked in the tree:

- **A record's field.** `sugar::arc_fillet_trims` levers `Leg::arm`, which `Leg::at_corner` fills. That one waits on `stored-reach-scalars-a-union-cannot-be-read-from`.
- **A curvature arm folded with a two-point read.** `seg::junction_reverses`, `seg::junction_turn` and `path`'s seam-arrival side take `Seg::arm()`: a line's length, an arc's `arc_lever(radius, chord) = min(radius, chord)`, a full turn's radius.
- **A two-point read alone.** `seg::sagitta` reads the chord `|b − a|`; `path::resolve_fillet` reads the two anchors' separation.

What stands in the way:

- The point-level reads, `points_lever_from` and `points_diameter`, live in `geom_brep`'s `extent.rs` and take `Point3`. `profile` reads `Point2` and does not depend on `geom_brep`. `crates/geom-core/README.md` ("Float rounding") puts the point-level reads beside `Lever` in `geom-core`.
- `scripts/gates/lever-doors.sh` holds both as doors that take a caller's points, with per-file pins that never rise above the merge base, so a new reference from `profile` is red.
- The curvature door, `Lever::curvature_radius`, has its home in `geom-brep`'s `curvature_arm.rs`, and the same gate pins every other reference.
- The gate counts references by name. It cannot tell a read of consumed points (a segment's own ends) from a read of computed ones.

Options, none chosen:

- **(A)** Move the point reads into `geom-core` with 2D forms and add a 2D curvature door; `profile`'s references enter `lever-doors.counts` as new pins, which the no-rise rule must be changed to admit.
- **(B)** Make `geom-core`'s point-level reads a source the ratchet no longer holds, leaving review to hold which points; this changes the gate's doctrine for every crate.
- **(C)** Keep the six pinned until the records unit and a curvature-door unit (what PR 7294 does).
