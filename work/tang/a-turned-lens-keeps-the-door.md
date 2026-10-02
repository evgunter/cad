---
id: a-turned-lens-keeps-the-door
kind: issue
title: A lens of two domes turned on each other keeps the crossing layer's door
status: open
opened: 2026-10-02
---

## What

The lens of `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
(`a_lens_of_two_domes_builds_with_its_discs_declared_rest`): the dome of
radius `√2` standing on its rim circle, unioned with the bowl that
mirrors it, the two discs declared `Rest`. With the two seams aligned it
builds at `2 · V(dome)`. Turned about the axis by `π/7` or `π/2`, it
refuses `CurvedPierceUnsupported` in both member orders (measured on
branch `tang/abutting-rim`, PR 3823; the row pins it).

## Why

Each dome rim semicircle lies on the bowl's sphere and runs along parts
of two bowl arcs. `reduce::lying_on` has two certificates:

- **(b), the chain**, needs both ends of the arc paired with vertices of
  the partner. A turned semicircle ends inside a bowl arc, and the end
  is paired only once the other sweep direction splits that arc.
- **(a), the boundary meets the circle only at the ends**, is
  conservative here: the bowl face's boundary lies on the rim's circle
  itself, which (a) never certifies.

With (a) forced true, order 0 builds (measured by mutating
`boundary_meets_circle_only_at`); the reviewers of PR 3823 measured
`V = 2 · V(dome)` there. So the refusal is the certificate's reach, not
the geometry.

## Direction

The arc runs along the partner's boundary on the same circle. A
certificate that reads an overlapping run of the partner's arcs, not
only a chain from paired end to paired end, would cover it, and so
would ordering the sweep so the partner's split vertices exist before
the arc is asked.
