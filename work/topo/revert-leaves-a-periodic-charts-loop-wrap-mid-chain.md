---
id: revert-leaves-a-periodic-charts-loop-wrap-mid-chain
kind: issue
title: Body::revert re-states plane rows with their frames but leaves a periodic chart's one-period loop wrap where the forward walk parked it, so tier 3 of a reverted body with such a loop reports LoopDiscontinuity
status: open
opened: 2026-09-14
refs: [revert-does-not-mirror-plane-chart-images, SHELL-9]
---

Found by the lane that closed `revert-does-not-mirror-plane-chart-images`
(2026-09-14), as the one thing left between `Body::revert`'s module
contract — "every certification survives the map" — and tier 3 of a
reverted body. With the plane images and rows mirrored, the drum's
reverted cavity reports exactly `NegativeVolume`
(`crates/sweep/tests/revert_plane_charts.rs`,
`reverted_drum_cavity_re_certifies_edge_for_edge_and_tier_3_reports_only_the_complement`).
The two-arc sphere's does not: `validate_geometric` of `revert()` of
its door-built cavity reports a pcurve `LoopDiscontinuity`
(`crates/sweep/tests/shell9_probe.rs`,
`sphere_reverted_cavity_and_the_grafted_loop`, whose docs say why —
the one-period `u` wrap the forward loop walk parked at the loop's
closure sits mid-chain once the loop runs the other way). Every
edge of that body re-certifies through the graft's meter; only the
walk's branch choice is stale.

`revert` carries every row key for key (`crates/topo/src/revert.rs`,
the map phase; `crates/topo/src/pcurves.rs`, the posture docs'
producer position for `revert`) and the stored rows are individually right — a row is a
function of the carrier parameter and a reversal does not touch it.
What the reversal does not re-state is the per-loop branch choice
`walk_loop` made in the forward direction (`crates/topo/src/pcurves.rs`,
`walk_loop` / `loop_closes`): the wrap is parked at the closure of a
walk that no longer runs that way. Today the producers' closing mints
re-derive every row (SHELL-9's convention), which is why nothing
downstream sees it; the contract at `revert.rs`'s "Validity class"
paragraph is still false of such a body.

Closing shape, undecided: either `revert` re-parks the wrap (a
reverse walk of each loop on a periodic chart, shifting the rows'
branches so the wrap sits at the reversed closure — a `shift_branch`
per row, no re-certification, and an involution only if the parking
rule is symmetric), or the loop-continuity pass accepts a one-period
wrap anywhere on a closed loop (it is the walk's convention, not
geometry), or the contract is narrowed to "every EDGE certification
survives; the pcurve map is re-derived by the producer". The middle
answer is TRIM's (`pcurves.rs`), the other two are this program's.

`revert.rs`'s "Validity class" paragraph now says exactly this
(2026-09-14, the fix pass of PR 2542): every edge certification and
every plane or curved ROW survives the map, and a periodic chart's
loop wrap is the one thing it does not re-state, naming this row as
the frontier. The posture docs in `pcurves.rs` call `revert`'s
position the producer position — a `&self -> Self` door outside the
guard's walk, not a fourth posture.
