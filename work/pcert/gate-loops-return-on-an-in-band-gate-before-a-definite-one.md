---
id: gate-loops-return-on-an-in-band-gate-before-a-definite-one
kind: issue
title: "Eight geom-brep gate loops, and run_spiric_checks' sequential stages, return on an in-band gate before a later gate's definite refusal speaks (identity, positivity and headroom polarities)"
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## What

Found by the sweep of the PCERT unit that closed
`an-in-band-gate-masks-a-definite-one-in-the-spiric-and-villarceau-ladders`
(2026-10-14). The polarities were corrected by PR 5089's second review.

Each loop below decides a list of gates through
`decide(...).map_err(...)?`, returning on the first in-band reading.
A later gate's definite refusal therefore never speaks, and the op
refuses with the wrong ending: a tolerance offer, or a defect, where a
datum it has not read yet already decides the refusal and names its
lever. Line numbers are as of the merge of main into PR 5089.

**Identity-shaped (every gate required Zero; any definite gate
refuses).** `geom_brep::ladder::read_identity` fits these as it is:
- `crates/geom-brep/src/pcurve_cache.rs:3745`: the winding lane's
  trigonometric coefficients (`affine_name`). `ChartWinding`'s
  escalation comes before `ChartWindingUnsupported`. (Since B2a the
  windings are structural and this loop is gone; the site is kept for
  the record.)
- `crates/geom-brep/src/pcurve_cache.rs:6531`: the spiric image's
  stored scalars (`pcurve_spiric_major`, `_minor`, `_offset`).
  `ImageIdentity`'s escalation comes before `ImageMismatch`. (E1 split
  check 4's copy readings out of `ChartWinding` as `ImageIdentity`;
  the shape is unchanged.)

**Positivity-shaped (every gate required Positive; Zero or Negative
refuses):**
- `crates/geom-brep/src/intersect.rs:2058`: cylinder × sphere
  degeneracy guard (`cs_cylinder_radius`, `cs_sphere_radius`).
  `RadiusEscalated` comes before `DegenerateOperand`.
- `crates/geom-brep/src/intersect.rs:2290`: plane × cone
  (`pn_aperture_sin`, `pn_aperture_cos`, `pn_seam_off_axis`).
  `Escalated` comes before `DegenerateOperand`.
- `crates/geom-brep/src/intersect.rs:3096`: cone × circle
  (`coc_aperture_sin`, `_cos`, `coc_seam_off_axis`): the same shape.

**Headroom-shaped (every gate passes on Positive or Zero; only Negative
refuses):**
- `crates/geom-brep/src/pcurve_cache.rs:3982`: the azimuth-period gates
  (`pcurve_azimuth_period`, azimuth then tube). `AzimuthPeriod`'s
  escalation comes before `AzimuthPeriodExceeded`.
- `crates/geom-brep/src/pcurve_cache.rs:5132`: the azimuth- and
  tube-period gates of the focal-section and spiric lanes: the same.
- `crates/geom-brep/src/pcurve_cache/projected.rs:2199`: the projected
  lane's period gates. The `AzimuthPeriod` / `TubePeriod` escalation
  comes before `*PeriodExceeded`.

**Sequential stages a loop sweep cannot see.** `run_spiric_checks`
decides three stages one after another, each returning on its first
in-band reading:
- the image's stored scalars (:6531);
- the wall's `pcurve_spiric_sense` (:6559);
- the chart ladder (:6621, which reads `read_identity` within itself).

All three escalate as `ImageIdentity` since E1.

An in-band `_sense` therefore still masks a definite chart gate, as an
in-band image scalar masks both. Any function that decides several
refusing gates in straight-line code has the same shape.

Not this shape:
- the ssi and props loops over samples, roots and clusters, where each
  element is its own requirement;
- `props/curved.rs`'s pole fold, which folds an in-band reading by
  design.

Territory: `intersect.rs` is GERM's, ROOTS' and TANG's; the
`pcurve_cache` sites are PCERT's (PCTAIL also claims `pcurve_cache.rs`).
Filed once, here, with that noted.

## Fix

`read_identity` reads "every gate Zero". Six of the eight sites, and the
stages, pass on another sign. The fix needs either:
- a polarity-aware ladder, where the reader returns pass, refuse or
  undecided per datum through a pass predicate (`Zero`, `Positive`, or
  `Positive | Zero`); or
- a second reader beside `read_identity` for the "every gate passes"
  shape.

Either keeps the rule and the recording (held runs, spliced superseded
where a later datum refuses). For the sequential stages the stages'
gates have to be read as one ladder, or each stage's undecided reading
held until the later stages have read.

Each site's door rows need a pose with an in-band gate ahead of a
definite refusal. Construct one where none exists, and check first
whether the site's own earlier gates make that order unreachable, as
the Villarceau incidence test does on most poses.
