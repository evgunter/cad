---
id: the-at-rest-face-face-confirm-bridges-an-in-band-relative-curvature
kind: issue
title: The at-rest face–face confirm reads the tangent table declared, so an in-band κ_rel that no op decided passes
status: closed
opened: 2026-10-19
closed: 2026-10-30
priority: P2
cost: E
refs: [the-census-discovers-no-same-solid-tangent-touch-through-face-interiors]
pr: 6079
branch: contacthold/glued-pair-reads-its-record
---


Found by both designers of the pinch-crescent fork (`[ev]` branch
`contacthold/ev-pinch-crescent`), off its question; both call it a
defect.

## What

`boolean::tangent_table_at_rest` (`crates/topo/src/boolean/mod.rs:6870`)
reads a face–face
record's tangent table through
`contact_verify::tangent_locus_relation` with `declared = true`
(`mod.rs:6885`): "the declared posture, the record standing as the
declaration". In that posture the second-order margin, the relative
transverse curvature κ_rel, is bridged where it lies in band
(`crates/topo/src/boolean/contact_verify.rs:33`, `:234`).

A record is not a declaration. It cites a decision an op made
definitely at its glue door, which reads the same table undeclared
(`tangency_reading`, C4/C7). So at rest a record can stand on a
tangency no op could have decided: a κ_rel in band confirms instead of
escalating.

## Owed

The at-rest confirm reads the table undeclared, as the glue door does:
an in-band κ_rel escalates (`CensusEscalated`), and a record bridges
nothing. A probe: a face–face record whose two carriers' κ_rel sits
inside the band confirms today and should escalate.

## Closed

Closed by PR 6079: contact_verify takes no `declared` flag; an in-band κ_rel escalates (`exact_kappa_zero_escalates`).
