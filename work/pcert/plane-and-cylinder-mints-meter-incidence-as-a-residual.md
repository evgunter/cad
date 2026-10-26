---
id: plane-and-cylinder-mints-meter-incidence-as-a-residual
kind: issue
title: The plane and cylinder mints meter a carrier's incidence as a residual, so an off-face carrier ends ChartResidual's defect whoever placed it
status: open
opened: 2026-10-25
---


**E1c's item** (`work/pcert/plan.md`). This is E1b's finding 8, deferred by the orchestrator's ruling (c).

## What happens

The sphere, cone and torus mints run `incidence::incident` (`crates/geom-brep/src/pcurve_cache.rs`, `mod incidence`) before they image a carrier. A carrier that is definitely off the chart ends `CarrierOffChart`, which E1b routes by `EdgeAuthority`: lever-only on a declared curve, a defect on a derived one.

The plane and cylinder mints do not run it in general:

- **Plane.** `derive_class`'s `Surface::Plane` arm maps any carrier's harmonic coefficients through the affine chart. It reads no incidence.
- **Cylinder.** `derive_class`'s `Surface::Cylinder` arm reads incidence only where `pcurve_chart_orientation` is in band (`incidence::general_lane`, E1's P1). The other arms take the carrier as given:
  - the meridian arm (`pcurve_chart_radial_moving` Zero, and its in-band tie-break);
  - the decided moving arm (β = ±1).

The carrier's miss is then read by the certificate:

- by check 3's residual schedule (`check_residual_as` in `run_harmonic_checks`);
- at the edge, by `certify.rs`'s `CertCheck::ChartResidual`, which ends `Ending::Residual(Unsized::Defect)`.

So a carrier off a plane or cylinder face ends as a defect whoever placed it, and E1b's authority routing never sees it.

## Measurement (E1b)

A sketch arc set onto a face it does not lie on, through `set_edge_curve`:
- off a cylinder's wall;
- off a plane side of a box.

Both refuse `ChartResidual` at sample 0 with margin 0.707: the arc's distance from the face, read as the residual's defect. On a sphere, cone or torus the same move ends `CarrierOffChart`, lever-only for a declared curve after E1b.

## The ruling

Of finding 8's three options, the orchestrator took **(c)**: the plane and cylinder mints read incidence before they image, so an off-face carrier ends `CarrierOffChart` as on the other charts. It is its own unit, E1c, after E1b.

**(a) is rejected**: routing `ChartResidual` itself by authority. `ChartResidual` also meters the image's own fidelity, so sending it to the modeler's lever could hide a kernel image bug.

Until E1c lands, D3's incidence clause holds only on the charts whose mint reads incidence: sphere, cone and torus.

## Overlap with E2

The cylinder half overlaps E2. E2 gives a cylinder line a projected lane, and it re-reads the cylinder `pcurve_chart_radial_moving` tie-break: the meridian arm's `Ok(Sign::Zero) | Err(_)` fold, which is where a cylinder carrier's incidence would be read first. E1c's cylinder incidence test and E2's tie-break work therefore touch the same arm. Whichever lands second re-reads it. If E2 is dispatched first, it can absorb the cylinder half of E1c, leaving E1c the plane.

## Done when

- A carrier definitely off a plane or cylinder face ends `CarrierOffChart` at the mint, at both lanes and every ε. It is routed by E1b's authority split: lever-only on a declared curve, a defect on a derived one.
- `ChartResidual` is left metering the image's own fidelity.
- A row holds the set_edge_curve arc off a cylinder wall and off a plane side.
