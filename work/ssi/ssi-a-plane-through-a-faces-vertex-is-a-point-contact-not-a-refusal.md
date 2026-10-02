---
id: ssi-a-plane-through-a-faces-vertex-is-a-point-contact-not-a-refusal
kind: issue
title: ssi: a plane through a face's corner vertex refuses TraceUnresolved instead of reporting a point contact, and a branch shorter than step/2^32 is lost depending on the extent
status: open
opened: 2026-10-02
priority: P2
cost: M
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
