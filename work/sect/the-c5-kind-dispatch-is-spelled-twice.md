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
  answers `SectionRead::Elsewhere(route(..))`. `pair_section::serves`
  restates which kinds it builds for, a one-kind filter the glue door's
  scan reads.
- So a pair's arm is named twice for cylinder × sphere: `route`'s
  `implemented: true` note, and `pair_section`'s arm. Nothing ties
  them; `route`'s note still describes the pair as two separate doors
  (`cylinder_sphere_section`, `cylinder_sphere_ssi`).

## What a fix has to supply

`route`'s answer read off the value's arms, so the kind dispatch is
spelled once. It cannot be derived today: `route` answers a kind pair
with no surfaces, and the value answers two surfaces, so `implemented`
would have to become a statement about the arm's existence rather than
its pose. The natural time is step 3 of `plan.md`, when the
classifier's arms move into the value and `route_pose`'s pose reading
moves with them.
