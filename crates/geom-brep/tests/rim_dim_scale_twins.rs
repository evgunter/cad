//! Dimensional-metering pins for the iso-rectangle predicate
//! (`props_rim_level`, S58/#714), the shape door's
//! ([`geom_brep::require_iso_rectangle`]).
//!
//! It decides a difference of `RimLevel`s, and a `RimLevel` carries its
//! own dimension per surface kind, so it is exposed to exactly the #89
//! mistake: a grouping predicate once metered EVERY rim-level
//! difference as `(level difference) × arm`, but a cone/cylinder rim
//! level is the slant/axial arc length `v` itself — already meters — so
//! `× arm` manufactured an AREA-dimensioned comparand, found as the
//! project's first in-band K landing on the mm-scale `cone_trunc` import
//! fixture. The ratified ε semantics (D4: max deviation from specified
//! geometry at a single point) require every `classify` comparand to be
//! a LENGTH. #89 was found at mm scale and nowhere else, which is why
//! these pins are scale twins rather than single rows.
//!
//! The rows below pin its margin as a length on a cone (bare `Length`
//! levels) and a sphere (`Unit(sin v, cos v)` levered at the radius),
//! plus the row that says what the band means: an interior rim inside
//! the band is ACCEPTED and one outside it is REFUSED, so ε buys a real
//! length.
//!
//! **The nightly executes this suite; the PR gate only builds it.** It is
//! rostered in `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run
//! under the DEFAULT selection by `scripts/k_probe_sweep.sh` in the
//! nightly's dev-probe row, whose tally is floored by `--check-executed`,
//! so a red here reds the nightly. The PR gate compiles it (`clippy
//! --all-features`) and runs none of it, so a PR can merge green over a red
//! here. By hand:
//! `cargo test -p geom-brep --features probe --test all -- rim_dim_scale_twins::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3 as p, v3};
use crate::shared::tol::band;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, curved_face_loops};
use geom_brep::require_iso_rectangle;
use geom_core::Sign;
use geom_core::k_stats::{self, Probe, SampleOutcome};

// ---------------------------------------------------------------------
// `props_rim_level` — the S58 iso-rectangle predicate (#714)
// ---------------------------------------------------------------------

/// `cone_trunc`'s cone (apex at the origin, axis +z, `tan α = 0.5`,
/// `scale` metres per source unit), with an L-shaped domain: the bottom
/// rim runs
/// `u ∈ [0, u1]` at slant `va`, an INTERIOR rim at slant `vm` steps
/// out to `u2`, and the top rim at `vb` returns. The interior rim is
/// what `props_rim_level` refuses, and its margin is the quantity
/// these rows are about: the SLANT SEPARATION from the nearer extreme,
/// bare — a length already, never `× arm`.
fn cone_interior_rim(scale: f64, vm_frac: f64) -> (Surface<Probe>, Vec<LoopEdge<Probe>>) {
    let half_angle = 0.5_f64.atan();
    let (sin_a, cos_a) = half_angle.sin_cos();
    let surface = Surface::Cone {
        apex: p(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: Probe(half_angle),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let (va, vb) = (scale / cos_a, 2.0 * scale / cos_a);
    let vm = va + vm_frac * (vb - va);
    let (u1, u2) = (0.6_f64, 1.2_f64);
    let gen_dir = |u: f64| v3(u.cos() * sin_a, u.sin() * sin_a, cos_a);
    let rim = |v: f64, t0: f64, t1: f64, forward: bool, tags: (u32, u32)| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: p(0.0, 0.0, v * cos_a),
                axis: v3(0.0, 0.0, 1.0),
                radius: Probe(v * sin_a),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            Probe(t0),
            Probe(t1),
            forward,
            tags.0,
            tags.1,
        )
    };
    let meridian = |u: f64, t0: f64, t1: f64, forward: bool, tags: (u32, u32)| {
        LoopEdge::hand_built(
            Curve3::Line {
                origin: p(0.0, 0.0, 0.0),
                dir: gen_dir(u),
            },
            Probe(t0),
            Probe(t1),
            forward,
            tags.0,
            tags.1,
        )
    };
    let edges = vec![
        rim(va, 0.0, u1, true, (0, 1)),
        meridian(u1, va, vm, true, (1, 2)),
        rim(vm, u1, u2, true, (2, 3)),
        meridian(u2, vm, vb, true, (3, 4)),
        rim(vb, 0.0, u2, false, (4, 5)),
        meridian(0.0, va, vb, false, (5, 0)),
    ];
    (surface, edges)
}

/// The same L, on a sphere of radius `scale`: rims at latitudes
/// `va`/`vm`/`vb`, meridian sides as great circles through the poles.
/// Sphere rims carry the latitude direction pair
/// `RimLevel::Unit(sin v, cos v)`, so the predicate's margin is the
/// direction CHORD `√((Δ sin v)² + (Δ cos v)²) × R = 2·sin(Δv/2)·R` —
/// the point deviation between the two rim circles, everywhere on the
/// sphere.
fn sphere_interior_rim(scale: f64, vm_frac: f64) -> (Surface<Probe>, Vec<LoopEdge<Probe>>) {
    let surface = Surface::Sphere {
        center: p(0.0, 0.0, 0.0),
        radius: Probe(scale),
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let (va, vb) = (0.2_f64, 0.8_f64);
    let vm = va + vm_frac * (vb - va);
    let (u1, u2) = (0.6_f64, 1.2_f64);
    let rim = |v: f64, t0: f64, t1: f64, forward: bool, tags: (u32, u32)| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: p(0.0, 0.0, scale * v.sin()),
                axis: v3(0.0, 0.0, 1.0),
                radius: Probe(scale * v.cos()),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            Probe(t0),
            Probe(t1),
            forward,
            tags.0,
            tags.1,
        )
    };
    let meridian = |u: f64, t0: f64, t1: f64, forward: bool, tags: (u32, u32)| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: p(0.0, 0.0, 0.0),
                axis: v3(u.sin(), -u.cos(), 0.0),
                radius: Probe(scale),
                u_ref: v3(u.cos(), u.sin(), 0.0),
            },
            Probe(t0),
            Probe(t1),
            forward,
            tags.0,
            tags.1,
        )
    };
    let edges = vec![
        rim(va, 0.0, u1, true, (0, 1)),
        meridian(u1, va, vm, true, (1, 2)),
        rim(vm, u1, u2, true, (2, 3)),
        meridian(u2, vm, vb, true, (3, 4)),
        rim(vb, 0.0, u2, false, (4, 5)),
        meridian(0.0, va, vb, false, (5, 0)),
    ];
    (surface, edges)
}

/// Runs the shape door at `Probe` and returns the definite-nonzero
/// `props_rim_level` margins (absolute), plus whether it refused.
fn rim_level_margins(surface: &Surface<Probe>, edges: &[LoopEdge<Probe>]) -> (Vec<f64>, bool) {
    k_stats::start_recording();
    let got = require_iso_rectangle(surface, edges, band());
    let samples = k_stats::take_samples();
    let margins = samples
        .iter()
        .filter(|s| s.predicate == "props_rim_level")
        .map(|s| s.margin.abs())
        .filter(|m| *m != 0.0)
        .collect();
    (margins, got.is_err())
}

/// **Cone, mm scale.** The predicate's margin IS the slant separation
/// of the interior rim from the nearer extreme — a LENGTH, bare, with
/// no `× arm` anywhere. #89's mistake here would multiply it by the
/// rim radius (~5e-4 m at this scale) and land a 2.2e-4 m separation
/// at 1e-7 m — inside the band on the sweep leg that found #89, and
/// inside this run's band at any ε ≥ 1e-7.
#[test]
fn mm_scale_cone_rim_level_margin_is_the_slant_separation() {
    let scale = 1e-3;
    let (surface, edges) = cone_interior_rim(scale, 0.5);
    let (margins, refused) = rim_level_margins(&surface, &edges);
    assert!(refused, "the interior rim must be refused");
    let cos_a = 0.5_f64.atan().cos();
    let expect = 0.5 * scale / cos_a; // half the slant extent
    assert!(
        margins
            .iter()
            .any(|m| ((m - expect) / expect).abs() < 1e-12),
        "no margin is the slant separation {expect:e}: {margins:?}"
    );
    let escalate = band().escalate();
    assert!(
        margins.iter().all(|m| *m > escalate),
        "every decided margin must clear this run's escalation threshold \
         {escalate:e} at mm scale: {margins:?}"
    );
}

/// **Cone, scale twins.** A point deviation scales LINEARLY with the
/// model. An arm-multiplied comparand answers 1e6 here.
#[test]
fn cone_rim_level_margin_scales_linearly_with_model_scale() {
    let (mm, _) = rim_level_margins(
        &cone_interior_rim(1e-3, 0.5).0,
        &cone_interior_rim(1e-3, 0.5).1,
    );
    let (m, _) = rim_level_margins(
        &cone_interior_rim(1.0, 0.5).0,
        &cone_interior_rim(1.0, 0.5).1,
    );
    let ratio = m[0] / mm[0];
    assert!(
        (ratio / 1e3 - 1.0).abs() < 1e-12,
        "margin must scale linearly with the model: ratio {ratio:e}"
    );
}

/// **Sphere, mm scale and scale twins.** Sphere rims carry the full
/// latitude direction pair, so the metering is the chord
/// `√((Δ sin v)² + (Δ cos v)²) × R = 2·sin(Δv/2)·R`: the point
/// deviation between the two rim circles, a length, linear in scale —
/// the same expression the torus's pair has always had. An axial-only
/// `|Δ sin v| × R` here would shrink by `cos v̄` toward the poles and
/// merge genuinely distinct near-polar rims in the ACCEPTING
/// direction (retired audit note N7; the near-polar refusal row lives
/// in `cert1_sphere_polar.rs`, on CI's roster).
#[test]
fn mm_scale_sphere_rim_level_margin_is_the_direction_chord() {
    let scale = 1e-3;
    let (surface, edges) = sphere_interior_rim(scale, 0.5);
    let (margins, refused) = rim_level_margins(&surface, &edges);
    assert!(refused, "the interior rim must be refused");
    let (va, vb) = (0.2_f64, 0.8_f64);
    let vm = 0.5 * (va + vb);
    let chord =
        |v0: f64, v1: f64| ((v0.sin() - v1.sin()).powi(2) + (v0.cos() - v1.cos()).powi(2)).sqrt();
    let expect = chord(vm, va).min(chord(vm, vb)) * scale;
    assert!(
        margins
            .iter()
            .any(|m| ((m - expect) / expect).abs() < 1e-12),
        "no margin is the direction-chord rim separation {expect:e}: {margins:?}"
    );
    // Two honest populations and NOTHING in the ambiguity band: a rim
    // sitting at its own extreme records the lift's rounding-scale
    // second-component residual (the lift recomputes the extreme's
    // cosine from its sine; the rim reads its own off stored data) —
    // decided Zero, far inside the coincidence threshold — while the
    // interior rim's chord is decisively past escalation.
    let b = band();
    assert!(
        margins.iter().all(|m| *m < b.zero() || *m > b.escalate()),
        "every margin must be far inside the band or decisively past \
         escalation (zero {:e}, escalate {:e}) at mm scale: {margins:?}",
        b.zero(),
        b.escalate()
    );

    let (m, _) = rim_level_margins(
        &sphere_interior_rim(1.0, 0.5).0,
        &sphere_interior_rim(1.0, 0.5).1,
    );
    // The scale twin compares the CHORD margins — the largest of each
    // run's population; the rounding-scale residuals are noise-floor
    // values with no scale law of their own.
    let big = |ms: &[f64]| ms.iter().copied().fold(f64::MIN, f64::max);
    let ratio = big(&m) / big(&margins);
    assert!(
        (ratio / 1e3 - 1.0).abs() < 1e-12,
        "margin must scale linearly with the model: ratio {ratio:e}"
    );
}

/// **What ε buys, in metres.** The band is what separates an interior
/// rim that is a real notch from one that is a wobble, and because the
/// comparand is a length that separation is a DISTANCE, not a
/// coordinate. A mm-scale cone whose interior rim sits **inside the
/// run's coincidence threshold** is a rectangle; one **decisively past
/// its escalation threshold** is not. Get the dimension wrong and
/// both rows move together, which is exactly how #89 went unnoticed.
///
/// **The offsets come from the run's own band.** They were literals —
/// `1e-9 m` for the accepting row against a stated
/// `Band { zero 1e-7 }` — and that row was **already red at
/// ε = 1e-12** (executed; `1e-9` is a thousand ε there, so it refuses),
/// unobserved because this suite is off CI's roster. A literal band in
/// a file that decides against the ambient one is a latent wrong test,
/// not a comment defect. **S233.**
#[test]
fn an_interior_rim_inside_the_band_is_one_level_and_outside_it_refuses() {
    let scale = 1e-3;
    let cos_a = 0.5_f64.atan().cos();
    let slant_extent = scale / cos_a; // vb − va
    let band = band();
    for (offset, want_refusal) in [(0.5 * band.zero(), false), (10.0 * band.escalate(), true)] {
        let (surface, edges) = cone_interior_rim(scale, offset / slant_extent);
        let (_, refused) = rim_level_margins(&surface, &edges);
        assert_eq!(
            refused,
            want_refusal,
            "an interior rim {offset:e} m from the extreme: refused = {refused}, \
             wanted {want_refusal} (Band {{ zero {:e}, escalate {:e} }})",
            band.zero(),
            band.escalate()
        );
    }
}

// ---------------------------------------------------------------------
// The cone flux lane's decides
// ---------------------------------------------------------------------

/// `cone_trunc`'s cone at `scale` metres per source unit, and a loop
/// through the chart polygon `verts` — `(u, slant in source units)`,
/// rectilinear: a step in `u` is a rim, a step in slant a generator,
/// and a step along slant 0 is the apex, which carries no edge.
fn cone_trunc_loop(scale: f64, verts: &[(f64, f64)]) -> Vec<LoopEdge<Probe>> {
    let (sin_a, cos_a) = 0.5_f64.atan().sin_cos();
    let n = verts.len();
    let id = |i: usize| {
        let i = i % n;
        u32::try_from(if verts[i].1 == 0.0 { n } else { i }).expect("small")
    };
    let mut edges = Vec::new();
    for i in 0..n {
        let ((ua, va), (ub, vb)) = (verts[i], verts[(i + 1) % n]);
        if va == vb && va == 0.0 {
            continue;
        }
        let (va, vb) = (va * scale / cos_a, vb * scale / cos_a);
        edges.push(if va == vb {
            let (t0, t1, forward) = if ua < ub {
                (ua, ub, true)
            } else {
                (ub, ua, false)
            };
            LoopEdge::hand_built(
                Curve3::Circle {
                    center: p(0.0, 0.0, va * cos_a),
                    axis: v3(0.0, 0.0, 1.0),
                    radius: Probe(va * sin_a),
                    u_ref: v3(1.0, 0.0, 0.0),
                },
                Probe(t0),
                Probe(t1),
                forward,
                id(i),
                id(i + 1),
            )
        } else {
            let (t0, t1, forward) = if va < vb {
                (va, vb, true)
            } else {
                (vb, va, false)
            };
            LoopEdge::hand_built(
                Curve3::Line {
                    origin: p(0.0, 0.0, 0.0),
                    dir: v3(ua.cos() * sin_a, ua.sin() * sin_a, cos_a),
                },
                Probe(t0),
                Probe(t1),
                forward,
                id(i),
                id(i + 1),
            )
        });
    }
    edges
}

/// One whole rim of `cone_trunc`'s cone at height `h·scale`, a loop of
/// its own, turning anticlockwise about the axis when `forward`.
fn whole_rim(scale: f64, h: f64, forward: bool) -> Vec<LoopEdge<Probe>> {
    let (sin_a, cos_a) = 0.5_f64.atan().sin_cos();
    let v = h * scale / cos_a;
    vec![LoopEdge::hand_built(
        Curve3::Circle {
            center: p(0.0, 0.0, v * cos_a),
            axis: v3(0.0, 0.0, 1.0),
            radius: Probe(v * sin_a),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        Probe(0.0),
        Probe(core::f64::consts::TAU),
        forward,
        0,
        0,
    )]
}

/// **The cone's flux lane decides lengths, at mm scale as at metre
/// scale.** `cone_trunc`'s cone carries the lane's decides on
/// five faces between them: `cone_interior_rim`'s L (the nappe, the side,
/// each end off the apex, each rim's sense, the winding, the loop's
/// vertices apart), a sector through the apex (its sweep), a seamed
/// wall with a split rim (the seam's two copies of a vertex a turn
/// apart), a wall with a hole (the outer loop's own side) and a band
/// between two whole rims (the winding's sense, which only a loop
/// that winds the axis decides). Every
/// one of those it decides definitely is a point deviation, so it
/// scales LINEARLY between the twins (an arm-multiplied comparand
/// answers 1e6) and clears the run's escalation threshold at mm scale.
#[test]
fn the_cone_flux_lane_decides_lengths_at_both_scales() {
    const LENGTHS: &[&str] = &[
        "props_cone_nappe",
        "props_cone_area_side",
        "props_cone_loop_off_apex",
        "props_cone_arc_sense",
        "props_chart_loop_turns",
        "props_chart_loop_sense",
        "props_cone_loop_vertex",
        "props_cone_apex_loop_net",
        "props_cone_apex_loop_sweep",
        "props_cone_loop_vertex_sheet",
        "props_cone_outer_side",
    ];
    let surface = |scale: f64| cone_interior_rim(scale, 0.5).0;
    let faces = |scale: f64| -> Vec<Vec<Vec<LoopEdge<Probe>>>> {
        let tau = core::f64::consts::TAU;
        vec![
            vec![cone_interior_rim(scale, 0.5).1],
            vec![cone_trunc_loop(
                scale,
                &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
            )],
            vec![cone_trunc_loop(
                scale,
                &[(0.0, 1.0), (2.0, 1.0), (tau, 1.0), (tau, 2.0), (0.0, 2.0)],
            )],
            vec![
                cone_trunc_loop(scale, &[(0.0, 1.0), (2.0, 1.0), (2.0, 2.0), (0.0, 2.0)]),
                cone_trunc_loop(scale, &[(0.5, 1.2), (0.5, 1.8), (1.5, 1.8), (1.5, 1.2)]),
            ],
            vec![whole_rim(scale, 1.0, true), whole_rim(scale, 2.0, false)],
        ]
    };
    let run = |scale: f64| -> Vec<(&'static str, f64)> {
        let s = surface(scale);
        let mut out = Vec::new();
        for face in faces(scale) {
            let loops: Vec<&[LoopEdge<Probe>]> = face.iter().map(Vec::as_slice).collect();
            k_stats::start_recording();
            let got = curved_face_loops(&s, &loops, true, band());
            let samples = k_stats::take_samples();
            got.unwrap_or_else(|e| panic!("a face at {scale:e} measures: {e:?}"));
            out.extend(
                samples
                    .iter()
                    .filter(|s| LENGTHS.contains(&s.predicate))
                    .filter(|s| {
                        matches!(
                            s.outcome,
                            SampleOutcome::Definite(Sign::Positive | Sign::Negative)
                        )
                    })
                    .map(|s| (s.predicate, s.margin.abs())),
            );
        }
        out
    };
    let (mm, m) = (run(1e-3), run(1.0));
    for name in LENGTHS {
        assert!(
            mm.iter().any(|(p, _)| p == name),
            "{name} decides nothing definite on these faces: {mm:?}"
        );
    }
    assert_eq!(
        mm.iter().map(|(p, _)| *p).collect::<Vec<_>>(),
        m.iter().map(|(p, _)| *p).collect::<Vec<_>>(),
        "the twins decide the same sequence"
    );
    let escalate = band().escalate();
    for ((name, a), (_, b)) in mm.iter().zip(&m) {
        let ratio = b / a;
        assert!(
            (ratio / 1e3 - 1.0).abs() < 1e-9,
            "{name}: margin must scale linearly with the model, ratio {ratio:e}"
        );
        assert!(
            *a > escalate,
            "{name}: margin {a:e} must clear this run's escalation threshold {escalate:e} at mm scale"
        );
    }
}

// ---------------------------------------------------------------------
// `props_torus_axis` — a carrier axis's length, levered
// ---------------------------------------------------------------------

/// The `props_torus_axis` margins of a torus about `z` with major radius
/// `3·scale` and minor radius `scale`, read through one meridian edge
/// centred on its major circle.
fn torus_axis_margins(scale: f64) -> Vec<f64> {
    let s = |v: f64| Probe(v * scale);
    let torus = Surface::Torus {
        center: p(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: s(3.0),
        minor_radius: s(1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let meridian = Curve3::Circle {
        center: p(3.0 * scale, 0.0, 0.0),
        axis: v3(0.0, 1.0, 0.0),
        radius: s(1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let edges = vec![LoopEdge::hand_built(
        meridian,
        Probe(0.0),
        Probe(core::f64::consts::FRAC_PI_2),
        true,
        0,
        1,
    )];
    k_stats::start_recording();
    geom_brep::require_iso_rectangle(&torus, &edges, band()).expect("one meridian edge");
    k_stats::take_samples()
        .iter()
        .filter(|s| s.predicate == "props_torus_axis")
        .map(|s| s.margin)
        .collect()
}

/// **A torus axis's length is decided at the model's scale.** The axis
/// is unit at rest, a pure number; levered by the meridian's reach from
/// the centre, `3 + 1`, its margin is `4·scale`, and so `1e3` apart
/// between the twins. The bare norm reads `1` at both.
#[test]
fn torus_axis_margin_is_the_meridian_reach_and_scales_linearly() {
    for scale in [1e-3, 1.0] {
        let margins = torus_axis_margins(scale);
        assert_eq!(
            margins.len(),
            1,
            "one axis decision at {scale:e}: {margins:?}"
        );
        let want = 4.0 * scale;
        assert!(
            ((margins[0] - want) / want).abs() < 1e-12,
            "at {scale:e} the axis margin {:e} is not the meridian reach {want:e}",
            margins[0]
        );
    }
}
