//! Review probes for PR 6020 (a chord between two copies of one site is
//! a whole turn). Each pose splits a full-revolve tube or counterbore by
//! a plane through a bore rim point and, where the split builds, checks
//! tiers 1, 3 and 3′ and both halves' volumes against an independent
//! polar quadrature of the material column above and below the plane.
//! A pose may refuse (typed); it may not build a wrong body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use topo::Body;
use topo::splitting::{SplitPart, SplitPlane, split};

fn tol() -> Tol {
    Tol::witness()
}

fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let lp = bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect());
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    sweep::revolve(&profile, axis, sweep::Revolution::Full, tol())
        .unwrap()
        .body
}

const TUBE: [(f64, f64); 4] = [(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 1.0)];
const COUNTERBORE: [(f64, f64); 6] = [
    (0.3, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (0.6, 1.0),
    (0.6, 0.6),
    (0.3, 0.6),
];

/// The material column at radius `r`: `[0, top]`, or none.
fn column(cb: bool, r: f64) -> Option<f64> {
    if cb {
        if r < 0.3 {
            None
        } else if r < 0.6 {
            Some(0.6)
        } else {
            Some(1.0)
        }
    } else if r < 0.5 {
        None
    } else {
        Some(1.0)
    }
}

/// `[below, above]` volumes of the solid on either side of the plane
/// through `p` with normal `n` (`n.y ≠ 0`), by a midpoint polar
/// quadrature over `r ∈ [0.3, 1]`, `θ ∈ [0, 2π)`.
fn quadrature(cb: bool, p: Point3<f64>, n: Vec3<f64>) -> [f64; 2] {
    let (nr, nt) = (1400usize, 2800usize);
    let (r0, r1) = (0.3, 1.0);
    let (dr, dt) = ((r1 - r0) / nr as f64, 2.0 * PI / nt as f64);
    let (mut below, mut above) = (0.0, 0.0);
    for i in 0..nr {
        let r = r0 + (i as f64 + 0.5) * dr;
        let Some(top) = column(cb, r) else { continue };
        for j in 0..nt {
            let th = (j as f64 + 0.5) * dt;
            let (x, z) = (r * th.cos(), r * th.sin());
            // n·((x, y, z) − p) = 0 → y.
            let yp = p.y - (n.x * (x - p.x) + n.z * (z - p.z)) / n.y;
            let cut = yp.clamp(0.0, top);
            let (lo, hi) = (cut, top - cut);
            let da = r * dr * dt;
            // Above the plane in y is the normal's side when n.y > 0.
            if n.y > 0.0 {
                above += hi * da;
                below += lo * da;
            } else {
                above += lo * da;
                below += hi * da;
            }
        }
    }
    [below, above]
}

fn volume(h: &Body<f64>) -> (f64, f64) {
    let m = topo::mass_properties(
        &topo::test_support::finished("the measured body", Body::clone(h), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .unwrap();
    (m.enclosure.volume, m.enclosure.volume_pad)
}

/// The outcome of one pose: `Err` a typed refusal; `Ok` the per-side
/// tier-3′ verdicts and volume errors against the quadrature.
fn pose(label: &str, cb: bool, p: Point3<f64>, n: Vec3<f64>) -> String {
    let body = revolved(if cb { &COUNTERBORE } else { &TUBE });
    let plane: SplitPlane<f64> = topo::test_support::split_plane(p, n, tol());
    let want = quadrature(cb, p, n);
    let res = match split(
        &sweep::test_support::finished("the operand", body, tol()),
        &plane,
        tol(),
    ) {
        Ok(r) => r,
        Err(e) => return format!("{label}: REFUSED {e:?}"),
    };
    let mut out = format!("{label}: built;");
    for ((side, part), w) in [("below", res.below), ("above", res.above)]
        .into_iter()
        .zip(want)
    {
        let SplitPart::Body(half) = part else {
            out += &format!(" {side} empty (want {w:.6});");
            assert!(w < 1e-3, "{label} {side}: empty but quadrature {w}");
            continue;
        };
        let h = half.into_body();
        assert_eq!(topo::validate(&h), Ok(()), "{label} {side}: tier 1");
        assert_eq!(
            topo::validate_geometric(&h, tol()),
            Ok(()),
            "{label} {side}: tier 3"
        );
        let t3p = topo::validate_pseudomanifold(&h, &topo::ContactRecords::default(), tol());
        let (v, pad) = volume(&h);
        let shells = h.solids().count();
        out += &format!(
            " {side} v={v:.9}±{pad:.1e} want≈{w:.6} err={:.1e} solids={shells} t3'={};",
            v - w,
            if t3p.is_ok() {
                "ok".into()
            } else {
                format!("{t3p:?}")
            }
        );
        // A wrong arc (a whole turn where a sliver was meant, or the
        // reverse) moves a volume by O(0.1); the quadrature is good to
        // ~1e-5 here.
        assert!(
            (v - w).abs() < 2e-4,
            "{label} {side}: volume {v}, quadrature {w}"
        );
    }
    out
}

/// Every probed pose, printed; a built pose must be a right body.
#[test]
fn review_6020_seam_poses() {
    let y = Vec3::new(0.0, 1.0, 0.0);
    let mut log = Vec::new();
    for t in [0.05f64, 0.2, 0.5] {
        for az in [0.0f64, 1e-10, -1e-10, 1e-3, 0.3] {
            let d = Vec3::new(az.cos(), 0.0, az.sin());
            let side = Vec3::new(-az.sin(), 0.0, az.cos());
            for s in [1.0, -1.0] {
                // Tube top bore rim, plane leaning away from the axis (the
                // PR's pose) and toward it (the bore wall touched from
                // outside its face: the section meets the face at P alone
                // with the face on one side only).
                let p = Point3::new(0.5 * d.x, 1.0, 0.5 * d.z);
                for (what, lean) in [("in", -1.0), ("out", 1.0)] {
                    let n = (y * t.cos() + d * (lean * t.sin())) * s;
                    log.push(pose(&format!("tube {what} t{t} a{az} s{s}"), false, p, n));
                }
                // Tube: the plane through P crossing the rim
                // transversally (leaning along the rim's tangent), so
                // the bore's section passes through P and crosses the
                // rim again at the antipode: a chord from one copy of P
                // to another would be wrong here.
                if t <= 0.5 {
                    let n = (y * t.cos() - side * t.sin()) * s;
                    log.push(pose(&format!("tube across t{t} a{az} s{s}"), false, p, n));
                    // And mixed: through P leaning partly along the tangent.
                    let n = (y * t.cos() - (side * 0.6 - d * 0.8) * t.sin()) * s;
                    log.push(pose(&format!("tube mixed t{t} a{az} s{s}"), false, p, n));
                }
                // Counterbore floor rim at the bore, leaning both ways.
                if t <= 0.2 {
                    let p = Point3::new(0.3 * d.x, 0.6, 0.3 * d.z);
                    for (what, lean) in [("in", -1.0), ("out", 1.0)] {
                        let n = (y * t.cos() + d * (lean * t.sin())) * s;
                        log.push(pose(&format!("cbore {what} t{t} a{az} s{s}"), true, p, n));
                    }
                }
            }
        }
    }
    for l in &log {
        println!("{l}");
    }
}

/// A cube of side 10 standing on the plane through `p` with unit normal
/// `n`, on `n`'s side, its base centred on `p`.
fn cube_on(p: Point3<f64>, n: Vec3<f64>) -> Body<f64> {
    let n = n * (1.0 / n.norm());
    let (u, _) = n.orthonormal_basis();
    // Right-handed (u × w = n), so the cube is not inside out.
    let w = n.cross(u);
    topo::test_support::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (10.0 * (x - 0.5), 10.0 * (y - 0.5), 10.0 * z);
            p + u * a + w * b + n * c
        },
        tol(),
    )
}

/// The Boolean lane at the same touch: the tube against a cube whose
/// face lies in the plane, through ∖, ∩ and ∪ in both orders where
/// they commute. A built result must hold the quadrature's volume.
#[test]
fn review_6020_boolean_seam_poses() {
    let y = Vec3::new(0.0, 1.0, 0.0);
    let mut log = Vec::new();
    for t in [0.2f64] {
        for az in [0.0f64, 0.3] {
            let d = Vec3::new(az.cos(), 0.0, az.sin());
            let side = Vec3::new(-az.sin(), 0.0, az.cos());
            for s in [1.0, -1.0] {
                let p = Point3::new(0.5 * d.x, 1.0, 0.5 * d.z);
                for (what, n) in [
                    ("in", y * t.cos() - d * t.sin()),
                    ("out", y * t.cos() + d * t.sin()),
                    ("across", y * t.cos() - side * t.sin()),
                ] {
                    let n = n * s;
                    let [below, above] = quadrature(false, p, n);
                    let tube = topo::AtRestBody::validate(revolved(&TUBE), tol()).unwrap();
                    let cube = topo::AtRestBody::validate(cube_on(p, n), tol()).unwrap();
                    let cases: [(&str, Result<_, _>, f64); 4] = [
                        ("tube − cube", topo::subtract(&tube, &cube, tol()), below),
                        ("tube ∩ cube", topo::intersect(&tube, &cube, tol()), above),
                        ("cube ∩ tube", topo::intersect(&cube, &tube, tol()), above),
                        (
                            "tube ∪ cube",
                            topo::union(&tube, &cube, tol()),
                            1000.0 + below,
                        ),
                    ];
                    for (op, r, want) in cases {
                        let label = format!("bool {what} t{t} a{az} s{s} {op}");
                        let line = match r {
                            Err(e) => format!("{label}: REFUSED {e:?}"),
                            Ok(r) => match r.body() {
                                None => format!("{label}: empty (want {want:.6})"),
                                Some(bb) => {
                                    let v = topo::test_support::pinned_volume(
                                        "the measured body",
                                        &bb.body,
                                        tol(),
                                    )
                                    .volume;
                                    let t3p = topo::validate_pseudomanifold(
                                        &bb.body,
                                        &bb.contacts,
                                        tol(),
                                    );
                                    assert!(
                                        (v - want).abs() < 2e-4,
                                        "{label}: volume {v}, quadrature {want}"
                                    );
                                    format!(
                                        "{label}: built v={v:.9} err={:.1e} solids={} t3'={}",
                                        v - want,
                                        bb.body.solids().count(),
                                        if t3p.is_ok() {
                                            "ok".into()
                                        } else {
                                            format!("{t3p:?}")
                                        }
                                    )
                                }
                            },
                        };
                        log.push(line);
                    }
                }
            }
        }
    }
    for l in &log {
        println!("{l}");
    }
}
