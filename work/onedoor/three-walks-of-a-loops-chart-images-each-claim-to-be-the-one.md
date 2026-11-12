---
id: three-walks-of-a-loops-chart-images-each-claim-to-be-the-one
kind: unit
title: Three walks of a loop's chart images each claim to be the one: chart_window, pcurves::loop_lift, boxes::torus_chart_window
status: spec
priority: P1
cost: M
opened: 2026-11-03
---


Found at the review of `onedoor/carrier-and-window` (PR 6480, 2026-11-03), which moved the first of them to the crate root. A design question: which walk is the one, and what the other two read from it.

Three readings walk a loop's half-edges on their face's chart, take each one's image, and pin each image's period branch against the one before. Each says it is the one:

- **`topo::chart_window`** (`run_azimuth_images`, under `face_azimuth_images` / `face_azimuth_window` / `face_loop_azimuth_windows`). It derives each edge's image from its carrier again (`geom_brep::chart_pcurve_over`) instead of reading the stored row. It pins every joint by its own decision: nearest branch through `pcurves::lever_periods` (`chart_window_branch`), the face's orientation at a sphere pole (`pole_branch`, `chart_window_pole_branch`), and a refusal at a cone apex (`ApexUnlifted`). It reads the azimuth channel alone, at the caller's band, and hulls it. Readers: the containment's sphere trim, the split's wideness rule (`splitting::rules`), and the chord join's island winding (`chart_island_winding`).
- **`topo::pcurves::loop_lift`** ("the readers' one accessor for where a loop's rows sit in its face's chart", with `Body::loop_lift`) over **`lift_decks`**. It reads the STORED rows (`Body::pcurve`) and the stored joint elements (`Body::joint`), and sums their deck transformations from a start chosen after the last reset. A pole or apex joint is a reset that restarts the first channel. It works on both channels, with no band (`ChartPeriods::stated`), and answers `LiftGap` where a row or element is missing. `lifted_images` is its per-half reader.
- **`topo::boolean::boxes::torus_chart_window`** ("The ONE walk", over `TorusChartWindow` steps that `face_window_steps` builds from `pcurves::lifted_images`). It covers a torus only and both channels, reads the stored lift, and abandons the window on a loop with no half-edge.

So the torus window already reads `loop_lift`'s chain, while `chart_window` decides its branches again from fresh images. Its pole and apex rules restate, as decisions, what `loop_lift` reads off stored reset elements. The question is whether `chart_window` should read the stored lift (and what then decides a body with no stored rows, mid-operation), or whether the stored lift should be checked against `chart_window`'s decisions; and whether "the one walk" claims at `loop_lift` and `torus_chart_window` survive the answer. The hazard is the one `chart-window-walk-written-twice` named: a pin argument stated at two sites, with nothing holding them equal.

## Designed (Ev, 2026-11-12, PR 6851: "i think this plan is good. i don't want to keep reset")

Two designers converged in two rounds (fork-log row 262). Ev ratified C4 with no reset anywhere. The final state:

- **One decision.** `pcurves::decide_joint` is the only code that decides a joint's branch. `chart_window::nearest_branch`, its K name `chart_window_branch` and `run_azimuth_images` are deleted. `pole_branch`/`PoleJoint` move beside `decide_joint`.
- **The singular set.** A joint on a sphere pole, a cone apex on one nappe, or a spindle torus's point stores the pole rule's decided element, the turn on the face's side. The rule is the general one: the angle between the two edges' tangents at the point, on the face's side. It is not today's meridian-only `pole_branch`. `JointElement::Reset` retires.
- **Refusals.** A joint whose element cannot be decided past the band is a typed refusal at the door that makes it. That covers a spline net stretch undecided in either channel (the first channel joins the second's `NetStretchUndecided`), and a singular point the rule does not decide.
- **One `Lift` type, two constructors the caller names.** *Read* takes the stored rows: no band, and a hole is loud (`LiftGap`). *Decided* runs the mint's walk (`walk_cycle`) at the caller's band, over a loop or an open run. There is no fallback between the two. Windows, hulls, the winding and "closes in the chart" (an integer winding) are methods on it.
- **Readers.** Containment's sphere trim, `cone_may_hold_apex` with the split's wideness rule, `torus_chart_window` (its wrap guard is the winding), and the mesh trim polygon all read the lift. The chord join's island winding reads a decided lift over its run at the join's band. `mesh::trimmed::choose_pole_azimuths` and its `PoleJunctions`/`PoleAzimuth` refusals retire. `chart_window.rs` keeps no walk, and its refusals stop borrowing the join's error, which closes `the-chart-window-walk-refuses-in-the-chord-joins-error`.
- **Sense.** A door that moves a loop between faces of opposite sense restates its singular-set elements: `kfmrh`, `ring_move`, `mfkrh`, `set_face_sense`, and any others the sweep finds. `revert` inverts them.
- **Owed as rows:**
  - The rule decides at the cone apex and the spindle. The corpus is the 245 tests that write an apex reset today and the 14 that write a spindle reset (census on `onedoor/looplift-probes`, `89b81dce`). Any body where it cannot decide refuses typed, and the unit counts them.
  - The lune and its complement get different elements, with widths 0.8 and `τ − 0.8`.
  - A kill's sum at a pole equals a fresh decision.
  - The probes on `onedoor/looplift-probes` (pole rules agree, twin joint, opposite sense) become rows.
- **Docs.** topo's README gains the lift in `[one-way.places]`. The `topo::joint` module docs and the "Where in the code" row for `chart_window` are rewritten to match.

## Spec (ONEDOOR, 2026-11-12)

- **Letter H, tier dual.** H because it carries a new rule (the general singular-set turn) plus a proof it owes at the apex and the spindle. Dual because it moves the pcurve mint, every lift reader in topo, the mesh's trim, and the doors that move loops. Under the protocol, H means a concurrent Opus/Fable review pair on one frozen head.

