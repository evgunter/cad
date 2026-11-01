---
id: c9-ring-class-phrase-names-a-retired-ring-or-the-exclusion-ring
kind: issue
title: The C9-ring conformal-rest class phrase at four sites: certification arithmetic, or the exclusion ring the same sentence names?
status: closed
opened: 2026-09-29
priority: P4
cost: E
closed: 2026-10-26
---


## What

Four sites name the class the census's proximity backstop refuses as
"the C9-ring conformal-rest / partial-embedding class":

- `crates/topo/src/census.rs` (CONTACT), the undecidable arms' doc,
  item 1 **Proximity**;
- `crates/topo/src/validate.rs` (RESTFRONT), `ValidationError::CensusUndecidable`'s
  doc ("the C9-ring conformal-rest/proximity class — the exclusion ring
  is the certified excluder this backstop stands in for");
- `crates/topo/src/instance.rs` (unowned), the module doc;
- `crates/sweep/tests/m5_pr9_boss_union.rs` (TCOST/TINT),
  `r1_probe_conformal_touch_between_instances_refuses_undecidable`'s
  doc and body comment ("naming the C9-ring class").

"C9 ring" was the name of the certification scalar (`RingInterval`)
until RING-3 dissolved it into `Interval`; C9 now says "certification
arithmetic". But validate.rs's sentence also names "the exclusion
ring", the certified excluder the backstop stands in for, so "C9-ring
class" may mean either the class whose certificate would live in
certification arithmetic, or the exclusion-ring class. None of the
four is runtime text and no row asserts the phrase.

## Proposed

CONTACT (and TOPO for `instance.rs`) says which it means. If the
former, re-word all four to "certification-arithmetic (C9)" as
`edge_nurbs.rs` and `validate.rs`'s plane × NURBS comment now say; if
the latter, drop "C9-" so the phrase names the exclusion ring alone.

## Found by

SCALAR-HYGIENE (`ring-3-residue-outside-its-fence`), 2026-09-29.

## Closed (2026-10-26, tracker sweep)

**Fixed in the sweep (prose only).** `crates/topo/README.md` answers the row's question. The C9 exclusion ring is the certified excluder by name: "that discovery is the C9 exclusion ring for same-solid pairs" (`:133`), and "the C9 exclusion ring for same-solid distinct-key curved pairs is built for the closed-form tangent line only" (`:1116`). So "the C9-ring class" meant the class that excluder would certify, not certification arithmetic. All five spellings now name it that way:
- `crates/topo/src/census.rs:8413`: "(the conformal-rest / partial-embedding class the C9 exclusion ring would certify)";
- `crates/topo/src/validate.rs:1931`: "the conformal-rest/proximity class — the C9 exclusion ring is the certified excluder …";
- `crates/topo/src/instance.rs:62`: "(the conformal-rest / partial-embedding class the C9 exclusion ring would certify)";
- `crates/sweep/tests/m5_pr9_boss_union.rs:220` (doc) and `:271` (comment): "the conformal-rest class …".

The edits are comments only, and no row asserts the phrase. The `design` flag is answered by the README.
