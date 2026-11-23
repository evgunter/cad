---
id: the-line-conic-router-reads-its-projections-from-the-lines-carrier-origin
kind: issue
title: curve_pair's line_conic router reads its overlap from the line carrier's origin, not through geom_core::one_line; whether the anchor costs any decision is unshown
status: closed
priority: P3
cost: E
refs: [5481, two-spans-projections-on-one-line-are-read-four-ways]
opened: 2026-10-20
closed: 2026-10-21
branch: nurbs/line-conic-anchor-witness
pr: 5521
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

## Closed

Not a defect, and the fix above would cost decisions. A line against
a conic's extent is not the two-spans-on-one-line reading, so
`line_conic` is no second home for `geom_core::one_line`; it keeps its
own anchor, and `pose.rs` now says why at `line_conic`.

**The witness** (commit `8c00c108` on `nurbs/line-conic-anchor-witness`,
`curve_pair::pose::line_conic_anchor_witness::probe_the_gap_under_both_anchors`;
removed in the next commit, re-run by checking that commit out):
- **The poses.** At `ε = 1e-12` (`K = 10`), 20 000 per placement of the
  line's origin: a line span `0.01`–`10` m long beside a circle of
  radius `0.1`–`10` m, off the line, whose extent along it starts
  `1e-13`–`1e-9` m past the span's end. The line's origin stands at
  the span, `10³` m or `10⁶` m back along it.
- **The two readings of the gap row** (`curve_pair_pose_gap`,
  Positive = apart):
  - **main**: projected from the carrier's origin, with
    `line_conic`'s `screened` charge;
  - **the recipe above**: projected from the span's start through
    `OnOneLine::of`, charged for both chains (each end's evaluation,
    `|o| + |d|·|t|`, plus the projections, `|a₁ − a₀| + |centre − a₀| + r`).
- **The oracle.** Every apart is checked against the real gap, the
  circle's extent start less the span's end, enclosed in `Interval`
  from the carrier's exact data; an apart whose enclosure is not past
  `ε` counts as unproven.

Two runs (the fuzz seed is drawn per run):

| origin | main apart | recipe apart | recipe gains | recipe loses | unproven apart (main / recipe) |
|---|---|---|---|---|---|
| at the span | 9849 / 9923 | 9863 / 9928 | 14 / 5 | 0 / 0 | 0 / 0 |
| `10³` m back | 9603 / 9677 | 8796 / 8882 | 0 / 0 | 807 / 795 | 0 / 0 |
| `10⁶` m back | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |

Neither reading misdecides. Main's charge is honest, and cheaper far
from the origin: its gap carries the carrier origin's metres once,
where a span-anchored read charges both ends' evaluation (`2|o|`). At
the span the anchor gains a handful of marginal poses in 20 000; `10³` m
back it loses about 8% of the aparts. At `10⁶` m every charge passes
every gap drawn.
