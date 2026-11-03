---
id: the-conical-ended-football-has-no-tour-stop
kind: issue
title: the conical-ended football builds through the public revolve door but has no tour stop, because its natural spot beside the vase renumbers the north-star audit and its STEP export refuses
status: open
opened: 2026-11-03
priority: P3
cost: M
---

(SPIRICARM, stage 1 piece 2 of `horn-and-spindle-tori-have-no-representation`.)

## What

The unit asked for the football in the tour if a natural spot exists. One does: an off-sheet stop beside `vase` in `demos/tour/src/bodies.rs`'s `stops`, authored in PATHS algebra as a user would — `Open.at((0, −1.4)).line_to((0.3, −0.8)).arc_to(Via { q: (0.6, 0), p: (0.3, 0.8) }).line_to((0, 1.4)).line_to(Start)`, revolved `Full` about `y`, one lemon sheet (`R ≈ −0.617`, `r ≈ 1.217`) between two cones. It was written, built and passed `demos/tour` clippy on the piece-2 branch, then withdrawn before merge for three costs CI named (run 41730711271):

- `crates/pncad/tests/all.rs::the_north_star_audit_has_a_row_for_every_tour_stop`: `docs/guide/north-star-audit.md` has a row per stop in tour order, so a stop after `vase` renumbers rows 5–53 and every "row N" cross-reference on that page and in `crates/pncad-py/tests/test_north_star.py`'s docstrings, and the headline tallies re-derive. A YES grade wants a Python row executing it (`TestVase` is the model).
- The tour's STEP manifest: `step_export` refuses `UnsupportedSurface { kind: "spindle" }` (`work/export/a-spindle-sheet-exports-as-degenerate-toroidal-surface.md`), so the scene needs `SceneBody::step_at_frontier` pinned to that variant, with a `retire` text naming that row.
- A new standalone frame, through the `[render]` flow.

## Done when

The stop is in the tour beside `vase`, its audit row graded (and every renumbered reference re-derived), its STEP frontier pinned or its export written, and its frame rendered.
