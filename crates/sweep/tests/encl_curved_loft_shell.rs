//! **Bodies with NURBS walls, against the shell verb and the offset
//! doors it runs.**
//!
//! The twisted loft (`common::approx::twisted_loft`) is lofted through
//! `sweep::loft_body` between a square and the same square turned
//! 0.3 rad: two planar caps and four bilinear SADDLE walls, so every
//! wall's offset is genuinely not a NURBS and has to be fitted. It is
//! the natural operand for the question "what does a shell of a
//! spline-walled body cost at the run's ε", and these rows pin why that
//! cost cannot be taken yet, at the thickness a user would ask for:
//! `topo::shell` moves every chart at once and refuses at a wall's
//! offset fit, or, where the fit certifies, past every seam, at a top
//! rim's at-rest certificate.
//!
//! A cap moved alone moves the cap's corners along the slanted seams:
//! each corner is the moved plane's root along its seam, and each rim
//! the moved plane's section of its wall. The straight prism's seams
//! are parallel to the cap normal, so its moved rim lands on an interior
//! row of each wall.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::test_support::upper;
use geom_core::{Dual64, Tol, Vec3};
use topo::construction::Construct;
use topo::{Body, EdgeKey, FaceKey, ReplaceFaceError, ShellError};

use crate::common::approx::{nurbs_walls, prism, twisted_loft};
use sweep::test_support::finished;

/// The wall thickness these rows shell at, in metres: 2.5% of the
/// 2 m section, a thickness a user would ask for.
const THICKNESS: f64 = 0.05;

fn is_spline_wall(walls: &[(FaceKey, impl Sized)], face: FaceKey) -> bool {
    walls.iter().any(|(k, _)| *k == face)
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

/// **The shelled twisted loft's wall–wall seams derive.** Every chart
/// moved inward by `THICKNESS` at once, as `topo::shell` moves them, read
/// per edge at its plan: each seam between two walls is the NURBS ×
/// NURBS section of their two moved fits, selected through
/// `Section::branch_at` at the old seam's middle, and plans. Where ε is
/// tighter than the twisted wall's fit reaches, no fit mints and the row
/// above is the pin.
#[test]
fn the_shelled_lofts_wall_seams_derive_where_its_fits_certify() {
    let eps = Tol::witness().eps();
    if eps < 1e-8 {
        return;
    }
    let body = twisted_loft(0.3);
    let faces: Vec<[FaceKey; 1]> = body.faces().map(|(k, _)| [k]).collect();
    let moves: Vec<(&[FaceKey], f64)> = faces
        .iter()
        .map(|f| {
            let sense = body.get_face(f[0]).expect("a live face").sense;
            (&f[..], if sense { -THICKNESS } else { THICKNESS })
        })
        .collect();
    let walls = nurbs_walls(&body);
    let mut seams = 0;
    for (edge, plan) in topo::offset_charts_edge_plans_for_tests(&body, &moves, Tol::witness()) {
        let halves = body.get_edge(edge).expect("the edge resolves");
        let sides: Vec<FaceKey> = [halves.he_plus, halves.he_minus]
            .into_iter()
            .filter_map(|he| body.face_of_half_edge(he))
            .collect();
        if sides.len() == 2 && sides.iter().all(|k| is_spline_wall(&walls, *k)) {
            seams += 1;
            assert!(
                matches!(plan, Ok(None)),
                "eps {eps:e}: the seam {edge:?} between two moved fits derives, got {plan:?}"
            );
        }
    }
    assert_eq!(seams, 4, "the twisted loft has four wall–wall seams");
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

/// The curved loft's shell moves every chart at once, so it mints every
/// wall's offset fit before any edge. Where ε is tighter than the fit
/// reaches it refuses at a wall's fit. Where the fit certifies, every
/// wall–wall seam derives as the NURBS × NURBS section of its two moved
/// fits, and the shell refuses at a top rim's at-rest certificate: the
/// plane × fit lane's chart image of the rim is the degree-1 interpolant
/// of its fixed foot schedule, and on this twisted wall limb 2's value
/// at a break is about 1e-6 m: in the band at 1e-6, where it escalates,
/// and past it at 1e-9, where it refuses outright
/// (`work/isotail/a-moved-lofts-top-rim-refuses-its-at-rest-certificate.md`).
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
    // Which door answers first depends on ε: a wall's fit reaches
    // 1.39486e-10 m on a 62 x 35 grid, the last within the sample cap,
    // so below that the fit refuses at the cap, and above it the fit
    // certifies and the seams derive. (1.3958e-10 before the offset
    // regularity floor became the area quotient's inf, PR 4984: the
    // tangential term divides by that floor, and it rose.)
    let eps = Tol::witness().eps();
    if eps < 1.39486e-10 {
        assert!(
            is_spline_wall(&nurbs_walls(&body), *face),
            "the refusing face is not a wall: {e}"
        );
        let ReplaceFaceError::Fit {
            error:
                geom_brep::OffsetFitError::WallReached {
                    wall: geom_brep::OffsetFitWall::SampleCap { .. },
                    achieved,
                    grid,
                    ..
                },
            ..
        } = error.as_ref()
        else {
            panic!("eps {eps:e}: expected the wall's fit to reach the sample cap, got {e}");
        };
        assert!(
            *grid == (62, 35) && (achieved - 1.39486e-10).abs() < 1e-14,
            "eps {eps:e}: the fit stopped at {achieved:e} on {grid:?}"
        );
    } else {
        // Limb 2's hull value at a break on the rim: 1.5994e-6 m at 1e-9,
        // past the band, so it refuses outright; 1.02e-6 m at 1e-6, in the
        // band, so it escalates. The rim's own foot reads 2.2e-16 m at
        // that break, so the miss is its pcurve's (`ImageMiss`). It moves when the rim's derivation does,
        // so the 1e-9 arm reads it within 5%.
        let ReplaceFaceError::Op {
            error: topo::EulerOpError::RechartFalsifies { edge, error, .. },
            ..
        } = error.as_ref()
        else {
            panic!("eps {eps:e}: expected a rim's at-rest certificate to refuse, got {error:?}");
        };
        if eps < 1e-7 {
            let geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                limb: geom_brep::ssi::SsiLimb::ImageMiss,
                margin,
            }) = error
            else {
                panic!("eps {eps:e}: expected limb 2's value to refuse outright, got {error:?}");
            };
            let value = margin.diagnostic_f64_for_error_text().value();
            assert!(
                value.is_some_and(|v| (v - 1.5994e-6).abs() <= 0.05 * 1.5994e-6),
                "eps {eps:e}: limb 2 reads {value:?}"
            );
        } else {
            assert!(
                matches!(
                    error,
                    geom_brep::CertifyError::Escalated {
                        check: geom_brep::CertCheck::PlaneNurbsHullValue,
                        ..
                    }
                ),
                "eps {eps:e}: expected limb 2 to escalate in the band, got {error:?}"
            );
        }
        // The refusing edge is a top rim: the top cap's edge with a wall.
        let halves = body.get_edge(*edge).expect("the refused edge resolves");
        let sides: Vec<FaceKey> = [halves.he_plus, halves.he_minus]
            .into_iter()
            .filter_map(|he| body.face_of_half_edge(he))
            .collect();
        let walls = nurbs_walls(&body);
        assert!(
            sides.contains(&top_cap(&body)) && sides.iter().any(|k| is_spline_wall(&walls, *k)),
            "eps {eps:e}: {edge:?} is not a top rim, its faces are {sides:?}"
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
        .map(|z| profile::SketchPlane::xy_at(geom_core::Point3::new(0.0, 0.0, *z)))
        .collect::<Vec<_>>();
    let body = sweep::loft_body::<Dual64>(
        &sweep::test_support::lift_sections::<Dual64>(&[square(), square()]),
        &sweep::test_support::lift_places::<Dual64>(&places, geom_core::Tol::witness()),
        1,
        Tol::witness(),
    )
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
        .map(|z| profile::SketchPlane::xy_at(geom_core::Point3::new(0.0, 0.0, *z)))
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

/// The shell moves every chart at once and mints every new surface
/// first, so it refuses at a wall's fit: the wall's net carries a C⁰
/// crease the fit's Taylor bound cannot cross. A cap moved alone meets
/// its rims instead, and moves: the skin's weights are constant along
/// the stacking, so the moved plane's section of each wall is the
/// wall's row at `v = |d|/2`, exact structure, and each rim is minted as
/// the moved cap and wall's `Intersection` on that row. The loft's
/// radius along a seam is `1 + 1.2·v(1 − v)` at height `2v`, so the
/// rims are circles of radius 1.02925 at `z = 0.05`. The moved cap is a
/// plane bounded by rational spline rims, and it measures: the certified
/// lane reads their vector area (`geom_brep::props::rational_rim`), and
/// its area and flux hold the disc's, while the structural door, which
/// has no quadrature lane, keeps the closed form's typed refusal. Tier 3
/// then refuses at a wall, whose trim rows are the cap's to rounding and
/// not its chart's exact rectangle.
#[test]
fn shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_moves_alone_onto_a_wall_row() {
    let body = vase();
    let walls = nurbs_walls(&body);
    let e = topo::shell(
        &finished("the vase", body.clone(), Tol::witness()),
        THICKNESS,
        Tol::witness(),
    )
    .expect_err("a spline-walled body does not shell today");
    let ShellError::Face { face, error } = &e else {
        panic!("expected a per-face offset refusal, got {e}");
    };
    assert!(
        is_spline_wall(&walls, *face),
        "the refusing face is not a wall: {e}"
    );
    assert!(
        matches!(
            error.as_ref(),
            ReplaceFaceError::Fit {
                error: geom_brep::OffsetFitError::PatchBound(
                    geom_brep::patch_bound::PatchBoundError::Crease
                ),
                ..
            }
        ),
        "expected the wall's fit to refuse at its crease, got {e}"
    );
    // The bottom cap, moved alone into the vase.
    let (cap, d) = body
        .faces()
        .find_map(|(k, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) if origin.z < 0.5 => {
                Some((k, THICKNESS * normal.z.signum()))
            }
            _ => None,
        })
        .expect("the vase has a bottom cap");
    let mut alone = body.clone();
    topo::replace_face_offset(&mut alone, cap, d, Tol::witness())
        .expect("the vase's cap moves alone");
    let z = -d;
    let cap_surface = alone.get_face(cap).expect("the moved cap resolves").surface;
    let Some(geom::Surface::Plane { origin, normal, .. }) = alone.get_surface(cap_surface) else {
        panic!("the moved cap is a plane");
    };
    assert!(
        (origin.z - z).abs() < 1e-12 && normal.z < -0.5,
        "the moved cap is the plane z = {z}, facing out of the body: {origin:?} {normal:?}"
    );
    let row = z / 2.0;
    let radius = 1.0 + 1.2 * row * (1.0 - row);
    let mut rims = 0;
    for (edge, data) in alone.edges() {
        let Some(curve) = alone
            .get_curve_geom(data.curve)
            .and_then(topo::CurveGeom::certified)
        else {
            panic!("{edge:?} carries a certified curve");
        };
        let geom_brep::EdgeDescription::Intersection { pair, .. } = curve.description() else {
            continue;
        };
        rims += 1;
        let wall = pair
            .other(cap_surface)
            .unwrap_or_else(|| panic!("the rim {edge:?} is not the moved cap's: {pair:?}"));
        assert!(
            walls
                .iter()
                .any(|(k, _)| alone.get_face(*k).is_some_and(|f| f.surface == wall)),
            "the rim {edge:?} meets a spline wall"
        );
        assert!(
            !curve.authority().is_declared(),
            "the rim {edge:?} is derived, not the sketch's record"
        );
        let (t0, t1) = curve.params();
        for t in [t0, 0.5 * (t0 + t1), t1] {
            let p = curve.carrier().eval(t);
            assert!(
                (p.z - z).abs() < 1e-12 && (p.x.hypot(p.y) - radius).abs() < 1e-12,
                "the rim {edge:?} is the circle of radius {radius} at z = {z}, at {p:?}"
            );
        }
    }
    assert_eq!(rims, 2, "each wall meets the moved cap along one rim");
    let rows = alone
        .pcurves()
        .filter(|(he, _)| {
            alone
                .face_of_half_edge(*he)
                .is_some_and(|f| is_spline_wall(&walls, f))
        })
        .filter(|(_, c)| {
            matches!(*c.pcurve(), geom_brep::Pcurve::IsoLine { p0, pl }
                if pl.y == 0.0 && (p0.y - row).abs() < 1e-9)
        })
        .count();
    assert_eq!(rows, 2, "each wall carries its rim on its row v = {row}");
    // The moved cap measures: a disc of the rims' radius in `z = |d|`,
    // facing `−z`, so its flux about the shell's reference `q` is
    // `(q_z − z)·πr²`. The rims hold the circle to 1e-12 at the points
    // above, so the area moves off the disc's by at most `2πr·1e-12`, and
    // the flux by that times the lever `|q_z − z|`, plus the area times
    // the 1e-12 its anchor, a rim point, sits off the plane `z`. The
    // reference is the cap's own first vertex now that it is the shell's
    // first quadrature face, so the lever is the rims' rounding.
    let [(_, reference)] = topo::test_support::shell_references(&alone)[..] else {
        panic!("the vase is one shell");
    };
    let reading = topo::test_support::face_reading(
        &alone,
        cap,
        Tol::witness(),
        topo::test_support::pinned_len(Tol::witness()),
    )
    .expect("a plane bounded by rational rims has a vector area");
    let disc = core::f64::consts::PI * radius * radius;
    let residual = 2.0 * core::f64::consts::PI * radius * 1e-12;
    let lever = (reference.z - z).abs();
    assert!(
        (reading.area - disc).abs() <= reading.area_pad + residual,
        "the moved cap's area {} ± {} misses the disc's {disc}",
        reading.area,
        reading.area_pad
    );
    let flux = (reference.z - z) * disc;
    assert!(
        (reading.flux - flux).abs() <= reading.flux_pad + lever * residual + disc * 1e-12,
        "the moved cap's flux {} ± {} misses {flux}",
        reading.flux,
        reading.flux_pad
    );
    // And the brackets are pinned, so the containment above is not
    // vacuous. The target is the volume's, which the cap's flux meets at
    // the first round: its lever about the reference is the rims'
    // rounding. Its area is read at that round's pieces: measured
    // 1.3265e-6 at every ε (1.4508e-6 with each span read as one power
    // series), and a rule with half the pieces per round reads about 32×
    // that (the `h⁵` remainder).
    assert_eq!(
        reading.met_at,
        Some(0),
        "the cap meets the volume target at its first round"
    );
    assert!(
        reading.area_pad <= 1.5e-6,
        "the moved cap's area bracket widened to ± {}",
        reading.area_pad
    );
    // Tier 3 now passes the cap and refuses at a wall: the moved cap's
    // rims are the walls' rows to rounding, not exactly, and a NURBS
    // wall's patch lane reads only an exact rectangle of its chart
    // (`work/shell/a-wall-trimmed-to-an-inexact-iso-row-has-no-rectangle.md`).
    let refusals = topo::validate_geometric(&alone, Tol::witness())
        .expect_err("a wall trimmed off its exact rectangle has no volume");
    assert!(
        matches!(
            refusals.as_slice(),
            [topo::ValidationError::VolumeUncomputable {
                source: topo::MassPropsError::Face {
                    face,
                    source: geom_brep::PropsError::QuadratureUnsupported { .. },
                    ..
                },
                ..
            }] if is_spline_wall(&walls, *face)
        ),
        "expected only a wall's volume refusal, got {refusals:?}"
    );
    // The structural door has no quadrature lane: the moved cap keeps
    // the closed form's typed refusal there.
    let structural = topo::test_support::face_reading_structural(
        &alone,
        cap,
        Tol::witness(),
        topo::test_support::pinned_len(Tol::witness()),
    )
    .expect_err("the structural door has no rational rule");
    assert!(
        matches!(
            structural,
            topo::MassPropsError::Face {
                face,
                source: geom_brep::PropsError::Unimplemented,
                ..
            } if face == cap
        ),
        "expected the moved cap's typed refusal, got {structural:?}"
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
    let places = [profile::SketchPlane::xy(), crate::common::plane_of(&tilt)];
    sweep::loft_body::<f64>(&[square(), square()], &places, 1, Tol::witness())
        .expect("the tilted-top prism lofts")
        .body
}

/// The march arm builds nothing yet: the moved tilted cap's section of
/// each polynomial wall is certified by the march, and the plane ×
/// NURBS edge certificate's limb 2 reads its composite a few
/// micrometres at a break where the marched branch's own foot is on the
/// wall (its foot reads 8.3e-17 and 2.8e-17 m at the two refusing
/// breaks at 1e-9): the pcurve's own miss (`SsiLimb::ImageMiss`), the
/// branch lying on both surfaces to ~5e-11
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
                limb: geom_brep::ssi::SsiLimb::ImageMiss,
                margin,
            }) => {
                let value = upper(*margin);
                assert!(
                    micrometres.contains(&value) && value > 10.0 * eps,
                    "eps {eps:e}, d {d}: limb 2 read {margin:e} at a break"
                );
            }
            // The door's escalation carries the check and the cause, not
            // the limb, and `ImageMiss` shares `HullValueChart`'s check and
            // predicate, so this arm pins the break's value as what
            // escalated; the limb is pinned where a refusal is definite.
            geom_brep::CertifyError::Escalated { check, cause, .. } => {
                assert_eq!(*check, geom_brep::ssi::SsiLimb::ImageMiss.check());
                assert_eq!(
                    cause.predicate,
                    Some("ssi_hull_value_chart"),
                    "eps {eps:e}, d {d}: the break's value escalated, not the bound"
                );
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
        .map(|z| profile::SketchPlane::xy_at(geom_core::Point3::new(0.0, 0.0, *z)))
        .collect::<Vec<_>>();
    let loft = |turn: f64| {
        sweep::loft_body::<Interval>(
            &sweep::test_support::lift_sections::<Interval>(&[square(0.0), square(turn)]),
            &sweep::test_support::lift_places::<Interval>(&places, geom_core::Tol::witness()),
            1,
            Tol::witness(),
        )
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
/// first and a spline carrier, and the body passes tier 3: its plane ×
/// NURBS certificate reads the cap as its fit, and its volume takes the
/// cap's bicubic trimmed region through the composite arm and holds the
/// moved box's.
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
                geom_brep::EdgeDescription::Intersection { pair, .. }
                    if pair == geom_brep::SurfacePair::new(new_side, cap_key)
            )
        })
        .collect();
    assert_eq!(
        sections.len(),
        1,
        "one edge is the moved side's section of the fitted cap"
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
    // section through the plane × NURBS lane over the fit, and takes the
    // volume over the fitted cap's trimmed region: a bicubic chart, past
    // the exact outer rule's window, so the composite arm's.
    let at_rest = topo::AtRestBody::validate(moved, Tol::witness())
        .unwrap_or_else(|e| panic!("the moved box passes tier 3: {e:?}"));
    let props = topo::mass_properties(
        &at_rest,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("the moved box has a volume");
    // The closed form is the box `(2 − T) × 2 × 1`, its cap a fit of the
    // plane `z = 1` within ε over an area under 4 m².
    let truth = (2.0 - THICKNESS) * 2.0;
    let slack = props.enclosure.volume_pad + 4.0 * Tol::witness().eps();
    assert!(
        (props.enclosure.volume - truth).abs() <= slack,
        "the moved box's volume {} ± {} misses {truth}",
        props.enclosure.volume,
        props.enclosure.volume_pad
    );
    // And the bracket is tight: the cap floors at its first round, so
    // a pad that grew (a looser rule, a lost round) would make the
    // containment above vacuous. Measured 3.7e-14 at the default ε.
    assert!(
        props.enclosure.volume_pad <= 1e-12,
        "the moved box's volume bracket widened to ± {}",
        props.enclosure.volume_pad
    );
}

/// **A moved plane's section with a held fit follows the plane.** The
/// box with the fitted cap has a side moved once, giving the side's edge
/// with the cap as the pair `{moved side, cap}`; that edge is re-stated
/// through the certified door (the same locus, naming the surfaces in
/// the other order, which the pair does not record), and the side moved
/// again. The re-derived edge is the section of the newly moved plane and
/// the held fit.
#[test]
fn a_moved_planes_section_with_a_held_fit_follows_the_plane() {
    use crate::common::approx::box_with_approx_cap;
    use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, SurfacePair};
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
                            EdgeDescription::Intersection { pair, .. }
                                if pair == geom_brep::SurfacePair::new(s1, s2)
                        )
                    })
            })
            .map(|(k, _)| k)
    };
    let step = -0.5 * THICKNESS;
    topo::replace_face_offset(&mut body, side, step, Tol::witness())
        .expect("the side moves against the fitted cap");
    let once = body.get_face(side).expect("the side survives").surface;
    let edge = section_of(&body, once, cap_key).expect("the side's section");
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .expect("the section is certified");
    let (t0, t1) = curve.params();
    let swapped = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(cap_key, once),
            selector: geom_brep::CurveSelector::Witness(curve.carrier().eval(0.5 * (t0 + t1))),
        },
        carrier: curve.carrier().clone(),
        param_start: t0,
        param_end: t1,
    };
    body.set_edge_curve(edge, swapped, Tol::witness())
        .expect("the re-statement certifies");
    assert_eq!(
        section_of(&body, cap_key, once),
        Some(edge),
        "the re-stated edge is the same pair"
    );
    topo::replace_face_offset(&mut body, side, step, Tol::witness()).expect("the side moves again");
    let twice = body.get_face(side).expect("the side survives").surface;
    assert_eq!(
        section_of(&body, twice, cap_key),
        Some(edge),
        "the re-derived section is the moved plane's with the held fit"
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
            topo::construction::FaceSurface::New {
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
                    geom_brep::EdgeDescription::Intersection { pair, .. }
                        if pair.other(cap_key).is_some_and(|side| matches!(
                            body.get_surface(side),
                            Some(geom::Surface::Plane { .. })
                        ))
                ) && matches!(c.carrier(), geom::Curve3::Nurbs(_))
            })
            .count();
        assert_eq!(
            sections, 4,
            "d = {d}: each cap edge is a plane side's section of the fit"
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
/// `±0.05`. Where ε is looser than the offset fit reaches (4.382e-11
/// m within the sample cap) and the moved rims certify (not at 1e-9)
/// the move builds: its corners are the
/// side planes' roots along the plane × fit sections, tier 3 refuses the
/// cap's volume alone
/// (`work/quadtail/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image.md`),
/// and the moved body maps rigidly. Tighter, the fit refuses first; at
/// 1e-9 the moved rim refuses at limb 2's value at a break where its own
/// foot reads 1.4e-15 and 6.5e-16 m (`ImageMiss`).
#[test]
fn a_moved_curved_fitted_cap_builds_where_its_fit_certifies() {
    // Pinned red since PR 4551: at ε = 1e-12 the cap's fit stops on (45, 48),
    // not the pinned (45, 45)
    // (`work/shell/a-curved-fitted-cap-rows-sample-cap-grid-pin-is-red-at-1e-12-on-main`).
    test_utils::pinned_red::row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "the moved curved fitted cap",
        &["eps 1e-12, d = 0.05: the cap's fit stopped at 4.38189780844083e-11 on (45, 48)"],
        a_moved_curved_fitted_cap_builds_where_its_fit_certifies_row,
    );
}

/// The row [`a_moved_curved_fitted_cap_builds_where_its_fit_certifies`] runs, under its pin.
fn a_moved_curved_fitted_cap_builds_where_its_fit_certifies_row() {
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
            topo::construction::FaceSurface::New {
                surface: geom::Surface::Nurbs(Arc::new(patch.clone())),
                sense: true,
            },
        )
        .expect("the cap takes a NURBS surface");
        let moved = topo::replace_face_offset(&mut body, cap, d, Tol::witness());
        // The cap's fit reaches 4.382e-11 m on a 45 x 45 grid, the last
        // within the sample cap. Above that it certifies, and at 1e-9 the
        // moved cap's rim then refuses at its plane × fit limb 2
        // (6.75e-7 m, above the band).
        if eps < 4.382e-11 {
            match &moved {
                Err(ReplaceFaceError::Fit {
                    error:
                        geom_brep::OffsetFitError::WallReached {
                            wall: geom_brep::OffsetFitWall::SampleCap { .. },
                            achieved,
                            grid,
                            ..
                        },
                    ..
                }) => assert!(
                    *grid == (45, 45) && (achieved - 4.382e-11).abs() < 1e-14,
                    "eps {eps:e}, d = {d}: the cap's fit stopped at {achieved:e} on {grid:?}"
                ),
                other => panic!(
                    "eps {eps:e}, d = {d}: expected the cap's fit to reach the sample cap, got {other:?}"
                ),
            }
            continue;
        }
        if eps < 2e-9 {
            assert!(
                matches!(
                    moved,
                    Err(ReplaceFaceError::Op {
                        error: topo::EulerOpError::RechartFalsifies {
                            error: geom_brep::CertifyError::PlaneNurbs(
                                geom_brep::PlaneNurbsRefusal::Limb {
                                    limb: geom_brep::ssi::SsiLimb::ImageMiss,
                                    ..
                                }
                            ),
                            ..
                        },
                        ..
                    })
                ),
                "eps {eps:e}, d = {d}: expected the moved cap's rim to refuse its limb 2, got {moved:?}"
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
        topo::transform_rigid(
            &body,
            &geom_core::test_support::rigid(&rigid, Tol::witness()),
            Tol::witness(),
        )
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
/// `Intersection` on the tilted plane and the moved cap.
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
            topo::construction::FaceSurface::New {
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
                EdgeDescriptionSpec::Intersection { pair, .. }
                    if pair == geom_brep::SurfacePair::new(tilted, cap_key)
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
