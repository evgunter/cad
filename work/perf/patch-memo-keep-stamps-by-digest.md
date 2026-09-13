---
id: patch-memo-keep-stamps-by-digest
kind: issue
title: PatchMemo::keep stamps an entry found by digest where record stamps one proved by its key
status: open
opened: 2026-09-13
---



## The asymmetry

`PatchMemo::record` (`crates/mesh/src/memo.rs`) re-stamps a hit
entry's picture only when the entry's FULL key bytes are the face's,
and says why: "A same-picture insert can have replaced this digest's
entry with one whose bytes are another face's; that entry is not this
face's and keeping it alive on this face's behalf would serve a
collision."

`PatchMemo::keep` — the door a caller uses when it reuses a whole
body's mesh without looking its faces up — does the same stamping with
no such check: it takes `PatchKeys`, walks the digests, and stamps
whatever entry each digest finds.

PERF-10 made the gap visible rather than creating it: the pick memo's
table level does the same job (`PickMemo::keep_tables`) and now keys by
`mesh::StoredPatchId`, which names ONE entry and cannot be satisfied by
a stranger. The two levels therefore stamp with different precision
for the same reason, and the looser one is the older one.

## What it costs

Eviction only, never a wrong patch: a collision under `keep` keeps a
stranger's entry alive one extra picture and lets the real one go,
costing a miss next picture. With a 128-bit FNV digest the state is
vanishingly rare, which is why this is an issue and not a bug.

## What a fix is

`PatchKeys` already carries each face's `StoredPatchId`
(`PatchKeys::stored_ids`). `keep` can stamp the entry under a digest
only when its id is the one the caller's tessellation reported —
the same shape `record` uses, and the same shape `keep_tables` already
has.
