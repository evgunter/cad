---
id: restfront
kind: program
title: RESTFRONT — the at-rest validator's not-yet-checked list, and what its checks cost
status: ready
opened: 2026-09-27
area: kernel
prefix: restfront/
tag: (RESTFRONT orchestrator)
paths: [crates/topo/src/validate.rs, crates/topo/src/tier3_tests.rs]
keep_out: [opened by ATREST's close (docs/doc-ledger/atrest-leaves-the-tracker.md) on ATREST's ground, and inherits its seam - TOPO keeps the Euler operators proper and the sibling cuts are ORIGIN TQUERY WALKS PROBE, a row here that has to edit another program's ground announces the seam in the PR that lands it rather than drawing a second fence]
priority: P1
---
**What tier 3 still does not examine.** ATREST made the at-rest
validator right in both directions on the ground it reads: a per-solid
sign (check 7), shell winding 0/1 (check 10), exact loop winding on line,
circle and ellipse loops (check 6), ring nesting through the carrier walk
(check 9), and datum refusals (check 1). Every row here is either a place
the validator still assumes rather than checks — named in
`validate.rs`'s not-yet-checked list — or a cost one of those checks
pays. A checker that is silent is not a checker that says yes; each row
turns an assumption into a refusal or into a stated, pinned limit.

Charter and order: `work/restfront/plan.md`; narrative in
`work/restfront/log.md`.
