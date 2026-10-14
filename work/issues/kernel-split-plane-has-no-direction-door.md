---
id: kernel-split-plane-has-no-direction-door
kind: issue
title: A kernel caller holding a plane direction must name a K site and form a Band to build a SplitPlane — the convenience exists only as a test fixture
status: open
opened: 2026-10-02
---


## What

Since `split-refuses-cylindrical-feature-box` (PR 3768),
`topo::splitting::SplitPlane.normal` is a `geom_core::UnitVec3`
(`crates/topo/src/splitting/mod.rs:99`). That is the right type: the
plane×conic sections read the normal's components as direction
cosines, and a direction that reached them unnormalized was P0's
cause. But a kernel caller authors a cutting plane the way the tour's
two cut scenes do — a point and a direction — and turning that
direction into a `SplitPlane` now takes three imports and two
`expect`s that say nothing about the plane:

- `demos/tour/src/cutaway.rs:35-37` and `demos/tour/src/curvedcut.rs:67-71`
  each form `Band::linear(tol)`, then call
  `UnitVec3::new(v, DATUM_UNIT_NORM, band)`. The K site name is
  borrowed from `topo::query`'s datum funnel (`crates/topo/src/query.rs:407`),
  which owns the editor's plane datums, not a kernel caller's split.
- The shape a caller wants already exists, test-only:
  `topo::test_support::split_plane(origin, dir, tol)`
  (`crates/topo/src/test_support_fixtures.rs:185`), under its own site
  `fixture_split_normal`. Every one of the ~100 fixtures the PR
  rewrote goes through it.
- The DOCUMENT layer does not have the friction: `Node.datum_plane`
  takes a direction and normalizes it under `datum_unit_norm`
  (`crates/pncad-py/tests/test_north_star.py:4494`, the cutaway's
  document spelling), so the gap is the kernel door only.
- The tour already met this shape once for frames and answered it
  with a tour helper wearing the tour's own band and funnel name
  (`demos/tour/src/scalar.rs:28`, `TOUR_FRAME_AXIS`); the two cut
  scenes did not take that route, so the band-and-mint spelling is
  now written twice in the demos.

`memories/demo-purpose.md` makes demo awkwardness a library finding;
PR 3768's body names this friction and says "not filed". This row is
that filing.

## What must be decided

Whether a kernel split plane gets a direction door (a constructor that
decides the length under a site the splitting lane owns and returns
the `UnitVec3` refusal typed), whether `UnitVec3` grows a caller-side
convenience instead, or whether the friction is the intended cost of
the ruling (PR 2457) and the demos should say so at the site.

## Found by

Review of PR 3768 (`tquery/split-cyl-feature`), 2026-10-02.
