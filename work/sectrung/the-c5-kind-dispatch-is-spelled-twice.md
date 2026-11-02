---
id: the-c5-kind-dispatch-is-spelled-twice
kind: issue
title: The C5 kind dispatch is spelled in two exhaustive matches: route answers per kind pair beside the section value's own constructor
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Found by `the-section-of-a-surface-pair-is-one-object` (2026-10-27).

## Measured

- `geom_brep::intersect::route` is the C5 table: an exhaustive
  `(SurfaceKind, SurfaceKind)` match whose `implemented` is per kind
  pair, and `route_pose` asks the same pairs again per pose, a second
  exhaustive match.
- `geom_brep::pair_section::pair_section`, the section value's one
  constructor, is a third: cylinder × sphere builds, every other pair
  answers `SectionRead::Elsewhere(route(..))`.
- `PairSection::serves` (`crates/geom-brep/src/pair_section.rs`)
  restates which kinds the value builds for, a one-kind filter the glue
  door's scan reads (`topo::boolean::glue::section_door`): a fourth
  spelling. It cannot be derived from the match, which answers two
  surfaces, where the scan asks of one face's kind before it has a
  partner.
- `route_pose`'s `(Cylinder, Sphere)` arm reads the value's pose
  (`CylinderSpherePose::decide`) for its answer, so its pose reading is
  the value's; the arm itself is still named there as well as in
  `pair_section`'s match.
- So a pair's arm is named in four places for cylinder × sphere:
  `route`'s `implemented: true` note (which now names the value and
  `ssi::march_cylinder_sphere`), `route_pose`'s arm, `pair_section`'s
  arm and `serves`. Nothing but the rows of
  `crates/geom-brep/tests/intersect_table.rs` and
  `crates/geom-brep/tests/pair_section.rs` ties them.

## What a fix has to supply

`route`'s answer read off the value's arms, so the kind dispatch is
spelled once. It cannot be derived today: `route` answers a kind pair
with no surfaces, and the value answers two surfaces, so `implemented`
would have to become a statement about the arm's existence rather than
its pose. The natural time is step 3 of `plan.md`, when the
classifier's arms move into the value and `route_pose`'s pose reading
moves with them.

## Evidence: sphere × torus answers ahead of its consumers (2026-10-28)

From the review of PR 5886 (`sect/torus-certificate`). That PR flips
`route(Sphere, Torus)` to `implemented: true`, because
`ssi::sphere_torus_ssi` certifies the pair, but `pair_section` has no
sphere × torus arm. `topo::replace_face` and `shell` pass the route gate
for a sphere × torus neighbour, then refuse at `section_closed` with
`SectionVerdict::Unsupported`. Before the flip they refused
`NeighborPairUnroutable`. The refusal is vaguer, and no wrong body comes
of it. The `implemented` flag answers per kind ahead of the consumers
that read it, which is this row's two-dispatch shape again.

## A third state (2026-10-29, SPLICE, PR 6026)

`pair_section_with` builds a plane against a NURBS or approximating
surface through the march door, and `pair_section` with no door refuses
it (`SectionError::NoMarch`). `serves` stays false for both kinds, so a
kind pair is now built by classification, built only with a lane, or
served elsewhere, and `serves` spells the first alone. The section
table's filter relies on it: a reader that bypassed `serves` would read
`NoMarch` there, which the interior-loop guard reads as R-reach
(`section_table.rs` `SectionReader::read`).
