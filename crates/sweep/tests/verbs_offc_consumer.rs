//! **The body-reachable consumer**: a lofted prism's NURBS walls carry
//! certified `Surface::Approx` surfaces at rest, and tier 3 validates
//! the body end to end by RE-DERIVING each face's certificate per call.
//!
//! All four walls convert together, and that is forced rather than
//! chosen: a vertical edge is shared by two walls, and the iso lane's
//! seam class requires the carrier and the chart's boundary row to sit
//! in ONE spline space, so one converted wall beside three unconverted
//! ones has no consistent carrier. The caps stay planar.
//!
//! # What the consumer is, and what it deliberately is not
//!
//! No new public verb. The replacement runs through the attach layer's
//! existing doors in the M6-1 surgery order — surfaces
//! (`set_face_surface`), then descriptions, then `mint_pcurves` — which
//! is the smallest honest path that makes an `Approx` face reachable.
//! A named `replace_face_surface` verb would be OFF-D's face-
//! replacement primitive built early, and the shell/rim surgery it
//! implies is fenced out of this unit.
//!
//! # Why the base is the wall PULLED BACK by `d`
//!
//! A face's edges lie on its surface. Replacing a wall's surface with
//! the offset of that same wall would move the face `d` away from its
//! own boundary — geometrically incoherent, and every edge certificate
//! would (correctly) go red. So the description used here names the
//! base whose OFFSET is the wall: the wall's own control net pulled
//! back `d` along the chart normal. `Offset { base, d }` then
//! describes exactly the surface the face already had, the fit
//! reproduces it to the certified tolerance, and what is under test is
//! the storage, the delegation and the re-derivation rather than a
//! geometry change nothing else in the body knows about.
//!
//! That the wall is planar is what makes the pull-back exact: a rigid
//! translation of a planar net has the same chart normal everywhere,
//! so `base + d·n` is the wall, bit for bit in exact arithmetic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::{Affine3, Tol, Vec3};
use topo::{Body, FaceKey, FaceSurface};

mod common;
use common::approx::{
    approx_walls, band, moved_box, planar_patch, prism, pulled_back, top_face, unit_box,
};

/// The prism with all four walls carrying certified `Approx` surfaces
/// at the given signed distance.
fn prism_with_approx_walls(d: f64, tolerance: f64) -> (Body<f64>, Vec<FaceKey>) {
    let mut body = prism();
    let faces = approx_walls(&mut body, d, tolerance);
    (body, faces)
}

// ---------------------------------------------------------------------

/// **The end-to-end row, both signs of `d`.** The body carries an
/// `Approx` face and validates at tier 3 — which means the per-face
/// re-derivation ran and agreed, the face's edges still certify
/// against the new chart, and its pcurves re-minted through the spline
/// lane on the fit.
#[test]
fn an_approx_faced_body_validates_at_tier_three() {
    for d in [0.05_f64, -0.05] {
        let (body, faces) = prism_with_approx_walls(d, 1e-9);
        let face = faces[0];
        assert!(
            matches!(
                body.get_surface(body.get_face(face).unwrap().surface),
                Some(Surface::Approx(_))
            ),
            "d = {d}: the face carries the approximating surface"
        );
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "d = {d}: tier 3 on an Approx-faced body"
        );
    }
}

/// Every half-edge of the `Approx` face carries a STORED pcurve cache
/// at rest: the chart mints, and it mints through the spline lane —
/// the iso images of the fit, not a closed-form harmonic table.
#[test]
fn the_approx_face_mints_its_iso_caches() {
    let (body, faces) = prism_with_approx_walls(0.05, 1e-9);
    let face = faces[0];
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the wall's outer loop is a cycle");
    };
    let mut hes = 0usize;
    for he in body.loop_cycle(first).unwrap() {
        hes += 1;
        let cache = body
            .pcurve(he)
            .unwrap_or_else(|| panic!("half-edge {he:?} of an Approx face carries no cache"));
        assert!(
            matches!(cache.pcurve(), topo::Pcurve::IsoLine { .. }),
            "a planar wall's rims and seams are exact iso lines, got {:?}",
            cache.pcurve()
        );
    }
    assert_eq!(hes, 4, "the wall is a quadrilateral");
}

/// **The never-trust posture, at the body.** Degrade the stored fit
/// behind an otherwise-valid body — the surface keeps a certificate
/// that says it is fine — and tier 3 reports `ApproxCertification`
/// naming the face. The stored certificate is never consulted.
#[test]
fn a_degraded_fit_on_a_face_goes_red_at_tier_three() {
    let d = 0.05;
    let (mut body, faces) = prism_with_approx_walls(d, 1e-9);
    let face = faces[0];
    let Some(Surface::Approx(live)) = body.get_surface(body.get_face(face).unwrap().surface) else {
        panic!("the wall carries an approximating surface")
    };
    let geom::SurfaceDescription::Offset { base, .. } = live.description();
    let base = Arc::clone(base);
    let honest = geom_brep::approx_offset_surface(Arc::clone(&base), d, 1e-9, band()).unwrap();
    let Surface::Approx(good) = &honest else {
        panic!("the door mints the variant")
    };

    // Coarsen one control point of the FIT by a millimetre, and keep
    // the honest certificate: a planted claim.
    let fit = good.fit();
    let mut control = fit.control().to_vec();
    control[0] = control[0] + Vec3::new(0.0, 0.0, 1e-3);
    let coarsened = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let planted = geom::ApproxSurface::certify(
        geom::SurfaceSpec {
            description: geom::SurfaceDescription::Offset { base, d },
            fit: coarsened,
            window: good.window(),
            tolerance: good.tolerance(),
        },
        |_, _, _, _| Ok::<_, geom_brep::OffsetFitError>(*good.certificate()),
    )
    .unwrap();

    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();
    // The map re-mint may itself refuse on the moved chart; the claim
    // under test is tier 3's, so run it whatever the mint said.
    let _ = topo::mint_pcurves(&mut body, Tol::witness());
    let errors = topo::validate_geometric(&body, Tol::witness())
        .expect_err("a degraded fit must not validate");
    assert!(
        errors.iter().any(|e| matches!(
            e,
            topo::ValidationError::ApproxCertification { face: f, .. } if *f == face
        )),
        "tier 3 must report the re-derivation failure on face {face:?}, got {errors:?}"
    );
}

/// **The boolean gate refuses the operand pair-scoped**, by kind, and
/// does NOT admit the face on the authority of the `Nurbs` its fit is.
/// The refusal names the GERM PAIR — `(approx, plane)` — which is the
/// pair-scoped gate's own shape: a kind disqualifies an operation only
/// through a pair it could enter.
///
/// The body here is an extruded BOX, not the lofted prism: the gate's
/// edge rule runs first, and a loft's wall carriers are rung-3
/// splines, so a lofted operand refuses at the edge rule whatever its
/// faces carry. A box has `Line` carriers throughout, so the FACE rule
/// is the one that decides — which is the rule under test. The control
/// row is the same box unmodified, which unions cleanly.
#[test]
fn the_boolean_gate_refuses_an_approx_operand_by_kind() {
    // Control: two overlapping boxes union through the gate.
    topo::union(&unit_box(), &moved_box(), Tol::witness())
        .expect("two planar boxes union through the gate");

    // The same box with its top face carrying an approximating
    // surface — a planar patch over the face's own footprint,
    // described as the offset of that patch pulled back by `d`.
    let mut a = unit_box();
    let face = top_face(&a);
    let patch = planar_patch(1.0);
    let approx =
        geom_brep::approx_offset_surface(Arc::new(pulled_back(&patch, 0.05)), 0.05, 1e-9, band())
            .expect("a planar patch's offset fits");
    a.set_face_surface(face, FaceSurface::New(approx))
        .expect("the attach-layer door accepts a live face");

    let e = topo::union(&a, &moved_box(), Tol::witness())
        .expect_err("an Approx operand is unsupported-kind for the boolean gate");
    assert!(
        matches!(
            e,
            topo::BooleanError::CurvedPairUnsupported {
                kind: geom_brep::SurfaceKind::Approx,
                other_kind: geom_brep::SurfaceKind::Plane,
                face: f,
                ..
            } if f == face
        ),
        "expected the pair-scoped gate's germ-pair refusal naming SurfaceKind::Approx on \
         {face:?}, got {e}"
    );
}

/// **Tessellation delegates**: the `Approx` face meshes through the
/// spline lane on its fit, producing triangles like any described
/// NURBS wall.
#[test]
fn the_approx_face_tessellates_through_the_delegate_path() {
    let (body, faces) = prism_with_approx_walls(0.05, 1e-9);
    let face = faces[0];
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).expect("an Approx-faced body meshes");
    let patch = mesh
        .patches
        .iter()
        .find(|p| p.face == face)
        .expect("the Approx face has a patch");
    assert!(
        !patch.triangles.is_empty(),
        "the delegate path produced no triangles for the Approx face"
    );
}

/// **Transform refuses typed** on an `Approx` face: the composition law
/// holds (pinned in `geom-brep`'s suite), but this pass cannot
/// re-derive the mapped certificate, so it declines rather than carry
/// an unre-derived claim across a geometry change.
#[test]
fn transform_refuses_an_approx_face_typed() {
    let (body, _) = prism_with_approx_walls(0.05, 1e-9);
    let e = topo::transform_rigid(
        &body,
        &Affine3::translation(Vec3::new(1.0, 0.0, 0.0)),
        Tol::witness(),
    )
    .expect_err("the transform pass refuses the kind");
    assert!(
        matches!(e, topo::TransformError::ApproxSurface),
        "expected the typed Approx refusal, got {e}"
    );
}
