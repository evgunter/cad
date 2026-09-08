# EVAL-10 — the evaluation's one nominal environment reaches every reader: `wire.rs` stops rebuilding it per node (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 10). **Item:**
`work/eval/nominal-environment-is-rebuilt-per-node-in-wire.md` (EVAL-9's
residue). **Track:** E — one implementer lane, one style review, fix
pass, record-at-merge; no A/B draw, no correctness arm (threading, no
keying). **Branch:** `eval/10-one-nominal-env`. **Difficulty:** S.

## The claim

**The nominal f64 environment is built once per evaluation and carried on
`wire::LaneEnv::nominal` (EVAL-9), and two readers in `wire.rs` still
build their own copy per node**: `profile_plane_f64`
(`crates/editor-core/src/eval/wire.rs`, `doc.param_env::<f64>()` inside;
called from the pre-pass in `eval/mod.rs` and from `section_of`) and
`section_of` (`program.resolve(&doc.param_env::<f64>())`, holding a
`LaneEnv` it does not read for this). `Doc::param_env::<f64>` is pure, so
the copies agree; what they cost is a `BTreeMap` per profile-bearing node
on the build path and a sentence ("the nominal environment is the
document's own, under no box and no seed") that has to stay true in
three places.

## What lands

1. `section_of` reads `lane.nominal` — one line.
2. `profile_plane_f64` takes the environment (`nominal: &ParamEnv<f64>`)
   instead of building one; its two call sites pass the evaluation's.
   After this, `doc.param_env::<f64>()` has ONE call site in the
   evaluator (`eval/mod.rs`, where `nominal_env` is built) — assert it
   with `rg -n 'param_env::<f64>' crates/editor-core/src/eval` in the PR
   body, and say what the remaining sites outside `eval/` are
   (`mate/member.rs`'s solve, per EVAL-9's sweep) and why they are not
   this unit's.
3. **One spelling of "evaluate this node's slot at f64", if it costs
   nothing**: `profile_plane_f64`'s `read` closure walks the frame node's
   slots by hand (`node.expr(slot)` + `expr::eval`, refusing
   `MissingSlot`), beside `slots::eval_slots` (the door EVAL-9 made the
   key's one door) and `slots::vec3`. If `eval_slots(frame_node, nominal)`
   + `slots::vec3` yields the same values AND the same refusals
   (`eval_slots` treats an absent expression as unreachable where the
   closure refuses `MissingSlot` — read both and say which is right for
   a frame node whose axis slot is missing), route `read` through the
   door; if the refusal shapes genuinely differ and the difference is
   load-bearing, keep the closure and say so in the PR body with the
   case. Either way no refusal a row pins may change.
4. The sentence about the environment lives ONCE, at `LaneEnv::nominal`'s
   doc; `profile_plane_f64`'s and `section_of`'s docs point at it.

## Sweep

`rg -n 'param_env::<' crates/editor-core/src` — every construction of a
parameter environment in editor-core with a disposition (the
evaluator's one, the solve's, any other). Blind spot: an environment
built through a helper not named `param_env`.

## Review

One style lane (`docs/prompts/reviewer-style-lane.md`). Claims: the diff
is threading only (no key changes: a base-vs-head run of the memo rows
and `eval9_nominal_in_the_key` green, and `tag::format::VERSION` untouched);
`rg 'param_env::<f64>'` over `eval/` has one hit; Q1 on the slot-walk
spellings (item 3) — is any other hand-walk of a node's slots left in
`wire.rs`; Q4 — every sentence describing where the nominal comes from is
true; Q6 — anything left is a file.

## Records at merge

`work/eval/log.md` entry; the item `closed` with `pr:`; this spec deleted
per `docs/DOC-LEDGER.md`.
