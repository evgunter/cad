---
id: reach-lever-from-reads-no-pivot-for-a-measured-or-face-reach
kind: issue
title: Reach::lever_from(pivot) reads no pivot for a Measured or a Face reach, so a lever 'from a point' is the caller's length from its own at
status: open
opened: 2026-11-24
priority: P2
---


Found by `oneround/reach-from-the-read-point` (PR 7661), confirmed by its review. Lines are on the branch after merging `main` at `a7b44d8eb`.

**What the door does.** `geom_brep::Reach::lever_from(pivot)` (`crates/geom-brep/src/extent.rs`, `Reach::reach_from`) answers in two ways:
- for `Ball` and `Region`, the region's far distance from `pivot`;
- for `Measured` it returns the caller's `lever`, and for `Face` its `across`, the face's farthest distance from the reach's own `at`, whatever `pivot` is. The doc says so ("whatever the pivot").

So the signature claims a reading from a point that two of its four variants do not make.

**Who builds those variants, and what a pivoted read then gets:**
- `Reach::Measured`: only through `Reach::of_frame`, which no production code calls today (one geom-brep test builds a literal, `carrier_resolution.rs`).
- `Reach::Face`, built through `Reach::of_face` by:
  - `topo::cover::cone_face_reach`, with `at` the caller's point and `across` the face cover's far distance from `at`;
  - `topo::boolean::join` (the plane×cone frame, about `join.rs:3097`), with `at` the centre of the hulled face covers' ball;
  - `topo::chord_join` (about `:1916`, from a cone reach's fields, handed to `plane_cylinder_section`).
- The consumers that pivot a `Face` reach: `geom_brep::intersect` `plane_cone_section` (`reach.lever_from(apex)`, about `:2417`) and `route_pose` (`reach.lever_from(anchor)`, about `:555`).
  - Each gets the face's far distance from `at`, not from the apex or the anchor.
  - Where `at` is not the pivot (join's ball centre against the cone's apex), the lever is a reach from another point. The difference is at most `|at − pivot|` either way (the triangle inequality), so neither bound is sure.
  - `hinge_lever` and `turn_lever` (the plane×cylinder rows) read `Face` from its own `at`, as documented.

**What a fix needs.** One of:
- (a) `Face` holds its region (the face cover), so `lever_from(pivot)` reads its far distance from `pivot`, as `Region` does;
- (b) the consumers read `across` under a name that does not claim a pivot, and pivot at `at`.

That is a change to what the record holds, or to the door's signature, so it is filed here rather than made by the conversion unit.

Per `work/README.md`'s bands: a door whose reading is not what its signature says, with no wrong verdict shown. P2.
