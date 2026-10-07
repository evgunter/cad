---
id: a-loft-caps-plane-is-summed-in-the-authored-vertex-order
kind: issue
title: A loft cap's Newell plane is summed from the authored start vertex, so re-spelling a section moves the cap's bits
status: open
opened: 2026-10-06
priority: P2
---


## Finding

`assemble` (`crates/sweep/src/loft.rs`) fits each cap plane with
`geom_brep::newell_plane(&far_loop, band)` over the cap's ring in the
canonical loop's vertex order. The canonical form keeps the AUTHORED
start vertex, and `newell_plane` (`crates/geom-brep/src/newell.rs`) sums
both the centroid and the cross-product normal in that order. So a
section written from another corner (or rolled by one of its own
symmetries) hands the same point set to the fit in a rotated order, and
the plane's `origin` and `normal` bits can move by an ulp.

That is the same class as
`loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body`
at bit level: since that unit, the walls and the section parameters are
a function of the section set and are pinned bit-identical across
spellings (`loft_v_is_the_section_set`), but the caps are not pinned
and, by this reading, are not order-free.

**Confidence**: sure. Measured by the review of PR 4193 on an
irregular, tilted three-station loft: across start vertices, the cap
`Plane`'s `origin`, `normal` and `u_ref` differ by ulps, while the
walls, the section parameters and the volume are bit-identical.

PR 4187 rewrites this cap fit (`swept::cap_plane`); the fix belongs on
whichever of the two lands second.

## The fix it points at

Sum in an order fixed by the point set rather than the spelling (for
example, start the ring at its lexicographically least vertex before the
fit), or make `newell_plane`'s sums order-free. Then extend
`loft_v_is_the_section_set`'s bit-identity to the cap surfaces.
