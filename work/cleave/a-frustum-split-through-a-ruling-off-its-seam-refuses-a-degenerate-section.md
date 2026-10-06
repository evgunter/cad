---
id: a-frustum-split-through-a-ruling-off-its-seam-refuses-a-degenerate-section
kind: issue
title: a frustum split through a ruling off its seam, 0.05 or 3 rad off tangency, refuses Join(DegenerateSection) at every eps; through the seam ruling the same pose answers
status: open
opened: 2026-10-06
priority: P1
cost: M
---



## What

The fixture is a frustum, radii 1 → 1/2 over height 1, revolved about y (`Revolution::Full`). It is
split by a plane that holds the wall's ruling at azimuth a, (cos a, 0, sin a) → (½ cos a, 1,
½ sin a), with the normal turned t about the ruling, off the outward normal. At a ∈ {0.3, 2} it
refuses `Join(DegenerateSection { face })` with both normals:

- t = 0.05 and t = 3, at ε 1e-6, 1e-9 and 1e-12;
- t = 1e-3 at 1e-9 and 1e-12;
- t ∈ {1e-4, 1e-5} at 1e-12.

The section is a quadrilateral of two rulings and two cap chords, about 0.1 wide at t = 0.05. It
is not degenerate. The volumes are closed-form (7/12 of the base segment's area, since the plane
holds the cone's apex), and they are not near zero: about 6.8e-5 against 1.83 at t = 0.05, and
about 1.5e-3 against 1.83 at t = 3. At t ∈ {0.4, 1, π/2, 2, −0.4, −1.2} the same poses answer.
The same plane through the seam ruling (a = 0) answers at the closed form with valid halves at
every one of those tilts: `cleave/seam-ruling-split`'s sweep, measured on main at 78bee3ac68
and on that branch. A solid cylinder or a tube at the same poses answers at every azimuth.

The payload's raising site is `Sweep::certify_section_area` (`splitting/join.rs`): the only
`DegenerateSection` raiser, where `decide("split_section_area", …)` reads the section's net area
as non-positive. `split_one_solid`'s mirrored rerun refuses too. Why the area reads that way is
not measured. Suspects: the conic-excess terms of the two ruling edges (cone × plane through
the apex), or the loop's winding.

## Owed

Measure the area terms per edge at a = 0.3, t = 0.05, then fix so the pose answers at the closed
form both ways. Sweep cones and frusta at every azimuth. The pose table is in
`crates/sweep/tests/split_through_a_seam_ruling.rs` (`a_frustum_split_through_its_seam_ruling_answers`,
with a = 0 and the ruling rotated to a).

## Found by

`cleave/seam-ruling-split`'s sweep of planes through a seam ruling, which compares the seam
azimuth with two off-seam twins.
