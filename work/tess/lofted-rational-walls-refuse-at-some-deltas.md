---
id: lofted-rational-walls-refuse-at-some-deltas
kind: issue
title: lofted rational walls refuse CertificateExceeded at some chordal tolerances, at sliver triangles in a dense row strip
status: closed
opened: 2026-10-20
priority: P0
cost: M
design: true
refs: [lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late, 5499]
closed: 2026-10-21
pr: 5531
branch: tess/lattice-metric-cdt
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

## The mechanism, measured

Traced on `main` at `ce0b61cc`, on the creased frustum (radii 1 → 0.5,
height 1, two semicircles) with a local probe of
`tessellate_trimmed`'s retry loop, not pushed: each round's offenders,
their `u` histogram and the worst `bound/δ`. Both walls trace
identically.

**The lattice.** A lofted wall is ruled (`mvv = 0`, `v` linear), so
`NurbsFaceBound::split_steps` takes the cap arm: the aspect window's
end, `ASPECT_CAP` = 16 on the speed ratio, as long in `v` as it
allows. The cell grid cuts `v` into 16 bands, and at δ = 1.5e-3 every
band is one row tall (`nvc = 1`, `cap = true`; two rows at 3e-4). So the rows sit at
`k/16` and the columns are dense: at δ = 1.5e-3 a band has 360 to 494
columns, `s_u ≈ 0.002–0.0028` against `s_v = 0.0625`, a parameter
aspect of about 0.03.

**The seed.** The seam is an iso line at `u = 0` and `u = 1`, carrying
its own chord schedule, sized from the whole-patch steps. At δ = 3.4e-3
it has a chord point at `v = 2/7`, a boundary point no row passes
through. A seam point between two rows fans across the
strip: at δ = 3.4e-3 the one offender per side is

    (0, 2/7) – (0.0298, 0.3125) – (0.0319, 0.25)      bound/δ 1.086

about 14 columns wide in `u` and a full row tall in `v`. Delaunay chose
it: in a strip about 30 times denser in `u` than in `v`, the empty
circle of a point between rows reaches about half a row height in `u`.
The `u` curvature across that width is what exceeds δ.

**The ladder walks the sliver, it does not remove it.** Centroid
refinement inserts the offender's centroid, another off-row point in the
same strip, and Delaunay forms the same fan one step further in.
δ = 3.4e-3, one offender per side, every round:

| round | offender's centroid `u` | bound/δ |
|---|---|---|
| 0 | 0.021 | 1.086 |
| 1 | 0.042 | 1.063 |
| 2 | 0.063 | 1.110 |
| 3 | 0.084 | 1.140 |
| 4 | 0.105 | 1.153 |
| 5 | 0.126 | 1.185 |
| 6 | 0.146 | 1.158 |

`MAX_GRID_RETRIES` = 6 rounds run out with the front at `u ≈ 0.15`,
which is where both the PR's and the review's last-round offenders sat
(`u ≈ 0.12–0.16`). At finer δ the seam has more chord points, and each
seeds a front:

| δ | round 0 offenders, worst bound/δ | round 6 offenders, worst bound/δ |
|---|---|---|
| 3.4e-3 | 2, 1.09 | 2, 1.16 |
| 1.5e-3 | 182, 2.53 | 350, 2.88 |
| 8e-4 | 394, 4.35 | 538, 5.30 |
| 3e-4 (`nvc = 2`) | 476, 3.08 | 788, 3.33 |

Round 0's offenders all lie in `u < 0.1` or `u > 0.9`, the nearest at
`u ≈ 0.01`: the seam's fans. They grow in number and in `bound/δ` each round. More rounds would not
converge: the front moves about 0.02 in `u` per round, and nothing stops
it before the wall's far side.

**Why fewer arcs refuse more.** The same frustum at δ = 1.5e-3:

| arcs per circle | `s_u` (band 0) | parameter aspect `s_u/s_v` | round 0 worst bound/δ | outcome |
|---|---|---|---|---|
| 2 | 0.00202 | 0.032 | 2.53 | refuses |
| 4 | 0.00405 | 0.065 | 0.915 | meshes |
| 8 | 0.00862 | 0.14 | 0.411 | meshes |

Each wall spans `2π/arcs` in `u ∈ [0, 1]`. Fewer arcs mean more columns
per row in parameter space, a wider seam fan in columns, and more
curvature under it. No offender sits near the crease. The crease does make the two-arc
wall the shape `segment_curve` builds for every circle section, which
is why this class now meets every lofted circle.

## What a fix must hold

- **The certificate stays the guarantee.** No triangle is accepted
  above δ, and a face that cannot meet δ still refuses typed.
- **Watertightness.** The seam's chord points are shared with the face
  across the seam (the wall itself, at a full-turn seam, or the
  neighbouring wall). Moving them moves the neighbour's boundary too, so
  the chord pass decides them, not the lane.
- **The crease lines** of #5499 stay cut. A new row or point on a crease
  is noted and chained like any other.
- **The ruled-wall strip stays.** The anisotropic lattice is the
  ruled-wall optimum and the reason a lofted wall costs as few
  triangles as it does. A fix that makes the rows dense pays for it on
  every lofted face.
- **Goldens may move.** If one does, re-baseline it and say what moved.

## Directions (not designed; for the designer pair)

1. **Rows through the boundary.** Give each boundary point that lies off
   the rows, beside a dense strip, a row of its own: a row of
   candidates at its `v`, across the band, or across a few columns
   either side. This is #5499's crease rule applied to boundary points:
   every point beside the line has a partner on its row, so its fan is
   one column wide. The cost is one extra row per off-row seam point.
   The open question is how local the row may be before its own end
   seeds a fan.
2. **Seam schedule on the rows.** Align the seam's chord schedule with
   the band rows: make its count a multiple that lands on `k/16`, or
   take the rows from its points. This is the item's first direction.
   The chord pass sees the seam's two faces, so this changes the chord
   pass, not the lane, and it does not reach a trim curve that is not an
   iso line.
3. **Split the long edge.** Refine an offending fan by splitting its
   long edge where it crosses a column, rather than inserting its
   centroid. This is the item's second direction. It converges where
   the centroid walks, but it adds points per offender per round, and
   it is the lane's last resort rather than its schedule.
4. **More rounds.** Rejected by the trace: the front moves about 0.02
   per round and grows in count.

Questions for the pair:
- Does a row through every off-row boundary point (1) subsume (2), or
  should the seam schedule also be aligned so that the common case adds
  nothing?
- Is the fan width measured in columns (`s_v/(2·s_u)`) the quantity a
  rule should bound, so that it reads off the band schedule before any
  triangle is formed?
- Does the same seed occur on non-lofted ruled walls (an extruded
  spline, a ruled surface between two curves), and does the fix cover
  them?

## Witness plan

- **The refusal rows.** The frustum and the cylinder, two arcs per
  circle, at the review's 20 δ, mesh at every δ: watertight, under δ,
  falsifier armed. Today the frustum refuses at 9 of the 10 δ ≤ 3.4e-3.
- **The seed row.** At δ = 3.4e-3 on the frustum, round 0 has no
  offender. Today it has 2, at `(0, 2/7)`.
- **The cost row.** The triangle count of the four- and eight-arc
  controls, and of the corpus's lofted bodies, measured before and
  after. A fix that densifies the strip shows here.
- **The controls.** The four- and eight-arc frusta still mesh at every δ.
