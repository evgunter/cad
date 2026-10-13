---
id: a-tangency-at-an-edges-end-confirms-two-records
kind: issue
title: A tangency at an edge's end vertex reads as both an edge-edge meeting and a vertex-on-edge contact; which record the carry lands is undecided
status: open
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
