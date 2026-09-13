---
id: closed-chain-junctions-pair-with-a-rotated-link
kind: unit
title: blend: a closed chain of three or more links refuses ChainNotG1 because the junction list is rotated against the ring
status: closed
opened: 2026-09-08
branch: blend/7-closed-chain-junctions
pr: 2483
closed: 2026-09-13
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

## Landed (PR 2483, with the fix pass)

`battery::Junction { vertex, arriving, leaving }` — a chain's junction
carries the two links the walk found incident to it, as positions
into the chain's links (read-only accessors; the fields are
crate-private), and the G1 check reads those. The record's order is
walk order, kept so the record is the same chain from any seed; the
check reads each junction's own pair. The `debug_assert!` on incidence
is a tripwire in debug-assertion builds, not a pin.

What the rows pin: `closed_chain_junctions::every_junction_of_every_walked_chain_touches_both_its_links`
(and R1's/R2's twin rows) pin the RECORD and stay green under a check
that mis-reads a correct record; the carve rows
(`n_arc_discs_…`, `n_arc_pocket_floors_…`, `n_arc_bores_and_boss_feet_…`,
the reviewers' N-past-the-suite rows, and
`review_blend1_r1_probes::r1_a_three_arc_rim_carves_where_a_two_arc_rim_does`)
pin the READ. The oracle is `test_support::wedge_fill` (the 90° corner
at the meridian), not a second implementation. Both closed-rim doors
carve N = 2…6 crossings on both material sides; the spec's "three-arc
bore" is convex, the concave rims are a boss foot and a pocket floor.

The `arcs.len() == 2` premise: every revolve/boolean site is a fixture
fact stated at the site (`assert_full_revolve_rim`); the extruded
two-arc fixtures (`m5_pr12_refusals::tilted_rim`, the r1 rows) were the
premise that excluded the defect and now say so; `is_seam_vertex`'s
`[(p, q), second]` and `cap_incidence`'s `[_, _, _]` are vertex
invariants of chain ENDS, unexercised at N ≥ 3 because closed rims have
none; `surgery.rs`'s per-crossing `arcs.len() != 2` is a vertex
invariant holding at every crossing of an N-arc rim.

Filed from here: `rim-of-refuses-extruded-multi-arc-rims`,
`battery-holds-the-chain-data-model-beside-the-predicates`,
`self-closed-link-sharing-its-vertex-records-two-junctions`.

## Closed (2026-09-13, PR 2483)

Every junction of a closed chain is judged against the two links that
touch it (`battery::Junction`, read by index; the walk records it at
the step that meets it). Three-, four-, five- and six-arc rims carve on
both closed-rim doors and both material sides at `wedge_fill`'s closed
form with pad 0; every two-link rim and open chain is bit-identical to
the merge base. The dual found no MAJOR; its record-level findings
(the premise table, the rows' claims, the oracle copy, the fixture
copies) are in the fix pass. Residues filed:
`rim-of-refuses-extruded-multi-arc-rims`,
`battery-holds-the-chain-data-model-beside-the-predicates`,
`self-closed-link-sharing-its-vertex-records-two-junctions`.

Recorded at the delta: re-homing `m5_pr12_refusals::cylinder` and
`blend6_verb_vocab::cylinder` on `disc_of_arcs` moved those two
fixtures by bits (vertex angles now `2π·i/n`, the two-arc circle
starting at 0°); every row of both suites is green and neither is in
the bit-dump corpus, so nothing pinned moved — a later differential on
those fixtures should not read the change as a kernel move.
