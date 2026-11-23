---
id: the-shallow-circle-torus-row-exercises-no-root-assertion
kind: issue
title: a_shallow_crossings_root_is_within_the_band_or_refused: the door refuses every pinned pose, so its root assertions never run
status: open
opened: 2026-11-21
priority: P2
cost: E
---


## What

`crates/geom-brep/src/curve_face/circle_torus.rs:2058` `a_shallow_crossings_root_is_within_the_band_or_refused` judges the circle × torus door on the three `SHALLOW_POSES`. On both lanes and every ε the door refuses each pose, so the row's root assertions (every true root held, each certified root within `Kε` of arc of one) never run. It passes vacuously on that half. Add a pose the door certifies, or a counter that requires one certified answer.

Filed from the second delta review of PR 7131.
