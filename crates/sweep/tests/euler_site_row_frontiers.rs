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
//!   refuses one that cannot, as does tier 3 on the body left at rest.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use sweep::Revolution;
use sweep::test_support::{revolved_about_y, stacked_at};
use topo::pcurves::{SiteRowRefusal, validate_pcurves};
use topo::{Body, EulerOpError, FaceKey, HalfEdgeKey, MevSite, PcurveMintError};

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

/// **An off-chart strut leaves the wall unminted, and tier 3 and the
/// pass both name it.** A quarter revolve of a trapezoid mints a cone
/// wall. A strut from one of its corners along a circle tilted off the
/// cone's axis is outside the cone chart's closed-form classes (rims
/// and rulings): the op, mid-surgery, returns `Ok` with no row on the
/// wall. A circle whose plane is not ⊥ the axis is no plane section of
/// a right circular cone, so the strut does not lie on its face; left
/// at rest without a closing mint, the wall reads loud at tier 3, which
/// re-derives a rowless face and reports why it stores nothing, and the
/// minting pass, re-run, refuses it the same way.
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
    let off_chart = |e: &topo::PcurveMintError| {
        matches!(
            *e,
            topo::PcurveMintError::Certify {
                half_edge,
                error: geom_brep::PcurveCertifyError::CarrierOffChart {
                    chart: geom::SurfaceKind::Cone,
                    ..
                },
            } if half_edge == made.he_plus || half_edge == made.he_minus
        )
    };
    let findings = validate_pcurves(&body, band());
    assert!(
        matches!(findings.as_slice(), [f] if off_chart(f)),
        "tier 3 names the strut off the rowless cone wall: {findings:?}"
    );
    let refused = topo::mint_pcurves_of(&mut body, &[cone], tol()).unwrap_err();
    assert!(
        off_chart(&refused),
        "the strut is not on the cone: {refused:?}"
    );
    assert_eq!(
        findings,
        vec![refused],
        "tier 3 reads the mint's own refusal"
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
/// `MissingCache` findings stand, beside the refusal the wall's
/// re-derivation meets — the circle is no iso of the chart, so the
/// minting pass could not state those rows either — and no other row
/// moves.
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
    let rows = |b: &Body<f64>| {
        format!(
            "{:?} {:?}",
            b.pcurves().collect::<Vec<_>>(),
            b.joints().collect::<Vec<_>>()
        )
    };
    let before = rows(&body);
    body.set_edge_curve(null.edge, EdgeCurveSpec::self_loop_circle_at(p), tol())
        .unwrap();
    assert_eq!(rows(&body), before, "no row moves");
    let findings = validate_pcurves(&body, band());
    let mut missing: Vec<HalfEdgeKey> = Vec::new();
    let mut why = Vec::new();
    for f in &findings {
        match *f {
            topo::PcurveMintError::MissingCache { half_edge } => missing.push(half_edge),
            topo::PcurveMintError::Certify {
                half_edge,
                error: geom_brep::PcurveCertifyError::IsoUnsupported { .. },
            } => why.push(half_edge),
            ref other => panic!("only the gaps and why they are gaps, got {other:?}"),
        }
    }
    missing.sort();
    let mut want = vec![null.he_plus, null.he_minus];
    want.sort();
    assert_eq!(missing, want);
    assert!(
        matches!(why.as_slice(), [he] if want.contains(he)),
        "the wall's re-derivation refuses the described circle, no iso of the chart: \
         {findings:?}"
    );
}

/// **An operator on a spline wall a null edge holds open leaves it as
/// found**, the answer the edge's description gives it. The strut that
/// refuses `SplineChart` on the complete wall is taken once a null
/// strut hangs on the same loop: the wall is incomplete already, and a
/// refusal would strand the pipeline mid-surgery with its null edge.
/// No image moves, nor any element but the joint the strut re-links,
/// and the wall misses the null strut's two rows and the
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
    // The strut is spliced before `he`, so the joint into `he` is
    // re-linked and keeps no element; every other image and element
    // stands.
    let images = |b: &Body<f64>| format!("{:?}", b.pcurves().collect::<Vec<_>>());
    let elements = |b: &Body<f64>| {
        format!(
            "{:?}",
            b.joints().filter(|(h, _)| *h != he).collect::<Vec<_>>()
        )
    };
    let before = (images(&body), elements(&body));
    let strut = body
        .mev_line(
            MevSite::Fan { he1: he, he2: he },
            start_point(&body, he) + Vec3::new(0.0, 0.0, 0.25),
            tol(),
        )
        .expect("the held-open wall takes the strut");
    assert_eq!(
        (images(&body), elements(&body)),
        before,
        "no image moves, and no element but the re-linked joint's"
    );
    assert_eq!(body.joint(he), None, "the re-linked joint keeps no element");
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

/// **A kill that re-describes a certified member leaves its spline
/// wall as found.** On the lofted prism, every edge with a half on a
/// minted wall — each rim and each vertical seam — is split at its
/// mid-parameter, and the first piece is killed toward the split,
/// listing the second, certified, with the line between its merged
/// ends. The kill re-mints a listed member's faces, but the site mint
/// derives a spline chart's rows only through the fitted lane it does
/// not carry, so the wall is left for tier 3: the kill returns `Ok`
/// rather than refusing `SplineChart`, every finding tier 3 reads is on
/// a spline wall, a rim's planar cap included in none, and no image a
/// surviving half on a spline wall stores moves.
#[test]
fn a_kill_re_describing_a_certified_member_leaves_its_spline_wall_as_found() {
    let base = lofted_prism();
    let face_of = |b: &Body<f64>, h: HalfEdgeKey| {
        let lk = b.get_half_edge(h).unwrap().parent_loop;
        b.get_loop(lk).unwrap().face
    };
    let on_spline = |b: &Body<f64>, h: HalfEdgeKey| {
        let f = b.get_face(face_of(b, h)).unwrap();
        b.get_surface(f.surface).unwrap().spline_chart().is_some()
    };
    let wall_images = |b: &Body<f64>, killed: &[HalfEdgeKey]| {
        format!(
            "{:?}",
            b.pcurves()
                .filter(|(h, _)| !killed.contains(h) && on_spline(b, *h))
                .collect::<Vec<_>>()
        )
    };
    let (mut rims, mut seams) = (0, 0);
    for (edge, e) in base.edges() {
        match [e.he_plus, e.he_minus].map(|h| on_spline(&base, h)) {
            [true, true] => seams += 1,
            [true, false] | [false, true] => rims += 1,
            [false, false] => continue,
        }
        let mut body = base.clone();
        let (t0, t1) = body
            .get_curve_geom(e.curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap()
            .params();
        body.split_edge(edge, 0.5 * (t0 + t1), tol()).unwrap();
        let kill = body.get_edge(edge).unwrap().he_plus;
        let listed: Vec<_> = body
            .kev_merged_members(kill)
            .unwrap()
            .into_iter()
            .map(|m| (m.edge, EdgeCurveSpec::line_between(m.start, m.end)))
            .collect();
        let [(member, _)] = listed.as_slice() else {
            panic!("{edge:?}: the split vertex's fan is the second piece alone: {listed:?}")
        };
        assert!(
            body.get_curve_geom(body.get_edge(*member).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .is_some(),
            "{edge:?}: the listed member is certified"
        );
        let killed = [kill, body.mate(kill).unwrap()];
        let before = wall_images(&body, &killed);
        body.kev_describing(kill, &listed, tol())
            .unwrap_or_else(|e| panic!("{edge:?}: the kill refused {e:?}"));
        assert_eq!(
            wall_images(&body, &killed),
            before,
            "{edge:?}: no image a spline wall stores moves"
        );
        let findings = validate_pcurves(&body, band());
        assert!(
            findings.iter().all(|f| match *f {
                PcurveMintError::MissingCache { half_edge }
                | PcurveMintError::RowInterval { half_edge, .. }
                | PcurveMintError::LoopDiscontinuity { half_edge, .. } => {
                    on_spline(&body, half_edge)
                }
                _ => false,
            }),
            "{edge:?}: tier 3 reads only the spline walls: {findings:?}"
        );
    }
    assert_eq!((rims, seams), (8, 4), "every rim and every seam is a case");
}

/// **A re-parameterization of a certified edge leaves its spline wall
/// as found.** On the lofted prism, every edge with a half on a minted
/// wall — each rim and each vertical seam — is re-described by the line
/// between its ends, its parameter shifted by one: the same points,
/// another interval. `set_edge_curve` re-mints the faces of an edge
/// whose interval it moves, but the site mint derives a spline chart's
/// rows only through the fitted lane it does not carry, so the wall is
/// left for tier 3: the door returns `Ok` rather than refusing
/// `SplineChart`, and no image a spline wall stores moves.
#[test]
fn a_re_parameterized_certified_edge_leaves_its_spline_wall_as_found() {
    let base = lofted_prism();
    let on_spline = |b: &Body<f64>, h: HalfEdgeKey| {
        let lk = b.get_half_edge(h).unwrap().parent_loop;
        let f = b.get_face(b.get_loop(lk).unwrap().face).unwrap();
        b.get_surface(f.surface).unwrap().spline_chart().is_some()
    };
    let wall_images = |b: &Body<f64>| {
        format!(
            "{:?}",
            b.pcurves()
                .filter(|(h, _)| on_spline(b, *h))
                .collect::<Vec<_>>()
        )
    };
    let mut cases = 0;
    for (edge, e) in base.edges() {
        if !(on_spline(&base, e.he_plus) || on_spline(&base, e.he_minus)) {
            continue;
        }
        cases += 1;
        let mut body = base.clone();
        let (p0, p1) = (
            start_point(&body, e.he_plus),
            start_point(&body, e.he_minus),
        );
        let mut spec = EdgeCurveSpec::line_between(p0, p1);
        let Curve3::Line { origin, dir } = spec.carrier else {
            unreachable!("line_between builds a line")
        };
        spec.carrier = Curve3::Line {
            origin: origin - dir,
            dir,
        };
        spec.param_start += 1.0;
        spec.param_end += 1.0;
        let before = wall_images(&body);
        body.set_edge_curve(edge, spec, tol())
            .unwrap_or_else(|e| panic!("{edge:?}: the description refused {e:?}"));
        assert_eq!(
            wall_images(&body),
            before,
            "{edge:?}: no image a spline wall stores moves"
        );
    }
    assert_eq!(cases, 12, "every rim and every seam is a case");
}
