---
id: the-line-conic-router-reads-its-projections-from-the-lines-carrier-origin
kind: issue
title: curve_pair's line_conic router reads its overlap from the line carrier's origin, not through geom_core::one_line; whether the anchor costs any decision is unshown
status: open
priority: P3
cost: E
refs: [5481, two-spans-projections-on-one-line-are-read-four-ways]
opened: 2026-10-20
---


Found in PR 5481 (`geom_core::one_line`, PR 1 of
`two-spans-projections-on-one-line-are-read-four-ways`), and filed
there by the orchestrator's ruling. It was not folded into that PR
because it is a different question (scope), not because its bits would
move.

## What

`crates/geom-brep/src/curve_pair/pose.rs` `line_conic` reads where a
line and a conic can meet as the overlap of two intervals on the
line:
- the line span's ends;
- the conic's extent `[c − r, c + r]`, with `c = (center − o)·u` and
  `r` its larger semi-axis.

Both are read from the line CARRIER's origin `o`, through the file's
own `on_line`, a second spelling of the projection `geom_core::one_line`
reads from the span's start (the #5481 review's S3). The router's gap
row (`curve_pair_pose_gap`) decides `−extent − screened`, and
`screened` carries `rounding_charge(|o| + |center| + …)`.

## That charge is not the defect it looks like

Read naively, `|o|` in the charge is an artefact of the anchor, and
reading from the span start would cut it. #5481's first head made that
cut for `line_line`, and its reviewer showed it wrong: the line's ends
are `o + dir·t`, evaluated in `f64` at `|t| ≈ |o|` when the origin is
far from the span, so they carry `~u·|o|` of rounding whichever anchor
reads them. Dropping it answered a touching pair "no meeting"
(`meet_rows::two_segments_touching_on_far_carriers_are_never_read_apart`).
`line_conic` charges those metres today, and is right to.

So the anchor alone buys at most the projection chain's own rounding,
the smaller term; the evaluation term stays. What is left is the second
spelling, and whether any pose decides differently for it.

## Witness first

Look for a pose this decides badly before building anything: a line
span beside a conic whose extent along the line ends a small gap from
it, at `ε = 1e-12`, at every placement of the line's origin (at the
span, and `10³`, `10⁶` m along it). Read the gap row's margin under both
anchors, each charged for the line ends' evaluation. If no pose's
decision moves, close this item as a spelling only, or fold the
spelling into whichever PR next touches `line_conic`.

## The fix, if the witness shows one

Read the line's ends through `geom_core::one_line`'s anchor (the span
`(0, (a₁ − a₀)·u)`, the conic's extent about `c = (center − a₀)·u`,
through `OnOneLine::of`), and charge both chains:
- **how the ends were produced**: `|o| + |dir|·|t|` per evaluated end,
  as `line_line` passes `on_one_line`'s `produced`; the conic's centre is
  a stored value;
- **the projections**: the points read (`a₀`, `a₁`, the centre) plus `r`.

The parallel screen keeps its own charge. Never charge the anchor's
points alone: that is the cut #5481's review caught.

## Bits

`line_conic`'s gap margins and its offset's evaluation point
(`a₀ + u·mid`) move. That bits move is never a reason to keep the
old anchoring: re-baseline whatever golden moves, and name it.
