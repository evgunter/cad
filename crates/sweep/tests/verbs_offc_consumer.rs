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

use crate::common;
use common::approx::{
    approx_walls, band, box_with_approx_cap, moved_box, prism, unit_box,
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
    let (a, face) = box_with_approx_cap(0.05, 1e-9);

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


// ---------------------------------------------------------------------
// The rigid map of an Approx-faced body
// ---------------------------------------------------------------------

/// A rotation about ẑ composed with a translation — `det = +1`, the
/// kernel's rigid contract, and general enough that no coordinate of
/// the moved body is the coordinate it started with.
fn rigid() -> Affine3<f64> {
    let mut map = Affine3::rotation_about_axis(
        geom_core::Point3::origin(),
        Vec3::unit_z(),
        core::f64::consts::FRAC_PI_3,
    );
    map.translation = map.translation + Vec3::new(0.3, -0.2, 1.1);
    map
}

/// The approximating surface `face` wears.
fn approx_face_surface(body: &Body<f64>, face: FaceKey) -> Arc<geom::ApproxSurface<f64>> {
    match body.get_surface(body.get_face(face).unwrap().surface) {
        Some(Surface::Approx(a)) => Arc::clone(a),
        other => panic!("{face:?} must wear an approximating surface, got {other:?}"),
    }
}

/// Two nets are the same net, bit for bit — control points, weights and
/// both knot vectors. `Point3` carries no `PartialEq`, so the
/// coordinates are compared one by one.
fn same_net(a: &NurbsSurface<f64>, b: &NurbsSurface<f64>, what: &str) {
    assert_eq!(a.weights(), b.weights(), "{what}: weights are invariants");
    assert_eq!(a.knots_u().knots(), b.knots_u().knots(), "{what}: knots u");
    assert_eq!(a.knots_v().knots(), b.knots_v().knots(), "{what}: knots v");
    assert_eq!(a.control().len(), b.control().len(), "{what}: net size");
    for (i, (p, q)) in a.control().iter().zip(b.control()).enumerate() {
        assert!(
            p.x == q.x && p.y == q.y && p.z == q.z,
            "{what}: control point {i} is {p:?}, not the mapped {q:?}"
        );
    }
}

/// **The end-to-end row, both signs of `d`: an `Approx`-faced body
/// moves.** What the map produces is the mapped base description, the
/// mapped fit, the same `d`, window and tolerance, and a certificate
/// re-derived on the mapped pair — the stored one is never carried.
///
/// The nets are compared BIT for bit against the same affine map
/// applied here: nothing in the mapping is an approximation, so a
/// tolerance would be the wrong instrument. The certificate's two limbs
/// are compared to 1e-9 instead, because they are measured on the
/// mapped geometry and a residual is a DISTANCE that a rigid map
/// preserves only up to rounding.
///
/// **Tier 3 is read as a difference, not as a verdict.** The fixture's
/// cap edges carry descriptions naming the plane the cap had before the
/// replacement (see [`box_with_approx_cap`] for the two gates that
/// force that), so the body reports `DescriptionNotAdjacent` there
/// whatever a map does. The claim a map can make is that it introduces
/// no finding — in particular no `ApproxCertification`, which is tier
/// 3's own independent re-derivation of the mapped certificate against
/// the mapped description, run per call and never off the stored copy.
#[test]
fn an_approx_faced_body_moves_under_a_rigid_map() {
    let map = rigid();
    for d in [0.05_f64, -0.05] {
        let (body, face) = box_with_approx_cap(d, 1e-9);
        let moved = topo::transform_rigid(&body, &map, Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}: an Approx-faced body must move: {e}"));

        let before = approx_face_surface(&body, face);
        let after = approx_face_surface(&moved, face);
        let geom::SurfaceDescription::Offset { base: b0, d: d0 } = before.description();
        let geom::SurfaceDescription::Offset { base: b1, d: d1 } = after.description();
        assert_eq!(d1, d0, "d = {d}: the offset distance is a rigid invariant");
        assert_eq!(
            after.tolerance(),
            before.tolerance(),
            "d = {d}: the tolerance is the claim, carried not re-chosen"
        );
        assert_eq!(
            after.window(),
            before.window(),
            "d = {d}: the window is a parameter-space fact"
        );
        same_net(b1, &b0.map_affine(&map), "the description's base");
        same_net(after.fit(), &before.fit().map_affine(&map), "the fit");

        let (c0, c1) = (before.certificate(), after.certificate());
        assert_eq!(
            c1.distance, c0.distance,
            "d = {d}: the certificate is about the same distance"
        );
        assert!(
            (c1.hull_sup - c0.hull_sup).abs() <= 1e-9
                && (c1.on_locus_max - c0.on_locus_max).abs() <= 1e-9,
            "d = {d}: the certificate's limbs must survive a rigid map — \
             hull_sup {} vs {}, on_locus_max {} vs {}",
            c1.hull_sup,
            c0.hull_sup,
            c1.on_locus_max,
            c0.on_locus_max
        );

        // The independent check: tier 3 re-derives the mapped
        // certificate itself, and finds nothing the operand did not
        // already have.
        let findings = |b: &Body<f64>| match topo::validate_geometric(b, Tol::witness()) {
            Ok(()) => Vec::new(),
            Err(e) => e.iter().map(|f| format!("{f:?}")).collect(),
        };
        assert_eq!(
            findings(&moved),
            findings(&body),
            "d = {d}: a rigid map must introduce no tier-3 finding"
        );
        assert!(
            !findings(&moved)
                .iter()
                .any(|f| f.contains("Approx") || f.contains("Certif")),
            "d = {d}: no finding about the mapped approximating surface: {:?}",
            findings(&moved)
        );
    }
}

/// **The props lane's verdict is the same either side of the map.**
///
/// The claim this row wanted to make is that volume and area are rigid
/// invariants, read as enclosures. It cannot be made on this body: the
/// quadrature refuses a spline face whose half-edges carry no stored
/// pcurve cache, and this fixture's `Approx` cap carries none — the iso
/// lane has no construction for its straight seam-class carriers (see
/// [`box_with_approx_cap`]). The body that DOES carry those caches is
/// the loft, which no rigid map can move at all while its wall seams
/// are `Curve3::Nurbs`.
///
/// So what is measured is the difference: the props lane answers the
/// image exactly as it answers the operand. A map that had disturbed
/// the geometry would not.
#[test]
fn the_props_lanes_verdict_is_the_same_either_side_of_the_map() {
    let (body, _) = box_with_approx_cap(0.05, 1e-9);
    let moved = topo::transform_rigid(&body, &rigid(), Tol::witness()).expect("the body moves");
    let read = |b: &Body<f64>| match topo::mass_properties(b, Tol::witness()) {
        Ok(m) => Ok((m.volume, m.surface_area, m.volume_pad, m.area_pad)),
        Err(e) => Err(format!("{e}")),
    };
    match (read(&body), read(&moved)) {
        (Ok((v0, a0, vp0, ap0)), Ok((v1, a1, vp1, ap1))) => {
            // The invariance row, if the lane ever answers here: the
            // two brackets must overlap, plus the map's own rounding.
            let dust = 1e-12;
            assert!(
                (v1 - v0).abs() <= vp0 + vp1 + dust,
                "volume is a rigid invariant: {v1} vs {v0} (pads {vp0} and {vp1})"
            );
            assert!(
                (a1 - a0).abs() <= ap0 + ap1 + dust,
                "area is a rigid invariant: {a1} vs {a0} (pads {ap0} and {ap1})"
            );
        }
        (Err(here), Err(there)) => assert_eq!(
            here, there,
            "the map changed what the props lane can compute"
        ),
        (here, there) => panic!("the map changed the props lane's verdict: {here:?} vs {there:?}"),
    }
}

/// **Composition, read off the body.**
///
/// The two-sided form — `offset(transform(b))` against
/// `transform(offset(b))` — is not constructible, and the reason is
/// structural. The face-level offset door (`topo::replace_face_offset`)
/// mints an `Approx` from a `Surface::Nurbs` operand, and this pass
/// refuses that kind, so `transform(b)` does not exist for any `b` the
/// door could offset.
///
/// What is pinned instead is the law at the surface, read off the moved
/// body: the mapped face's description names a mapped base, and a FRESH
/// fit of that base's offset — the offset door run on the mapped
/// description alone, with nothing of the original body in it —
/// certifies at the same tolerance to the same bound. That is
/// `M(S + d·n)` and `M(S) + d·n_M` meeting at the body's own face.
#[test]
fn the_mapped_face_is_a_certified_fit_of_the_mapped_description() {
    for d in [0.05_f64, -0.05] {
        let (body, face) = box_with_approx_cap(d, 1e-9);
        let moved = topo::transform_rigid(&body, &rigid(), Tol::witness()).expect("the body moves");
        let after = approx_face_surface(&moved, face);
        let geom::SurfaceDescription::Offset { base, .. } = after.description();
        let fresh = geom_brep::approx_offset_surface(Arc::clone(base), d, after.tolerance(), band())
            .unwrap_or_else(|e| panic!("d = {d}: the mapped description must still fit: {e}"));
        let Surface::Approx(fresh) = &fresh else {
            panic!("the door mints the variant")
        };
        let (a, b) = (after.certificate(), fresh.certificate());
        assert!(
            (a.hull_sup - b.hull_sup).abs() <= 1e-9
                && (a.on_locus_max - b.on_locus_max).abs() <= 1e-9,
            "d = {d}: the mapped fit and a fresh fit of the mapped description must certify \
             to the same bound — hull_sup {} vs {}, on_locus_max {} vs {}",
            a.hull_sup,
            b.hull_sup,
            a.on_locus_max,
            b.on_locus_max
        );
    }
}

/// **The gate order is part of the contract**: a non-rigid map refuses
/// `NotRigid` FIRST, on an `Approx`-faced body too. The `Approx` arm
/// rests on a statement about isometries, so it must never be the arm
/// that reports a scale or a reflection.
#[test]
fn a_non_rigid_map_refuses_not_rigid_first_on_an_approx_body() {
    let (body, _) = box_with_approx_cap(0.05, 1e-9);
    let scale = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::new(0.0, 0.0, 3.0),
        ),
        Vec3::zero(),
    );
    let mirror = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::zero(),
    );
    for (name, m) in [("scale", scale), ("mirror", mirror)] {
        match topo::transform_rigid(&body, &m, Tol::witness()) {
            Err(topo::TransformError::NotRigid { .. }) => {}
            other => panic!("{name} on an Approx-faced body: expected NotRigid, got {other:?}"),
        }
    }
}

/// **A degraded fit does not survive the map.** The same planted
/// corruption [`a_degraded_fit_on_a_face_goes_red_at_tier_three`] uses —
/// a fit edited after certification, carrying an honest-looking
/// certificate — is refused by the transform door itself, typed
/// `ApproxRecertify` with the fit door's own limb verbatim. The map
/// mints no certificate it did not derive, so a mapped degradation
/// cannot ship behind a fresh-looking one.
#[test]
fn a_degraded_fit_does_not_survive_the_map() {
    let d = 0.05;
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let good = approx_face_surface(&body, face);
    let geom::SurfaceDescription::Offset { base, .. } = good.description();
    let base = Arc::clone(base);
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

    let e = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect_err("a degraded fit must not move");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxRecertify {
                source: geom_brep::OffsetFitError::Limb { .. }
            }
        ),
        "expected the fit door's limb refusal under ApproxRecertify, got {e}"
    );
}

/// **The map does not change what the pcurve pass can reach.**
///
/// `the_approx_face_mints_its_iso_caches`'s claim cannot be asserted on
/// a mapped body, and the obstruction is not this door's: the only
/// bodies whose `Approx` faces carry stored caches are lofts, whose
/// wall seams are `Curve3::Nurbs` carriers this pass refuses, while the
/// body it CAN move has line carriers, for which the iso lane's seam
/// class has no construction. So what is measurable is that the pass's
/// verdict on the `Approx` chart is the same either side of the map —
/// the map neither wins nor loses reach — which is asserted here by
/// running the pass on both.
#[test]
fn the_pcurve_pass_reaches_the_same_charts_either_side_of_the_map() {
    let (mut body, _) = box_with_approx_cap(0.05, 1e-9);
    let mut moved = topo::transform_rigid(&body, &rigid(), Tol::witness()).expect("the body moves");
    let here = topo::mint_pcurves(&mut body, Tol::witness()).map_err(|e| format!("{e}"));
    let there = topo::mint_pcurves(&mut moved, Tol::witness()).map_err(|e| format!("{e}"));
    assert_eq!(
        here.is_ok(),
        there.is_ok(),
        "the map changed the pcurve pass's reach: {here:?} vs {there:?}"
    );
}

/// **Every other scalar refuses typed, naming its lane.** The offset fit
/// is derived at `f64` only, so an `Approx` face at any other scalar has
/// no certificate to re-derive — and the stored one is a claim about a
/// different geometry. The refusal is `ApproxLaneUnsupported`, and it
/// names the lane so a reader knows which capability is missing rather
/// than which kind is unwelcome.
///
/// The surface is an `f64`-certified one read at `Interval`
/// (`ApproxSurface::map_scalar` — a structural lift, not a second
/// certification door), which is exactly how such a face becomes
/// reachable at a scalar with no fit lane.
///
/// The refusal happens in the surface pass, before any curve is touched.
/// `transform_rigid` takes `&Body<T>` and works on a CLONE, so what
/// there is to pin is that the clone is dropped rather than returned: no
/// body comes back at all.
#[cfg(feature = "interval")]
#[test]
fn an_approx_face_refuses_typed_at_a_scalar_with_no_fit_lane() {
    use common::approx::{planar_patch, pulled_back};
    use geom_core::{Bounds, Interval, Real};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};

    let approx_f64 = geom_brep::approx_offset_surface(
        Arc::new(pulled_back(&planar_patch(1.0), 0.05)),
        0.05,
        1e-9,
        band(),
    )
    .expect("a planar patch's offset fits");
    let Surface::Approx(a) = &approx_f64 else {
        panic!("the door mints the variant")
    };
    let lifted = a.map_scalar(Interval::from_f64);

    let iv = Interval::from_f64;
    let v = |x: f64, y: f64| ProfileVertex::new(geom_core::Point2::new(iv(x), iv(y)), iv(0.0));
    let lp = ProfileLoop::new(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)]);
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a square is a valid profile");
    let mut body = sweep::extrude(&profile, sweep::Extrusion::Distance(iv(1.0)), Tol::witness())
        .expect("a square prism extrudes at Interval")
        .body;
    let face = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                // The outward cap: `Interval` orders no better than its
                // bracket does, so the choice reads the enclosure's low
                // end — definite on these exact dyadic coordinates.
                Some(Surface::Plane { origin, normal, .. })
                    if Bounds::lo(normal.z) > 0.5 && Bounds::lo(origin.z) > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the extruded box has a top cap");
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(lifted))))
        .expect("the attach-layer door accepts a live face");

    let e = topo::transform_rigid(
        &body,
        &Affine3::translation(geom_core::Vec3::new(iv(1.0), iv(0.0), iv(0.0))),
        Tol::witness(),
    )
    .expect_err("a scalar with no fit lane cannot move an Approx face");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxLaneUnsupported { lane: "interval" }
        ),
        "expected ApproxLaneUnsupported naming the interval lane, got {e}"
    );
}
