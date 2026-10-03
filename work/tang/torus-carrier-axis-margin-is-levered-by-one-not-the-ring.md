---
id: torus-carrier-axis-margin-is-levered-by-one-not-the-ring
kind: issue
title: carrier_eq's torus axis-parallel margin is levered by one metre, not the ring's extent, so a declared tilt can bridge more than Kε at the tube
status: closed
opened: 2026-09-26
priority: P1
cost: M
closed: 2026-10-02
---

## What

`rest::carrier_pair_verdict` (`crates/topo/src/boolean/rest.rs`) calls
`carrier_eq::carrier_eq_verdict` with `arm = T::one()`. The torus arm
of `carrier_eq_verdict` meters axis parallelism as
`Margin::levered(|a₁ × a₂|, arm)`, so a relative tilt θ reads as
`θ · 1 m`. The displacement that tilt actually causes at the tube is
`θ · (R + r)`. On a torus with `R + r > 1 m`, a declared `Rest` pair
tilted by an angle whose one-metre reading sits inside the band is
BRIDGED by the declaration (C4), even though at the tube it
displaces the two carriers by more than Kε. The same `T::one()` lever
feeds the cylinder arm's parallelism margin, where the scale is the
patch's extent.

This predates the torus doors PR (#3265), but that PR makes it matter.
The crossing layer's carrier-identity rung (`reduce.rs`'s
`on_declared_rest_carrier`) now reads the door's "one carrier" verdict
as zero clearance for every edge on the declared face. So a bridged
tilt becomes a zero-clearance posture for edges that may stand more
than Kε off the other carrier.

## The fix's shape

Lever the angular data by the carrier's own extent: `R + r` for the
torus (the lever `torus_chart_trim` already uses for the major angle),
and a patch extent for the cylinder. This is a change to the verified
contract, so the rows that pin bridged declarations need re-measuring
with it.

## Home

TANG (`carrier_eq.rs`; `rest.rs` is shared with ZIP). Filed from the
review of PR #3265 (GERM torus doors).

## The plane arm shares the lever (CONTACT-9)

`rest::carrier_pair_verdict` passes the same `T::one()` into the plane
arm (`oriented_plane_eq`), so a declared plane pair tilted by `θ` is
verified at `θ · 1 m`, not at the faces' extent. CONTACT-9 probed a
10 m wedge whose bottom was declared `Rest` against a block's top,
tilted so that the far end stood 12 to 90 ε below. No op returned a
wrong answer. Each refused: in band (`bool_contact_edge`,
`bool_sector_within`), at the join, or `ContactContradicted`. The
metric readings downstream caught what the door's lever bridged. The
fix above should lever the plane arm by the face extent too.

## Where the literal lives (PR 3747)

The 1 m arm is a `T::one()` literal at three sites in two crates, which
the fix must move together: `rest::flush_pair_relation` and
`rest::carrier_pair_verdict` (`crates/topo/src/boolean/rest.rs`), and
`tangent_locus`'s `let arm = T::one()` (`crates/geom-brep/src/locus.rs`,
the `tangent_locus_axis_parallel` row), which meters the DEV-1 witness
lane's axis parallelism at the same arm. Their docs say the three must
agree.

## Outcome (2026-10-02)

The three `T::one()` arms are gone. `geom_brep::ExtentBall` is the one
lever: a ball enclosing the faces a verdict is consumed on, and
`lever_from(pivot)` its farthest reach from where the ladder reads the
position datum. `rest::pair_reach` builds it for a declared pair (the
torus's and the sphere's own ball; otherwise the ball around
`census::face_reach`'s box), `carrier_eq::at_consumed_extent`
re-anchors each kind's datum nearest the ball's centre for the
undeclared readings (a declared pair is read through
`carrier_eq::declared_reading`, below) and returns the arm (plane: the radius; cylinder: from the foot on the second axis;
torus: `R + r` from its centre), and `tangent_locus` takes the same
ball. The ball is measured once, on the operands at rest
(`DeclaredPairs::build`), and the mid-operation sites (`insert`,
`recl`, `vtxfac`) read it from there via `DeclaredPairs::reach_of`:
mid-operation, a face whose boundary carries null scaffolding has no
readable box (filed as
`work/contact/census-face-reach-returns-a-nan-box-for-an-unclaimable-boundary-edge.md`).
Red-then-green rows: `rest::lever_rows` (torus, cylinder),
`contact9_side_codes::a_declared_plane_tilt_is_read_across_the_faces`
(10 m plane) and `locus::tests::the_axis_row_reads_the_tilt_across_the_extent`
(the DEV-1 row). Moved: the coplanar-sector declaration row in
`reduce.rs` now contradicts (filed as
`work/hone/a-coplanar-sector-offers-a-rest-the-door-contradicts-across-the-faces.md`),
and `offer_rows`' `tangent_screen_of_a_tilted_block` quotes its tilt at
the pair's `1.5·√2` m lever.

## Fix pass (2026-10-02)

The review found the ladder still decided the offset and the tilt as
separate margins (a bridge of about 2·Kε) and read an upper bound past
the band as a definite contradiction. A declared pair is now read as
ONE displacement over its consumed extent (`carrier_eq::declared_reading`):
an upper bound at every point of the ball (position datum + tilt ×
reach + radius differences, summed) bridges when in band; a lower bound
(a reading across the ball, and each face's boundary vertices, the
points known to be consumed) contradicts when past the band; between
them the declaration is `CarrierEqError::Unsettled`. The extent is a
`ConsumedExtent` (ball + witnesses) every ladder takes. An unreadable
extent is a typed outcome (`rest::PairUnread::Extent`) refused at every
caller. Rows: `contact9_side_codes`' wedge pose and sweep,
`rest::lever_rows::an_offset_and_a_tilt_in_band_each_do_not_bridge_their_sum`.
Filed: `the-tangent-offer-drops-for-a-face-with-null-scaffolding-mid-op.md`,
`lever-a-declared-pair-by-its-contact-patch-not-both-whole-faces.md`.

## Closed (2026-10-02, TANG, PR 3795)

The declared door reads a pair as one displacement over its consumed
extent (`carrier_eq::declared_reading`). The upper bound sums the
position data at a pivot, the angular data levered from it to the
extent's far reach, and the radius differences. The lower bound is the
displacement at a consumed face vertex. The verdict:
- `Bridged` when the upper bound is in band;
- `Contradicted` when the lower bound is past the band;
- escalated in between (`Coincide::DeclaredReach`, the merge's
  `MergeDecision::DeclaredReach`).

The torus bound covers its centre, major-radius and tilt terms
together. C4's `Rest` sentence is reworded to match, and the PR body
names the one shift: a datum definite only at the ball's far side
escalates instead of contradicting. Review: a single full review, then
two delta reviews, each APPROVE-WITH-FIXES with no MAJOR. Filed out of
it:
- `lever-a-declared-pair-by-its-contact-patch-not-both-whole-faces`;
- `the-tangent-offer-drops-for-a-face-with-null-scaffolding-mid-op`;
- `a-flush-pair-with-no-readable-extent-has-no-typed-finding`.
