---
id: the-tube-and-radius-guards-decide-on-the-band-where-check-1-reads-lo
kind: issue
title: The section arms decide a torus tube, a cylinder radius and a cone aperture on the tolerance band; tier-3 check 1 compares the same datums' lower bound with zero
status: open
opened: 2026-09-24
priority: P3
cost: D
refs: [ATREST-6]
---

## What

Two postures on one fact, in two crates. Found by ATREST-6's review
(PR 3183). Which posture is right is not ATREST-6's call.

- **At rest.** `topo::validate`'s tier-3 check 1
  (`analytic_surface_verdicts`, `crates/topo/src/validate.rs`) refuses
  a stored datum outside its convention as
  `ValidationError::UnrepresentableSurfaceDatum`. It reads each margin
  of `geom::Surface::representability_margins`
  (`crates/geom/src/surfaces.rs`) with `Bounds::lo(margin) > 0`. That
  is a statement about the DATUM: not metered, no band, no `k_stats`
  name.
- **At the section arms.** `geom_brep::intersect` decides the same
  datums as metered trileans on the tolerance band:
  - `plane_torus_section`'s `pt_tube_guard` decides `Margin::of(r)`,
    and `Zero | Negative` returns `SectionError::DegenerateTorus`;
  - `cone_cylinder_section`'s `coc_cylinder_radius` decides the
    cylinder radius the same way;
  - `coc_aperture_sin` / `coc_aperture_cos` decide the cone's
    `α ∈ (0, π/2)`.

**The example.** A torus with `r = 1e-15` (or a cylinder of radius
`1e-15`) has `lo > 0`, so it passes tier 3 at rest. Decided on the
band at `Tol::witness()`, the same margin is `Zero`, so the section arm
refuses it as a degenerate torus (or an escalation). A body that
validates can therefore be refused by the boolean for a reason the
validator does not share. Before ATREST-6 this held for the torus tube
only. The cylinder and cone halves arrived with check 1's new arm.

**The scalar half, same check.** Check 1's poison read asks
`geom_core::is_finite_length`, and that reads the value channel. At
`Interval`, an enclosure whose upper end overflowed still contains its
truth and answers finite. At `f64`, the same computation is `∞` and is
refused. A stored datum lifted by `Interval::from_f64` cannot hit this,
because NaN and `±∞` become NaI and are refused. A datum COMPUTED at
interval type can. This is stated at the check (`poisoned_datums`'s
docs), and it belongs to whichever answer this row reaches.

## What must be decided

One of three answers:

- Is a datum inside its convention a band question (the section arms'
  posture, and then tier 3 should meter it and name it)?
- Is it a datum-sign question (check 1's posture, and then the section
  arms' refusals are insurance that fires on bodies tier 3 passes)?
- Do the two answer different questions and both stand, with the gap
  documented at both sites?

## Fence

GERM (`crates/geom-brep/src/intersect.rs`); the check-1 half is
ATREST's (`crates/topo/src/validate.rs`), and a seam to announce.
