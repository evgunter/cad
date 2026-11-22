---
id: one-stored-form-waits-on-the-readers-of-a-conics-centre
kind: issue
title: Every reader that rebuilds or restates a conic from its centre goes through the conic: the sites, and what each showed when the centre was a derived one
status: open
opened: 2026-11-21
priority: P1
cost: H
refs: [the-section-table-admits-a-curve-on-its-resolution, a-conics-points-are-restated-in-centre-form-outside-its-evaluator, a-section-conic-is-anchored-at-a-general-point-of-its-arc]
---


## What is owed

A conic is one of two placements behind the one `Conic` reading, held
about its centre or about a point of the arc it serves, and every
reading of where a conic is goes through the conic (PR 7258, approved).
There is no single stored form to wait for. What is owed is the last
clause: the readers below take a conic's centre as a datum and rebuild
or restate the conic from it themselves, so each of them reads an
arc-held conic through a centre that is derived, and loses or misreads
the placement.

This row is the spec of the readers unit: each family below, and each
door on the list after it, reads the conic (its point, its cover, its
image, its placement) and not `frame().origin()` and the sizes.

## What a derived centre costs these readers, measured

Measured once with EVERY conic's centre derived
(`oneround/anchored-conic-one-form` at `e04a87227`, a build in which a
door handed a centre summed the anchor `c + u·a` once and
`Conic::center()` / `Conic::frame()` derived the centre back on each
read). That build is not the design; it is the measurement of what
these readers do with a derived centre, which an arc-held conic gives
them today. Every reader that takes a conic's centre as a datum then
reads `(c + u·a) − u·a`:

- at `Interval`, the semi-axis's width and `a` times the direction's
  enter the centre twice over, and a point half a turn from the anchor
  carries them three times where the centre form carries them once;
- at `f64`, the derived centre is `fl(fl(c + u·a) − u·a)`, a rounding of
  the semi-axis off `c`, so anything stated from the centre is a
  different conic from the one the evaluator draws.

The rows it moved, with their numbers, are on
`analysis/oneround/conic-vertex-probes` under
`analysis/oneround/conic-vertex/step-b-one-form/`.

## The families of reader, and what each showed

The unit's earlier sweep counted about 90 production sites that read a
conic's centre as a datum. By family:

| family | where | what it showed with every centre derived |
|---|---|---|
| an arc's cover | `Curve3::cover_over`, `crates/geom/src/cover/` | at `a = 1e7` the cover's ball misses the evaluator's point by 4.2e-10 m (`arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size`). **The ball misses the arc's TRUE point by that much on `main` already**: there the evaluator makes the cover's own 4.2e-10 error (`1e7 − 1e7·cos t`) and the row passes; under one form the evaluator is right to 6e-17 and the cover is the one left wrong (the analysis branch's `two-facts.md`) |
| an edge's extent bound | `crates/geom-brep/src/extent.rs` | **short by an ulp**: a circle's sup 4 under the evaluated diameter 4.000000000000001 (`edge_extent_bounds`) |
| chart-frame readers of a circle leg | `crates/topo/src/ruled_region.rs`, `torus_region.rs` | the count of queries a frame answers moves (35 → 36, 33 → 51); not shown wrong |
| an edge's chart image | certification's check 4, `crates/geom-brep/src/certify.rs` (`pcurve_map_residual`) | a residual of an ulp of the radius where zero; a carried certificate no longer equals a fresh one |
| `Curve3::param_near` | `crates/step-import/src/build.rs` (`settled_foot`), and every foot read | `eval ∘ param_near` no longer idempotent; the STEP round trip's fixed point goes by an ulp |
| two statements of one circle compared by bit | `crates/mesh/src/curved.rs` (the pinch read) | another face refuses first |
| the certified lane's readers at `Interval` | blend's contact edge (`shared_row_touch`), rung 3's `ssi_hull_sup`, profile validation, the cone lane's envelope, the graze side | four builds become refusals, and the widths in the analysis table |
| the symbolic scalar | `sweep::swept`'s placed rim, limb 2's memo | a registration is lost: `c + u·r − u·r` is not recognised as `c` |

The fourth to sixth rows are
`a-conics-points-are-restated-in-centre-form-outside-its-evaluator.md`'s
three; the cover, the extent bound and the chart-frame readers are the
same class and are not on that row.

## Where it bites today

Only an ellipse can be held on its arc, and only `plane_cone_section`
mints one so. For it: the cover and the chart image are still stated
from the derived centre
(`the-arc-cover-misses-a-far-centred-arc-by-its-centres-rounding-and-the-depth-gap-decides-on-it`,
`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`).
The families above are where the same reading would bite every conic
that comes to be held on its arc
(`a-construction-whose-centre-runs-off-still-stores-a-centre`,
`a-section-conic-is-anchored-at-a-general-point-of-its-arc`).

## The doors that rebuild a conic through a centre constructor

Each reads a conic's centre frame and builds a conic from it with
`Circle::new` / `Curve3::circle` (or an ellipse's), so its result is
held at a centre whatever its source was. Listed by both reviews of
PR 7203; none meets an arc-held conic today, because every one of the
same-locus sites takes a circle and a circle is held at its centre
only. They are the sites that lose the placement the day a circle can
be held on its arc.

- Same locus, the placement lost: `crates/topo/src/replace_face.rs`
  (the three doors the unit named, circle arms only),
  `crates/topo/src/offset_axial.rs` (two), `crates/topo/src/ring_path.rs`
  (`torus_circle_arc`), the `Rim` round trip
  (`crates/topo/src/rim_wedge.rs`, `crates/topo/src/boolean/mod.rs`, two
  sites), `MeetSegment::Arc` in `crates/topo/src/validate.rs`,
  `crates/geom-brep/src/curve_pair/on_surface.rs`, and `crates/sweep/src/swept.rs`
  (`circle_at`, to evaluate).
- Moved, or another curve: `crates/geom/src/pole_form.rs`
  (`half_turn_curve`, which also re-squares the frame),
  `crates/sweep/src/blend/junction.rs`,
  `crates/sweep/src/blend/open/end_face.rs`.
- From an ellipse's derived centre frame, a different conic:
  `crates/topo/src/ring_path.rs` (`ConicArc::distance_floor`),
  `crates/sweep/src/blend/surgery.rs` (`piece_along`).

`crates/step-export/src/writer.rs` writes the derived centre, which is
Ev's decision on PR 4812 and not one of these.

## Not established

- The exact count per family. The table names where each was seen, not
  every site.
- Six moved rows were not checked against an oracle (the analysis
  README's "moved, not checked"). None is shown wrong on `main` today.
  The one suspicion they leave about `main` is already on the
  restatements row: a predicate that holds only while two statements of
  one circle agree to the bit (`mesh`'s pinch read, and perhaps
  `SelfTouchingTrimLoop`, which
  `a_slit_tip_on_a_curved_face_reads_its_full_turn::the_slit_block_meshes`
  now trips).
