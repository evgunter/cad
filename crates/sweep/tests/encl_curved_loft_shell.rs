//! **Bodies with NURBS walls, against the shell verb and the offset
//! door it runs per face.**
//!
//! The twisted loft (`common::approx::twisted_loft`) is lofted through
//! `sweep::loft_body` between a square and the same square turned
//! 0.3 rad: two planar caps and four bilinear SADDLE walls, so every
//! wall's offset is genuinely not a NURBS and has to be fitted. It is
//! the natural operand for the question "what does a shell of a
//! spline-walled body cost at the run's ε", and these rows pin why that
//! cost cannot be taken yet, at the thickness a user would ask for:
//! `topo::shell` refuses before any wall is fitted.
//!
//! A cap's offset moves the cap's corners, so each seam between two
//! walls that ends at a moved corner is re-anchored on its lofted
//! spline carrier. The twist slants those seams, and the per-chart
//! door moves the cap rigidly along its normal, so the moved corner
//! leaves a slanted seam by the thickness times the slant's sine: the
//! oblique-junction refusal, met here on a spline wall. The straight
//! prism's seams are parallel to the cap normal, so its re-anchor
//! holds and the moved rim lands on an interior row of each wall.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Dual64, Tol, Vec3};
use topo::{Body, EdgeKey, FaceKey, ReplaceFaceError, ShellError};

use crate::common::approx::{nurbs_walls, prism, twisted_loft};
use sweep::test_support::finished;

/// The wall thickness these rows shell at, in metres: 2.5% of the
/// 2 m section, a thickness a user would ask for.
const THICKNESS: f64 = 0.05;

fn is_spline_wall(walls: &[(FaceKey, impl Sized)], face: FaceKey) -> bool {
    walls.iter().any(|(k, _)| *k == face)
}

fn is_cap<T: geom_core::Real>(body: &Body<T>, face: FaceKey) -> bool {
    matches!(
        body.get_face(face)
            .and_then(|f| body.get_surface(f.surface)),
        Some(geom::Surface::Plane { .. })
    )
}

/// The `z = 1` cap, whose chart normal points out of the body.
fn top_cap<T: geom_core::Real + geom_core::Bounds>(body: &Body<T>) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.lo() > 0.5 && origin.z.lo() > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the loft has a top cap")
}

/// `edge` is a seam between two spline walls.
fn assert_wall_seam(body: &Body<f64>, edge: EdgeKey, context: &str) {
    let walls = nurbs_walls(body);
    let halves = body.get_edge(edge).expect("the refused edge resolves");
    let sides: Vec<FaceKey> = [halves.he_plus, halves.he_minus]
        .into_iter()
        .filter_map(|he| body.face_of_half_edge(he))
        .collect();
    assert!(
        sides.len() == 2 && sides.iter().all(|k| is_spline_wall(&walls, *k)),
        "{context}: {edge:?} is not a seam between two spline walls"
    );
}

/// An INWARD cap offset of the curved loft derives its rims: each is
/// the moved plane's section of its wall — an interior row, minted
/// exactly, stated as the cap and wall's `Intersection` with the
/// sketch's record dropped — and each corner is the moved plane's root
/// along its slanted seam, so it slides along the seam rather than
/// along the cap normal.
#[test]
fn the_curved_lofts_cap_moves_its_corners_along_the_slanted_seams() {
    let body = twisted_loft(0.3);
    let walls = nurbs_walls(&body);
    let cap = top_cap(&body);
    let rims = |b: &Body<f64>| {
        b.edges()
            .filter_map(|(_, e)| {
                b.get_curve_geom(e.curve)
                    .and_then(topo::CurveGeom::certified)
            })
            .filter(|c| {
                matches!(
                    c.description(),
                    geom_brep::EdgeDescription::Intersection { .. }
                )
            })
            .count()
    };
    assert_eq!(
        rims(&body),
        0,
        "the loft's cap rims rest as images in the caps' charts"
    );
    let mut moved = body.clone();
    topo::replace_face_offset(&mut moved, cap, -THICKNESS, Tol::witness())
        .expect("the curved loft's cap moves inward");
    assert_eq!(
        rims(&moved),
        4,
        "the moved cap's rims are its sections of the walls"
    );
    let declared = moved
        .edges()
        .filter_map(|(_, e)| {
            moved
                .get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
        })
        .filter(|c| c.authority().is_declared())
        .count();
    assert_eq!(
        declared, 4,
        "only the unmoved cap's rims keep the sketch's record"
    );
    let z = 1.0 - THICKNESS;
    // Each seam ends at a moved corner on the moved plane, and on the
    // seam's own (untouched) carrier.
    let mut corners = 0;
    for (edge, data) in moved.edges() {
        let Some(curve) = moved
            .get_curve_geom(data.curve)
            .and_then(topo::CurveGeom::certified)
        else {
            continue;
        };
        let (t0, t1) = curve.params();
        let (a, b) = (curve.carrier().eval(t0), curve.carrier().eval(t1));
        if (a.z - b.z).abs() < 0.5 {
            continue;
        }
        assert_wall_seam(&moved, edge, "a re-anchored seam");
        let top = if a.z > b.z { a } else { b };
        assert!(
            (top.z - z).abs() < 1e-9,
            "the seam ends on the moved plane, at z = {}",
            top.z
        );
        corners += 1;
    }
    assert_eq!(corners, 4, "four seams meet the moved cap");
    let rows = moved
        .pcurves()
        .filter(|(he, _)| {
            moved
                .face_of_half_edge(*he)
                .is_some_and(|f| is_spline_wall(&walls, f))
        })
        .filter(|(_, c)| {
            matches!(*c.pcurve(), geom_brep::Pcurve::IsoLine { p0, pl }
                if pl.y == 0.0 && (p0.y - z).abs() < 1e-9)
        })
        .count();
    assert_eq!(
        rows, 4,
        "each wall carries the moved rim on its interior row v = {z}"
    );
}

/// With its caps derived, the curved loft's shell moves on to the walls
/// and refuses at the first one: its offset fit where ε is tighter than
/// the fit reaches, its fitted edge with the cap where the fit
/// certifies.
#[test]
fn shelling_the_curved_loft_refuses_at_a_walls_fit() {
    let body = twisted_loft(0.3);
    let e = topo::shell(
        &finished("the operand", body.clone(), Tol::witness()),
        THICKNESS,
        Tol::witness(),
    )
    .expect_err("a spline-walled body does not shell today");
    let ShellError::Face { face, error } = &e else {
        panic!("expected a per-face offset refusal, got {e}");
    };
    assert!(
        is_spline_wall(&nurbs_walls(&body), *face),
        "the refusing face is not a wall: {e}"
    );
    // Which wall door answers first depends on ε: the fit reaches about
    // 4.1e-9 m in its round budget, so below that the fit refuses, and
    // above it the fit certifies and the fitted wall's own edge with the
    // cap refuses (a plane × fitted-surface section C5 does not route).
    let eps = Tol::witness().eps();
    if eps < 1e-8 {
        let ReplaceFaceError::Fit {
            error: geom_brep::OffsetFitError::BudgetExhausted { achieved, .. },
            ..
        } = error.as_ref()
        else {
            panic!("eps {eps:e}: expected the wall's fit to exhaust its budget, got {e}");
        };
        assert!(
            *achieved > eps,
            "eps {eps:e}: the fit stopped short of ε, at {achieved:e}"
        );
    } else {
        assert!(
            matches!(
                error.as_ref(),
                ReplaceFaceError::FittedBoundaryUnsupported { .. }
            ),
            "eps {eps:e}: expected the fitted wall's cap edge to refuse, got {e}"
        );
    }
}

/// An OUTWARD cap offset runs each seam's corner past the end of the
/// wall patch that carries it, by the offset itself: the foot of the
/// moved corner is the carrier's domain end, and the door names that
/// rather than a point off the carrier.
#[test]
fn an_outward_cap_offset_runs_past_the_seam_patchs_end() {
    for (name, body) in [("prism", prism()), ("twisted", twisted_loft(0.3))] {
        let mut moved = body.clone();
        let e = topo::replace_face_offset(&mut moved, top_cap(&body), THICKNESS, Tol::witness())
            .expect_err("the walls end at the cap and do not extend");
        let ReplaceFaceError::ReanchorPastCarrierEnd { edge, gap } = e else {
            panic!("{name}: expected the past-the-end refusal, got {e}");
        };
        assert_wall_seam(&body, edge, name);
        assert!(
            (gap - THICKNESS).abs() < 1e-12,
            "{name}: the corner is the offset past the seam's end, got {gap}"
        );
    }
}

/// The straight prism's seams are parallel to the cap normal, so an
/// INWARD cap offset re-anchors every one of them, and the moved cap's
/// rim runs along an interior row of each wall's chart, where the
/// pcurve mint places it exactly: an iso line at the moved height.
#[test]
fn the_prisms_inward_cap_offset_mints_its_rim_on_the_walls_interior_row() {
    let body = prism();
    let walls = nurbs_walls(&body);
    let mut moved = body.clone();
    let cap = top_cap(&body);
    topo::replace_face_offset(&mut moved, cap, -THICKNESS, Tol::witness())
        .expect("the prism's cap moves inward");
    let mut rims = 0;
    for (he, cache) in moved.pcurves() {
        let Some(face) = moved.face_of_half_edge(he) else {
            continue;
        };
        if !is_spline_wall(&walls, face) {
            continue;
        }
        let geom_brep::Pcurve::IsoLine { p0, pl } = *cache.pcurve() else {
            continue;
        };
        // A row: `u` moves and `v` is fixed, strictly inside the chart.
        if pl.y == 0.0 && p0.y > 0.0 && p0.y < 1.0 {
            assert!(
                (p0.y - (1.0 - THICKNESS)).abs() < 1e-12,
                "the rim's row is the moved cap's height, got v = {}",
                p0.y
            );
            rims += 1;
        }
    }
    assert_eq!(
        rims, 4,
        "each of the four walls carries the moved rim on an interior row"
    );
}

/// A scalar that holds no NURBS lane cannot read a spline seam's foot,
/// and says so by name rather than re-anchoring on a guess.
#[test]
fn a_dual_scalar_refuses_the_spline_seam_re_anchor_by_name() {
    let v = |x: f64, y: f64| (geom_core::Point2::new(x, y), 0.0);
    let square = || {
        vec![profile::test_support::bulge_loop(vec![
            v(0.0, 0.0),
            v(2.0, 0.0),
            v(2.0, 2.0),
            v(0.0, 2.0),
        ])]
    };
    let places = [0.0, 1.0]
        .iter()
        .map(|z| geom_core::Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect::<Vec<_>>();
    let body = sweep::loft_body::<Dual64>(&[square(), square()], &places, 1, Tol::witness())
        .expect("the square prism lofts at a dual")
        .body;
    let mut moved = body.clone();
    let cap = top_cap(&body);
    let Err(ReplaceFaceError::NurbsLaneUnsupported { scalar, .. }) = topo::replace_face_offset(
        &mut moved,
        cap,
        <Dual64 as geom_core::Real>::from_f64(-THICKNESS),
        Tol::witness(),
    ) else {
        panic!("expected the lane refusal at a dual");
    };
    assert_eq!(scalar, <Dual64 as geom_core::Real>::NAME);
}

/// The circular vase: three circle sections of radius 1, 1.3 and 1 at
/// heights 0, 1 and 2, lofted at degree 2 — two spline walls whose
/// seams leave each cap at a slant.
fn vase() -> Body<f64> {
    let circle = |r: f64| {
        vec![profile::test_support::bulge_loop(vec![
            (geom_core::Point2::new(r, 0.0), 1.0),
            (geom_core::Point2::new(-r, 0.0), 1.0),
        ])]
    };
    let places = [0.0, 1.0, 2.0]
        .iter()
        .map(|z| geom_core::Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect::<Vec<_>>();
    sweep::loft_body::<f64>(
        &[circle(1.0), circle(1.3), circle(1.0)],
        &places,
        2,
        Tol::witness(),
    )
    .expect("the vase lofts")
    .body
}

/// The vase's walls are rational, and the moved plane's section of one
/// is not exact structure as a row (its skinned weights differ along
/// the stacking by an ulp), so the door marches it. The plane × NURBS
/// certificate then refuses the marched rim on its rational wall, by
/// its own limb-2 bound, as the door re-charts the cap.
#[test]
fn shelling_the_vase_refuses_at_its_rims_certificate() {
    let body = vase();
    let e = topo::shell(
        &finished("the vase", body.clone(), Tol::witness()),
        THICKNESS,
        Tol::witness(),
    )
    .expect_err("a spline-walled body does not shell today");
    let ShellError::Face { face, error } = &e else {
        panic!("expected a per-face offset refusal, got {e}");
    };
    assert!(is_cap(&body, *face), "the refusing face is not a cap: {e}");
    let ReplaceFaceError::Op {
        error:
            topo::EulerOpError::RechartFalsifies {
                edge,
                error:
                    geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                        limb: geom_brep::ssi::SsiLimb::HullSup,
                        value,
                        ..
                    }),
                ..
            },
        ..
    } = error.as_ref()
    else {
        panic!("expected the rim certificate's limb-2 refusal, got {e}");
    };
    assert!(
        *value > 1e2 * Tol::witness().eps(),
        "limb 2 is far past the band, not at its edge: {value}"
    );
    let data = body.get_edge(*edge).expect("the rim resolves");
    let walls = nurbs_walls(&body);
    assert!(
        [data.he_plus, data.he_minus]
            .into_iter()
            .filter_map(|he| body.face_of_half_edge(he))
            .any(|f| is_spline_wall(&walls, f)),
        "the refused rim bounds a spline wall"
    );
}

/// A straight square prism lofted to a top section tilted 0.2 rad about
/// a line through its centre: four POLYNOMIAL bilinear walls whose
/// rows are not level in the top cap's normal, so a moved top cap's
/// section of a wall is no row of it and the section lane marches it.
fn tilted_top_prism() -> Body<f64> {
    let v = |x: f64, y: f64| (geom_core::Point2::new(x, y), 0.0);
    let square = || {
        vec![profile::test_support::bulge_loop(vec![
            v(0.0, 0.0),
            v(2.0, 0.0),
            v(2.0, 2.0),
            v(0.0, 2.0),
        ])]
    };
    let tilt = geom_core::Affine3::rotation_about_axis(
        geom_core::Point3::new(1.0, 1.0, 1.0),
        Vec3::unit_x(),
        0.2,
    ) * geom_core::Affine3::translation(Vec3::new(0.0, 0.0, 1.0));
    let places = [geom_core::Affine3::identity(), tilt];
    sweep::loft_body::<f64>(&[square(), square()], &places, 1, Tol::witness())
        .expect("the tilted-top prism lofts")
        .body
}

/// The march arm builds nothing yet: the moved tilted cap's section of
/// each polynomial wall is certified by the march, and the plane ×
/// NURBS edge certificate's limb 2 measures it a few micrometres off,
/// though the marched branch lies on both surfaces to ~5e-11
/// (`work/ssiedge/plane-nurbs-limb-two-refuses-a-non-row-section.md`).
/// The gap is a length, not a multiple of ε, so which verdict it earns
/// is ε's: past the band it refuses, inside the band it escalates.
#[test]
fn a_tilted_caps_marched_rim_refuses_at_its_certificate() {
    let body = tilted_top_prism();
    let cap = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if origin.z > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the tilted top cap");
    // The march's own tolerance is ε's, so the gap moves with it (about
    // 3.1e-6 to 3.7e-6 at d = -0.05 and 1.4e-5 to 1.5e-5 at d = -0.2
    // over ε in [1e-12, 1e-6]); it stays micrometres.
    let eps = Tol::witness().eps();
    let micrometres = 1e-6..1e-4;
    for d in [-0.05, -0.2] {
        let mut moved = body.clone();
        let e = topo::replace_face_offset(&mut moved, cap, d, Tol::witness())
            .expect_err("the marched rim does not certify");
        let ReplaceFaceError::Op {
            error: topo::EulerOpError::RechartFalsifies { error, .. },
            ..
        } = &e
        else {
            panic!("d {d}: expected the marched rim's certificate to refuse, got {e}");
        };
        match error {
            geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                limb: geom_brep::ssi::SsiLimb::HullSup,
                value,
                ..
            }) => {
                assert!(
                    micrometres.contains(value) && *value > 10.0 * eps,
                    "eps {eps:e}, d {d}: limb 2 refused at {value:e}"
                );
            }
            geom_brep::CertifyError::Escalated { check, .. } => {
                assert_eq!(*check, geom_brep::ssi::SsiLimb::HullSup.check());
                assert!(
                    10.0 * eps > micrometres.start,
                    "eps {eps:e}, d {d}: limb 2 escalated with no micrometre in the band"
                );
            }
            _ => panic!("eps {eps:e}, d {d}: expected limb 2 to answer, got {e}"),
        }
    }
}

/// At `Interval`: the straight prism's cap moves through the decided
/// shortcut (its walls carry the move onto themselves), and the curved
/// loft's slanted rim, which only the `f64` section lane derives,
/// refuses by name.
#[test]
fn at_interval_the_prism_moves_by_the_shortcut_and_a_slanted_rim_refuses_by_name() {
    use geom_core::{Interval, Real};
    let v = |x: f64, y: f64| (geom_core::Point2::new(x, y), 0.0);
    let square = |turn: f64| {
        let (s, c) = turn.sin_cos();
        let r = |x: f64, y: f64| {
            v(
                1.0 + c * (x - 1.0) - s * (y - 1.0),
                1.0 + s * (x - 1.0) + c * (y - 1.0),
            )
        };
        vec![profile::test_support::bulge_loop(vec![
            r(0.0, 0.0),
            r(2.0, 0.0),
            r(2.0, 2.0),
            r(0.0, 2.0),
        ])]
    };
    let places = [0.0, 1.0]
        .iter()
        .map(|z| geom_core::Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect::<Vec<_>>();
    let loft = |turn: f64| {
        sweep::loft_body::<Interval>(&[square(0.0), square(turn)], &places, 1, Tol::witness())
            .expect("the loft builds at interval")
            .body
    };
    let d = <Interval as Real>::from_f64(-THICKNESS);
    let prism = loft(0.0);
    let mut moved = prism.clone();
    topo::replace_face_offset(&mut moved, top_cap(&prism), d, Tol::witness())
        .expect("the prism's cap moves through the shortcut at interval");
    let twisted = loft(0.3);
    let mut moved = twisted.clone();
    let Err(ReplaceFaceError::NurbsLaneUnsupported { scalar, .. }) =
        topo::replace_face_offset(&mut moved, top_cap(&twisted), d, Tol::witness())
    else {
        panic!("expected the section lane's refusal at interval");
    };
    assert_eq!(scalar, <Interval as Real>::NAME);
}
