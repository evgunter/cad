---
id: three-walks-of-a-loops-chart-images-each-claim-to-be-the-one
kind: issue
title: Three walks of a loop's chart images each claim to be the one: chart_window, pcurves::loop_lift, boxes::torus_chart_window
status: open
priority: P1
cost: M
design: true
opened: 2026-11-03
---


Found at the review of `onedoor/carrier-and-window` (PR 6480, 2026-11-03), which moved the first of them to the crate root. A design question: which walk is the one, and what the other two read from it.

Three readings walk a loop's half-edges on their face's chart, take each one's image, and pin each image's period branch against the one before. Each says it is the one:

- **`topo::chart_window`** (`run_azimuth_images`, under `face_azimuth_images` / `face_azimuth_window` / `face_loop_azimuth_windows`). It derives each edge's image from its carrier again (`geom_brep::chart_pcurve_over`) instead of reading the stored row. It pins every joint by its own decision: nearest branch through `pcurves::lever_periods` (`chart_window_branch`), the face's orientation at a sphere pole (`pole_branch`, `chart_window_pole_branch`), and a refusal at a cone apex (`ApexUnlifted`). It reads the azimuth channel alone, at the caller's band, and hulls it. Readers: the containment's sphere trim, the split's wideness rule (`splitting::rules`), and the chord join's island winding (`chart_island_winding`).
- **`topo::pcurves::loop_lift`** ("the readers' one accessor for where a loop's rows sit in its face's chart", with `Body::loop_lift`) over **`lift_decks`**. It reads the STORED rows (`Body::pcurve`) and the stored joint elements (`Body::joint`), and sums their deck transformations from a start chosen after the last reset. A pole or apex joint is a reset that restarts the first channel. It works on both channels, with no band (`ChartPeriods::stated`), and answers `LiftGap` where a row or element is missing. `lifted_images` is its per-half reader.
- **`topo::boolean::boxes::torus_chart_window`** ("The ONE walk", over `TorusChartWindow` steps that `face_window_steps` builds from `pcurves::lifted_images`). It covers a torus only and both channels, reads the stored lift, and abandons the window on a loop with no half-edge.

So the torus window already reads `loop_lift`'s chain, while `chart_window` decides its branches again from fresh images. Its pole and apex rules restate, as decisions, what `loop_lift` reads off stored reset elements. The question is whether `chart_window` should read the stored lift (and what then decides a body with no stored rows, mid-operation), or whether the stored lift should be checked against `chart_window`'s decisions; and whether "the one walk" claims at `loop_lift` and `torus_chart_window` survive the answer. The hazard is the one `chart-window-walk-written-twice` named: a pin argument stated at two sites, with nothing holding them equal.
