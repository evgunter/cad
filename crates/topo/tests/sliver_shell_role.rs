//! **A near-tangent ∩'s sliver lump reads its role off its own size.**
//!
//! A prism's corner on a cube's face tilted `d` rad off one of the
//! corner's edges (`crates/sweep/examples/near_tangent_census_probe.rs`,
//! pose `nt e0 a<k> d<d>`): the ∩ keeps a wedge 2 m long and `2·d` m
//! thick at its far end as its own shell. At ε = 1e-12 the wedge is tens
//! of bands thick at `d ≥ 1e-10`, and its certified role is read off the
//! polyhedron of its vertex points. At `d = 1e-11` it is in band, and the
//! enclosure sits inside the band.
//!
//! Wider ε stands these rows down: the wedge is a band or less thick, and
//! the in-band twin is the pose measured at ε = 1e-12.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, BooleanDecision, BooleanDeclarations, BooleanError, ShellRole, classify_shells,
    intersect_with,
};

use crate::common;

type V3 = [f64; 3];

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scale(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}
fn unit(a: V3) -> V3 {
    scale(a, 1.0 / dot(a, a).sqrt())
}
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

/// The probe's prism `profile` (corner `v`, its edge `e` there) and its
/// cube at pose `nt e<·> a<a> d<d>`, in the probe's own arithmetic.
fn corner_pose(
    profile: &[(f64, f64)],
    v: V3,
    e: V3,
    a: u32,
    d: f64,
    tol: Tol,
) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let prism = common::prism::<f64>(profile, 1.0, tol).body;
    let prism = AtRestBody::validate(prism, tol).expect("the prism is at rest");
    let e = unit(e);
    let (p1, p2) = basis(e);
    let al = std::f64::consts::TAU * (f64::from(a) + 0.25) / 16.0;
    let base = add(scale(p1, al.cos()), scale(p2, al.sin()));
    let m = unit(unit(add(base, scale(e, d))));
    let (u, w) = basis(m);
    let cube = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
            let p = add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
            Point3::new(p[0], p[1], p[2])
        },
        tol,
    );
    let cube = AtRestBody::validate(cube, tol).expect("the cube is at rest");
    (prism, cube)
}

/// The probe's `notch307` prism ∩ its cube at pose `nt e0 a<a> d<d>`.
fn notch307_meet(a: u32, d: f64, tol: Tol) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
    let (prism, cube) = corner_pose(&profile, [2.0, 1.0, 1.0], [2.0, 1.0, 0.0], a, d, tol);
    intersect_with(&prism, &cube, &BooleanDeclarations::default(), tol)
}

/// Stands the row down where ε is wider than `widest`.
fn stood_down_above(widest: f64, tol: Tol) -> bool {
    if tol.eps() > widest {
        test_utils::vacuity::stood_down(
            "eps too wide for the pose",
            "the near-tangent wedge is a band or less thick at this ε, or past its pose's band",
        );
        return true;
    }
    false
}

/// **The witness**: at `d = 1e-9` and `1e-10` the wedge is certified a
/// lump of its own, and the ∩ builds.
#[test]
fn a_near_tangent_wedge_tens_of_bands_thick_reads_outer() {
    let tol = Tol::witness();
    if stood_down_above(1e-11, tol) {
        return;
    }
    for d in [1e-9, 1e-10] {
        let meet = notch307_meet(0, d, tol)
            .unwrap_or_else(|e| panic!("d = {d:e}: the ∩ builds, got {e:?}"));
        let body = &meet.body().expect("the ∩ is not empty").body;
        let shells = classify_shells(body, tol)
            .unwrap_or_else(|e| panic!("d = {d:e}: every shell classifies, got {e:?}"));
        let roles: Vec<_> = shells.iter().map(|s| s.role).collect();
        assert_eq!(
            roles,
            [ShellRole::Outer, ShellRole::Outer],
            "d = {d:e}: the bulk and the wedge are two lumps"
        );
        assert!(
            shells.iter().any(|s| s.volume.abs() < 1e-12),
            "d = {d:e}: one lump is the wedge: {shells:?}"
        );
    }
}

/// **The in-band twin**: at `d = 1e-11` (pose `a3`) the wedge is less
/// than a band thick, and the ∩ refuses its result as the operands'
/// ill-conditioning (D10, Booleans), on the wedge's certified enclosure
/// wholly inside the sliver band, not on one straddling zero.
#[test]
fn a_near_tangent_wedge_in_band_refuses_on_an_enclosure_inside_the_band() {
    let tol = Tol::witness();
    if stood_down_above(1e-12, tol) {
        return;
    }
    let refusal = notch307_meet(3, 1e-11, tol).map(|_| ());
    let Err(BooleanError::Escalated {
        decision: BooleanDecision::ShellRole { others: 0, .. },
        diag,
    }) = &refusal
    else {
        panic!("the ∩ refuses its in-band shell, got {refusal:?}");
    };
    assert_eq!(
        (diag.predicate, diag.terminal_sliver),
        (Some("positive_volume_exact"), true),
        "the certified reading is wholly inside the sliver band: {diag:?}"
    );
}

/// **A walk that reads zero does not hide a certified sliver**: at
/// `vee300 nt e0 a1 d6e-9`, ε = 1e-9, the ∩'s lump has its `V/A`
/// certified in [1.69973e-9, 1.69973e-9], wholly in band, while the f64
/// walk reads 8.7e-10 (prism first) and 3.3e-10 (cube first), in the
/// zero band. The door reads the certificate, in both orders.
#[test]
fn a_sliver_the_walk_reads_as_zero_refuses_in_band_in_both_orders() {
    let tol = Tol::witness();
    if (tol.eps() - 1e-9).abs() > 1e-21 {
        test_utils::vacuity::stood_down(
            "eps other than 1e-9",
            "the walk reads this pose's lump in the zero band at ε = 1e-9 only",
        );
        return;
    }
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)];
    let (prism, cube) = corner_pose(&profile, [2.0, 0.5, 1.0], [2.0, 3.5, 0.0], 1, 6e-9, tol);
    let decls = BooleanDeclarations::default();
    for (order, refusal) in [
        ("prism first", intersect_with(&prism, &cube, &decls, tol)),
        ("cube first", intersect_with(&cube, &prism, &decls, tol)),
    ] {
        let refusal = refusal.map(|_| ());
        let Err(BooleanError::Escalated {
            decision: BooleanDecision::ShellRole { others: 0, .. },
            diag,
        }) = &refusal
        else {
            panic!("{order}: the ∩ refuses its certified sliver in band, got {refusal:?}");
        };
        assert!(diag.terminal_sliver, "{order}: {diag:?}");
        assert!(
            diag.to_string().contains("enclosure [1.69972"),
            "{order}: the certified V/A is quoted: {diag}"
        );
    }
}
