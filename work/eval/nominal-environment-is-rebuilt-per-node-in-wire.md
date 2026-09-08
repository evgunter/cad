---
id: nominal-environment-is-rebuilt-per-node-in-wire
kind: issue
title: Two readers in wire.rs rebuild the nominal f64 environment per node although the evaluation now carries one
status: spec
opened: 2026-09-08
refs: [interval-content-key-hashes-bits-the-pre-pass-does-not-read]
branch: eval/10-one-nominal-env
---

(EVAL-9 implementer, found by that unit's sweep. In EVAL's fence, so
filed here per `docs/prompts/implementer-discipline.md` §6.)

**The finding.** EVAL-9 gives the evaluation ONE nominal environment,
built beside the lane environment in `evaluate_at_descent` and carried
to every reader on `wire::LaneEnv::nominal`
(`crates/editor-core/src/eval/wire.rs`, the lane bundle;
`crates/editor-core/src/eval/mod.rs`, `nominal_env`). The content key's
slot nominals and the profile program's f64 resolution read it from
there. Two readers inside `wire.rs` still build their own copy, per
node:

- `crates/editor-core/src/eval/wire.rs:891` — `profile_plane_f64`
  calls `doc.param_env::<f64>()` on every profile node and every loft
  or sweep section. It takes `(doc, plane, tol)` and no lane bundle, so
  reaching the shared one is a signature change.
- `crates/editor-core/src/eval/wire.rs:4203` — `section_of` calls
  `program.resolve(&doc.param_env::<f64>())` on every section, although
  it already holds a `LaneEnv` and could read `lane.nominal`.

**Why it is not a correctness hole.** `Doc::param_env::<f64>` is a pure
function of the document, so the three copies are equal by
construction and no reader can see a different nominal from the key's.
What it costs is one `BTreeMap` per call — allocation on the build
path, in functions called once per profile-bearing node — and one more
place to keep the sentence "the nominal environment is the document's
own, under no box and no seed" true.

**Why not in EVAL-9.** `wire.rs`'s `Pinned` arm and `validate.rs` were
EVAL-8's lane while EVAL-9 ran, so that unit's `wire.rs` edit was held
to `LaneEnv` and its construction. The remaining change is threading,
not keying.

**What a fix needs.** `section_of` reads `lane.nominal` — one line, no
signature change. `profile_plane_f64` takes the environment rather than
building one, which moves its two call sites
(`eval/mod.rs`'s pre-pass and `section_of`) and leaves
`doc.param_env::<f64>()` with a single call site in the evaluator.
