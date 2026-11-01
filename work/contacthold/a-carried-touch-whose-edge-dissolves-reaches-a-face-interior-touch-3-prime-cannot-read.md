---
id: a-carried-touch-whose-edge-dissolves-reaches-a-face-interior-touch-3-prime-cannot-read
kind: issue
title: A union then a subtract reaches a two-solid tangent touch through a face's interior: the op answers, 3′ refuses CensusUnsupported, and asking for that state directly refuses TangentSlitArmUnbuilt
status: open
opened: 2026-10-12
priority: P3
---



Found by PR 4781's second review (`contacthold/same-side-tangent-touch`).

## What

Kiss two unit discs about `(0, 0)` and `(2, 0)`, `z ∈ [0, 1]`: the
union is two solids carrying the edge–edge record of their seam edges
on the ruling `x = 1, y = 0`. Carry that record
(`ContactRecords::carried`) into a subtract of a far box
(`x ∈ [-2, -0.5]`) from the result. The left disc's seam edge on the
ruling dissolves, so its wall runs on through the line, and the touch
is now a two-solid tangent touch through a face's interior.

- The op returns `Ok`.
- 3′ on its records refuses `CensusUnsupported` ("no curve-touch lane
  along an edge's carrier").
- Asking for the same state directly (the cut disc unioned with the
  right disc) refuses `TangentSlitArmUnbuilt` at the op
  (`BooleanError::TangentSlitArmUnbuilt`, `crates/topo/README.md` C7).

Reproducer: `crates/sweep/tests/review_4781_r2_probes.rs::review_4781_r2_record_survives`
(ignored, prints; the `∖ far box` step).

## Question

Two doors disagree on one state: one answers and leaves the result 3′
cannot read, the other refuses up front. Is the gap the census's (a
curve-touch lane for an edge lying along a face's interior, which the
face–face row C7 names would also need) or the carry's (the op should
refuse, or keep a vertex on the line, when a carried edge–edge record's
edge dissolves into a face)? Measure which before choosing; both may be
owed.

## 2026-10-12 — the carried route refuses at the op (CONTACTHOLD)

`contacthold/face-face-touch-row` retires `TangentSlitArmUnbuilt` (the
reduction mints a face–face row where the glue door's tangent line runs
through both faces' interiors) and gives edge–face and face–face
records a carried kind. Re-run, `review_4781_r2_record_survives`'s
`∖ far box` step now refuses at the op:

```
CarryRefused { row: Carried(A), refusal: Unread { record: 2, cells: [Edge(EdgeKey(10v1)), Face(FaceKey(4v1))] } }
```

The edge–edge record's left edge dissolved into the wall, so the record
lands on the right disc's seam edge against the left disc's wall face,
an edge–face pair the census's confirm has no lane for
(`work/restread/an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane.md`).
The op no longer answers a result 3′ cannot read; it refuses the record
it cannot place. The direct route (the cut disc unioned with the right
disc) was not re-run. What remains is that lane.
