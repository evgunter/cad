---
id: ssi-a-plane-through-a-faces-vertex-is-a-point-contact-not-a-refusal
kind: issue
title: ssi: a plane through a face's corner vertex refuses TraceUnresolved instead of reporting a point contact, and a branch shorter than step/2^32 is lost depending on the extent
status: open
opened: 2026-10-02
priority: P2
cost: H
design: true
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
  NURBS wall. It answers either `On` (the side lies in the plane), or
  certified roots with their slope margins.
  - Root isolation runs to the sweep floor minted per side
    (`SweepFloor`), and `SSI_BOUNDARY_BISECTIONS` retires.
  - An unclamped wall needs a side extraction first.
  - The boolean's NURBS crossing layer reuses this door once it is
    wired.
- **SSI's boundary pass.** It runs before seeding in `plane_nurbs_ssi`.
  - A corner on the plane is classified by the plane distance's two
    inward partials over a corner cell (`NurbsBoxes::deriv_box`), walking
    the tube ladder. The result is a branch start, a `Corner`, or a graze.
  - In band, the pass reports `Corner { corner, reach }` or
    `Side { side, reach }`. Here `reach` is |φ| / inf |∂φ| over the cell,
    plus ε.
  - The exact empty answer stands outside the domain.
  - The only refusal the pass keeps is a graze (a double root along a
    side). Its ending names the side and the move-the-geometry lever.
- **Output and accounting.**
  - `SsiOutcome.boundary: Vec<SsiBoundaryContact>`. Its type doc and
    Display say "region", not "contact".
  - The exhaustiveness sweep banks contact regions beside tubes, and the
    receipt counts them apart.
- **Branches.**
  - Open branches name their two crossings, and
    `BranchEnd::BoundaryInBand` goes.
  - A trace runs from one crossing to the crossing on the side it
    leaves, with its step capped at |AB|/5. A match that is missing or
    doubled is a defect.
  - `push_boundary`, `ssi_branch_open_end`, the short-branch re-march
    and `TraceUnresolved` retire.
  - Each known end is settled onto both surfaces, or refuses
    `EndNotOnLocus`.
- **Short clips.** Below a fixed multiple of Kε, the candidate is the
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
