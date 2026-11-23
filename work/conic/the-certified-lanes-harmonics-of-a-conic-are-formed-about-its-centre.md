---
id: the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre
kind: issue
title: The certified lane's harmonics and heights of a conic are formed about its centre
status: open
opened: 2026-11-23
priority: P1
cost: H
parent: one-stored-form-waits-on-the-readers-of-a-conics-centre
refs: [7510, 7258]
---


Cut from `one-stored-form-waits-on-the-readers-of-a-conics-centre` on 2026-11-23, when its census came in at 323 production reads against the row's 90 (PR 7510, the first part: directions, cover, extent and the gate). The functions below are pinned by that PR's gate (`clippy.toml` `disallowed-methods`, an `#[expect]` on each function, `scripts/gates/conic-centre-reads.counts`) as of `bd07a867e7`. **The sorting is the implementer's, from the census line of each read and not from reading each function through**, and a `°` marks a function that reads only `Circle::frame`. **What these readers should ask the conic is a design question in front of three designers** (`analysis/design-fork/conic-reader-interface`); two of the three already say this family is not one family, so the list is where to look, not the unit's shape.

## The functions

- geom-brep `implicit.rs`: `sample_rounding`, `quadric_harmonics`, `conic_cone_harmonics`, `conic_axis_distance_floor`, `conic_torus_harmonics`, `conic_torus_jet`, `conic_residual_harmonics`, `circle_sphere_harmonic`°
- geom-brep `curve_pair/pose.rs`: `line_conic`, `height`, `conic_conic`, `one_curve_gap`, `coplanar_circles`, `coplanar_ellipses`, `jet_at`, `level_on`; `pose/window.rs`: `height_jet`
- geom-brep `carrier_net.rs`: `harmonic`; `curve_face/conic_quadric.rs`: `apex_spread`; `curve_face/spline_face.rs`: `of`; `curve_face/conic_oracle/exact.rs`: `truth`
- geom `curves/on_carrier.rs`: `clearance` (and `distance_within`°)
- topo `ring_path.rs`: `plane`, `misses_in_plane`, `coplanar_apart`, `plane_roots`°, `path_parity`°, `path_resolution`°, `plane_torus`°
- topo `splitting/section.rs`: `plane_section`; `splitting/section_loops.rs`: `whole_conic_principal`, `line_clears_conic`, `conics_clear`; `shell.rs`: `cut_counting`; `census/depth_gap.rs`: `face_read` (the far axis as depth datum: the arc-cover row's probe B); `readback.rs`: `edge_pose`
- topo `boolean/join.rs`: `bool_connect`, `germ_section_loops`, `pk_section`; `boolean/vtxfac.rs`: `read_vertex_on_face`; `injective.rs`: `one_conic`, `one_stated_conic`, `planes_through`
- sweep `blend/open/end_face.rs`: `plan`, `depth`, `outside`, `apart`; `blend/surgery.rs`: `piece_distance`

## In none of the four families

- geom `curves.rs`: `same_content_by`°, `frame` (`Curve3::frame` itself, one of the five gated doors), `data`°, `representability_margins` (the stored-data walks): with whichever unit retires `Curve3::frame`.
- step-export `writer.rs`: `edge_curve` (the derived centre by the owner's decision; listed so the gate holds it, not owed).
- geom `cover.rs`: `arc_of`, PR 7510's own new function, which reads `Conic::frame` to fill the piece's `frame` field. The implementer reported it against itself; it is a fix-pass item of PR 7510, not this row's.

## Decided (Ev, PR 7523, 2026-11-23): one polynomial along a conic, in `geom`

The design question this row carried is answered; the three designers' final texts are in PR 7523's body, and the fork's rounds and measurements on `analysis/design-fork/conic-reader-interface`. **This row is the track's first unit: it builds the polynomial and moves the functions above onto it.**

- **The position.** A conic states where it is as `P(θ) = S + U·κ(θ) + V·σ(θ)`: `S` the point it stores, `U` and `V` its semi-axis vectors, `κ` and `σ` the placement's two functions of the parameter (`cos θ`, `sin θ` about a centre; `cos θ − cos θ₀`, `sin θ − sin θ₀` about a point of the arc, formed so nothing cancels at the anchor). It is opaque: no reader sees `S`, the phase or the placement. `point_at` is already this sum, so the evaluator's bits do not move.
- **The polynomial.** One type in `geom`, built by a reader from the position: a height above a plane (degree 1), a squared distance from a point or an axis (degree 2: sphere, wall, cone), a torus (degree 4). It replaces the private centre-shaped harmonic structs (`ConicHarmonics`, `ConicTorusHarmonics`, `CircleSphereHarmonic`, `half_angle::TrigPoly` and their kin).
- **`geom` states numbers, `geom-brep` decides.** The polynomial answers values and derivatives with a rounding bound read at the point, ranges over a window, the magnitude its rounding is metered against, and its half-angle coefficients about an anchor the ladder names, formed there and never a re-based copy. Every band, sign, meter, subdivision, ladder and refusal stays in `geom-brep`.
- **First-degree extremes and roots are posed per placement, inside the polynomial.** A centre-held conic keeps crest ± half-chord; an arc-held one is posed about its anchor (`h − 2P·sin²(φ/2) + Q·sin φ`). The opaque height that `oneround/conic-pair-readers` adds to `Conic` becomes this first degree.
- **Meters read the conic's speed over the window** (`Conic::speed_over`), not its whole-turn bound.

**Measured** (`analysis/oneround/conic-harmonic/` on `oneround/conic-harmonic-probe`, `m1.md` to `m4.md`; a scratch polynomial is on that branch): a root posed about the stored point misses by 2e-15 m where the centre form misses by 4e-9 m at a semi-axis of 1e6 m; the stored-point residual under the existing subdivision certifies 456 of 480 against 261; coefficients formed about the anchor from two jets are right to 4e-16 m in the median where a re-based constant is wrong by up to 64 m; forcing the half-angle form onto a centre-held conic loses at a touch on `Interval`.

**Owed with the unit:**
- **Verdicts read from centre-built conics move, and each is judged against an oracle** (decision 3 of PR 7523). In the fourth measurement 61 of 72,428 door calls changed verdict, 49 of them a refusal becoming an answer the suite's exact oracle accepts.
- **To trace before anything is called benign:** three of that measurement's five failing rows (a sliver escalation at `Interval`, two pinned query counts) and one move of 2.5e-2 in a test that still passes.
- **Not measured:** `Interval` for any arc-held reading; a wall, a cone or a torus in place of the sphere; whether any moved f64 bit is nearer the truth. The three final texts differ on who forms the quartic ladder's coefficients from the polynomial's jets and on the arc-held first-degree form at `Interval`: the unit measures both before choosing.
- **The level of a point against a conic** (`level_on`, `Level::jet_at`): `F = X²/A² + Y²/B² − 1` loses about u·a metres by its own form whatever its centre (the suspects lane, `analysis/oneround/conic-suspects/task1.md` on `oneround/conic-suspects-probe`). `oneround/conic-pair-readers` is asked to re-pose it; what that unit leaves is this one's.
- **Each function's line in the class sweep** (`analysis/oneround/conic-harmonic/sweep2-readers.md`, with the runs in `task1.md` and `task2.md`): 22 suspects, 18 covered by a named meter, 15 not applicable, 8 not traced. The readers that need a body to reach (`section_loops::line_clears_conic` and `conics_clear`, `replace_face::cone_v_range`, `solid_contain::passes`, the blend surgery's ring clearance) were not run: no body builds on a section large enough.

## Moved to CONIC (2026-11-23)

Moved from ONEROUND by `git mv` when that track was split on Ev's answer to PR 7523 (`work/conic/log.md`); the id and the body above are unchanged.

## Evidence from GERM (`germ/harmonic-belly`, 2026-11-23)

A centre-held conic's first-degree "crest ± half-chord" loses the belly's
digits when the conic is far larger than its belly. The belly here is the
extreme nearer the plane, `√(A² + B²) − |d₀|`, and both terms are of the
conic's size. For an ellipse of semi-axes 1e5 grazed near its vertex at
ε 1e-12, 596 of 2000 poses read definite on a belly the band does not
decide, and 52 read against the exact sign
(`curve_face::pose_rows::a_graze_of_a_far_centred_conic_is_read_wrong_nowhere`).
GERM's readers now take that extreme from `geom_core::Real::harmonic_belly`:
`(A² + B² − d₀²)/(√(A² + B²) + |d₀|)`, the numerator exact on the stored
terms. They do this through `curve_pair::pose::belly`, read by the plane arm,
`line_conic`, `conic_conic`, `transverse` and
`section_loops::line_clears_conic`. The polynomial's first degree should
keep that reading when it takes these readers over. The arc-held
sibling is `work/germ/an-arc-held-ellipses-plane-is-read-at-its-derived-centre.md`.
