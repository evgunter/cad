---
id: torus-carrier-axis-margin-is-levered-by-one-not-the-ring
kind: issue
title: carrier_eq's torus axis-parallel margin is levered by one metre, not the ring's extent, so a declared tilt can bridge more than Kε at the tube
status: open
opened: 2026-09-26
priority: P1
cost: M
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
re-anchors each kind's datum nearest the ball's centre and returns the
arm (plane: the radius; cylinder: from the foot on the second axis;
torus: `R + r` from its centre), and `tangent_locus` takes the same
ball. Red-then-green rows: `rest::lever_rows` (torus, cylinder),
`contact9_side_codes::a_declared_plane_tilt_is_read_across_the_faces`
(10 m plane) and `locus::tests::the_axis_row_reads_the_tilt_across_the_extent`
(the DEV-1 row). Moved: the coplanar-sector declaration row in
`reduce.rs` now contradicts (filed as
`work/hone/a-coplanar-sector-offers-a-rest-the-door-contradicts-across-the-faces.md`),
and `offer_rows`' `tangent_screen_of_a_tilted_block` quotes its tilt at
the pair's `1.5·√2` m lever.
