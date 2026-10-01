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
//!   the op stores nothing on the face — and the minting pass, which
//!   leaves a face uncovered only for a carrier that can lie on it,
//!   refuses one that cannot.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
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

/// A lofted square prism, whose walls are minted DESCRIBED-NURBS charts.
fn lofted_prism() -> Body<f64> {
    let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
    let sq = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0),
            v(2.0, 0.0),
            v(2.0, 2.0),
            v(0.0, 2.0),
        ])]
    };
    sweep::loft_body::<f64>(&[sq(), sq()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the prism builds")
        .body
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
    let mut body = lofted_prism();
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

/// **An off-chart strut leaves the wall unminted, and the pass names
/// it.** A quarter revolve of a trapezoid mints a cone wall. A strut
/// from one of its corners along a circle tilted off the cone's axis
/// is outside the cone chart's closed-form classes (rims and rulings):
/// the op returns `Ok` with no row on the wall, and tier 3, which reads
/// only stored rows, has nothing to say. A circle whose plane is not ⊥
/// the axis is no plane section of a right circular cone, so the strut
/// does not lie on its face, and the minting pass, re-run, refuses it
/// as `CarrierOffChart` rather than leaving the face uncovered.
#[test]
fn a_tilted_circle_strut_on_a_minted_cone_leaves_the_wall_unminted() {
    let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
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
    let refused = topo::mint_pcurves_of(&mut body, &[cone], tol()).unwrap_err();
    assert!(
        matches!(
            refused,
            topo::PcurveMintError::Certify {
                half_edge,
                error: geom_brep::PcurveCertifyError::CarrierOffChart { chart: "cone", .. },
            } if half_edge == made.he_plus || half_edge == made.he_minus
        ),
        "the strut is not on the cone: {refused:?}"
    );
    assert!(cycle.iter().all(|&he| body.pcurve(he).is_none()));
}

/// **The fitted frontier refuses last.** On the lofted prism's minted
/// wall, a `mef` whose chord misses its far end fails two gates: the
/// geometry gate (the curve does not certify) and the mint-site gate
/// (the wall is a spline chart). The operator's precondition order puts
/// the mint-site refusal after every other check, so the refusal names
/// the certification, and the body is untouched.
#[test]
fn a_chord_that_fails_certification_on_a_spline_wall_names_the_certification() {
    let mut body = lofted_prism();
    let (_, first) = minted_face(&body, |s| s.spline_chart().is_some());
    let cycle = body.loop_cycle(first).unwrap();
    let (he1, he2) = (cycle[0], cycle[2]);
    let p1 = start_point(&body, he1);
    let before = format!("{body:?}");
    let refused = body
        .mef(
            topo::MefSite::Chords { he1, he2 },
            EdgeCurveSpec::line_between(p1, p1 + Vec3::new(0.3, 0.3, 0.3)),
            topo::FaceSurface::Inherit,
            tol(),
        )
        .unwrap_err();
    assert!(
        matches!(refused, EulerOpError::Certification { .. }),
        "{refused:?}"
    );
    assert_eq!(format!("{body:?}"), before);
}

/// **A null edge described on a spline wall leaves the wall as
/// `mev_null` left it.** A null strut on the lofted prism's minted wall
/// leaves it missing the strut's two rows; the edge's first description
/// would mint them, and on a spline chart those rows derive only
/// through the fitted lane, which `set_edge_curve` does not carry. The
/// door describes the edge and leaves the rows as found: the two
/// `MissingCache` findings stand, and no other row moves.
#[test]
fn a_null_edge_described_on_a_spline_wall_leaves_its_rows_as_found() {
    let mut body = lofted_prism();
    let (_, he) = minted_face(&body, |s| s.spline_chart().is_some());
    let p = start_point(&body, he);
    let null = body
        .mev_null(
            MevSite::Fan { he1: he, he2: he },
            topo::NewVertexSide::Above,
        )
        .unwrap();
    let rows = |b: &Body<f64>| format!("{:?}", b.pcurves().collect::<Vec<_>>());
    let before = rows(&body);
    body.set_edge_curve(null.edge, EdgeCurveSpec::self_loop_circle_at(p), tol())
        .unwrap();
    assert_eq!(rows(&body), before, "no row moves");
    let mut missing: Vec<HalfEdgeKey> = validate_pcurves(&body, band())
        .into_iter()
        .map(|f| match f {
            topo::PcurveMintError::MissingCache { half_edge } => half_edge,
            other => panic!("only missing rows are reported, got {other:?}"),
        })
        .collect();
    missing.sort();
    let mut want = vec![null.he_plus, null.he_minus];
    want.sort();
    assert_eq!(missing, want);
}

/// **An operator on a spline wall a null edge holds open leaves it as
/// found**, the answer the edge's description gives it. The strut that
/// refuses `SplineChart` on the complete wall is taken once a null
/// strut hangs on the same loop: the wall is incomplete already, and a
/// refusal would strand the pipeline mid-surgery with its null edge.
/// No row moves, and the wall misses the null strut's two rows and the
/// new strut's two.
#[test]
fn a_strut_on_a_spline_wall_a_null_edge_holds_open_leaves_its_rows_as_found() {
    let mut body = lofted_prism();
    let (_, he) = minted_face(&body, |s| s.spline_chart().is_some());
    let null = body
        .mev_null(
            MevSite::Fan { he1: he, he2: he },
            topo::NewVertexSide::Above,
        )
        .unwrap();
    let rows = |b: &Body<f64>| format!("{:?}", b.pcurves().collect::<Vec<_>>());
    let before = rows(&body);
    let strut = body
        .mev_line(
            MevSite::Fan { he1: he, he2: he },
            start_point(&body, he) + Vec3::new(0.0, 0.0, 0.25),
            tol(),
        )
        .expect("the held-open wall takes the strut");
    assert_eq!(rows(&body), before, "no row moves");
    let mut missing: Vec<HalfEdgeKey> = validate_pcurves(&body, band())
        .into_iter()
        .map(|f| match f {
            topo::PcurveMintError::MissingCache { half_edge } => half_edge,
            other => panic!("only missing rows are reported, got {other:?}"),
        })
        .collect();
    missing.sort();
    let mut want = vec![null.he_plus, null.he_minus, strut.he_plus, strut.he_minus];
    want.sort();
    assert_eq!(missing, want);
}
