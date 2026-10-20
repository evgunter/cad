---
id: mint-once-is-pinned-only-by-timing
kind: issue
title: Mint-once is pinned only by timing: a never-filled chart slot leaves every row green
status: closed
opened: 2026-10-20
closed: 2026-10-20
priority: P3
---


**P3.** Found by #5419's FULL review (https://github.com/evgunter/cad/pull/5419#issuecomment-6285986929).

**What.** #5419 mints each wall's `ChartedNurbs` once per operation, through `topo::offset_derive::WallCharts`'s `OnceCell` slots. The review's mutant M3, where a slot is never filled so every section re-mints, leaves every row green: only timing shows mint-once. A later change that quietly re-mints would pass review.

**Done when** a row counts mints, like derivative's test-only `WEIGHT_HALVINGS`. On the shelling fixture it asserts 5 mints for 4 walls: one per wall in the planning frame, plus the post-frame `plane_nurbs_limbs` mint. M3 must go red on it.

**Ride-alongs** from the same review:
- the `Wall` doc still says the door "refuses", though topo now panics on `WrongLane`;
- the slot type is spelled out four times in nested tuples;
- the moved-slot keying relies on a unique `old_key`, which the `planned_charts` test hook doesn't gate;
- two dead `work/shell/` citations in `replace_face.rs` (`:2099`, `:3682`).

## Closed (2026-10-20)

**The mint count.** `geom_brep::charted_nurbs_mints()` is a test-only
(`test-support`) per-thread count of `ChartedNurbs::mint` and its grid
form.

**The row.**
`offd_r1_probes::moving_every_chart_mints_each_spline_walls_chart_once`
moves every chart of the planar prism and of the twisted loft through
`offset_surfaces_together`. Each body has four spline walls, and the
row asserts four mints per operation.
- **M3** (a slot never filled) reads 8 on the prism and 16 on the
  loft, so it goes red.
- **Held slots too.** Moving every chart never reads a held slot
  (`WallCharts::held`), so a held-wall re-mint passes that row. The
  second row,
  `offd_r1_probes::moving_two_opposite_walls_mints_each_held_walls_chart_once`,
  moves two opposite walls and holds the rest (moves of zero), so
  each held wall is read by both its seams. It asserts four mints
  (two moved, two held), and a held slot never read reads 6.
- **Why not the shelling fixture.** It is in the slow set, which CI's
  default run skips, and it costs 54 s. The prism and loft rows run
  in 0.13 s in the default set, and their walls are each read by
  several sections.

**Ride-alongs.**
- The `Wall` doc says the door's refusal panics at topo's call sites.
- The slot type is one alias, `offset_derive::WallSlot`, and the two
  `type_complexity` allows it forced are gone.
- `planned_charts` asserts no two moves share an `old_key`, as the
  door's `SharedSurfaceKey` gate does.
- The two `work/shell/` citations in `replace_face.rs` point at
  `work/shelf/`, where those items moved.
