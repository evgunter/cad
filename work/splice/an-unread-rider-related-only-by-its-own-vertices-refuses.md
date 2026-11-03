---
id: an-unread-rider-related-only-by-its-own-vertices-refuses
kind: issue
title: An unread carrier related to a face only by vertices the two share in one operand refuses as a rider: the lofted cylinder's unions and the tilted rod's other spins stop there
status: open
opened: 2026-11-03
priority: P0
cost: M
refs: [a-spline-rider-reads-as-lying-in-no-plane-face, check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges]
---

Filed by SPLICE's `splice/kind-reads` lane (PR 6437), at the orchestrator's
ruling on its re-review.

## Finding

`topo::boolean::slit::holding` reads an edge whose carrier's place on a
face is not read (`slit::Placed::Unread`: a spline face, or a spline or
spiric carrier no certificate clears) as lying in the face wherever
anything relates the two, and refuses where both the edge's faces go into
the face's material (or where reading them refuses). PR 6437 dropped the
midpoint check that used to stand between: a sampled "in no face" gave a
wrong answer on a keel across a notch
(`slit::spline_rider::a_spline_keel_across_a_notch_refuses_unread`).

`riders` relates an edge to a face through its ends as well as records,
and an end's faces include the faces of its own vertex in the body. So an
unread edge is asked against every other face around its own vertices.
Builds that shipped now refuse there:

- `spline_spiric_operands` (`sweep/tests`): the lofted cylinder against a
  far brick, `O ∪ B` and `B ∪ O`, and against the brick around it,
  `B ∖ O`: a circle rim arc of the cylinder, asked against the cylinder's
  OTHER spline wall, which holds both the arc's vertices; the arc's place
  on a spline face is unread: `EdgeInFace { SplineFace }`. Pinned by identity
  in `the_lofted_cylinders_unions_refuse_at_its_own_rims`.
- `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`:
  the steeper tilt's union (β 0.7, spins 0 and π/2, `h ∪ c`) stops at
  `"unread rider"` (`EdgeInFace { CarrierUnread }`) at every ε row, where
  it read the result gate's volume (`"spiric wall volume"`); pinned in
  `OTHER_SPINS_STOPS`. Whether its edge and face are one operand's,
  related by shared vertices alone, is not yet read: the first step of
  this row is to read it.

## What would let them build again

A STRUCTURAL exemption, stated and tested, never a sampled point: an end
lies on the face only as a vertex the edge and the face share in one
operand's own topology, with no record (no vertex-vertex partner, no
vertex-on-face or vertex-on-edge record, no edge-on-face or crossing
record). Its soundness argument, to be made and pinned: each operand is at
rest and tier-3 clean, so a face of it meets its edges only along its own
boundary edges; every incidence the join makes between the operands is
recorded; so such an edge leaves such a face at the shared vertex and lies
along no piece of it. What has to be shown is the middle step: that the
join, the zip and the slit's own surgery mint no unrecorded incidence
between an edge and a face of different origin (the result's section edges
and fused vertices among them), and the exemption has to read origin from
topology (lineage or the operand split), not from a sample. Rows: the
three cylinder builds build again (and the rod's union, if its stop is
the same case), and the notch keel
(`a_spline_keel_across_a_notch_refuses_unread`), whose ends lie on the face
by records, still refuses.

## 2026-11-03: the exemption is sound by lineage, not by shared topology (SPLICE re-review of PR 6437)

Re-banded P0 (cost M): a disjoint far union refusing is a normal verb
broken on normal geometry.

**What is sound.** An unread edge `e` and a face `f` of the result may be
exempt from the rider reading where:

1. both cells' split roots (`Body::split_root`, the lineage the boolean
   keeps) lie in ONE operand's input;
2. the vertex they share is unfused: no vertex-vertex record partners it
   with a vertex of the other operand (`slit::partners` empty);
3. no record names the pair: no edge-on-face, vertex-on-face,
   vertex-on-edge or crossing record relating `e`, its ends, or `f`'s
   boundary edges.

Then `e` and `f` are an edge and a face of one operand's own topology,
meeting only at that operand's own vertex, and an edge of a solid at rest
lies along no piece of a face it does not bound.

**What is not sound.** Shared-vertex topology alone (`f` among the faces of
`e`'s end vertex in the result body): a fused vertex joins the two
operands, so an edge of one and a face of the other can share it, and the
edge can lie in the face. The reading has to go through lineage, never a
sample and never the joined body's incidence alone.

**What it rests on, unchecked today.** "An edge of a solid at rest lies
along no piece of a face it does not bound" is the operand's tier-3
validity, and two things leave it unread:

- nothing validates the operands at the boolean's entry (an `AtRestBody`
  is trusted, not re-checked);
- tier 3 / 3′ does not read an edge lying in a non-incident face for a
  spiric or spline carrier
  (`work/restfront/check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges`).

So this row owes the lineage exemption AND that check (tier 3 reading a
spiric or spline edge against the faces it does not bound), or an explicit
contract on the operand type that states it, together: the exemption
without either rests on an assumption nothing holds.

**The rows that must build again** (each pinned now as a refusal):

1. `spline_spiric_operands`: the lofted cylinder ∪ the far brick, `O ∪ B`
   (`the_lofted_cylinders_unions_refuse_at_its_own_rims`, by identity);
2. the same, `B ∪ O`;
3. the lofted cylinder subtracted from the brick around it, `B ∖ O`;
4. `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`,
   the β 0.7 union at spins 0 and π/2 (`"unread rider"` in
   `OTHER_SPINS_STOPS`), if its pair is the same-operand case; and the
   notch keel (`slit::spline_rider::a_spline_keel_across_a_notch_refuses_unread`),
   whose ends lie on the face by records, still refuses.

