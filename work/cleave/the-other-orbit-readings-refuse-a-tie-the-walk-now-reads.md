---
id: the-other-orbit-readings-refuse-a-tie-the-walk-now-reads
kind: issue
title: strut_anchor, held_cut/tied_held, on_arc and holds_whole still refuse a tie of two germs along one edge's two sides that walk_order and leads now read
status: closed
closed: 2026-11-01
opened: 2026-10-27
priority: P1
cost: M
refs: [a-box-corner-at-a-rings-tip-reads-two-crossing-germs-along-one-direction]
branch: cleave/orbit-readings-read-the-beside-tie
pr: 6184
---

Disclosed by PR 5812 (`cleave/flush-partners-at-a-slit-build`) and its
claims review.

## What

PR 5812 reads one more tie of a vertex orbit: two germs along one
direction in one sector entry, each along one of the other solid's
coincident edges there (a slit's or ring's), ordered by the side of
the line each edge's flanker flush with the entry lies on
(`boolean/insert.rs` `beside_order`, through `walks_before_beside`).
`walk_order` and `leads` read it, and so does the strut facing
(`walk_faces_first`). The other readings of the same order still read
the tie bare (`walks_before`, `insert.rs:1528`) and refuse or take
their own tie rule there:

- `strut_anchor` (`:2077`): "two dangling null edges in one corner
  start along one direction";
- `held_cut` and `tied_held` (`:1432`, `:1497`): a cut along an end
  germ's own direction;
- `on_arc` (`:1554`) and `holds_whole` (`:1099`): an arc's end at a
  tie.

None is reached by a known witness with such a pair: the ringed-plate
and keel probes of PRs 5545 and 5812 build or refuse elsewhere. A pose
with two plans' struts at one shared vertex, each along one of a
ring's two edges, would reach them.

## Owed

Read every orbit position through `walks_before_beside`, so one
reading orders a vertex's cuts everywhere, or show a site cannot meet
the pair, with a witness for each that can.


## Built (branch `cleave/orbit-readings-read-the-beside-tie`)

**The witnesses.** Existing slit tests reach `held_cut` with a tie
`beside_order` decides, both signs (counted by PR 6184's claims
review: 599, 179 and 149 readings):
`a_keel_on_the_floor_subtracts_as_a_slit::a_flush_partner_crossing_the_slit_intersects_it_in_either_order`
(its zero-area ring closed at both ends too),
`::a_face_glued_to_the_slit_floor_merges_its_two_edges` and
`::a_slab_flush_with_the_slit_floor_intersects_it_keeping_the_keel`.
Two new pins reach it at a pinch: the bricks `[0, 1]² × [0.5, 1.5]`
and `[−1, 0]² × [0.5, 1.5]` touching along the z-axis, against a
partner whose face at the pinch's lower end lies in `x = 0`, flush
with both bricks' faces there:

- fans: wedges 90°–200°, 90°–250° and 270°–20°, sheared along that face
  (`union_flush_onto_edge_contact::a_pinch_line_in_a_face_flush_with_both_bricks_builds_in_every_op`),
  reaching `held_cut` (`on_arc`);
- struts: the notched prism stood in `x = 0`, along +x or −x, at four
  notches
  (`::two_dangling_null_edges_meeting_beside_the_pinch_line_build_in_all_but_one_op`),
  reaching `held_cut` and `holds_whole` (through `nests`, `nested` and
  `mint_directed`'s holder search).

On main the same ties reached `tied_held`, which reads them by the
runs' codes. Every op of the pins builds alike on main and the branch,
at its closed-form volume, tier 3 and 3′, at ε default, 1e-6 and
1e-12, but the notched prism's `prism ∖ pinch`, which refuses
`LineWedges` `Overlap` on a line away from the pinch line on both
(evidence added to `lumps-meeting-four-sheets-on-a-line-refuse-line-wedges`).
The pins hold the direction: `beside_order`'s sign turned round at the
new readers alone turns both red (`SharedVertexCrossings`), and the
three slit tests above. Over the whole topo suite, every held cut a
tie's reading decides reads the same by the runs' codes (1015 of 1015;
`work/honeone/germ-orders-about-a-normal-have-two-homes.md`).

`strut_anchor` meets no tie anywhere in the topo suite. Its doc argues
why two struts hung at one vertex cannot start along one direction
whether or not the tie's reading orders them, and it refuses any
such tie. `beside_order` reads two flankers on one side apart too,
wherever their offsets from the line differ; it reads `Zero` only at
one offset.

`tied_held`'s far-ends test reads one arc only where the far ends tie
unread; far ends the tie's reading orders are nested runs, which no
pose reaches, and refuse `ClassificationInvariant` rather than answer.

**The change.**
- `insert::cut_order` is the orbit's one position order (`CutOrder::
  Apart` or `Tied(Sign)`), the one place a tie is read further
  (`beside_order`, which now carries a `Sign`); `walks_before` is its
  `Option<bool>`. `walk_order`, `leads`/`segment`, `held_cut`
  (`on_arc`), `tied_held`, `holds_whole` (`arc_holds`), `strut_anchor`
  and `walk_faces_first` read it.
- `walk_faces_first` asks `cut_order` before the strut order where the
  tie's reading can read the two (`Cut::tie_reads`), as main's order
  did; every other pair reads the strut order alone.
- A `Cut` carries its flankers; a `Germ` embeds its `Cut`; each solid's
  cuts are built once in `plan_null_pairs`. `strut_faces_first` takes
  `(Cut, Cells)`; `vtxfac`'s pierce germs pass `Cut::apart` (a pierce's
  germs run through no edge of the pierced solid).
- `walks_before_beside`, `beside_tie` and `Placed` are gone.

## Closed

2026-11-01, PR 6184. Every position reader of a vertex orbit reads
one order, `insert::cut_order`, whose tie arm reads the sides two cuts
along one direction lie beside. The readers the row named are reached
with such a tie by the three slit tests above and by the two pinch
witnesses, which pin its direction; `strut_anchor` and `tied_held`'s
far ends are not reached, and refuse a tie read apart there.
