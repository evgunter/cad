---
id: a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime
kind: issue
title: A box intersected with a cylinder whose wall passes through the box's corner edge builds at the right volume and fails tier 3′
status: open
priority: P0
cost: M
opened: 2026-10-09
refs: [4399]
---

Found by the dual review of the tube-on-a-ball unit (PR 4399, review r1, its N4), and present on base `b1a15ad0` before that PR.

## What

The box is `[0, 2]² × [0, 1]` (`sweep::test_support::brick`). The cylinder is the extrude of the circle about `(0.5, 0.5)` with radius `√0.5`, over `z ∈ [−1, 2]`. Its wall passes through the box's corner edge `x = y = 0`, a ruling of the wall, and both box faces at that edge cross the wall there.

- `A ∩ B` and `B ∩ A` build at the right volume, `π/4 + 0.5 = 1.285398163`.
- The built body passes tier 2 and the certificate.
- It fails tier 3′: `topo::validate_pseudomanifold` refuses it against its own contact records.

A body the boolean ships fails a tier the door gates on, so this is a live wrong answer.

## Repro

Review r1's `tube_on_ball_r1_family_line_edges`, the pose `"line/crossing faces"`, on branch `join/tube-ending-on-a-ball-review-r1` (`crates/sweep/tests/tube_on_ball_review_r1_probes.rs`). It is an ignored row: run it with `--ignored --nocapture` and read `outcome`'s `t3p=false`.

## What the taker owes

Find which contact or edge the census reads as non-manifold at the corner ruling (a line edge of the box lying in the wall, met by both box faces), and decide whether the body or the census is wrong.
