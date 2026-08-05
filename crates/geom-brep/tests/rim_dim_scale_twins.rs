//! Dimensional-metering pins for `du_of_rims`' rim-level grouping
//! (the #89 in-band-landing retirement).
//!
//! The defect (M7, found as the project's first in-band K landing —
//! `props_rim_level_group` margin `5.590169943747308e-7` inside
//! `Band { 1e-7, 1e-6 }` on the mm-scale `cone_trunc` import fixture
//! at ε = 1e-7): the grouping metered EVERY rim-level difference as
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
//! - at mm scale it lies OUT of the ε = 1e-7 band
//!   `Band { zero 1e-7, escalate 1e-6 }` — the landing is retired,
//!   not retuned;
//! - the margin scales LINEARLY with model scale (mm twin vs metre
//!   twin, ratio exactly 1000): the scale-quadratic area comparand
//!   would answer 1e6 here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::props::{LoopEdge, curved_face};
use geom_core::k_stats::{self, Probe};
use geom_core::{Band, Point3, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

fn band() -> Band {
    Band::linear().unwrap()
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
    let rim = |v: f64, forward: bool, tags: (u32, u32)| LoopEdge {
        carrier: Curve3::Circle {
            center: p(0.0, 0.0, v * cos_a),
            axis: v3(0.0, 0.0, 1.0),
            radius: Probe(v * sin_a),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        t0: Probe(0.0),
        t1: Probe(u1),
        forward,
        start: tags.0,
        end: tags.1,
    };
    let meridian = |u: f64, forward: bool, tags: (u32, u32)| LoopEdge {
        carrier: Curve3::Line {
            origin: p(0.0, 0.0, 0.0),
            dir: gen_dir(u),
        },
        t0: Probe(va),
        t1: Probe(vb),
        forward,
        start: tags.0,
        end: tags.1,
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
/// `√5/2 mm` — LENGTH-dimensioned, and far OUT of the ε = 1e-7 band
/// the area-dimensioned comparand landed in (#89 retirement).
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
    // Band { zero: 1e-7, escalate: 1e-6 } — the ε = 1e-7 band the
    // area-dimensioned margin (5.590169943747308e-7) landed inside.
    assert!(
        m > 1e-6,
        "margin {m:e} must clear the ε = 1e-7 escalation threshold"
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
