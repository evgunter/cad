---
id: blend-arms-perp-unit-is-an-undecided-gram-schmidt
kind: issue
title: blend arms' perp_unit is a hand Gram-Schmidt on an undecided normalize
status: closed
opened: 2026-10-01
priority: P2
cost: E
closed: 2026-11-23
pr: 7625
branch: onehome/mints-sweep
parent: carrier-frames-the-production-mints
---


## What

`crates/sweep/src/blend/arms.rs` `perp_unit(x, a)` is a Gram–Schmidt
step written by hand — `(x - a * a.dot(x)).normalize()` — whose
residual length is never decided. Its doc says poison propagates when
`x` is parallel to `a` ("the honest answer"), and that is D2 row 3 for
a value; but the result is the seam reference of every chart built
below it, so a residual that is merely SHORT (inside the band, not
exactly zero) normalizes to a definite direction made of rounding,
and nothing downstream can tell.

## Shape

`geom_core::RigidMotion::from_aim_and_reference` (or `UnitVec3::new` on
the residual) is the ladder written once: the residual's length
decided under a K name the blend lane owns, refusing typed
(`OrthonormalizeError` naming `OrthoAxis::V`) where it decides zero or
escalates in band. Found by the `linalg/decided-not-minted` sweep for
the hand Gram–Schmidt shape (`- a * a.dot(x)).normalize()`); the
`linalg` branch fixed the same shape in `geom-brep`'s
`torus_meridian_orient`.

## Callers (2026-11-04)

`onecopy/p0-sheared-fillet` adds one: `open::end_face`'s perpendicular
cap lays the band cylinder's `u_ref` into the cap plane with it. There
the residual is about 1 (the cap's normal is the band's axis, decided
`Zero` apart by the battery's `EndSection::Circle`), so it is not the
short-residual case this row is about, but the row's fix reaches it.

## Moved to ONEHOME and closed (2026-11-23)

Claimed by `carrier-frames-the-production-mints` (branch
`onehome/mints-sweep`, group G1). Every chart seam this row named is now
minted by a decided door, under `fillet3_carrier_frame`
(`blend::arms::frame_about`, `frame_along`): the plane–sphere torus,
the corner chart (`corner_chart` returns the minted frame, so
`octant_chart` does too), the chamfer's corner patch, the meridian
sheet (`Meridian::new` holds one decided frame and every arm reads it),
and `open::end_face`'s perpendicular cap circle. A residual in band
escalates; a definite zero is the surgery's own invariant. `perp_unit`
survives only as a point's azimuth about a held axis
(`chamfer_setback_point`, `frustum_rim_side`), which no chart takes as
its seam.
