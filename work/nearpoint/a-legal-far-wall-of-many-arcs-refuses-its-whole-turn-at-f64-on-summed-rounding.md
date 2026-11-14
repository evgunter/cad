---
id: a-legal-far-wall-of-many-arcs-refuses-its-whole-turn-at-f64-on-summed-rounding
kind: issue
title: A legal far cylinder or torus wall of 40 to 200 arcs a rim refuses props_chart_loop_whole at f64 and eps 1e-12, definitely (LoopsWindChart) on some: each arc's rounding times the lever, summed over the loop
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Found by BOXFINISH's edge-turns review (branch `boxfinish/edge-turns`,
PR 6946). It is pre-existing: the reading on the PR's merge base
refuses these walls identically.

## Measured

Hand-built legal walls through `curved_face_loops`, frame at
(30, −110, 200), axis (1, 1, 1)/√3, at `f64` and ε = 1e-12. The pins
citing this row are in `crates/geom-brep/tests/far_walls_of_many_arcs.rs`.

- **Definite (`LoopsWindChart`), a legal body refused as no boundary:**
  - cylinder r 990, 200 arcs a rim;
  - torus major 600, 200 arcs a parallel;
  - torus major 990, 200 arcs a parallel.
- **Escalated on `props_chart_loop_whole`:**
  - cylinder r 213, 200 arcs: margin 2.62e-12;
  - cylinder r 410, 200 arcs: 6.91e-12;
  - cylinder r 990, 40 arcs: 1.24e-12;
  - torus major 100, 200 arcs: 1.58e-12;
  - torus major 300, 200 arcs: 5.33e-12;
  - torus major 990, 40 arcs: 1.50e-12.

At ε = 1e-9 and 1e-6 every one of them measures.

**What it is made of** (inferred, not measured per arc): each arc's
`f64` rounding, estimated at about 1e-16 rad, times the lever, summed
over the loop's arcs against a slack of `3ε` once for the loop. The
margins grow with the arc count and the radius, as that sum would. The sum grows with the count of arcs and with the
radius; the slack does not. A definite refusal of a legal body is the
worse half: tier 3 calls such a body no boundary, rather than reporting
that it cannot decide.

Unmeasured: whether a slack per edge (each arc's own rounding bound)
would decide these, and what it would admit.
