---
id: the-pierce-reads-a-struts-facing-the-stars-site-already-names
kind: issue
title: vtxfac's piercing runs read a strut's facing geometrically (strut_faces_first) where the star's site names it topologically
status: closed
opened: 2026-11-02
priority: P1
cost: M
closed: 2026-11-15
pr: 7008
branch: onehome/pierce-strut-facing
---

Found by `run-loops-of-split-and-pierce-are-twins` stage 1 (`onehome/star-sites`).

## What

`topo::star` resolves a run's site topologically, and its strut carries a tip that names its facing (`star::Tip`). A run of duplicates alone gets the strut whose tip stands for the arc, so `he_plus` faces the start. A run holding the whole orbit gets the strut whose tip stands for the rest, so `he_plus` faces the end. The split mints exactly that.

The Boolean's pierce (`boolean/vtxfac.rs`, `classify_vertex_on_face`'s run loop) still re-reads the facing of the same strut geometrically through `boolean::insert::strut_faces_first`. Both readings now stand, and the pierce refuses typed (`ClassificationInvariant`, naming both) where a definite geometric answer contradicts the site's `star::TipSide`. That is two readings of one fact, kept in step by a check.

## Evidence that they agree

A one-off instrumented run, since removed, compared the two readings at every pierce strut over the topo suite and the first 1 740 tests of the sweep suite. That was 1 863 struts: 1 431 with the tip on the run's side and the geometric read `true`, and 432 with the tip on the other side and the read `false`. They agreed at every one. The typed check guards this agreement from PR 6392 on: any pierce row that reached a contradiction would refuse.

They agree by construction too. A strut's two germs lie in different entries of its corner's sector. `walk_faces_first` then orders them by entry, which is the orbit order the star's tip reads. `strut_facing`'s edge votes can only name the half beside the edge the germ runs along, which is the same half the orbit order names. A start germ along the departure edge would need the run to start after its own end.

## Owed

Decide whether the pierce drops its geometric read and the check, and takes the star's tip alone. The one verdict that would move is `strut_faces_first`'s `ClosedEdge` refusal (`CurvedBooleanUnsupported` at a closed edge's lone vertex with a germ along it). No row reaches that pose, and the topological tip would build there. That needs a row reaching it, with an oracle independent of both readings, before the refusal goes. The vertex–vertex planner's struts are different: their germs can share one entry, where only the geometric read can order them. They keep `strut_faces_first`.

## Closed (2026-11-15)

The pierce takes the star's tip alone. `vtxfac`'s run loop no longer calls
`strut_faces_first`, and the `ClassificationInvariant` that held the two
readings in step is gone with it. The strut is minted by `site.tip()`, which
`facing_at` already carried.

The one verdict the row named stays. A closed edge's lone vertex with a germ
along it still refuses `CurvedBooleanUnsupported`, now asked structurally by
`insert::strut_at_closed_edge`, which reads `strut_facing`'s own closed-edge
rule. No row reaches that pose: a probe with a full-revolve cylinder whose rim
circle lies exactly on a ball, under all three ops in both orders, read no
pierce strut at all. So no answer there is checked yet, and the refusal waits
for a row that reaches it with an oracle independent of the tip, as this row
asked.

The vertex–vertex planner keeps `strut_faces_first`. `Cut::apart`, which only
the pierce's read built, is now test-only.

## Note (2026-11-16, PR 7008's review)

The geometric read the pierce no longer takes carried refusal arms of its
own, and they left the pierce with it: `strut_facing`'s contradictory-votes
`ClassificationInvariant` (`crates/topo/src/boolean/insert.rs:2381`) and
`walk_faces_first`'s coincidence and tie refusals. The reviewer counted 0
of them fired across the 241 strut reads. The follow-up PR makes
`strut_at_closed_edge` propagate `strut_facing`'s refusal instead of
swallowing it, so the contradictory-votes arm is read at the pierce again
(fail loud); the walk's refusals stay out. The follow-ups are
`two-spellings-of-a-wrap-edges-tip`,
`the-planner-walks-a-closed-edge-pose-the-pierce-refuses` and
`a-strut-pose-row-with-an-oracle-independent-of-the-tip`.

