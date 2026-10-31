//! **A seam inside a face is no boundary of it**
//! (`work/flush/the-side-readers-aims-decide-exact-ties-at-a-documents-nominal.md`):
//! a ray read by the turn of a face's loops leaves the face's own seam
//! out, so a point read in the plane holding a revolved tube's axis and
//! its seam (where the schedule's `+x` ray runs through the seam, every
//! product along it exactly zero) reads its analytic side, and so does
//! the same tube with a slit across a cap in that plane.
//!
//! The slit half guards the line pass only, not the seam skip: the slit
//! is a line, which `edge_meets` meets wherever the ray crosses the cap
//! (the ray's trace on the cap is the slit's own line), and a face met
//! at an edge never takes the turn. It is no witness that a planar face
//! leaves its own seam out of the turn. A curved slit would be one, but
//! none was built: the cap's only vertices are its rims' seam vertices,
//! both on that line, and a circle arc through them meets the line only
//! there, so the ray's trace never crosses such a slit inside its span.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::test_support::{finished, revolved_about_y};
use topo::construction::{Construct, MekrSite};
use topo::{AtRestBody, Body, ContactRecords, LoopBoundary, SolidContainment};

/// The tube `0.5 < r < 1`, `0 < y < 1` about the `y` axis; with
/// `slit`, its `y = 0` cap's two rims joined by a chord across it.
fn tube(slit: bool, tol: Tol) -> Body<f64> {
    let rect = vec![
        (Point2::new(0.5, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.5, 1.0), 0.0),
    ];
    let mut body = revolved_about_y(rect, sweep::Revolution::Full, tol);
    if slit {
        let (_, cap) = body
            .faces()
            .find(|(_, f)| {
                let s = body.get_surface(f.surface).unwrap();
                matches!(s, geom::Surface::Plane { origin, .. } if origin.y.abs() < 1e-12)
                    && !f.rings.is_empty()
            })
            .expect("the y = 0 cap is an annulus");
        let first = |l| match body.get_loop(l).unwrap().boundary {
            LoopBoundary::Cycle { first } => first,
            LoopBoundary::Empty { .. } => panic!("the cap's rims are cycles"),
        };
        let (target, ring) = (first(cap.outer), first(cap.rings[0]));
        let at = |h| {
            let v = body.get_half_edge(h).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        };
        let chord = geom_brep::EdgeCurveSpec::line_between(at(target), at(ring));
        body.mekr(MekrSite::Cycles { target, ring }, chord, tol)
            .expect("the slit");
    }
    body
}

/// Whether `q`, in the tube's own frame, lies in it, and how far it lies
/// from its boundary.
fn truth(q: Point3<f64>) -> (bool, f64) {
    let r = q.x.hypot(q.z);
    let margin = (r - 0.5)
        .abs()
        .min((r - 1.0).abs())
        .min(q.y.abs())
        .min((q.y - 1.0).abs());
    (r > 0.5 && r < 1.0 && q.y > 0.0 && q.y < 1.0, margin)
}

/// Every point of a grid in `z = 0` of the tube turned about `z`, read
/// by `point_in_solid` against its analytic side, and rays in that plane
/// through the seam read whole: a ray from outside nets no crossing.
#[test]
fn a_turned_tube_reads_its_side_in_its_seam_plane() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the band");
    let z = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), "the turn's axis", band).unwrap();
    for slit in [false, true] {
        for angle in [0.3_f64, core::f64::consts::FRAC_PI_4, 1.0, -0.7, 2.0] {
            let what = format!("slit {slit}, turned {angle}");
            let m = RigidMotion::rotation(z, angle).unwrap();
            let turned = topo::transform_rigid(&tube(slit, tol), &m, tol).unwrap();
            let body: Body<f64> = if slit {
                turned
            } else {
                let body: AtRestBody<f64> = finished("the turned tube", turned, tol);
                assert!(
                    topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol).is_ok(),
                    "{what}: the body is at rest with no contacts"
                );
                body.into_body()
            };
            let (s, c) = angle.sin_cos();
            let back = |q: Point3<f64>| Point3::new(c * q.x + s * q.y, -s * q.x + c * q.y, q.z);
            let mut wrong = Vec::new();
            let mut read = 0;
            for i in 0..61 {
                for j in 0..31 {
                    let q = Point3::new(
                        -1.5 + 3.0 * f64::from(i) / 60.0 + 0.0013,
                        -0.4 + 1.8 * f64::from(j) / 30.0 + 0.0007,
                        0.0,
                    );
                    let (inside, margin) = truth(back(q));
                    if margin < 1e-6 {
                        continue;
                    }
                    let want = if inside {
                        SolidContainment::In
                    } else {
                        SolidContainment::Out
                    };
                    match topo::point_in_solid(&body, q, band, tol) {
                        Ok(got) if got == want => read += 1,
                        got => wrong.push((q, got)),
                    }
                }
            }
            assert!(
                wrong.is_empty() && read > 1500,
                "{what}: {} of the grid read wrong or not at all ({read} right): {:?}",
                wrong.len(),
                &wrong[..wrong.len().min(4)]
            );
            // Rays in z = 0 from outside, crossing the seam's ruling and the
            // slit's line: each nets nothing.
            let face = body.faces().next().unwrap().0;
            let to = |p: Point3<f64>| Point3::new(c * p.x - s * p.y, s * p.x + c * p.y, 0.0);
            for (x, y, dx, dy) in [
                (-3.0, 0.5, 1.0, 0.0),
                (-3.0, 0.31, 1.0, 0.0),
                (0.9, 2.0, 0.07, -1.0),
                (0.2, 2.0, 0.21, -1.0),
            ] {
                let q = to(Point3::new(x, y, 0.0));
                let d = Vec3::new(c * dx - s * dy, s * dx + c * dy, 0.0);
                let d = d / d.norm();
                let counts = topo::test_support::ray_counts(&body, face, (q, d), band, (0, false))
                    .unwrap_or_else(|e| panic!("{what}: the ray from {q:?} reads: {e}"));
                let net: i32 = counts.iter().map(|h| h.2).sum();
                assert_eq!(
                    net, 0,
                    "{what}: the ray from {q:?} along {d:?} nets {net}: {counts:?}"
                );
            }
        }
    }
}
