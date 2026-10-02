---
id: swept-cocircular-arc-runs-build-one-wall
kind: issue
title: sweep: a run of cocircular arcs still sweeps one wall per arc (one surface key), where the ruling builds one wall per run on every carrier kind
status: open
opened: 2026-10-01
priority: P1
cost: M
---


Split out of `swept-continuation-walls-reach-the-boolean-unmerged`
(Ev's 2026-10-01 ruling, "construct": one wall per cosurface run on
every carrier kind, curved runs allowed to land as their own row).

## What holds

`crates/sweep/src/swept.rs::wall_runs` joins LINE segments only. Two
adjacent cocircular same-turn arcs still sweep one wall each, on one
shared surface key (`FaceSurface::Shared` in `extrude.rs::side_surface`
and `revolve/partial.rs::sweep_loop`), with a strut / latitude rim
between them described at rest in that one chart. A curved same-key
adjacency is the maximal-faces gate's canonical form, so the boolean
admits it; what the ruling's README text (`crates/sweep/README.md`,
"Walls: one per run") says is not yet true of it.

## The row

- `wall_runs` admits arc pairs; extrude's and the partial revolve's run
  builders already take a run of any length, so what changes is the
  run wall's surface (a cylinder/torus/sphere from the run's first
  segment, `u_ref` from its leading vertex) and the arc chain's top /
  end rims.
- A run that is the whole closed loop (a circle) keeps its canonical
  cut (C12.5): one run starting at the loop's start vertex, its strut
  the cut. `wall_runs` today `unreachable!`s on an all-joined loop,
  because no line loop can be one.
- The full revolve's `collapse_runs` collapses arcs too: the run's arc
  is the first arc with its end moved, the sweep summed.
- Fixtures that rely on the split today: `common::latitude_seam::two_arc_sphere`
  and `revert_periodic_wrap::two_arc_torus` (same-surface latitude
  seams on a sphere and a torus), `review_m2_pr4::survives_notched_circle_wrap_join_shares_the_key`
  and the cocircular-D in `review_m2_pr4::survives_sub_eps_oblique_vector_used_as_given`.
  The plane and line-wall latitude rings already moved to hand-cut
  fixtures (`latitude_seam::ring_on_cap`, `ring_on_wall`).
