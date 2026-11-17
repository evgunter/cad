---
id: few-ulps-is-taken-at-several-scales-across-injectives-readings
kind: issue
title: few_ulps is taken at several differently spelled scales across injective.rs's touch, cover and placement readings
status: open
opened: 2026-11-17
priority: P3
cost: E
---


## Finding

From EMBED lane half-sphere's review (PR 7086).

`few_ulps` (`crates/topo/src/injective.rs`) is the one slack for a
chart value that rounding alone can move. Its callers take it at
different scales:
- `whole_turn`: `TURN.max(lo.abs()).max(hi.abs())`;
- `covered`: each end of `want` separately;
- `vacuous`, the touch: bare `hi`, or bare `lo`;
- `present_chart`, the touch's face side: bare `hi`, or bare `lo`.

PR 7086 introduced `chart_slack` (`few_ulps` of the largest value and
`TURN`) and moved its own three readings onto it: `face_extent`, the
`sheets` overlap and the zero-width guard.

The other spellings are safe today only where `TURN` dominates the
values read, or where the value read is the one that rounds. A chart
value much smaller than 1 in magnitude reads a slack of a few ulps of
1 in every case, which is the floor `few_ulps` gives.

## What is needed

Move each caller onto `chart_slack` over the values it compares, or
say at the site why another scale is the one that rounds. Each touch
and cover reading is an ε concession, so a scale change moves what it
drops. Measure the injectivity rows and the tour walk before and
after.
