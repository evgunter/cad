---
id: contact-records-carry-operand-labels-into-the-at-rest-currency
kind: issue
title: ContactRecords' operand-labelled lists (a_on_b, b_on_a; VvContact a/b as A-clone/B-clone keys) are reduction vocabulary used as the at-rest declaration type
status: closed
opened: 2026-10-02
priority: P3
cost: M
refs: [3856]
rides_with: at-rest-records-are-unordered-cell-pairs
closed: 2026-10-11
---


## What

`ContactRecords` (`crates/topo/src/boolean/mod.rs`) labels its lists by operand. At rest the labels mean nothing: `census`'s `Declared::index` chains `a_on_b` with `b_on_a`, as do `product::carry_contacts` and editor-core's `checks.rs`; step-import and assembly fill one list arbitrarily; and a split result has no A or B at all. One type means two things. Both designers recommend a symmetric at-rest record type (`vv`, `vf`, `curves`, `patches`) in the contact layer, with the A/B lists kept on `BooleanReduction` where `shell_witness` and `coplanar_conic_rows` read them.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## Parked on the D10 hold (2026-10-08)

`ContactRecords` is the record the one recording door replaces. (CONTACT close-out triage; CONTACT's log (`docs/doc-ledger/contact-leaves-the-tracker.md` names the SHA it is read at).)

## Re-parked on B2 (2026-10-10)

Its trigger, stage 4 B (`coincidences-are-recorded-at-one-door`, PR
4354), fired, but B left `ContactRecords` as it was (the S4-B ruling,
option (b)); the record this row is about is rewritten by B2
(`contact-records-cite-their-decision`, live on
`intent/s4-b2-records-cite`), which makes every row cite its
`Coincidence` across ~99 files. Building on the record's shape while
B2 rewrites it would collide and be built twice. Re-read it against
B2's merged record. (CONTACTHOLD orchestrator)

## Ruled (Ev, PR 4547, 2026-10-10)

The at-rest record is an unordered pair of the body's own cells plus
its citations, in one list a citation addresses by position. It stores
no operand label, no granularity and no witness, and every pair whose
interiors meet is recorded (a straight edge in a plane included).
Fork log row 108; D1 and C3 state it. What's left is the build. The
boolean's reduction keeps its operand-tagged working set and drops the
tags at carry. (CONTACTHOLD orchestrator)

## Closes with the unit (2026-10-10)

Built by `at-rest-records-are-unordered-cell-pairs` (PR 4598 and its
stacked successor): `CurveContact` and the operand-labelled lists are
gone from the at-rest record; the reduction's operand-tagged set is
`ReductionContacts`. This row closes when that stack merges.
