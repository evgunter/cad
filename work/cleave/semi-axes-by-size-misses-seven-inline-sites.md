---
id: semi-axes-by-size-misses-seven-inline-sites
kind: issue
title: Seven inline readings of the larger semi-axis bypass Curve3::semi_axes_by_size, and one production bind still says major
status: open
opened: 2026-10-14
priority: P3
cost: E
---

Found by the narrow re-review of PR 4861 (`cleave/section-ellipse-re-pose`, verdict MERGE). A half-fix of the class that PR consolidated: its sweep matched `semi_*` names, so locals named `a`/`b` escaped it.

## What

These do not route through `Curve3::semi_axes_by_size`:
- `topo/src/ring_path.rs` ~669 (`c.a.abs().min(c.b.abs())`);
- `topo/src/ring_path.rs` ~1469 (signed `conic.a.min/max(conic.b)`);
- `splitting/containment.rs` ~896 and ~1062;
- `geom-brep/src/extent.rs` ~308 (`conic_arc_reach`);
- the half-abs `semi_u.abs().min(semi_v)` at `certify.rs` ~2717 and at `props/curved.rs` ~2212.

The two half-abs sites are field-asymmetric by design, because certification admits a negative `semi_u`, but nothing says so. One production bind still claims "major": `geom-brep/src/extent.rs` ~455, `semi_u: major, semi_v: minor`, which is used positionally.

None of these changes behaviour for positive semi-axes.

## Fix

- Route each site through `semi_axes_by_size`, or state at the site why its reading is ordered.
- Rename the bind.
- Re-sweep, matching by data shape (`.min(`/`.max(` on two semi-axis values) rather than by name.
