---
id: no-door-rejects-geometry-past-eps-f64-resolution
kind: issue
title: D4 ¶4's size range is unenforced: past 2^52·ε a certification residual reads rounding, and the f64 lane's Zero is luck
status: open
opened: 2026-10-12
---


(Filed by TANG's `tang/far-origin-rod-mapped-source` lane, PR 4909,
from the root cause of
`work/tang/a-far-origin-rod-cut-escalates-its-mapped-source-at-eps-1e-12.md`.)

## What

D4 ¶4 promises "a documented model size range; geometry outside the
range is rejected at construction". No door implements it, and no
range is named next to ε: there is no size-range or session-box
constant anywhere in `crates/*/src`.

f64 spaces coordinates at `ulp(x) ≈ |x|·2⁻⁵²`. A point-to-point
residual between two independently rounded evaluations can read a
whole ulp, so ε certifies a residual only while every magnitude the
evaluation passes through stays within `2⁵²·ε`. That is about 4.5 km
at ε = 1e-12 and 4500 km at 1e-9.

**The magnitude that binds is the evaluation's, not only the body's.**
A description stored with a far origin evaluates through that origin,
even when the body itself sits near zero. For example, `geom-brep`
`mapped.rs` `MappedSource::eval` computes `place(point) + vec·u`, and
`geom` `Curve3::eval` computes a line as `origin + dir·t`.

Past the range, every point-to-point certification residual reads the
rounding of its two evaluations, not a miss. These are `check_residual`
in `geom-brep`'s `certify.rs` and `pcurve_map_residual` in
`pcurve_cache.rs`. The f64 lane then certifies Zero wherever the two
roundings happen to agree, and it escalates wherever they don't. The
Interval lane straddles. Nothing tells the user that the data sits
outside the range where ε means anything.

**Measured (TANG).** A rod extruded over z ∈ ±1e5, cut at z = 0.5:
- At sample 3 the first split's strut compares `z = −62499.8125`
  (carrier) against `−62499.81249999999` (description).
- That is one ulp at |z| ≈ 6.2e4, 7.28e-12 > ε = 1e-12, so
  `carrier_matches_mapped_source` escalates.
- That verdict is correct. The row now stands the pose down where
  `far·f64::EPSILON` exceeds the zero band.

**A stored parameter adds a second floor, inside the same range.**
- A split description stores its sub-range as `u ∈ [0, 1]` over the
  whole source (`mapped.rs`, `SubRange`), so it names a point only to
  `lever·2⁻⁵³`.
- The lever is `|vec|` for an `ExtrudedPoint`, `|b − a|` for a
  `PlacedSegment` line, `r·|sweep|` for an arc, and `r·|angle|` for a
  `RevolvedPoint`.
- The strut above stores `span = fl(0.5000025)`, which in exact
  rationals misses the carrier by 1.228e-12 at sample 3 and 3.28e-12 at
  s = 1. That is below the evaluation ulp that binds.
- Each lever is at most 2π times the magnitude its evaluation passes
  through. So a range that bounds those magnitudes bounds this floor
  too, to within that factor.

## Repair shape (a design question)

1. **Name the range once**, beside `Tol` (geom-core, `tolerance.rs`),
   as a function of ε: `2⁵²·ε` less a margin for the levers above.
2. **Decide where it is enforced.** At construction, as D4 ¶4 says,
   over stored origins and extents as well as the body's coordinates.
   Or as a typed refusal naming the range, wherever a residual's own
   f64 floor exceeds the band.
3. **What a better-minted datum would and would not fix.** Storing a
   split's cut as a model-space length or point, not as `u`, removes
   the parameter floor. It does not move the evaluation's own ulp, so
   it does not bring a pose like the one above back into range.

Weigh this with designers first; it touches D4 ¶4's wording.

## What already appeals to the range

These argue reachability from "the session box (D4 ¶4)", a range this
item says no door enforces. Whatever the repair names becomes their
citation:

- `crates/geom-core/src/linalg/vec.rs`: the overflow and
  extreme-exponent notes at lines 170, 376, 431, 510 and 767.
- `work/frame/normalize-overflow-yields-zero-axis.md`
- `work/vgeom/sketch-headings-guard-zero-length-but-not-an-infinite-one.md`
- `work/encl/no-tighten-offer-quotes-a-value-below-the-runs-resolution.md`
  needs the same floor from the other side: a resolution below which no
  ε decides anything. The two should share one definition.
- `work/nurbs/revolved-point-eval-levers-angle-width-by-the-coordinates.md`
  (closed) measured the `RevolvedPoint` lever at a far placement.

## 2026-10-13: the box now has one home (ENCL, PR 4925)

PR 4925 implements D4 ¶4's ε floor. The session box is now a constant, geom-core `tolerance::SESSION_BOX_EXTENT` = 1 km, and `tolerance::EPS_MIN` = box · 2⁻⁵² ≈ 2.2e-13 m is derived from it. `Tolerance` refuses any ε below that floor. The rejection at construction is still unbuilt. That lane filed the finding as an ENCL row, and it is folded in here rather than kept as a duplicate.

### What

D4 ¶4 (`docs/DESIGN.md`) reads: "the session box is 1 km, no coordinate larger in magnitude; geometry outside the range is rejected at construction". The box now has one home, geom-core's `tolerance::SESSION_BOX_EXTENT`, and the floor `tolerance::EPS_MIN` is derived from it. Nothing reads the extent as a bound on geometry: no constructor, builder door or import door refuses a coordinate beyond 1 km.

`EPS_MIN` is only honest while the box holds. Past 1 km a coordinate's f64 spacing exceeds ε_min, so a run at ε_min decides lengths its coordinates cannot state.

### Where

- `crates/geom-core/src/tolerance.rs`, `SESSION_BOX_EXTENT`: read by `EPS_MIN`, by error text in `ToleranceError`'s and `editor-core`'s `EditError::InvalidTolerance` Display, and by nothing that sees a point.
- `crates/profile/src/lib.rs`, the crate docs' "Deferred (named): session-box enforcement (D4 ¶4)" paragraph, and `crates/profile/src/sugar.rs`, `bulge_from_via`'s "Session-box gap" note: construction sugar mints a ~1e15 m radius carrier that validates.
- `crates/profile/tests/review_m2_pr2.rs`, `annulus_at_ten_kilometers_validates`: a row that pins a 1e4 m scene validating. It is outside the box and moves when this lands.
- `crates/step-import/src/lib.rs`, `import_step`: an imported body is placed and adopted with no extent check.
- `crates/geom-core/src/linalg/vec.rs` and `affine.rs`: several docs argue "far outside the session box (D4 ¶4)" for overflow postures. They assume a box that no door enforces.

### Repair shape

There are two candidate homes for the check: a construction-time check at the doors that mint geometry (sketch/profile, builders, import placement), or a validator check at rest (`topo::validate`). D4 ¶4 says "at construction". The box's lever arm (`Band::angular_at`'s conservative universal arm) and the SSI scale levers (`work/ssimarch/ssi-scale-levers-cite-a-size-range-with-no-number.md`) then have a refusal to cite. Weigh where the check lives and what a box-crossing refusal names before implementing.

## 2026-10-14: seam note from GERM (`germ/pt-section-gaps`)

GERM closed `work/germ/plane-torus-section-gaps-decide-far-coordinates-uncharged.md`
into this item: the plane×torus section's misreads all sat at
`1e5`–`1e6` m, outside the box, and charging its accepting gaps with
the coordinates' resolution would be a tolerance growing with
position (D4 ¶1). Inside the box the section reads as at the origin.
For whoever builds the box gate:

- **(a) The far-frame rows run geometry outside the box.**
  `topo`'s `ring_path::far_frame_rows` (PR 5012) moves its islands to
  coordinates of about `1e6`. Once the gate exists, ask which of those
  rows still exercise reachable geometry, and whether PR 5012's
  per-site charge of `ring_path`'s `resolution` still earns its place
  inside the box. Its section-lane row has already moved inside
  (`a_torus_circle_the_section_lane_mints_in_the_box_reads_its_kind_at_the_origin`).
- **(b) K has no lower bound.** geom-core `tolerance.rs`
  `refuse_env_k` admits any finite K above 1, while the floor argument
  (a gap's few roundings at the box edge stay inside the escalation
  band at `EPS_MIN`) relies on K covering those few roundings. A K
  near 1 leaves no room for them. One site that leans on it with a
  difference of far dot products still in place: geom-brep `locus.rs`,
  `Section::faces_clear` reads `lo − at`, with `at` the locus's
  origin dotted with `across` and `lo`, `hi` the caller's face extents
  along it, each a far dot product.
- **(c)** The ruling and both designers' in-box measurements are in
  the GERM item above.
