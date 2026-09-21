---
id: check-10-is-not-made-by-the-mixed-tier-3-passes
kind: issue
title: check 10 is not made by validate_pseudomanifold or contact_marks: a sign-level read has no lane-dispatched door
status: open
priority: P2
cost: D
parent: ATREST-1
rides_with: tier3-prime-still-couples-plus-v-to-the-reporting-target
opened: 2026-09-20
---

Disclosed by ATREST-1 when check 10 landed. `validate_geometric` makes
it; `validate_pseudomanifold` and `contact_marks` do not, so a body
holding two outer boundaries under one solid still passes the mixed
tier-3 passes.

The reason is D-D of `ATREST-1`'s spec. A shell's role is a SIGN, and
check 10 reads it through `props::sign_certified` restricted to that
shell's faces — a certified door, bounded on `CertifiedBounds`. The
mixed passes keep their lane: their check 7 arrives through the
`PlusVCheck` hook `lane_certificate`, which reads at the REPORTING
target. Wiring check 10 to that hook would mint a second instance of
`tier3-prime-still-couples-plus-v-to-the-reporting-target` inside the
program that filed it, so it was not wired.

This row therefore rides with that one: once tier 3' has a sign-level
door, check 10 joins the battery through it and this closes with it.

`validate_geometric`'s not-yet-checked list cites this file.
