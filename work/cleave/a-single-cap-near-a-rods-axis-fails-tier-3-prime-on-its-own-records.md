---
id: a-single-cap-near-a-rods-axis-fails-tier-3-prime-on-its-own-records
kind: issue
title: Two rods at right angles, one capped about 1e-4 off the other's axis, build union and capped-minus-full bodies that fail tier 3′ on their own records
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [5793]
---


Found by the narrow re-review of PR 5793 (CLEAVE), whose bisector scan
runs the same scene on main and on the branch.

## What

`sweep/tests/bisector_nodes.rs`
`the_near_node_poses_at_the_witness_band` builds two unit rods at
right angles with one rod capped δ off the other's axis (the "A's cap"
and "B's cap" poses), in every op, both operand orders and both poses,
and asserts the volume only. At ε = 1e-9, δ = 1e-4 and δ = 1.34e-4
(`δ = √(2·c·Kε)` at c = 0.5 and 0.9), every `∪` cell and the
capped-minus-full `∖` cells (12 of 24 per δ) build at the closed-form
volume, and tier 3′ refuses each on its own contact records. Main and
the branch agree cell for cell. The same cells at δ = 1e-8 and 1e-6
fail 3′ too, where they build.

Witness: the reviewer's probe
(`rev5793r_probe.rs`, `bisector_scan`, env `PROBE_BIS`, `BIS_D`), which
prints `t3'=false` for those cells, against `bisector_nodes.rs`'s
poses. The test passes because it reads no tier 3′.

P2: a build that ships with records its own tier-3′ reading refuses,
in one witness family.

## Fix

Find which record 3′ refuses (a pierce of the cap's rim through the
other rod's wall at the node, `2δ` from the arc's ends, is the
likely one). Then fix the record or the reading, and have the test
assert tier 3′ on each body it builds.
