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
