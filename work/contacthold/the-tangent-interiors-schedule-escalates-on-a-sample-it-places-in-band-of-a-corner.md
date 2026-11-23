---
id: the-tangent-interiors-schedule-escalates-on-a-sample-it-places-in-band-of-a-corner
kind: issue
title: The glue door's tangent-interiors schedule escalates on a sample it places in band of a face corner
status: closed
branch: contacthold/one-tangent-interiors-reading
pr: 5006
opened: 2026-10-13
closed: 2026-10-19
---


## What

`crates/topo/src/boolean/mod.rs` `locus_through_both_faces` (landed in
PR 4921) asks whether a verified tangent line runs through both faces'
interiors by sampling the line at `geom_brep::sample_param` over the
overlap `witness_span` gives (the projections of the two faces'
boundary vertices), and asking `contain::face_placement` of each
sample. The first sample whose placement escalates is returned as the
refusal, before any later sample is read.

The sliver family (`crates/sweep/tests/review_4831_probes.rs`
`p10_the_sliver_family`) shows it. The fat disc is `h` thick and the
tangent ruling through (2, 0) runs along its wall's seam edge over
z ∈ [0, h]. The schedule's second station sits `h/8` from the fat
disc's bottom corner (2, 0, 0). For `h/8` in band, the boundary
pre-pass escalates on `bool_contact_vertex`
(`Containment { decision: Some(Loop(Boundary)), escalation: Margin }`).
Every op whose first operand is the fat disc then refuses `Escalated`.
The other order answers, because the thin disc's face is read first
and answers false. Measured at ε 1e-6 (h = 3e-5, margin 3.75e-6) and
1e-7 (h = 3e-6, margin 3.75e-7). Before PR 4921 the same poses answered
at their closed forms.

## Why it is a defect

The geometry has nothing in band. The plate is 30 ε thick and every
feature is at least that far apart. The point in band of the corner is
one the door placed there itself. Two other facts make the escalation
unnecessary:

- The line runs along the fat disc's boundary edge for its whole length,
  so no sample can be strictly inside that face.
- A sample within the escalation width of a face corner can never read
  `In` definitely. Its distance to the boundary is at most its distance
  to the corner.

The answer also depends on operand order. That is a symptom: the
`&&` reads `faces[0]` first.

## Directions

- Skip schedule stations within the escalation width of either face's
  boundary vertices. They cannot answer `In`.
- Or read on past an escalated station, and escalate only when no
  station answers `true` and the escalated one could.

Pinned in p10 (`sliver_refusal`, the `h/8` window) until fixed. The
fix flips that window's rows to answers, which turns the pin red.
