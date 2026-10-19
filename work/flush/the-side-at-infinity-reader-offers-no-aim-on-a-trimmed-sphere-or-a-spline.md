---
id: the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline
kind: issue
title: The side-at-infinity reader offers no aim on a trimmed sphere, an untrimmable cone or a spline, so a shell of only those falls back to its volume
status: open
opened: 2026-10-14
priority: P2
cost: M
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
