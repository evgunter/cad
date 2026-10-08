---
id: a-live-spliced-into-another-body-is-caught-by-nothing
kind: issue
title: a Live proven against one Body and spliced into another resolves in the second with nothing to catch it
status: open
opened: 2026-09-29
priority: P3
cost: M
design: true
refs: [live-guard-proves-ordering-not-identity]
---


Found by PR 3424's fix pass, answering its review: `live.rs`'s header
said a `Live` proven in one body and spliced into another was "the
validator's business", and no validator row catches it —
`review_m1_pr1::foreign_keys_resolve_arbitrarily_as_documented` checks
only that a foreign key resolves without panicking, and
`release_corruption::foreign_parent_loop_garbage_in_garbage_out_release`
plants a wrong `parent_loop`, not a cross-body `Live`. `body.rs`'s
module docs call foreign keys "not protected against". The `Live`
token claims "this key resolves in THIS body" and carries no body
identity, so the type cannot refuse it either. What a taker decides:
whether a `Live` should carry the identity of the body that proved it
(a brand, a body id checked at use), or the claim should be scoped down
to what a key can promise. `design: true`: either answer changes the
token.
