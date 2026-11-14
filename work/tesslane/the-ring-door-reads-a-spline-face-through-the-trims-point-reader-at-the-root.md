---
id: the-ring-door-reads-a-spline-face-through-the-trims-point-reader-at-the-root
kind: unit
title: the root ring door reads a spline face: the spline trim's point reader moves to the crate root and the trim calls it
status: open
opened: 2026-11-14
priority: P1
cost: H
---


Follows `a-rings-lift-is-placed-against-its-outer-loop-in-one-place` (unit 3a, branch `tesslane/ring-placement`). `a-curved-faces-domain-is-read-from-every-loop` needs it for spline faces.

## What it is

`Body::face_lift` (`crates/topo/src/pcurves_rings.rs`) places a face's rings in its outer loop's sheet by the ring rule (`pcurves::place_ring`). It reads an analytic face through `Outer::side`. It answers a spline face `RingPlacementError::SplineChart`: the door does not read a spline chart yet, and that answer says nothing about the face. The spline trim places its own rings by the same rule (`boolean::spline_region::Trim::with_rings`, reading a point with `Trim::place_in_cover`). The root cannot reach that reader: `crates/topo/README.md` `[topo.homes]` says a reading both the root and an engine need lives at the root.

The final state is that the trim's point reader lives at the crate root, the door reads a spline face through it, and `spline_region` calls it from inside the engine. Then `SplineChart` retires.

## What moves, by name (`crates/topo/src/boolean/spline_region.rs`)

- The trim: `Trim` (`of`, `placed`, `from_pieces`, `with_rings`, `place`, `place_in_cover`, `read_window`, `principal`, `exits_clear`, `copies`, `copy`), `Place`, `Piece`, `Rect`, `gap`, `close_gap`, `walks_forward`, `image_pieces`, `bezier_pieces`, `half_width`, `window_arms`, and the `Placed` alias.
- The cells it meters by: `Patch` and its impl, `bezier_cells`, `bezier_insertions`, `refined`, `blend`, `point_rounding`, `middle`, `tracks`, `mutant`.
- Their constants: `SPLINE_REGION_TRIM`, `SPLINE_REGION_ARM`, `PIECE_DEPTH`, `PIECE_VISITS`, `COPIES`, and `Caps`, which the carrier walk shares.
- What stays in the engine: `spline_face_placement`, `spline_chart_placement`, `read`, `read_capped`, `carrier_walk`. They reach the cells and the trim at the root.
- The module docs' *The trim, in the chart* and *Rounding* sections move with the code. Its predicate names (`bool_spline_region_*`) are a K name's prefix, which `[topo.homes]` says names the module that decides; renaming them is part of the move.

## What the gate says, each direction

`scripts/gates/engine-leaves.sh`, run through `scripts/run-gates.sh` (which sets `GATE_BASE`):
- **Root to engine is refused.** The root naming `boolean` raises `names-boolean`, as `pcurves_rings.rs` calling `spline_region::ring_decks` did on 3a's first head.
- **Crate-wide items in the engine are refused.** A `pub(crate)` item in the engine raises `crate-visible boolean/spline_region.rs`.
- **Engine to root is allowed.** `spline_region` already imports `crate::pcurves::{RingMiss, Side}`.

## Rows that read this code today

- SPLICE: `work/splice/a-spline-faces-trim-reads-nothing-at-a-closed-seam-or-a-pole` names `Trim::exits_clear` and `Trim::read_window`.
- SPLICE: `work/splice/the-spline-chart-cell-walk-is-written-twice` (the cells), `work/splice/the-spline-face-floors-read-charge-is-wider-than-its-read`, `work/splice/a-meeting-on-a-closed-charts-seam-is-not-isolated`, `work/splice/the-spline-placement-read-certifies-no-second-in-trim-foot` and `work/splice/an-end-on-a-spline-face-is-placed-in-space-not-on-its-sheet` all read `spline_region`'s readings.
- PCERT: `work/pcert/the-projected-frame-box-reads-an-uncharged-f64-restriction` cites the module.
- ONEDOOR: `work/onedoor/chord-joins-ring-side-reads-the-splitting-engines-loop-containment` cites the module.

Each of those rows would cite the new home after the move. The move is on SPLICE's and the boolean programs' ground, and is announced on their logs at dispatch.

## Not measured

Whether any door-built body has a ring on a spline face. A boolean through a lofted or swept wall is where one would come from. Every one of the 24 rings `crates/mesh/tests/lift_probes.rs` measures is on an analytic chart. Measure that before pricing the unit against its need.
