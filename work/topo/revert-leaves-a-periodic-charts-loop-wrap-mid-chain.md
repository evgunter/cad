---
id: revert-leaves-a-periodic-charts-loop-wrap-mid-chain
kind: unit
title: Body::revert re-states plane rows with their frames but leaves a periodic chart's one-period loop wrap where the forward walk parked it, so tier 3 of a reverted body with such a loop reports LoopDiscontinuity
status: dispatched
opened: 2026-09-14
refs: [revert-does-not-mirror-plane-chart-images, SHELL-9]
branch: topo/revert-reparks-the-wrap
pr: 2573
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

## Brief (TOPO, 2026-09-14) — block TOPO-B4 slot 1, dual at review

**The answer to give.** `validate_geometric(&body.revert())` reports no
pcurve `LoopDiscontinuity` on a body whose curved faces carry rows on
a periodic chart, and `revert ∘ revert` is the identity on those rows
bit for bit. The forward loop walk parks a periodic chart's one-period
wrap at the loop's closure; after the reversal the loop runs the other
way and the wrap sits mid-chain. Phase 1 decides between the two
closings the row names and says why: (a) `revert` re-parks the wrap —
a reverse walk of each loop on a periodic chart, shifting the rows'
branches so the wrap sits at the reversed closure (a `shift_branch`
per row through the affine door the plane mirror uses, no
re-certification — and an involution only if the parking rule is
itself reversible: prove it or measure it); (b) `revert` drops the
rows of every periodic-chart face and states the posture (the
`Neither`/producer position `pcurves.rs` already records for it),
leaving the closing mint to re-derive them — honest, loud, and the
weaker answer. (a) is the answer the row wants; (b) is acceptable only
with the involution shown to fail under (a).

**Rows.** Red-first: the two-arc sphere's door-built cavity
(`shell9_probe`'s `sphere_reverted_cavity_and_the_grafted_loop`)
— `LoopDiscontinuity` on the merge base, `[NegativeVolume]` only at
the head; a torus wall the same way; the involution on both. The
drum's plane-only rows stay byte-identical (the plane mirror's rows).
Control: a non-periodic curved chart's rows are untouched.

**Receipt.** Every reader of a loop's branch parking (`walk_loop`,
`loop_closes`, the seam-aware certifiers) with whether it assumes the
forward direction; every producer's closing mint that re-derives the
wrap (SHELL-9's convention) and whether it still needs to.
`crates/topo/src/pcurves.rs` is TRIM's — `walk_loop`/`loop_closes` are
read, and a reverse-parking helper, if (a) needs one, lands there by
announced seam (announce on `work/trim/log.md` in the PR).

Branch `topo/revert-reparks-the-wrap`. PR title: "TOPO: revert re-parks
a periodic chart's wrap at the reversed closure". Do not close the
item; the dual runs at review.

## Built (TOPO lane, 2026-09-14; branch `topo/revert-reparks-the-wrap`)

Phase 1, against the brief's two closings. Measured (the recovered
probe, extended): the finding sits on the SECOND half-edge of the
reversed cycle — the source's `prev(first)` — on the two-arc sphere,
its cavity, and a cone through its apex; the tube torus, a two-arc
torus, the drum, the collinear-cap drum and its cavity, and a half
drum report nothing but `NegativeVolume`. Why: the forward walk parks
a wrap only at the closure, so with `first` fixed the reversal puts
that joint right after `first`; and a loop wraps at closure only when
it has an azimuth-free joint (a pole, an apex) where the walk skips
the shift — a torus has none, a full rim absorbs a drum's azimuth.
The arithmetic re-park of closing (a) — `shift_branch` per row — is
NOT a bitwise involution: `(x + τ) − τ` is `x` for the walk's own
azimuths (`0`, `π`, `τ`: 40 rows measured, 0 broken) and is not for
`1 + ε`, so a body with a finer azimuth would round-trip to different
bits. Neither (a) as written nor (b) was taken: `revert` moves each
curved loop's `Cycle::first` to its source predecessor, which is the
same joint at the reversed closure with no row touched — exact
structure, `prev`/`next` swap under the map, so the second reversal's
`prev` of the moved anchor is the source's `next` of it, and the
involution is proved rather than measured. Plane loops keep their
anchor (no period, no wrap; the planar battery's bits stay).

Rows: `sweep`'s `revert_periodic_wrap` (sphere and cavity, apex cone
— red on the merge base with `LoopDiscontinuity`; the six
non-wrapping periodic controls; the twisted loft as the non-periodic
control; each pins the anchor move, rows bit for bit, the wrap at the
reversed closure, tier 3 exactly `NegativeVolume`, and the
involution), and `revert::tests`' anchor row on `ops_cube` and a
lone plane face. Seams: one prose paragraph in TRIM's `pcurves.rs`
and one clause in SHELL's `shell.rs`, announced on both logs.

## Fix pass (TOPO, 2026-09-14; two blinded reviews, both MERGEABLE-AFTER-FIXES)

The rule is now ONE rule: every loop's `first` moves to its source
predecessor, plane loops included (a plane chart has no period, so
the move is a no-op in meaning; the partition's only observable
consequence was the assertion written to observe it — R1 re-anchored
every loop and the planar battery and the census stayed green). The
invariant the anchor now carries is stated at the type
(`crates/topo/src/entity.rs`, `LoopBoundary::Cycle`'s `first`) and
in `transform.rs`'s orientation-reversing tripwire; `revert.rs`'s
anchor bullet is the invariant and a pointer.

The argument for the mechanism NOT taken lives here, not in the
module header. The brief's closing (a) as written — a `shift_branch`
per row — re-parks the wrap by arithmetic on the stored rows, and
`revert ∘ revert` is then bit-identical only where the addition is
exact: `(x + τ) − τ` is `x` for the walk's own azimuths on this tree
(`0`, `π`, `τ`; 40 rows measured, 0 broken) and is not for
`x = 1 + ε`, so a body with a finer azimuth would round-trip to
different bits, and `revert`'s D9 involution is the rule's, not the
fixtures'. Moving the anchor is (a)'s answer by exact structure:
`prev`/`next` swap under the map, so the moved anchor's `prev` in
the result is its `next` in the source, and the second reversal lands
on the source anchor with no arithmetic anywhere.

The census golden's first account was false: four of `voided_rod`'s
40 verdicts changed SIGN (`props_rim_side ×2`, `props_rim_dir_group
×2`, both anchor-relative by construction in `props/curved.rs`), 36
moved; the PR body carries the corrected account and the suite now
carries the sorted-multiset row beside the golden. Filed:
`work/tint/census-verdict-golden-hashes-an-anchor-ordered-stream.md`,
`work/props/rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order.md`.
