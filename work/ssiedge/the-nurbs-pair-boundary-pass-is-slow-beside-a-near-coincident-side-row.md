---
id: the-nurbs-pair-boundary-pass-is-slow-beside-a-near-coincident-side-row
kind: issue
title: The NURBS x NURBS boundary pass walks a side row passing 1e-4 m from the other chart in ~10k face pieces (~20 s debug)
status: open
opened: 2026-10-10
priority: P4
cost: M
refs: [the-nurbs-pair-boundary-pass-reports-no-side-or-corner-region]
---

Measured on the NURBS × NURBS arm's boundary pass
(`crates/geom-brep/src/ssi/four.rs`, `face_roots`). For the twisted
loft's walls moved together OUTWARD by `100·ε` at `CAD_TOLERANCE_EPS=1e-6`
(`offd_r1_probes`, `moved_together_outward_every_edge_leaves_the_fits_windows`),
each seam's two side rows lie about 1e-4 m apart, and the pass over the
seam's pair cost about 20 s in a debug build: 8.5k–11k face pieces, of
which 6k–7k reached the Krawczyk test, nearly all on the two faces
holding the shared row (`u₁` high, `u₂` low), at pieces about 4e-3
parameter units wide. The mean-value exclusion along each chart's normal
cut the count by a quarter; its slack there was still ~2e-4 m against a
~1e-4 m gap. That row now pays nothing, because the pass runs lazily and
those seams refuse at their seed first, but an inward move of the same
size, whose section is derived 1e-4 m inside both rows, pays it.

Owed: a face walk whose cost does not scale with the row's length over
its gap: e.g. a second-order (Taylor) exclusion along the normal, or
excluding a face by the other chart's distance to the side curve as one
composite (the limb-2 tensor composition, applied to a boundary curve).
