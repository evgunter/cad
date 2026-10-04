---
id: a-kill-that-re-anchors-a-loops-first-leaves-its-rows-a-period-off-the-pass
kind: issue
title: A kill that re-anchors a minted loop's first leaves its rows a whole period off the rows the minting pass derives from the new first
status: open
opened: 2026-09-30
priority: P3
cost: M
---

Found by the fix pass of `a-null-edge-that-is-killed-leaves-its-face-half-minted`
(branch `topo/null-kill-remint`), from its review's probe.

The minting pass pins each loop's branch on a periodic chart from the
loop's `Cycle::first` (`pcurves::mint_face`, through `walk_cycle` in
`crates/topo/src/pcurves.rs`), so the rows it derives for a loop depend
on which half-edge is `first`. The kills keep the rows they find
(`Neither` in `pcurves::staleness_posture`), and re-anchor a surviving
loop's `first` when the half that held it dies (`Body::kev`'s
`kev_execute`, `Body::kef`, `crates/topo/src/euler_kill.rs`; `Body::kemr`,
`crates/topo/src/euler_ring.rs`). So a kill whose dead half was a minted
loop's `first` leaves that loop's rows valid but, on a periodic chart,
possibly a whole period off the rows the pass derives from the new
`first`.

**Measured** (a probe after each `kev` in `boolean::rest::undo_struts`,
`crates/topo/src/boolean/rest.rs`, over sweep's `ci` profile; not
committed): 48 kills leave a minted face complete, and in every one the
strut half the `kev` removes was the loop's `first`. In 25 the rows are
still the pass's byte for byte; in 23, across six tests
(`m9_3_zip::two_peg_plate_union_is_exactly_additive`,
`m9_3_wall_door::declared_rest_two_peg_reaches_downstream_of_classification`,
`curved_mergedoor::consumed_side_of_the_pair_is_gone_and_one_record_ships` (now `consumed_side_of_the_pair_leaves_the_door_nothing_to_record`)
and three `r1_probes_m9_3` probes), every row of the loop differs from
the pass's in its image or interval — in the row inspected, the image
alone: `p0.x` is `2π` where the pass has `0`. The pass was run at the
boolean's own `Tol` and at `Tol::witness()` and gives the same answer
at both, so this is the branch, not the band. In 39 of the 48 every
row is still the operand's own, as its builder minted it. The review
measured the same at this unit's merge base.

Nothing reads the difference today: the REST lane's closing
`mint_pcurves` re-derives every row. It is a state in which a face's
stored rows are not the rows its door's contract names (the site mint's
rows are the pass's byte for byte), and a site mint that later re-mints
one loop of such a face from its `first` puts that loop on the pass's
branch and leaves the others where they are.

Shapes: a kill that moves a minted loop's `first` re-mints that loop
(it needs a band, as `a-kill-that-releases-a-loop-from-its-last-null-edge-leaves-its-gaps`
says of the kills), or the pass pins a loop's branch from something a
kill does not move. The first is local to the kills; the second changes
every minted row's derivation and wants its own measurement.

**Witness** (`crates/topo/tests/euler_site_pcurve_rows.rs`,
`a_kef_whose_dead_half_anchored_the_loop_keeps_the_pass_rows` and its
`kef_minting` twin, `#[ignore]`d until this row is settled): the minted
wall over `[4.2, 5.4]` (across `3π/2`, where the principal branch
jumps), split by a `mef_chord` up `u = 4.5` whose plus half is the old
loop's `first`, then `kef` of that chord from the new face. Both doors
keep rows whose image (`p0.x`: `−τ` kept, `0` derived) and certificate
residual differ from `mint_pcurves`' on every row of the loop.

**(b) does not close the class.** The pass needs an anchor per loop:
which half of a loop wrapping the period carries the jump, and which
period a loop that does not wrap sits in. An anchor read off the loop's
geometry (its least vertex, a fixed window) moves when a kill removes
the extremal piece, merges two loops lifted apart (`kef`) or splits one
(`kemr`). It also decides on the chart's seam, where a full turn's
vertices sit exactly. So (b) moves the class rather than removing it,
and adds an in-band decision. (a) it is.

**(a) is a fork, measured** (a probe after every public kill, over
topo's suite and sweep's `ci` profile; not committed). In sweep, of
the loops a kill re-anchors on a complete face of a periodic chart:

| door | re-anchored | rows ≠ the pass's |
|---|---|---|
| `kef_minting` | 4,146 | 1,770: 168 a whole shift, 1,589 mixed (part of the loop shifted, part not), 12 a torus's v period, 1 the pass refuses (`LoopNotClosed`) |
| `kef` | 16 | 8 (mixed) |
| `kev` | 140 (`boolean::rest::undo_struts`) | 0 |
| `kemr` | 37 (20 new rings) | 0 |

The `kev` measurement above does not reproduce at `d51af4c69`. Re-probed
over the six tests it names, comparing every row of the face, their 18
`kev`s on complete periodic faces move no anchor and leave every row the
pass's. Across the whole profile, the 140 that move one leave none
different. The class is live through `kef` and `kef_minting`.
`kef_minting`'s come from `merge_faces` (2,691), `boolean::zip` (1,362)
and `boolean::rest` (91). Every one of those producers re-mints after
the kill: `merge_coplanar_faces_declared` re-mints its staged result, and
the boolean runs its closing pass. In topo's suite,
`loop_reparenting_pcurve_rows::a_move_whose_rows_stand_carries_them_as_found`
pins `kef` and `kef_minting` carrying rows that differ from the pass's,
as found. The kills re-anchor `first` unconditionally (module docs of
`crates/topo/src/euler_kill.rs` and `euler_ring.rs`), so almost every
kill moves it.

The options:

- **A1.** The band doors (`kev_describing`, `kef_minting`, and a new
  `kemr_minting`) re-mint every loop whose `first` they move, through
  the site mint. The keys-only doors refuse `KeysOnly` where they would
  move the anchor of a loop on a complete periodic face, as `kef` and
  `ring_move` already do for rows that do not stand (PR 2527's ruling
  keeps the kills keys-only). Every production keys-only caller on such
  a face moves to a band door: `undo_struts`, `merge_faces`'s `kev`,
  `chord_join`, `revolve`, `step-import`, and blend. `kef_minting` then
  re-mints on about 4,000 calls per sweep `ci` run.
- **A2.** A1, with the kills keeping `first` wherever it survives (the
  unconditional re-anchor retired). The refusal narrows to a dead
  anchor, a merge (`kef`) and a split (`kemr`'s ring).
- **A3.** Only the band doors re-mint. The keys-only doors keep
  `Neither`, and the posture states the residue.
- **A4.** Retire the claim for the kills. A kill's rows are certified
  and branch-continuous; the producer's closing pass is what makes them
  the pass's, as the measurement shows every producer already does. The
  site-mint contract ("the pass's byte for byte") stays a claim about
  the minting doors.
