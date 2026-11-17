---
id: a-clamped-view-ending-within-three-ulps-of-a-knot-holds-a-span-no-hull-reads
kind: issue
title: a clamped view of a periodic loop whose end lies one to three ulps from a knot inside it holds a span that narrow, which the certificate's tube limb can refuse
status: open
opened: 2026-11-17
priority: P3
cost: E
design: true
---


Found by the periodic-loop carrier unit
(`a-quadric-pairs-section-spline-carries-a-c0-knot-the-chord-pass-refuses`,
branch `sectarms/periodic-loop-carrier`), which was asked to handle a
clamped view that starts within rounding of a knot and say what it did.
It did nothing: the view is the exact restriction, and this row is what
that leaves.

## What

`geom::PeriodicNurbs3::clamped_view` cuts the span `[a, b]` out of the
loop by knot insertion at `a` and `b` (`NurbsCurve3::split_at`). Where
`a` lies a few ulps *under* a knot `u` of the loop, `u` is the view's
first interior knot and the first span is `u − a` wide. The insertion is
exact and the view is a valid clamped cubic with simple interior knots,
but that span's four control points differ by less than their own
rounding, so nothing read off their differences (a derivative hull, a
box orientation) means anything. The same holds at the view's end for a
knot a few ulps under `b`. An end exactly on a knot, or a few ulps past
one that then lies outside the view, has no such span.

## Measured

`geom_brep::certify_rung3` (the edge door's limbs) on the clamped view
of a whole seated turn, the seat `k` ulps under a knot, on three
cylinder × sphere poses (wall radius 0.5; ball `(d, R)` = `(0.5, 0.3)`,
`(0.2, 0.699)`, `(0.1, 0.7)`), four loops, three knots each (the 4th,
41st and 121st), at the witness band:

| `k` ulps under the knot | seats | certify | refuse `TubeProbeSilent { rungs: 20, refused: 20 }` |
|---|---|---|---|
| 0 (on the knot) | 12 | 12 | 0 |
| 1 | 12 | 7 | 5 |
| 2 | 12 | 11 | 1 |
| 3 | 12 | 11 | 1 |
| 4, 6, 8, 12, 16, 32, 64, 256, 4096, 2²⁰ | 120 | 120 | 0 |

So a first span of one to three ulps refuses limb 3 on some loops, and
one of four or more never did. The refusal is loud and typed; no wrong
answer was seen. A seat `1e-15` over the base knot (a last span about
4.5 ulps wide) certified on every pose.

## Where it can arise

An edge's span ends are foot parameters read on the loop
(`CarriedComponent::place`, `PairSection::seat`). A foot within three
ulps under a knot is a coincidence of measure about `1e-13` per site per
loop, except by construction: a site that is one of the march's own
samples (every knot of the loop is one) reads a foot within rounding of
that knot, on either side.

## The fork

Three ways to keep such a view readable, none of which the ratified text
picks:

1. **Cut the view at the knot** where its end is within a few roundings
   of it, and restate the end knot as the end asked. Built and measured
   on this branch (eight roundings: every seat above certified) and
   taken out: it names the unit roundoff outside `geom-core`, which
   `[simple.conditioned]` and `scripts/gates/bare-rounding-allowlist.sh`
   refuse, and the view is then not the loop's restriction.
2. **Decide the end at the knot** within the band (a `k_stats` reading:
   Zero cuts at the knot, a decided gap inserts, a gap in the band
   escalates). That is the near-start refusal PR 7042 removed, at every
   knot of the loop: at ε = 1e-6 a loop of 265 knots refuses about one
   site in two hundred.
3. **Read the foot at the knot** where the knot's point is itself within
   the zero of the site: both parameters are then witnesses of the site
   (`CarriedComponent::reads`' "the reading's witness"), so the choice
   refuses nothing. It moves a parameter by up to the band over the
   speed, and is a new decision site in `pair_section`.

Until one is picked the view is the exact restriction, and a span that
narrow refuses at the edge door.

`geom::curves::periodic_nurbs::tests::a_view_beside_a_knot_keeps_the_exact_insertion`
pins the insertion; `geom-brep`'s `pair_section::a_seated_loop_has_no_start`
pins that seats an ulp to each side of a knot read a simple-knotted view
of the loop.
