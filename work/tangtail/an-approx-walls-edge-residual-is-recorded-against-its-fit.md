---
id: an-approx-walls-edge-residual-is-recorded-against-its-fit
kind: issue
title: An edge on an Approx wall records its residual against the fit, and nothing adds the fit's own distance from the description
status: open
opened: 2026-10-15
priority: P3
cost: M
---

Found by PR 5194's review (m3).

## What happens

Both the plane × NURBS lanes meter an `Approx` wall through `spline_chart()`, which is its FIT:
- the `Intersection` lane (`geom_brep::plane_nurbs_limbs`, `Resolved::PlaneNurbs`);
- the `OnPair` lane (`geom_brep::plane_nurbs_incidence`, `Resolved::OnPairNurbs`), both in `crates/geom-brep/src/certify.rs`.

The edge's certificate records the lane's bound as its `max_residual`. D4 ¶2 reads that number as the edge's distance from its intensional description, but here it is the distance from the fit. The distance from the description is that bound PLUS the fit's own certified distance from the description, the `hull_sup` of the surface's `OffsetCertificate` (`geom::ApproxSurface`). Nothing adds the two.

Pinned as a limit by `crates/geom-brep/tests/onpair_plane_nurbs.rs::an_approx_wall_is_metered_against_its_fit`. A coarse fit refuses a line on the true offset, with a miss the description does not have.

## What closing it needs

When the wall is an `Approx`, the lane's recorded residual becomes `bound + fit.hull_sup`, banded as one sum, in both lanes. The fit's `hull_sup` is a stored record that tier 3 re-derives in `f64`, so it must be read where tier 3 reads it, not trusted from the store. Some fitted edges that certify today against the fit would then refuse; that needs its own fence.
