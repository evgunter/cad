---
id: plane-and-cylinder-mints-meter-incidence-as-a-residual
kind: issue
title: The plane and cylinder mints, and the sphere, cone and torus class arms, meter a carrier's incidence as a residual, so an off-face carrier ends ChartResidual's defect
status: open
opened: 2026-10-25
---


**E1c's item** (`work/pcert/plan.md`). E1b's finding 8, deferred by the
orchestrator's ruling (c), widened by review A on E1d to the class arms of
the sphere, cone and torus mints.

## What happens

A carrier off its face should end as its incidence reading does
(`CarrierOffChart`, or `Escalated { ChartIncidence }`), which E1d ends by
the door that reads it (`IncidenceRead::ending`): a definite miss in the
door's defect (the kernel's at a build, the stored culprit at rest, the
damaged file at import), a read in band in the geometry's lever at every
door. That holds only where the mint runs the incidence test
(`incidence::incident`, `crates/geom-brep/src/pcurve_cache.rs`,
`mod incidence`) before it images the carrier. These mints do not:

- **Plane.** `derive_class`'s `Surface::Plane` arm maps any carrier's
  harmonic coefficients through the affine chart. It reads no incidence.
- **Cylinder.** `derive_class`'s `Surface::Cylinder` arm reads incidence
  only where `pcurve_chart_orientation` is in band (`incidence::general_lane`,
  E1's P1). The meridian arm (`pcurve_chart_radial_moving` Zero, and its
  in-band tie-break) and the decided moving arm (β = ±1) take the carrier
  as given.
- **The sphere, cone and torus CLASS arms** (review A, E1d): the sphere's
  polar circle, the cone's rim and the torus's parallel image a carrier
  their class gates accept (the carrier plane ⊥ the axis, centred on it)
  without reading incidence. The class gates read the plane and the
  centre, not the radius, so a circle of the wrong radius passes them.

The carrier's miss is then read by the certificate:

- by check 3's residual schedule (`check_residual_as` in `run_harmonic_checks`);
- at the edge, by `certify.rs`'s `CertCheck::ChartResidual`, which ends
  `Ending::Residual(Unsized::Defect)`.

So such a carrier ends `ChartResidual`: a definite miss is right only by
accident at a build and at rest, an in-band read ends as a defect, not in
its lever, and at the import door a miss within ε_in is offered setting ε
to ε_in, where the door rule gives the damaged file or the lever.

## Measurement (E1b)

A sketch arc set onto a face it does not lie on, through `set_edge_curve`:
- off a cylinder's wall;
- off a plane side of a box.

Both refuse `ChartResidual` at sample 0 with margin 0.707: the arc's
distance from the face, read as the residual's defect. On a sphere, cone
or torus the same move, off every class, ends `CarrierOffChart`.

## The ruling

Of finding 8's three options, the orchestrator took **(c)**: the mints
read incidence before they image, so an off-face carrier ends as its
incidence read, as on the charts that read it. It is its own unit, E1c,
after E1d2 and before E1e (`plan.md`).

**(a) is rejected**: routing `ChartResidual` itself by what was read.
`ChartResidual` also meters the image's own fidelity, so ending it in a
lever could hide a kernel image bug.

Until E1c lands, D3's incidence clause is not finished on any chart: on
the sphere, cone and torus it holds only on the carriers their class gates
send to the incidence test.

## Overlap with E2

The cylinder half overlaps E2. E2 gives a cylinder line a projected lane,
and it re-reads the cylinder `pcurve_chart_radial_moving` tie-break: the
meridian arm's `Ok(Sign::Zero) | Err(_)` fold, which is where a cylinder
carrier's incidence would be read first. E1c's cylinder incidence test and
E2's tie-break work therefore touch the same arm. Whichever lands second
re-reads it. If E2 is dispatched first, it can absorb the cylinder half of
E1c.

## Done when

- A carrier off a plane or cylinder face, or one a sphere, cone or torus
  class arm accepts while off the face, ends as its incidence read at the
  mint, at both lanes and every ε, never `ChartResidual`.
- That read ends by the door rule (`IncidenceRead::ending`): a definite
  miss is the kernel's defect at a build, the stored culprit at rest and
  the damaged file at the import door, with no ε_in offer; a read in band
  ends in the geometry's lever at every door, never a defect.
- `ChartResidual` is left metering the image's own fidelity.
- Rows hold the `set_edge_curve` arc off a cylinder wall and off a plane
  side, and a wrong-radius circle on each class arm (sphere polar circle,
  cone rim, torus parallel), each read at a build, at rest and at import.
