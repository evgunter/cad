---
id: a-creased-spline-gets-no-second-order-clearing
kind: issue
title: A spline with a crease gets no second-order clearing and no tangency floor, so a touch or meeting on it walks to the band
status: parked
opened: 2026-10-26
priority: P1
cost: M
blocked_on: [the-crease-arms-retire-once-no-carrier-carries-a-c0-knot]
---

Found by the review of PR 5715 (branch `splice/touch-doors`), which
made `geom::NurbsCurve3::accel_upper_bound` infinite (poison at
`Interval`) wherever `KnotStructure::crease_runs` is non-empty: an
interior knot of multiplicity at least the degree, where `C′` may
jump. That is sound. Without it, a cornered degree-1 net read zero
curvature and two doors cleared past the corner
(`carrier_touch_rows::a_spline_with_a_corner_crossing_the_face_is_not_off_it`,
`curve_pair::meet_rows::a_spline_with_a_corner_meets_a_line_past_it`).

**What it costs.** A creased spline now gets neither:
- the second-order clearing, in `boolean::carrier_touch::clusters`
  and `curve_pair::walk::pieces_apart`; nor
- the walk's tangency floor (`curve_pair::walk::floor`).

So a tangency on it is walked down to the band. The same PR measured
the first-order walk alone exhausting `geom_core::bisect::PIECE_BUDGET`
at ε = 1e-12 on an uncreased spline touch. Expect that refusal back for
a creased spline's touch or tangent meeting once the operand gate is
gone. That covers every multi-span degree-1 net, any net with a
`degree`-fold knot, and the nine-point NURBS circle. The refusal is a
door that stays, never a wrong answer.

**What a fix needs.** One of:
- a bound scoped to a parameter window, finite when the window holds
  no crease, which each consumer reads for its own piece; or
- piece walks that split at the creases first and read a per-span
  bound.

Either way the bound has to stay unusable across a crease. That is why
the PR made the whole-domain bound non-finite rather than per-span.

**Measured on today's demos (2026-10-26):** neither track witness is
creased.
- The klein loop's lofted body carries four NURBS edges, all degree 2
  with interior multiplicities 1.
- The lily's lofted long leaf and the three sepals (the sheath) each
  carry two NURBS edges, degree 3 with interior multiplicities 1.

Probe: each body's NURBS edges read through `knot_values().structure()`.
So those walls keep the second-order bounds. Creased edges arrive with
a degree-1 or knot-repeating section or loft.


Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

Moved from SPLICEFACE to SPLICE on 2026-11-02, by `git mv`, when SPLICEFACE merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.

## Parked on the crease arms' retirement (SPLICE, 2026-11-03)

The `splice/crease-window-bound` lane measured the premise on main `d1ac6881`, and it holds:
- `carrier_touch::clusters` / `off_face`, on a cylinder quarter sheet touched between creases by a degree-1 three-span net and by the nine-point circle: right at ε 1e-9 and 1e-6 from the first-order bound alone; at 1e-12 both exhaust `PIECE_BUDGET` (`clusters = None`, `off_face = Ok(false)`, so the door stays shut).
- `curve_pair::walk::meet`, net × circle and nine-point circle × line, both tangent: `MeetFault::Unsettled(Exhausted)` at all three ε, with the touch already in `found`.

Every answer is a refusal, never a wrong one. It is parked rather than built, because DESIGN [topology] (Ev, `[ev]` PR 5913, Decision 2) makes every face chart and edge carrier C¹ strictly inside its cell, with the kernel cutting where one is not. A window-scoped bound, or walks split at the creases, would exist only to serve carriers the ratified design removes at rest. `tesssize/the-crease-arms-retire-once-no-carrier-carries-a-c0-knot` names this row as one that becomes unreachable, and says of crease arms "delete it, do not generalise it". That row closes this one.
