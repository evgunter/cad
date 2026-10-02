---
id: interval-sin-theta-as-cross-over-norms-loses-the-shared-magnitude
kind: issue
title: an interval sin θ spelled |n1×n2|/(|n1||n2|) carries the gradient magnitudes in numerator and denominator, so its enclosure is wider than the angle's, and on the chain that width sets the certifiable box at every ε above the default
status: open
opened: 2026-10-02
priority: P2
refs: [SYM-15]
---


## What

`geom_brep::dihedral::wedge_decided` reads the angle between two
surfaces as `sin θ = ‖n1 × n2‖ / (‖n1‖·‖n2‖)`
(`crates/geom-brep/src/dihedral.rs:263`). At an interval scalar each
gradient's magnitude appears in the numerator AND the denominator, so
their widths do not cancel. The quotient's enclosure is wider than the
angle's own.

On the four-link chain (`demos/tour/src/chain.rs`, SYM-15) that width
is what sets the certifiable box at every ε above the default:
- At sample 4 of the tip pin's cap circle, the plane's normal and the
  cylinder's axis are both exactly `±z`, and the radial `w ⊥ axis`. So
  over the boxes the gradients are enclosed on, the true image of
  `sin θ` is `{1}`.
- The quotient's lower end instead falls as `r·(1 − x)/(1 + x)`, with
  `x` the box over the one where the radial enclosure reaches zero.
- The box stops certifying where that lower end drops under `K·ε`. At
  the default ε that crossing sits at the poison itself, so the poison
  bounds the box there. At `1e-6` and `1e-5` it comes earlier, and the
  wall is a straddle (SYM-15, the chain item's "What Phase 1 found").

**Measured by SYM-15's review** (probe branch `sym/15-review` @
`e282874edc`, env `SYM15_COSFORM`, 2026-10-02):
- Spelled through the cosine instead, `sqrt(1 − cos²)` with
  `cos = n1·n2/(‖n1‖‖n2‖)`, the chain's box stops moving with ε: 4 links
  `1.10961e-1` and 2 links `3.69207e-1` at `1e-9`, `1e-6` and `1e-5`
  alike. SYM-15's own measurement with the cross spelling (the chain
  item's table) gives 4 links `1.10961e-1` / `1.08290e-1` /
  `8.62605e-2` and 2 links `3.70208e-1` / `3.60318e-1` / `2.87797e-1`
  at the same three ε.
- That spelling is not shippable as it stands. It reds three
  near-tangent lib rows, where `1 − cos²` loses the small angle to
  cancellation (re-run by SYM-15 with the review's spelling under
  `SYM15_COSFORM=1 cargo nextest run -p geom-brep --lib`, 2026-10-02):
  - `certify::tests::tangent_intersection_is_refused`;
  - `dihedral::tests::a_wedge_in_the_zero_band_at_the_arms_tolerance_leaves_the_arm_binding`;
  - `dihedral::tests::near_tangent_planes_escalate`. At two links it poisons `EdgeKey(5v1)` about 0.3 %
  before the poison point.

## The class

The class is any interval `‖a × b‖ / (‖a‖·‖b‖)`, or a sine read off
normalised vectors, where a shared magnitude sits on both sides.
- Hit list, grepped for `.cross(…).norm() /` and `sin_theta =` over
  `crates/*/src`:
  - `geom-brep/src/dihedral.rs:263` (`wedge_decided`, this row);
  - `geom-brep/src/ssi/march.rs:563` (`ssi_transversality`, the same
    spelling);
  - `geom-brep/src/tangent.rs:79` (the tangent jet's `sin θ`);
  - `geom-brep/src/edge_nurbs.rs:788` (`normal_angle_sine`);
  - `sweep/src/blend/battery.rs:729` (`sin θ` of two normalised
    tangents);
  - `sweep/src/blend/battery.rs:2230`, a point-line distance
    `‖(foot − o) × d‖ / ‖d‖` with `d` on both sides.
- None is changed here.
- **Blind spot:** a spelling through intermediate names that are not
  `sin_theta`, or through `normalize()` before the cross product, is
  matched only where the second pattern caught it.

## Candidates

- **Intersect the two sound enclosures.** Both the cross form and the
  cosine form enclose the true `sin θ`, so their intersection is sound
  and only narrows. It takes the cosine form's width where the angle is
  large, and the cross form's where it is small, the near-tangent rows.
- **A form with the magnitudes factored out**, for example the cross
  product of unit vectors built once. This moves the dependency rather
  than removing it, so it is only worth having if measured.

## Home

PROPS: enclosure certificates and interval honesty. Filed by SYM-15.
