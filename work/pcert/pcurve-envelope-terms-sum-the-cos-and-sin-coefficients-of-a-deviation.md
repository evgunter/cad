---
id: pcurve-envelope-terms-sum-the-cos-and-sin-coefficients-of-a-deviation
kind: issue
title: "pcurve envelope: the plane arm and the Tilt and Drift terms sum a harmonic deviation's cos and sin coefficients, a lever up to 2 on the point deviation"
status: open
opened: 2026-10-08
priority: P2
cost: E
---

(TANG implementer, the sweep of
`work/tang/a-rim-offset-half-the-zero-band-builds-in-one-member-order-only.md`.
The file is PCERT's.)

## What

A harmonic deviation `d(t) = da·cos t + db·sin t` has the exact sup
`σ_max([da db]) = √(m + √(q² + (da·db)²))`, with
`m = (‖da‖² + ‖db‖²)/2` and `q = (‖da‖² − ‖db‖²)/2`; for scalars
`aa·cos t + bb·sin t` it is `√(aa² + bb²)`. Several of check 4's terms
in `crates/geom-brep/src/pcurve_cache.rs` bound it by `‖da‖ + ‖db‖`
instead. That overshoots the point deviation by up to 2 (a radius
change: `da ⊥ db`, equal lengths) or `√2` (the scalar tilt). D4 reads
margins as point deviations, so a row whose carrier stands `δ` off reads
up to `2δ` and escalates from `δ = zero/2`.

Sites (names first; line numbers at `a29f4b36c`):

- check 4's plane arm in `certify_harmonic`, `d_c.norm() + d_a.norm() +
  d_b.norm() + …` (`:3868`). A circle of radius `R` imaged against a
  carrier of `R + δ` reads `2δ`;
- `incidence`, the `Drift` terms `a_r.norm() + b_r.norm()`:
  `CylinderMoving { Zero }` and `CylinderMeridian` (`:4218`, `:4222`),
  `ConeRim { Zero }` (`:4275`), `SphereParallel { Zero }` (`:4310`), and
  the torus parallel (`:4365`);
- `incidence`, the `Tilt` terms `|a·n| + |b·n|`: `ConeRim` (`:4266`),
  `SphereParallel` (`:4302`), the torus parallel (`:4357`). A circle
  tilted by `φ` reads up to `√2·R·sin φ` for a point deviation of
  `R·sin φ`.

## Measured

The shape cost a member-order symmetry once. Before PR 3812 restated
the periodic envelope, the cylinder and sphere arms read a radius offset
as `‖Δa‖ + ‖Δb‖ = 2δ`. A dome rim `δ = zero/2` off the tube's wall
(PR 3823's fixture) therefore sat exactly on the envelope's flip, and
rounding chose: the sphere row read `0.99999964·zero` and built, the
cylinder row `1.00000008·zero` and refused `Merge(Pcurve … Envelope)`.
Which row the merge certifies follows the member order (the REST zip
keeps the first operand's seam edge), so one order built and the other
refused. PR 3812's `Radius` term reads `δ` once, and both build; the
mutant that doubles it reproduces the refusal
(`sweep` `pi_seam_and_kiss_through_the_boolean::a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders`).

The sites above are the same shape and are **unmeasured**: no fixture
here reaches them with a deviation between `zero/2` and `zero`.

## What it costs

Over-refusal only, the safe direction: a row whose point deviation is
inside the zero band escalates. Where two operands' rows bound one
seam, the order asymmetry above can return, as a knife-edge at a
fraction of the band.

## Fix

Bound each pair by its exact sup, which is a closed form. Each site
then needs a row with a deviation in `(zero/2, zero)` that certifies,
and a mutant restoring the sum that refuses it.
