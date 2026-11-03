---
id: geom-brep-public-certify-doors-end-caller-input-as-the-kernels-defect-at-a-build
kind: issue
title: geom-brep's public pcurve certify doors end a caller's input as the kernel's defect at a build
status: open
opened: 2026-11-03
priority: P3
cost: M
---


(PCERT implementer, from the review of PR 6448, `pcert/certify-arms-end`,
fix Q1; the orchestrator's ruling.)

## What

`geom_brep::PcurveCache`'s certify doors (`certify`, `certify_general`,
`certify_fitted`, `certify_projected`, `crates/geom-brep/src/pcurve_cache.rs`)
are public, and reachable from pncad through its `geom_brep` re-export.
A caller hands them an image and a carrier it built. Read at
`Reading::Build`, a refusal of that input ends in the kernel's defect
wherever its decision routes `Unsized::Defect` (or a definite incidence
miss, E1d's door rule), though the input was the caller's. Measured on
`pcert/certify-arms-end` (`PcurveCertifyError::render(Reading::Build)`):

- `ImageMismatch { image: Fitted, .. }` (an analytic chart at
  `certify_general`, the `is_analytic` check):
  "pcurve certification: the Fitted image offered is not its carrier's
  image at this door — …. There is no way through: this is a kernel
  defect; report it"
- `CarrierOffChart { margin: None, .. }`: "… does not lie on the cylinder
  chart — …. The edge is not on its face. There is no way through: this is
  a kernel defect; report it"
- `ResidualExceeded { check: ImageIdentity, .. }`: "… definitely exceeds
  the tolerance band — the pcurve does not represent the carrier through
  the map …. There is no way through: this is a kernel defect; report it"
- `PlacementTurnsExceeded` and `SectorRefused`: the same ending.
- For contrast, `ResidualExceeded { check: MapResidual, .. }` ends in the
  last resort ("Recourse: loosen the tolerance, as a last resort; …"),
  since `MapResidual` routes `Unsized::Fit`.

The one wrong-door case, a fitted-grade image at the closed-form door, has
its own arm since PR 6448 (`FittedGradeAtClosedFormDoor`, the hand-off
lever).

## Repair shape

The door, not the refusal, knows whose input it read: a mint's own image
(the kernel's) or a caller's (the caller's). Decide whether the public
certify doors read at a reading of their own (a caller's input, as the
import door reads a file's), or whether they leave the public API.
