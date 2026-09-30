---
id: a-kill-that-releases-a-loop-from-its-last-null-edge-leaves-its-gaps
kind: issue
title: A kill that takes the last null edge off a minted loop leaves the rows the loop missed while it was held open
status: open
opened: 2026-09-30
priority: P3
cost: M
---

Found by `a-null-edge-that-is-killed-leaves-its-face-half-minted`
(branch `topo/null-kill-remint`), whose measurement it is.

That unit made the site mint (`crates/topo/src/pcurves.rs`,
`StoredRows::remints`, `site_rows`) take a minted face whose only gaps
are on loops a null edge holds open, or that its door takes the last
null edge off, and mint whole every loop its door rewires and leaves
running through no null edge. The doors that release a
loop that way are the Euler operators that rewire it (`mev`, `mef`,
`mekr`) and a null edge's first description (`Body::set_edge_curve`).
The kills do not: `Body::kemr` (`crates/topo/src/euler_ring.rs`),
`Body::kef` and `Body::kev` (`crates/topo/src/euler_kill.rs`) run no
site mint and take no `Tol` to mint with. So a kill that takes the last
null edge off a loop leaves the loop missing the rows of any half an
operator added to it while it was held open — a face half-minted by a
door the posture table declares `Neither`.

**Measured** (probes on every null-edge kill, not committed; topo's
suite and sweep's `ci` profile): of 6,385 null-edge kills in sweep
(`kef` 4,142 and `kemr` 1,914 in `chord_join::ChordJoiner::cut_core`,
`kev` 328 in `boolean::rest::undo_struts`, one test `kev`), one
releases a minted loop with gaps:
`bool1_fix_pass::conic_section_boundary_restates_on_its_own_carrier`,
where the section polygon lies on the annulus face itself, so the
cut's `kemr` leaves that face missing two chord rows on loops no null
edge holds. It is transient there: the split's `mfkrh`
(`splitting::finish`) promotes the ring the next step and the face is
complete. Every other kill leaves its faces dead, on a chart that
mints nothing, unminted, complete (`undo_struts`' 48), or still held
open by another null edge. None in topo's suite, whose operands carry
no rows.

Public-door witness, on `topo::null`'s `ruling_cut` fixture: hang the
null strut, run the `mef` along the ruling (the held piece now misses
the strut's two rows and its chord half), then `kev` the strut — the
held piece is left missing the chord half's row on a loop no null edge
holds.

Shape: the kill's plan runs the site mint over the loop(s) it releases
(the killed halves are what blocked the walk). It needs a band, so
either the kills grow a `Tol` (a signature change on three core
doors, with `kev`'s fan-merge door `kev_describing` beside them) or the
cut in `chord_join` re-mints after its kill, which is a second
spelling. Settle with `topo/kill-loop-anchor-proof`'s changes to the
kill plans in view.

**`kef` has a band twin now** (2026-09-30, PR 3531,
`loop-reparenting-doors-drop-rows-they-could-now-re-mint-under-decide`):
`kef` stays keys-only, per PR 2527's `kev` ruling, and
`kef_minting(he, tol)` runs the site mint over the surviving face when
its remnant's rows do not stand there — the "grow a `Tol`" shape above,
as a second door rather than a signature change. Neither runs it where
the remnant's rows stand, so a kill of a null edge whose remnant
carries its rows still leaves a released loop's gaps; `kemr` and `kev`
have no band door for it.
