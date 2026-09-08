---
id: two-validation-payload-discriminants-still-uncrossed
kind: issue
title: two ValidationError payload discriminants still cross as prose only
status: open
opened: 2026-09-08
refs: [LIB-FINDINGS]
needs_ev: true
---

Disclosed by LIB-FINDINGS, which crossed the two payloads Ev's
ruling named and not the other two. Filed as its own file because a
residue disclosed only in a closed item's prose is invisible to the
re-homing sweep.

## The measurement

`ValidationError.findings` now carries, per failure, the arm's word
plus `CensusContact` as `contact_kind` and `CensusSubject` as
`subject_kind`/`entity_kind`. Two payload discriminants of the same
enum did NOT move and are `INTERIOR` in the binding census with the
reason stated at each entry
(`crates/pncad-py/tests/test_binding_census.py`):

- **`StaleDeclaration`** — `ValidationError::StaleContactDeclaration`'s
  payload, which record lost its witness. Four arms
  (`crates/topo/src/validate.rs`), and which one it is decides which
  record a caller withdraws.
- **`RingContact`** — `ValidationError::RingMeetsOuter`'s payload:
  vertex-on-vertex, vertex-on-edge, or edge-along-edge. Which one it
  is decides where the ring has to move.

From Python each arrives as `variant == "stale_contact_declaration"`
or `"ring_meets_outer"` and prose, which is the discrimination the
words exist to spare a caller from parsing — the same measurement
`census-findings-cross-without-a-per-arm-tag` made about
`CensusContact` before LIB-FINDINGS closed it.

## Why LIB-FINDINGS did not take them

Ev's ruling (2026-09-08, `[ev]` PR 2196) names the payload the door
carries: "`CensusSubject` as `subject_kind` plus the entity kind or
the pair", and "`CensusContact` and `CensusSubject` leave `INTERIOR`
together". Two types, named. A unit that crossed four would be
answering a question that was asked about two, and the shape is not
free: each new word is public Python vocabulary and a census row.

## What a unit closing it would decide

1. Whether the finding's attribute list grows per payload type
   (`stale_kind`, `ring_contact_kind`) or whether the arm-specific
   payloads share one attribute — the second is cheaper on the class
   and worse on the reader, since one word would mean different
   things under different `variant`s.
2. Whether the projection stops at the discriminant here as it does
   everywhere else, given neither payload's FIELDS can cross: both are
   arena keys, and no key crosses to a surface that holds names.

Adjacent but not this: `pncad-py-seven-doors-lack-field-projection`
asks about the six other doors' payload ATTRIBUTES. This asks about
two payloads of the door LIB-FINDINGS just projected.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

Your ruling on `census-findings-cross-without-a-per-arm-tag` named two
payload types (`CensusSubject`, `CensusContact`) and LIB-FINDINGS
crossed exactly those two. The same door has two more payload
discriminants (`StaleDeclaration`, 4 arms — which record lost its
witness; `RingContact`, 3 arms — where the ring has to move), and
`witness-bifurcation-arm-has-no-inner-word` is the same question one
door over. Does the ruling generalise, and in what shape?

- **(A) Generalise: every payload DISCRIMINANT of a projected refusal
  crosses as an attribute of its own, named per type (`stale_kind`,
  `ring_contact_kind`), `None` on every other arm** — one attribute per
  concept, never one word meaning different things under different
  `variant`s. Fields that are arena keys still do not cross (no key
  crosses to a surface that holds names), so the projection stops at
  the discriminant, as it does everywhere else. Mechanical afterwards
  (two exhaustive maps, two attributes, census rows), and it settles
  the bifurcation arm too: it crosses the day the M6 solver constructs
  it. Recommended.
- **(B) Cross only what a caller has asked for** — the two named types
  stay the whole answer until a caller needs `stale_kind`. Cheaper on
  public vocabulary; keeps the Rust/Python asymmetry the census rows
  record, and leaves the next lane to ask again.
- **(C) One shared `payload_kind` attribute** whose meaning depends on
  `variant`. Cheapest on the class and worst on the reader.

Recommendation: **(A)**. The per-arm rule you licensed says a
discriminant a caller can act on crosses beside the carrier's word;
there is no reason in the door's shape for two of four to stay prose,
and the census has already learned to measure the move.
