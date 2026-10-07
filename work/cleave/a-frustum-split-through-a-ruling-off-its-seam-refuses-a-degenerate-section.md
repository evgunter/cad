---
id: a-frustum-split-through-a-ruling-off-its-seam-refuses-a-degenerate-section
kind: issue
title: a frustum split through a ruling off its seam, 0.05 or 3 rad off tangency, refuses Join(DegenerateSection) at every eps; through the seam ruling the same pose answers
status: closed
opened: 2026-10-06
priority: P1
cost: M
closed: 2026-10-06
branch: cleave/frustum-apex
pr: 4181
---



## What

The fixture is a frustum, radii 1 → 1/2 over height 1, revolved about y (`Revolution::Full`). It is
split by a plane that holds the wall's ruling at azimuth a, (cos a, 0, sin a) → (½ cos a, 1,
½ sin a), with the normal turned t about the ruling, off the outward normal. At a ∈ {0.3, 2} it
refuses `Join(DegenerateSection { face })` with both normals:

- t = 0.05 and t = 3, at ε 1e-6, 1e-9 and 1e-12;
- t = 1e-3 at 1e-9 and 1e-12;
- t ∈ {1e-4, 1e-5} at 1e-12.

The true section is a quadrilateral of two rulings and two cap chords, about 0.1 wide at
t = 0.05. The volumes are closed-form (7/12 of the base segment's area, since the plane holds
the cone's apex), and they are not near zero: about 6.8e-5 against 1.83 at t = 0.05, and about
1.5e-3 against 1.83 at t = 3. At t ∈ {0.4, 1, π/2, 2, −0.4, −1.2} the same poses answer. The same
plane through the seam ruling (a = 0) answers at the closed form with valid halves at every one
of those tilts. Measured by `cleave/seam-ruling-split`'s sweep, on main at 78bee3ac68 and on that
branch. A solid cylinder or a tube at the same poses answers at every azimuth.

**The refused loop is not that quadrilateral** (measured by the review of PR 4158, at a = 0.3,
t = 0.05). It is a two-edge digon:

- a top-cap chord;
- a scaffold `Line` across the cone face, joining the wall's two top-rim crossings (azimuths
  0.30 and 0.19) to each other.

Its area is 0. It has no placeholder and no ruling edge. The wall's crossings were paired top↔top
rather than top↔bottom along a ruling. The plane holds the cone's apex, so its section of the
cone is a degenerate conic: a pair of rulings, not a curve that pairs crossings on one rim.
`Sweep::certify_section_area` (`splitting/join.rs`, the only `DegenerateSection` raiser) reads
the digon's area as zero and refuses. `split_one_solid`'s mirrored rerun refuses too. Through the
seam ruling, the mirrored rerun rescues a 4-gon. This is not PR 4120's one-vertex placeholder loop.

## Owed

Find where the cone wall's crossings are paired. A plane through the apex should pair each rim
crossing with the other rim's crossing along a ruling, not with the second crossing on its own
rim. Then fix that so the pose answers at the closed form both ways. Sweep cones and frusta at
every azimuth.

The pose table is in `crates/sweep/tests/split_through_a_ruling.rs`
(`a_frustum_split_through_its_seam_ruling_answers`). Its closed form reads the base line's
distance as `n.x / hypot(n.x, n.z)`, which holds at azimuth 0 only. Rotated to the ruling at a,
the distance is `(n.x·cos a + n.z·sin a) / hypot(n.x, n.z)`, with the ruling and its normals
rotated by a about y.

## Found by

`cleave/seam-ruling-split`'s sweep of planes through a seam ruling, which compares the seam
azimuth with two off-seam twins.

## Built (branch cleave/frustum-apex)

**Cause, measured.** The wall's crossings were never given fixed partners. `fixed_partners`
(`splitting/join.rs`) hands a curved face with more than two crossings to `conic_pairs`, and
`conic_pairs` paired only a `SectionCase::Conic`: the cone's `ApexLinePair` (and the cylinder's
`ParallelLines`) reached it as `SectionCase::Straight`, which carried no lines, so the face fell
back to the book's rule — the first loose half of opposite sense, in the sweep's lexicographic
order. That order keys first on the in-plane `u` axis, the projection of x. At a = 0.3,
t = 0.05 the four wall crossings sort as the top crossings at azimuths 0.30 (x = 0.478) and 0.19
(x = 0.491), then the two base crossings near x = 0.96, so the first join on the cone face paired the two top crossings (a traced
`Sweep::take_neighbor` print: `end … down=true at (0.4777, 1, 0.1478) <- half … down=false at
(0.4912, 1, 0.0936) fixed=false`). At t = 0.4 the two rulings straddle the seam, each lands in
its own wall face with two crossings, and the book's rule has only one choice; that is why
those tilts answered.

**Fix.** `SectionCase::Straight` carries the two rulings (`chord_join.rs`, `section_case`), and
`conic_pairs` hands a two-ruling section to `ruling_pairs`: each crossing goes to the ruling it
lies on (`split_join_ruling_side`, its distance from the other ruling less its distance from
this one, refused undecided), and each ruling's crossings are paired along it by
`pair_along_line`, the planar face's along-line pairing factored out of `line_pairs`. Nothing
is special to the frustum: the same arm pairs a cylinder cut parallel to its axis.

**Measured at ε 1e-6, 1e-9 and 1e-12, main against the branch**, by a probe over azimuths
{0, 0.3, 1, 2, 3, 4, 5.5}, tilts {±0.05, ±0.4, 1, π/2, 2, ±3, −1.2} and the near-tangent
{1e-3, 1e-4, 1e-5}, both normals:

- frustum and flared frustum (radii ½ → 1, apex below), tilts ≥ 0.05: 44 of 140 poses each
  refused `DegenerateSection` on main at every ε; on the branch every one answers at the closed
  form with tiers 1, 2, 3, 3′ on both sides;
- each frustum about the axes y, (1, 1) and (1, 0.2), φ ∈ {0, 0.3, 1, 2, 4}, the eight tilts
  of the pose table, both normals: 78 of 240 refused `DegenerateSection` on main at every ε
  (y 16, (1, 1) 28, (1, 0.2) 34; a fourth axis, (0.3, 1), refuses 14 more of 80); all answer;
- a solid cylinder about the same axes cut parallel to it at offsets {0, 0.5, 0.9, −0.7}:
  20 of 120 refused `DegenerateSection` on main at every ε, all on the tilted axes ((1, 1) 6,
  (1, 0.2) 14; the same interleaving); all answer;
- `plane_section` of the frustum through a ruling: 28 of 112 refused on main at every ε; all
  answer one region with the trapezoid's closed-form area;
- no pose that answered on main refuses or answers wrongly on the branch.

**A crossing left over on a ruling refuses.** `pair_along_line` pairs each crossing with the first
loose one before it of opposite sense. On a planar face a leftover keeps the book's rule; on a
ruling that rule could pair it across to the other ruling, so `ruling_pairs` refuses it as
`SectionCrossings { case: NotAlternating }` instead. Unreached, measured: an instrumented run
(a panic on any leftover) of `topo`, `sweep`, `mesh` and `editor-core`, and of the probe above at
the three ε, met none. It is marked **Unpinned**, as is the side escalation at an apex.

Pinned in `crates/sweep/tests/split_through_a_ruling.rs` (the seam-ruling suite, renamed):
`a_frustum_split_through_a_ruling_answers`, `a_flared_frustum_split_through_a_ruling_answers`,
`a_frustum_about_a_tilted_axis_split_through_a_ruling_answers` (slow set),
`a_cylinder_cut_parallel_to_its_tilted_axis_answers`,
`a_frustum_section_through_a_ruling_is_its_trapezoid` (all red on main), and
`a_near_tangent_cut_through_a_frustum_ruling_never_answers_wrongly`, over both frusta at azimuths
{0, 0.3, 1, 2, 3, 4, 5.5} and t ∈ {1e-3, 1e-4, 1e-5}. That gate holds what a split answer
promises: closed, valid sides (tiers 1 and 2), and every volume a side certifies at its closed
form. A side whose volume the band cannot certify refuses typed at `mass_properties`.

**The full cone is a different defect.** It refuses `Reduce(SliverSector)` on `sector_straight`
at the apex vertex, margin exactly 0.0, in the reduction, before any pairing; evidence added to
`a-convex-graze-of-a-cone-refuses-at-some-azimuths`.

**Near tangency, measured** (t ∈ {1e-3, 1e-4, 1e-5}, the seven azimuths, both normals, both
frusta: 84 poses per ε), on main's in-band graze decision (#4179). The sliver's depth off the
plane is `big·(|n⊥| − |n·r̂|)` ≈ 0.559·t² (5.59e-7, 5.59e-9 and 5.59e-11 m at the three tilts);
where it is within ε the graze lands the frustum whole on its material side.

- At 1e-6, 72 poses land whole (every off-seam azimuth, every tilt), each holding tiers 1–3′ and
  the whole volume. The 12 at the seam azimuth a = 0 refuse in the reduction instead
  (`ConsecutiveOnSectors`, `SliverSector`, `CrossingEscalated`, `CrossingInsertion`); evidence
  added to `a-convex-graze-of-a-cone-refuses-at-some-azimuths`.
- At 1e-9, 24 land whole (t = 1e-5 off the seam), 28 cut clean at tiers 1–3′ (t = 1e-3), 28
  refuse `CrossingEscalated` (t = 1e-4, depth 5.59e-9 in the escalation band) and 4 refuse
  `ConsecutiveOnSectors` (a = 0, t = 1e-5).
- No pose lands whole with its depth beyond ε.
- At 1e-12, none lands whole and 56 cut. Every certified volume is right, and on ten poses a side
  escalates at tier 3 or 3′:

- tier 3 `VolumeUncomputable` on `props_du_consistent`, margin 1.986e-12, on the sliver side,
  where `mass_properties` refuses typed: the frustum at a = 5.5, t = 1e-4, both normals; the
  flared frustum at a ∈ {0.3, 2, 3}, t = 1e-4, both normals;
- tier 3′ `CensusEscalated` on `pm_census_ee_gap`, on both sides, whose volumes certify right: the
  flared frustum at a = 1, t = 1e-5, s = +1 (margin 1.145e-12) and at a = 4, t = 1e-5, s = −1
  (1.836e-12).

All ten refused `DegenerateSection` on main. The record is in
`split-sides-are-not-finished-bodies`.

**Not done here (S1):** `SectionCase::Straight` still carries `Curve3`, because the tables in
`geom-brep` build it so, and typing it only in `topo` would move the non-line arm rather than
remove it. Filed as `work/reach/the-ruled-section-tables-carry-their-rulings-as-any-curve.md`.

## Closed (PR 4181, 2026-10-06)

A section of two rulings pairs each ruling's crossings along that ruling (`ruling_pairs`, through the
same `pair_along_line` the parallel-lines case uses); a crossing left over refuses
`SectionCrossings { NotAlternating }`. The near-tangent gate covers both frusta over every measured
azimuth: a wrong volume is red, and a typed tier-3 refusal is `split-sides-are-not-finished-bodies`'
debt. Line-typed ruling tables are filed on REACH
(`the-ruled-section-tables-carry-their-rulings-as-any-curve`).
