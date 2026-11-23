---
id: a-section-conic-is-anchored-at-a-general-point-of-its-arc
kind: issue
title: A section conic can be anchored only at a principal vertex: a cylinder's shallow off-axis cut, whose in-box arc lies half a radian from any vertex, is still stored at its far centre
status: open
opened: 2026-11-20
priority: P1
cost: H
refs: [the-section-table-admits-a-curve-on-its-resolution, a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Filed by `oneround/anchored-conic` (2026-11-20), the unit that built
the anchored form of
`the-section-table-admits-a-curve-on-its-resolution` with the anchor
at a principal vertex only.

## What is not built

An ellipse held on its arc (`geom::Ellipse::anchored`; a circle is
held at its centre only) is minted with its anchor at its point at
`θ = 0`, its vertex on `u_ref`. `plane_cone_section` mints its ellipse
so, at the
near vertex, and its in-box points stand on both surfaces at their own
coordinates' rounding (worst 6.7e-16 m at semi-axes from 1e3 to 1e6,
against 7e-13 … 7e-10 before).

`plane_cylinder_section` does not. A plane `θ` off a cylinder's axis
and off it cuts an ellipse of semi-axis `r/θ` whose centre is where
the plane meets the axis, `d/θ` away; the part of it inside the box
lies 0.42–0.50 rad of eccentric anomaly from the nearest principal
vertex. No vertex is near that arc, so a vertex anchor does nothing
for it. Measured against the exact-rational oracle
(`carrier_resolution::tests::a_far_centred_section_stands_at_the_scale_its_form_is_stored_at`,
which holds the cylinder's sections to `4u` of their SEMI-AXIS and the
cone's to `4u` of their own coordinates; the cylinder half is the pin
that tightens when this row lands):

| semi-axis | samples | worst miss, held at its centre | anchored at a fixed major vertex |
|---|---|---|---|
| 1e4 | 527 | 1.09 u·a (1.7e-12 m) | 2.08 u·a |
| 1e5 | 680 | 1.11 u·a (1.6e-11 m) | 1.41 u·a |
| 1e6 | 680 | 1.16 u·a (2.2e-10 m) | 1.62 u·a |

What this costs today: `carrier_resolution` charges such a carrier
`64·u·a`, and `chord_join` refuses a section end as a kernel
limitation once that, `K` times over, passes ε: past a semi-axis of
about 2e4 at ε = 1e-9
(`chord_join::section_end_rows::an_unresolved_carriers_pin_is_a_kernel_limitation`).
Valid geometry, unbuilt.

## Two shortcuts that do not work

- **A fixed major vertex as the anchor.** It is the in-box one only by
  luck, and elsewhere the terms a point is summed from are `2a` long
  where the centre form's are `a`: the table's last column, worse than
  the centre form. Choosing "the vertex nearer the reach" is no
  better: with the reach at the centre it is a near-tie, noise at
  `f64` and the hull of both vertices at `Interval` (three sweep rows
  went "too wide to be useful" under it).
- **Keeping parameter zero at the seam and storing the anchor's
  anomaly `θ₀`**, evaluating `cos θ − cos θ₀` and `sin θ − sin θ₀` by
  product formulas. The evaluation is then fine, and the parameter is
  not: an edge's parameter near `θ₀ ≈ 0.45` has an ulp of about
  `u·0.45`, so the arc position resolves only to `u·a·0.45` whatever
  the evaluator does. Zero has to sit AT the anchor for the parameter
  itself to be fine there.

## What it takes

The anchor at a general point of the arc, parameter zero there: the
stored placement at that point, the principal directions and
semi-axes as now, and the anchor's eccentric anomaly as
`(cos θ₀, sin θ₀)`, so that the eccentric anomaly is `θ₀ + t`. A
principal frame and not a pair of conjugate semi-diameters: at
eccentricity `a/b` the conjugate pair is nearly parallel and its cross
product cancels at `u·a/b` relative.

Then every reader that pairs the PARAMETER with `u_ref`/`v_ref` or
the semi-axes itself reads a phased ellipse wrong, silently, with
nothing red at compile. The census (read-only, 2026-11-19, on
`onehome/carrier-frames` at `3d1ccd39d`), production code:

- geom: `Cover::arc` and its `Arc` piece (callers `cover_over`,
  `topo::separating`, `census::depth_gap`); `curves/boxes.rs`
  `extent_along`, `conic_frame_aabb`, the arc boxes (safe only if
  `Conic::along` absorbs the phase); `Curve3::reversed` (the phase
  negates); `seamed_near`, `quarter_turned_where`, `seamed_toward`,
  `section_frame`, `major_frame().phase`; `Curve3::data()` and
  `representability_margins` (emit `URef` as the seam).
- geom-brep: every harmonics builder in `implicit.rs` (the wall and
  sphere builder, `conic_cone_harmonics`, `conic_torus_harmonics`,
  `conic_residual_harmonics`, the torus residual's frame), which form
  coefficients in `t` from `frame().u_ref()/v_ref()` × `semi_axes()`
  and not from `semi_axis_vectors()`; `curve_pair/pose.rs` (an
  `Azimuth::of(w·v_ref, w·u_ref)` read as a parameter, `level_on`,
  `one_curve_gap`, `line_conic`, the window terms, `pose/window.rs`);
  `pcurve.rs` `ellipse_pcurve_on_plane`; `pcurve_cache.rs`
  `derive_class_zero` (the cone focal-section image, `t0` from
  `section_frame`), `run_iso_arc_checks`, `plane_image`,
  `carrier_harmonic`; `props/curved.rs` `require_cone_ellipse`;
  `curve_face/conic_oracle/exact.rs` `truth`;
  `pair_section/cylinders.rs` (`u_ref` read as the major direction).
- topo: `injective.rs` `conic` and its consumers; `stands.rs`
  `line_meetings`; `ring_path.rs` `ConicArc::of_frame` and its
  `n·u_ref`, `n·v_ref` coefficients; `chord_join.rs`
  `SectionConic::seamed_near` / `param`;
  `splitting/section_loops.rs` `whole_conic_principal`;
  `replace_face.rs` `transport_curve`, `homothety`;
  `offset_axial.rs` `mint_carrier`; `readback.rs` `edge_pose`;
  `boolean/conic_quadric/shallow_sweep.rs` `lifted`;
  `pcurves/decision.rs`.
- sweep: `blend/open/end_face.rs` `of` (an ellipse through
  `major_frame`).
- step-export `writer.rs` `edge_curve`: writes `ELLIPSE(placement,
  semi_u, semi_v)` and the edge's parameters as that entity's anomaly,
  so the parameters or the reference direction must turn by `θ₀`.

Phase-invariant, checked: `conic_axis_distance_floor` (a sup over the
turn), `props/loop_area.rs` (the areal rate), `certify_section_area`
(`Δt` only). Circles need no phase: `u_ref` is the radial direction at
parameter zero.

The list is from reading; it is not exhaustive for `pcurve_cache.rs`,
`curve_pair/pose.rs` and `implicit.rs` beyond the functions named.

## What this row closes on a re-seamed ellipse

`crates/geom-core/README.md` says of a conic held about a point of the
arc it serves that its parameter zero is there (PR 7258, approved; PR
7194's sentence said it of every conic). After the phase
(`Ellipse::seamed_toward`, `crates/geom/src/curves/conic.rs`) that holds
for a conic as minted and not for a re-seamed one: a re-seam turns the
seam under the anchor, so parameter zero is at the turned seam's
principal vertex and the anchor is a quarter or a half turn from it. The
parameter stays the eccentric anomaly from `u_ref`, and the anchor's own
anomaly is stored as its `(cos, sin)`.

What that costs, measured against exact rationals (ellipses of semi-axes
`a` and `0.37·a`, `a` from 1 to 1e6, points within 0.9 rad of the anchor):

| the anchor's anomaly from the seam | a point's worst miss |
|---|---|
| zero (as minted) | 0.55 to 0.75 `u` of the point's own size and its chord from the anchor |
| a quarter turn | 1.45 to 2.7 `u·a` |
| a half turn | 2.5 to 3.1 `u·a` |
| a general anomaly | 2.8 to 3.6 `u·a` |

So a re-seamed ellipse holds a point near its anchor to the semi-axis's
rounding, a centre-form ellipse's resolution, because the parameter
itself carries the quarter turn. The one production caller that re-seams
is `SectionConic::seamed_near` (`crates/topo/src/chord_join.rs`), which
`chord_arc` calls with the chord's middle; `Curve3::seamed_near` is the
door under it.

Closing it is this row's general form: parameter zero stays AT the
anchor through a re-seam, a point is summed by the chord from it, and
the sites that pair the parameter with the principal directions (about
60) read the principal angle from the conic. The stored anchor and its
anomaly's pair are what that form reads.
`conic::tests::a_re_seamed_ellipse_resolves_near_its_anchor_at_the_semi_axis`
pins today's `u·a` from both sides and goes red when it becomes
arc-scale; `a_re_seam_moves_parameter_zero_and_never_the_anchor` pins
where parameter zero and the anchor are.

## Re-banded P1 (2026-11-23, ONEROUND orchestrator)

`a-construction-whose-centre-runs-off-still-stores-a-centre` (P1, owed
by Ev's answer on PR 7258) is parked on this row: the cylinder's
tilted ellipse, one of its four constructions, has no principal vertex
near the arc it serves, so it can hold its arc only at a general
point. A blocker carries the band of what waits on it
(`work/README.md`, Priority). It follows `oneround/conic-readers`,
which puts the readers of a conic through the conic first; the census
above of readers that pair the parameter with `u_ref` / `v_ref` is
what remains for this row after it.

## Owed by this row: `u_ref` and `v_ref` leave `Conic` (2026-11-23)

From the three designers' agreed answer on what a reader of a conic's position asks the conic (`analysis/design-fork/conic-reader-interface`; with Ev as an `[ev]` PR once the kernel measurement is in). Reading the seam direction is pairing it with the parameter, since θ = 0 lies along it, and moving parameter zero to the anchor changes what `u_ref` means. The end state is that `Conic` keeps `axis`, and the principal frame is asked as `major_frame()` / `section_frame()`, whose answer states the parameter at its vertex; a read that wants an in-plane basis with no parameter gets a frame with no seam meaning.

**Remove or rename them; never re-document them under the same name**: the rename is the compile-time census of every pairing, and a re-documentation leaves every hand-written pairing compiling and wrong. If a readers unit removes them first (the pairing reads are the same sites those units convert), this section is discharged there.
