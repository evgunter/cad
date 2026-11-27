---
id: a-sections-two-arcs-seamed-a-quarter-turn-apart-are-two-statements-a-rounding-apart
kind: issue
title: A section's two arcs, each seamed near its own middle, are stated a quarter turn apart through a turn that rounds at f64, so they are two statements an ulp apart and no certificate may read them as one conic: a cone cut out through a cap refuses on its cut cap
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [two-conics-are-compared-through-three-doors, 7605]
---


## Finding

From `embed/conic-symmetry` (2026-11-23), measured on main `a497926d1`.

`chord_arc` (`crates/topo/src/chord_join.rs`) seams each arc of a
section conic near that arc's own middle
(`SectionCurve::Conic(conic.seamed_near(middle))`). Two arcs of one
section whose middles lie in principal quadrants a half turn apart are
then stated by an exact negation: one anchor, one axis, `u_ref`, the
stored `v` and both phase components negated, every bit. Two whose
middles lie a quarter turn apart are stated through
`Ellipse::quarter_turned_where`, which at an `Inexact` witness takes
`Frame::quarter_turned_renormalized` (`u' = normalize(w × u)`,
`v' = w × u'`) and so rounds.

The frustum of radii 0.7 and 0.31 over a height of 0.9 about `y`, split
by the plane through `(0, 0.86, 0)` whose normal is `y` tilted 0.3
toward the azimuth 0.7, the side below (the cut runs out through the
top cap), states its two section arcs on one anchor and one axis with
the semi-axes exchanged and

- `u_ref (0.644217687237691, −2.775557561562892e-17, −0.7648421872844885)`,
  stored `v (0.7306816499355124, −0.29552020666133955, 0.6154446635582734)`,
  phase `(−0, −1)`;
- `u_ref` the first's `v` bit for bit, stored
  `v (−0.6442176872376909, 2.7755575615628914e-17, 0.7648421872844884)`,
  phase `(−1, 0)`:

the second's `v` is one ulp off the first's `−u_ref` in all three
components. The same holds turned, a hundred units out and mirrored
(`self_intersection_proves_injectivity::a_cones_split_states_one_ellipse_through_its_wall_and_two_arcs_a_rounded_quarter_apart_through_its_cap`).

Two statements a rounding apart state two curves
(`crates/geom-core/README.md`, "A conic is the curve its stored numbers
state"), so the embedding certificate's `one_stated_conic`
(`crates/topo/src/injective.rs`) reads them as two, the cut cap is
`Structural` at the vertex where they meet, and the body refuses there.
On main since PR 7414 a full revolve's wall is one band, so a cut
through the wall alone states its section as one closed ellipse and
holds no pair for this row to read: both wall cuts read `Embedded` at
`f64` and at `Interval` in all four placements (16 of 16 asks,
`self_intersection_proves_injectivity::a_split_cone_refuses_on_a_pair_holding_its_wall`).
This cut still refuses `Uncertified { Departure }` on its cut cap in all
four placements at both scalars (8 of 8), the cap against the cone's one
wall face across a section arc.

**What decides it, and a count.** Each arc is seamed at the principal
vertex nearest its own middle, so the two arcs' middles decide it:
opposite, a half turn, exact; in adjacent quadrants, a quarter turn,
rounded. Measured on one band over a scan of 1,152 splits of a frustum
and a rod, either side kept (pinned as
`self_intersection_proves_injectivity::a_splits_section_is_one_ellipse_through_the_wall_and_one_or_two_arcs_through_a_cap`):

- every cut through the wall alone (864 of 864) states one closed
  ellipse, so the wall is no longer this row's subject;
- a cut that runs out through a cap leaves its wall arc whole (177 of
  288) or cut in two by the wall's wrap edge (111), and of those two-arc
  cuts 58 state a quarter turn and 53 a half: meeting a cap does not
  decide which;
- no split states two arcs any other way, run back or both (0 of 111),
  though `oriented_arc`'s clockwise branch
  (`crates/topo/src/chord_join.rs`) is a door that could; no split
  refuses.

Before one band (the review's scan on `a497926d1`) every wall-alone
cut stated two arcs a half turn apart (864 of 864), and of 66 two-arc
cap cuts 26 stated a quarter and 40 a half. This row first said a cap
cut states a quarter, read off 2 cap cuts and 12 wall cuts built by
the lane; that was half right.

Of the 24 cone/cone census tests run through the at-rest gate with
PR 7605's reading and this unit's in one tree, 12 still fail, and none
of the 12 was traced to a quarter turn: their refusals are a cone
against a plane (5), two cones (5), a plane against a torus (1) and a
depth budget (1), and they are the 12 that failed before the anchored
ellipse.

**The band.** P1, by `work/README.md`'s definitions: no verb on main
is gated on this (no cut cone holds on main at all), one family of cap
cuts refuses behind PR 7605, and no census test was traced to it. It is
P0 the day the at-rest gate asks the embedding question in the split.

**Not checked:** the census tests' own bodies, and which other section
doors seam two arcs of one conic a quarter turn apart.

The cone-image lane's row
`two-arcs-of-one-cone-section-are-stated-as-two-anchored-ellipses-so-no-reading-knows-them-one-conic`
(on its branch, on ONEROUND's slate) is about the half-turn pairs,
which `embed/conic-symmetry` reads as one; this row is the quarter
turn, which nothing may.

## What a fix needs

One of: the two arcs of one section stated on one seam, or on seams a
half turn apart (an exact restatement, `Ellipse::restatements`); or a
quarter turn that is an exact permutation of the stored axes at `f64`
too. The second alone would not make the pair one stated conic: the
position reads `v_ref` as `axis × u_ref`, and `axis × (axis × u_ref)`
is `−u_ref` only where the stored axes are rigid to the bit, so an
exact quarter turn is a symmetry of the frame's lift (its polar factor)
and not of the formula `geom` evaluates. Which of the two readings a
quarter turn is owed is this program's question
(`two-conics-are-compared-through-three-doors`, "the same locus, in
either sense: exactly").
