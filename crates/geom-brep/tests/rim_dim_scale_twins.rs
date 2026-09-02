//! Dimensional-metering pins for the two rim-level predicates:
//! `du_of_rims`' grouping (`props_rim_level_group` — the #89
//! in-band-landing retirement) and, since S58/#714, the iso-rectangle
//! predicate itself (`props_rim_level`).
//!
//! Both decide a difference of `RimLevel`s, and a `RimLevel` carries
//! its own dimension per surface kind, so both are exposed to exactly
//! the #89 mistake. #89 was found on a mm-scale import fixture and
//! nowhere else, which is why these pins are scale twins rather than
//! single rows.
//!
//! The defect (M7, found as the project's first in-band K landing —
//! `props_rim_level_group` margin `5.590169943747308e-7` inside
//! `Band { 1e-7, 1e-6 }` on the mm-scale `cone_trunc` import fixture,
//! **on the hosted sweep's `CAD_TOLERANCE_EPS=1e-7` leg**): the
//! grouping metered EVERY rim-level difference as
//! `(level difference) × arm`, but a cone/cylinder rim level is the
//! slant/axial arc length `v` itself — already meters — so `× arm`
//! manufactured an AREA-dimensioned comparand (two lengths
//! multiplied). On a mm body that shrank the true rim separation
//! (√5/2 mm ≈ 1.118e-3 m) by the arm (5e-4 m) straight into the
//! band; above 1 m scale it inflates margins instead. The ratified ε
//! semantics (D4: max deviation from specified geometry at a single
//! point) require every `classify` comparand to be a LENGTH.
//!
//! These pins build a native truncated-cone wall patch with
//! `cone_trunc`'s exact proportions (`makeCone(1, 0.5, 1)`: radii
//! 1 → 0.5, height 1, all × scale) at the recording scalar `Probe`
//! and assert, per D4's ε semantics:
//!
//! - the grouping margin IS the slant rim separation `√5/2 · scale`
//!   (a length, bare — no arm);
//! - at mm scale it lies OUT of the run's band, whatever the run's ε
//!   is — the landing is retired, not retuned;
//! - the margin scales LINEARLY with model scale (mm twin vs metre
//!   twin, ratio exactly 1000): the scale-quadratic area comparand
//!   would answer 1e6 here.
//!
//! The `props_rim_level` rows below do the same three things for the
//! predicate S58 generalised to all four kinds, on the two kinds whose
//! metering choice is NEW there — a cone (bare `Length` levels) and a
//! sphere (`Unit(sin v, cos v)` levered at the radius) — plus the row that
//! says what the band means: an interior rim inside the band is
//! ACCEPTED and one outside it is REFUSED, so ε buys a real length.
//!
//! **CI EXECUTES THIS SUITE.** It is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run under the
//! DEFAULT selection by `scripts/k_probe_sweep.sh`, whose tally is floored
//! by `--check-executed`, so every assertion below is a gate and a red here
//! fails the merge. By hand:
//! `cargo test -p geom-brep --features probe --test all -- rim_dim_scale_twins::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, curved_face};
use geom_core::Tol;
use geom_core::k_stats::{self, Probe};
use geom_core::{Band, Point3, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn p(x: f64, y: f64, z: f64) -> Point3<Probe> {
    Point3::new(Probe(x), Probe(y), Probe(z))
}

fn v3(x: f64, y: f64, z: f64) -> Vec3<Probe> {
    Vec3::new(Probe(x), Probe(y), Probe(z))
}

/// `cone_trunc`'s proportions at `scale` metres per source-unit:
/// apex at the origin, axis +z, `tan α = 0.5`; the wall runs between
/// the rims at axial `1·scale` (radius `0.5·scale`) and `2·scale`
/// (radius `1·scale`), azimuth `u ∈ [0, π/2]`. Slant levels are
/// `v = axial/cos α`, so the rim separation is `Δv = (√5/2)·scale`.
fn cone_patch(scale: f64) -> (Surface<Probe>, Vec<LoopEdge<Probe>>) {
    let half_angle = 0.5_f64.atan();
    let (sin_a, cos_a) = half_angle.sin_cos();
    let surface = Surface::Cone {
        apex: p(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: Probe(half_angle),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let (va, vb) = (scale / cos_a, 2.0 * scale / cos_a);
    let u1 = core::f64::consts::FRAC_PI_2;
    // Generator direction at azimuth u (unit: axis·cosα + radial·sinα).
    let gen_dir = |u: f64| v3(u.cos() * sin_a, u.sin() * sin_a, cos_a);
    let rim = |v: f64, forward: bool, tags: (u32, u32)| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: p(0.0, 0.0, v * cos_a),
                axis: v3(0.0, 0.0, 1.0),
                radius: Probe(v * sin_a),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            Probe(0.0),
            Probe(u1),
            forward,
            tags.0,
            tags.1,
        )
    };
    let meridian = |u: f64, forward: bool, tags: (u32, u32)| {
        LoopEdge::hand_built(
            Curve3::Line {
                origin: p(0.0, 0.0, 0.0),
                dir: gen_dir(u),
            },
            Probe(va),
            Probe(vb),
            forward,
            tags.0,
            tags.1,
        )
    };
    // Chart-CCW: rim va (+u), meridian u1 (+v), rim vb (−u),
    // meridian 0 (−v). The rim closure takes the SLANT level v
    // (axial height `v·cos α`, radius `v·sin α`).
    let edges = vec![
        rim(va, true, (0, 1)),
        meridian(u1, true, (1, 2)),
        rim(vb, false, (2, 3)),
        meridian(0.0, false, (3, 0)),
    ];
    (surface, edges)
}

/// Runs the closed form at `Probe` and returns the definite-nonzero
/// `props_rim_level_group` margins (absolute), plus the computed area.
fn rim_group_margins(scale: f64) -> (Vec<f64>, f64) {
    let (surface, edges) = cone_patch(scale);
    k_stats::start_recording();
    let got = curved_face(&surface, &edges, Probe(1.0), band());
    let samples = k_stats::take_samples();
    let contribution = got.expect("truncated-cone wall patch computes");
    let margins = samples
        .iter()
        .filter(|s| s.predicate == "props_rim_level_group")
        .map(|s| s.margin.abs())
        .filter(|m| *m != 0.0)
        .collect();
    (margins, contribution.area.0)
}

/// The mm twin: the grouping margin is the honest slant separation
/// `√5/2 mm` — LENGTH-dimensioned, and far OUT of the run's band at
/// every ε this CI runs. The area-dimensioned comparand landed inside
/// the band the M7 sweep had (#89 retirement, at that sweep's
/// ε = 1e-7).
#[test]
fn mm_scale_rim_group_margin_is_the_slant_separation() {
    let scale = 1e-3;
    let (margins, area) = rim_group_margins(scale);
    let expect = 5.0_f64.sqrt() / 2.0 * scale;
    assert_eq!(
        margins.len(),
        1,
        "one distinct-level comparison: {margins:?}"
    );
    let m = margins[0];
    assert!(
        ((m - expect) / expect).abs() < 1e-12,
        "margin {m:e} is not the slant rim separation {expect:e}"
    );
    // The retirement pin: the true separation sits DECISIVELY outside
    // the RUN's band — read from it, never from a literal. At the
    // sweep's ε = 1e-7 that band was { 1e-7, 1e-6 } and the
    // area-dimensioned margin (5.590169943747308e-7) landed inside it.
    let escalate = band().escalate();
    assert!(
        m > escalate,
        "margin {m:e} must clear this run's escalation threshold {escalate:e}"
    );
    // Fixture sanity: Area = sin α · Δu · (v_hi² − v_lo²)/2.
    let (sin_a, cos_a) = 0.5_f64.atan().sin_cos();
    let (va, vb) = (scale / cos_a, 2.0 * scale / cos_a);
    let expect_area = sin_a * core::f64::consts::FRAC_PI_2 * (vb * vb - va * va) / 2.0;
    assert!(((area - expect_area) / expect_area).abs() < 1e-12);
}

/// The scale-twin linearity pin (the ε-semantics contract): the
/// grouping margin is a point deviation, so it scales LINEARLY with
/// model scale. The pre-fix area comparand scaled QUADRATICALLY
/// (ratio 1e6 between these twins).
#[test]
fn rim_group_margin_scales_linearly_with_model_scale() {
    let (mm, _) = rim_group_margins(1e-3);
    let (m, _) = rim_group_margins(1.0);
    assert_eq!(mm.len(), 1);
    assert_eq!(m.len(), 1);
    let ratio = m[0] / mm[0];
    assert!(
        (ratio / 1e3 - 1.0).abs() < 1e-12,
        "margin must scale linearly with the model: ratio {ratio:e} (an \
         area-dimensioned comparand answers 1e6 here)"
    );
}

// ---------------------------------------------------------------------
// `props_rim_level` — the S58 iso-rectangle predicate (#714)
// ---------------------------------------------------------------------

/// `cone_patch`'s cone, with an L-shaped domain: the bottom rim runs
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

/// Runs the closed form at `Probe` and returns the definite-nonzero
/// `props_rim_level` margins (absolute), plus whether it refused.
fn rim_level_margins(surface: &Surface<Probe>, edges: &[LoopEdge<Probe>]) -> (Vec<f64>, bool) {
    k_stats::start_recording();
    let got = curved_face(surface, edges, Probe(1.0), band());
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
/// run's coincidence threshold** must MEASURE; one **decisively past
/// its escalation threshold** must REFUSE. Get the dimension wrong and
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
fn an_interior_rim_inside_the_band_measures_and_outside_it_refuses() {
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
