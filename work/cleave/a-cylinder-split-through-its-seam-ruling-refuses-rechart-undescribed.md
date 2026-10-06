---
id: a-cylinder-split-through-its-seam-ruling-refuses-rechart-undescribed
kind: issue
title: A solid cylinder split by a plane through its seam ruling (0.4 rad off tangency) refuses Finish(Euler(RechartUndescribed{seam})) with both normals at every eps; off the seam the same pose builds
status: dispatched
opened: 2026-10-06
priority: P0
cost: M
branch: cleave/seam-ruling-split
---


## What

Found by the delta review of PR 4098 (DR-4098b, a pre-existing NOTE). A solid cylinder split by a
plane that contains its seam ruling, tilted 0.4 rad off tangency to the wall, refuses
`Finish(Euler(RechartUndescribed{seam}))`. This happens:
- with both normals;
- at ε 1e-6, 1e-9 and 1e-12;
- on main (`cadf2ed188`), on PR 4098's head, and merged with PR 4120.

The same pose through any ruling that is not the seam builds. A user does not know where a seam
is, so this is an ordinary split refusing on ordinary geometry: P0.

The refusing door is `splitting/finish.rs`'s re-chart of the faces a null pair divides (the S7
site of PR 4098's review). The reviewer's probes are in
`~/.local/share/cad-work/cleave-review-4098b/scratch/rvb_probes.rs` (session-local; rebuild the
pose from this paragraph if they are gone).

## Owed

- Measure first: which edge is the `seam` the re-chart cannot describe, and why the seam ruling is
  special.
- Then fix it where it starts, so the pose builds both ways at the closed-form volumes, with valid
  halves. Do not special-case the seam.
- Sweep cones, and full-revolve walls, for the same plane through a seam.

## Built (branch cleave/seam-ruling-split)

**Measured first** (instrumented `set_face_surfaces_describing`'s refusal and the promotion in
`splitting/finish.rs`, solid cylinder, a = 0, t = 0.4, both normals):

- The refused edge `2v1` is the operand's own seam: the line (1, 1, 0)→(1, 0, 0), described
  `Chart { surface: cylinder, seam: true }`. Off the seam, both rulings of the section are fresh
  edges with no description; through the seam, the plane runs along an existing edge, which the
  section boundary reuses.
- It bounds a null-pair face that inherited the cylinder's chart and a wall fragment that wears
  that chart too. `section_plane_restatements` skipped every edge whose other face wears the moving
  face's chart — a proxy for "the other face is the null mate" — so the seam edge was not
  restated. Once one side moved onto the section plane, its seam claim (the chart on both sides,
  `Named::adjacent_to`) was stranded: `RechartUndescribed`.
- Whether it bites depends on which chart the null pair inherited: where it inherited a cap's
  plane, the seam edge was already incoherent before the move and the door skipped it. That is
  why some seam poses answered on main (a frustum under one normal, a tube's bore off the axis).

**Fix**: `section_plane_restatements` restates an edge naming the face's chart when its
description would not survive the face leaving that chart (`Named::survives_one_side_leaving`,
the door's own adjacency rule), not when the other face wears another chart. The seam edge is
restated as a plain image on the chart, which the door reads as the wall that keeps it; the
boundary pass then describes it as the plane × wall intersection.

**Pinned** in `crates/sweep/tests/split_through_a_seam_ruling.rs`: solid cylinder, tube, and
counterbore outer walls, and a frustum, each through its seam ruling at t ∈ {0.05, 0.4, 1, π/2,
2, 3, −0.4, −1.2}, both normals. They answer at the closed-form volumes, and both halves pass
tiers 1, 2, 3 and 3′. Every family had poses red on main; t = π/2 holds the axis, so it also
runs along the bores' seams. A near-tangent row (t ∈ {1e-5, 1e-4, 1e-3}, on and off the seam)
asserts that whatever answers, answers right. Green at ε 1e-6, 1e-9 and 1e-12.

**Sweep** (606 poses per ε, seam azimuth against azimuths 0.3 and 2, at ε 1e-6, 1e-9, 1e-12):
main → fix moved 62 (1e-6), 68 (1e-9) and 80 (1e-12) poses from `RechartUndescribed` to the closed form, with no
regressions. After the fix, every seam pose answers wherever the off-seam twin does. In the
band (t ≤ 1e-3) both refuse, with different refusal kinds. Where the twin refuses
`DegenerateSection` (frustum, t ∈ {0.05, 3}), the seam pose answers; that is filed as its own
row.
