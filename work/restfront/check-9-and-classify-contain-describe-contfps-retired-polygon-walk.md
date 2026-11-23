---
id: check-9-and-classify-contain-describe-contfps-retired-polygon-walk
kind: issue
title: validate.rs still describes contfp as walking the vertex polygon: classify_contain renders ArcLoopUnsupported as 'arcs over fewer than three corners', and check 9's gate comments say contfp walks ArcParity
status: closed
opened: 2026-09-26
priority: P2
cost: E
pr: 3388
closed: 2026-09-28
---


Filed by CONTACT-4, which moved `boolean::contain::contfp` onto the
carrier walk (`splitting::containment::point_in_carrier_loop`) and did
not edit `validate.rs` (ATREST's ground; ATREST-12 moves check 9 onto
the same walk).

**What is now false in `validate.rs`:**

- `classify_contain`'s `ContainError::ArcLoopUnsupported` arm
  (`validate.rs:2315`) renders *"its boundary is arcs over fewer than
  three corners, which the check cannot read as a region"* with the
  recourse *"split an arc so the boundary has three corners, or draw
  the region as one circle"*. `contfp` now reads a two-vertex arc loop
  (a half-disc, a lens) on its carriers and answers it. The arm fires
  only for a loop with a spiric or spline edge, asked about a point
  within that edge's reach (the walk's `LoopEdge::Unrowed` ball). The
  sentence and its recourse name the wrong mechanism. `ContainError`'s
  own `Display` in `boolean/contain.rs` and
  `BooleanError::ArcLoopContainmentUnsupported`'s now say the true one:
  *"has an edge on a spiric or spline carrier … model the boundary with
  lines, circles or ellipses"*.
- Check 9's gate comment (`validate.rs:5605`, the `ArcParity` bullet)
  says *"`contfp` walks it — one point's verdict"*, and the paragraph
  ending *"Both wait on the general arc-aware walk"* (`:5618`) treats
  that walk as missing. `contfp` no longer consults `loop_shape`, so
  check 9 is `LoopShape`'s only consumer. `LoopShape`, `loop_shape`,
  `LoopCircle` and `disc_side` stay in `boolean/contain.rs` for it.

ATREST-12 is likely to rewrite the gate comment anyway. The
`classify_contain` sentence is user-facing, so it matters more.

**Four more sites** (found by the CONTACT-4 review):

- **`validate.rs:593-596`**, `CensusUnsupportedCause::Containment`'s
  doc: *"an arc-bearing loop no walk expresses"*. The arm now means a
  spiric or spline edge whose ball every scheduled ray from the point
  could meet.
- **`validate.rs:5572-5575`**, check 9's instruments paragraph. It says
  the arm pools `point_in_loop_*` *"the way `boolean::contfp` … already
  pool[s]"*, and that `disc_side` is *"the row `contfp` decides the
  same class on"*. `contfp` reads neither now.
  - An all-line loop reaches `point_in_loop`'s walk without its
    pre-pass, under the same `point_in_loop_*` names.
  - A disc loop reaches the carrier walk under `point_in_arc_loop_*`.
- **`validate.rs:6497`**, `nesting_region`'s doc: *"the classifier
  `boolean::contfp` dispatches its own walks on"*. `contfp` no longer
  asks `loop_shape`.
- **`validate.rs:6530`**, `NestingRegion`'s doc: *"`contfp` dispatches
  on `LoopShape` itself because it answers every class"*. That is no
  longer true for the same reason.

## Re-homed

Moved from `work/atrest/` with ATREST's close, id kept. In CONTACT-4's
reconciliation with ATREST-12, main's `validate.rs` still has three of
the sites:
- `:597`, `CensusUnsupportedCause::Containment`'s doc: *"an
  arc-bearing loop no walk expresses"*;
- `:2468`, `classify_contain`'s `ArcLoopUnsupported` sentence and
  recourse;
- `:6233`, *"the way `boolean::contfp` … already pool[s]"*.

ATREST-12's rewrite retired the other two (the `nesting_region` and
`NestingRegion` docs). Its check 9 no longer reads `disc_side`, which
has no caller left and is deleted.

## Closed

PR 3388 fixed the three sites main still had, plus the references to
the deleted `disc_side`:
- `classify_contain`'s `ArcLoopUnsupported` reason and recourse now say
  a spiric or spline edge near the point, and to model the boundary with
  lines, circles or ellipses (the variant's own `Display`).
- `CensusUnsupportedCause::Containment`'s doc, and the header of
  `crates/sweep/tests/census_containment_cause.rs`, name the same arm.
- Check 9's instruments paragraph lists `contfp` among the
  `point_in_arc_loop_*` pool. It drops the `disc_side` pointer.
- `ring_nesting`'s doc no longer argues against `disc_side`, and
  `docs/KERNEL-VERBS.md` no longer points at that argument.
