//! **The two frontiers of an Euler operator's mint-site rows**, on the
//! charts only a sweep builds: a DESCRIBED-NURBS wall and a CONE.
//!
//! An Euler operator that adds a half-edge to a face whose pcurve rows
//! are complete mints that half-edge's row before it returns, under its
//! own `Decide` bound, through the closed-form derivation the analytic
//! charts share (`topo::pcurves`' `site_rows`; the cylinder rows are
//! `topo`'s `euler_site_pcurve_rows`). Two places sit outside that:
//!
//! - a SPLINE chart, whose images derive from the edge's description
//!   through the fitted lane, which the operators do not carry — the op
//!   refuses there, typed, with the body untouched;
//! - a carrier outside an analytic chart's closed-form classes, where
//!   the minting pass itself leaves the face uncovered — the op gives
//!   the face that same answer, and it stores nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::{ProfileVertex, RawLoop};
use sweep::Revolution;
use sweep::test_support::{revolved_about_y, stacked_at};
use topo::pcurves::{SiteRowRefusal, validate_pcurves};
use topo::{Body, EulerOpError, FaceKey, HalfEdgeKey, MevSite};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The first face of `body` on a chart `pick` accepts that stores a
/// row, with the first half-edge of its outer loop.
fn minted_face(body: &Body<f64>, pick: impl Fn(&Surface<f64>) -> bool) -> (FaceKey, HalfEdgeKey) {
    body.faces()
        .find_map(|(fk, f)| {
            if !pick(body.get_surface(f.surface).unwrap()) {
                return None;
            }
            let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            body.pcurve(first).is_some().then_some((fk, first))
        })
        .expect("the fixture has a minted face on that chart")
}

fn start_point(body: &Body<f64>, he: HalfEdgeKey) -> Point3<f64> {
    let v = body.get_half_edge(he).unwrap().start;
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// **The fitted frontier.** A strut on a lofted prism's wall — a
/// described-NURBS chart, minted by the loft — refuses
/// `SiteRowRefusal::SplineChart`, naming the wall, and leaves the body
/// exactly as it was. Before this refusal existed the op returned `Ok`
/// with the wall half-minted.
#[test]
fn a_strut_on_a_minted_spline_wall_refuses_with_the_body_untouched() {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let sq = || {
        vec![profile::ProfileLoop::new(vec![
            v(0.0, 0.0),
            v(2.0, 0.0),
            v(2.0, 2.0),
            v(0.0, 2.0),
        ])]
    };
    let mut body = sweep::loft_body::<f64>(&[sq(), sq()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the prism builds")
        .body;
    let (wall, he) = minted_face(&body, |s| s.spline_chart().is_some());
    let before = format!("{body:?}");
    let refused = body
        .mev_line(
            MevSite::Fan { he1: he, he2: he },
            start_point(&body, he) + Vec3::new(0.0, 0.0, 0.25),
            tol(),
        )
        .unwrap_err();
    assert_eq!(
        refused,
        EulerOpError::PcurveMint {
            face: wall,
            refusal: SiteRowRefusal::SplineChart
        }
    );
    assert_eq!(format!("{body:?}"), before);
}

/// **An uncovered carrier leaves the face uncovered.** A quarter
/// revolve of a trapezoid mints a cone wall. A strut from one of its
/// corners along a circle tilted off the cone's axis is outside the
/// cone chart's closed-form classes (rims and rulings), so the minting
/// pass would store nothing on that face — and the op gives it that
/// answer: it returns `Ok`, the wall stores no row, and the pass,
/// re-run, agrees.
#[test]
fn a_tilted_circle_strut_on_a_minted_cone_leaves_the_wall_unminted() {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let mut body = revolved_about_y(
        vec![v(1.0, 0.0), v(2.0, 0.0), v(1.5, 1.0), v(1.0, 1.0)],
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    );
    let (cone, he) = minted_face(&body, |s| matches!(s, Surface::Cone { .. }));
    // The revolve axis is the sketch's y-axis, world `Y`: a circle in a
    // plane normal to world `X` is not a rim of this cone.
    let p = start_point(&body, he);
    let (r, theta) = (0.1, 0.5);
    let carrier = Curve3::Circle {
        center: p - Vec3::unit_y() * r,
        axis: Vec3::unit_x(),
        radius: r,
        u_ref: Vec3::unit_y(),
    };
    let end = carrier.eval(theta);
    let spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, theta).unwrap();
    let made = body
        .mev(MevSite::Fan { he1: he, he2: he }, end, spec, tol())
        .unwrap();
    let f = body.get_face(cone).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("the cone wall is bounded by a cycle")
    };
    let cycle = body.loop_cycle(first).unwrap();
    assert!(cycle.contains(&made.he_plus));
    assert!(
        cycle.iter().all(|&he| body.pcurve(he).is_none()),
        "the cone wall kept a row the minting pass would not store"
    );
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    topo::mint_pcurves_of(&mut body, &[cone], tol()).unwrap();
    assert!(cycle.iter().all(|&he| body.pcurve(he).is_none()));
}
