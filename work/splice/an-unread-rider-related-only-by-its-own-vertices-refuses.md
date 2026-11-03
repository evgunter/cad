---
id: an-unread-rider-related-only-by-its-own-vertices-refuses
kind: issue
title: An unread carrier related to a face only by vertices the two share in one operand refuses as a rider: the lofted cylinder's unions and the tilted rod's other spins stop there
status: parked
opened: 2026-11-03
priority: P0
cost: M
refs: [a-spline-rider-reads-as-lying-in-no-plane-face, check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges, a-section-spiric-on-a-cap-plane-reads-unread-against-the-wall-it-ends-on]
blocked_on: [the-at-rest-gate-certifies-embedding]
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

## 2026-11-03: Phase 0, parked on CLEAR's embedding gate (SPLICE `splice/same-operand-rider`, PR 6487)

**Measured** on main at `5f6aaf16` (default ε), with a probe in
`slit::riders` where `holding` refuses `SplineFace` or `CarrierUnread`. The
probe printed each edge's provenance and split root, which operand's input
image (through `ops::KeyView`) the edge, its root, the face and both ends lie
in, the ends' `slit::partners`, and every record naming the pair.

- **The three cylinder stops are this case.** In `O ∪ B` (far), `B ∪ O`
  (far) and `B ∖ O` (around), the edge is a rim circle arc, its own split
  root, and the face is the cylinder's other spline wall. The edge, its root,
  the face and both shared vertices all lie in one operand's input. There is
  no partner, no record and no crossing (`Names { ends: true, record: false }`).
- **The tilted rod's β 0.7 union (spins 0 and π/2) is not.** Its edge is a
  section chord the join minted (`Mef` at one spin, `Mekr` at the other), in
  neither operand's input. It bounds the donut's torus wall and the rod's
  plane cap, so its carrier is a spiric in the cap's plane. It is asked against
  the rod's cylinder wall, whose rim holds both its ends. The lineage
  exemption must leave it refused. It is its own row,
  `a-section-spiric-on-a-cap-plane-reads-unread-against-the-wall-it-ends-on`,
  so this row's rows to build again are the three cylinder builds.

**The guarantee is CLEAR's `the-at-rest-gate-certifies-embedding`.**
`docs/DESIGN.md` `[topology.tiers]` already states it. A finished body's
boundary is embedded off its recorded contacts: it meets itself only along
its own edges and recorded contacts, certified by the clearance engine's
injectivity question. That is exactly what the exemption rests on, and the
gate is unbuilt. Nothing nearer gives it:

- **Check 9 does not.** Check 9, with RESTFRONT's
  `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges`
  arms completed, reads one face's loops against each other (a ring against
  its outer loop). It never reads an edge against a face it does not bound.
- **Tier 3 does not.** No tier-3 check reads that pair for any carrier kind:
  it is global self-intersection, first on `validate_geometric`'s "does NOT
  yet check" list.
- **Analytic carriers never needed it.** `slit::carrier_on_surface` reads
  them `On` or `Off` geometrically. Only the `Placed::Unread` arm would rest
  on the operand being at rest.
- **`AtRestBody` does not.** It promises `validate_geometric_certificate`,
  which is tiers 1–3. PR 6371's `finish` is still tiers 1–3.
- **A check at the boolean's entry would be building CLEAR's gate early,
  in a scoped form.** It would be a face-set query through
  `topo::clearance::Side`, asked only for the pairs the exemption clears. At
  `f64` it has no lane: `AtRestPolicy::clearance_lane` is `None` there, and
  splitting it is item 1 of CLEAR's row. Its spline departure arm outruns its
  depth budget at a loft's cap corner
  (`departure-reads-a-spline-sides-zero-to-second-order-so-a-cap-corner-outruns-the-depth-budget`),
  and that corner is this row's own witness.

**Options.**
- (a) The exemption waits on CLEAR's gate. **Ruled:** the SPLICE
  orchestrator chose (a). The kernel stays fail-loud until the gate checks
  what `[topology.tiers]` promises.
- (b) The exemption ships citing the ratified clause as the operand type's
  contract, with nothing holding it at runtime until CLEAR lands. Not built;
  it is to be put to Ev as the alternative.

**Lineage, for when the row resumes.**
- **`Body::split_root` is edges only.** It chases `Provenance::SplitEdge`, and
  the graft forwards provenance. A join chord is its own root and lies in no
  input.
- **Faces have no split root.** A face's operand is read from:
  - the key view's image (A `Direct`, B `Graft`);
  - `Descendants::fragments`, the join's chord-mef divisions, in clone keys
    before the view;
  - the join's absorbed faces.
- **Three steps can leave one operand's key on geometry that is partly the
  other's.** All run before `part_riders`:
  - the coplanar merge (`OutputStage::merged`), which can keep A's face key
    on a region that took in B's face;
  - edge joins (`OutputStage::edge_joins`, `killed_lineage`), which can join
    pieces of both operands into one edge;
  - the zip's substitutions (`Descendants::cells`).
- **Lineage is therefore computed exactly in `ops.rs`**, where `views`,
  `desc` and `OutputStage` live at both `part_riders` calls, and handed to the
  slit pass. It is not read from `split_root` inside `slit.rs`.
- **A fused vertex leaves no vertex-vertex record.** `carry` drops a record
  whose two cells became one (`record` → `Contact::new`), so `slit::partners`
  does not see a zip fusion. Condition 2 would read the zip's fusion rows
  instead: `Descendants::vertex_merges()`, `share_points`' point classes and
  `cone_splits`.
- **Conditions 2 and 3 may add nothing.** With exact lineage, condition 1
  carries the soundness argument: a piece of an input edge and a fragment of
  an input face of one embedded operand meet only along the face's own
  boundary. Conditions 2 and 3 are kept only if some bug can turn a row red
  through them. Otherwise they go: a check no bug can turn red is
  documentation.
