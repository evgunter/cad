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

use geom_core::test_support::upper;
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
/// the fit reaches, its seam with the next wall where the fit
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
    // above it the fit certifies and the wall refuses at its seam.
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
        // The wall's rims with the caps derive as the plane × fit
        // sections; its seam with the next, unmoved spline wall is their
        // section, `Approx × Nurbs`, which has no arm.
        let ReplaceFaceError::NeighborPairUnroutable {
            edge,
            kind: geom::SurfaceKind::Approx,
            other_kind: geom::SurfaceKind::Nurbs,
        } = error.as_ref()
        else {
            panic!("eps {eps:e}: expected the fitted wall's seam to refuse, got {e}");
        };
        assert_wall_seam(&body, *edge, "the refused seam");
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
/// its own limb-2 bound, as the door re-charts the cap: the shell
/// refuses at a cap before any wall moves, so the walls' smooth seams
/// are never reached. A wall moved alone refuses at its fit instead:
/// its net carries a C⁰ crease the fit's Taylor bound cannot cross.
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
                        margin,
                    }),
                ..
            },
        ..
    } = error.as_ref()
    else {
        panic!("expected the rim certificate's limb-2 refusal, got {e}");
    };
    assert!(
        upper(*margin) > 1e2 * Tol::witness().eps(),
        "limb 2 is far past the band, not at its edge: {margin}"
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
    let (wall, _) = walls[0];
    let mut alone = body.clone();
    let e = topo::replace_face_offset(&mut alone, wall, -THICKNESS, Tol::witness())
        .expect_err("a vase wall does not move alone today");
    assert!(
        matches!(
            e,
            ReplaceFaceError::Fit {
                error: geom_brep::OffsetFitError::PatchBound(
                    geom_brep::patch_bound::PatchBoundError::Crease
                ),
                ..
            }
        ),
        "expected the wall's fit to refuse at its crease, got {e}"
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
                margin,
            }) => {
                let value = upper(*margin);
                assert!(
                    micrometres.contains(&value) && value > 10.0 * eps,
                    "eps {eps:e}, d {d}: limb 2 refused at {margin:e}"
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

/// **The edge a fitted face meets a moved plane along is their section
/// over the fit, certified at rest.** The box whose cap wears a
/// certified `Approx` (`common::approx::box_with_approx_cap`; no public
/// door builds a fitted face bounded by planes) has one side wall moved:
/// the side's edge with the fitted cap is derived as the moved plane's
/// section of the fit, stored as their `Intersection` with the plane
/// first and a spline carrier, and the body passes tier 3's structural
/// phase, whose plane × NURBS certificate reads the cap as its fit.
///
/// The corner where the moved side meets the cap and the next side is
/// that side's root at the section's end, within ε of the fit's window
/// edge. The fit itself moves in
/// `a_moved_fitted_cap_stands_its_corners_on_the_held_sides`.
#[test]
fn a_moved_plane_meets_a_fitted_cap_along_their_certified_section() {
    use crate::common::approx::box_with_approx_cap;
    let (body, cap) = box_with_approx_cap(0.05, Tol::witness().eps());
    let cap_key = body.get_face(cap).expect("the cap resolves").surface;
    let side = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { normal, origin, .. })
                    if normal.x.abs() > 0.5 && origin.x > 1.0
            )
        })
        .map(|(k, _)| k)
        .expect("the box has an x = 2 side");
    let side_key = body.get_face(side).expect("the side resolves").surface;
    let mut moved = body.clone();
    topo::replace_face_offset(&mut moved, side, -THICKNESS, Tol::witness())
        .expect("the side moves against the fitted cap");
    let new_side = moved.get_face(side).expect("the side survives").surface;
    assert_ne!(new_side, side_key, "the side wears its moved plane");
    let sections: Vec<_> = moved
        .edges()
        .filter_map(|(_, e)| {
            moved
                .get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
        })
        .filter(|c| {
            matches!(
                *c.description(),
                geom_brep::EdgeDescription::Intersection { s1, s2, .. }
                    if s1 == new_side && s2 == cap_key
            )
        })
        .collect();
    assert_eq!(
        sections.len(),
        1,
        "one edge is the moved side's section of the fitted cap, plane first"
    );
    let section = sections[0];
    assert!(
        matches!(section.carrier(), geom::Curve3::Nurbs(_)),
        "the section's carrier is the plane × fit trace"
    );
    let (t0, t1) = section.params();
    for t in [t0, 0.5 * (t0 + t1), t1] {
        let p = section.carrier().eval(t);
        assert!(
            (p.x - (2.0 - THICKNESS)).abs() < 1e-9 && (p.z - 1.0).abs() < 1e-9,
            "the section lies on the moved side and the cap, at {p:?}"
        );
    }
    // Tier 3 recertifies every edge in its structural phase, the
    // section through the plane × NURBS lane over the fit, and reaches
    // its volume check only on a body that passed that phase. The volume
    // is not taken yet: the fitted cap's trimmed region is past the
    // exact quadrature window
    // (`work/quad/a-fitted-face-trimmed-by-a-section-has-no-volume-rule.md`).
    let refusals = topo::validate_geometric(&moved, Tol::witness())
        .expect_err("the fitted cap's quadrature is not built");
    assert!(
        matches!(
            refusals.as_slice(),
            [topo::ValidationError::VolumeUncomputable {
                source: topo::MassPropsError::Face {
                    face,
                    source: geom_brep::PropsError::QuadratureUnsupported { what },
                    ..
                },
                ..
            }] if *face == cap && what.starts_with("trimmed exact lane's Newton–Cotes window")
        ),
        "expected only the fitted cap's quadrature refusal, got {refusals:?}"
    );
}

/// **A section with a plane names the plane first, whichever seat
/// either surface held before.** The box with the fitted cap has a side
/// moved once, giving the side's edge with the cap as
/// `Intersection { moved side, cap }`; that edge is re-stated with the
/// seats swapped (`Intersection { cap, moved side }`, the same locus,
/// which certifies in either order), and the side moved again. The
/// re-derived edge names the newly moved plane first and the held fit
/// second.
#[test]
fn a_moved_planes_section_with_a_held_fit_names_the_plane_first() {
    use crate::common::approx::box_with_approx_cap;
    use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec};
    let (mut body, cap) = box_with_approx_cap(0.05, Tol::witness().eps());
    let cap_key = body.get_face(cap).expect("the cap resolves").surface;
    let side = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { normal, origin, .. })
                    if normal.x.abs() > 0.5 && origin.x > 1.0
            )
        })
        .map(|(k, _)| k)
        .expect("the box has an x = 2 side");
    let section_of = |body: &topo::Body<f64>, s1, s2| {
        body.edges()
            .find(|(_, e)| {
                body.get_curve_geom(e.curve)
                    .and_then(topo::CurveGeom::certified)
                    .is_some_and(|c| {
                        matches!(
                            *c.description(),
                            EdgeDescription::Intersection { s1: a, s2: b, .. } if a == s1 && b == s2
                        )
                    })
            })
            .map(|(k, _)| k)
    };
    let step = -0.5 * THICKNESS;
    topo::replace_face_offset(&mut body, side, step, Tol::witness())
        .expect("the side moves against the fitted cap");
    let once = body.get_face(side).expect("the side survives").surface;
    let edge = section_of(&body, once, cap_key).expect("the side's section, plane first");
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .expect("the section is certified");
    let (t0, t1) = curve.params();
    let swapped = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: cap_key,
            s2: once,
            witness: curve.carrier().eval(0.5 * (t0 + t1)),
        },
        carrier: curve.carrier().clone(),
        param_start: t0,
        param_end: t1,
    };
    body.set_edge_curve(edge, swapped, Tol::witness())
        .expect("the swapped seats certify");
    assert_eq!(
        section_of(&body, cap_key, once),
        Some(edge),
        "the fit now holds s1"
    );
    topo::replace_face_offset(&mut body, side, step, Tol::witness()).expect("the side moves again");
    let twice = body.get_face(side).expect("the side survives").surface;
    assert_eq!(
        section_of(&body, twice, cap_key),
        Some(edge),
        "the re-derived section names the moved plane first"
    );
}

/// **A moved fitted cap stands its corners on the held sides.** The
/// unit box's cap is swapped for a NURBS patch over `[-1, 3]²` at
/// `z = 1`, wider than the face, and moved by `d`: its offset is a fit,
/// its four edges are each side plane's section with the fit, and each
/// corner is the neighbouring side's root along each of the two
/// sections meeting it (the fit is not rooted along the box's vertical
/// lines). The closed form is the moved box: every corner at
/// `(x, y, 1 + d)` with `x, y ∈ {0, 2}`.
///
/// Each section's corners are rooted at the lever of the section's own
/// domain, the curve the root is isolated over — not the old edge's
/// parameters read on it, which extrapolate past the section's ends.
///
/// Tier 3 passes every phase it reaches and refuses the cap's volume
/// alone, where the edges' spans are a sub-range of their sections'
/// domains (`work/quad/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image.md`);
/// the volume `4 (1 + d)` is not taken yet.
#[test]
fn a_moved_fitted_cap_stands_its_corners_on_the_held_sides() {
    use crate::common::approx::{top_face, unit_box};
    use std::sync::Arc;
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let corner = |x: f64, y: f64| geom_core::Point3::new(x, y, 1.0);
    let patch = geom::NurbsSurface::new(
        kv.clone(),
        kv,
        vec![
            corner(-1.0, -1.0),
            corner(-1.0, 3.0),
            corner(3.0, -1.0),
            corner(3.0, 3.0),
        ],
        vec![1.0; 4],
    )
    .expect("a bilinear patch");
    let eps = Tol::witness().eps();
    for d in [0.05, -0.05] {
        let mut body = unit_box();
        let cap = top_face(&body);
        // Lifts RechartStrandsDescriptions: the cap's chart is the lane under test; its edges are not.
        body.set_face_surface_unvouched_for_tests(
            cap,
            topo::FaceSurface::New {
                surface: geom::Surface::Nurbs(Arc::new(patch.clone())),
                sense: true,
            },
        )
        .expect("the cap takes a NURBS surface");
        let arms = topo::offset_corner_arms_for_tests(&body, cap, d, Tol::witness());
        assert_eq!(arms.len(), 4, "d = {d}: the move derives each cap edge");
        for (edge, carrier, arm) in &arms {
            let geom::Curve3::Nurbs(section) = carrier else {
                panic!("d = {d}: {edge:?} derives a spline section, got {carrier:?}");
            };
            let (lo, hi) = section.domain();
            let own = section.eval(lo).distance(section.eval(hi));
            assert!(
                *arm == own,
                "d = {d}: {edge:?}'s corners are rooted at an arm of {arm}, not its section's own \
                 {own}"
            );
        }
        topo::replace_face_offset(&mut body, cap, d, Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}: the fitted cap moves: {e}"));
        let cap_key = body.get_face(cap).expect("the cap survives").surface;
        assert!(
            matches!(body.get_surface(cap_key), Some(geom::Surface::Approx(_))),
            "d = {d}: the moved cap wears its offset fit"
        );
        let sections = body
            .edges()
            .filter_map(|(_, e)| {
                body.get_curve_geom(e.curve)
                    .and_then(topo::CurveGeom::certified)
            })
            .filter(|c| {
                matches!(
                    *c.description(),
                    geom_brep::EdgeDescription::Intersection { s1, s2, .. }
                        if s2 == cap_key
                            && matches!(body.get_surface(s1), Some(geom::Surface::Plane { .. }))
                ) && matches!(c.carrier(), geom::Curve3::Nurbs(_))
            })
            .count();
        assert_eq!(
            sections, 4,
            "d = {d}: each cap edge is a side's section of the fit, plane first"
        );
        let mut corners = Vec::new();
        for (_, v) in body.vertices() {
            let p = *body.get_point(v.point).expect("a live vertex's point");
            let off = |c: f64, want: &[f64]| {
                want.iter()
                    .map(|w| (c - w).abs())
                    .fold(f64::INFINITY, f64::min)
            };
            let gap = off(p.x, &[0.0, 2.0])
                .max(off(p.y, &[0.0, 2.0]))
                .max(off(p.z, &[0.0, 1.0 + d]));
            assert!(
                gap <= eps,
                "d = {d}: a corner at {p:?} is {gap:e} off the moved box"
            );
            if (p.z - (1.0 + d)).abs() <= eps {
                corners.push(p);
            }
        }
        assert_eq!(
            corners.len(),
            4,
            "d = {d}: the cap's four corners moved by d"
        );
        for (i, a) in corners.iter().enumerate() {
            for b in &corners[i + 1..] {
                assert!(
                    a.distance(*b) > 1.0,
                    "d = {d}: two cap corners stand together, at {a:?} and {b:?}"
                );
            }
        }
        let refusals = topo::validate_geometric(&body, Tol::witness())
            .expect_err("the fitted cap's quadrature is not built");
        assert!(
            matches!(
                refusals.as_slice(),
                [topo::ValidationError::VolumeUncomputable {
                    source: topo::MassPropsError::Face {
                        face,
                        source: geom_brep::PropsError::QuadratureUnsupported { what },
                        ..
                    },
                    ..
                }] if *face == cap
                    && what.starts_with("a General trim image whose carrier interval is not its own knot domain")
            ),
            "d = {d}: expected only the fitted cap's quadrature refusal, got {refusals:?}"
        );
    }
}

/// **A curved fitted cap moves where its fit certifies.** The unit
/// box's cap is swapped for a biquadratic NURBS bump over `[-1, 3]²`
/// (its middle control point raised 0.1 above `z = 1`) and moved by
/// `±0.05`. Where ε is looser than the offset fit reaches (about
/// 2.4e-9 m in its round budget) the move builds: its corners are the
/// side planes' roots along the plane × fit sections, tier 3 refuses the
/// cap's volume alone
/// (`work/quad/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image.md`),
/// and the moved body maps rigidly. Tighter, the fit refuses first.
#[test]
fn a_moved_curved_fitted_cap_builds_where_its_fit_certifies() {
    use crate::common::approx::{top_face, unit_box};
    use std::sync::Arc;
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
        .expect("a clamped quadratic knot vector");
    let mut control = Vec::new();
    for x in [-1.0, 1.0, 3.0] {
        for y in [-1.0, 1.0, 3.0] {
            let bump = if x == 1.0 && y == 1.0 { 0.1 } else { 0.0 };
            control.push(geom_core::Point3::new(x, y, 1.0 + bump));
        }
    }
    let patch =
        geom::NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).expect("a biquadratic bump");
    let eps = Tol::witness().eps();
    for d in [0.05, -0.05] {
        let mut body = unit_box();
        let cap = top_face(&body);
        // Lifts RechartStrandsDescriptions: the cap's chart is the lane under test; its edges are not.
        body.set_face_surface_unvouched_for_tests(
            cap,
            topo::FaceSurface::New {
                surface: geom::Surface::Nurbs(Arc::new(patch.clone())),
                sense: true,
            },
        )
        .expect("the cap takes a NURBS surface");
        let moved = topo::replace_face_offset(&mut body, cap, d, Tol::witness());
        if eps < 2e-9 {
            assert!(
                matches!(
                    moved,
                    Err(ReplaceFaceError::Fit {
                        error: geom_brep::OffsetFitError::BudgetExhausted { .. },
                        ..
                    })
                ),
                "eps {eps:e}, d = {d}: expected the cap's fit to exhaust its budget, got {moved:?}"
            );
            continue;
        }
        moved.unwrap_or_else(|e| panic!("eps {eps:e}, d = {d}: the curved fitted cap moves: {e}"));
        // The side planes stand the corners at x, y ∈ {0, 2}; the bump
        // sets their height.
        for (_, v) in body.vertices() {
            let p = *body.get_point(v.point).expect("a live vertex's point");
            let off = |c: f64| c.abs().min((c - 2.0).abs());
            let gap = off(p.x).max(off(p.y));
            assert!(
                gap <= eps,
                "eps {eps:e}, d = {d}: a corner at {p:?} is {gap:e} off the box's sides"
            );
        }
        let refusals = topo::validate_geometric(&body, Tol::witness())
            .expect_err("the fitted cap's quadrature is not built");
        assert!(
            matches!(
                refusals.as_slice(),
                [topo::ValidationError::VolumeUncomputable {
                    source: topo::MassPropsError::Face {
                        face,
                        source: geom_brep::PropsError::QuadratureUnsupported { what },
                        ..
                    },
                    ..
                }] if *face == cap
                    && what.starts_with("a General trim image whose carrier interval is not its own knot domain")
            ),
            "eps {eps:e}, d = {d}: expected only the fitted cap's quadrature refusal, got {refusals:?}"
        );
        let rigid = geom_core::Affine3::rotation_about_axis(
            geom_core::Point3::new(0.3, -0.2, 0.1),
            Vec3::new(0.0, 0.0, 1.0),
            0.7,
        );
        topo::transform_rigid(&body, &rigid, Tol::witness())
            .unwrap_or_else(|e| panic!("eps {eps:e}, d = {d}: the moved body maps rigidly: {e}"));
    }
}

/// The box whose cap wears the planar NURBS patch over `[0, 2]²`, each
/// cap edge its `IsoLine` image on the patch's chart.
fn spline_capped_box() -> (Body<f64>, FaceKey) {
    use crate::common::approx::{box_with_spline_cap, planar_patch};
    box_with_spline_cap(geom::Surface::Nurbs(std::sync::Arc::new(planar_patch(1.0))))
}

/// The certified curve `edge` of `body` carries.
fn certified_curve(body: &Body<f64>, edge: EdgeKey) -> geom_brep::EdgeCurve<f64> {
    body.get_curve_geom(body.get_edge(edge).expect("a live edge").curve)
        .and_then(topo::CurveGeom::certified)
        .expect("a certified curve")
        .clone()
}

/// **A moved spline cap plans each edge on its image's own row.** The
/// box's cap wears a planar NURBS patch whose chart is
/// `(u, v) ↦ (2u, 2v, 1)`, and each cap edge is described as its
/// `IsoLine` image on that chart (`common::approx::box_with_spline_cap`):
/// the edges along `y` hold `u` fixed, and the edges along `x` hold `v`
/// fixed and MOVE `u`. Every side is a plane containing the cap's
/// normal, so it holds the move, and each edge is the row of the new
/// fit its image names: a `u` row or a `v` row at the image's own fixed
/// parameter, re-stated at the chart's own domain end, its ends the old
/// corners carried along the normal. The door then builds the moved
/// box, every corner of the cap at `z = 1 + d`.
#[test]
fn a_moved_spline_caps_edges_are_each_images_own_row() {
    use geom_brep::{EdgeDescriptionSpec, Pcurve};
    let d = -THICKNESS;
    let (body, cap) = spline_capped_box();
    let cap_key = body.get_face(cap).expect("the cap resolves").surface;
    let (mut u_rows, mut v_rows) = (0, 0);
    for (edge, spec) in topo::offset_edge_specs_for_tests(&body, cap, d, Tol::witness()) {
        let spec = spec.unwrap_or_else(|e| panic!("{edge:?}: the cap edge plans: {e}"));
        let EdgeDescriptionSpec::Chart {
            surface,
            image: Some(Pcurve::IsoLine { p0, pl }),
            ..
        } = spec.description
        else {
            panic!("{edge:?}: a row of the new fit, got {:?}", spec.description);
        };
        assert_eq!(surface, cap_key, "{edge:?}: on the cap's own chart key");
        let old = certified_curve(&body, edge);
        let (t0, t1) = old.params();
        let (a, b) = (old.carrier().eval(t0), old.carrier().eval(t1));
        let fixed = if pl.x == 0.0 {
            u_rows += 1;
            assert!(
                (a.x - b.x).abs() < 1e-12,
                "{edge:?}: a u row is an edge along y"
            );
            (p0.x, a.x)
        } else {
            v_rows += 1;
            assert_eq!(pl.y, 0.0, "{edge:?}: a row holds one parameter fixed");
            assert!(
                (a.y - b.y).abs() < 1e-12,
                "{edge:?}: a v row is an edge along x"
            );
            (p0.y, a.y)
        };
        assert_eq!(
            2.0 * fixed.0,
            fixed.1,
            "{edge:?}: the row is the image's own line, at the chart's own end"
        );
        let shift = Vec3::new(0.0, 0.0, d);
        let ends = [
            spec.carrier.eval(spec.param_start),
            spec.carrier.eval(spec.param_end),
        ];
        assert!(
            ends[0].distance(a + shift) < 1e-9 && ends[1].distance(b + shift) < 1e-9,
            "{edge:?}: the row runs between the carried corners, got {ends:?} for {a:?}–{b:?}"
        );
    }
    assert_eq!((u_rows, v_rows), (2, 2), "two edges of each kind");

    let mut moved = body.clone();
    topo::replace_face_offset(&mut moved, cap, d, Tol::witness())
        .expect("the fitted cap moves between its held sides");
    let mut corners = 0;
    for (he, _) in moved.half_edges() {
        if moved.face_of_half_edge(he) == Some(cap) {
            let p = moved.half_edge_start_point(he).expect("a corner");
            assert!(
                (p.z - (1.0 + d)).abs() < 1e-9,
                "a corner of the moved cap is at z = {}, got {p:?}",
                1.0 + d
            );
            corners += 1;
        }
    }
    assert_eq!(corners, 4, "the cap keeps its four corners");
}

/// **A side that does not hold the move takes the section route.** The
/// box with the spline cap has its `x = 2` side re-charted onto a plane
/// tilted about the cap's edge, so the edge still lies on it but the
/// cap's normal does not. That edge is a row of the cap's fit, and the
/// row moved with the fit would leave the tilted plane; it plans as the
/// section of the new fit with the plane instead, stated as their
/// `Intersection` with the plane first, on the tilted plane and the
/// moved cap.
#[test]
fn a_spline_caps_row_beside_a_tilted_side_is_their_section() {
    use geom_brep::EdgeDescriptionSpec;
    let d = -THICKNESS;
    let (mut body, cap) = spline_capped_box();
    let cap_key = body.get_face(cap).expect("the cap resolves").surface;
    let side = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { normal, origin, .. })
                    if normal.x.abs() > 0.5 && origin.x > 1.0
            )
        })
        .map(|(k, _)| k)
        .expect("the box has an x = 2 side");
    let tilt: f64 = 0.3;
    let normal = Vec3::new(tilt.cos(), 0.0, -tilt.sin());
    // Lifts RechartStrandsDescriptions: only the cap's edge with this
    // side is planned, and it lies on the tilted plane.
    let tilted = body
        .set_face_surface_unvouched_for_tests(
            side,
            topo::FaceSurface::New {
                surface: geom::Surface::Plane {
                    origin: geom_core::Point3::new(2.0, 0.0, 1.0),
                    normal,
                    u_ref: Vec3::new(0.0, 1.0, 0.0),
                },
                sense: true,
            },
        )
        .expect("the attach-layer door accepts a live face");
    let on_tilted =
        |p: geom_core::Point3<f64>| (p - geom_core::Point3::new(2.0, 0.0, 1.0)).dot(normal);
    let mut sections = 0;
    for (edge, spec) in topo::offset_edge_specs_for_tests(&body, cap, d, Tol::witness()) {
        let spec = spec.unwrap_or_else(|e| panic!("{edge:?}: the cap edge plans: {e}"));
        let old = certified_curve(&body, edge);
        let (t0, t1) = old.params();
        if on_tilted(old.carrier().eval(t0)).abs() > 1e-12
            || on_tilted(old.carrier().eval(t1)).abs() > 1e-12
        {
            continue;
        }
        sections += 1;
        assert!(
            matches!(
                spec.description,
                EdgeDescriptionSpec::Intersection { s1, s2, .. } if s1 == tilted && s2 == cap_key
            ),
            "the tilted side's edge is its section with the fit, got {:?}",
            spec.description
        );
        let (s0, s1) = (spec.param_start, spec.param_end);
        for t in [s0, 0.5 * (s0 + s1), s1] {
            let p = spec.carrier.eval(t);
            assert!(
                on_tilted(p).abs() < 1e-9 && (p.z - (1.0 + d)).abs() < 1e-9,
                "the section lies on the tilted side and the moved cap, at {p:?}"
            );
        }
    }
    assert_eq!(sections, 1, "one cap edge lies on the tilted side");
}
