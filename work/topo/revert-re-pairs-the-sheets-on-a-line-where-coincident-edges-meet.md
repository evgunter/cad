---
id: revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet
kind: issue
title: Body::revert refuses a body with coincident edges on one line: build the re-pairing the complement's material sectors decide
status: open
opened: 2026-10-14
priority: P2
cost: H
blocked_on: [4795]
refs: [reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier]
---

## What

`Body::revert` (`crates/topo/src/revert.rs`, `coincident_edges`)
refuses `RevertError::CoincidentEdges` on any body where two distinct
edges run between the same two point keys on one line or one stored
curve. ∖ passes the refusal on as `BooleanError::Revert` (the seamed
finish, `boolean/finish.rs`; the single-operand fallback,
`boolean/ops.rs` `finish_fallback`; the void door,
`boolean/voids.rs` `insert_hollow_voids`, as
`VoidInsertError::Revert`). That is a construction gap on valid input:
the split's one-end slit is a legal tier-3′ body, and subtracting it
where its slit is kept refuses.

## Owed

The re-pairing, as an Euler-operator construction inside `revert`:
where 2k sheets meet on a line, pair them by the complement's material
sectors. The slit (PR 4622's one-end form, pinned in
`crates/topo/tests/a_doubled_line_does_not_revert.rs`) worked in
closed form:

- Sheets round the keel: floor either side, wall a, wall b. The slit
  body's material sectors are (floor, wall a) and (wall b, floor),
  each `atan2(2, 0.6)` ≈ 73.3°; its two edges a and b bound them.
- The complement's sectors are (wall a, wall b), `2·atan2(0.6, 2)` ≈
  33.4°, and the half-space under the floor, π. So the complement has
  one crease edge wall a | wall b and the floor running flat across
  the line: the glued tip, an edge lying in a face.
- The construction: drop the slit from the floor's loop, join a and b
  into one edge between the walls, and join the open end's two vertex
  copies (one point key) into one vertex, whose link then crosses the
  keel's line twice — once along the new edge, once through the floor.
- The result carries an edge-on-face contact (the new edge in the
  floor), which `revert`'s signature cannot hand back today: it returns
  a body and no records. The door that reverts owes the record.

The pinch (two solids touching along a line, its ends on shared point
keys) re-pairs across the solids: bricks in quadrants 1 and 3 complement
to quadrants 2 and 4, each a crease between one brick's face and the
other's. `B − pinch` in `crates/topo/tests/pierce_strut_at_a_pinch.rs`
(bare and reflex) and `cutter ∖ pinch` in
`crates/topo/tests/union_flush_onto_edge_contact.rs` pin the refusal
until this lands; their answers are the re-paired bodies.

Two kissing cusps (D1's doubled cusp) re-pair as two seams; see the
`[ev]` PR 4795 body, report A of round 2, and its question (A) on D1's
"legal together or not at all".

Blocked on 4795: the pairing rule is that PR's principle, and a ruling
for radial edges would change what the complement is.

## What today's refusals read

- `Body::revert` and the void door read structurally, deciding nothing
  (the void door's contract): two line edges on the same two point
  keys (`census::shared_segments`). A pair a record backs, or a partial
  overlap, reaches their naive map unrefused.
- ∖ reads every collinear overlap of its operand's kept part
  (`census::collinear_overlaps`), partial overlaps included, and
  refuses where the result's wedges on the line overlap
  (`census::line_wedges`). The pinch of unequal bricks (heights 1.5 and
  1.25, a partial overlap) is pinned refused there.
- Coincident curved edges are outside both readers, as they are
  outside the census's exact sweeps; PR 4622's curved keel slit is the
  known instance, unmeasured. A curve key shared by two edges, which a
  certified curve's own parameter span would make one arc, is minted
  by no door.
- An edge lying in a face and a face–face line touch: measured for the
  edge in a face,
  `subtracting-a-glued-tip-reverts-its-edge-on-face-line-into-the-rider-form`.

The re-pairing retires the refusals and these with them.

## Tests that return with the re-pairing

- `crates/editor-core/tests/union_pinch_member_order.rs`
  `a_slab_holding_the_contact_refuses_to_be_cut`: on main it built
  `slab ∖ blocks` with the contact as two 270° edges (overlap 180°) and
  ran six ops of that slab against the plate. It now pins the refusal;
  the six ops come back once the slab builds right.
- `crates/topo/tests/records_cite_their_decision.rs`
  `cutting_a_touching_pair_out_of_a_slab_refuses_its_revert`: on main it
  cut the same slab and read a carried edge-edge record's lineage at a
  plate's pierce (`a_carried_edge_edge_record_split_at_a_pierce_names_its_inputs_cells`).
  It now pins the refusal; the lineage row comes back with the re-pairing.
