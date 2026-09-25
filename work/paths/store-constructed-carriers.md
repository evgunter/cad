---
id: store-constructed-carriers
kind: unit
title: Store the carriers circle, Center and fillet arcs are built from; delete the hand copies of bulge→carrier and the bulge accessor
status: parked
opened: 2026-09-25
priority: P1
cost: D
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
blocked_on: [geom-brep-sketch-segment-full-turn]
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

Two more readers of the kept bulge, found in review of #3224:
- `lift.rs::chain_form` writes `ArcData::Bulge { b: bulges[src] }`, and
  `lift_seamed` refuses a loop whose bulges are not finite. Once bulges
  retire, that writer needs a b derived from Δθ, and `tan(Δθ/4)` is
  lossy (b = 1.0 comes back 0.9999999999999999), so the lift owes a
  lossless route: a program step spelled on the carrier, or a bulge
  derivation shown exact.
- **The validate-time carrier check arrives with this unit.** D1 says a
  stored carrier is verified at validation. Unit 1 derives every
  carrier at the lowering, so it holds by construction and `build_seg`
  does not re-check it. Once this unit stores the carriers the
  constructions build, the carrier is no longer a function of the
  chord and b, and validation needs the predicate that verifies it
  against its vertices.

Found in the fixture-door migration (#3231): a loop built through the
canonical fixture door stores its carrier as given, but `map_scalar`,
`reversed` and the lift re-derive the carrier from the stored
`tan(Δθ/4)` bulge. So for such loops the result can differ from the
given carrier in the last bits. Retiring the stored bulge here removes
that.

From #3231's review:
- The magnitude is worse than last bits. A one-segment full circle
  built through the canonical door has a zero chord, so re-lowering it
  from the kept bulge gives a NaN centre and zero radius.
- `RawLoop::new` adds a carrier→bulge copy, `(sweep/4).tan()`, which
  joins `sugar.rs::bulge_from_center`'s tail and `path/verbs.rs`. All
  three go when the kept bulge retires.

**From `geom-brep-sketch-segment-full-turn` (2026-09-25).** Two of the
seven hand copies are gone. `SketchSegment::eval` reads the segment's
stored centre and sweep. `skin::segment_curve` reads the stored centre,
radius and sweep. Both are still `seg::arc_carrier`'s derivation,
carried across the boundary. `ValidatedSegment::bulge` has no reader
left in `geom-brep`, `topo` or `sweep`. Its readers are the lift
(profile), `anchor`, the `stackup` digest and `viewer::flatten`.
