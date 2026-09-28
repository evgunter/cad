---
id: cone-offset-slide-and-shift-cancel-at-a-narrow-cone
kind: issue
title: ConeOffset's apex slide and v-shift cancel as 1/sin α: a narrow cone's moved rim station carries (|d|/sin α)·u of rounding
status: open
opened: 2026-09-28
priority: P3
cost: E
refs: [c5-gate-admits-every-pose-of-an-implemented-pair]
---

## What

Found by GERM's aperture-division sweep (the C5 pose-gate unit).
**Unmeasured — analysis only.**

`geom_brep::ConeOffset` (`crates/geom-brep/src/offset.rs`) divides by
`sin α` twice: `apex()` slides the apex by `d / sin α`, and `shift()`
moves `v` by `d·cos α / sin α`. Neither division is unguarded in the
sense the section arms guard theirs: `α` is a stored datum that tier-3
check 1 holds strictly inside `(0, π/2)` at rest, and `sin α` of a
stored `α` is relatively exact, so each quotient is correctly rounded.
The sweep's disposition was therefore "no divisor guard owed".

What is left is a CANCELLATION one step later. A parallel re-minted at
`v + shift` about the slid apex (`transport_curve`'s cone arm,
`crates/topo/src/replace_face.rs`) stands at axial station
`apex − d/sin α + (v + d·cot α)·cos α`, and the two `O(d/sin α)` terms
cancel to `v·cos α − d·sin α`. Each carries rounding of order
`(|d|/sin α)·u`, so at `α = 1e-8`, `d = 0.1` the station is uncertain
by about `1e-9` m — the witness ε — and it grows as `α` closes. The
minted circle is still ON the minted cone (its radius and centre come
from the same `v`); what moves is WHICH parallel it is.

## Why it is not a wrong answer today

The error lands on a rim vertex that must still stand on an untouched
neighbour, and the per-chart door's re-anchor gate
(`ReanchorOffCarrier`) and the attach layer's certification measure
exactly that residual. So a narrow enough cone refuses loudly at a
gap that is rounding rather than geometry — a spurious refusal, never
a silent one.

## What would settle it

Measure first: offset the cone face of a frustum at `α ∈ {1e-4, 1e-6,
1e-8}` through `replace_face_offset` and read the re-anchor gap
against `d·sin α`'s closed form. If the rounding term shows, the fix is
to compose the rim station directly (`v·cos α − d·sin α` about the old
apex) rather than through the slid apex.

