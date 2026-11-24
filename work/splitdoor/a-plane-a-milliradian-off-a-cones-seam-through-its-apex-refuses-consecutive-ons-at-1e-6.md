---
id: a-plane-a-milliradian-off-a-cones-seam-through-its-apex-refuses-consecutive-ons-at-1e-6
kind: issue
title: a plane through a cone's apex holding its seam ruling a milliradian off tangency refuses ConsecutiveOnSectors at 1e-6, where off the seam it lands whole
status: open
opened: 2026-10-28
priority: P3
cost: M
---


## What

A full cone of half-angle 30° or 45° (profile `(0, 0), (tan α, 0),
(0, 1)` revolved whole about `y`), split by a plane through its apex
that holds the seam ruling (azimuth 0) and is turned `t = 10⁻³` off
tangency about it, either normal, refuses at ε 1e-6
`Reduce(ConsecutiveOnSectors { vertex })` at the apex. The seam edge
lies in the plane, and the extremal ruling the walk subdivides the
adjacent apex sector at (`sector_shape::apex_shape`) stands ≈ t/2 from
it, where the plane's side reads On: the sliver between the seam and
the plane's other ruling is ≈ t²/2 deep, within the band. The same
tilt held at azimuths 0.3 and 2, where the grazed-off ruling is no
edge, lands whole on the material side at its closed form.

Measured on `splitdoor/cone-apex`; pinned as a typed refusal in
`crates/sweep/tests/a_plane_through_a_cones_apex.rs`. At 60° the base
circle refuses first
(`a-sixty-degree-cones-base-circle-reads-a-milliradian-graze-in-band-at-1e-6`).

## Where to look

Rule (b) (`splitting/rules.rs` `apply_rule_b`) refuses two consecutive
On entries before reading them; an On edge beside an On subdivision
ruling of the same apex sector is one contact (the sliver within the
band); the off-seam pose lands, by inference reading its On ruling as a
lone duplicate (unmeasured).

## 2026-11-23: the seam pose refuses ahead of rule (b) on main

(GREEN U19.) Since PR 7299 (`germ/crossing-consumers`, `54e6a5c5b8`) the
split's classify reads every edge through `curve_face::cross`. At ε 1e-6
the 30° and 45° seam poses, either normal, now refuse there first:
`Reduce(CrossingEscalated { edge, fault: Method { door: PlaneCrossing,
fault: Unsettled(Tangency) } })`, on the seam edge lying in the plane.
So rule (b) is not reached on main. Off the seam both still land.
Measured on hosted CI, probe run 45317834459: `ConsecutiveOnSectors` on
`f7dd37696d` (7299's first parent), `CrossingEscalated` on `54e6a5c5b8`
and on `a433ff83d6`. `a_plane_through_a_cones_apex.rs` pins the crossing
refusal now. This row's question (two consecutive Ons read as one contact)
has no witness on main until the crossing settles the in-plane edge.
