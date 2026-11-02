---
id: the-pierce-reads-a-struts-facing-the-stars-site-already-names
kind: issue
title: vtxfac's piercing runs read a strut's facing geometrically (strut_faces_first) where the star's site names it topologically
status: open
opened: 2026-11-02
priority: P1
cost: M
---

Found by `run-loops-of-split-and-pierce-are-twins` stage 1 (`onehome/star-sites`).

## What

`topo::star` resolves a run's site topologically, and its strut carries a tip that names its facing (`star::Tip`). A run of duplicates alone gets the strut whose tip stands for the arc, so `he_plus` faces the start. A run holding the whole orbit gets the strut whose tip stands for the rest, so `he_plus` faces the end. The split mints exactly that.

The Boolean's pierce (`boolean/vtxfac.rs`, `classify_vertex_on_face`'s run loop) still re-reads the facing of the same strut geometrically through `boolean::insert::strut_faces_first`. It then rebuilds the site with `star::strut(corner, Tip::facing(..))`. That is two readings of one fact.

## Evidence that they agree

An instrumented run compared the two readings at every pierce strut: the topo suite plus the first 1740 tests of the sweep suite, 1 863 struts in all (`Arc`/`true` and `Rest`/`false`). They agreed at every one.

They agree by construction too. A strut's two germs lie in different entries of its corner's sector. `walk_faces_first` then orders them by entry, which is the orbit order the star's tip reads. `strut_facing`'s edge votes can only name the half beside the edge the germ runs along, which is the same half the orbit order names. A start germ along the departure edge would need the run to start after its own end.

## Owed

Decide whether the pierce drops its geometric read and takes the star's tip. The one verdict that would move is `strut_faces_first`'s `ClosedEdge` refusal (`CurvedBooleanUnsupported` at a closed edge's lone vertex with a germ along it). No row reaches that pose, and the topological tip would build there. That needs a row reaching it, with an oracle independent of both readings, before the refusal goes. The vertex–vertex planner's struts are different: their germs can share one entry, where only the geometric read can order them. They keep `strut_faces_first`.
