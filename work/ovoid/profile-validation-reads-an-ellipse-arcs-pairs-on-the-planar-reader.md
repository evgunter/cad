---
id: profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader
kind: unit
title: Profile validation reads line x ellipse, circle x ellipse and ellipse x ellipse pairs on the planar meeting reader
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: an-ovoid-cannot-be-authored
blocked_on: [a-profile-segment-is-an-ellipse-arc, profile-validation-reads-its-pairs-through-the-plane-instance]
---



Unit 4 of `an-ovoid-cannot-be-authored` (Ev, PR 7289): "Profile
validation: line×ellipse, circle×ellipse and ellipse×ellipse rows". It
is **wiring, not a reader**. Ev's planar-readings ruling of the same day
(PR 7290, merged) puts every planar carrier-pair reading in one
`geom-brep` reader, a plane instance of C5's co-surface arm
(`crates/geom-brep/README.md` C5), which profile's pair pass reads
through a new `geom-brep` dependency; that build is PROFILE's
`check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`
(its "Weighed" section gives the order). The reader's table already
routes the three ellipse pairs (`geom_brep::curve_pair`'s `Method::Closed`
rows `Arm::LineConic`, `Arm::EllipseCircle`/`CircleEllipse`,
`Arm::EllipseEllipse`, read in `curve_pair/pose.rs` by `line_conic`
and `conic_conic` → `coplanar_ellipses`). So **the ellipse pair rows
are that build's reader's, and this unit writes none of its own**: it
connects the ellipse segment to the reader once profile reads it, and
pins the rows.

## What is true when it is done

- `seg::pair_contacts`'s typed refusal for a pair with an ellipse
  (left by `a-profile-segment-is-an-ellipse-arc`) is gone; the pair
  pass hands the pair to the reader, as it hands line and arc pairs
  once the planar build's "profile onto it" step lands.
- Rejection rows: a crossing, a tangent touch, an overlap on one
  carrier, near-concentric ellipse × circle, and clear pairs, for each
  of line × ellipse, circle × ellipse, ellipse × ellipse, at every
  lane scalar; a meeting carries its kind (the reader's
  `curve_pair::Meeting` kind, from the same build).
- Containment of one loop in another reads an ellipse arc through the
  planar build's ray driver.
- The lily-shaped meridian (a disc line, a neck cone, an ellipse belly,
  a pucker cone, a lip line, the axis close) validates.

## Check first

The designers of PR 7290 did not trace `curve_pair/pose.rs`'s line ×
conic level arm or `coplanar_ellipses`. And the ovoid's designers'
"quartic, certified closed form" for the ellipse pairs is not what the
table does: `coplanar_ellipses` sends the pair to the certified
subdivision on a level of degree two in the first's angle. Read both
arms against the three planar defects PR 7290's designers measured on
`meet` (an in-band carrier clearance escalating whatever the spans; a
tangent touch returned as an interior meeting; no kind on a meeting)
and check the planar build fixed them for conic arms too. If an
ellipse arm still has one, **stop and report** on PROFILE's log: the
fix is the reader's, not a profile-side row.

## Order

Parked on `a-profile-segment-is-an-ellipse-arc` and on the planar
build. It waits on the whole `check-9-arm-5` row only because that
build is not yet cut into units; **when it is, re-park this unit on
the unit that moves profile's pair pass onto the reader** (the build
order's seventh step). This is the critical path to every multi-segment
ellipse profile, so to the spheroid's mint (question in the cut PR:
whether to wait, or read `curve_pair::meet` early with its known
defects).

## Leaves for later

- The quartic sign ladder (`crates/geom-brep/src/quartic.rs`) stays
  where it is: profile reads it through `geom-brep`, so nothing moves
  down to `geom-core`.

## Rows and ground

- Closes the ellipse half of part 1. Changes no other row.
- Crosses CPAIR and ONECOPY (`curve_pair/pose.rs`) only if a check
  fails, and then it stops.
- Review: **single (FULL)**: the rows are the substance. Difficulty
  **M**.
