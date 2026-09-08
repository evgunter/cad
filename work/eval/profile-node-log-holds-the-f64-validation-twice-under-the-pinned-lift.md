---
id: profile-node-log-holds-the-f64-validation-twice-under-the-pinned-lift
kind: issue
title: The Profile node log holds the f64 validation twice under the pinned lift: the pre-pass validates and the op validates the same Profile<f64> again
status: spec
refs: [bracket-scope-is-run-op-not-the-node]
opened: 2026-09-08
branch: eval/8-validate-once
---

## What

With the node's verdict frame open across the whole of `eval_node`
(`crates/editor-core/src/eval/mod.rs`, EVAL-7), a Profile node's log
shows what the node computes: on the one-solid part fixture
(`crates/editor-core/tests/kstats_bracket_rows.rs`,
`the_profile_nodes_log_opens_with_the_pre_pass_and_the_ops_decisions_follow`)
the log is 144 entries — two `datum_unit_norm` (the plane's axes), four
`path_junction_turn` (the replay), 69 validation verdicts from
`wire::prepare_profile`'s `validate_recording` at f64
(`crates/editor-core/src/eval/wire.rs`), and then the SAME 69 again
from the op's lane validation, because under `ProfileLift::Pinned` at
`T = f64` the op validates `from_f64` of the profile the pre-pass
just validated, under the same tolerance (VQ6). The row pins
`pre[6..] == op` for exactly this reason.

## Why it matters

Certification and `vdiff` read the log; a doubled sequence is not a
wrong record (both validations ran) but it is the record of duplicated
work — a few dozen predicate evaluations per Profile node on the build
path — and a reader of a Profile node's log sees every validation
verdict twice without being told why. At the Interval scalar and under
the guided lift the op's validation is a different computation and the
doubling is not one.

## Home

EVAL (`eval/mod.rs`, `eval/wire.rs`). Whether the op's validation is
skipped when it would be the pre-pass's bit for bit (`T = f64`, pinned
lift), or kept and the log's doc says the op re-validates, is a design
choice about what the op's log means at the build scalar; not decided
here.
