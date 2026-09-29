---
id: carrier-eq-source-rung-reads-orientation-its-own-way
kind: issue
title: boolean::carrier_eq::source_rung (curved rung 1) classifies a source pair by same_base plus outward bits, beside source_declaration's orient-based ladder
status: dispatched
opened: 2026-09-29
priority: P4
cost: E
refs: [three-spellings-of-one-chart-answer-the-same-question-differently]
branch: origin/curve-walk
---


Disclosed by PR 3429's fix pass, filed so it is scheduled.
`crates/topo/src/boolean/carrier_eq.rs` `source_rung` (curved rung 1)
decides "one declared surface" by `same_base` and takes orientation
from the faces' `outward` bits, not from `GeomSource.orient`; the
planar rung 1 (`plane_eq`) and the merge door now go through
`source::source_declaration`. Only the `same_base` half routes as is.
Decide whether the outward-bit orientation is the same fact as
`orient` composed with face sense (then route it) or a different one
(then say so at the site).
