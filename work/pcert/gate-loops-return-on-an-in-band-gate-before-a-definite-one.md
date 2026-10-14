---
id: gate-loops-return-on-an-in-band-gate-before-a-definite-one
kind: issue
title: Eight geom-brep gate loops return on an in-band gate through ? before a later gate's definite refusal speaks
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## What

Found by the sweep of the PCERT unit that closed
`an-in-band-gate-masks-a-definite-one-in-the-spiric-and-villarceau-ladders`
(2026-10-14). Each loop below decides a list of gates through
`decide(...).map_err(...)?`. A definite reading refuses with a named
refusal, and the first in-band reading returns as an escalation. So an
in-band gate read early masks a later gate's definite refusal, and the
op refuses with the wrong ending: a tolerance offer, or a defect, where
a datum it has not read yet already decides the refusal and names its
lever.

- `crates/geom-brep/src/intersect.rs:2060` — cylinder × sphere
  degeneracy guard (`cs_cylinder_radius`, `cs_sphere_radius`):
  `RadiusEscalated` before `DegenerateOperand`.
- `crates/geom-brep/src/intersect.rs:2284` — plane × cone
  (`pn_aperture_sin`, `pn_aperture_cos`, `pn_seam_off_axis`):
  `Escalated` before `DegenerateOperand`.
- `crates/geom-brep/src/intersect.rs:3080` — cone × circle
  (`coc_aperture_sin`, `_cos`, `coc_seam_off_axis`): the same shape.
- `crates/geom-brep/src/pcurve_cache.rs:3658` — the winding lane's
  trigonometric coefficients (`affine_name`): `ChartWinding`'s
  escalation before `ChartWindingUnsupported`.
- `crates/geom-brep/src/pcurve_cache.rs:3895` — the azimuth-period
  gates (`pcurve_azimuth_period`, azimuth then tube): `AzimuthPeriod`'s
  escalation before `AzimuthPeriodExceeded`.
- `crates/geom-brep/src/pcurve_cache.rs:5045` — the azimuth- and
  tube-period gates of the focal-section and spiric lanes: the same.
- `crates/geom-brep/src/pcurve_cache.rs:5347` — the spiric image's
  stored scalars (`pcurve_spiric_major`, `_minor`, `_offset`):
  `ChartWinding`'s escalation before `ImageMismatch`.
- `crates/geom-brep/src/pcurve_cache/projected.rs:2199` — the projected
  lane's period gates: `AzimuthPeriod` / `TubePeriod` escalation before
  `*PeriodExceeded`.

Not this shape: the ssi and props loops over samples, roots and
clusters, where each element is its own requirement; and
`props/curved.rs:3801`, which folds an in-band reading by design.

Territory: `intersect.rs` is GERM's, ROOTS' and TANG's; the
`pcurve_cache` sites are PCERT's (PCTAIL also claims `pcurve_cache.rs`).
Filed once, here, with that noted.

## Fix

Read each list through `geom_brep::ladder::read_identity`, as the spiric
and Villarceau ladders do. Each site's door rows need a pose with an
in-band gate ahead of a definite one; construct one where none exists,
and check first whether the site's own earlier gates make the order
unreachable, as the Villarceau incidence test does.
