---
id: value-digest-axis-in-plane-and-measure-unavailable-share-tag-24
kind: issue
title: The value digest's AxisInPlane and MeasureUnavailable arms both write tag 24
status: open
opened: 2026-10-02
priority: P3
cost: E
---


## What

`stackup.rs`'s `payload_digest` opens every arm with a tag word, and
two arms write the same one:

- `ValuePayload::Datum(DatumValue::AxisInPlane { .. })` writes
  `d.u64(24)` (`crates/editor-core/src/stackup.rs:936`), its comment
  choosing 24 as "not a number among the datum arms above".
- `ValuePayload::MeasureUnavailable { .. }` writes `d.u64(24)` and
  nothing else (`crates/editor-core/src/stackup.rs:1015`), under a
  comment that calls it "APPENDED, never reused (the tag rule this
  digest shares with the content keys)".

Each arm took 24 as the next free number without seeing the other.

## Why it matters

The digest's tag is what says which arm a payload is. With 24 shared,
the two arms are told apart only by what follows it: the axis's ten
scalars against the measure's empty tail. `MeasureUnavailable`'s whole
stream is a prefix of every `AxisInPlane` stream, so the two digests
differ only by the hash of that tail, not by construction. Today the pairing in
`pair_pass` (`stackup.rs:747`) compares one node's two passes, and a
datum node and a measure node never meet there, so no live answer is
wrong. It is the tag rule broken: a third arm appended on the same
reasoning collides for real.

## Fix

Give `MeasureUnavailable` a fresh number past every one in use (27;
`Gauge` holds 26, and 20 is retired in `RETIRED_VALUE_DIGEST_TAGS`).
Then extend `stackup::tests::value_digest_tags_reuse_no_retired_number`,
which already reads every arm's tag out of the `VALUE-DIGEST-ARMS`
sentinels, to refuse a tag two arms share, so the next collision goes
red. It checks only retired numbers today because this collision
would turn it red.

Found by RECIPE's follow-up to PR 3902, while putting retired tag 20
under a guard.
