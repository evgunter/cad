---
id: ATREST-1
kind: unit
title: tier 3 learns to ask which solid: a per-solid volume sign and a per-solid shell-role check
status: spec
opened: 2026-09-20
priority: P0
cost: D
branch: atrest/1-per-solid
---


Carries two P0 admit-holes that are the same defect stated twice:
`an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned`
and `tier-3-does-not-check-shell-roles-per-solid`. Tier 3 never asks
which SOLID anything belongs to — it pins the body's TOTAL signed
volume and never groups shells — so a reverted part beside a larger
ordinary solid certifies, and so does a solid holding two `Outer`
boundaries.

One call answers both: `props::classify_shells_of(body, &solid.shells,
tol)` returns each shell's signed volume and its decided role, and
`validate.rs` calls it from nowhere today. The solid's volume is the
sum of its own shells'; its roles are the same rows read for
`Outer`/`Void`. Split across two lanes the two rows would mint that
per-solid walk twice in one 9200-line file and conflict over it.

Spec: `docs/ATREST-1-SPEC.md`. Outside protocol v7 (opus/opus, full
review): the change is a new refusal on the door every program reads
as proof, so the review carries correctness claims alongside the style
questions — but it is neither especially tricky nor hard to reverse.
