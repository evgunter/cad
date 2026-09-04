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
use common::approx::{approx_walls, band, box_with_approx_cap, moved_box, prism, unit_box};

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

/// Re-mint an `ApproxSurface` from parts behind a certifier that just
/// hands back `cert` — the planted-claim primitive every corruption row
/// below uses. It is the only way to build a surface whose stored
/// certificate is not a certificate of its stored pair, which is
/// exactly the shape the never-trust posture exists for.
fn plant(
    description: geom::SurfaceDescription<f64>,
    fit: NurbsSurface<f64>,
    window: geom::ApproxWindow,
    tolerance: f64,
    cert: geom::OffsetCertificate,
) -> geom::ApproxSurface<f64> {
    geom::ApproxSurface::certify(
        geom::SurfaceSpec {
            description,
            fit,
            window,
            tolerance,
        },
        |_, _, _, _| Ok::<_, geom_brep::OffsetFitError>(cert),
    )
    .unwrap()
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
        same_net(
            b1,
            &b0.map_points(|p| map.transform_point(p)),
            "the description's base",
        );
        same_net(
            after.fit(),
            &before.fit().map_points(|p| map.transform_point(p)),
            "the fit",
        );

        let (c0, c1) = (before.certificate(), after.certificate());
        assert_eq!(
            c1.distance, c0.distance,
            "d = {d}: the certificate is about the same distance"
        );
        // The SAMPLED limb is a distance and survives the map. The
        // certified BOUND does not — see
        // `the_hull_bound_is_frame_dependent_and_the_sampled_residual_is_not`,
        // which measures it — so what is asserted of `hull_sup` is the
        // claim it makes, not its value.
        assert!(
            (c1.on_locus_max - c0.on_locus_max).abs() <= 1e-12,
            "d = {d}: the sampled residual is a distance: {} vs {}",
            c1.on_locus_max,
            c0.on_locus_max
        );
        assert!(
            c1.hull_sup <= after.tolerance(),
            "d = {d}: the mapped surface must honour the tolerance it stores: {} > {}",
            c1.hull_sup,
            after.tolerance()
        );

        // The independent check: tier 3 re-derives the mapped
        // certificate itself, and finds nothing the operand did not
        // already have. The operand's own baseline is exactly one
        // finding — check 7 wanting the pcurve caches the seam class
        // will not mint (see `box_with_approx_cap`) — so this compares
        // a set of one against a set of one, and names what is in it.
        let findings = |b: &Body<f64>| match topo::validate_geometric(b, Tol::witness()) {
            Ok(()) => Vec::new(),
            Err(e) => e.iter().map(|f| format!("{f:?}")).collect(),
        };
        let (here, there) = (findings(&body), findings(&moved));
        assert_eq!(
            there, here,
            "d = {d}: a rigid map must introduce no tier-3 finding"
        );
        assert!(
            !there
                .iter()
                .any(|f| f.contains("Approx") || f.contains("Certif")),
            "d = {d}: no finding about the mapped approximating surface: {there:?}"
        );
        assert!(
            there.iter().all(|f| f.contains("VolumeUncomputable")),
            "d = {d}: the only wall is check 7 wanting caches: {there:?}"
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
        let fresh =
            geom_brep::approx_offset_surface(Arc::clone(base), d, after.tolerance(), band())
                .unwrap_or_else(|e| panic!("d = {d}: the mapped description must still fit: {e}"));
        let Surface::Approx(fresh) = &fresh else {
            panic!("the door mints the variant")
        };
        let (a, b) = (after.certificate(), fresh.certificate());
        // Same frame, same base, same `d`: here the two runs really do
        // measure the same quantities, so both limbs are compared —
        // and tightly, at a thousandth of the tolerance rather than at
        // it.
        assert!(
            (a.hull_sup - b.hull_sup).abs() <= 1e-12
                && (a.on_locus_max - b.on_locus_max).abs() <= 1e-12,
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
    let planted = plant(
        geom::SurfaceDescription::Offset { base, d },
        coarsened,
        good.window(),
        good.tolerance(),
        *good.certificate(),
    );
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
///
/// **Decorative until issue record 1346 lifts the NURBS-carrier
/// refusal**: the loft's `Approx` walls already carry iso caches, so
/// the claim this row wants becomes assertable the day that body
/// moves.
#[test]
fn the_pcurve_pass_reaches_the_same_charts_either_side_of_the_map() {
    let (mut body, _) = box_with_approx_cap(0.05, 1e-9);
    let mut moved = topo::transform_rigid(&body, &rigid(), Tol::witness()).expect("the body moves");
    let here = topo::mint_pcurves(&mut body, Tol::witness()).map_err(|e| format!("{e}"));
    let there = topo::mint_pcurves(&mut moved, Tol::witness()).map_err(|e| format!("{e}"));
    // The TEXT, not the boolean: both sides refuse today, so comparing
    // `is_ok()` would compare `false` with `false` and pass on any two
    // refusals whatever.
    assert_eq!(
        there, here,
        "the map changed what the pcurve pass says about this chart"
    );
    let Err(text) = &here else {
        // The row is not wrong if the pass ever mints here — it is
        // finished. Say so rather than assert a refusal.
        return;
    };
    assert!(
        text.contains("seam"),
        "the fixture's cap refuses at the iso lane's seam class; got {text}"
    );
}

/// A gently bowed polynomial patch over `[0,1]²` — a base with real
/// curvature, so the certified hull bound is a real number rather than
/// f64 dust and a row comparing it says something.
fn bowed() -> NurbsSurface<f64> {
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            control.push(geom_core::Point3::new(
                u,
                v,
                0.15 * u * (1.0 - u) + 0.1 * v * v,
            ));
        }
    }
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
}

/// A box wearing a CURVED `Approx` cap, fitted at 1e-6. Its cap's
/// descriptions are not adjacent — the bowed base has nothing to do
/// with the box's straight rim — and that is deliberate: this fixture
/// exists for what the certificate does under a map, and a row that
/// reads a certificate needs a body that moves, not a body that
/// validates.
fn box_with_curved_approx_cap(d: f64) -> (Body<f64>, FaceKey, Surface<f64>) {
    let honest = geom_brep::approx_offset_surface(Arc::new(bowed()), d, 1e-6, band())
        .expect("the bowed base's offset fits at 1e-6");
    let mut body = unit_box();
    let face = common::approx::top_face(&body);
    body.set_face_surface(face, FaceSurface::New(honest.clone()))
        .expect("the attach-layer door accepts a live face");
    (body, face, honest)
}

/// **The hull bound is FRAME-DEPENDENT; the sampled residual is not.**
///
/// The claim this row replaces said "a residual is a distance, and a
/// rigid map preserves it" of both limbs. Only one of them is a
/// distance. `on_locus_max` is `max_i |S_fit(uᵢ,vᵢ) − (S + d·n)(uᵢ,vᵢ)|`
/// — two points, one gap, computed the same way in either frame, and
/// measured invariant to 1e-12 here. `hull_sup` is a certified BOUND
/// assembled from per-cell control-hull enclosures in the AMBIENT
/// frame: a rotation re-splits the same geometry across the coordinate
/// axes, the enclosures widen or narrow, and the bound moves with them.
///
/// The claim was also VACUOUS where it was made. Both limbs are
/// certified `<= tolerance` and non-negative, so on a fixture minted at
/// 1e-9 the assertion `|Δ| <= 1e-9` holds for any two certified limbs
/// whatever — a mapped bound five orders above its operand's would have
/// passed it. That is why this row is minted at 1e-6: the slack it
/// asserts is a thousandth of the tolerance rather than equal to it.
///
/// So the row asserts the split rather than the invariance: the sampled
/// limb survives, the bound is allowed to move and is only required to
/// stay under the tolerance the surface claims — and the movement is
/// asserted to be REAL on at least one map, so that a future change
/// making the bound frame-independent fails here loudly rather than
/// leaving a stale caveat behind.
#[test]
fn the_hull_bound_is_frame_dependent_and_the_sampled_residual_is_not() {
    let d = 0.02;
    let (_, _, honest) = box_with_curved_approx_cap(d);
    let Surface::Approx(a0) = &honest else {
        panic!("the door mints the variant")
    };
    let c0 = *a0.certificate();
    let oblique = {
        let mut m = Affine3::rotation_about_axis(
            geom_core::Point3::origin(),
            Vec3::new(0.3, -0.4, 0.8).normalize(),
            1.1,
        );
        m.translation = m.translation + Vec3::new(0.3, -0.2, 1.1);
        m
    };
    let maps: [(&str, Affine3<f64>); 3] = [
        (
            "translation only",
            Affine3::translation(Vec3::new(0.3, -0.2, 1.1)),
        ),
        ("z by pi/3 + translation", rigid()),
        ("oblique axis by 1.1", oblique),
    ];
    let mut worst_hull = 0.0_f64;
    for (name, map) in maps {
        let (body, face, _) = box_with_curved_approx_cap(d);
        let moved = topo::transform_rigid(&body, &map, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: the curved-capped box must move: {e}"));
        let c1 = *approx_face_surface(&moved, face).certificate();
        assert!(
            (c1.on_locus_max - c0.on_locus_max).abs() <= 1e-12,
            "{name}: the sampled residual is a distance and must survive: {} vs {}",
            c1.on_locus_max,
            c0.on_locus_max
        );
        assert!(
            c1.hull_sup <= 1e-6,
            "{name}: the mapped surface still honours the tolerance it stores: {}",
            c1.hull_sup
        );
        worst_hull = worst_hull.max((c1.hull_sup - c0.hull_sup).abs());
    }
    assert!(
        worst_hull > 1e-9,
        "the hull bound moved by only {worst_hull:e} across every map — if it has become \
         frame-independent, this row and the caveats that cite it are the things to retire"
    );
}

/// **The window rule is one rule, and both doors enforce it.** A window
/// narrower than the base's chart rectangle is a bound the derivation
/// never proved, planted here behind an otherwise honest surface. It
/// must refuse at the validator (which re-derives per call) AND at the
/// map — the two doors read the same surface, so a disagreement between
/// them would be a body one door calls sound and the other refuses.
///
/// Both refusals go through `geom_brep::certify_offset_over`, the one
/// home; deleting the check there reds this row twice.
#[test]
fn a_narrowed_window_refuses_at_the_validator_and_at_the_map() {
    let d = 0.05;
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let good = approx_face_surface(&body, face);
    let w = good.window();
    let narrowed = geom::ApproxWindow {
        u: (w.u.0, (w.u.0 + w.u.1) * 0.5),
        v: w.v,
    };
    let planted = plant(
        good.description().clone(),
        good.fit().clone(),
        narrowed,
        good.tolerance(),
        *good.certificate(),
    );
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();

    // The validator, through production code (`PropsQuadLane`'s
    // re-derivation lane, which is what tier 3 calls per face).
    let findings = match topo::validate_geometric(&body, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e.iter().map(|f| format!("{f:?}")).collect(),
    };
    assert!(
        findings.iter().any(|f| f.contains("ApproxCertification")),
        "tier 3 must report the window it cannot honour on {face:?}: {findings:?}"
    );

    // The map.
    let e = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect_err("a narrowed window is a claim nothing proved");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxRecertify {
                source: geom_brep::OffsetFitError::WindowUnsupported { .. }
            }
        ),
        "expected ApproxRecertify(WindowUnsupported), got {e}"
    );
    // Door-neutral text: the refusal names the window, not whichever
    // door happened to raise it.
    let text = format!("{e}");
    assert!(
        !text.contains("approx_offset_surface"),
        "the window refusal must not name the storage door from the map: {text}"
    );
}

/// **The re-derivation classifies against the tolerance the mapped
/// surface will STORE.** An interior fit control point nudged by 1e-8
/// — a hundred thousand times smaller than a visible coarsening, and
/// away from the corners every on-locus sample hits — still refuses,
/// because the hull limb is a sup over the whole rectangle rather than
/// over the schedule.
///
/// The row is the guard on the classification tolerance: re-deriving at
/// anything looser than the stored 1e-9 (the measured excess is
/// 2.17e-9) lets this surface through while the mapped surface goes on
/// claiming 1e-9.
#[test]
fn a_micro_edit_of_an_interior_control_point_does_not_survive_the_map() {
    let d = 0.05;
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let good = approx_face_surface(&body, face);
    let fit = good.fit();
    let mut control = fit.control().to_vec();
    let mid = control.len() / 2;
    control[mid] = control[mid] + Vec3::new(0.0, 0.0, 1e-8);
    let nudged = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let planted = plant(
        good.description().clone(),
        nudged,
        good.window(),
        good.tolerance(),
        *good.certificate(),
    );
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();
    let e = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect_err("a 1e-8 edit is ten times the tolerance the surface claims");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxRecertify {
                source: geom_brep::OffsetFitError::Limb { .. }
            }
        ),
        "expected a limb refusal, got {e}"
    );
}

/// **The mapped certificate is the re-derivation's, field by field.**
/// A bogus certificate — every limb zero, the wrong distance, a made-up
/// cell count — planted behind a GOOD pair. The pair certifies, so the
/// map succeeds; what it must ship is what it measured, not what it was
/// handed. Every field is compared against an independent
/// `certify_offset` on the mapped pair, and `rounds` is the one that
/// differs: it is the fit's provenance and is carried, which this row
/// pins rather than assumes.
#[test]
fn a_planted_certificate_is_replaced_by_the_re_derivation_field_by_field() {
    let d = 0.05;
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let good = approx_face_surface(&body, face);
    let bogus = geom::OffsetCertificate {
        distance: 42.0,
        cells: 999,
        samples: 1,
        on_locus_max: 0.0,
        hull_sup: 0.0,
        normal_floor: 0.0,
        curvature_reach: 0.0,
        rounds: 7,
    };
    let planted = plant(
        good.description().clone(),
        good.fit().clone(),
        good.window(),
        good.tolerance(),
        bogus,
    );
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();
    let moved = topo::transform_rigid(&body, &rigid(), Tol::witness()).expect("a good pair moves");
    let after = approx_face_surface(&moved, face);
    let c = after.certificate();
    let geom::SurfaceDescription::Offset { base, .. } = after.description();
    let fresh = geom_brep::certify_offset(base, after.fit(), d, 1e-9, band())
        .expect("the mapped pair certifies independently");
    assert_eq!(
        c.distance, d,
        "the distance is re-derived, not the planted 42"
    );
    assert_eq!(c.cells, fresh.cells);
    assert_eq!(c.samples, fresh.samples);
    assert_eq!(c.normal_floor, fresh.normal_floor);
    assert_eq!(c.curvature_reach, fresh.curvature_reach);
    assert_eq!(
        c.hull_sup, fresh.hull_sup,
        "bit for bit the re-derivation's"
    );
    assert_eq!(c.on_locus_max, fresh.on_locus_max);
    assert_eq!(
        c.rounds, 7,
        "`rounds` is the FIT's provenance and is carried — see OffsetCertificate::rounds"
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
    let mut body = sweep::extrude(
        &profile,
        sweep::Extrusion::Distance(iv(1.0)),
        Tol::witness(),
    )
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

/// **What an `Approx`-capped part still cannot do, pinned so that
/// lifting any of it is loud.** A user who places one of these bodies
/// meets three walls after the map, and each is a different door's
/// gap rather than a property of the map:
///
/// - **mass properties** refuse, because the quadrature wants a stored
///   pcurve cache on every half-edge of a spline face and the iso
///   lane's seam class will not mint one over a straight carrier;
/// - **tessellation** refuses for the same missing caches;
/// - **STEP export** refuses by kind: the writer has no printer for an
///   approximating surface (`OFFSET_SURFACE` is the entity it would
///   need), so it declines rather than emitting the fit as if the fit
///   were the described geometry.
///
/// The row asserts each refusal and its shape. When one is built, this
/// reds, and `work/shell/no-approx-faced-body-is-both-movable-and-valid.md`
/// is the file to update.
#[test]
fn the_walls_a_placed_approx_capped_part_still_meets() {
    let (body, _) = box_with_approx_cap(0.05, 1e-9);
    let placed = topo::transform_rigid(&body, &rigid(), Tol::witness()).expect("the part places");

    let props = topo::mass_properties(&placed, Tol::witness())
        .expect_err("the quadrature wants caches this chart cannot mint");
    assert!(
        format!("{props}").contains("no stored pcurve cache"),
        "mass properties must refuse for the missing caches, got {props}"
    );

    let mesh = mesh::tessellate(&placed, 0.05, Tol::witness())
        .expect_err("tessellation wants the same caches");
    assert!(
        format!("{mesh}").contains("no stored pcurve cache"),
        "tessellation must refuse for the missing caches, got {mesh}"
    );

    let step = step_export::step_string(
        &placed,
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect_err("the STEP writer has no printer for the kind");
    assert!(
        matches!(
            step,
            step_export::StepExportError::UnsupportedSurface {
                kind: "approximating surface",
                ..
            }
        ),
        "expected the kind refusal, got {step}"
    );
}
