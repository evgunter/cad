---
id: a-keys-only-door-drops-a-loops-rows-across-a-sense-flip-on-a-chart-with-poles
kind: issue
title: A keys-only door cannot restate a pole turn: it drops the moved loop's rows
status: open
opened: 2026-11-12
---


Found by `onedoor/three-walks` (2026-11-12), implementing the ruling
on PR 6851 that a door moving a loop between faces of opposite sense
restates its singular-set elements.

A joint on a chart's singular set stores the turn on the face's side
(`crates/topo/src/joint.rs`, "The singular set"), so it changes when
the loop's face changes sense. Which joints are on the set is a
decision at a band (`topo::pcurves`' `singular_at`, through
`geom_brep::chart_singularity::vertex_at_singular_point`), and the
stored element carries no mark of it. So a door with no band cannot
restate, and none does:

- `Body::sides_agree` (`crates/topo/src/euler_ring.rs`) says whether
  a loop's rows stand across a move: one sense, or a chart with no
  singular set. Where they do not, `kfmrh`, `ring_move`, `mfkrh` and
  `kef` treat the rows as rows about another chart
  (`Body::plan_moved_rows`): the `_minting` door decides them again,
  and the keys-only door refuses `SiteRowRefusal::KeysOnly` where the
  band door would write, and otherwise drops them.
- `set_face_sense` (`crates/topo/src/attach.rs`) has no band door. On a
  sphere, cone or spindle face whose bit changes it drops the face's
  rows, which is loud at rest (`Unminted`) until the face is minted
  again.
- The three callers that promoted a ring with `mfkrh(Inherit)` and then
  set the host's bit back (`shell.rs`, `chord_join.rs`,
  `boolean/slit.rs`) now promote it as the host's piece in one step
  (`Body::mfkrh_piece`), so the sense never flips and the rows stand.

The open question is whether `set_face_sense` should grow a band door
that decides the face's joints again in place, in the shape of the
other `_minting` doors. No production caller needs one today.
