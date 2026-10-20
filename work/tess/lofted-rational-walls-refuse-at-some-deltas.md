---
id: lofted-rational-walls-refuse-at-some-deltas
kind: issue
title: lofted rational walls refuse CertificateExceeded at some chordal tolerances, at sliver triangles in a dense row strip
status: open
opened: 2026-10-20
---


## Finding

A straight loft of round sections meshes at most chordal tolerances δ
and refuses typed at some, `CertificateExceeded` with a finite bound a
few times δ. Measured on a geometric sweep of 60 δ from 3e-2 down by
×0.93 (down to about 4e-4):

| body | refuses at δ |
|---|---|
| `sweep::test_support::lofted_cylinder(1.0)`: two semicircles, a crease per wall | 8.6e-4 to 1.6e-3 (10 steps), and 4.1e-4 to 4.5e-4 |
| the same circle as four quarter arcs (bulge `tan(π/8)`), no crease | 4.1e-4 to 4.5e-4 |

The four-arc loft meshes on `main`, so the class predates the crease
lines (`crates/mesh/src/trimmed.rs`, `Creases`); the creased walls
only reach it over a wider band.

**Where.** On the creased wall at δ = 1.2e-3, every triangle above δ
on the last refinement round is away from the crease (`u = ½`), at
`u ≈ 0.12–0.16` and `0.84–0.88`. Each fans from a refinement centroid
to the grid row on either side, the rows `1/16` apart in `v` and the
columns about `1/150` apart in `u`. On the first round the offenders
are fans from the seam's chord points (`u = 0, 1`, `v = 1/3, 2/3`),
which sit between two grid rows; the seam's chord count comes from the
whole-patch steps, the rows from the aspect-capped band schedule. This
is the anisotropic-strip sliver the module docs of
`crates/mesh/src/trimmed.rs` describe (a centroid re-admits the empty
circle beside itself), and the refinement ladder's six rounds run out.

## Fix

Not designed. Two directions: align the seam chord schedule with the
band rows it meets (the seam is an `IsoLine` whose moving channel is
`v`), or refine an offending fan by splitting its long edge at the
grid row rather than inserting its centroid.
