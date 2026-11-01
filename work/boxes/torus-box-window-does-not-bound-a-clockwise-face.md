---
id: torus-box-window-does-not-bound-a-clockwise-face
kind: issue
title: The torus box window is orientation-blind: it bounds the patch a loop encloses, not the face, so it does not bound a clockwise-wound face's region
status: open
opened: 2026-10-18
priority: P3
cost: M
---


Found in the review of RESTREAD's PR 5269 (`restread/torus-region`),
2026-10-18, by reading.

## What

`boolean::boxes::TorusChartWindow` (the one window walk; its readers
`boxes::face_box`, `census::torus_chart_window` and
`splitting::classify::torus_window_reach`) folds a torus face's
boundary pcurves into a chart rectangle. That rectangle bounds the patch
the loop encloses in the chart. A face whose loop runs clockwise under
its outward normal holds the other side of that loop, the rest of the
torus. The window still answers the patch rectangle, so the box it
yields does not contain the face. `boolean::torus_region`, which reads
the side from the loop's orientation, reads such a face as the
complement (`solid_contain`'s `stated_periods_rows`, the clockwise wall
`boxes::tests::torus_wall_clockwise`).

## Reach

Latent. No door mints a clockwise torus face at rest: tier 3 checks the
orientation and the minting doors run the loop counterclockwise. The one
known source was the test fixture `boxes::tests::torus_wall`, whose `mef`
left a clockwise loop under `sense: true`; PR 5269 gave it `sense: false`,
so the fixture is now the patch it describes. The clockwise wall stays
as a row of its own.

## The shape of a fix

Read the loop's winding from the face's sense against the chart's
orientation (the walk already has both), and answer the whole chart, or
no window, where the face is the outside of its loop.
