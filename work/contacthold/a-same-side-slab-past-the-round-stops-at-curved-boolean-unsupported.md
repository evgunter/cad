---
id: a-same-side-slab-past-the-round-stops-at-curved-boolean-unsupported
kind: issue
title: A same-side slab running past a quarter round's ends (sameside-flush, beyond) stops at CurvedBooleanUnsupported, a separate door from the short-circuit
status: open
opened: 2026-10-11
priority: P3
---


Seen by both reviews of PR 4704, and recorded in
`a-same-side-tangent-touch-refuses-at-every-op`'s Finding. The
`sameside-flush` and `beyond` postures (the slab's `y` running past the
round's ends) refuse `CurvedBooleanUnsupported` at every op, on head and
on base. That is a different door from the short-circuit; one designer
suspects `recl::require_same`'s declared-Tangent arm, or the cap arc's
vertex on the slab top's interior. Measure which door refuses before
acting on either theory. The reviewers' print rows in
`crates/sweep/tests/review_4704_r1_probes.rs` and
`review_4704_flush_round_oracle.rs` reproduce it.
