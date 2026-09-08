---
id: nominal-environment-is-rebuilt-per-node-in-wire
kind: issue
title: The nominal f64 environment is built once per evaluation and again per profile-bearing node inside wire.rs
status: open
opened: 2026-09-08
refs: [interval-content-key-hashes-bits-the-pre-pass-does-not-read]
---

(EVAL-9 implementer, found by that unit's sweep. In EVAL's fence, so
filed here per `docs/prompts/implementer-discipline.md` §6.)

**The finding.** EVAL-9 gives the evaluation ONE nominal environment,
built beside the lane environment and shared by every reader
(`crates/editor-core/src/eval/mod.rs`, `evaluate_at_descent`'s
`nominal_env`): the content key's slot nominals and the profile
program's f64 resolution both read it. Two readers inside `wire.rs`
still build their own copy, per node:

- `crates/editor-core/src/eval/wire.rs:885` — `profile_plane_f64`
  calls `doc.param_env::<f64>()` on every profile node and every loft
  or sweep section.
- `crates/editor-core/src/eval/wire.rs:4197` — `section_of` calls
  `program.resolve(&doc.param_env::<f64>())` on every section.

**Why it is not a correctness hole.** `Doc::param_env::<f64>` is a pure
function of the document, so the three copies are equal by
construction and no reader can see a different nominal from the key's.
What it costs is one `BTreeMap` per call — allocation on the build
path, in a function called once per profile-bearing node — and one
more place to keep the sentence "the nominal environment is the
document's own, under no box and no seed" true.

**Why not in EVAL-9.** `wire.rs` was EVAL-8's lane while EVAL-9 ran, so
the unit could not touch it; and the fix is a threading change
(`nominal_env` through `OpEnv` or as an argument), not a key change.

**What a fix needs.** One environment reaches `wire.rs`'s two readers
— `OpEnv` is the natural carrier, since it is already the ambient
seam bundle `run_op` takes — and `doc.param_env::<f64>()` has one call
site in the evaluator.
