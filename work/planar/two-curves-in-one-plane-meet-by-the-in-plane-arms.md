---
id: two-curves-in-one-plane-meet-by-the-in-plane-arms
kind: unit
title: C5's plane instance: two spans in one plane meet by the table's in-plane arms, coplanarity given by a type, ends as their holder has them
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [a-pose-proves-two-spans-apart-from-their-covers]
---


PLANAR's second unit. The reader every later unit reads.

## The ruled text

C5 (`crates/geom-brep/README.md`), the co-surface paragraph: *"On a
plane record, and in a sketch plane, two curves both lie in the
surface, so no plane of either is transversal to it: they meet by the
table's own in-plane arms (two lines; a line and a conic by the conic's
distance from the line; two circles; two ellipses), entered with
coplanarity given by the type that holds the pair (two edges of one
plane record, or two segments of one sketch plane) and never read, each
candidate a root in a parameter that is settled by its place on each
span. A span there keeps its ends as its holder has them, a profile's
stored vertices or a face's vertex positions."* PR 6567: none of these
pairs is handed to `meet`.

## What it builds (named in the tree, 2026-11-22)

1. **The coplanarity type**, two constructors and no other: two edges
   of one `Plane` surface record (the keys compared, as
   `curve_pair::on_surface::OnSurface`'s `same_record` compares them,
   refusing two records), and two segments of one sketch plane. Its
   fields are private, so no caller can assert coplanarity otherwise.
2. **A span that keeps given ends.** `curve_pair::span::Span::new`
   evaluates its ends (`ends: (carrier.eval(t0), carrier.eval(t1))`,
   a private field); a second constructor takes them as given. A full
   turn is `Span::endless`.
3. **The lowering of a sketch segment to a planar span**, exhaustive
   over `geom_brep::SketchSegment` (`crates/geom-brep/src/mapped.rs`):
   a line is origin `a`, unit chord direction, span `(0, |b − a|)`; an
   arc is `Curve3::Circle` about its centre with axis `±z` by the sweep's
   sign and `u_ref = (a − c)/r`, span `(0, |Δθ|)`; ends `(a, b)` as
   stored, lifted to the sketch plane's identity frame exactly. No angle
   is computed. A face's edges need no lowering: they are `Curve3`
   spans with their vertices' positions as ends.
4. **The instance's entry**, in the co-surface arm
   (`crates/geom-brep/src/curve_pair/on_surface.rs`, where
   `CoSurface::InPlane` today hands an in-plane pair back to its caller
   unread), or a sibling `curve_pair/plane.rs` if the lane finds the arm
   too full. It routes through `pose.rs`'s own in-plane arms, with no
   second table:
   - two lines: `line_line`, whose offset is the lines' distance, the
     in-plane one for coplanar lines;
   - a line and a conic: `route`'s projection gap, then the level arm of
     `line_conic` (the code after `Route::Level`: `Slant`, `owed`,
     `level.roots`) with the conic's height off the line's plane held
     at zero rather than computed;
   - two circles: `one_curve`, then `coplanar_circles` with `slant` zero;
   - an ellipse and a circle, or two ellipses: `one_curve`, then
     `coplanar_ellipses` (whose level `F` already ignores the height).
   `conic_conic`'s height screen, its swing readings and `transverse`
   are not reached: those read coplanarity.
5. **Candidates settled by place.** `curve_pair::posed` reads a
   candidate with no window bound through `Found::read`, the
   point-on-span reading; in the instance each candidate is its
   parameter on each span (`ta`, and `tb` from the foot), settled by
   `Span::place_param` on each, with `meet`'s stretch rule
   (`Found::finish`) after.

## Done when

Rows in `curve_pair` (a new `plane_rows.rs` or beside
`curve_pair/plane_rows.rs`, which already exists for the plane door),
each at `f64` and `Interval`, both orders, a sketch plane and a plane
record placed off the origin and tilted (normal `(1, 2, 3)/√14`,
origin `(10³, 700, −300)`, ε 10⁻¹²):
- the five span-clear fixtures of
  `a-pose-proves-two-spans-apart-from-their-covers` read no meeting;
- a crossing, a corner joint (a meeting at an end of both), a tangent
  touch, two arcs of one circle sharing a run (`shared: Some`), two
  collinear segments overlapping, a full turn against a line;
- line × ellipse, circle × ellipse and ellipse × ellipse, one crossing
  and one touch each, against the certified walk as oracle (C5: the walk
  is the closed forms' differential oracle);
- the same tilted pairs through `meet` are recorded beside them, so the
  PR shows what the instance decides that the lift did not;
- refusals: an edge pair on two records refuses typed (the arm's
  `OffRecord`); an in-band carrier reading the stage cannot clear still
  escalates, on its own row.
- C5's companion-table row in `crates/geom-brep/README.md` names the
  instance's file.

## First check, and what to do if it fails

**Whether `line_conic`'s level arm separates from its tilt reading.**
`Slant` carries the conic's height `h` along `u × m`; the arm raises
`row::SLANT_TIGHT` (`Unsettled::Meter`) unless `Slant::flat`. Check that
`h` held at zero makes `flat` true and that nothing after `Route::Level`
reads `den` or the route's offset. Then the same for
`coplanar_circles`' `slant` (used only in the `row::RADII` spread) and
`coplanar_ellipses`. **Second**, whether the ends can be read without a
3-D contact: `curve_pair::closed` reads `pose::end_candidates` through
`Found::read` before the pose. In one plane an end's distance from the
other carrier is an in-plane distance, not the zero-by-model gap 6567
refused, so this is expected to hold; say in the PR how it is read. If
either arm cannot be entered without reading coplanarity, or the ends
cannot be read without the 6567 reading, stop and report: that is a
question for the clause, not a spelling to improvise.

## Leaves for later

- Each reader: `check-9-reads-ring-against-outer-through-the-plane-instance`,
  `the-census-reads-one-plane-records-pairs-through-the-plane-instance`,
  `the-seg-differential-agrees-with-the-plane-instance`,
  `profile-validation-reads-its-pairs-through-the-plane-instance`,
  `both-planar-ray-walks-share-one-driver-beside-the-reader`.
- The kind on each meeting: ONECOPY's
  `a-touchs-crossing-or-side-is-re-decided-from-tangents-at-its-one-point`.
- The sweep's own lowering of a segment to a carrier
  (`sweep::swept::placed_segment_spec_about`) is not moved here; whether
  it should read this lowering is a question in the cut's PR.

## Rows

Changes none on its own; it is the reader the parked rows wait for.

## Territory

`crates/geom-brep/src/curve_pair/{pose,span,on_surface}.rs` and
`curve_pair.rs`: CPAIR and ONECOPY. `crates/geom-brep/src/mapped.rs`: no
program. `crates/geom-brep/README.md`'s companion table: a re-word that
lands with the change. Announce on CPAIR's and ONECOPY's logs.

## Review

**Dual**, difficulty **H**: a new public entry to the ratified reader,
the first code under PR 7290's clause, and a refactor of the pose's
line × conic arm that every reader of `meet` runs.
