---
id: point-in-solid-reads-in-band-against-a-face-plane-far-from-the-face
kind: issue
title: point_in_solid reads witnesses in-band against a face's plane where the point is far from the face itself, so a shell or wedge whose witnesses all lie near such planes refuses (ShellWitnessExhausted) where the answer is clear
status: review
opened: 2026-10-01
priority: P2
cost: M
branch: cleave/far-plane
---


Unmeasured beyond what two lanes saw. The hypothesis has not been checked.

- PR 3716's review traced the `m5_pr8_bvh_diff` grazing fixture: a
  corner read in-band against operand B's top plane while sitting 4 m
  from the face itself. PR 3716 now treats that reading as
  inconclusive, and another witness decides.
- After PR 3716, `offer_rows`' `wide_wedge_with_a_far_vertex_{union,intersect}`
  refuse `ShellWitnessExhausted` with 0 witnesses on the boundary and
  26 in-band. Before, they refused `Containment`. Why every witness of
  that wedge reads in-band was not investigated. The guess is the same
  grazing against an infinite plane.

What to establish first is where `point_in_solid`'s in-band margin
comes from: the face's carrier plane, or the face as bounded. A
reading against an unbounded carrier, for a point outside the face's
extent, would explain both cases.

## Measured on `reach/opensign-red` (2026-10-01)

That branch levers the ray's parallel test against a face's plane
(`bool_point_in_solid_denom`) by the selection's reach instead of
reading the bare cosine. With it, both `wide_wedge_with_a_far_vertex_*`
cases build below the vertex's offer, where they refused
`ShellWitnessExhausted` before. The `m5_pr8_bvh_diff` corner's reading
is still unexamined.

Measured by the branch's review (probes over the wide wedge at
ε = 4.95e-10, just below the vertex's offer):

- On main the union and the intersect both refuse
  `ShellWitnessExhausted`, with 26 of the shell's witnesses in band.
  The first in-band margin is `bool_point_in_solid_denom`, at 1.91e-9:
  the bare cosine against a face's plane.
- On the branch the union builds, with volume 64.5 (the closed form),
  and 0 of 1720 `point_in_solid` probes against the closed form answer
  wrong. The intersect is `Empty`.

## Measured on main at 6cf5d4b (2026-10-02, cleave/far-plane)

- **`wide_wedge_with_a_far_vertex_{union,intersect}`.** At
  ε = 4.95e-10 both PASS, with 0 in-band witness readings. PR 3755
  re-baselined them to `Withdrawn(Passes)`. Union volume is 64.5
  (8·8·1 + sin 30°), and the intersect is `Empty`. The same holds at
  1e-12.
- **The `m5_pr8_bvh_diff` grazing pose** (k ∈ {2, 5, 9}): 36 in-band
  readings, all from B's top corners against A. The first is the
  boundary pre-pass, not a ray: `bool_point_in_solid_plane` on A's
  bottom carrier, with elevation k·ε, while the corner is 4–5 m from
  the face. Fixing only that exposed two more layers, one after the
  other:
  - the in-plane loop walk's `point_in_loop_side`, on A's side face
    whose carrier holds the corner: its first ray, +x, passes k·ε from
    the vertex (5, 0, 1 + k·ε);
  - the 3-D ray +x hitting A's x = 6 face k·ε below its bottom edge,
    where `point_in_loop_boundary` reads in band at the hit point.
- After all three: 0 in-band readings.
