---
id: the-other-orbit-readings-refuse-a-tie-the-walk-now-reads
kind: issue
title: strut_anchor, held_cut/tied_held, on_arc and holds_whole still refuse a tie of two germs along one edge's two sides that walk_order and leads now read
status: open
opened: 2026-10-27
priority: P1
cost: M
refs: [a-box-corner-at-a-rings-tip-reads-two-crossing-germs-along-one-direction]
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

**The witnesses.** No ring pose reaches these sites with the pair: a
ring's two coincident edges share both their vertices, so germs along
them pair with one vertex, one plan, and two plans' cuts beside them
need two vertices of the ringed solid at one point. A pinch has them:
the bricks `[0, 1]² × [0.5, 1.5]` and `[−1, 0]² × [0.5, 1.5]` of
`union_flush_onto_edge_contact`, touching along the z-axis, against a
partner whose face at the pinch's lower end lies in `x = 0`, flush with
both bricks' faces there, the axis inside its corner. Both pairs'
cuts on the axis lie along one direction in one entry, each along one
of the pinch's two edges, each edge's flush flanker on its own side.

- Fans: the wedge from 90° to 200° or 250°, sheared along its 90° face
  toward −y, and the wedge from 270° to 20° sheared toward +y
  (`a_pinch_line_in_a_face_flush_with_both_bricks_builds_in_every_op`).
  On main the tie reached `held_cut` (`on_arc`) and `tied_held`, which
  read it by the runs' codes.
- Struts: the notched prism of
  `two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op`
  stood in `x = 0`, along +x or −x, at four notches
  (`two_dangling_null_edges_meeting_beside_the_pinch_line_build_in_every_op`).
  On main the tie reached `holds_whole` (`arc_holds`, through `nests`,
  `nested` and the holder search in `mint_directed`), `held_cut` and
  `tied_held`.

Every op of both builds on main as on the branch, at its closed-form
volume, tier 3 and 3′, at ε default, 1e-6 and 1e-12, but the notched
prism's `prism ∖ pinch`, which refuses `LineWedges` `Overlap` on a
line away from the pinch line, on main and branch alike (evidence added
to `lumps-meeting-four-sheets-on-a-line-refuse-line-wedges`). With the
change, `beside_order` decides every tie those poses meet, and
`tied_held` is no longer reached there.

`strut_anchor` meets no such tie: two struts' lower germs beside the
two sides of one line would put the struts on the two sides of it, so
one of them reaches the line last, not first; two beside one side read
alike and stay tied. Its refusal stays the invariant it states.

**The change.** A `Cut` carries the flankers of the edge of the other
solid it runs along, and `walks_before` reads the beside tie itself
(`beside_order`, now taking two cuts): `walk_order`, `leads`,
`segment`, `held_cut` (`on_arc`), `tied_held` (its far ends' tie too),
`holds_whole` (`arc_holds`) and `strut_anchor` ask that one order.
`walks_before_beside`, `beside_tie` and `Placed` are gone.
`walk_faces_first` hands `beside_order` to `strut_order` as its tie.
