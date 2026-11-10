---
id: main-red-flagged-census-reads-a-lift-site-as-an-expression-citation
kind: issue
title: Main red since ac5d2e44e2: geom-core flagged_census::every_shipped_site_cites_a_ledger_row_that_exists panics on crates/profile/src/lift.rs:286, a door's site read as citing its ledger row by an expression
status: open
opened: 2026-11-10
priority: P2
cost: E
---


Filed by the ONECOPY census lane on 2026-11-10, at the ONECOPY
orchestrator's request. **It is not ONECOPY's red**: it came in with
`ac5d2e44e2` ("Fix pass: DISCIPLINES reads 'recorded at the coincidence
door' …"), another program's coincidence-door work. It reds every branch
that merges main after that commit, in the slow set at both ε.

## What

The slow-set test `flagged_census::every_shipped_site_cites_a_ledger_row_that_exists`
(`crates/geom-core/tests/flagged_census.rs:497`) panics:

    /home/user/cad/crates/profile/src/lift.rs:286: a door's site cites its ledger row as an expression, not a string literal. The census can only verify literals; give the site its row directly, or teach this test to resolve the indirection — do not leave it unchecked.

`crates/profile/src/lift.rs:286` is the `compare(&rotated(loop_, &tangent, rotation), &replayed)`
call in the lift's verdict. Either the site gives its row as a literal, or
the census learns to resolve the indirection.

Seen on `onecopy/crease-self-touch` (merge of main at `a56c1965d4`) and
`onecopy/census-sort-home` (merge at `5471e64c1f`). Neither branch touches
`crates/profile` or `crates/geom-core/tests/flagged_census.rs`.
