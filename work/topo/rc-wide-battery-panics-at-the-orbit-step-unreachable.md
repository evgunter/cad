---
id: rc-wide-battery-panics-at-the-orbit-step-unreachable
kind: issue
title: rc_wide_battery panics in 8 of 84 shards at insert.rs orbit_step_at's unreachable! (was 40 320 lines, no panic)
status: open
opened: 2026-10-04
priority: P0
cost: M
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

## Removed by JOIN's PR 4036 (branch `join/reflex-corner-vertex-vertex`)

The panicking poses are four-germ vertex pairs. A fan minted first
moves the half-edge that the pair's strut, in the other entry of the
same physical sector, anchors on. The anchor read
(`insert::mint_directed`'s unshared strut `orbit_step_at`) then finds
the half at the fan's copy.

PR 4036 marks a multi-run pair's runs `shared`, so its struts mint
before its fans, anchored off the sectors. On that branch, after
merging main, `rc_wide_battery`, `pierce_runs_battery` and both reflex
batteries run to the end with no panic. r1 and r2 report the same:
525 of r1's probe runs panic at the merge-base, and head builds 513 of
them SOUND and refuses 12 typed.

Without the shared marking (review mutants M3, M3x) the panic returns.
The read stays a panic, not a typed refusal. Whether it should become a
typed refusal is TOPO's question; this note only records that the
reachable path is gone.
