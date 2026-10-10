---
id: zero-to-distinct-cells-on-one-point-doors-audit
kind: issue
title: Audit the doors other than the boolean that turn a Zero into distinct cells on one point (e.g. a revolve whose profile meets its axis twice): does each record the coincidence it decides?
status: open
opened: 2026-10-10
priority: P2
cost: M
---


Split off `boolean-vertex-contact-records-are-inferred-from-values`
(closed 2026-10-10), whose "Owed" item 3 this is. That row is answered
for the boolean: every `ContactRecords` row cites the `Coincidence`
that decided it (`boolean/reduce.rs::push_vv` / `push_vf` →
`boolean/ops.rs::cite_rows`, `DecisionSite::VertexFusion`), and the
`unproven-coincidence` lint walks those coincidences.

## The question

Which other doors decide a Zero and leave two distinct cells standing
on one point, rather than fusing them? For each, does the door record
that decision as a `Coincidence` (D10: a Zero coincidence is recorded
at the one door and linted), or does a value coincidence survive into
the result with nothing citing it?

Candidate named by the parent row: a revolve whose profile meets its
axis twice (two poles on one axis; a profile vertex on the axis whose
image is one point for two profile cells). Other candidates to check:
the sweep doors' pole and seam handling, and any construction that
keeps copies (the boolean's `copies_of` is the boolean's own case).

## Owed

1. List the doors, by name, that can produce distinct cells at one
   point from a Zero verdict.
2. For each: recorded and cited, fused, or uncited. File each uncited
   one on its owner's slate.
