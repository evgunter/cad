---
id: the-cap-sliver-screen-reads-an-exact-zero-where-a-lune-slits-wall-ends-at-its-arcs-centre
kind: issue
title: the cap-sliver screen refuses a lune slit's fillet from r = 2/3, reading an exact zero where the wall ends at its arc's centre
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Found in the knife-edge fillet unit (PR 6510); both of its reviewers measured it.

## What

A plate bored by the lune of `sweep::test_support::lune` (inside `(0,2) r 2`, outside `(0,1) r 1`, closed by the `x = 0` wall from `(0,4)` down to `(0,2)`) has a slit on the kiss. The fillet builds that slit through r = 0.666 and refuses from r = 2/3 (0.6667, 0.67, 0.8, 0.95) with `RingClearance { bounded: true }`, reading about ±1e-16 and deciding Zero. The band is clear of the wall at every one of those radii. The cusp built from the same lune builds at 0.95. Pinned in `crates/sweep/tests/a_knife_edge_fillet_builds_where_its_ball_is_defined.rs`, `the_lune_cusp_builds_to_its_room_and_the_slit_stops_at_two_thirds`.

## Where the zero comes from

The refusal is the cut-off's cap-sliver screen: `crates/sweep/src/blend/surgery.rs`, the end-face cycle pass, at `s.clearance(carrier, window)` near line 2990. It meters each end-face edge against the region `Ω` that `CapSliver` proves encloses the sliver (`crates/sweep/src/blend/open/end_face.rs`, `whole_clearance` and `line_clearance`).

One of `Ω`'s floors is the half-plane along the section's second axis, the unit ⊥ `u_ref`. Its boundary is the line through the ball's centre and its foot on the big arc. That line passes through the big arc's centre `(0, 2)` at every radius, because the foot lies on the line of centres. The wall edge ends exactly at `(0, 2)`. So that floor's term is `floor − high = 0 − 0` on the wall at every r, in exact arithmetic.

An instrumented run at r = 0.7 read `floor 1.4e-17`, `high −2.8e-17`, `term 4.2e-17` on the edge `x = 0, y ∈ [2, 4]`. Every other term was negative: the disc reach −1.17, the `toward` floor −1.18, and the other axis floors −0.31, −2.0 and −3.99. Below 2/3 another term clears the edge; from 2/3 on, none does. The screen's max is then the exact-zero term, which classifies Zero, and the cycle reads as a bound.

## Why it is not fixed in place

The floor directions are general and correct. The zero is where the fixture's wall ends, on the line one floor is drawn along. Clearing the edge needs a term that sees the sliver's real extent past that line, for example the exact triangle or sliver reading `Exact` already holds for "inside" and "crossing", used for "outside" too. That is the shape `a-chord-ends-triangle-could-read-a-cycle-outside` names on a chord end. It is a change to the screen's three-way verdict, not to the knife fillet.

P2: a correct shape refused, conservatively and with a bound. Nothing wrong is built.
