---
id: three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order
kind: issue
title: Three wedges meeting at one point of a plate's top refuse in every member order, with three different refusals
status: closed
opened: 2026-10-02
priority: P1
cost: M
closed: 2026-10-06
branch: tang/holes-meeting-at-a-vertex
pr: 4129
---


## What

The fixture as filed is the plate `[0,3] × [0,2] × [0,1]` with three
triangular prisms over the sectors 0°–60°, 120°–180° and 240°–300° of
radius 0.4 about (1.5, 1). They stand on it with z ∈ (0.5, 2.0), and
their footprints on its top are three holes meeting at one vertex. The
original probe declared the prisms' coincidences, and reported three
refusals (`RingHomingAmbiguous`, a zero-area ring-run `JoinDesync`, and
`ClassificationInvariant`).

Those sectors are a flush continuation. The lateral faces at 0° and
180° are coplanar, face the same way and touch along the line, and so
are 60°/240° and 120°/300°. Undeclared, every order refuses
`UndeclaredCoincidence` before the join. Under D10 that refusal becomes
an `unproven-coincidence` finding, and declaring the pairs is held
ground, so the fixture as filed is not this row's to build.

The row's question is a face whose holes pass through one vertex,
with k holes, in any order. This PR builds that with right prisms
leaning out of their footprints, which meet only at the vertex.
Upright prisms with non-flush sectors leave a second problem: three
solids sharing one contact line. That is filed as
`three-solids-touching-along-one-line-refuse-their-union`.

## Review tier

SINGLE, FULL: a vertex shared by several holes is new topology for the
join; one full review.

## Closed (2026-10-06, TANG, PR 4129)

Measured on 443f33b7. Under the D10 hold, the item's own fixture is
two problems, neither of them the face:

- **The 60° sectors are flush.** The lateral faces at 0° and 180°
  are coplanar and face the same way, and so are 60°/240° and
  120°/300°. Every order refuses `UndeclaredCoincidence` before the
  join, in topo and editor-core alike. The three refusals the item names came
  from a declared run, which the hold puts out of reach.
- **Upright prisms share one contact line.** With 50° sectors there
  is no coincidence left, only three solids touching along the
  vertical line through the vertex. The third prism's step refuses in
  `recl_edges`. With the plate last, the join desyncs on a pinch face. The
  three prisms alone refuse naming. That is contact, and it is filed and parked
  as `three-solids-touching-along-one-line-refuse-their-union`.

The face's own class shows once the prisms lean apart along axes
tilted out of their footprints, so they meet only at the vertex.
Folding three or more of them before the plate makes their shared
vertex pierce the top with one Out run per prism. Two fixes:

- **The ring struts hang in angular order.**
  `vtxfac::classify_vertex_on_face` refused three or more runs
  (`PierceRunsUnordered`; this also closes
  `ring-struts-of-three-or-more-runs-hang-in-run-order`, closed in JOIN's directory). Now it hangs
  the ring struts clockwise about the pierced face's outward normal
  (`vtxfac::ring_order`), and each strut faces its germs from the next
  run's start germ. The rows pin the hang direction (the mirror order
  crosses). They do not pin the sort, since every row's run order is
  already angular order; that is filed as
  `nested-pierce-runs-have-no-ring-order`.
- **The crossing vertex is a junction.** With the struts fixed,
  editor-core's union refused one step earlier, in naming
  (`names/emit_topo.rs`, `name_boolean_vertices`). The wedges' axes
  cross at the vertex, which has two A edges, one B edge and four seam
  lines, and no arm named that. The seam-junction arm now also takes a
  vertex on two or more seam lines with operand edges on both sides,
  one side holding exactly one. Its name is the set of lines, which
  straight lines make unique, and it is the same in every member order
  that mints it at the same step. Which step mints the vertex still
  depends on the order, on main as well; that is filed as
  `work/wire/a-pinch-vertex-is-named-by-the-fold-step-that-mints-it.md`.

Rows (`crates/topo/tests/holes_meeting_at_a_vertex.rs` and
`crates/editor-core/tests/union_pinch_member_order.rs`):

- every member order of two, three and four wedges;
- three wedges on one side, which leave the top a reflex sector at
  the vertex;
- an L-shaped hole with its reflex corner at the vertex, beside two
  wedges.

Each order asserts its counts, the closed-form volume, tiers 3 and 3′,
one vertex at the meeting point, and the first order's body by
geometry. Topo also asserts that every face's corners at one point are
disjoint, and editor-core asserts `check_mesh`.
On 443f33b7:

- topo refuses `PierceRunsUnordered` in 6 of 24 orders for each
  three-hole fixture and 48 of 120 for four wedges;
- editor-core refuses `Naming(Emission)` at the wedges-only step;
- the two-wedge fixture builds there and is the control.

The bare tier-3 gap the corner check exposes is filed as
`work/restfront/tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex.md`.
