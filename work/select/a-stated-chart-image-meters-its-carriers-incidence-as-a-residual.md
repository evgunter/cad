---
id: a-stated-chart-image-meters-its-carriers-incidence-as-a-residual
kind: issue
title: A stated chart image of a carrier off its face ends ChartResidual, not the incidence read: only the mint reads incidence first
status: open
opened: 2026-10-28
---



Found by PCERT's E1c sweep (`pcert/mints-read-incidence`; closed item
`work/pcert/plane-and-cylinder-mints-meter-incidence-as-a-residual.md`).

## What happens

E1c makes every chart mint read the carrier's incidence before it images
the carrier (`derive_class`, `spiric_chart_pcurve` and
`incidence::incident` in `crates/geom-brep/src/pcurve_cache.rs`). An
off-face carrier then ends as its incidence read, by E1d's door rule
(`IncidenceRead::ending`).

A description can also STATE its image instead of minting it:
`EdgeDescriptionSpec::chart_image` and `EdgeDescriptionSpec::iso` in
`crates/geom-brep/src/description.rs`. `EdgeCurve::certify`
(`crates/geom-brep/src/certify.rs`) certifies a stated image and never
reads the carrier's incidence. So an off-face carrier with a stated image
is metered by the residual and ends `CertCheck::ChartResidual`'s defect.

At the import door that means the ε_in offer, not the damaged file. A
read in band ends as a defect, not in its lever. These are the endings
E1c removed from the mint.

## Measurement (E1c, f64, main at c591ab53c7 plus E1c)

`EdgeCurve::certify` with `EdgeDescriptionSpec::chart_image`:
- A plane `z = 0` with the stated `IsoLine { p0: (0, 0), pl: (1, 0) }` and a carrier line `z = 0.5` along `x`:
  `ResidualExceeded { check: ChartResidual, sample: 0, margin: 0.5 }`.
- A unit cylinder about `z` with the stated `IsoLine { p0: (0, 0), pl: (0, 1) }` and a ruling at radius 2:
  `ResidualExceeded { check: ChartResidual, sample: 0, margin: 1.0 }`.

The same carriers described by `EdgeDescriptionSpec::chart` (minted) end
`CarrierOffChart` under `ChartIncidence`.

## Repair shape (open)

Read the carrier's incidence on the stated image's chart before the
stated image is metered, as the mint does. Only then does
`ChartResidual` meter the image's own fidelity (E1c's ruling (a) rejected
routing `ChartResidual` by what was read).

The cost is one sampled incidence test per stated-image edge. Callers:
builders' iso boundaries, and the import door's adopted pcurves, which
are EXCH's path.
