---
id: a-poisoned-curve-carrier-datum-is-refused-at-rest-only-by-accident
kind: issue
title: A curve carrier's poisoned or out-of-convention datum at rest is refused only by check 2's residual, which names the edge and not the datum
status: open
opened: 2026-09-24
priority: P4
cost: D
refs: [ATREST-6]
---

## What

The sibling of `quadric-datums-unchecked-at-rest`, one dimension
down, found by ATREST-6's sweep and NOT measured.

Check 1 now names an analytic SURFACE whose stored datum describes no
locus. Nothing names a CURVE's: a circle or ellipse radius that is
NaN, zero or negative, or a line `dir` that is zero, reaches tier 3
only through check 2's carrier re-certification, whose refusal is
`ValidationError::EdgeCertification` with a `CertifyError` — and no
`CertifyError` variant names a carrier datum
(`crates/geom-brep/src/certify.rs`, `CertifyError`: the variants are
residual, transversality, winding, escalation and lane refusals).
Attachment certifies a carrier at mint, so the path in is the raw
arena swap `tier3_tests.rs`'s `wrong_cache_at_rest_is_rejected_by_tier3`
already uses.

`validate_geometric`'s not-yet list (`crates/topo/src/validate.rs`,
"Curve conventional-invariant certification") names the unit-frame
half of this and argues it is "partially implied by the residual
checks"; the poison half is the same argument and the same accident.

## What is owed first

The measurement: a pillow edge's carrier swapped for a circle of each
bad radius, and what tier 3 answers — before deciding whether check 2
owes a named datum refusal or a check of its own.

## Fence

Track P. `crates/topo/src/validate.rs` (check 2).
