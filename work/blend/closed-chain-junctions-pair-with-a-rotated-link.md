---
id: closed-chain-junctions-pair-with-a-rotated-link
kind: issue
title: blend: a closed chain of three or more links refuses ChainNotG1 because the junction list is rotated against the ring
status: open
opened: 2026-09-08
---


## Finding (unit 1's style review, PR 2123, frozen head `c3b2c9f0`)

`crates/sweep/src/blend/battery.rs` `walk_chains` builds a chain's
junction list as `joints_back.reverse() ++ joints_fwd` (`:1276`–`:1277`).
For a CLOSED walk that list reads `[closing vertex, j01, j12, …]`, while
the G1 check (`:1408`–`:1412`) pairs `junctions[i]` with
`ring[i], ring[i+1]` — so every junction is judged against one link that
does not touch it, and `pick` (`:1425`–`:1439`) then reads that link's
far-end tangent. A two-link closed rim is immune (both links touch both
vertices), and that is the only closed-rim shape any suite builds
(`fillet_h5_hostless_rim.rs:191, :351`, `blend_seam_split_rim.rs:255,
:334, :401` all assert `arcs.len() == 2`).

Measured: the PRISTINE three-arc cylinder's whole raised rim (a circle
authored as three arcs, extruded) refuses `ChainNotG1` with margin
0.75 at arm 0.866 (`sin 120° · chord`) through both `run_battery` and
`fillet_edges`; the two-semicircle rim of the same body builds. Witness
row, adopted on PR 2123 as a characterization row that goes red when
this lands:
`crates/sweep/tests/review_blend1_r1_probes.rs::r1_a_three_arc_rim_refuses_chain_g1_at_a_junction_where_a_two_arc_rim_builds`.

## Fix shape

Pair each junction with the two links that are INCIDENT to it (read
off the walk, not off position), or rotate the closed chain's junction
list so `junctions[i]` sits between `ring[i]` and `ring[i+1]` and say
which convention the walk keeps. Rows: the three-arc rim carves at the
plane–cylinder closed form on both material sides; an N-arc rim for
N ≥ 3 in the annulus door (does the seam-split walk see three
crossings?); the existing two-arc rows bit-identical. The `arcs.len()
== 2` premise across the closed-rim suites is the class to sweep.

## Slot

Block BLEND-B1 slot 1 (reordered ahead of
`ladder-rim-phase-may-retire-a-new-split-key`, 2026-09-08 — a pristine
three-arc rim refusing is a user-facing defect; the ladder orientation
is unmeasured). Spec `docs/BLEND-7-SPEC.md`, to be written; pre-draw
fields logged there before dispatch.
