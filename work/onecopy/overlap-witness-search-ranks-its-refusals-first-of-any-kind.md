---
id: overlap-witness-search-ranks-its-refusals-first-of-any-kind
kind: issue
title: boolean::overlap's witness search is a hand-rolled ladder ranking OverlapRefusal first-of-any-kind
status: closed
opened: 2026-11-06
priority: P2
cost: M
closed: 2026-11-06
pr: 6553
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

`OverlapRefusal`'s kinds are mixed, and the door meets them two ways.
- **Kept by `edge_witness`'s `keep`, first-of-any-kind:**
  - `Meeting`, from a located edge's fault: in band where it is an
    escalation;
  - a cut sort's refusal (`sort_params`): a `Meeting` escalation;
  - `Placement(ContainError)`, whose standing `ContainError::refusal`
    already states (`Result<Refusal, Aborts>`);
  - the rest of `place`'s refusals (`OnPlane`, and `Unread` and
    `StaleFace` from its own reads) and every refusal of
    `piece_witness` (`Corner`, `Normal`, `Unread`).
- **Returned at once by `?`, ahead of any later witness:**
  - `StaleFace` and `SplineCarrier`, in the door's preamble;
  - `OppositeSenses` and `Senses`, from `one_sense`;
  - `Boundary` and `Unread`, from `edge_witness`'s reading of `y`'s
    boundary (`face_boundary(…)?`, and the lone-vertex and
    uncertified-curve refusals);
  - the `loops(x)?` and `edge_span(…)?` reads.

None of the kept ones is ranked by `Evidence`, and a `Placement` that
aborts is kept rather than stopping the search. A `?` refusal in the
second list stops the search ahead of a witness the other face could
hold. Those that read the whole query (a stale face, a spline carrier,
opposed senses) are aborts by right. So is `Senses` in band: one sense
is the door's premise, which every later reading needs, so its early
return is right.

## What closing it takes

1. Give `OverlapRefusal` a standing beside its variants (`ladder`'s
   module docs), delegating `Placement` to `ContainError::refusal`.
2. Port the pieces onto one `climb` (`Any`), the two faces chained, and
   the `&mut Option` threaded through `edge_witness` gone.
3. **This moves rankings**: a limit now outranks an earlier in-band
   refusal. It also moves outcomes: an aborting placement stops the
   search ahead of a later witness. Measure the reach and pin each change
   for a ruling.
