---
id: a-two-surface-residual-meters-a-moved-face-as-the-descriptions-miss
kind: issue
title: An edge's two-surface residual meters a face moved off it as the description's miss: in band it ends in the kernel-limit recourse, not the incidence lever
status: open
opened: 2026-10-28
---



Found by PCERT's E1c (PR 5889, the review's L3 sweep of the at-rest
readers). Owner: select, which owns the description ladder.

## What happens

An edge described on a pair of faces (`Resolved::Intersection` in
`run_checks`, `crates/geom-brep/src/certify.rs`) meters its carrier
against each surface with the sampled implicit residual
`carrier_on_surface_1` / `carrier_on_surface_2`, refusing as
`CertCheck::SurfaceResidual { surface }`. It reads no incidence first.
So when one of the two faces is moved off the edge, the miss is
metered as the description's residual and ends by `SurfaceResidual`'s
table, not by `IncidenceRead::ending`:

| door | definite (0.5 m) | in band (5.5e-9 m at ε 1e-9) |
|---|---|---|
| build | loosen the tolerance, last resort | loosen the tolerance, last resort |
| at rest | kernel defect or damaged file | loosen the tolerance, last resort |
| import, ε_in 1e-6 | the damaged file | the ε_in stopgap offer |
| import, ε_in 10 | the ε_in stopgap offer | the ε_in stopgap offer |

The incidence read ends these as the kernel's defect at a build, the
stored culprit at rest and the damaged file at import when definite,
and in its lever (`INCIDENCE_UNSHOWN`) at every door when in band. The
chart description (`ChartIncidence`, E1c) and the planar tier's boundary
(E1c's L3(2)) already end that way, so one moved face ends its edges
three ways at rest: the pcurve rows and the planar boundary in the
lever, the pair description in the kernel-limit recourse.

## Measurement (E1c, f64, ε 1e-9)

A unit square extruded 2 m (`sweep::test_support::extruded`), its `+x`
side plane moved along its normal in place
(`Body::with_surface_moved_for_tests`), then `AtRestBody::validate`:
each of the side's four edges refuses `EdgeCertification` with
`ResidualExceeded { check: SurfaceResidual { surface: <the side> } }`
at 0.5 m (margin −0.5) and `Escalated { check: SurfaceResidual,
predicate: carrier_on_surface_2 }` at 5.5e-9 m (mid-band). The table
above is `CertifyError::ending` at `Reading::Build`, `Reading::AtRest`
and `FileCoincidence::new(1e-6)` / `new(10.0)` on those refusals. The
mid-band probe is the in-band row of
`sweep a_face_moved_off_its_edges_ends_as_their_incidence_read_at_rest`
(which reads the pcurve and planar findings and leaves this one out).

**At the import door** the same meter answers first. `freecad/cylinder.step`
with its wall `#45 = CYLINDRICAL_SURFACE('',#46,0.5)` grown to radius 1
refuses `Adoption` at the top rim `#21`, and its attempts never reach a
chart rung:
- `Intersection`: `ResidualExceeded { check: SurfaceResidual, margin: −3.75e-4 }`;
- `TangentIntersection`: `ResidualExceeded { check: TangentParallel, margin: 1e-3 }`.

It ends "the file is damaged", from the residual's table, not the
incidence read's. Grown to `0.5000000055` (mid-band at ε 1e-9) the file
imports. The import reaches the chart mint only through a
`MappedCurve` candidate (a file that states a pcurve) and the plane ×
spline rim (`crates/step-import/src/adopt.rs`).

## Repair shape (open)

Read the carrier's incidence on each of the pair's surfaces before the
two residuals meter it, as the chart arm does since E1c (`read_incidence`
reads an analytic chart; a spline surface of the pair reads nothing
there and keeps its residual). The residual is then left metering the
description's own fidelity. Whether a pair whose two surfaces are both
read on keeps `SurfaceResidual` at all is the design question.
