---
id: the-line-conic-router-reads-its-projections-from-the-lines-carrier-origin
kind: issue
title: curve_pair's line_conic router reads its overlap from the line carrier's origin, so a span far from that origin is charged its metres and a gap the band allows can go unseen
status: open
priority: P3
cost: E
refs: [5481, two-spans-projections-on-one-line-are-read-four-ways]
opened: 2026-10-20
---


Found in PR 5481 (`geom::one_line`, PR 1 of
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

Both are read from the line CARRIER's origin `o`, which can stand
metres from the span. The router's gap row (`curve_pair_pose_gap`)
decides `−extent − screened`, and `screened` carries
`rounding_charge(|o| + |center| + …)`. So the charge grows with how
far the carrier's origin lies from the origin of space, not with the
span. It is honest, but coarse: a gap between the span and the conic's
extent that the band allows can go unseen. The pair then routes on to
the tilt and the offset, where it may escalate or polish.

PR 5481 moved `line_line` onto `geom::one_line::on_one_line`, which
reads from the first span's start with the charge of the points it
reads. `line_conic` shares only the interval meet (`OnOneLine::of`).

## Witness first

Look for a pose this decides badly before building anything:
- a line span a metre long near `x = 10³` on a carrier whose origin
  stands `10⁶` m away along it;
- beside a conic whose extent along the line ends a small gap, say
  `1e-10`, short of the span at `ε = 1e-12`.

On main the gap's charge is about `rounding_charge(10⁶) ≈ 4·10⁻⁹`,
so the gap is not read; with the span-start anchor it is about
`7·10⁻¹²`. Then check what the unread gap actually costs: a needless
escalation or a polish, never a wrong answer, since the conic's
points are read afterwards. If nothing downstream decides differently,
say so and downgrade, as `the-curve-pair-line-line-crossing-…` was.

## The fix

Anchor at the line span's start `a₀`: the span `(0, (a₁ − a₀)·u)`, the
conic's extent about `c = (center − a₀)·u`, through `OnOneLine::of`.
Charge the points read (`a₀`, `a₁` and the conic's centre, plus `r`),
as `on_one_line` charges its four. The parallel screen keeps its own
charge.

## Bits

`line_conic`'s gap margins and its offset's evaluation point
(`a₀ + u·mid`) move. That bits move is never a reason to keep the
old anchoring: re-baseline whatever golden moves, and name it.
