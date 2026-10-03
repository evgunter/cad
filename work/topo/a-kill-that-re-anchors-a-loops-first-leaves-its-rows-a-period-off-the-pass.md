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
