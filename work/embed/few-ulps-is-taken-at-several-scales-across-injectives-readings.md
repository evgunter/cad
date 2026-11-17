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
- `present_chart`, the touch's face side: bare `hi`, or bare `lo`;
- `present_chart`, the zero-width guard: `few_ulps(lo).max(few_ulps(hi))`,
  which is the touch's slack at either end of the rectangle.

PR 7086 added `chart_slack` (`few_ulps` of the largest value and
`TURN`). The extent is read at that scale by `face_extent` and by the
`sheets` overlap; the touch, `vacuous` and the guard are not.

**The guard cannot move without the touch** (measured on PR 7086).
With the guard on `chart_slack`, `a_filleted_drum_certifies_across_its_fillet`
at `Interval` refused `Uncertified` at depth 40, with 9,660 refused cell
pairs. The guard was then wider than the touch. A cell reaching just
past the touch onto a line's face side read no width, and every cell
below it reached as far.

The other spellings are safe today only where `TURN` dominates the
values read, or where the value read is the one that rounds.

## What is needed

Move the touch, `vacuous`, `covered`, `whole_turn` and the guard onto
one scale together, or say at each site why another scale is the one
that rounds. Each touch and cover reading is an ε concession, so a
scale change moves what it drops. Measure the injectivity rows (the
filleted drum first) and the tour walk before and after.
