---
id: inline-refuses-a-placed-part-off-the-identity
kind: issue
title: Inline refuses every part placed by the place gesture once its instance stands off the identity (ValueOnAMovedWorld)
status: open
opened: 2026-10-18
---


## Finding

Found by the stage 3 C review (PR #5267, R2's m3). Inline lands a
part's world mates where its instance stood by appending the instance's
solved motion to each world read (`crates/editor-core/src/refactor.rs`,
`Landing::Pinned`). A value on such a mate is charted on the world's own
frame, which the appended motion does not move, so inline refuses it
off the identity (`InlineError::ValueOnAMovedWorld`,
`refactor.rs:1419`, raised at `refactor.rs:4059`; the row is
`crates/editor-core/tests/intent_s3_c_mates.rs:1198`
`inline_refuses_a_moved_landing_under_a_world_value`).

The "place part" gesture (`mate::place_at`) is how every corpus
document and the façade place a copy, and it always writes three values
on its world mate. So a part placed the ordinary way cannot be inlined
once its instance stands anywhere but the identity: the refusal is the
common case, not an edge.

## What would close it

Inline carries a valued world mate across a moved landing: re-express
the values against the moved world (the mate's world read moved by the
landing, the values re-read off the copy's landed pose with
`Chart::slide_at` / `spin_at`, as `place_at` writes them), or restate
the mate as the landing's frame mate. Either keeps every name and pose
through a split-then-inline round trip off the identity, and the row
above becomes a landing row.
