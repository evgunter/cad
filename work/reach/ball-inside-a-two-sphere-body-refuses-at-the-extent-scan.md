---
id: ball-inside-a-two-sphere-body-refuses-at-the-extent-scan
kind: issue
title: A ball strictly inside a two-sphere body refuses FallbackExtentUnsupported because the full spheres cross
status: review
opened: 2026-10-01
refs: [sphere-union-sphere-refuses-though-the-section-is-closed-form]
branch: reach/extent-scan-faces
pr: 3801
---

Found by the delta review of PR 3659 (probe `zz_rv3659b.rs`, `reach-review3659b`).

## Measured (PR 3659 head `7b61fa03`)

All on the y axis, every ball a full revolve about `y` (one seam):

- `u1 = ball(1.0, y=0) ∪ ball(0.8, y=1.4)` (the snowman);
- `lens = ball(1.0, y=0) ∩ ball(0.8, y=1.4)`;
- `chain3 = u1 ∪ ball(0.6, y=2.3)`.

| operands | outcome |
|---|---|
| `u1 × ball(0.5, y=0.75)` | `FallbackExtentUnsupported` |
| `lens × ball(0.6, y=0.8)` | `FallbackExtentUnsupported` |
| `lens × ball(0.9, y=1.0)` | `FallbackExtentUnsupported` |
| `chain3 × ball(0.5, y=1.9)` | `FallbackExtentUnsupported` |

In each, the small ball lies strictly inside the two-sphere body, so no
edge of either operand crosses a face of the other, and the reduction
finds no crossing. The containment fallback then runs the sphere extent
scan (`boolean::ops`, the `bool_sphere_sphere_gap` /
`bool_sphere_sphere_nested` arm). That scan reads the SURFACES: the
small ball's full sphere crosses the full sphere of a face it never
meets (the trimmed-away part of the big ball inside the other one), so
"neither separated nor strictly nested" holds of the surfaces while the
FACES are disjoint, and the scan refuses.

## What a fix has to supply

A face-scoped extent verdict for a sphere pair: whether the section
circle of the two full spheres meets the trimmed FACE. The face's chart
trim (`solid_contain::sphere_chart_trim`, a latitude band × azimuth
window) can place the circle — it is a latitude circle of a coaxial
pair, and an exact circle in general — and a section circle wholly
outside the face's trim certifies the pair disjoint at this face.

## Fixed (branch `reach/extent-scan-faces`)

Re-measured on `origin/main` `d5d6fe8f`: every pose × op × order
refused `SpheresMeet` (the scan's decided `bool_sphere_sphere_nested`
refusal, split out of `FallbackExtentUnsupported` since filing), at
the same arm. The hypothesis held: the arm decides on the carriers.

`sphere_extent_scan` now asks, once the carriers cross, whether the
FACES meet: `sphere_faces_apart` runs the section certificate's own
per-pair rule (`pair_verdict`, the body of `section_pairs`) over every
face on the sphere against the partner face, and only a pair the rule
does not clear refuses. The class sweep moved two more arms onto the
same reading: the scan's cylinder arm (its pairs are the section
pass's now, as torus and cone pairs are) and the plane arm's
trimmed-group escape. Pinned by `snowman.rs`
`a_ball_inside_a_two_sphere_body_builds` and
`a_lens_beside_a_slab_its_trimmed_sphere_crosses_builds`, and
`verbs_cylsph_opening` `a_contained_ball_builds_through_the_section_pass`.

The rounded stack of PR 3657 is not this cause: it refuses at the
section pass's R-tan on a coincident cylinder pair
(`section_cylinder_pair_coincident`), and stays on
`rounded-stack-subtract-and-intersect-refuse-fallback-extent`. The
carrier-tangency siblings are
`extent-scan-carrier-tangency-off-the-faces-refuses`.
