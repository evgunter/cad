---
id: the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline
kind: issue
title: The side-at-infinity reader offers no aim on a trimmed sphere, an untrimmable cone or a spline, so a shell of only those falls back to its volume
status: review
opened: 2026-10-14
priority: P2
cost: M
branch: flush/sphere-spline-aims
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
