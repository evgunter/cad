---
id: a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex
kind: unit
title: A section ellipse is re-posed at the principal vertex nearest its arc, with a signed span, so its parameter resolves to model scale
status: closed
opened: 2026-10-12
priority: P2
cost: M
parent: a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join
branch: cleave/section-ellipse-re-pose
closed: 2026-10-14
---

Part 1 of the design on
`a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join.md`
(read its "Designed" section).

## What

A section ellipse's parameter resolves only to ulp(θ)·A where the face
uses it. A plane nearly along a cylinder's axis cuts an ellipse with
A ≈ r/θ, and the face's part of it sits near θ = ±π/2: at θ = 3e-5
that is 7.3e-12. A clockwise arc's span, computed as 2π minus a small
span, adds ulp(2π)·speed. This is the kernel's share of the in-band
and definite endpoint misses at 1e-12. It is also, likely, FLUX's
`a-slab-face-nearly-along-a-cylinder-ruling-exhausts-the-volume-quadrature`.

## Owed

- Each section chord's ellipse puts its seam (θ = 0) at the principal
  vertex nearest its arc. The choice must be a function of the inputs
  (D9), with a documented deterministic tie-break.
- Its parameter window is a signed span about the seam, never reduced
  through 2π (`oriented_arc`'s clockwise branch).
- `Curve3::Ellipse`'s convention becomes "`u_ref` is a principal
  direction". Rename the two semi-axis fields so neither name claims
  "major", and retire `AxesSwapped`. Say in the PR that restfront's
  `an-ellipse-stored-minor-over-major-passes-tier-3` becomes the
  intended state.
- Readers that assume `u_ref` on the major axis:
  - `chart_pcurve`'s cone/`Ellipse` focal-section image: read the
    major frame through `FocalImage`'s `t0` phase field.
  - `arc_leaving`, which levers by `conic.sa` and must take the larger
    semi-axis.
  - Sweep for the rest, by shape, not only by these names.
- The prototype diff is at the orchestrator's scratchpad
  `se1/proto3.diff`; it is a sketch, not the spec.
- Pins at all three ε:
  - the review's coin-flip family (66 poses) and through-axis planes
    θ = 1e-6…3e-4, which build;
  - the cone family, unchanged;
  - FLUX's quadrature pose, re-measured and noted on its row.

The seam crosses CURVED's (`geom/src/curves.rs`), GERM's (`intersect.rs`)
and PCERT's (`pcurve_cache.rs` / `chart_pcurve`) ground; the PR announces it.

## Built (branch cleave/section-ellipse-re-pose)

- `geom::Curve3::seamed_near` seams an ellipse at the principal vertex
  nearest a parameter. It picks the dominant of the eccentric anomaly's
  cosine and sine; a tie keeps the seam's axis, and a zero points
  forward. Its quarter turn, `Curve3::quarter_turned_where`,
  renormalizes the turned seam. `chord_join::SectionConic::seamed_near`
  routes through it at both chord lanes, and so do `major_frame` and
  `section_frame`. `oriented_arc` reads each span forward from its own
  start. Circles keep their seam.
- `Curve3::Ellipse { semi_u, semi_v }`: `u_ref` is a principal
  direction, in either order. `AxesSwapped` is retired.
  `Curve3::semi_axes_by_size` is the one reading of the larger and the
  smaller semi-axis. Every inline spelling routes through it, as does
  `geom_brep::Conic` (fields `semi_u`/`semi_v`). Aliased `major`/`minor`
  binds of the two fields are renamed to the fields.
- `topo::readback::edge_pose` reads an ellipse's `u_ref` through
  `major_frame`, so `Pose::u_ref` stays its major direction.
- Measured at rest (every split side validated; CLEAVE's probe on head
  against main `98353b4835`):
  - coin-flip family, ε = 1e-12: 66/66 valid at rest, where main
    refused 56 at the join's endpoint pin;
  - through-axis planes, ε = 1e-12: 64/96 valid. The other 32 refuse at
    rest on the frame margin, which is RESTFRONT's
    `tier-3s-frame-check-reads-a-section-ellipses-rounding-amplified-by-its-semi-axis`.
    Main refused all 96 at the join.
  - ε = 1e-9 and 1e-6: unchanged from main. That includes 18 coin-flip
    poses at 1e-6 whose sliver side refuses `VolumeUncomputable` at
    rest, as on main.
  - The off-axis family (704 poses, 1e-12) has 89 more valid and 1
    regression (`w0.7 φ3.9 θ1e-4 z0.5`, now the join's `EndpointStart`
    pin), and some poses changed stage. The PR body lists them.
- Pins: `sweep` `section_ellipse_seam_poses`, which tables every pose's
  outcome at rest per ε row and carries the readback and slab rows;
  `topo` `chord_join::seam_rows`; `geom`
  `a_reseam_goes_to_the_vertex_its_direction_names`,
  `the_major_frame_is_read_from_the_semi_axes` and
  `a_section_frame_is_read_off_the_axis_of_revolution`; and `geom-brep`
  `pcurve_cone_section` at every principal seam.
- Re-baselined: `sweep`'s thread-count digest (`tilted_cut_upper`'s
  pads narrow at 1e-6 and 1e-12, from the re-seamed chords) and
  `step-export`'s `cut_cylinder.expect` pad. The montage-uv cells
  re-render.
