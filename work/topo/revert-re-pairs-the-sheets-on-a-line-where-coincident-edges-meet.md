---
id: revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet
kind: issue
title: Body::revert refuses a body with coincident edges on one line: build the re-pairing the complement's material sectors decide
status: open
opened: 2026-10-14
priority: P2
cost: H
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
  overlap, reaches their naive map unrefused. Measured at the void
  door: `insert_voids(box, [s, s], cavity)`, the cavity the pinch of
  unequal bricks below, grafts the touch as two 270° edges whose
  wedges overlap by 180°. Tier 3 passes the result, and only tier 3′
  refuses it, for want of records, not for the wedges.
- ∖ reads every collinear overlap among the edges of its operand that
  its result keeps (`census::collinear_overlaps`), partial overlaps
  included, and refuses where the result's wedges on the line overlap
  (`census::line_wedges`). The pinch of unequal bricks (heights 1.5 and
  1.25, a partial overlap) is pinned refused there. A reading of
  either in band escalates as an input reading
  (`BooleanDecision::LineEdges`, `BooleanDecision::LineWedges`), with
  its lever.
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
- `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`
  `a_touching_vertex_along_two_arches_shared_ray_builds_in_every_op`
  (slow set; TANG, PR 5196): the plate united with two pyramids standing
  on `MEET` that share an edge ray, so two coincident edges lie along
  it. A pyramid along the ray with its faces in the arches, less them
  (`x − y`), reverts their kept part and refuses `Revert { source:
  WedgesOverlap }`: two cells a pose, the pyramid "in the wedge between
  them" and the one with "a face in each". `builds_where`'s `reverting`
  lists pin them; they come back as bodies with the re-pairing.

## 2026-10-17: the arches' two cells were not this row's (RESTREAD, PR 5259)

`census::line_wedges` read every line edge against the line through
`collinear_overlap` with no parallel screen. So an edge leaving the
line's end at a definite angle (`a_vertex_read_by_two_sector_passes`'s
arches: six edges at the ray's ends, 11° to 50° off it) read as lying
on it, and their wedges "overlapped". Only one edge lies on the arches'
ray. Behind its parallel screen (`census::parallel_overlap`) the line
holds that one edge, revert has nothing to re-pair, and `x − y` builds
for both pyramids at every pose. Each body holds its material, its
volume sums and tier 3, and passes 3′. The test's `reverting` lists are
gone; the arches entry above no longer returns with the re-pairing. The
other refusals listed here still refuse on the branch's CI.


**Unblocked (CLEAVE, 2026-10-19).** `[ev]` PR 4795 merged (`0a702a9bc1`). Ev ratified the D1 sentence in `crates/topo/README.md` tier 3: where more than two sheets meet on a line, the material decides the edges, and the complement re-pairs. Radial edges were declined.
## 2026-10-19: the Boolean gate refuses the class at every op (CLEAVE, `cleave/gate-refuses-overlapping-wedges`)

The result gate now reads every line where coincident edges lie
(`boolean/ops.rs` `coincident_wedges_disjoint`) and refuses an overlap
`BooleanError::LineWedges { fault: Overlap }`, for ∪ and ∩ as well as
∖, where the join pairs each operand's own faces at a line four sheets
meet on. ∖'s reverted-line check is folded into it, so the ∖ rows above
now refuse `LineWedges` instead of `Revert { WedgesOverlap }` (the
fallbacks that revert B whole or a cavity still refuse `Revert {
CoincidentEdges }`). Every newly refusing op read overlapping wedges on
main (topo, sweep, editor-core, slow set included, three ε). Besides the
rows above, these return with the re-pairing:

- `boolean_pinch_copies`: the four pinches along an operand edge (270°
  and 270°; 90° and 313.6°);
- `pierce_strut_at_a_pinch`: the staircase X itself (its second cut,
  270° and 270°), so its six ops against the plate, `P − X` among them;
- `review_4740_r1_probes`: the reflex blocks' ∩ and the quarters' ∖
  (270°, 270°), `a − nested` (90°, 310°);
- `union_flush_onto_edge_contact`: the unions of the flat-corner,
  three-pieces, two-dangling and lens witnesses (260°/190°, 230°/190°,
  225°/225°, 330.8°/195.6°), and the ending-in-the-cube's-face lens's
  four `y`s;
- sweep `ring_carry_through_by_piece`: the ring pinched to a rim vertex
  (seam 180°, pocket edge 270°), and with it the rim fillet's refusal on
  the pocket's ring;
- sweep `a_ruling_lying_on_a_wall`: `t ∖ b` on a seam ruling (180°,
  270°);
- sweep `join_reflex_wedge_probes`: four poses' ops (`OVERLAPPING`);
- topo `a_vertex_read_by_two_sector_passes` (15 cells, `OVERLAPPING`),
  `a_vertex_read_again_classes_every_edge` (four scenes, and the void on
  the lying pyramid's ray, 30.6° and 356.1°, with its three scenes),
  `three_solids_on_one_line` (fourteen cells);
- topo `a_vertex_read_again_classes_every_edge::partners_along_the_face_build_sound_against_the_germ`:
  146 cells at the default ε, 125 at 1e-6 and 141 at 1e-12
  (`PARTNERS_OVERLAPPING`, which says why the count moves with ε).

## 2026-10-20: the slivers of no area merge where they are alone on their line (CLEAVE, `cleave/boolean-merges-a-glued-slit`)

Where the join leaves a face of no area between two coincident edges,
and no third edge of the result lies along their line, the output
stage merges the two edges into one between the two walls
(`boolean/slit.rs` `merge_glued_slits`). Two sheets meet there, so the
pairing is forced. That is the glued slit's case
(`work/cleave/a-boolean-glues-away-the-face-a-slit-lies-in.md`), and
it reaches part of `union_flush_onto_edge_contact` too, which now
builds at the test's own volume, class probes and 3′, at all three ε:

- the flat-corner witness's two unions and `cutter ∖ pinch`;
- the three-pieces witness's two unions (its `cutter ∖ pinch` still
  refuses);
- both two-dangling witnesses' two unions and `prism ∖ pinch`;
- the one-segment lens's two unions and `cube ∖ y`, in both orders of
  `y`.

The ending-in-the-cube's-face lens's four `y`s, the lens inside
another along one end, and the three and four crossings' `cutter ∖
pinch` still refuse `LineWedges`. No sliver alone on its line forms
there, so this row's re-pairing is still owed.

## 2026-10-21: tier 3′ reads the wedges on a line (RESTREAD, `restread/coincident-wedges`)

The census reads two coincident line edges' wedges wherever pass 5
certifies their overlap, and refuses an overlap
`CoincidentWedgesOverlap`. The void door's graft above
(`insert_voids(box, [s, s], cavity)`, the unequal pinch) now refuses
at tier 3′ for its wedges with its two records, not only for want of
them, and the slit cavity grafted with no re-pairing refuses the same
way (`crates/topo/tests/a_doubled_line_does_not_revert.rs`). The doors
still build those bodies: the void door, the shell verb's inner twin
(`shell.rs`, through `insert_hollow_voids`) and `Body::revert` read
only shared segments, so a partial overlap reverts unrefused, and each
gates its result at tier 3 alone, which reads one edge at a time. This row's
re-pairing is still what makes them right.

## 2026-10-25: two void-door calls build an unpaired line (RESTREAD, `restread/coincident-wedges`)

A second instance, across two calls of the void door rather than
inside one cavity: two lune prisms inserted one after the other into a
brick (`insert_void` each), whose cusp edges kiss on one line, leave
two full-turn slit edges whose material overlaps. Each call reverts its
cavity alone, so neither sees the other's edge on the line. Tier 3
passes the body, and tier 3′ refuses it
`CoincidentWedgesOverlapUndecided { why: Degenerate }` once the kiss
is recorded (`crates/sweep/tests/two_wedge_ends_on_one_line.rs`). The
re-pairing owed here covers a line the destination already holds an
edge on, as well as the cavity's own.
