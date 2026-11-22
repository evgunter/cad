//! Reviewer probes for PR 7329 (the departure margin). Not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, unused_imports, dead_code)]
use super::*;
use geom_core::test_support::fixture_lever;

fn iv0(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}
fn band() -> Band {
    Band::linear(geom_core::Tol::witness()).unwrap()
}
fn key<K: From<slotmap::KeyData>>(n: u64) -> K {
    slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<Interval> {
    Vec3::new(iv0(x), iv0(y), iv0(z))
}
fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(iv0(x), iv0(y), iv0(z))
}
fn arm(length: f64) -> Option<Lever<Interval>> {
    Some(fixture_lever(iv0(length)))
}
fn along(e: u64) -> Along {
    Along {
        edges: vec![key(e)],
        vertex: None,
        whole: true,
        params: None,
        on_edges: true,
    }
}
fn on(edges: &[u64], vertices: &[u64]) -> On {
    On {
        edges: edges.iter().map(|&e| key(e)).collect(),
        vertices: vertices.iter().map(|&v| key(v)).collect(),
        whole: true,
    }
}
fn presented(surface: &Surface<Interval>, x: (f64, f64), y: (f64, f64), zero: Zero) -> Presented<'_> {
    Presented {
        surface,
        map: Map::Chart,
        x,
        y,
        zero,
        corner: false,
        cache: Cache::default(),
    }
}

/// CLAIM 2. One edge, two readings: a cylinder of radius `r` (axis z) and
/// a plane through its ruling at (r, 0, .) that it leaves at the sine `s`,
/// both levered at the SAME one-metre reach. Check 4's class
/// (`geom_brep::classify_dihedral`, the folded arm) beside the engine's
/// carrier arm (`sign_on`, the reach alone), in both directions.
#[test]
fn probe_check4_and_the_engine_on_one_edge() {
    for (r, s) in [
        (1e-3, 1e-4),
        (1e-3, 5e-6),
        (1e-3, 5e-7),
        (1e-3, 5e-9),
        (1.0, 5e-6),
        (1.0, 5e-9),
        (1e3, 5e-8),
        (1e3, 5e-9),
    ] {
        let cyl = Surface::Cylinder {
            frame: geom::Frame::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0)),
            radius: iv0(r),
        };
        let c = (1.0 - s * s).sqrt();
        // The plane through the ruling, normal (-c, s, 0).
        let n = v3(-c, s, 0.0);
        // A frame for the plane surface: axis n, u_ref along z.
        let plane = Surface::Plane {
            frame: geom::Frame::new(p3(r, 0.0, 0.0), n, v3(0.0, 0.0, 1.0)),
        };
        let reach = fixture_lever(iv0(1.0));
        let at_edge = p3(r, 0.0, 0.5);
        let check4 = geom_brep::classify_dihedral(&cyl, &plane, at_edge, reach, band());
        let check4 = match check4 {
            Ok(geom_brep::Dihedral::Corner(_)) => "Corner (definite)".to_string(),
            Ok(geom_brep::Dihedral::Tangent(_)) => "Tangent (Zero: smooth class)".to_string(),
            Err(e) => format!("InBand {:?}", e).chars().take(90).collect(),
        };
        // The engine, the cylinder leaving the plane's carrier.
        let p = presented(&cyl, (0.0, 1e-3), (0.0, 1.0), Zero::Side(Side::XLo, along(1)));
        let psi = Psi::Carrier(Implicit::Plane {
            o: p3(r, 0.0, 0.0),
            n,
        });
        let engine_cyl = sign_on(&psi, None, &p, None, arm(1.0), band());
        let engine_on = sign_on(
            &Psi::Linear { n, q: p3(r, 0.0, 0.0) },
            Some(&on(&[1], &[])),
            &p,
            None,
            arm(1.0),
            band(),
        );
        println!(
            "[probe2] r={r:e} sine={s:e}: sine*r={:e} sine*reach={:e} | check 4: {check4} | engine (carrier arm): {engine_cyl:?} | engine (plane through the edge): {engine_on:?}",
            s * r,
            s
        );
    }
}

/// CLAIM 4. A cone's apex at several half-angles, plain, turned and far:
/// the corner reading with the apex side collapsed, against planes
/// through the ruling u = 0 turned by phi about it. Truth: over
/// u in [0, du] the face's direction across the ruling is e_theta(u), so
/// the sine is n.e_theta(u), sampled here; the reading must never give a
/// sign the samples contradict, and must answer nothing where the
/// samples change sign.
#[test]
fn probe_the_apex_reading_against_sampled_sines() {
    let frames: [(&str, [f64; 3], [f64; 3], [f64; 3]); 3] = [
        ("plain", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        (
            "turned",
            [0.3, -0.2, 0.1],
            [0.48, 0.6, 0.64],
            [0.8, -0.64, 0.0],
        ),
        ("far", [600.0, -800.0, 300.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
    ];
    let mut wrong = 0;
    let mut answered = 0;
    let mut asked = 0;
    for (pose, o, a, u) in frames {
        // Orthonormalize u against a.
        let an = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
        let a = [a[0] / an, a[1] / an, a[2] / an];
        let d = u[0] * a[0] + u[1] * a[1] + u[2] * a[2];
        let u = [u[0] - d * a[0], u[1] - d * a[1], u[2] - d * a[2]];
        let un = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt();
        let u = [u[0] / un, u[1] / un, u[2] / un];
        let w = [
            a[1] * u[2] - a[2] * u[1],
            a[2] * u[0] - a[0] * u[2],
            a[0] * u[1] - a[1] * u[0],
        ];
        for alpha in [1e-3, 0.05, 0.5, 1.0, 1.5, 1.5697] {
            let cone = Surface::Cone {
                frame: geom::Frame::new(
                    p3(o[0], o[1], o[2]),
                    v3(a[0], a[1], a[2]),
                    v3(u[0], u[1], u[2]),
                ),
                half_angle: iv0(alpha),
            };
            for du in [0.5, 0.05, 1e-6] {
                for phi in [0.0_f64, 0.3, 1.2, 1.5, 1.5707, 1.5708, 1.62, 2.0] {
                    // The ruling's direction and the cone's normal on it.
                    let (sa, ca) = alpha.sin_cos();
                    let ruling = [
                        sa * u[0] + ca * a[0],
                        sa * u[1] + ca * a[1],
                        sa * u[2] + ca * a[2],
                    ];
                    let normal = [
                        ca * u[0] - sa * a[0],
                        ca * u[1] - sa * a[1],
                        ca * u[2] - sa * a[2],
                    ];
                    let _ = ruling;
                    let (sp, cp) = phi.sin_cos();
                    // n = cos(phi) e_theta(0) + sin(phi) normal: a plane
                    // through the ruling.
                    let n = [
                        cp * w[0] + sp * normal[0],
                        cp * w[1] + sp * normal[1],
                        cp * w[2] + sp * normal[2],
                    ];
                    let zero = Zero::Corner((Side::XLo, along(1)), (Side::YLo, Along::at(key(7))));
                    let p = presented(&cone, (0.0, du), (0.0, 0.3), zero);
                    let psi = Psi::Linear {
                        n: v3(n[0], n[1], n[2]),
                        q: p3(o[0], o[1], o[2]),
                    };
                    let got = sign_on(&psi, Some(&on(&[1, 2], &[7])), &p, None, arm(1.0), band());
                    // Sampled truth: n . e_theta(t), t in [0, du].
                    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
                    for i in 0..=200 {
                        let t = du * f64::from(i) / 200.0;
                        let e = [
                            -t.sin() * u[0] + t.cos() * w[0],
                            -t.sin() * u[1] + t.cos() * w[1],
                            -t.sin() * u[2] + t.cos() * w[2],
                        ];
                        let s = n[0] * e[0] + n[1] * e[1] + n[2] * e[2];
                        lo = lo.min(s);
                        hi = hi.max(s);
                    }
                    asked += 1;
                    let bad = match got {
                        Some(Sign::Positive) => lo <= 0.0,
                        Some(Sign::Negative) => hi >= 0.0,
                        _ => false,
                    };
                    if got.is_some() {
                        answered += 1;
                    }
                    if bad {
                        wrong += 1;
                        println!("[probe4] WRONG {pose} alpha={alpha} du={du} phi={phi}: {got:?} sampled [{lo:e}, {hi:e}]");
                    }
                    if got.is_none() && (lo > 1e-6 || hi < -1e-6) {
                        println!("[probe4] unanswered though one-signed: {pose} alpha={alpha} du={du} phi={phi} sampled [{lo:e}, {hi:e}]");
                    }
                }
            }
        }
    }
    println!("[probe4] cone apex: asked {asked}, answered {answered}, wrong {wrong}");
    assert_eq!(wrong, 0);
}

/// CLAIM 4. A sphere's pole, several radii and centres: the meridian's
/// plane, the lune on each side, and a plane that is NOT the meridian's
/// (it holds the pole and the meridian's tangent there only) which the
/// `on` membership would have to refuse upstream: here stated as held,
/// to see what the reading alone answers.
#[test]
fn probe_the_pole_reading() {
    let pole = core::f64::consts::FRAC_PI_2;
    for (r, c) in [
        (1.0, [0.0, 0.0, 0.0]),
        (1e-3, [0.0, 0.0, 0.0]),
        (30.0, [600.0, -800.0, 300.0]),
        (1e-6, [600.0, -800.0, 300.0]),
    ] {
        let s = Surface::Sphere {
            frame: geom::Frame::new(p3(c[0], c[1], c[2]), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0)),
            radius: iv0(r),
        };
        for du in [1.0, 0.1, 1e-9] {
            for dv in [1.0, 0.1, 1e-9] {
                let zero = Zero::Corner((Side::XLo, along(1)), (Side::YHi, Along::at(key(7))));
                let p = presented(&s, (0.0, du), (pole - dv, pole), zero);
                let meridian = Psi::Linear {
                    n: v3(0.0, 1.0, 0.0),
                    q: p3(c[0], c[1], c[2]),
                };
                let got = sign_on(&meridian, Some(&on(&[1, 2], &[7])), &p, None, arm(1.0), band());
                println!("[probe4] sphere r={r:e} c={c:?} du={du:e} dv={dv:e}: {got:?} (true sine over the cell: [{:e}, 1])", du.cos());
            }
        }
    }
}

/// CLAIM 1. A cylinder of radius `r` on its ruling, leaving a plane
/// through the ruling squarely (sine 1 at the ruling), read over a cell
/// `du` wide: the face's whole excursion from the plane is at most 2r.
/// With r far inside the band the cell is certified at a metre's arm.
#[test]
fn probe_a_face_wholly_inside_the_band_of_its_separator() {
    for r in [1e-3, 1e-8, 1e-9, 1e-12] {
        for du in [1.5, 0.5, 1e-3] {
            let cyl = Surface::Cylinder {
                frame: geom::Frame::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0)),
                radius: iv0(r),
            };
            let p = presented(&cyl, (0.0, du), (0.0, 1.0), Zero::Side(Side::XLo, along(1)));
            let plane = Psi::Linear {
                n: v3(0.0, 1.0, 0.0),
                q: p3(r, 0.0, 0.0),
            };
            let got = sign_on(&plane, Some(&on(&[1], &[])), &p, None, arm(1.0), band());
            println!(
                "[probe1] r={r:e} du={du}: {got:?}; the face's farthest point from the plane in this cell is {:e} m (band {:e})",
                r * du.sin().max(if du > 1.5707 { 1.0 } else { 0.0 }),
                band().escalate()
            );
        }
    }
}
