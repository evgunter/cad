---
id: lane-profile-defaults-a-missing-replay-record-to-empty
kind: issue
title: lane_profile covers a missing pass-1 replay record with an empty one, so an internal break reads as a lane structure disagreement
status: open
opened: 2026-09-28
priority: P3
cost: E
---

Found by the GATHER editorial pass over `eval/wire.rs`
(`wire-rs-accumulation-residue-comment-ratio-and-wire-sweep`), where a
twelve-line comment was defending this line rather than the code
holding it.

## Finding

`crates/editor-core/src/eval/wire.rs`, `lane_profile`, reads pass 1's
record for each program loop as

```
let record = pre.structure.replay.get(li as usize).cloned().unwrap_or_default();
```

One record per program loop holds by construction of pass 1
(`prepare_profile`), so a miss is an internal break. The fallback does
not refuse it: it hands `profile::replay_guided`
(`crates/profile/src/path/program.rs`) an EMPTY `ReplayStructure`. A
loop with a fillet then refuses at `Guide::consume`; a loop without one
is caught only by `replay_guided`'s step-count shape check, which
reports `StructureRefusal::shape(0, n)` — the refusal for a lane whose
elaboration disagreed with the f64 pass. Either way the node error is
`ProfileLaneReplay`, which tells a caller the guided lift diverged from
the pinned one, not that the evaluation lost its own record.

The comment that stood above the line (now two sentences) said so in
as many words: "this comment says which half of the vocabulary is
actually holding the line".

## Direction

Refuse the miss where it happens, as the internal break it is (the
`ProfileAnchor` / `ProfilePieces` precedent a few lines up in
`prepare_profile`), and drop the comment.
