---
id: a-kill-that-re-anchors-a-loops-first-leaves-its-rows-a-period-off-the-pass
kind: issue
title: A kill that re-anchors a minted loop's first leaves its rows a whole period off the rows the minting pass derives from the new first
status: dispatched
opened: 2026-09-30
priority: P3
cost: H
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

## Ruled

Ev, PR 4024, 2026-10-04, on the recommendation below (R): "this sounds
good!". Two designers had weighed it blind, then each was shown the
other's report, and they converged on R. The fallback is recorded here
as the alternative that was not taken.

## The final state

**The premise, as corrected.**
- A loop whose rows all stand one whole period over is not a defect: tier 3
  admits it, and `revert` relies on it. No reader depends on that gauge.
- The real defect is a `kef` merge of two loops lifted separately. That
  leaves a mid-loop jump, a latent `LoopDiscontinuity` (the mixed cases),
  which only the producers' closing re-mint hides.
- The text that gives `first` its row meaning (`entity.rs`, `revert.rs`,
  PR 2573) is agent-written.

**The final state (R).**
- A row is an **image** (the edge's certified chart curve, a function of
  the edge and the chart alone) plus the half-edge's **joint element**: the
  integer carrying its image onto the end of the half-edge before it, or a
  reset marker at a pole or apex.
- A loop's lift is derived by summing elements from `first`. Readers get one
  lift accessor, and `first` stays where that accessor starts, but no stored
  byte depends on it.
- **Every kill stays keys-only and exact.** The new joint's element is the
  sum of the two elements adjacent to the killed edge at that vertex,
  because the killed edge's halves share one image on one chart:
  - `kef`: e(prev(m)→m) + e(he→next(he));
  - `kemr`: one such sum per side;
  - `kev`'s strut: e(prev(he)→he) + e(m→next(m)).
- **"The pass's rows, byte for byte" becomes true at every door**, `revert`
  included (it inverts the elements), with no new refusal. The witness rows
  go green as written.
- **The site mint shrinks to the joints a door creates.** That closes
  `euler-site-mint-re-walks-the-rewired-loop-on-every-op` as a property.
- **Spline charts closed in u:** make the iso lane gauge-free too, or leave
  the drop there.
- **The link primitive should take the element** (`link(from, to, element)`)
  in the module that PR 3970's ruled part creates.

**Ratified text it changes:** one sentence of C4 (`crates/geom-brep/README.md`),
the seam's "two chart images (u = α and u = α + 2π)", which becomes "one
image, two joint elements". PR 4024 made that edit.

**Not taken: the fallback.** Keep `first` as
the lift origin and drop the byte contract for tier 3's "valid lift from
`first`":
- `kev` keeps the anchor unless its half dies;
- `kemr` keeps today's rule, with one residual (a wrapping parent split
  across sides of different winding);
- keys-only `kef` refuses on a complete periodic face, and its twin re-walks.

Before building, read the probe of `validate_pcurves` after each kill that
keeps rows (report on `analysis/probe/topo-kill-rows-tier3`). It measures
how many of today's mixed merges are tier-3-loud, which is the regression
the build has to remove.

## Evidence: the probe (2026-10-04)

The report is `probe-report.md` on `analysis/probe/topo-kill-rows-tier3`, base
`0edbbecb`. It measures complete periodic faces right after the outermost kill
door, on f64 bodies only.

**Mixed merges are tier-3-loud: confirmed.** In sweep's `ci` profile:

| door | (i) byte-equal | (ii) whole-loop shift | (iii) mixed | pass refuses |
|---|---|---|---|---|
| `kef` | 13, clean | 0 | 8, `LoopDiscontinuity` | 0 |
| `kef_minting` | 6,277, clean | 168, clean | 1,633, all `LoopDiscontinuity` (1,419 introduced by the kill, 214 already loud) | 1, tier-3 clean |
| `kev` | 592, clean | 0 | 0 | 0 |
| `kemr` | 73, clean | 0 | 0 | 0 |

- No mixed case is tier-3-clean.
- Of the 1,427 mixed faces the kill introduced:
  - 1,319 jump at the lift junction `c→b`;
  - **93 jump at an old closure joint left mid-loop by `kef`'s unconditional `first := next(m)`** (`euler_kill.rs:1625`), on wrapping loops;
  - 8 jump by a whole torus period in v;
  - 7 have an in-band lever (a sub-band-radius cylinder).
- `kev` met no wrapping loop. `kemr` met 3 faces with two wrapping loops each, all clean.

The build has to take every (iii) to (i). The 168 (ii) shifts become (i) under R
too, since no stored byte then depends on `first`.

**Off-question rows filed:**
- `the-pass-refuses-a-tier-3-clean-loop-anchored-past-a-pole-slit`: the pass and
  tier 3 read a zero-lever pole joint differently, so the pass's verdict depends
  on `first`. R's reset marker is where it is decided.
- `pin-branch-meters-the-joint-arm-as-a-length-and-skips-an-in-band-shift`.

**Sequencing:** build after PR 4029 (the `EulerOpError` conversion) merges.
Both touch `euler_kill.rs`.
