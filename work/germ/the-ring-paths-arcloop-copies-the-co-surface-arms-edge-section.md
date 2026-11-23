---
id: the-ring-paths-arcloop-copies-the-co-surface-arms-edge-section
kind: issue
title: The ring path's ArcLoop decides a torus loop arc's own loop of its plane section by hand, a copy of the co-surface arm's edge Section
status: open
opened: 2026-11-22
---

Filed by GERM's `germ/co-surface-table` lane (PR 7431).

## Finding

The co-surface arm now holds an edge's own section of its plane
(`geom_brep::curve_pair::on_surface::Section::{Circle, Oval}`, read by
`OnSurface::meet`): which of a torus circle's plane curves, and which
oval of a spiric's plane, a meeting lies on. The ring path keeps its own
copy of that reading: `ArcLoop` and `LoopSplit` in
`crates/topo/src/ring_path.rs` decide, on
`split_ring_path_torus_loop`, which loop of a torus loop arc's plane
section a crossing of that plane lies on (`ring_path::Oval`,
`LoopArc::torus_spiric_arc`), and the ring path reads its pairs through
the 3D curve-pair table (`geom_brep::curve_pair::meet`, `ring_path.rs`'s
`use geom_brep::curve_pair::{Hit, Span, SpanEnd, meet}`), which C5 says
no consumer holding two curves on one surface does.

The unit `co-surface-direction-table-and-spiric-plane-door` names this
copy as going "when the ring path moves"; PR 7431 does not move the
ring path. Moving it reads the ring path's pairs through
`OnSurface::meet` and retires `ArcLoop`'s section reading. Not verified
here: whether every ring-path pair is co-surface (charted from one
record), which `meet` needs.

## Also: `plane_roots` reads the curve × face door directly

Added from PR 7431's confirmation review. Since PR 7411
(`germ/plane-pose-far-frame`, merge `3547fe5764`), `ring_path::plane_roots`
(`crates/topo/src/ring_path.rs:2178`) reads a circle arc against a
plane through `geom_brep::curve_face::cross` (`:2189`). The arc and the
plane come from one torus record, so this is a co-surface pair, but the
read skips `same_record` and spells its own direction: the arc across
the plane. The move belongs on `OnSurface::meet`, which reads the
direction from `on_surface::direction`. It also makes C5's ratified
sentence true: two curves on one surface read through the co-surface
arm.

## Evidence: `face_normal::the_planar_sense_flip_lives_in_one_place`

PR 7431's review saw this topo row red at the PR's head `7f72931e9d`,
naming `ring_path.rs`. Re-run 2026-11-23 on worktrees:

- **Red at `7f72931e9d`.** The row's guard at that head is file-wide:
  `code.contains("Surface::Plane {") && code.contains("from_chart")`
  over every topo file. `ring_path.rs` holds `from_chart` at `:195` and
  `Surface::Plane {` at `:2188` and `:3537`.
- **The pair landed with PR 7411.** Its commit `12e3d936bb` ("ring path
  reads cross, uncharged") added the `Surface::Plane {` mints. With
  `3547fe5764^1` alone the file holds no `Surface::Plane {`, so the
  file-wide guard was red on main from `3547fe5764` onward.
- **Green on `origin/main` `16f708eb62`.** PR 7494 (`ciw/census-text-pins`,
  merge `9eb483e0c7`, commit `e4c75addce`) narrowed the guard to one fn
  body at a time (`test_utils::source::fn_items`). `from_chart` and the
  plane mints sit in different fns, so it passes.
- **Not a main red today, so no P0 row.** Main was red from `3547fe5764`
  until `9eb483e0c7`. PR 7431 picks the narrowing up when it next merges
  main. Its merge ref already carries it.
