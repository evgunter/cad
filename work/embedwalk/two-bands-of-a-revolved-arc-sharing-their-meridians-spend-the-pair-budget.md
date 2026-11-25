---
id: two-bands-of-a-revolved-arc-sharing-their-meridians-spend-the-pair-budget
kind: issue
title: Two π-band faces of one revolved arc, sharing both meridians that end on rim corners, spend the whole pair budget: on a spindle sheet and on a ring torus alike
status: dispatched
opened: 2026-11-25
priority: P1
cost: M
refs: [a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine]
branch: embedwalk/two-bands
---


## Finding

Measured by lane `embedwalk-spindle` (2026-11-25) with the at-rest
gate's per-solid ask ported onto the branch that gives a spindle sheet
its window (`topo::clearance`'s `window_of`, `sheet_latitude`) and its
by-kind discharge (`topo::injective::by_kind`). Untraced to the cell.

A full revolve of an arc on a spindle sheet whose ends are not on the
axis (a band between two rims, or between two cones) builds the sheet
as two π-bands sharing two meridian edges, each meridian ending on a
rim corner where the two bands and a cap or cone face meet. The sheet
band pair `(3v1, 5v1)` refuses `Budget { stop: Pairs { max_cell_pairs:
65536 } }` at the gate's budgets, about 3.5 s an ask, while each band
against itself is discharged by kind and every other pair holds:

- `sweep::all bool3_torus_doors::a_spindle_profile_revolves_to_a_sheet_and_no_torus`
  (two discs and the sheet: `CellReceipt { candidates: 9, discharged:
  31474, refused: 1, splits: 65527, certified: 14, by_kind: 4,
  abandoned: 34061 }`);
- every conical-ended football (four cones and the sheet's two bands):
  `a_conical_ended_football_revolves::*` (19 asks over five tests) and
  the two `stl::all` acceptance-export tests (two asks each).

**Not the sheet's own**: the near-horn ladder
(`a_conical_ended_football_revolves::the_near_horn_ladder_reads_its_band`)
builds the same body on a ring torus past the horn, and its torus band
pair `(3v1, 5v1)` refuses identically. The ring twin of the bool3
profile, moved one unit off the axis, is ONE whole-turn torus face and
holds (`discharged: 11`), so the shape is the band pair, not the kind.
The football and the apple, whose two bands share meridians that end
at the apexes (collapsed sides), hold at both scalars
(`sweep::all self_intersection_proves_injectivity::bodies_on_a_spindle_sheet_hold_by_kind_at_both_scalars`).

What a trace should settle first: which cell pair the walk spends the
budget on (at the rim corner, where a meridian meets a parallel and the
third face, or along the shared meridian away from it), and so whether
this is a presentation EMBED owns (`crates/topo/src/injective.rs`) or
the walk's. Re-home it if the trace puts it on EMBED's ground.

## Trace (lane `embedwalk-twobands`, 2026-11-25)

Demonstrated with a probe asking `topo::clearance::embedded` on main
(`436425120`) with every cell pair of the walk logged (instrumentation
not committed). The spindle receipt reproduces the finding's exactly.

- **Not at the rim corner, not along the shared meridian.** Of the
  bool3 spindle's 96 995 visits of `(3v1, 5v1)`, 65 479 are splits on
  the distance path with no shared feature near (`Near` empty), and
  63 248 of those are of cells that do not overlap in the chart. The
  touching arm splits 49 times and certifies 14. The splits sit at
  `v ≈ ±1.84`, the sheet's apexes: both bands' windows were the whole
  sheet, `u ∈ [0, 2π]`, `v ∈ [−v*, v*]`, while the faces span
  `|v| ≤ 0.583` between the rims. A cell of one band holding an apex
  and a cell of the other holding it are at distance zero at every
  depth. They are off both faces, but the region never says so:
  `outside: 0`, because every cell either fails
  `cell_region`'s singular-set clearance (it holds the apex) or is
  placed `None` (`contain::curved_interior_placement` has no spindle
  arm, `work/inside/the-point-door-places-no-point-on-a-spindle-sheet-face.md`).
- **The ring-torus twin is the same walk.** At `d = 2·escalate` the
  torus band pair's 64 891 splits sit at `v ≈ π`, the inner equator,
  `2e-8` from the axis and off both faces (`|v| ≤ 2`): the window was
  the whole tube, and `cell_region` cannot clear the axis there before
  the budget. At `d = 1e-3` the same pair already held (1653 pairs).
- **So the cause is the walk's window** (`clearance::window_of`), not a
  reading in `injective.rs`. The fix gives a torus band the chart
  rectangle its boundary's pcurves span (`cover::bounded_torus_window`)
  and a sheet band the latitude they span
  (`cover::bounded_sheet_latitude`). Measured with it: the bool3 spindle
  `Embedded` in 18 pairs (0.05 s); the near-horn ladder's torus, horn
  and sheet rungs `Embedded` in about 600 pairs each.
