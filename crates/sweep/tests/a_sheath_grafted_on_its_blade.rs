//! **A sheath grafted on its blade at their shared section**: the lily
//! tour's wall 8 in miniature. Two lens lofts stand on one base section,
//! in one plane, running away from it in opposite directions, so each
//! one's base rim arcs ARE the other's, and each arc rises into a NURBS
//! wall of its own body.
//!
//! The boolean reads each rim arc against the other body's spline walls
//! by their boundary first (C5's curve × face clause): along the wall it
//! shares, a run of one circle the curve-pair reading decides; against
//! the lens's other wall, the other body's own rim edge, which meets that
//! wall at the two tips only. Neither asks the surface anything, so the
//! union gets past the contact, in either operand order. A NURBS tip
//! seam rising from the shared plane is read against the other body's
//! base cap there by the curve × face table's ring-composite arm, which
//! finds no crossing inside it: its start on the cap is its end vertex's
//! meeting. Each NURBS seam of one body is read against the other's
//! NURBS walls by the table's spline-face arm, its end at a tip accounted
//! for as the end vertex's meeting. At the result, the shared rim, a
//! seam between the two bodies' NURBS walls, reads transverse: each
//! wall's normal off the rim's image in its chart, levered over each
//! chart's certified curvature. The rim is the `v = 0` row of both walls,
//! a curve known in advance, so it is described by that shared row (D2's
//! structural selector), certified per wall with no uniqueness tube. Its
//! certificate then bounds the walls' crossing over the whole rim, and on
//! one of the lens's two rims the walls are tangent at a point: their
//! crossing reading changes sign across it. The union refuses there. This
//! row is that site's tripwire: it reds when the rim certifies.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;
use topo::{BooleanError, EdgeDescribeFailure, EulerOpError};

/// The lens on chord `width`, rising `ridge` above it and dropping
/// `keel` below, run counterclockwise from the `+s` margin.
fn lens([width, ridge, keel]: [f64; 3]) -> Vec<ProfileLoop<f64>> {
    let half = 0.5 * width;
    vec![bulge_loop(vec![
        (Point2::new(half, 0.0), -ridge / half),
        (Point2::new(-half, 0.0), -keel / half),
    ])]
}

/// A lens blade from the origin along `d`, its spine an arc of length
/// `len` turning `curl` rad toward `v`, five stations, the section
/// `section(s)` at the fraction `s` of the way.
fn blade(
    d: Vec3<f64>,
    v: Vec3<f64>,
    (len, curl): (f64, f64),
    section: impl Fn(f64) -> [f64; 3],
    tol: Tol,
) -> topo::AtRestBody<f64> {
    const STATIONS: usize = 5;
    let band = Band::linear(tol).expect("the run's tolerance forms a band");
    let u = v.cross(d);
    let r = len / curl;
    let mut sections = Vec::with_capacity(STATIONS);
    let mut places = Vec::with_capacity(STATIONS);
    for k in 0..STATIONS {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((STATIONS - 1) as f64);
        let (sa, ca) = (curl * s).sin_cos();
        let p = Point3::origin() + d * (r * sa) + v * (r * (1.0 - ca));
        let vk = v * ca - d * sa;
        sections.push(lens(section(s)));
        places.push(SketchPlane::from_frame(
            RigidMotion::gram_schmidt(p, u, vk, "sheath_blade", band)
                .expect("the station's two directions span a plane"),
        ));
    }
    let lofted = loft_body::<f64>(&sections, &places, 3, tol)
        .expect("the lens blade lofts")
        .body;
    topo::test_support::finished("a lens blade", lofted, tol)
}

/// **Both orders stop where the walls touch on the rim.** Each rim is
/// each wall's `v = 0` row, recovered as the walls' shared row. On one of
/// them the walls' crossing reading changes sign mid-rim (measured
/// 2026-10-29 near `t = 0.6477` of `[0, 0.924]`: the reading is
/// definitely negative just before, definitely positive just after), so
/// the walls are tangent there and the rim's certificate escalates its
/// crossing (`Escalated { Transversality }`, `shared_row_crossing`) at ε
/// 1e-9, 1e-6 and 1e-12. A certificate that read the crossing at its
/// samples alone stepped over that point and passed the rim; before the
/// shared row, the rim's witness refused at limb 3 (`TubeNotOneArc`).
/// Before the exact images, limb 2 refused on the fitted image's 9.0e-6 m
/// miss; before the seam's spline entry, the seam's dihedral arm
/// (`Escalated { LeverArm(Seam), "dihedral_arm" }`); before the spline
/// span × spline face arm, `CurvedBooleanUnsupported { operand: B, kind:
/// Nurbs }`; before the boundary pass, the contact on a wall of B; before
/// the ring-composite arm, `EdgeCarrierUnsupported` on a tip seam of A
/// against the cap of B.
#[test]
fn a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim() {
    let tol = Tol::witness();
    let base = [0.170, 0.028, 0.020];
    let (d, v) = (Vec3::unit_x(), Vec3::unit_z());
    let leaf = blade(
        d,
        v,
        (1.2, -0.3),
        |s| [0, 1, 2].map(|i| base[i] + ([0.300, 0.034, 0.016][i] - base[i]) * s),
        tol,
    );
    let sheath = blade(-d, v, (0.34, 0.85), |_| base, tol);
    for (what, a, b) in [
        ("blade ∪ sheath", &leaf, &sheath),
        ("sheath ∪ blade", &sheath, &leaf),
    ] {
        let err = topo::union(a, b, tol).expect_err(what);
        assert!(
            matches!(
                &err,
                BooleanError::EdgeUndescribed {
                    failure: EdgeDescribeFailure::Intersection,
                    cause: Some(cause),
                    ..
                } if matches!(
                    **cause,
                    EulerOpError::Certification {
                        error: geom_brep::CertifyError::Escalated {
                            check: geom_brep::CertCheck::Transversality,
                            ..
                        },
                    }
                )
            ),
            "{what}: at the rim, where its walls touch: {err:?}"
        );
        // The cap read on the way: every spline edge of A, against every
        // plane face of B, through the arm the planar lane reads, crosses
        // nothing inside itself.
        let lane = geom_brep::NurbsLane::<f64>::certified();
        let band = Band::linear(tol).expect("the run's tolerance forms a band");
        let mut read = 0;
        for (_, edge) in a.edges() {
            let Some(curve) = a
                .get_curve_geom(edge.curve)
                .and_then(topo::CurveGeom::certified)
            else {
                continue;
            };
            let geom::Curve3::Nurbs(net) = curve.carrier() else {
                continue;
            };
            for (_, f) in b.faces() {
                let Some(plane @ geom::Surface::Plane { .. }) = b.get_surface(f.surface) else {
                    continue;
                };
                let roots = lane.surface_crossings(net, curve.params(), plane, band);
                assert!(
                    matches!(roots.as_ref().map(Vec::len), Ok(0)),
                    "{what}: a seam of A crosses a cap of B: {roots:?}"
                );
                read += 1;
            }
        }
        assert!(read > 0, "{what}: A has a spline edge and B a plane face");
    }
}

/// Re-review probe: the two walls' signed crossing sine along each shared
/// rim, read from the walls' own derivatives at the `v = 0` row, finely
/// sampled, with no certificate in the loop.
#[test]
#[ignore = "re-review probe: 7 minutes, prints the measurement"]
fn rr_probe_the_walls_crossing_along_the_rims() {
    let tol = Tol::witness();
    let base = [0.170, 0.028, 0.020];
    let (d, v) = (Vec3::unit_x(), Vec3::unit_z());
    let leaf = blade(
        d,
        v,
        (1.2, -0.3),
        |s| [0, 1, 2].map(|i| base[i] + ([0.300, 0.034, 0.016][i] - base[i]) * s),
        tol,
    );
    let sheath = blade(-d, v, (0.34, 0.85), |_| base, tol);
    let walls = |body: &topo::AtRestBody<f64>| {
        let mut out = Vec::new();
        for (_, f) in body.faces() {
            if let Some(geom::Surface::Nurbs(n)) = body.get_surface(f.surface) {
                let (u0, u1) = n.knot_values_u().domain();
                let (v0, _) = n.knot_values_v().domain();
                let on = (0..=8).all(|k| {
                    let u = u0 + (u1 - u0) * k as f64 / 8.0;
                    n.eval(u, v0).x.abs() < 1e-9
                });
                if on {
                    out.push(n.clone());
                }
            }
        }
        out
    };
    let (la, sa) = (walls(&leaf), walls(&sheath));
    println!("leaf walls on x=0: {}, sheath: {}", la.len(), sa.len());
    for a in &la {
        let (u0, u1) = a.knot_values_u().domain();
        let (va, _) = a.knot_values_v().domain();
        for b in &sa {
            let (w0, w1) = b.knot_values_u().domain();
            let (vb, _) = b.knot_values_v().domain();
            // Same row? compare at the u mid.
            let um = 0.5 * (u0 + u1);
            let p = a.eval(um, va);
            // Locate b's parameter by a fine scan then Newton on |S_b(w) - p|.
            let closest = |p: Point3<f64>| {
                let mut best = (f64::INFINITY, w0);
                for k in 0..=4000 {
                    let w = w0 + (w1 - w0) * k as f64 / 4000.0;
                    let dd = (b.eval(w, vb) - p).norm();
                    if dd < best.0 {
                        best = (dd, w);
                    }
                }
                let mut w = best.1;
                for _ in 0..30 {
                    let dv = b.ders(w, vb);
                    let r = b.eval(w, vb) - p;
                    let step = r.dot(dv.du) / dv.du.dot(dv.du);
                    w = (w - step).clamp(w0, w1);
                }
                (w, (b.eval(w, vb) - p).norm())
            };
            if closest(p).1 > 1e-9 {
                continue;
            }
            println!("rim: leaf u [{u0}, {u1}], sheath u [{w0}, {w1}]");
            let n = 20000;
            let mut prev: Option<(f64, f64)> = None;
            let mut min_abs = (f64::INFINITY, 0.0);
            for k in 0..=n {
                let u = u0 + (u1 - u0) * k as f64 / n as f64;
                let p = a.eval(u, va);
                let (w, miss) = closest(p);
                let da = a.ders(u, va);
                let db = b.ders(w, vb);
                let na = da.du.cross(da.dv);
                let nb = db.du.cross(db.dv);
                let t = da.du * (1.0 / da.du.norm());
                let s = na.cross(nb).dot(t) / (na.norm() * nb.norm());
                if s.abs() < min_abs.0 {
                    min_abs = (s.abs(), u);
                }
                if let Some((pu, ps)) = prev {
                    if ps.signum() != s.signum() {
                        println!(
                            "  sign change between u = {pu:.7} ({ps:+.3e}) and u = {u:.7} ({s:+.3e}), fraction {:.5}, miss {miss:.1e}",
                            (u - u0) / (u1 - u0)
                        );
                    }
                }
                if k % 2000 == 0 {
                    println!(
                        "  u = {u:.5} (fraction {:.3}): sin = {s:+.4e}",
                        (u - u0) / (u1 - u0)
                    );
                }
                prev = Some((u, s));
            }
            println!("  least |sin| = {:.3e} at u = {:.6}", min_abs.0, min_abs.1);
        }
    }
}
