//! **A corner cut on a far wall reads its section at the section's own
//! width.** A plate with one side an arc of radius 10 km, its centre
//! 10 km outside the session box, split by a plane `12ε` inside the
//! corner where the arc wall, a flat wall and the top meet. The section
//! is a triangle about `30ε` across with one side an arc of an ellipse
//! whose centre is as far off as the wall's. The margin
//! `split_section_area` reads for it is in `(ε, 10ε)`, so the split
//! escalates there. That margin is `2A/P` with the arc side counted at
//! its carrier's greatest speed: at most the triangle's geometric width
//! (which for the second cut is itself past `10ε`), and no less than
//! that over the ellipse's axis ratio.
//!
//! A reading of the arc's segment about its far centre is off by four
//! orders here: it certified one of these sections and passed the other
//! to a later stage.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, split};

const FAR: f64 = 1.0e4;

#[test]
fn a_corner_cut_in_the_gap_escalates_at_the_sections_own_width() {
    let tol = Tol::witness();
    let eps = tol.eps();
    if eps < 1.0e-9 {
        test_utils::vacuity::stood_down(
            "ε below 1e-9",
            "the plate's far arcs do not certify there (`carrier_matches_mapped_source`)",
        );
        return;
    }
    let bulge = ((5.0 / FAR).asin() / 2.0).tan();
    let plate = topo::finish(
        sweep::test_support::prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(10.0, 0.0), 0.0),
                (Point2::new(10.0, 1.0), bulge),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            1.0,
            tol,
        ),
        tol,
    )
    .expect("the plate");
    let corner = Point3::new(10.0, 1.0, 1.0);
    // The corner's three edges, leaving it: down the wall, along the
    // top of the flat wall, and along the arc.
    let rise = (FAR * FAR - 25.0).sqrt();
    let edges = [
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(-rise / FAR, 5.0 / FAR, 0.0),
    ];
    for normal in [Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.05, 1.0, 1.0)] {
        let n = normal / normal.norm();
        let d = 12.0 * eps;
        let [a, b, c] = edges.map(|e| corner + e * (-d / n.dot(e)));
        let twice_area = (b - a).cross(c - a).norm();
        let width = twice_area / ((b - a).norm() + (c - b).norm() + (a - c).norm());
        assert!(
            width > 2.0 * eps && width < 20.0 * eps,
            "normal {normal:?}: the cut's triangle is {width:e} wide, not what this row is \
             sized to"
        );
        let plane = topo::test_support::split_plane(corner - n * d, n, tol);
        let got = split(&plate, &plane, tol);
        let Err(SplitError::Join(topo::JoinError::Escalated { diag, .. })) = &got else {
            panic!("normal {normal:?}: a section {width:e} wide read {got:?}");
        };
        assert_eq!(
            diag.predicate,
            Some("split_section_area"),
            "normal {normal:?}: {diag:?}"
        );
        let read = diag
            .margin
            .diagnostic_f64_for_error_text()
            .value()
            .expect("a point margin at f64");
        assert!(
            0.55 * width <= read && read <= width * (1.0 + 1.0e-6),
            "normal {normal:?}: the section is {width:e} wide and the split read {read:e}"
        );
    }
}
