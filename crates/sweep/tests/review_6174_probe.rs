//! Review 6174 probe (not part of the PR): one census pose's split,
//! its outcome printed; run with `REVIEW_RAY_DUMP=1` to dump each aimed
//! ray (`side_reader`).
#![allow(clippy::print_stdout, clippy::unwrap_used)]

use geom_core::{Point2, Point3, Tol, Vec3};
use topo::AtRestBody;
use topo::splitting::split;

#[test]
fn review_6174_one_pose() {
    let tol = Tol::witness();
    let i: f64 = std::env::var("REVIEW_I").map_or(1.0, |s| s.parse().unwrap());
    let psi: f64 = std::env::var("REVIEW_PSI").map_or(5e-7, |s| s.parse().unwrap());
    let l: f64 = std::env::var("REVIEW_L").map_or(0.0, |s| s.parse().unwrap());
    let sign: f64 = std::env::var("REVIEW_SIGN").map_or(1.0, |s| s.parse().unwrap());
    let phi = -3.1 + 6.2 * i / 23.0;
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let body = AtRestBody::validate(sweep::test_support::prism(circle, 1.0, tol), tol).unwrap();
    let turn = geom_core::Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        phi,
    );
    let cyl = AtRestBody::validate(
        topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol), tol).unwrap(),
        tol,
    )
    .unwrap();
    let (c, s) = (phi.cos(), phi.sin());
    let v = Point3::new(c, s, 0.0);
    let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin()) * sign;
    let plane = topo::test_support::split_plane(v + Vec3::new(-s, c, 0.0) * l, n, tol);
    let got = split(&cyl, &plane, tol);
    println!(
        "REVIEW-POSE phi {phi} psi {psi:e} l {l} sign {sign} eps {:e}: {}",
        tol.eps(),
        match &got {
            Ok(_) => "built".to_owned(),
            Err(e) => format!("{e:?}").chars().take(600).collect(),
        }
    );
}
