---
id: the-ring-door-reads-a-spline-face-through-the-trims-point-reader-at-the-root
kind: unit
title: the root ring door reads a spline face: the spline trim's point reader moves to the crate root and the trim calls it
status: closed
opened: 2026-11-14
priority: P1
cost: H
branch: tesslane/spline-ring-door
closed: 2026-11-14
pr: 6951
---


Follows `a-rings-lift-is-placed-against-its-outer-loop-in-one-place` (unit 3a, branch `tesslane/ring-placement`). `a-curved-faces-domain-is-read-from-every-loop` needs it for spline faces.

## What it is

`Body::face_lift` (`crates/topo/src/pcurves_rings.rs`) places a face's rings in its outer loop's sheet by the ring rule (`pcurves::place_ring`). It reads an analytic face through `Outer::side`. It answers a spline face `RingPlacementError::SplineChart`: the door does not read a spline chart yet, and that answer says nothing about the face. The spline trim places its own rings by the same rule (`boolean::spline_region::Trim::with_rings`, reading a point with `Trim::place_in_cover`). The root cannot reach that reader: `crates/topo/README.md` `[topo.homes]` says a reading both the root and an engine need lives at the root.

The final state is that the trim's point reader lives at the crate root, the door reads a spline face through it, and `spline_region` calls it from inside the engine. Then `SplineChart` retires.

## What moves, by name (`crates/topo/src/boolean/spline_region.rs`)

- The trim: `Trim` (`of`, `from_pieces`, `with_rings`, `place`, `place_in_cover`, `read_window`, `principal`, `exits_clear`, `copies`, `copy`), `Place`, `Piece`, `Rect`, `gap`, `close_gap`, `walks_forward`, `image_pieces`, `bezier_pieces`, `half_width`, and `window_arms`.
- The cells it meters by: `Patch` and its impl, `bezier_cells`, `bezier_insertions`, `refined`, `blend`, `point_rounding`, `middle`, `tracks`, `mutant`.
- Their predicate names and caps: `SPLINE_REGION_TRIM`, `SPLINE_REGION_ARM`, and `Caps`, which the carrier walk shares. The work bounds already live at the root.
- What stays in the engine: `spline_face_placement`, `spline_chart_placement`, `read`, `read_capped`, `carrier_walk`. They reach the cells and the trim at the root.
- The module docs' *The trim, in the chart* and *Rounding* sections move with the code. Its predicate names (`bool_spline_region_*`) are a K name's prefix, which `[topo.homes]` says names the module that decides; renaming them is part of the move.

## What the gate says, each direction

`scripts/gates/engine-leaves.sh`, run through `scripts/run-gates.sh` (which sets `GATE_BASE`):
- **Root to engine is refused.** The root naming `boolean` raises `names-boolean`, as `pcurves_rings.rs` calling `spline_region::ring_decks` did on 3a's first head.
- **Crate-wide items in the engine are refused.** A `pub(crate)` item in the engine raises `crate-visible boolean/spline_region.rs`.
- **Engine to root is allowed.** `spline_region` already imports `crate::pcurves::Side` and the work bounds.

## Rows that read this code today

- SPLICE: `work/splice/a-spline-faces-trim-reads-nothing-at-a-closed-seam-or-a-pole` names `Trim::exits_clear` and `Trim::read_window`.
- SPLICE: `work/splice/the-spline-chart-cell-walk-is-written-twice` (the cells), `work/splice/the-spline-face-floors-read-charge-is-wider-than-its-read`, `work/splice/a-meeting-on-a-closed-charts-seam-is-not-isolated`, `work/splice/the-spline-placement-read-certifies-no-second-in-trim-foot` and `work/splice/an-end-on-a-spline-face-is-placed-in-space-not-on-its-sheet` all read `spline_region`'s readings.
- PCERT: `work/pcert/the-projected-frame-box-reads-an-uncharged-f64-restriction` cites the module.
- ONEDOOR: `work/onedoor/chord-joins-ring-side-reads-the-splitting-engines-loop-containment` cites the module.

Each of those rows would cite the new home after the move. The move is on SPLICE's and the boolean programs' ground, and is announced on their logs at dispatch.

## One reader, not two

Review of `a-rings-lift-is-placed-against-its-outer-loop-in-one-place` found that the analytic reader (`pcurves_rings.rs`, `Outer::side` and `Outer::parity`) is near line for line the trim's (`Trim::read_window` and `Trim::copy`). Both use:
- the same clear-in-either-channel rule;
- the same `continue` ladder;
- the same half-open `v ≥ q_v` parity;
- `Near` at the depth limit.

What differs is the box source: Bézier pieces with rounding charges on a spline chart, `Pcurve::chart_box` on an analytic one. So the loop can be one, over a piece reading (box, ends, halves) that each family supplies, with each family keeping its own boxes. Then `with_rings` decides what it decides now. This unit makes them one. The trim's ring rows (`boolean/spline_region_rows.rs`) pin the region its ring placement gives, not the deck it moved the ring by, since the trim reports no deck; a pin on the deck returns here, when the door reads a spline face and hands its deck out. The work bounds are already one home (`pcurves::{SHIFTS, PIECE_DEPTH, PIECE_VISITS, HALVINGS, WINDOW_SHARE}`, which the trim reads).

The other ray-parity readers in `topo` are the class to sweep when the loop is made one:
- `chart_region`'s `ray_walk` and `point_in_polygon`;
- `splitting/containment.rs`'s `point_in_vertex_polygon` and `point_in_loop`;
- the point-in-face door's model-space ray cast that `validate.rs` reports (its `GRAZED` ending);
- `cell_region`, which reads in model space and draws no chart polygon, so is likely not in the class.

## Measured since filing

A door-built body does close a ring on a spline face: `crates/sweep/tests/a_partner_across_a_lofted_wall_is_joined_on_its_section_value.rs`, `a_brick_inside_a_lofted_walls_face_closes_a_ring_on_the_wall`. Every operation there refuses at the inside/outside test before finishing, so the ring exists mid-operation only. No at-rest body today holds a ring on a spline face, and so none reaches `SplineChart`.

## Unit (2026-11-14, TESSLANE)

**Letter H. Tier: single review, FULL.** A large move across the engine seam with predicate names renamed, then two readers made one: a placement or a point reading that changes in the move is silent, so the review carries claims to falsify. Not dual: the verdicts are pinned by rows that exist (`boolean/spline_region_rows.rs`, sweep's lofted-wall rows), and the unit sets no new contract.

Branch `tesslane/spline-ring-door`, cut from main.

**Owed by the unit, beyond the sections above:**

- **Measure before building.** The "what moves" list above was written from reading. Compile the move and say in the PR body what the trim reaches and what in the engine reaches it; where the list is wrong, the measurement wins and the PR says so.
- **Three commits, each readable apart.** First the move with nothing else changing: the code lives at the crate root where `[topo.homes]` puts it, the engine calls it, `spline_region_rows.rs` passes unedited, and the predicate names take their new prefix with the roster following (`recourse_roster` names only main's three `ring_path` sites). Then the door reads a spline face and `SplineChart` retires. Then the two parity loops become one over a piece reading each family supplies.
- **No verdict moves.** If making the loop one changes what `with_rings` decides on any row, or the move changes a spline reading, stop and report it; do not re-pin.
- **The deck pin returns.** A row pins the deck the door hands out for a ring on a spline face. No at-rest body holds such a ring today: say, measured, how the row builds one (a hand-built body, or the mid-operation body sweep's lofted-wall rows reach), and that it reds on no move and on the wrong move.
- **The class sweep is named, not rebuilt.** Each ray-parity reader listed above gets one disposition: served by the one loop here, or not this unit and filed.
- **The citing rows follow.** Each SPLICE, PCERT and ONEDOOR row above cites the new home, in this branch.
- **Run what CI runs before every push:** `bash scripts/run-gates.sh`; `-p topo` whole at the default eps (its source censuses read every file, so a narrow filter misses them); the rows you add or move at `CAD_TOLERANCE_EPS` 1e-6 and 1e-12 as well; `recourse_roster` and the other slow-set censuses by name. A row that probes near a boundary states its offsets in the run's tolerance.

## Closed (2026-11-14, PR 6951)

The spline trim and the Bézier cells it meters by live at the crate root (`crates/topo/src/spline_trim.rs`); the boolean's spline arm keeps its carrier walk and two placements and calls the root. `Body::face_lift` reads a spline face through `spline_trim::ring_decks`, which places each ring by `Trim::ring_shift`, the rule the trim places its own rings by, so the door's deck and the trim's are one decision. `SplineChart` is gone; `SplineTrimUnread { ring, cause }` names the cases where the trim is not read. The two ray-parity loops are one, `chain_parity::ray_parity_as`, over a piece reading each chart family supplies.

No verdict moved: the move was checked item by item against main, and the review ran main's two loops against the one over 138,000 readings in both families, twice, with every answer the same.

What "the section above" got wrong, by the implementer's measurement before building: `CELL_DEPTH`, `keep`, `Caps` and `CELL_VISITS` stay in the engine with the carrier walk; `Control`, `tracks`, `bezier_insertions`, `close_gap`, `walks_forward`, `half_width` and the mutant hook move; the ONEDOOR row needed no edit.

Left for others: the cells take no `Body`, so their one home is `geom-brep` (`work/splice/the-spline-chart-cell-walk-is-written-twice`, which names their address now); `chart_region`'s chart-polygon ray parity is a second reader of the class (`work/charttail/the-chart-polygons-ray-parity-and-the-chain-piece-parity-are-two-readers`); no fixture reaches `SplineTrimUnread`.
