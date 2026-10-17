---
id: a-tangency-at-an-edges-end-confirms-two-records
kind: issue
title: A tangency at an edge's end vertex reads as both an edge-edge meeting and a vertex-on-edge contact; which record the carry lands is undecided
status: closed
closed: 2026-10-16
opened: 2026-10-13
priority: P3
cost: M
---

Found by the round-2 review of PR 4946 (`census-curved-record-lanes-read-circles-only`, branch `fuse/census-reads-every-curved-edge`).

A design row: what `boolean::records::landings` should do with the
pair, not a defect in either reading.

Measured on PR 4946's census reading: the unit circle's arc `(0.3, π/2)`
and the line `y = 1` (or a circle touching it from outside at that end)
read `Some(true)` as an edge-edge meeting. The arc stays within ε of
the line for about 4.5e-5 m inside its end, so a point of its interior
is on both. The same contact is also the arc's end vertex resting on
the line's interior, a vertex-on-edge record. Both confirm, so the
carry can land either record, or both, for one contact.

## Closed (2026-10-16)

C5's stretch rule (`curve-pair-meetings-are-runs`, PR 5214) decides it. The touch at the arc's end is the one meeting of its stretch, and it takes the whole stretch, so no point of the arc's interior is a meeting of its own. The census confirm reads the pair as no interior meeting. `census::curved::tests::a_tangency_and_a_shallow_crossing_are_decided` pins the circle touching at the arc's end as `Some(false)`, and `a_tangent_touch_at_an_arcs_end_holds_no_interior_meeting_for_pass_five_prime` pins the line. Only the vertex-on-edge record confirms.
