---
id: cylinder-union-battery-refusals-carry-no-repeated-id
kind: issue
title: A cylinder-through-a-prism-corner union battery still refuses Triangulation, SelfTouchingTrimLoop and 'degenerate trimmed boundary' with no repeated id
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by the review of `tesslie/nu-2-repeat` (PR 7200).

## What

PR 7200 makes the trimmed lane hold a vertex its loop meets twice on one
chart branch at one UV location (`crates/mesh/src/trimmed.rs`,
`one_location_per_branch`). On the review's battery, 292 panics and 43
`Triangulation` refusals became meshes. The refusals that remain carry
no repeated mesh id, so they have another cause. Per δ:

- 147 `Triangulation`;
- 9 `SelfTouchingTrimLoop`;
- 8 `MissingEntity { what: "degenerate trimmed boundary" }`. PR 7196,
  open as this is filed, rewords that refusal.

None is measured: no raising site or payload has been read past the
variant.

## The battery

Rebuild it from the `cyl` set of `r2_pinch_probes` (branch
`join/pierce-pinch-families-review-r2`, `cyl_case`).
`crates/sweep/tests/a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta.rs`
(`cylinder_through`) builds one body of it on main's API. The cases:

- prisms `Lbot`, `notchbot`, `Ltop` and `notch307`, `v` the reflex corner;
- 60 Fibonacci directions `m`;
- ψ ∈ {0, 0.9, 2.2}, the axis ⊥ `m` turned by ψ;
- zoff ∈ {0.9, 0}, the arcs' junctions that far round the wall from `v`;
- both orders (pc, cp);
- union, meshed at δ 1 and at δ 0.3.

## The shape to give

For each refusal family, read the raising site and the face's trim
polygon. Then either mesh it right (`check_mesh` and `check_volume`)
or refuse it typed with its true cause.
