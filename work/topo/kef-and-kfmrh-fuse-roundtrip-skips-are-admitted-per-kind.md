---
id: kef-and-kfmrh-fuse-roundtrip-skips-are-admitted-per-kind
kind: issue
title: seqgen's may_skip_roundtrip_at admits every Kef and KfmrhFuse skip per kind, though both arms decide their skip from pre-kill reads
status: open
opened: 2026-10-05
priority: P4
cost: E
refs: [movefac-roundtrip-re-make-is-unbuilt]
---


## What

Found by the sweep of `movefac-roundtrip-re-make-is-unbuilt`.

`crates/topo/src/seqgen.rs`'s `OpChoice::may_skip_roundtrip_at`
narrows `Kev` and `Movefac` to the site and lets `Kef` and
`KfmrhFuse` fall through to `OpChoice::may_skip_roundtrip`, which
admits every choice of those kinds. Both arms of `roundtrip` decide
their skip from reads taken before the kill:

- `KfmrhFuse(f1, f2, _)` skips exactly where
  `fusion_remake_shell(body, f1, f2)` is `None`.
- `Kef(he, _)` skips exactly where the mate's loop is `[mate]` alone,
  `next(he) != he`, and the surviving loop is not the outer of a
  ring-free face.

So the fuzz row's per-step bar
(`seqgen/random_op_sequences.rs`, `random_op_sequences_hold_all_properties`)
is blind to a widening inside either arm, the blindness its own doc
names.

## The shape to give

`may_skip_roundtrip_at` asks both site questions, through one helper
per arm that `roundtrip` also calls, so the classifier and the arm
cannot drift; `may_skip_roundtrip`'s per-kind list stays the
documented superset.
