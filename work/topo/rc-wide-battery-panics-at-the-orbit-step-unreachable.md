---
id: rc-wide-battery-panics-at-the-orbit-step-unreachable
kind: issue
title: rc_wide_battery panics in 8 of 84 shards at insert.rs orbit_step_at's unreachable! (was 40 320 lines, no panic)
status: closed
opened: 2026-10-04
priority: P0
cost: M
closed: 2026-10-05
pr: 4043
---


Found by JOIN's PR 4031 fix pass (2026-10-04), running
`crates/sweep/tests/join_rc_probes.rs` `rc_wide_battery` (release,
`#[ignore]`d, nightly) on main `8793177b`.

## Measured

- **Main `45dc18f9`** (before PRs 4029 and 4033): 40 320 lines, no panic.
- **Main `8793177b`**: the battery panics at
  `crates/topo/src/boolean/insert.rs` `orbit_step_at`'s `unreachable!`:
  "the orbit step from HalfEdgeKey(1v1) at VertexKey(1v1) lands on
  HalfEdgeKey(46v1), which starts at VertexKey(21v1): every public door
  keeps the body tier-1-valid, where every such walk closes".
  - Unsharded, the test dies after 1 475 lines.
  - Run as `RCW_SHARD=k/84`, k = 0..83 (one shard per profile and
    turn), 8 shards panic and 36 952 lines print; 3 368 of the base's
    lines are lost.
  - The panicking (profile, turn) pairs: sqQ1 −20, dLeft 190, dDown
    100, dRight 7, dRight 33, eBot −20, eLeft 0, eLeft 33.
  - In each, the last line printed before the panic is a
    `JoinDesync`, e.g. `RCW sqQ1 rot=-20 sx=-0.75 sy=0.3 S_ab => ERR
    JoinDesync { what: "B senses agree at a matched pair" }`, so the
    panic is the next pose's.
- Every line that does print is identical to the base's.
- PR 4031's head gives the same 36 952 lines and panics in the same 8
  shards.

## Why it is reachable

`2ad740e2` ("the boolean's torn-operand and torn-graft-source refusals
panic") turned this site into an `unreachable!`, premised on a
tier-1-valid body. `orbit_step_at` runs on the reduction's body
mid-insertion, after struts are hung (`insert.rs`, the dangling-strut
arm `None => orbit_step_at(body, vertex, sectors[from].he)`), where that
premise is the insertion's own invariant and not the operand gate's. On
the base these poses answered, a body or a typed refusal, and none
aborted. Now a public `union`, `intersect` or `subtract` aborts the
process on them. Under the fail-loud rule that is a typed refusal to
restore, or an invariant to prove at the site. Which commit in the
range moved the walk here was not bisected.

## Closed 2026-10-05

**A genuine insertion defect, reached by input, not a legitimate
mid-insertion state.** At every panicking pose one vertex pair (one
plan) has two runs at A's vertex in `b ∖ a`: a fan, minted first, whose
`mev_null` carries `sectors[k].he` to its copy vertex, then a dangling
strut in entry `k`, whose corner is `next(mate(sectors[k].he))`. The
sector table was read before either mint, and `reconcile_shared` only
reconciles one pair's runs against *other* pairs' cuts, so nothing
stops the second run reading a half the first moved. That is the strut
form of `work/join/four-germ-vertex-pairs-run-b-in-a-order` (whose fan
form refuses `Euler(FanStartMismatch)` from `mev_fan_plan`).

On `45dc18f9` the unchecked step landed at the copy and the strut hung
there; the join refused `JoinDesync` downstream on all 56 such poses.
PR 4033's `orbit_step_at` turned the same read into an abort. Its
premise, "a half starting at `vertex`", was never established for this
caller: the strut site reads a sector half after earlier mints.

`insert.rs` `mint_directed` now checks, before any strut read, that the
strut's corner half still starts at `vertex`, and refuses
`ClassificationInvariant` "an earlier run at the vertex carried a
strut's corner to its copy" when it does not. With that, `orbit_step_at`'s
premise holds at every caller. All 84 shards complete: 40 320 lines, no
panic; they differ from the base's in exactly those 56 lines.
Pinned by `crates/sweep/tests/join_rc_probes.rs`
`a_strut_whose_corner_an_earlier_run_moved_refuses_typed`, one pose per
aborting (profile, turn) pair.
