---
id: an-extrude-cap-plane-normal-reads-the-profile-variables
kind: issue
title: an extrude's cap plane takes its normal from a Newell sum over the profile's points, so at Sym the normal is A/sqrt(A²) in the profile's variables, where the sketch plane states it
status: open
opened: 2026-10-21
priority: P2
cost: M
refs: [5587]
---

## What

D10 `[coincidence.structural]` says an extrude's end cap "comes out as
`plane(frame, direction, depth)` with no profile variable in it". After
stage 3 D a profile is drawn on its own `xy`, so a cap's normal is ±z by
construction. The cap's carrier is not built that way:

- `crates/sweep/src/swept.rs` `cap_plane` takes the plane from
  `geom_brep::newell::newell_plane(&ordered, band)`.
- `newell_plane` (`crates/geom-brep/src/newell.rs`) returns
  `normal_sum.normalize()`. For an `xy` profile, `normal_sum = (0, 0, A)`,
  where `A` is the shoelace sum over the profile's points.
- `cap_plane` reads the sketch's own normal
  (`place.linear.c2.normalize()`), but only to decide the orientation
  (`cap_plane_orientation`). It then keeps or flips Newell's normal.

At `Sym` the profile's points are symbols, so the cap normal is
`A / sqrt(A²)` in the profile's variables. It is a function of them where
D10 says it is none. Every identity a cap's normal enters then has to
see `sqrt(A²)·sqrt(A²) = A²` through the forms built on it, and those
forms freeze past the door's budget.

## Why it matters now

A mated seat no longer needs this: the door reads it by the mate that
states it (`[coincidence.structural]`, `crates/editor-core/src/coincide.rs`
`stated_by_a_mate`), with no replay. What still pays for the cap's
normal is the replay of the rows no mate states
(`crates/editor-core/src/coincide/replay.rs`): an at-rest row between
two copies' caps, such as two flush copies of a pattern standing by
the identity bundle, which the replay reads by the carrier verdict on
the two caps at `Sym`, and a node's row on a cap, such as a flush
union's. Each such cap carries `A / sqrt(A²)` into the forms the door
has to discharge.

## Measured (INTENT seat re-measure, 2026-10-21)

Measured on the mated seat, before the door read it by its mate: local,
debug, on main at `eb6894fe`, with the replay of placed copies switched
on. A scratch change in `cap_plane` returned the sketch plane's outward
normal with Newell's origin, once the orientation was decided. Under it:

- mate6's mated seat
  (`mate6_gather_mints::a_mated_record_cites_the_census_row_the_lint_reports`'s
  stand) proves at the door's 256 terms. Without the change it proves
  only from 512.
- The seat's frozen forms went from 59 to 12.
- A top seated on a linear pattern's copy also proves at 256. Without the
  change it was unproven even at 16384 terms.

The full numbers are in
`work/intent/a-mate-placed-seat-is-unproven-at-the-doors-budget.md`
§ "Measured after D (2026-10-21)".

## Priority

P1 rested on the mated seat, which the door now proves without a
replay. No unmated row has been measured to need this change, so it is
P2 until one is: the rows that freeze on the door's budget today
(`glue_on_zero_rows::a_flush_rest_glues_and_records_its_unproven_rows`,
`coincidence_door::a_filleted_box_records_one_unproven_turn_per_corner`)
are the first to measure with the scratch change above.

## Owed

Build the cap carrier from the construction: the sketch plane's normal
(oriented as `cap_plane` decides), at the cap's offset along the sweep.
Do not re-derive it from the points. A Newell plane stays the right
reading where the points are not known coplanar by construction. The
row: at `Sym`, an extrude's two cap normals are exactly `±z`, with no
profile symbol in them.
