---
id: one-stored-form-waits-on-the-readers-of-a-conics-centre
kind: issue
title: One stored form for every conic waits on the readers of a conic's centre: under a derived centre they widen at Interval and part from the evaluator at f64
status: open
opened: 2026-11-21
priority: P1
cost: H
refs: [the-section-table-admits-a-curve-on-its-resolution, a-conics-points-are-restated-in-centre-form-outside-its-evaluator, a-section-conic-is-anchored-at-a-general-point-of-its-arc]
---


## Finding

One stored form for every conic (PR 7194: anchored, the centre derived
and never stored) was built on `oneround/anchored-conic-one-form` at
`e04a87227` and measured. A door handed a centre sums the anchor once,
`anchor = c + u·a`, and `Conic::center()` / `Conic::frame()` derive the
centre back on each read (`crates/geom/src/curves/conic.rs`). Every
reader that takes a conic's centre as a datum then reads
`(c + u·a) − u·a`:

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

| family | where | what it showed under a derived centre |
|---|---|---|
| an arc's cover | `Curve3::cover_over`, `crates/geom/src/cover/` | **unsound at a far centre**: at `a = 1e7` the cover's ball misses the arc's own point by 4.2e-10 m (`arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size`) |
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

## What this row is

The prerequisite of one stored form, whatever is decided about what a
reader of a centre should read: until the readers above read the
conic's own form (or the centre they read is not derived through the
semi-axis), one form turns builds into refusals on the certified lane
and leaves a cover that does not hold its arc. The other prerequisite is
a door that can anchor on the arc
(`a-section-conic-is-anchored-at-a-general-point-of-its-arc.md`): with a
vertex anchor an arc near another vertex of a long ellipse is summed
across a semi-axis (6.5e-13 where
`seam_rows::an_arc_near_a_minor_vertex_resolves_at_its_own_scale` holds
1e-14).

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
