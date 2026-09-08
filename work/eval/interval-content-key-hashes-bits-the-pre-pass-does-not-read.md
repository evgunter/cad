---
id: interval-content-key-hashes-bits-the-pre-pass-does-not-read
kind: issue
title: At Interval the content key hashes a slot's bounds while the profile precompute and the pinned op read its nominal f64, which the key does not hold
status: review
opened: 2026-09-08
refs: [bracket-scope-is-run-op-not-the-node, 2176]
branch: eval/9-nominal-in-the-key
pr: 2190
---

## What

`content_key` (`crates/editor-core/src/eval/mod.rs`, EVAL's) folds a
frame node's slot values in at the evaluation scalar: at `Interval` the
frame's key is a function of each slot's `(lo, hi, dec)` bits
(`eval/memo.rs`'s slot hashing, DOCM's). Two readers of the same frame
read something else: `wire::profile_plane_f64`
(`crates/editor-core/src/eval/wire.rs`) reads the plane's slots at the
NOMINAL f64 from the document, and the pinned op embeds that f64
placement whole (`placement.map(T::from_f64)`). The nominal is not in
the key at Interval, so a nominal edit under a compensating box leaves
every key equal and the memo serves a value whose placement is the
OLD nominal's.

Found by the EVAL-7 correctness review (PR 2176); reproduces on the
base, so it is a pre-existing hole the bracket move did not open. The
hit-site assertion EVAL-7 added (`eval_node`, the memo hit) cannot see
it: a `Verdict` is `(predicate, sign)` and both runs' `datum_unit_norm`
verdicts are `Positive`.

## The probe (the future red-first row)

Frame `u = [1, p, 0]` with `p` a scalar document parameter, nominal
`0.0`, evaluated at `Interval` with `param_box p ∈ [-0.25, 0.25]` as
`prior`; then `SetDocParamValue p = 0.5` evaluated with
`p ∈ [-0.75, -0.25]` (the same Interval bits) and that prior:
`recomputed = 0`, the frame's and the profile's content keys equal the
cold run's, and the reused profile's plane is the identity while the
cold run's is `(0.894, 0.447, …)`. Written in full in the review's
probe (`review_interval_hit_with_a_nominal_edit_under_a_compensating_box`);
the row asserts the hit's profile payload equals the cold run's.

## Fix shape

Each slot's nominal f64 joins the key beside its lane bits (a
content-key format bump, `tag::format::VERSION`), so a nominal edit
moves every key that reads the nominal. `content_key` is EVAL's; the
slot hashing in `eval/memo.rs` is DOCM's, so the change crosses one
announced seam. Not built in PR 2176; the orchestrator schedules it.

## Closed

EVAL-9 (PR 2190). Every slot feeds its nominal f64 beside its bits at
the evaluation scalar, under `tag::slot`'s word, and both lists come
from one door: `eval_node` calls `slots::eval_slots` twice, at the
lane environment and at the document's nominal one (carried on
`wire::LaneEnv::nominal`), and `content_key` zips them. A slot that
refuses at the nominal refuses its node typed. `tag::format::VERSION`
6 -> 7. The probe is the row
`over_a_param_box::a_nominal_edit_under_a_compensating_box_does_not_hit`
(`crates/editor-core/tests/eval9_nominal_in_the_key.rs`), red at the
merge base on the served plane (identity where the edited document's
is `(0.894, 0.447, …)`) and green at the head, with the neighbouring
rows pinning that the box is still a real input and that the typed
refusal is reached. `eval/memo.rs` is untouched: the feed lives at the
key.
