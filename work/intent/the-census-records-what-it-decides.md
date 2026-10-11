---
id: the-census-records-what-it-decides
kind: unit
title: D10 stage 4 PR J0: the at-rest census records what it decides between groups (a row of its own Zero and a record citing it); import's declaration channel retires
status: closed
priority: P0
cost: H
parent: an-unattributed-contact-at-rest-is-a-finding
opened: 2026-10-11
closed: 2026-10-11
---

Cut out of J (`an-unattributed-contact-at-rest-is-a-finding`) by the
orchestrator on 2026-10-11, after a designer pair converged on the
fork J's lane raised: a census contact that becomes a row with no
record is found again by every later census over the body, where it
sits inside one copy and refuses (a nested assembly at its parent, an
imported assembly at its own gate).

The ruling:

- **What a contact found at rest is.** A `CensusAtRest` row (the
  finding, judged at the door, reported by the lint) and a contact
  record on the result citing it. A row with no record violates D1
  (ii); the record is what lets nesting confirm the touch rather than
  rediscover it.
- **One kernel door.** The tier-3′ census, given the body, its records
  and the caller's partition of its solids into groups, returns new
  records, `CensusAtRest` rows and findings. A touch no record backs
  between two groups is a row carrying the Zero's own `MarginDiag` (D10
  forbids a synthetic one) and the record of that pair citing it. A
  touch inside one group is that group's defect and refuses typed (D1
  (iii)). The kernel never learns what a group means. Callers: the
  product gather (groups are copies; J), import's aggregate gate
  (groups are the file's instances), an op's finished-body gate (one
  group).
- **This unit owns** the census pushes surfacing `MarginDiag`,
  `UndeclaredContact`'s retirement at the kernel (it is
  `UnrecordedContact`, the defect inside one group), `StepImport::Solid`
  gaining `contacts` (the resolved records were dropped, and pncad-py
  wrapped `Body::plain`), and `declared_contacts`, `ImportContact`,
  `resolve_declarations` and `DecisionSite::ImportAnchor` retiring.

## Closed

`topo::record_at_rest` is the door: it runs the caller's census, mints
each touch between groups as a `CensusAtRest` row and a record citing
it, and reruns until no new touch appears (at most three passes). An
edge lying in a face gets no record of its own (the census confirms one
on no lane); its bounds' records back it on the next pass, as D1 says.
`ValidationError::UnrecordedContact` carries `margin`, the Zero the
census decided (`None` for a pierce and a same-key conformal patch).
Import records the kiss assembly's corner as one vertex–vertex record
and ships it on `StepImport::Solid::contacts`.
