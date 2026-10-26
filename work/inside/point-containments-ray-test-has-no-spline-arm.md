---
id: point-containments-ray-test-has-no-spline-arm
kind: issue
title: Point containment's ray test has no arm for a spline face, so a selection holding one casts no aimed ray and reads its side off its volume
status: open
opened: 2026-10-25
priority: P3
cost: H
---


Filed by FLUSH's `flush/sphere-spline-aims`
(`work/flush/the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline.md`),
whose spline step it is the residue of.

## What

`boolean/solid_contain.rs` `ray_crosses` lists the surface kinds the
ray test has an arm for; `SurfaceKind::Nurbs` and `SurfaceKind::Approx`
are not among them, so `face_geo` refuses such a face
`KindUnsupported` through `uncrossable` on every ray. The side
reader (`side_reader.rs` `cast_aims`) therefore casts no ray at a
selection holding one: every ray would fault on that face whatever it
aimed at. Any shell or solid with a spline face, spline-only or not,
reads its role and orientation off its volume
(`side_reader.rs` `side_at_infinity`'s fallback), the reading D10
keeps only where no aim reads.

An aim on a spline face (its chart's window middle) is no use until a
ray can cross the face, so the side reader offers none: it belongs to
this row, added beside the arm.

## The step

A ray–spline crossing arm in `face_geo` / `ray_crossings` (a certified
root count of the ray against the patch, each root tested against the
face's trim), `ray_crosses` growing `Nurbs`, and an aim at the face's
chart window middle in `side_reader.rs` `aim_at`. Check 10's silence on
such faces (`work/restfront/check-10-is-silent-where-point-in-solid-refuses.md`)
closes with the same arm.
