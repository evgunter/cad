//! Review probes for PR 4826 (one certified conic side). Not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::Bracket;
use geom_core::{Band, Point2, Point3, Sign, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::splitting::{SplitPart, split};
use topo::{AtRestBody, SolidContainment, point_in_solid};

const R: f64 = 0.5;
const DEPTH: f64 = 6.0;

fn cylinder(tol: Tol) -> AtRestBody<f64> {
    let lp = bulge_loop(vec![(Point2::new(-R, 0.0), 1.0), (Point2::new(R, 0.0), 1.0)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol).unwrap();
    let c = extrude(
        &profile,
        Extrusion::Distance {
            depth: DEPTH,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    topo::test_support::finished("cyl", c, tol)
}

/// Oblique-cut lower piece and the cap ellipse's +x major end.
fn elliptic_piece(tol: Tol) -> (AtRestBody<f64>, Point3<f64>) {
    let body = cylinder(tol);
    let theta = 0.2f64.acos();
    let cut = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, DEPTH / 2.0),
        Vec3::new(-theta.sin(), 0.0, theta.cos()),
        tol,
    );
    let lower = split(&body, &cut, tol).unwrap().below.body().unwrap().clone();
    let apex = Point3::new(R, 0.0, DEPTH / 2.0 + R * theta.tan());
    (topo::test_support::finished("lower", lower, tol), apex)
}

fn inside_cyl(p: Point3<f64>) -> bool {
    p.x * p.x + p.y * p.y < (R * 0.98).powi(2) && p.z > 0.02 && p.z < DEPTH - 0.02
}

/// Oracle: every grid point of `orig` (as `inside`) definitely off the
/// plane lies in the piece on its side and not in the other.
fn check_membership(
    label: &str,
    inside: &dyn Fn(Point3<f64>) -> bool,
    above: &SplitPart<f64>,
    below: &SplitPart<f64>,
    origin: Point3<f64>,
    n: Vec3<f64>,
    tol: Tol,
) -> usize {
    let band = Band::linear(tol).unwrap();
    let mut checked = 0;
    for i in 0..9 {
        for j in 0..9 {
            for k in 0..25 {
                let p = Point3::new(
                    -R + (i as f64 + 0.5) * 2.0 * R / 9.0,
                    -R + (j as f64 + 0.5) * 2.0 * R / 9.0,
                    (k as f64 + 0.5) * DEPTH / 25.0,
                );
                if !inside(p) {
                    continue;
                }
                let d = (p - origin).dot(n);
                if d.abs() < 1e-3 {
                    continue;
                }
                let (want, other) = if d > 0.0 { (above, below) } else { (below, above) };
                let inn = |part: &SplitPart<f64>| match part.body() {
                    Some(b) => point_in_solid(b, p, band, tol).unwrap(),
                    None => SolidContainment::Out,
                };
                assert_eq!(inn(want), SolidContainment::In, "{label}: {p:?} (d={d}) not in its side");
                assert_eq!(inn(other), SolidContainment::Out, "{label}: {p:?} (d={d}) in the other side");
                checked += 1;
            }
        }
    }
    checked
}

fn side_verdicts(v: &[geom_core::k_stats::Verdict]) -> Vec<(&'static str, Sign)> {
    v.iter()
        .filter(|x| x.predicate.starts_with("conic_") || x.predicate.starts_with("tangent_sector_order2"))
        .map(|x| (x.predicate, x.sign))
        .collect()
}

/// Claim 1: the ellipse piece split by the ruling-tangent plane with the
/// normal flipped to −x: the piece must land whole ABOVE.
#[test]
fn p1_elliptic_ruling_plane_flipped_normal_lands_above() {
    let tol = Tol::witness();
    let (piece, apex) = elliptic_piece(tol);
    for (n, want_above) in [(Vec3::new(1.0, 0.0, 0.0), false), (Vec3::new(-1.0, 0.0, 0.0), true)] {
        let plane = topo::test_support::split_plane(apex, n, tol);
        let br = Bracket::open();
        let r = split(&piece, &plane, tol).unwrap_or_else(|e| panic!("{n:?}: {e:?}"));
        let v = br.finish().verdicts;
        eprintln!("p1 n={n:?}: {:?}", side_verdicts(&v));
        let (whole, empty) = if want_above { (&r.above, &r.below) } else { (&r.below, &r.above) };
        assert!(matches!(empty, SplitPart::Empty), "{n:?}: the other side is empty");
        assert!(whole.body().is_some(), "{n:?}: whole piece");
        let want_sign = if want_above { Sign::Positive } else { Sign::Negative };
        let c = v.iter().filter(|x| x.predicate == "conic_bend_side" && x.sign == want_sign).count();
        assert!(c >= 2, "{n:?}: conic_bend_side {want_sign:?} count {c}");
        let th = 0.2f64.acos();
        let inside = |p: Point3<f64>| {
            inside_cyl(p) && -th.sin() * p.x + th.cos() * (p.z - DEPTH / 2.0) < -0.02
        };
        let k = check_membership("p1", &inside, &r.above, &r.below, apex, n, tol);
        assert!(k > 50);
    }
}

/// Claim 1: circular cylinder, plane x = R tangent along the ruling,
/// both normals.
#[test]
fn p2_circle_ruling_plane_both_normals() {
    let tol = Tol::witness();
    let body = cylinder(tol);
    let o = Point3::new(R, 0.0, 0.0);
    for (n, want_above) in [(Vec3::new(1.0, 0.0, 0.0), false), (Vec3::new(-1.0, 0.0, 0.0), true)] {
        let plane = topo::test_support::split_plane(o, n, tol);
        let br = Bracket::open();
        let r = split(&body, &plane, tol).unwrap_or_else(|e| panic!("{n:?}: {e:?}"));
        let v = br.finish().verdicts;
        eprintln!("p2 n={n:?}: {:?}", side_verdicts(&v));
        let (whole, empty) = if want_above { (&r.above, &r.below) } else { (&r.below, &r.above) };
        assert!(matches!(empty, SplitPart::Empty));
        assert!(whole.body().is_some());
        let k = check_membership("p2", &inside_cyl, &r.above, &r.below, o, n, tol);
        assert!(k > 50);
    }
}

/// Claim 1: a plane through the bottom-cap vertex holding the circle's
/// tangent (y) but tilted about it, so it cuts the body; both normal
/// signs. The arcs at the vertex depart in the plane.
#[test]
fn p3_circle_tilted_tangent_plane_cuts_both_normals() {
    let tol = Tol::witness();
    let body = cylinder(tol);
    let o = Point3::new(R, 0.0, 0.0);
    for alpha in [0.3f64, 1.2] {
        for s in [1.0, -1.0] {
            let n = Vec3::new(alpha.cos(), 0.0, alpha.sin()) * s;
            let plane = topo::test_support::split_plane(o, n, tol);
            let br = Bracket::open();
            let r = split(&body, &plane, tol).unwrap_or_else(|e| panic!("{alpha} {s}: {e:?}"));
            let v = br.finish().verdicts;
            eprintln!("p3 alpha={alpha} s={s}: {:?}", side_verdicts(&v));
            assert!(
                v.iter().any(|x| x.predicate == "conic_bend_side"),
                "{alpha} {s}: the in-plane arc departure was read by its bend"
            );
            let k = check_membership("p3", &inside_cyl, &r.above, &r.below, o, n, tol);
            assert!(k > 50);
        }
    }
}

/// Claim 2: a plane through the bottom-cap vertex tilted off the cap by
/// β about the circle's tangent, so the cap's far end stands m·ε above
/// it: the whole cylinder lies Above, touching at one vertex (and the
/// far semicircle points definitely off). The semicircle's real rise
/// is 2R·sinβ = m·ε; the certified lift caps at a quarter turn, R·sinβ.
#[test]
fn p4_near_cap_plane_window() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let body = cylinder(tol);
    let o = Point3::new(R, 0.0, 0.0);
    let mut fails = vec![];
    for m in [12.0, 16.0, 19.0, 25.0, 40.0] {
        let beta = (m * eps / (2.0 * R)).asin();
        let n = Vec3::new(-beta.sin(), 0.0, beta.cos());
        let plane = topo::test_support::split_plane(o, n, tol);
        let br = Bracket::open();
        let r = split(&body, &plane, tol);
        let v = br.finish().verdicts;
        eprintln!("p4 m={m}: {:?} -> {:?}", side_verdicts(&v), r.as_ref().map(|r| (r.above.body().is_some(), r.below.body().is_some())));
        match r {
            Ok(r) => {
                if !(matches!(r.below, SplitPart::Empty) && r.above.body().is_some()) {
                    fails.push(format!("m={m}: wrong answer"));
                }
            }
            Err(e) => fails.push(format!("m={m}: refused {e:?}")),
        }
    }
    assert!(fails.is_empty(), "{fails:#?}");
}

/// Claim 2: the same near-cap plane, but the cylinder's circles are cut
/// into quarter arcs. The certified lift levers the chord as an arc
/// length (`2r·sin²(c/2r)`), 0.84 of the far end's true rise `r·sinβ`,
/// which is also `split_conic_plane_parallel`'s margin. For `r·sinβ` just
/// past `K·ε` the parallel gate and the old sagitta (exactly the far
/// end's rise) decide, and the certified lift falls in band.
#[test]
fn p5_quarter_arc_near_cap_plane_window() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let q = (core::f64::consts::PI / 8.0).tan();
    let lp = bulge_loop(vec![
        (Point2::new(R, 0.0), q),
        (Point2::new(0.0, R), q),
        (Point2::new(-R, 0.0), q),
        (Point2::new(0.0, -R), q),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol).unwrap();
    let c = extrude(
        &profile,
        Extrusion::Distance {
            depth: DEPTH,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    let body = topo::test_support::finished("cyl4", c, tol);
    let o = Point3::new(R, 0.0, 0.0);
    let mut fails = vec![];
    for m in [10.5, 11.0, 11.5, 12.0, 15.0] {
        let beta = (m * eps / R).asin();
        let n = Vec3::new(-beta.sin(), 0.0, beta.cos());
        let plane = topo::test_support::split_plane(o, n, tol);
        let br = Bracket::open();
        let r = split(&body, &plane, tol);
        let v = br.finish().verdicts;
        eprintln!("p5 m={m}: {:?} -> {:?}", side_verdicts(&v), r.as_ref().map(|r| (r.above.body().is_some(), r.below.body().is_some())));
        match r {
            Ok(r) => {
                if !(matches!(r.below, SplitPart::Empty) && r.above.body().is_some()) {
                    fails.push(format!("m={m}: wrong answer"));
                }
            }
            Err(e) => fails.push(format!("m={m}: refused {e:?}")),
        }
    }
    assert!(fails.is_empty(), "{fails:#?}");
}
