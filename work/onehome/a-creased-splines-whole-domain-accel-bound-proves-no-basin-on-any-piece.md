---
id: a-creased-splines-whole-domain-accel-bound-proves-no-basin-on-any-piece
kind: issue
title: A creased spline's whole-domain accel bound is +∞, so the walk proves no basin on any piece of it, transverse crossings included
status: closed
opened: 2026-11-16
priority: P2
cost: M
closed: 2026-11-16
branch: onehome/creased-accel-per-span
---
## Closed (2026-11-16): the window reads the curvature of its own run

`NurbsCurve3::accel_upper_bound_over` (`crates/geom/src/curves/nurbs.rs`)
bounds the second derivative over a parameter run. It takes the max of
the per-span bounds whose spans the run meets, so a crease costs only the
runs that hold it: a crease inside the run, or not clear of it at its
ends, reads +∞. The per-span bounds are the same `spline::accel_pieces`
maxima the whole-domain bound reduces, chunked by span and memoised beside
it. `geom::carrier_speed::curvature_hi_over` is the curvature door over a
run, `accel_hi / speed_lo²` as before. The walk's `window`
(`crates/geom-brep/src/curve_pair/walk.rs`) now reads each carrier's
curvature through `Read::curvature_near`: the run within the window's
`span` of the pair, widened by the carrier's speed floor. That is the
reach premise P2 states.

`a_spline_with_a_corner_meets_a_line_past_it`
(`crates/geom-brep/src/curve_pair/meet_rows.rs`) pins a transverse
crossing on a creased spline, away from the crease, as basin-proved. The
seam row's crossings now prove their basin. On the probe branch with this
merged, under `CAD_RING_PROBE_REFUSE`, both rows pass and every walk read
they log is `a-walk`.

`pieces_apart` and the floor still read the whole-carrier accel and
curvature bounds. Neither one stands between the seam row and its basin.

Found by the ring-width probe for the walk-stop fork (probe branch
`onehome/ring-width-probe`, never merged, @96b64d1e30), 2026-11-16.

The walk reads each carrier's curvature once, for the whole carrier:
`Read::of` (`crates/geom-brep/src/curve_pair/walk.rs:71`) takes
`geom::carrier_speed::curvature_hi`. On a spline that is
`accel_hi / speed_lo²` (`crates/geom/src/carrier_speed.rs:257`), and
`accel_hi` is `NurbsCurve3::accel_upper_bound`
(`crates/geom/src/curves/nurbs.rs:1288`, its doc at ~1263). That bound is
whole-domain, and it is +∞ wherever the knots hold a crease
(`KnotStructure::crease_runs`).

The bound is already computed per span: the max over `spline::accel_pieces`
(`SPEED_PIECE_SPLITS` pieces per nonempty span), each piece bounding its own
open span. It is then max-reduced over the domain. So on a creased spline
`K = ∞` on every piece. The window's basin (`window`, `walk.rs:276`, on
`curve_pair_walk_basin`) is `sin φ − 2K·span > 0`, and it cannot prove
anywhere on such a carrier, however square the crossing.

`accel_upper_bound`'s doc states the cost as "a creased spline reads no
second-order bound even inside a smooth span, so its touches and tangent
meetings walk to the band". The cost also covers transverse crossings. In
`ring_path::spline_wall_rows::an_island_across_a_closed_walls_seam_is_read_but_not_from_just_below_the_seam`
(`crates/topo/src/ring_path/spline_wall_rows.rs:322`), all 7,800 ringed
walk skips are a Nurbs×Nurbs interior crossing with sine ≈ 1
(0.99999…) and span 4e-9 to 1.2e-8. Carrier a's speed floor is 5.655 and its
accel bound is +∞. The basin there needs only K < 5e7.

The walk-stop fork's ring rule keeps only the window certificate (two-sided,
or one-sided at a held end) and otherwise escalates. Under that rule this row
is red at every crossing on a creased spline carrier. The fork depends on this
row: the per-piece bound lands with the rule or before it.

## Done when

The walk's window reads a curvature bound for its own pieces, not the whole
carrier's. A piece inside one smooth knot span takes that span's
second-order bound, so a crease elsewhere on the carrier costs it nothing.
The seam row's crossings prove their basin, and a row pins a transverse
crossing on a creased spline, away from the crease, as basin-proved.
