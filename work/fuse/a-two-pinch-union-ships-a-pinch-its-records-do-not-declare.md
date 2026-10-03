---
id: a-two-pinch-union-ships-a-pinch-its-records-do-not-declare
kind: issue
title: A union that welds two pinches ships a touching its contact records do not declare, so the tier-3′ pass refuses it UndeclaredContact
status: open
opened: 2026-10-02
priority: P2
cost: M
---


Found by the result-gate unit
(`a-boolean-result-gate-ships-a-scaffold-at-rest`), measuring
`validate_pseudomanifold(&body, &contacts)` on every result the door
builds, at the default ε on the `ci` profile.

## What

16 results of `editor-core`'s
`union_pinch_member_order::two_pinches_build_one_body_in_every_member_order`
(12 seamed, 4 through the fallback's two-operand arm) carry non-empty
`contacts`, so `BooleanBody`'s doc makes them tier-3′ currency. Each
passes tier 3 and fails the census with `UndeclaredContact`: the result
touches itself somewhere its records do not name. The records declare
one pinch and the body holds a touching beside it.

Results with empty `contacts` that touch themselves (43 in
`editor-core`, 2 in `topo`'s `m3_pr6_tier3prime::closure_kiss_vs_mover`)
are not this item: the doc makes them tier-3 currency, which runs no
census. Whether it should is Ev's question on `[ev]` PR 3870.

## The fix

Find which touching goes unrecorded (the census names it) and record
it where the door makes it, or refuse there. Gating tier 3′ at the door
refuses all 16 until this lands.
