---
id: torus-carrier-axis-margin-is-levered-by-one-not-the-ring
kind: issue
title: carrier_eq's torus axis-parallel margin is levered by one metre, not the ring's extent, so a declared tilt can bridge more than Kε at the tube
status: open
opened: 2026-09-26
priority: P1
cost: D
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
