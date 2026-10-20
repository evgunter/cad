---
id: lofted-rational-walls-refuse-at-some-deltas
kind: issue
title: lofted rational walls refuse CertificateExceeded at some chordal tolerances, at sliver triangles in a dense row strip
status: open
opened: 2026-10-20
priority: P0
cost: M
design: true
refs: [lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late, 5499]
---


## Finding

A straight loft of round sections meshes at some chordal tolerances δ
and refuses typed at others, `CertificateExceeded` with a finite bound a
few times δ. A creased frustum refuses at 9 of the 10 δ ≤ 3.4e-3,
so a lofted circle, the shape
`lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`
set out to mesh, still does not mesh at the tolerances a fine export
asks for. Hence P0, with the item it continues.

**#5499's review** (20 δ per body, log-spaced from 5e-2 to 3e-4 and
scaled by body size, with the falsifier armed):

| body | refuses at |
|---|---|
| frustum, radii 1 → 0.5, two semicircles (a crease per wall) | 9 of the 10 δ ≤ 3.4e-3 |
| the same frustum as four quarter arcs, no crease | 4 of 20 δ |
| the same frustum as eight arcs, no crease | none |
| `sweep::test_support::lofted_cylinder(1.0)`: two semicircles, a crease per wall | 5 of 20 δ |
| the same cylinder as four quarter arcs, no crease | 2 of 20 δ |

**The PR's own sweep** (60 δ from 3e-2 down by ×0.93, to about 4e-4):

| body | refuses at δ |
|---|---|
| `lofted_cylinder(1.0)` | 8.6e-4 to 1.6e-3 (10 steps), and 4.1e-4 to 4.5e-4 |
| the same circle as four quarter arcs (bulge `tan(π/8)`), no crease | 4.1e-4 to 4.5e-4 |

Every refusal is typed. No refusal carries `bound: inf`, every mesh
that does build passes `check_mesh`, and the falsifier reads at most
0.4999 of each certificate. So this is a liveness defect, not a
soundness one.

The crease-free four-arc loft also refuses at some δ on `main`, so the class predates
the crease lines (`crates/mesh/src/trimmed.rs`, `Creases`). The crease
makes it far more frequent: two arcs per circle refuse more often than
four, and eight arcs never refuse. That points at the span per patch,
not at the crease line itself: no offender sits near it (below).

**Where.** On the creased cylinder wall at δ = 1.2e-3, every triangle
above δ on the last refinement round is away from the crease (`u = ½`),
at `u ≈ 0.12–0.16` and `0.84–0.88`. On the frustum at δ = 1.5e-3, the
review found all 6436 last-round offenders at `u ≈ 0.12–0.16` and
`0.84–0.87`, none within 0.02 of the crease. Each fans from a refinement centroid
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
