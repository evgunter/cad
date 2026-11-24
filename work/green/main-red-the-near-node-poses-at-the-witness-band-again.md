---
id: main-red-the-near-node-poses-at-the-witness-band-again
kind: issue
title: main red: sweep bisector_nodes::the_near_node_poses_at_the_witness_band is red again (its 2026-11-14 row is closed)
status: closed
opened: 2026-11-23
priority: P1
cost: M
refs: [main-red-a-near-node-cap-at-1e-8-refuses-on-quadrature]
closed: 2026-11-23
pr: 7682
---


## The finding

(Filed by the WALKS orchestrator, 2026-11-23.) Hosted CI on PR 7477 (`walks/refusal-split`, head 3befc445, test (kernel), job 137448075289) panics at `crates/sweep/tests/bisector_nodes.rs:636` in `bisector_nodes::the_near_node_poses_at_the_witness_band`. PR 7475's FULL reviewer reproduced the same red on that PR's merge base (origin/main f1ca5d1b) with the same symptom, so it is main's, not either PR's. The row that last fixed this test, `main-red-a-near-node-cap-at-1e-8-refuses-on-quadrature`, closed 2026-11-14; the test file was last touched by GERM on 2026-11-20 (`77fe49a98b`, `f6c4c19a46` on `germ/sector-side-order2`), the first place to bisect.

## What a fix is

Bisect main's merges since 2026-11-14 to the merge that re-reddened it; the program whose change did is owed the fix.

## Closed (2026-11-23, GREEN U18)

This is the same red as the one PR 7354 caused in
`main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9`.
Bisected on hosted CI: the test passes on `4a94f7109b^1` (run 45312853278)
and fails on `4a94f7109b`, PR 7354's merge (run 45312883535), with the
payload main has at `a433ff83d6` (run 45307943329). The payload is the
cell `A's cap δ = 1e-8, direct (B, A), ∪` refusing on
`Euler(RebasedCarrier { … TransversalityArm … dihedral_arm_wedge })`.
GERM's `germ/sector-side-order2` commits were not the cause. At the
default ε the row is now held under `test_utils::pinned_red::row`, and
the judgement is SECTCONE's, in
`work/sectcone/conic-pairs-on-the-value-moved-two-band-edge-refusals-and-boss-unions-section`.
