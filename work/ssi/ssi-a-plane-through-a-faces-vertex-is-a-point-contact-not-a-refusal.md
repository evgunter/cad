---
id: ssi-a-plane-through-a-faces-vertex-is-a-point-contact-not-a-refusal
kind: issue
title: ssi: a plane through a face's corner vertex refuses TraceUnresolved instead of reporting a point contact, and a branch shorter than step/2^32 is lost depending on the extent
status: closed
opened: 2026-10-02
priority: P2
cost: H
pr: 3862
branch: ssi/ev-boundary-contact
closed: 2026-10-02
---


(SSI implementer `ssi-short`, from the review of PR 3730, 2026-10-01.)

## What

`ssi/march.rs`'s `march_both` refuses `SsiError::TraceUnresolved` when
the first trace has no length: the seed alone, because both
`push_boundary` searches dropped their ends. Two different geometries
reach it, and the march cannot tell them apart.

- **A point contact.** The flat 1 m bilinear wall cut by `x + z = d`
  with `d = 0` (the plane through the face's corner vertex) meets it at
  one point. That is a decided geometric fact, a point contact, and the
  operation refuses it instead of reporting it. Pinned by
  `m5_pr7_ssi.rs`'s
  `a_plane_through_a_walls_corner_vertex_refuses_as_the_marchs_limit`.
- **A branch below the boundary search's resolution.** `push_boundary`
  bisects the step `SSI_BOUNDARY_BISECTIONS = 32` times, so it resolves
  about `step/2^32`, and the first step is `SSI_STEP_MAX·extent`. Below
  that, a real branch is lost, and whether it is lost depends on the
  caller's extent.

Measured on PR 3730's head (same wall and plane, branch `d·√2`):

| d | ε | extent 1 m | extent 1.5 m | extent 100 m |
|---|---|---|---|---|
| 0, 1e-16 | 1e-6, 1e-9, 1e-12 | `TraceUnresolved` | `TraceUnresolved` | `TraceUnresolved` |
| 1e-11 | 1e-6, 1e-9, 1e-12 | `TraceUnresolved` | `TraceUnresolved` | `TraceUnresolved` |
| 1e-9 (1.4 nm) | 1e-12 | certifies | certifies | `TraceUnresolved` (3.125 m step) |
| 1e-9 | 1e-6, 1e-9 | `StepCollapsed` | `StepCollapsed` | `TraceUnresolved` |
| 1e-7 | 1e-12 | certifies | certifies | certifies |

At ε 1e-12 the 1.4 nm branch is 1400ε long and certifies at a 1 m
extent, but it is lost at 100 m.

## Open

- A point contact wants a decision of its own (C7's tangent-contact
  family is the nearest), so the operation reports the contact rather
  than refusing it.
- The boundary search's resolution is relative to the step, not to
  the tolerance. Bisecting until the bracket is below the band, rather
  than a fixed 32 times, would make the loss independent of the extent.
  That would be a D9 count chosen by a rule rather than a constant.

## The edge-flush case (from PR 3730's delta review)

A plane flush with a face's boundary edge reaches the same refusal.
The cause is that `march` ends a trace at its first state whose
`ssi_branch_open_end` margin is not decided positive. That margin is a
per-coordinate distance to the domain box, not a distance along the
branch, so a branch lying in the band of a box face never gets a state
decided inside.

Fixture: the flat 1 m wall cut by the plane `z = 0` (its bottom edge),
or by `x = off` (its `u = 0` edge) for `off ∈ {0, 0.3ε, 0.9ε}`.

- At ε 1e-6, 1e-9 and 1e-12, the march and the short-branch re-march
  both return 3 states.
- PR 3730 refuses them as `TraceUnresolved`. Before that PR's fix they
  reached `Fit(TooFewPoints{3,4})` with the kernel-defect ending.
- Pinned by `m5_pr7_ssi.rs`'s
  `a_plane_flush_with_a_walls_edge_refuses_as_the_marchs_limit`.

A plane flush with a face's edge is ordinary in a boolean, so this case
matters as much as the vertex touch.

**Open:** the branch here is the face's own edge, and the operation
should yield it, by recognizing the edge or by tracing along the
boundary with the open-end decision read along the branch, rather than
refuse.

## Design (designer pair, converged in 3 rounds, 2026-10-02)

The binding text is C3 in `crates/geom-brep/README.md`, as this row's
`[ev]` PR edits it. The build covers:

- **`geom_brep::boundary_section`.** Plane × one boundary curve of a
  NURBS wall. It answers either `On` (the side lies within the band of
  the plane), or certified roots with their slope margins.
  - Root isolation runs to the sweep floor minted per side
    (`SweepFloor`), and `SSI_BOUNDARY_BISECTIONS` retires on this lane
    (the ℝ³ lane keeps its slab search as `SSI_SLAB_BISECTIONS`; see
    Closed).
  - An unclamped wall needs a side extraction first.
  - The boolean's NURBS crossing layer reuses this door once it is
    wired.
- **SSI's boundary pass.** It runs before seeding in `plane_nurbs_ssi`.
  - A corner within the band is classified by the plane distance's two
    inward partials over a corner cell (`NurbsBoxes::deriv_box`), walking
    the tube ladder. The result is a branch start, a `Corner` region,
    nothing, no region (its roots are ordinary crossings), or a graze.
  - A side within the band is a `Side` region where the wall's slope
    across it is one-signed over a strip beside it, nothing where the
    strip is clear of the plane, and otherwise decided by its roots.
  - In band, the pass reports `Corner { corner, reach }` or
    `Side { side, reach }`. At a corner `reach` is
    |φ| · max(s_u / inf|φ_u|, s_v / inf|φ_v|) over the cell, the speeds
    the cell's; along a side it is sup|φ| · s⊥ / inf|φ⊥|, read piecewise
    along the side; both plus ε.
  - A region is reported only where its cover lies inside its cell and
    its reach is at most `SSI_REGION_REACH_MAX · Kε`; beyond it the
    roots decide. A root in a reported region's cell is the region's.
  - The exact empty answer stands outside the domain.
  - The pass refuses a graze (a double root along a side), naming the
    side and the move-the-geometry lever; a side whose slope across it
    does not clear the band (`BoundaryTangent`) and a side within the
    band with no bounded region and no root (`RegionUnbounded`), both
    toward C7; and a root that will not settle (`EndNotOnLocus`).
- **Output and accounting.**
  - `SsiOutcome.boundary: Vec<SsiBoundaryContact>`. Its type doc and
    Display say "region", not "contact".
  - The exhaustiveness sweep banks contact regions beside tubes, and the
    receipt counts them apart.
- **Branches.**
  - Open branches name their two crossings, and
    `BranchEnd::BoundaryInBand` goes from this lane.
  - A trace runs from one crossing to the unused crossing on the side
    it leaves nearest where its last step meets that side, with its step
    capped at a fifth of the distance to the nearest unused crossing. A
    march that finds no match refuses `CrossingUnmatched` as the march's
    limit, the last resort.
  - `push_boundary`, `ssi_branch_open_end`, the short-branch re-march
    and `TraceUnresolved` retire from this lane; the ℝ³ lane keeps
    them behind its own exit type (Closed).
  - Each known end is settled onto both surfaces, or refuses
    `EndNotOnLocus`.
- **Short clips.** Below `SSI_SHORT_CLIP · Kε`, the candidate is the
  Hermite cubic through the two certified ends and their tangents.
  - C2's three limbs decide it.
  - A failure is a sized refusal in |AB|.
  - Pin it with a test row over √2·Kε ≤ |AB| < 5Kε at ε 1e-9 and 1e-12.
- **Test rows.** The two `TraceUnresolved` rows in `m5_pr7_ssi.rs` flip
  to `Ok` with contacts, and the open-end escalation row retires.

**Precondition.** A plane does not yet certify against a curved wall: no fixture certifies above about 1/m of curvature. The causes are filed as
`plane-nurbs-ssi-does-not-certify-a-curved-dome`: a seed refined off the chart, the fit budget, the step
rungs' mixed units, and a HullSup underestimate. That row must land before any curved-wall row of this build.

**For the future join.** SSI's corner margin and the boolean's vertex
margin can differ by up to ε. So the join must accept a `Corner` region
whose reach contains both its crossing vertices, and take the Hermite
candidate from those vertices.

## Closed (2026-10-02, PR 3862; rules revised by Ev's rulings of 2026-10-03)

Built as designed, on the branch that carried the ruling, with two rules
changed after review:

- **A region iff coincident.** A `Corner` or `Side` region is reported
  exactly where the boundary pass certifies the locus within ε of the
  corner or side (its cover inside its cell, and its certified distance
  at most ε; `reach` is that distance plus ε). The region cap
  (`SSI_REGION_REACH_MAX`) is gone. A side whose strip holds the locus
  but not within ε (`SideClass::Apart`) is no region: it keeps its
  interior roots, each decided along it, and an arc there is traced.
  A corner both of whose sides are apart is classified as a corner on
  no decided side, and their roots there are its; a branch's start
  corner is one crossing by the corner's identity, not its cell, since
  on a wall narrower than the widest rung every cell is the whole
  domain.
  On rational walls a plane between ε and `Kε` off a side then traces
  the metre-long branch, which the certificate refuses as it does
  mid-wall (Ev, option (i)); the remedy is filed as
  `ssi-a-near-side-locus-could-take-the-walls-own-iso-curve` (P2).
- **March first, then the Hermite.** Every branch between known ends is
  marched; where the march cannot progress, its step in the band, the
  candidate is the Hermite cubic through the two ends (`ssi/ends.rs`).
  `SSI_SHORT_CLIP` is gone; the sized refusal reads the step as the
  length over `SHORT_BRANCH_STEPS`.
- `RegionUnbounded` is gone: the one geometry found to reach it (a wall
  turning back short of a plane within the band of its side, no side
  crossed) is a certified empty answer the exhaustiveness sweep gives
  without it.

The rest as built:

- `geom_brep::boundary_section` (`ssi/section.rs`): plane × one NURBS
  curve, Bernstein pieces in interval arithmetic, roots isolated by the
  sweep's own recursion to the section floor minted per side
  (`SweepFloor::section`). `SSI_BOUNDARY_BISECTIONS` is gone from this
  lane; every `KnotVector` is clamped by construction, so the side
  extraction is `nurbs_iso`'s row copy.
- The boundary pass (`ssi/boundary.rs`) runs before seeding in
  `plane_nurbs_ssi`; the branches between known ends and the Hermite
  candidate are `ssi/ends.rs`.
- `SsiOutcome.boundary`, `Exhaustiveness.contact`, and
  `BranchEnd::Crossings` are the output; the ℝ³ lane keeps its slab
  search behind `SlabExit` (`ssi-r3-slab-is-not-geometry`).

The two `TraceUnresolved` rows report regions; the open-end escalation
row retired. Measured answers on the 1 m flat wall, every ε of 1e-6,
1e-9 and 1e-12 (`m5_pr7_ssi.rs`):

| geometry | answer |
|---|---|
| corner clip `x + z = d`, `d < 0` | `Ok`, empty |
| `0 ≤ d ≤ 0.9ε` | `Ok`, one `Corner` region, reach `d + ε` |
| `d > ε`, `|AB| = d·√2 < 5Kε` | one branch, the Hermite candidate, certified |
| `|AB| ≥ 5Kε` | one branch, marched between its crossings |
| edge plane `x = off`, `off < 0` | `Ok`, empty |
| `0 ≤ off ≤ 0.9ε` | `Ok`, one `Side` region, reach `off + ε` |
| `off ≥ 3ε` | one branch, 1 m |
| semicircle of radius `2Kε` (half cylinder cut by `z = ½`) | `ShortBranchUncertified`, limb 1 |

Residue filed: `ssi-r3-slab-is-not-geometry`,
`ssi-a-near-side-locus-could-take-the-walls-own-iso-curve`. The
final-chord and step-scale rows carry this lane's evidence and stay open
for the ℝ³ lane.
