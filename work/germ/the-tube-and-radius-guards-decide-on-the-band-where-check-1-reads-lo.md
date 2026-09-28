---
id: the-tube-and-radius-guards-decide-on-the-band-where-check-1-reads-lo
kind: issue
title: The section arms decide a torus tube, a cylinder radius and a cone aperture on the tolerance band; tier-3 check 1 compares the same datums' lower bound with zero
status: open
opened: 2026-09-24
priority: P3
cost: D
refs: [3183]
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

**Two more on the band side (2026-09-28, the C5 pose-gate unit,
PR 3322).** `plane_cone_section` now decides `pn_aperture_sin` /
`pn_aperture_cos` the way `cone_cylinder_section` decides
`coc_aperture_*`, and `intersect.rs`'s module docs state the file's
band posture on the cone's convention.

**The review's argument against a numerical reading of those guards,
recorded as evidence.** The PR first justified them as "a divisor
within ε of zero carries no relative accuracy". That is not so for a
STORED `α`: `sin α` of a stored datum is relatively exact (the same PR
says so of `ConeOffset`), and the two red-first outputs are close to
correct for the datums as stored — at `fl(π/2)` the axis-normal circle
of radius `≈ 3e16·h` is the true section of that (nearly flat) cone,
and at `α = 1e-300` the apex lane's tangent generator is, to within the
band, the axis itself. So the guards are a POSTURE (the band side of
this row's question), not a numerical necessity, and they refuse cones
that tier-3 check 1 passes. They stay pending this row's ruling; the
PR's rationale now says so. One consequence the review also found, and
the PR fixed: because the guards run ahead of the pose trileans,
`route_pose` must read their refusal as an UNclassified pose, not a
served one.

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
