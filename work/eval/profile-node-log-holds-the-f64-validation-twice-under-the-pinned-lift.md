---
id: profile-node-log-holds-the-f64-validation-twice-under-the-pinned-lift
kind: issue
title: The Profile node log holds the f64 validation twice under the pinned lift: the pre-pass validates and the op validates the same Profile<f64> again
status: closed
pr: 2186
refs: [bracket-scope-is-run-op-not-the-node]
opened: 2026-09-08
branch: eval/8-validate-once
closed: 2026-09-08
---

## Closed

PR 2186. `ValidatedProfile<f64>::lift_onto(plane)` lands in
`crates/profile` (announced seam to S-BOOL): the `f64` canonical form
embedded at the target scalar, decisions carried AS the f64 decisions
by the pinned lift's design (structure is selected once, at f64,
identically for every lane; the guided lift is the one that
re-verifies), an arc's carrier re-derived at the target scalar through
`seg::arc_carrier` over one `ChordFrame` shared with `build_seg`. The
`Pinned` arm lifts `ProfilePre::validated_f64`; `anchor::embed_profile`
retires. The Profile node's log under the pinned lift is the pre-pass's
75 at every scalar (`kstats_bracket_rows`); the lifted form equals the
re-validated form bit for bit in every value channel over the corpus
at f64, `Dual64` and `Interval` (`pinned_lift_validates_once`) and over
the profile fixtures (`profile/tests/validated_map.rs`); the one
behaviour change is pinned by a row — a margin definite at f64 and
indeterminate at `Interval` is served under `Pinned` with the f64 log
and refused under `Guided`; `asm2a`'s 799 → 730 and the m4 eps-audit
populations halved back to their pre-EVAL-7 values, tabled in the PR;
`m10_6_certifying_keys.txt` (Guided) did not move.

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
