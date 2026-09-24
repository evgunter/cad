---
id: the-re-basing-gates-staleness-and-both-halves-arms-have-no-door-left-to-feed-them
kind: issue
title: the re-basing gate's carried-staleness arm and both gates' both-halves null arm have no door left to feed them
status: open
opened: 2026-09-24
priority: P4
cost: E
refs: [kevs-fan-merge-needs-a-re-describing-kill-door]
---


## What

Filed by `kevs-fan-merge-needs-a-re-describing-kill-door` (branch
`topo/kev-describing-door`), which gave `Body::kev` its refusal and
added `Body::kev_describing`. After that unit, two arms of the
re-basing machinery guard states that no public door produces, and
each is exercised only by a fixture that builds its state through the
kill's crate-internal, ungated execution (`Body::kev_plan` +
`Body::kev_execute`, `crates/topo/src/euler_kill.rs`).

- **The carried-staleness arm** of `Body::certify_rebased_run`
  (`crates/topo/src/euler.rs`, the "A pre-existing staleness is
  carried, not refused" paragraph): it carries a moved edge whose
  carrier ALREADY missed its own endpoint. Its one producer was the
  unchecked fan merge; `kev` now refuses that merge and
  `kev_describing` either re-describes or re-certifies each member.
  Measured on the generator: `seqgen`'s 64 x 32 pinned streams held
  a stale edge at 734 of 2048 steps on the merge base and at 0 on the
  unit's head, and `seqgen::split_site` now ASSERTS coherence over
  every generated edge (the proptest and teardown rows green). Row:
  `euler::tests::the_gate_carries_a_run_an_earlier_kev_had_already_made_stale`,
  whose stale edge is now planted through `kev_execute`.
- **The both-halves null arm** of both gates (the mev gate's in
  `certify_rebased_run`, and `kev`'s keys-only gate in
  `Body::kev_keys_only_gate`): it carries a null edge whose two halves
  both move. That needs a null SELF-LOOP, and none can be built: a
  null edge is minted only by `mev_null`, always to a fresh vertex;
  two existing vertices are joined only by `mef`/`mekr`, which mint a
  certified edge; and closing a null edge onto one vertex therefore
  takes a kill of a certified edge that moves ONE end of the null
  edge, which both kill doors refuse (`RebasedNullEdge`). Rows:
  `euler::tests::a_fan_mev_moving_both_halves_of_a_null_edge_keeps_it_one_vertex`
  and `euler::tests::kevs_fan_merge_moving_both_halves_of_a_null_edge_keeps_it_one_vertex`,
  over `null_self_loop_beside_a_strut`, which now builds through
  `kev_execute` and says why.

## The question

Whether each arm stays as defence for a body that arrives with the
state (an import path, a door written later), or retires with a
fixture change. A gate arm with no producer is documentation until
something produces its input; retiring it makes the refusal
unconditional, which is the stricter reading. Neither is decided
here. What the sweep behind this claim could not see: a stale
carrier produced by a door that writes vertex POINTS rather than
re-basing half-edges (`offset_axial.rs`, `offset_together.rs`,
`replace_face.rs` each write `.point = new_point`); none of the three
was read for whether it re-describes every incident edge.
