---
id: certificate-and-meter-readings-still-fold-per-coordinate-boxes-into-norms
kind: issue
title: Certificate and meter readings outside the offset fit still fold per-coordinate boxes into norms, which [tolerance.residuals] forbids
status: open
opened: 2026-11-12
priority: P2
cost: H
---


Filed by ONEHOME's certifier fix (PR 6820), from the sweep fork 259
asked for after PR 6824 ratified `[tolerance.residuals]`: "a
vector-valued bound is read from the Euclidean norm of each
coefficient, never from a per-coordinate box folded into a norm", so a
rigid map moves it only by its rounding width.

PR 6820 re-posed the offset fit's: `‖E‖`'s floor
(`PatchSpans::cell_norm_inf`) and the curvature meter's forms
(`PatchCell::balls`, `offset_meters::cell_curvature`). The readings
below still build an ambient-frame box (componentwise hulls, `rect_box`
/ `deriv_box`, per-axis gaps) and fold it into a norm or a length, so a
rotation of the geometry moves them by more than rounding. Each feeds
a certificate's or a meter's decision. Line numbers are at PR 6820's
head.

## Offset meters (not re-posed in PR 6820)

- `crates/geom-brep/src/offset_meters.rs` `cell_distance`: per-axis gaps
  of the cell's spatial box folded as `√Σg²`, the lower bound
  `chart_arm` reads (the lower end of `classify_seam`'s bracket, so its
  slack can cost an escalation and never decide a class).

## SSI implicit-partner readings (plane, cylinder, sphere, torus forms over a box)

- `crates/geom-brep/src/ssi/residual.rs` `ImplicitForm::gradient_sup`
  (`√(gx²+gy²+gz²)` of the frame-gradient box): the divisor of limb 3's
  chart-tube margin (`ssi/enclose.rs`), `ssi/boundary.rs`'s `sine`
  (`ssi_boundary_strip`), `pair_section/spline_implicit/sweep.rs`'s
  `g_len` (`spline_sweep_regular`), and `section.rs`'s slope divisor
  (`decide_crossing`).
- `ssi/residual.rs` `ceiling_metres` and `floor_metres` (radial reach of
  the frame box; `pcurve_cache/projected.rs`'s `Meters::divisor`), read by
  `section.rs`, `pair_section/spline_implicit/curved.rs`,
  `pair_section/spline_implicit/sweep/critical.rs` and `ssi/boundary.rs`.
- `ssi/enclose.rs` `implicit_enclosure` / `implicit_gradient_enclosure`
  (`norm_sq(box − c)`): `graph_margin` → `certify.rs`'s
  `box_graph_margin` (limb 3's spatial tube margin over per-axis
  control-point hulls padded as cubes) and the one-arc Krawczyk proof
  (`ssi/one_arc.rs`).
- `ssi/enclose.rs` `Box3::centre_and_reach` (an L1 fold of half-widths):
  the section component's site-band pad (`ssi/pair.rs`
  `branch_witness`, `pair_section/spline_pair.rs`) and the plane ×
  spline bracket and critical-point witness pads
  (`pair_section/spline_implicit.rs`).
- `ssi/one_arc.rs` `norm3` / `farthest` / `nearest` (box-to-box gaps,
  then a norm; `crossing_near`, limb 3's end reach) and
  `r3_reaches_end` (a cube standing in for an ε-ball).
- `ssi/boundary.rs` `beyond_reach` (`√Σ mig²` of `deriv_box`), which
  settles `side_stretch`'s `false`.
- `pair_section/spline_implicit/sweep/critical.rs` `along_line` (norms
  of jet boxes: the crease-line slope's sine; the sector lengths).

## Curve × face, spline incidence

- `crates/geom-brep/src/curve_face/spline_face.rs` `past_an_end`
  (`PAST_THE_END`) and the `AT_THE_END` `far` margin (norms of
  `deriv_box` tangents).
- `crates/geom-brep/src/pcurve_cache/spline_incidence.rs` `box_gap`, maxed
  with the frame-free `slab_gap` into the D3 incidence `Off` floor.

## Mass properties (`crates/geom-brep/src/props/quad`)

- `patch.rs` and `trimmed.rs`'s `p_bound` (a 1-norm of the position
  box, the flux pad's lever), `rational.rs`'s over a weight hull.
- `patch.rs`, `trimmed.rs` `area_cell`, `rational.rs` `area_blocks`:
  the integrand hull `‖cross(h_su, h_sv)‖` of hull boxes and the
  Lipschitz pads `norm_hi(d_u/d_v)`.
- `trimmed.rs` `sup_g`, `rate_u/v`, `col_g`, `lune_g` (the `widen` pads
  and the extent gate) and its `norm_lo` rate floor
  (`props_trim_piece_monotone`).
- `rotational.rs`'s chart-term pad (a lever's 1-norm) and `spiric.rs`'s
  planar cap area (the norm of a vector-area box).

## Levers, balls, topo, mesh

- `crates/geom/src/cover.rs` `box_ball` / `box_range` (`Cover::ball`,
  `distance_range`, the `ExtentBall::of_cover` levers) and
  `crates/geom-brep/src/extent.rs` `of_overlap`, `of_box`,
  `box_diagonal`; their callers `crates/topo/src/separating.rs`
  (`planar_axis`), `census/in_plane.rs` (the steep margin's lever),
  `boolean/carrier_pair.rs`, `census.rs`'s `face_reach`.
- `crates/topo/src/census.rs` (a face's vertex box, per-axis gap then a
  norm: census proximity) and `crates/topo/src/clearance.rs`
  `separation` (the norm of two per-coordinate cell enclosures'
  difference: the clearance margin and the `min_separation` bracket).
- `crates/mesh/src/nurbs_cert.rs` (`norm_sq` of `PatchCell`'s signed
  boxes `s_u … s_vv`: the tessellation certificate's partial bounds).
  `PatchCell::balls` now carries the ball form these could read.

## Read and set aside

Readings that are componentwise by construction and fold no norm
(`ssi/four.rs`'s ℝ⁴ residual and Jacobian, `intersect/section_bounds.rs`'s
sine floor from per-axis hulls, `ssi/boundary.rs`'s `plane_partial`),
partner-frame boxes that a common rigid map of both operands does not
move, cost-only boxes (culling, refinement, telemetry), and the session
box (`crates/topo/src/session_box.rs`, a world cube by design) are not
this row's. `crates/topo/src/boxes.rs` and the `bvh` crate were not
traced.

## Done when

Each reading above is re-posed in norm/ball form (or argued, at its
site, to fold no norm), with a row that rotates its input and reads the
decision unmoved up to rounding.
