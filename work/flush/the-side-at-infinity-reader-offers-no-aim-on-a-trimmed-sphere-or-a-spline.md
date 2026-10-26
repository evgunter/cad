---
id: the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline
kind: issue
title: The side-at-infinity reader offers no aim on a trimmed sphere, an untrimmable cone or a spline, so a shell of only those falls back to its volume
status: closed
opened: 2026-10-14
priority: P2
cost: M
branch: flush/sphere-spline-aims
pr: 5730
closed: 2026-10-26
---


Filed by FLUSH's shell-role reader unit (`flush/shell-role-reader`):
the residue of the ratified design's curved step (DESIGN D10
"Booleans", D1 tier 3 check 7; fork-log row 138).

## What

`boolean/solid_contain.rs` `side_at_infinity`, the one reader of a
shell's role and of a solid's orientation, aims (`aim_at`) at a planar
face's certified interior point and at the middle of a cylinder's,
cone's or torus's chart window, or a fixed point of a closed sphere's
chart, uncertified: the ray's start lies beyond every face, so an aim
off the face only steers it (`report-A-round3.md`'s argument). A
trimmed sphere face (`FaceGeo::SpherePatch`), a cone face the door
cannot trim (`FaceGeo::PartialCone`) and a spline face offer none. A
shell or solid none of whose faces offers an aim that reads falls back
to its volume's sign (`props::shell_volume_role`, `validate.rs`
`plus_v_errors`): the thinness ruler D10 retired, for those shells only.
A body thinner than any face of it can hold a certified point (a needle
a few `Kε` across, `sweep/tests/review_blend_k_rk_probes.rs`) offers no
planar aim either, and refuses its orientation at rest.

## The step

An aim on a trimmed sphere (a point of its region, `SphereFaceRegion`)
and on a spline (its chart's window middle) needs no new theory; an
untrimmable cone has no window to aim in. The ray walk crosses spheres
already and has no spline arm, so a spline shell keeps the volume until
it does.

Absorbs CLEAVE's
`a-sliver-operands-orientation-falls-back-to-its-volume-where-it-has-no-planar-face`
(its operand gate is retired; its first finding is this row, narrowed).

## Measured (`flush/sphere-spline-aims`, on main at `cbbc206c`)

`sweep/tests/trimmed_sphere_aims.rs`'s fixtures, built through the
plain booleans from `y`-poled balls (each passes `validate_pseudomanifold`
with no contacts at ε 1e-9, 1e-6 and 1e-12): a lens (unit ball ∩ a
0.8 ball at `z = 0.9`), the two balls' union, the unit ball less the
other and the other less the unit ball, the lens and the bitten ball
again with the second ball off the axis at `(0.13, 0.07, 0.9)`, and a
4-cube less the lens and less the two balls (each a cavity shell of
two trimmed sphere faces). Every sphere face of them is a
`FaceGeo::SpherePatch`: two sphere surfaces, so no closed group.

On main, `side_reader.rs` `aim_at` answers `None` for every
`SpherePatch` face at every ε (its arm is the match's `_`, before any
ε-dependent read): **6 of 6 solid selections and 2 of 2 cavity shells
offer no aim** (`Unread { aimed: 0, set_aside: [] }`) and fall back to
the volume; the blocks' outer shells read off their planar faces. Probe
lines at ε 1e-9: every sphere face "no aim".

## Fix

`boolean/sphere_region.rs` `SphereFaceRegion::aim`: each boundary arc's
middle stepped along its great circle toward the face (`N × τ`, the
region's own convention for which side is the face) by half its
circle's angular radius, the steps' mean carried radially onto the
sphere. It is evaluation, not a decision: the aim only steers, since
the ray starts beyond every face (fork-log row 138). `aim_at` takes it
for `FaceGeo::SpherePatch`. After: every sphere face of every fixture
offers an aim, **each aim reads its side** (`Side(Out)`), every solid
and outer shell reads `Out` and both cavities `In`, at ε 1e-9, 1e-6
and 1e-12 (`trimmed_sphere_shells_read_their_side_off_aimed_rays` pins
it; `probe_trimmed_sphere_aims` prints it).

**The untrimmable cone** (`FaceGeo::PartialCone`), and the untrimmable
wall (`FaceGeo::PartialWall`), have no window to aim in and keep the
volume; `aim_at`'s docs say so.

**The spline** gets no aim here. `cast_aims` casts no ray at all at a
selection holding a face the ray test has no arm for (`uncrossable`,
`ray_crosses`: `Nurbs`, `Approx`), so an aim on a spline face would
never be cast: a spline shell, spline-only or not, reads its volume.
The arm and the aim are filed together on INSIDE's slate as
`point-containments-ray-test-has-no-spline-arm`.

## Review fix pass (2026-10-26)

The review (APPROVE-WITH-FIXES) found the first aim, an unweighted mean
of each arc's step of half its circle's angular radius, landing outside
the face on zones with opposite seams, a zone a thousandth wide, rims in
unequal arc counts, a symmetric zone and a ringed cap. In bodies, all
four aims of a sliver lens (unit balls 1.999 apart) missed and the
solid read its volume.

`SphereFaceRegion::aim` now steps from a point of each arc (the golden
section of its span) by an angle bounded by half its own circle's
angular radius and by half a lower bound on that point's distance from
every other arc (`clearance`: the greater of the distance to the arc's
whole circle and a sampled bound), and takes the deepest step. It is
evaluation alone and lands in the face by construction.

- Every one of the review's 22 region shapes, in both senses, aims inside
  the face (`the_aim_lies_in_the_face` pins them, with unequal arc
  counts and a ring beside a great rim).
- Over the review's 32 bodies, 157 faces, at ε 1e-9, 1e-6 and 1e-12:
  every sphere face's aim reads its side, and every solid reads. The one
  face set aside is the bowl's (a graze), which holds no sphere.
- The sliver lens and a block less it join
  `trimmed_sphere_shells_read_their_side_off_aimed_rays`.
- Exact ties at the nominal: on
  `the-side-readers-aims-decide-exact-ties-at-a-documents-nominal`.

No region shape the review or this pass built still lands outside, so
CLEAVE's absorbed sliver row stays closed. Still on the volume:
- a sphere patch with no boundary arc;
- a face whose outward normal at the aim is refused;
- untrimmable cones and walls;
- a selection with a spline face (INSIDE's
  `point-containments-ray-test-has-no-spline-arm`);
- a selection with a sphere face bounded by a non-circle edge
  (`PartialSphereFace`, appended to RESTFRONT's
  `check-10-is-silent-where-point-in-solid-refuses`).

## Resolution

Closed by PR 5730 (2026-10-26). A trimmed sphere face (`FaceGeo::SpherePatch`) now offers an aim (`SphereFaceRegion::aim`), so a shell or solid made only of trimmed spheres reads its side off aimed rays instead of its volume.
- **How the aim is built.** It is evaluation only, and it lands inside the face by construction. From the golden section of each boundary arc, it steps along the great circle into the face. The step is capped by half the arc's own circle's angular radius and by half a lower bound on the distance to every other arc. The deepest step is the aim.
- **Why no certificate is needed.** An aim only steers: the ray starts beyond every face.
- **What does not aim.**
  - A spline gets no aim, because `cast_aims` casts no ray at a selection holding a spline face. The ray arm is filed on INSIDE as `point-containments-ray-test-has-no-spline-arm`.
  - An untrimmable cone or wall keeps the volume.
  - A sphere face with a non-circle edge is appended to RESTFRONT's `check-10-is-silent-where-point-in-solid-refuses`.
- **CLEAVE's absorbed row closes with this one.** That is `a-sliver-operands-orientation-falls-back-to-its-volume-where-it-has-no-planar-face`: the sliver lens now reads off rays.

One FULL review. The first head's aim, an unweighted mean of arc steps, landed outside zones and slivers, and the sliver lens still read its volume. The fix pass rebuilt it; the delta check approved, finding no break in 98 shapes and nothing moving between Out and In across 32 bodies at 3 ε. The new exact ties at the nominal on symmetric sphere bodies (all `bool_pierce_normal_on_chart`) are recorded on `the-side-readers-aims-decide-exact-ties-at-a-documents-nominal`. Batteries: 0 lines moved, though they hold no trimmed sphere.

