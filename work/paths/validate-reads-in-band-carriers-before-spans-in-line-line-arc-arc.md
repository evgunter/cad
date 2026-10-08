---
id: validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc
kind: issue
title: The simplicity pair pass still escalates an in-band carrier reading whose contact lies off a segment: line x line endpoint sides, line x circle off the line's span, circle x circle
status: open
opened: 2026-10-07
priority: P2
cost: M
---


Found by the class sweep of
`a-straight-arrival-off-an-arc-departure-escalates-in-carrier-line-circle`,
which fixed two instances in `crates/profile/src/seg.rs`: `line_arc`
now asks `arc_clear_of_carrier` before an in-band
`carrier_line_circle` escalates, and `joint` answers "no contact" on
either span's definite miss of a secant candidate before the other's
in-band reading escalates.

## The shape

The simplicity pair pass (`seg::pair_contacts`, called by
`validate.rs` `judge_pair`) decides a CARRIER question first and
escalates on an in-band verdict before asking whether the contact it is
about lies on both segments. Each sibling below is measured with a
scratch probe at the default ε, every margin placed at
`ε·(1 + k)/2`, and each refuses `Escalated` although the two segments
are at least 1/3 apart:

| classifier | fixture | refusal |
|---|---|---|
| `line_line` | `chain` (0,0) (2,0) (2,2) (4,2) (4,b) (6,−1) (6,3) (0,3): `(4, b)` in band of segment 0's carrier, 2 past its end | `SegmentPair(0, 3)`, `chord_side` 5.5e-9 |
| `line_arc`, foot off the LINE span | `chain` (−1,0) (1,0) (1,−1) (5,−1) (5,5) (3,5) (3,3, −(3 − b)) (1,3) (−1,3): the major arc's carrier band above y = 0 at x = 2, the line ending at x = 1, 1/3 below the arc | `SegmentPair(0, 6)`, `carrier_line_circle` −5.5e-9 |
| `arc_arc` | two half-discs, `chain` (0,1, 1) (0,−1, 0) and (2+b,−1, 0) (2+b,1, −1): circles externally in band at (1, 0), each arc on its far side | `SegmentPair((0,0), (1,1))`, `carrier_circles_external` 5.5e-9 |

The fixed `line_arc` certificate reads the arc's side only: the arc's
span definitely excludes the circle's point facing the line and both
arc endpoints lie definitely on the centre's side. The line's side
(foot definitely off the line's span and the line's nearer endpoint
definitely outside the circle) needs a radial margin no predicate in
the inventory names; `arc_arc` needs the same per arc against the other
circle; `line_line` needs the endpoint's distance along the other
carrier.

## The tangent arms: measured, and split out

`line_arc`'s decided-Zero arm, and `arc_arc`'s two tangent arms, took
one candidate at the tangency point and span-checked it alone. That
silent miss was filed and fixed on its own, at P0:
`validate-settles-a-tangent-pair-on-one-candidate-and-misses-a-touch-within-eps`.

A candidate that a span definitely misses now leaves the pair's ends to
read (`seg::end_touches`). This holds in the tangent arms and in the
secant arms, where a shallow crossing had the same hole. That fix does
not touch the siblings above: each still escalates on an in-band
carrier reading before any candidate is formed.
