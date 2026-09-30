---
id: value-channel-digest-tag-24-collides
kind: issue
title: the value-channel digest's discriminator is not injective: tag 24 is claimed twice
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## What

`crates/editor-core/tests/fixture/value_channel.rs` prefixes each
payload arm with a discriminator word so that two runs that produced
DIFFERENT payload kinds cannot hash the same. Two arms claim 24:

- `ValuePayload::Datum(DatumValue::AxisInPlane { .. })` — emits `24`,
  then four scalars, a point and a vector;
- `ValuePayload::MeasureUnavailable { .. }` — emits `24` and nothing
  else, under a comment stating it is "at its own tag".

So the tag is not injective, and the comment asserting that it is, is
false. (The arms' payloads differ in length, so the two do not collide
in practice at a node whose neighbours are stable; that is a property
of the surrounding feed, not of the discriminator, and it is exactly
the kind of accidental separation that stops being true when an arm is
edited.)

Every other arm has a unique tag (10-23, 13-22 in use); 25 is free.

## Why it matters more now than it did

Pre-existing on `main` — it arrived with the digest when it was private
to `m10_di_dual_corpus.rs`. SUITE/D114 moved the feed VERBATIM into
`fixture/value_channel.rs` (no drift; the move was checked
line-by-line) and it is now the shared home of what "bit-identical to
the `f64` run" quantifies over, read by both editor-core cross-scalar
differentials. A discriminator that is not injective is a weaker claim
than the module's docs make, in the one file whose job is to make that
claim precise.

## Fix

Give `MeasureUnavailable` tag 25 and say in the `AxisInPlane` arm that
24 is its.

Filed by SUITE/D114 (found while moving the feed, not fixed there: the
move was kept verbatim so it could be checked as a move).
