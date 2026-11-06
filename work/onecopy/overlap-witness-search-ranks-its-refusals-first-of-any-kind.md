---
id: overlap-witness-search-ranks-its-refusals-first-of-any-kind
kind: issue
title: boolean::overlap's witness search is a hand-rolled ladder ranking OverlapRefusal first-of-any-kind
status: open
opened: 2026-11-06
priority: P2
cost: M
---


Found by `kept-refusal-loops-the-ladder-row-never-listed`'s sweep
(`onecopy/refusal-standing`).

## What

`boolean::overlap`'s door (the `[(a, b), (b, a)]` loop over
`edge_witness`, sharing one `&mut Option<OverlapRefusal>`) is the
ladder under `Any`. The module docs say: "the door answers `true` wherever a witness decides,
and refuses only where none decides and some reading refused". It is
hand-rolled across two calls. `edge_witness`'s `keep` closure holds the
first refusal of any kind, in piece order, over both faces' pieces.

`OverlapRefusal`'s kinds are mixed:
- in band: `Senses`, `OnPlane`, and `Meeting` of an escalation;
- limits: `SplineCarrier` and a `Boundary` that does not read;
- `Placement(ContainError)`, whose standing `ContainError::refusal`
  already states (`Result<Refusal, Aborts>`).

None is ranked by `Evidence`, and a `Placement` that aborts is kept
rather than stopping the search.

## What closing it takes

1. Give `OverlapRefusal` a standing beside its variants (`ladder`'s
   module docs), delegating `Placement` to `ContainError::refusal`.
2. Port the pieces onto one `climb` (`Any`), the two faces chained, and
   the `&mut Option` threaded through `edge_witness` gone.
3. **This moves rankings**: a limit now outranks an earlier in-band
   refusal. It also moves outcomes: an aborting placement stops the
   search ahead of a later witness. Measure the reach and pin each change
   for a ruling.
