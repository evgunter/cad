---
id: a-walk-refusals-standing-lives-on-the-ladder-not-its-type
kind: issue
title: A spline walk refusal's standing is Refusal::of_walk in geom_brep, not a method on geom::PieceWalkRefusal
status: open
opened: 2026-11-09
---


`geom_brep::ladder::Refusal::of_walk` is the one table of a spline walk refusal's standing: `Exhausted` is unsettled, `Unbounded` is a limit. Every wrapper that carries a `geom::PieceWalkRefusal` delegates to it:
- `curve_pair::MeetFault::Unwalked`;
- `topo` `validate::EdgeTangentUnread::Unwalked`, and through it `link::TouchUnread::EdgeTangent`;
- `curve_face::BoundaryRefusal::Unread`, through its `MeetFault`.

It lives on the ladder only because `geom` cannot see `ladder::Refusal` (its own doc says so). Once the ladder lives in `geom_core`, the table becomes `PieceWalkRefusal::refusal()` in `geom`, beside its `sentence()` and `recourse()`. The callers then read `why.refusal()`.

A carrier refusal's whole standing is `escalation()`, then `Margin` or the walk's. It needs no method of its own while no wrapper carries a whole `CarrierRefusal` into a ladder. `SpanRefusal` carries one, but nothing ranks it.

Open this once the `geom_core::ladder` move has landed.
