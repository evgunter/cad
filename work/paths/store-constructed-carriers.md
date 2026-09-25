---
id: store-constructed-carriers
kind: unit
title: Store the carriers circle, Center and fillet arcs are built from; delete the hand copies of bulge→carrier and the bulge accessor
status: parked
opened: 2026-09-25
priority: P1
cost: D
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
blocked_on: [circle-lowers-to-one-segment]
---


Unit 5 of the #3218 lowering. Outputs move by ulps, and that is the point. The seven hand copies of the bulge→carrier formula go (`seg::arc_carrier`, `path.rs`, `lift.rs`, `skin::segment_curve`, `SketchSegment::eval`, `anchor::signed_area`, `viewer::flatten`), and so does `tube.rs`'s hand traversal. The bulge accessor is deleted with its last reader (Ev, #3218). Re-check whether `FilletArcFlattenedInStorage` and `profile-fillet-radius-off-at-eps-1e-6` dissolve.

**Unit 1 left bulge stored beside the canonical segment
(`ProfileLoop::bulges`, `ValidatedSegment::bulge`) to stay byte-identical.
This unit retires that storage**, together with every reader unit 1 left
on it except the geom-brep boundary, which is unit 2's:
- `seg::build_seg`'s straightness margin, sagitta and apex (K-stream
  margins, so re-baseline the K CSVs);
- `ValidatedSegment::lift` / `ProfileLoop::map_scalar`, which rebuild
  at U from (chord, b). The rebuild is re-derived from the stored
  carrier, which is the certified-lift design question unit 1 deferred;
- `lift.rs`'s `compare` ulps;
- the `anchor::derive_naming` bit-match and `signed_area`;
- the `stackup` digest over (x, y, b);
- `viewer::flatten`.

Unit 1's report on the PR lists each with its reason.
