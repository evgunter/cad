---
id: a-placer-row-states-what-a-poisoned-row-cannot
kind: issue
title: A Part's index and a pattern's count refuse with PlacerRow::States over a node that may be poisoned, so the cause is stated on no row
status: closed
opened: 2026-10-01
priority: P2
cost: E
closed: 2026-10-16
---


## Finding

Found by MSOLVE-11's review (PR 3680, ruling R2), and pinned as it
stands today by
`msolve11_mate_log::a_part_index_refusal_behind_a_poisoned_pattern_is_pointed_at_a_silent_row`.

`crates/editor-core/src/mate/member.rs`, `check_reference`, refuses
three things with `PlacerRow::States`. The claim behind that row value
is that the named node fails in its own right with the same refusal,
so its row states the cause and the mate's sentence only points there:

- a pattern's count that does not evaluate, at the pattern;
- a `Part`'s index that does not evaluate, at the `Part`;
- a `Part`'s index outside its value, at the `Part`.

The claim fails whenever an input of the named node itself fails. The
evaluation never reaches that node's slots: its row reads "poisoned
through N". Nothing then states the refusal the mate points at. The
mate carries no line (`MateFault::carried` returns `None` for
`States`), and the named row shows a different cause.

The pinned row builds this with a `Part` index that overflows at
`k = 1`, over a pattern whose instance's part does not resolve:

- the mate names the `Part` with `States`;
- the `Part` is `Poisoned` through the instance.

The pattern's `Count` has the same shape whenever the pattern's own
input fails.

## Shape

Probably a one-liner at the raise site. `check_reference` holds no
evaluation, so it cannot read the named node's standing. The
evaluation's mate node can: where the placer's row is not `Failed`,
read `PlacerRow::Silent`. That puts the decision where the standing is
known, as `MateFault::carried` already reads the row value. It is a
re-siting of the decision, not a new mechanism. The pinned row flips:
`Silent`, and the mate carries the refusal.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `a-mate-reads-face-variables`, not on the whole program: the raise site is member.rs check_reference's head walk through Pattern counts and Part indices, replaced by a typed unresolved read once mate sides read Face variables. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Re-parked at F's close (2026-10-10)

F (`a-mate-reads-face-variables`, PR 4566) made a mate's sides selections, but kept the solve's member walk: `mate/member.rs`'s `check_reference` still raises `PlacerRow::States` over a pattern's count and a `Part`'s index, and a mate the solve faults fails with that fault before poison propagation, so the pinned row stands as written. The walk goes with the spanning tree and roots in stage 3 B, so the row waits there.

## Closed by INTENT stage 3 B (PR #5232)

The decision moved where the standing is known, as the row's Shape proposed. The placement reporting a solve fault (`placement_fault`, `crates/editor-core/src/eval/wire.rs`) reads a `PlacerRow::States` fault's placer off the evaluation, and turns it `Silent` when the placer did not fail in its own right. The fault then carries the refusal. The pinned row in `msolve11_mate_log` is flipped: the fault is `Silent` and carries the `Part`'s index refusal.
