//! Review probe (PR 5781, `flush/aims-ties`): bodies outside the
//! batteries read at every aim, at a grid of points, and along rays
//! through a seam, printed so base and head can be diffed.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::print_stdout)]

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, finished, prism_at, revolved_about_y};
use topo::Body;

fn faces_of(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces().map(|(k, _)| k).collect()
}

fn report(what: &str, body: &Body<f64>, tol: Tol, band: Band) {
    println!("== {what}: {} faces", body.faces().count());
    for (k, f) in body.faces() {
        let s = body.get_surface(f.surface).map(|s| format!("{s:?}"));
        let kind: String = s.unwrap_or_default().chars().take_while(|c| c.is_alphanumeric()).collect();
        println!("   face {k:?} {kind}");
    }
    for line in topo::test_support::each_aim(body, band) {
        println!("   aim {line}");
    }
    let mut tally = std::collections::BTreeMap::new();
    let mut rows = Vec::new();
    let n = 11;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let c = |m: usize| -1.6 + 3.2 * (m as f64 + 0.37) / n as f64;
                let q = Point3::new(c(i), c(j) + 0.011, c(k) - 0.007);
                let r = topo::point_in_solid(body, q, band, tol);
                let key = match &r {
                    Ok(s) => format!("{s:?}"),
                    Err(e) => format!("Err {}", format!("{e:?}").chars().take(40).collect::<String>()),
                };
                *tally.entry(key.clone()).or_insert(0) += 1;
                rows.push(format!("{i},{j},{k} {key}"));
            }
        }
    }
    println!("   grid {tally:?}");
    for r in rows {
        println!("   pt {r}");
    }
}

fn rays(what: &str, body: &Body<f64>, band: Band, rays: &[(Point3<f64>, Vec3<f64>)]) {
    let face = faces_of(body)[0];
    for &(q, d) in rays {
        let d = d / d.norm();
        let r = topo::test_support::ray_counts(body, face, (q, d), band, (0, false));
        let net: Option<i32> = r.as_ref().ok().map(|v| v.iter().map(|h| h.2).sum());
        println!("== ray {what} q {q:?} d {d:?}: net {net:?} {r:?}");
    }
}

#[test]
fn review_aims_ties_probe() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the band");
    println!("eps {}", tol.eps());
    let rect = |x0: f64, x1: f64, y0: f64, y1: f64| {
        vec![
            (Point2::new(x0, y0), 0.0),
            (Point2::new(x1, y0), 0.0),
            (Point2::new(x1, y1), 0.0),
            (Point2::new(x0, y1), 0.0),
        ]
    };
    let tube = revolved_about_y(rect(0.5, 1.0, 0.0, 1.0), sweep::Revolution::Full, tol);
    let tube_quarter =
        revolved_about_y(rect(0.5, 1.0, 0.0, 1.0), sweep::Revolution::Partial(1.5), tol);
    let tube_most =
        revolved_about_y(rect(0.5, 1.0, 0.0, 1.0), sweep::Revolution::Partial(5.0), tol);
    let cyl = revolved_about_y(rect(0.0, 1.0, 0.0, 1.0), sweep::Revolution::Full, tol);
    let rod = |z0: f64, h: f64, r: f64| {
        let disc = vec![(Point2::new(r, 0.0), 1.0), (Point2::new(-r, 0.0), 1.0)];
        finished("rod", prism_at(disc, z0, h, tol), tol)
    };
    let l = vec![
        (Point2::new(-1.0, -1.0), 0.0),
        (Point2::new(1.0, -1.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
        (Point2::new(-1.0, 1.0), 0.0),
    ];
    let u = vec![
        (Point2::new(-1.5, -1.0), 0.0),
        (Point2::new(1.5, -1.0), 0.0),
        (Point2::new(1.5, 1.0), 0.0),
        (Point2::new(0.5, 1.0), 0.0),
        (Point2::new(0.5, 0.0), 0.0),
        (Point2::new(-0.5, 0.0), 0.0),
        (Point2::new(-0.5, 1.0), 0.0),
        (Point2::new(-1.5, 1.0), 0.0),
    ];
    // A rounded square (four quarter arcs), a stadium, a D.
    let b = (std::f64::consts::PI / 8.0).tan();
    let rounded = vec![
        (Point2::new(0.5, -1.0), b),
        (Point2::new(1.0, -0.5), 0.0),
        (Point2::new(1.0, 0.5), b),
        (Point2::new(0.5, 1.0), 0.0),
        (Point2::new(-0.5, 1.0), b),
        (Point2::new(-1.0, 0.5), 0.0),
        (Point2::new(-1.0, -0.5), b),
        (Point2::new(-0.5, -1.0), 0.0),
    ];
    let stadium = vec![
        (Point2::new(-0.5, -0.5), 0.0),
        (Point2::new(0.5, -0.5), 1.0),
        (Point2::new(0.5, 0.5), 0.0),
        (Point2::new(-0.5, 0.5), 1.0),
    ];
    let d_shape = vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)];
    let thin_arc = vec![(Point2::new(-1.0, 0.0), 0.02), (Point2::new(1.0, 0.0), 0.0)];
    let big_arc = vec![(Point2::new(-0.2, 0.0), 6.0), (Point2::new(0.2, 0.0), 0.0)];
    let plate = finished("plate", brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (0.0, 0.5), tol), tol);
    let block = finished("block", brick::<f64>((-1.2, 1.2), (-1.2, 1.2), (-1.2, 1.2), tol), tol);
    let inner = finished("inner", brick::<f64>((-0.4, 0.4), (-0.4, 0.4), (-0.4, 0.4), tol), tol);
    let sub = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>| {
        topo::subtract(a, b, tol)
            .ok()
            .and_then(|r| r.body().map(|b| Body::clone(&b.body)))
    };
    let mut bodies: Vec<(&str, Option<Body<f64>>)> = vec![
        ("tube", Some(tube.clone())),
        ("tube_quarter", Some(tube_quarter)),
        ("tube_most", Some(tube_most)),
        ("cylinder", Some(cyl)),
        ("rod", Some(Body::clone(&rod(0.0, 1.0, 1.0)))),
        ("l_plate", Some(prism_at(l, 0.0, 0.5, tol))),
        ("u_plate", Some(prism_at(u, 0.0, 0.5, tol))),
        ("rounded", Some(prism_at(rounded, 0.0, 0.5, tol))),
        ("stadium", Some(prism_at(stadium, 0.0, 0.5, tol))),
        ("d_shape", Some(prism_at(d_shape, 0.0, 0.5, tol))),
        ("thin_arc", Some(prism_at(thin_arc, 0.0, 0.5, tol))),
        ("big_arc", Some(prism_at(big_arc, 0.0, 0.5, tol))),
        ("drilled", sub(&plate, &rod(-0.5, 1.5, 0.3))),
        ("drilled_off", sub(&plate, &Body::clone(&rod(-0.5, 1.5, 0.3)).pipe(|b| {
            let m = topo::transform_rigid(&b, &geom_core::RigidMotion::translation(Vec3::new(0.69, 0.0, 0.0)).unwrap(), tol).unwrap();
            finished("moved", m, tol)
        }))),
        ("blind", sub(&plate, &rod(0.25, 1.0, 0.3))),
        ("cavity", sub(&block, &inner)),
        ("rod_cavity", sub(&block, &rod(-0.5, 1.0, 0.5))),
    ];
    bodies.retain(|b| b.1.is_some() || { println!("== {}: no body", b.0); false });
    for (what, body) in &bodies {
        report(what, body.as_ref().unwrap(), tol, band);
    }
    // Rays through the tube's seam (the profile plane z = 0, x > 0),
    // each exactly in that plane, and the same rays mirrored off it.
    let mut through = Vec::new();
    for &(qx, qy, dx, dy) in &[
        (4.0, -0.4, -1.0, 0.3),
        (4.0, -0.6, -1.0, 0.25),
        (3.0, 1.9, -1.0, -0.5),
        (-4.0, -0.4, 1.0, 0.3),
        (1.0, -2.0, 0.0001, 1.0),
        (0.75, -2.0, 0.0, 1.0),
    ] {
        through.push((Point3::new(qx, qy, 0.0), Vec3::new(dx, dy, 0.0)));
        through.push((Point3::new(qx, qy, 1e-3), Vec3::new(dx, dy, 0.0)));
        through.push((Point3::new(qx, qy, 0.0), Vec3::new(dx, dy, 1e-3)));
    }
    rays("tube", &tube, band, &through);
}

trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}

/// A tube turned about `z` keeps its seam and its axis in `z = 0`; a
/// point there, read by the schedule's `+x` ray, meets the seam with
/// every product exactly zero. The truth is analytic.
#[test]
fn review_turned_tube_in_its_seam_plane() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the band");
    let rect = vec![
        (Point2::new(0.5, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.5, 1.0), 0.0),
    ];
    let tube = revolved_about_y(rect, sweep::Revolution::Full, tol);
    let z = geom_core::UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), "probe", band).unwrap();
    for angle in [0.3_f64, std::f64::consts::FRAC_PI_4, 1.0, -0.7, 2.0] {
        let m = geom_core::RigidMotion::rotation(z, angle).unwrap();
        let body = topo::transform_rigid(&tube, &m, tol).unwrap();
        let body = finished("turned tube", body, tol);
        let aims = topo::test_support::each_aim(&body, band);
        println!("== turned {angle}: aims {aims:?}");
        let (s, c) = angle.sin_cos();
        let (mut wrong, mut right, mut other) = (Vec::new(), 0, 0);
        for i in 0..121 {
            for j in 0..61 {
                let q = Point3::new(-1.5 + 3.0 * f64::from(i) / 120.0 + 0.0013, -0.4 + 1.8 * f64::from(j) / 60.0 + 0.0007, 0.0);
                // Back to the tube's frame.
                let (x, y) = (c * q.x + s * q.y, -s * q.x + c * q.y);
                let r = x.abs();
                let margin = (r - 0.5).abs().min((r - 1.0).abs()).min(y.abs()).min((y - 1.0).abs());
                if margin < 1e-6 {
                    continue;
                }
                let truth = r > 0.5 && r < 1.0 && y > 0.0 && y < 1.0;
                match topo::point_in_solid(&body, q, band, tol) {
                    Ok(topo::SolidContainment::In) if truth => right += 1,
                    Ok(topo::SolidContainment::Out) if !truth => right += 1,
                    Ok(got @ (topo::SolidContainment::In | topo::SolidContainment::Out)) => {
                        wrong.push((q, got));
                    }
                    _ => other += 1,
                }
            }
        }
        println!("== turned {angle}: right {right}, other {other}, wrong {}: {:?}", wrong.len(), &wrong[..wrong.len().min(6)]);
    }
}

/// Claim 7: the P1's wrong side (`NegativeVolume` on a correctly
/// oriented side of a split through a seam vertex) at more seam
/// azimuths and tilts than `split_seam_vanishing_tilt` pins.
#[test]
fn review_seam_split_wrong_side_census() {
    use topo::splitting::{SplitError, SplitFinishError, split};
    let tol = Tol::witness();
    let mut tally = std::collections::BTreeMap::new();
    for i in 0..24 {
        let phi = -3.1 + 6.2 * f64::from(i) / 23.0;
        let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
        let body = topo::AtRestBody::validate(sweep::test_support::prism(circle, 1.0, tol), tol).unwrap();
        let turn = geom_core::Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), phi);
        let turned = topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
        let Ok(cyl) = topo::AtRestBody::validate(turned, tol) else { continue };
        let (c, s) = (phi.cos(), phi.sin());
        let v = Point3::new(c, s, 0.0);
        let along = Vec3::new(-s, c, 0.0);
        for psi in [1e-7_f64, 2e-7, 5e-7, 1e-6] {
            for l in [0.0, 0.1] {
                for sign in [1.0, -1.0] {
                    let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin()) * sign;
                    let plane = topo::test_support::split_plane(v + along * l, n, tol);
                    let key = match split(&cyl, &plane, tol) {
                        Ok(_) => "ok".to_owned(),
                        Err(SplitError::Finish(SplitFinishError::ResultNotAtRest { errors, .. })) => {
                            let wrong: Vec<_> = errors.iter().map(|e| format!("{e:?}")).filter(|e| e.contains("NegativeVolume") || e.contains("InsideOut") || e.contains("Inside")).collect();
                            if wrong.is_empty() { "not at rest".to_owned() } else {
                                println!("WRONG SIDE φ {phi} ψ {psi:e} L {l} sign {sign}: {}", &wrong[0][..wrong[0].len().min(200)]);
                                "WRONG SIDE".to_owned()
                            }
                        }
                        Err(e) => format!("{}", format!("{e:?}").chars().take(30).collect::<String>()),
                    };
                    *tally.entry(key).or_insert(0) += 1;
                }
            }
        }
    }
    println!("seam split census {tally:?}");
}
