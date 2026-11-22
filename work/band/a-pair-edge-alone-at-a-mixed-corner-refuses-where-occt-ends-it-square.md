---
id: a-pair-edge-alone-at-a-mixed-corner-refuses-where-occt-ends-it-square
kind: issue
title: A pair edge alone at a mixed corner refuses, though a strip ending square in the odd edge's other face would build
status: open
opened: 2026-11-17
priority: P2
cost: M
refs: [a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds]
---

Filed by the BAND orchestrator (2026-11-17) from the OCCT comparison Ev asked for on PR 7038 (comment 6937345056; renders and scripts on `band/occt-compare-renders` at 8394370cd0, `docs/renders/band-2026-11/occt-compare.md`).

**Measurement.** At a mixed corner (`crates/sweep/tests/band_mixed_corner.rs`, the flush L and the inset boss), a chamfer naming ONE pair edge alone refuses `FOOT_INSIDE_A_FACE` at every plan angle tried (60°, 90°, 120°) and on `leaning` (PR 7038's fix pass, M4, pinned in `band_mixed_corner_odd_plus_one.rs::the_mixed_corner_recourse_builds_where_it_says`). OCCT 8.0.1 builds the same request as an all-flat solid: the strip ends square in the plane of the odd edge's other face (`occt-flush-d-chamfer.png`).

**Against the rule** (`a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`, "The odd edge alone, or one pair edge alone"): the rule says the cut-off builds, and that a foot landing inside a neighbouring face refuses on the existing door. So the refusal is what the rule says today; building it means a cut-off end whose foot may sit inside the odd edge's face, ending in that face's plane. Whether that end is the rule's cut-off or a new end is the first question for the unit.

P2: the recourse already names requests that build, so nothing is stuck; it is a natural request a user may make.
