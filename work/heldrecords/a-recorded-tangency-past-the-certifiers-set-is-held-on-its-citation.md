---
id: a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation
kind: issue
title: A recorded tangency past the tangent certifier's set is held on its citation at tier 3: the census has no lane that confirms it
status: open
opened: 2026-11-22
priority: P0
cost: M
---


Found in the review fix pass of PR 7124 (`embed/records`, 2026-11-22),
ruled by the EMBED orchestrator the same day against the unit's
ratified text ("its result names the bar it paid (tier 3, or 3′)").

**The shape.** A plate (−2..2)²×(0..1), a neighbour (2..300) flush
with it along x, and a rod of radius 0.5 leaning 0.1·ε rad toward the
plate's y = 2 face, tangent to it within the band over the pair's own
extent (`crates/editor-core/tests/union_fold_face_face_touch.rs`,
`a_merged_face_far_longer_than_the_pair_holds_the_pass_s_line`). The
union's fold decides the tangency over the pair's extent and records
the face–face touch, citing its row. The neighbour's face merges with
the plate's, so the record lies on a face 298 long.

**What the census answers.** The plate and the rod alone, either
order: confirmed. Any of the four member orders with the neighbour:
`CensusUnsupported { FacePair, ContactLane(NotCertifiable { "a
face-face record on two carriers with no closed-form tangent locus
(plane x cylinder along a ruling, parallel cylinders)" }) }`. Over the
merged face's extent the leaning carriers have no tangent locus the
certifier holds, so it reads nothing.

**What the gate does** (`census::confirm_only`'s `reading`,
`crates/topo/src/census/confirm_only.rs`): this is the one cause it
classes as no lane. The record is held on its citation and the body's
bar is `Bar::Tier3`, pinned by
`union_fold_face_face_touch::a_record_no_lane_reads_is_held_at_tier_3`.
Every other cause the confirmation can raise refuses.

**Owed.** A reading that confirms a recorded tangency where it was
decided: over the extent its op decided it at (a record "is read where
it is minted and carried by lineage", `[ev]` PR 4951), or a certifier
whose set holds a tangency within the band over a longer face. Until
then a body holding such a record is tier 3, and nothing downstream
may read it as tier 3′. Population on 2026-11-22: one test in the fast
set reaches this class; the slow set, the tour and the demos are
logged in PR 7124's body.

**After PR 7124's second fix pass (2026-11-23): what the reviewers
showed, what closed, what remains. P0** (EMBED's orchestrator;
HELDRECORDS may re-band): it is the unit's own property, bounded.

*Demonstrated.* `sweep review_7124_delta_probes::red_no_record_between_bodies_that_touch_nowhere_is_held`,
red on `bbb6955172`: a rod and a block 9.5 apart, two rods 19 apart,
joined; of 560 planted records, one cell from each body, 24 were held
at `Bar::Tier3`. Their lanes answered "not mine" having read no
distance. A caller's `Built::recording(...)` reaches the gate with
such a record, so a public call made a finished body holding a false
one.

*Closed.* "No lane" is no longer one cause's name.
`ContactRefusal::NotCertifiable` carries `reach`, stated where it is
raised, and a gate holds a record only on `ContactReach::NoLane`, and
only once the pair's separation is read (`census::confirm_only`): the
Boolean's narrow phase over the two cells' covers (the axis between
them, the world's axes, each planar cell's normal) and their boxes. A
pair decided apart refuses stale. Everything else that was
`NotCertifiable` is a reading that ran and did not decide
(`ContactReach::Unread`: a trim the containment door does not read, a
face with no extent or no surface, a span bound the schedule did not
reach, a meter that refused, a torus outside its convention) and
refuses. The two shapes the tangent locus names needed nothing more:
a plane parallel to a cylinder's axis and two parallel cylinders
already read their carriers' gap and contradict a record of a pair
apart (`sweep a_record_no_lane_reads`, four rows). The reviewer's row
is green.

*What can still be held unconfirmed.* A record of a kind that says
`NoLane`, whose two cells no separating direction above parts. The
kinds: two carriers with no closed-form tangent locus (a plane across
a cylinder's axis, cylinders whose axes are not parallel, any other
pair of kinds: this row's shape, the one a suite reaches with a true
record); a kind outside the rest inventory; a pair outside the jet
certificate's span-bound lane, or with no span bounds; an edge lying
on a face's carrier away from a conventional vertex. Demonstrated,
and asserted as held so that it goes red when this row is done:
`sweep a_record_no_lane_reads::a_false_record_no_lane_reads_and_no_axis_parts_is_still_held`,
a rod standing in the notch of an L-shaped prism, its foot circle on
the carrier plane of the prism's top and 0.25 clear of the top,
recorded as touching it and held at tier 3. A second shape, in the
same file: `a_wall_against_the_cap_of_a_block_it_touches_along_a_ruling_is_still_held`,
a rod against a block's side, whose wall and the block's cap meet only
at a point on both boundaries; the wall recorded as touching the cap
is held at tier 3. A caller's `recording(...)` reaches both.

*The fix is the census's missing lane*, not anything about the
citation: the ratified end state is that the census confirms the
record (`crates/topo/README.md`, tier 3), so each kind above owes a
reading, and "held on its citation" is the shortfall until it has
one. The citation is the document's trail and is not what certifies a
record.
