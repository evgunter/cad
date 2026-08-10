//! M8-14 (#222): **long-turn sweeps certify.** The executed frontier
//! this unit retires: `sweep_body` on helix arc paths ≥ ~0.5 turn
//! refused at `nurbs_span_meter` (`ParamSpan` escalation) because the
//! integral speed meter's single global chord collapsed under
//! `sweep_places`' near-antipodal frame roll. The meter's integral arm
//! now joins a per-span scan with the global chord (derivation on
//! `NurbsCurve3::speed_lower_bound`; corpus + flips in
//! `geom-curves/tests/m8_14_long_turn_meter.rs`), so a long-turn
//! corner path whose speed never drops meters positively and the body
//! assembles and certifies.
//!
//! The oracle is independent (Pappus-style): a constant CENTERED
//! SYMMETRIC profile swept in a normal-section path-following frame
//! encloses volume `A·L` in the continuum — the curvature moment
//! cancels by the profile's symmetry and roll about the tangent drops
//! out of the Jacobian (the twisted-cubic demo's argument, applied to
//! a path the kernel refused until this unit). `L` is the
//! interpolant's arc length by dense chordal summation; the analytic
//! helix length `Δθ·√(r² + c²)` brackets the interpolant's from
//! above, and both agree with the certified volume to the stated
//! discretization tolerance.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Mat3, Point3, Vec3};
use geom_curves::NurbsCurve3;
use sweep::sweep_body;

mod common;
use common::quad;

/// Profile half-width: small against the helix radius, so the swept
/// tube is embedded (no station's section reaches the axis).
const H: f64 = 0.08;
/// Helix radius (m) and pitch per revolution (m).
const R: f64 = 1.0;
const PITCH: f64 = 0.4;

/// A helix arc of `turns` revolutions, radius [`R`], pitch [`PITCH`]
/// per revolution, interpolated at 33 exact points per revolution
/// (min 33) — the same path family the montage-v2 probe measured
/// refusing at ≥ 0.5 turn.
fn helix_path(turns: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = f64::from(u32::try_from(k).unwrap()) / f64::from(u32::try_from(n).unwrap());
            let a = std::f64::consts::TAU * turns * s;
            Point3::new(R * a.cos(), R * a.sin(), PITCH * turns * s)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// The profile placement: plane through the path start, normal to the
/// start tangent (the skinned demo's frame recipe — `sweep_places`
/// carries this frame along the path by minimal rotation).
fn start_place(path: &NurbsCurve3<f64>) -> Affine3<f64> {
    let (lo, _) = path.domain();
    let d = path.deriv(lo);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    let v = n.cross(u);
    let p = path.eval(lo);
    Affine3::from_parts(Mat3::from_cols(u, v, n), Vec3::new(p.x, p.y, p.z))
}

/// The interpolant's arc length by dense chordal summation — the
/// oracle's `L`, independent of every kernel meter.
fn chordal_length(path: &NurbsCurve3<f64>, samples: usize) -> f64 {
    let (lo, hi) = path.domain();
    #[allow(clippy::cast_precision_loss)]
    let n = samples as f64;
    (0..samples)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let a = lo + (hi - lo) * (i as f64) / n;
            #[allow(clippy::cast_precision_loss)]
            let b = lo + (hi - lo) * ((i + 1) as f64) / n;
            (path.eval(b) - path.eval(a)).norm()
        })
        .sum()
}

/// Sweep a centered square along a helix of `turns` revolutions at
/// `stations` stations; certify; return (volume, pad, oracle A·L).
fn swept_helix(turns: f64, stations: usize) -> (f64, f64, f64) {
    let path = helix_path(turns);
    let place = start_place(&path);
    let profile = quad([(-H, -H), (H, -H), (H, H), (-H, H)]);
    let swept = sweep_body::<f64>(&profile, place, &path, stations, 3)
        .unwrap_or_else(|e| panic!("a {turns}-turn helical sweep must build now: {e:?}"));
    let m = topo::props::mass_properties(&swept.body).expect("mass properties certify");
    let area = (2.0 * H) * (2.0 * H);
    let len = chordal_length(&path, 20_000);
    (m.volume, m.volume_pad, area * len)
}

/// **The flip this unit exists for**: a FULL-TURN helical sweep — a
/// complete revolution of frame roll, straight through the
/// near-antipodal band that used to collapse the chord meter — builds
/// and certifies, and its volume agrees with the independent Pappus
/// oracle to the station-discretization bracket.
#[test]
fn a_full_turn_helical_sweep_builds_and_certifies() {
    let (volume, pad, oracle) = swept_helix(1.0, 33);
    let rel = ((volume - oracle) / oracle).abs();
    assert!(
        rel <= 1e-4,
        "full-turn helix: certified volume {volume} vs A·L oracle {oracle} \
         (rel {rel}) — outside the 33-station discretization bracket"
    );
    assert!(
        pad < 1e-9,
        "the certified enclosure must stay far tighter than the \
         discretization bracket (pad {pad})"
    );
    // The analytic continuum length brackets the interpolant's from
    // above (a chord never beats its arc), and tightly.
    let analytic = std::f64::consts::TAU * (R * R + (PITCH / std::f64::consts::TAU).powi(2)).sqrt();
    let interp = chordal_length(&helix_path(1.0), 20_000);
    assert!(
        interp <= analytic && (analytic - interp) / analytic < 1e-3,
        "interpolant length {interp} vs analytic helix length {analytic}"
    );
}

/// The half-turn signature itself — the exact threshold the issue
/// filed (`≥ ~0.5 revolutions refuse`) — now builds and certifies.
#[test]
fn a_half_turn_helical_sweep_builds_and_certifies() {
    let (volume, pad, oracle) = swept_helix(0.5, 17);
    let rel = ((volume - oracle) / oracle).abs();
    assert!(
        rel <= 1e-4,
        "half-turn helix: certified volume {volume} vs A·L oracle {oracle} (rel {rel})"
    );
    assert!(pad < 1e-9, "certified pad {pad}");
}

/// Multi-revolution: two full turns of frame roll (four antipodal
/// crossings). If this row ever regresses, the frontier follow-up
/// rule applies — pin the refusal and file it.
#[test]
fn a_two_turn_helical_sweep_builds_and_certifies() {
    let (volume, pad, oracle) = swept_helix(2.0, 65);
    let rel = ((volume - oracle) / oracle).abs();
    assert!(
        rel <= 1e-4,
        "two-turn helix: certified volume {volume} vs A·L oracle {oracle} (rel {rel})"
    );
    assert!(pad < 1e-9, "certified pad {pad}");
}
