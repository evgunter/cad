---
id: coplanar-sectors-read-touching-at-a-short-arm-where-their-bounds-part-at-their-reach
kind: issue
title: pair_search reads two coplanar sectors touching where their bounds part at their own reach, levered at a short arc's arm
status: dispatched
priority: P2
cost: M
opened: 2026-10-28
---

Found by the sweep of the crescent short-arc lane (branch
`splitdoor/crescent-short-arcs`), sibling of
`a-crescent-refusal-fires-where-short-arcs-carry-the-section`.

## What

`boolean/sectors.rs` `sector_overlap` reads whether two coplanar
sectors overlap from `within` (`within_signs`: each bound's cross with
the partner's bound, levered at the sector's `arm`) and `parallel_same`
of the bound pairs at `a.arm.min(b.arm)`. The arm is the sector's
shorter chord, or the bisector arm of a split corner, so a short edge
anywhere at the corner shortens the lever of every bound there.

Measured on the pose of `a_closed_conic_in_the_partners_face.rs`
`short_arcs_at_a_crescents_tip` (`both, s = 1e-5`, default ε): at the
vertex pair where the short arcs end, the floor's and the top face's
half-sectors are each a long arc (reach `0.6`, `0.2`) and a bisector
(arm `6e-6`); the long arcs leave the pair `2e-5` rad apart, a wedge
`4e-6` m wide at their reach, and `pair_search` reads the pair
touching. Nothing ships wrong there: the section reaches the wedge
along the coincident short arcs, and `crescent` now reads it so
(`coincident_run`). Where no such run carries the section, the pair is
a first-order overlap the sector passes never record, and only the
crescent refusal stands between it and a union that keeps both faces.
No pose of that shape (two bounds that part at their reach, read
touching at a short arm, with no run) has been built.

## The same shape elsewhere, unmeasured

Two directions read at a corner, levered at a sector's arm or the
shorter of two, where each direction's own reach is known:

- `boolean/recl.rs` ray-event grouping (`parallel_same_dir` at
  `mentions[i].arm.min(mentions[j].arm)`), and `bool_ee_collinear`
  (2026-10-29: now `sectors::parallel_same` passed `RAY_COLLINEAR`,
  PR 5983; same arm);
- `boolean/insert.rs` `bool_shared_cut_order` and `strut_order` at
  `s.arm`;
- `census.rs` `strictly_inside` (tier 3's line wedge, scaled to the
  wedge's arm) and `sector_shape.rs`'s reflex and straight readings of
  a corner's own two bounds, which its module docs fix as the shared
  convention.

## Further sites (PR 5949's review), dispositions

- `boolean/recl.rs` `recl_sectors`: its `arm` (the pair's shorter) goes
  to `sectors::runs_into`, which reads a kept edge's direction against a
  dropped sector's bound with `parallel_same` at that arm, though both
  carry their own reach: **the same shape**, unmeasured. A Zero there
  records a fate edge running into the dropped face. The same `arm` goes
  to `require_same`, which reads the faces' relation from their normals:
  not the shape.
- `boolean/recl.rs` `resolve_edge_edge`: `bool_dir_same` at the
  flankers' shorter arm reads the sense of a direction already read on
  the flanking plane, a cosine near ±1 that no lever blurs: not the
  shape.
- `boolean/recl.rs` `resolve_edge_sector`: the arm goes to the tangent
  side reading, which reads each bound over its own reach
  (`reached_bounds`) and only the bisector at the arm (the bisector's
  lever is `work/contacthold/a-glued-tangencys-bisector-reads-its-curving-at-its-sectors-short-arm.md`),
  and to `require_same` (normals): not the shape.
- `boolean/insert.rs` `germ_dir` / `bool_germ_line`: the two faces'
  normals' cross at the pair's shorter arm, `pair_search`'s own margin
  read again. Normals, no per-bound reach: not the shape, though the
  lever is the shorter sector's.
- `boolean/insert.rs` `beside_order` (`bool_beside_flush`) and
  `walks_after` (`bool_shared_cut_order`): a flanker's unit direction
  off the sector's plane, and the order of two germs' second-order
  offsets, levered at the sector's arm though each flanker is an edge
  with its own reach: **the same shape**, unmeasured; both refuse in
  band rather than read a side.

