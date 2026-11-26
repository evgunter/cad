---
id: a-straight-carrier-over-a-plane-reads-its-parting-radius-as-one-over-zero
kind: issue
title: A straight carrier over a plane reads its parting radius as 1/[0,0] at Interval
status: open
opened: 2026-11-24
priority: P3
---


`clusters` in `crates/topo/src/boolean/carrier_touch.rs` (reached from `off_face`) reads the parting radius over a plane as `T::one() / bend`, `bend` from `geom::carrier_speed::curvature_hi`, guarded only by `geom_core::is_finite_length(bend)`. On a `Curve3::Line` the bound is exactly zero, so at `Interval` the quotient is `1 / [0, 0]`, no enclosure, and `tangency_floor(..).hi()` reads `NaN`; at `f64` it is `+∞` and the floor is `+∞`. Either way `ell.hi() > floor` is never true, so the walk never halves a piece it has not cleared: each one is kept as a cluster at its first length. Whether that loses a cluster, or only costs nothing because a line over a plane never reaches this walk, is not read.

Found by SECTCONE's sweep for the shape `1 / κ` at a generic scalar, while fixing the same shape in `CarriedComponent::foot_along` (`section_site_foot_bend`, PR 7734, `work/sectcone/a-carried-sites-foot-escalates-where-its-polish-stalls-or-clamps-at-a-run-end.md`). There the bound is raised to `f64::MIN_POSITIVE` before its reciprocal, which keeps it a curvature bound and the radius finite. Here, a line's own floor is probably a different reading, since a line over a plane parts from it at a constant slope. So the fix is the owner's choice.
