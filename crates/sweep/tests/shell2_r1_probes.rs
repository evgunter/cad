//! Reviewer probes for SHELL-2 (lane shell-2-r1) — the transform
//! door's `Approx` arm, the new `map_affine` door, and the end-to-end
//! consumer exercise. Nothing here is a lane deliverable; these rows
//! exist to falsify the PR's claims.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Point3, Tol, Vec3};
use topo::{Body, FaceKey, FaceSurface};

use crate::common;
use common::approx::{band, box_with_approx_cap, planar_patch, pulled_back, top_face, unit_box};

/// The rigid map the lane's own rows use: a rotation about ẑ plus a
/// translation.
fn rigid() -> Affine3<f64> {
    let mut map = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::unit_z(),
        core::f64::consts::FRAC_PI_3,
    );
    map.translation = map.translation + Vec3::new(0.3, -0.2, 1.1);
    map
}

fn approx_face_surface(body: &Body<f64>, face: FaceKey) -> Arc<geom::ApproxSurface<f64>> {
    match body.get_surface(body.get_face(face).unwrap().surface) {
        Some(Surface::Approx(a)) => Arc::clone(a),
        other => panic!("{face:?} must wear an approximating surface, got {other:?}"),
    }
}

// ---------------------------------------------------------------------
// C2 — `map_affine` is the map of the surface, rational weights and all
// ---------------------------------------------------------------------

/// A genuinely RATIONAL bi-quadratic patch: a quarter-circle sweep in
/// `u` (weights `1, 1/√2, 1`) extruded in `v`, so no weight row is
/// constant and the denominator is a real function of `u`.
fn rational_patch() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let w = core::f64::consts::FRAC_1_SQRT_2;
    // u-major: three u rows of two v columns.
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 1.0),
    ];
    let weights = vec![1.0, 1.0, w, w, 1.0, 1.0];
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

/// Deterministic pseudo-random parameters in `(0, 1)`.
fn params(n: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let mut s = 0x2545_F491_4F6C_DD1D_u64;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    for _ in 0..n {
        out.push((next().mul_add(0.98, 0.01), next().mul_add(0.98, 0.01)));
    }
    out
}

/// **C2.** For a rational surface, the mapped net evaluates to the
/// affine image of the original point at the SAME `(u, v)` — for a
/// rigid map and for a general (scaling, shearing) affine map, since
/// the door's docs claim the identity for every affine map.
#[test]
fn c2_map_affine_is_the_map_of_a_rational_surface() {
    let s = rational_patch();
    let shear = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(2.0, 0.3, 0.0),
            Vec3::new(0.0, 0.5, 0.1),
            Vec3::new(0.4, 0.0, 3.0),
        ),
        Vec3::new(-1.0, 2.0, 0.5),
    );
    for (name, map) in [("rigid", rigid()), ("shear", shear)] {
        let mapped = s.map_affine(&map);
        assert_eq!(mapped.weights(), s.weights(), "{name}: weights invariant");
        assert_eq!(
            mapped.knots_u().knots(),
            s.knots_u().knots(),
            "{name}: knots invariant"
        );
        for (u, v) in params(64) {
            let want = map.transform_point(s.ders(u, v).point);
            let got = mapped.ders(u, v).point;
            let e = (got - want).norm();
            assert!(
                e <= 1e-13,
                "{name}: S'({u},{v}) = {got:?} is not M(S(u,v)) = {want:?} (err {e:e})"
            );
        }
    }
}

/// **C2, normals.** Under a RIGID map the mapped net's unit chart
/// normal is the linear part applied to the original's.
#[test]
fn c2_map_affine_carries_normals_under_a_rigid_map() {
    let s = rational_patch();
    let map = rigid();
    let mapped = s.map_affine(&map);
    for (u, v) in params(32) {
        let j0 = s.ders(u, v);
        let j1 = mapped.ders(u, v);
        let n0 = j0.du.cross(j0.dv).normalize();
        let n1 = j1.du.cross(j1.dv).normalize();
        let want = map.linear * n0;
        let e = (n1 - want).norm();
        assert!(e <= 1e-13, "normal at ({u},{v}) off by {e:e}");
    }
}

// ---------------------------------------------------------------------
// C3 — the window check against what the storage door can mint
// ---------------------------------------------------------------------

/// A flat bilinear patch at height `z` over `[0,2]²` whose KNOT DOMAIN
/// is `[2, 5]` rather than `[0, 1]` — same locus, different window.
fn offdomain_patch(z: f64) -> NurbsSurface<f64> {
    let kv = KnotVector::clamped(vec![2.0, 2.0, 5.0, 5.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, z),
        Point3::new(0.0, 2.0, z),
        Point3::new(2.0, 0.0, z),
        Point3::new(2.0, 2.0, z),
    ];
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap()
}

/// **C3.** An `Approx` surface the storage door minted over a base
/// whose knot domain is not `[0, 1]` still maps: the f64 arm's window
/// check compares against the MAPPED base's own rectangle, and a map
/// leaves knots alone.
#[test]
fn c3_a_storage_door_mint_over_an_offdomain_base_still_maps() {
    let d = 0.05;
    let mut body = unit_box();
    let face = top_face(&body);
    let approx = geom_brep::approx_offset_surface(
        Arc::new(pulled_back(&offdomain_patch(1.0), d)),
        d,
        1e-9,
        band(),
    )
    .expect("the off-domain cap's offset fits");
    let Surface::Approx(a) = &approx else {
        panic!("the door mints the variant")
    };
    assert_eq!(
        a.window(),
        geom::ApproxWindow {
            u: (2.0, 5.0),
            v: (2.0, 5.0)
        },
        "the storage door stores the base's own rectangle"
    );
    body.set_face_surface(face, FaceSurface::New(approx.clone()))
        .expect("the attach-layer door accepts a live face");
    let moved = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect("an off-domain window must not be refused by the map");
    assert_eq!(
        approx_face_surface(&moved, face).window(),
        a.window(),
        "the window travels verbatim"
    );
}

// ---------------------------------------------------------------------
// C1 — planted corruptions the PR's own row does not cover
// ---------------------------------------------------------------------

/// Re-mint an `ApproxSurface` from parts with a certifier that just
/// hands back `cert` — the planting primitive.
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

/// **C1, shape A: a NARROWED window behind an honest certificate.**
/// The stored certificate says the fit is good over the base's whole
/// rectangle; the stored window claims a sub-rectangle. The map must
/// re-derive and refuse rather than mint a surface carrying a claim
/// nothing proved.
#[test]
fn c1_a_narrowed_window_does_not_survive_the_map() {
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
    // The asymmetry: `geom_brep::recertify_approx` — the door tier 3
    // reaches — does NOT check the window, so the planted body is
    // tier-3 clean of any Approx finding while the map refuses it.
    let tier3 = match topo::validate_geometric(&body, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e.iter().map(|f| format!("{f:?}")).collect(),
    };
    eprintln!("shell-2-r1 probe: tier 3 on the narrowed-window body: {tier3:?}");
    assert!(
        !tier3.iter().any(|f| f.contains("Approx")),
        "if this ever fires the window rule reached the validator too: {tier3:?}"
    );

    let e = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect_err("a narrowed window is a claim this lane never proved");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxRecertify {
                source: geom_brep::OffsetFitError::WindowUnsupported { .. }
            }
        ),
        "expected the window refusal under ApproxRecertify, got {e}"
    );
}

/// **C1, shape B: the BASE edited after certification.** The fit is
/// untouched and the certificate is the honest one the door minted;
/// what moved is the description the fit is supposed to approximate.
/// Nothing about the fit looks wrong, so only a re-derivation on the
/// mapped pair catches this.
#[test]
fn c1_an_edited_base_does_not_survive_the_map() {
    let d = 0.05;
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let good = approx_face_surface(&body, face);
    let geom::SurfaceDescription::Offset { base, .. } = good.description();
    let mut control = base.control().to_vec();
    control[0] = control[0] + Vec3::new(0.0, 0.0, 1e-3);
    let tilted = NurbsSurface::new(
        base.knots_u().clone(),
        base.knots_v().clone(),
        control,
        base.weights().to_vec(),
    )
    .unwrap();
    let planted = plant(
        geom::SurfaceDescription::Offset {
            base: Arc::new(tilted),
            d,
        },
        good.fit().clone(),
        good.window(),
        good.tolerance(),
        *good.certificate(),
    );
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();
    let e = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect_err("an edited base must not move behind a stale certificate");
    assert!(
        matches!(e, topo::TransformError::ApproxRecertify { .. }),
        "expected ApproxRecertify, got {e}"
    );
}

/// **C1, shape C: a MICRO edit of the fit**, a hundred thousand times
/// smaller than the lane's own 1e-3 coarsening and applied to an
/// INTERIOR control point (the lane edits `control[0]`, a corner that
/// every on-locus sample schedule hits). Still refused: the hull limb
/// is a sup over the whole rectangle, not a sample.
#[test]
fn c1_a_micro_edit_of_an_interior_control_point_does_not_survive_the_map() {
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
        .expect_err("a 1e-8 edit is still ten times the 1e-9 tolerance");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxRecertify {
                source: geom_brep::OffsetFitError::Limb { .. }
            }
        ),
        "expected a limb refusal, got {e}"
    );
    eprintln!("shell-2-r1 probe C1c: {e}");
}

// ---------------------------------------------------------------------
// C4 — the refusal ORDER, the finiteness gate
// ---------------------------------------------------------------------

/// **C4.** `NonFiniteMap` is decided before the `Approx` arm is
/// reached — the lane's own row covers `NotRigid` only.
#[test]
fn c4_a_non_finite_map_refuses_before_the_approx_arm() {
    let (body, _) = box_with_approx_cap(0.05, 1e-9);
    for (name, t) in [
        ("nan", Vec3::new(f64::NAN, 0.0, 0.0)),
        ("inf", Vec3::new(0.0, f64::INFINITY, 0.0)),
    ] {
        let mut map = rigid();
        map.translation = t;
        match topo::transform_rigid(&body, &map, Tol::witness()) {
            Err(topo::TransformError::NonFiniteMap { .. }) => {}
            other => panic!("{name} translation on an Approx body: got {other:?}"),
        }
    }
}

// ---------------------------------------------------------------------
// The end-to-end exercise
// ---------------------------------------------------------------------

/// **A user places an `Approx`-faced part, measures it and exports
/// it.** Every step goes through a public door: `extrude` for the
/// part, `geom_brep::approx_offset_surface` + `Body::set_face_surface`
/// for the fitted cap, `topo::transform_rigid` for the placement,
/// `topo::validate_geometric` and `topo::mass_properties` for the
/// measurement, `step_export::step_string` for the export.
///
/// The row asserts only what actually holds; everything else it
/// PRINTS, because what this exercise is for is the scope question.
#[test]
fn e2e_a_user_places_an_approx_faced_part_measures_and_exports_it() {
    let d = 0.05;
    let mut part = unit_box();
    let face = top_face(&part);
    let cap = geom_brep::approx_offset_surface(
        Arc::new(pulled_back(&planar_patch(1.0), d)),
        d,
        1e-9,
        band(),
    )
    .expect("the storage door mints the fitted cap");
    part.set_face_surface(face, FaceSurface::New(cap))
        .expect("the attach door accepts it");

    let place = rigid();
    let placed = topo::transform_rigid(&part, &place, Tol::witness())
        .expect("the placed part must come back");

    eprintln!(
        "e2e validate(part)  = {:?}",
        topo::validate_geometric(&part, Tol::witness()).map_err(|e| e.len())
    );
    eprintln!(
        "e2e validate(placed)= {:?}",
        topo::validate_geometric(&placed, Tol::witness()).map_err(|e| e.len())
    );
    for (name, b) in [("part", &part), ("placed", &placed)] {
        match topo::mass_properties(b, Tol::witness()) {
            Ok(m) => eprintln!(
                "e2e props({name}) = V {} ± {}, A {} ± {}",
                m.volume, m.volume_pad, m.surface_area, m.area_pad
            ),
            Err(e) => eprintln!("e2e props({name}) REFUSED: {e}"),
        }
        match step_export::step_string(b, &step_export::StepOptions::default(), Tol::witness()) {
            Ok(s) => eprintln!("e2e step({name}) = {} bytes", s.len()),
            Err(e) => eprintln!("e2e step({name}) REFUSED: {e}"),
        }
    }
    match topo::validate_geometric(&placed, Tol::witness()) {
        Ok(()) => eprintln!("e2e: the placed part is tier-3 clean"),
        Err(e) => {
            for f in e.iter() {
                eprintln!("e2e placed finding: {f:?}");
            }
        }
    }
}

/// **Q3 probe: the lane's certificate-agreement assertion is vacuous at
/// its own fixture.** Both limbs are certified `<= tolerance` and are
/// non-negative, so at `tolerance = 1e-9` the difference of any two of
/// them is inside the row's `1e-9` slack by construction. Printed here
/// with the actual numbers.
#[test]
fn q3_the_certificate_agreement_bound_equals_the_fixture_tolerance() {
    let d = 0.05;
    let tol = 1e-9;
    let (body, face) = box_with_approx_cap(d, tol);
    let moved = topo::transform_rigid(&body, &rigid(), Tol::witness()).expect("the body moves");
    let (c0, c1) = (
        *approx_face_surface(&body, face).certificate(),
        *approx_face_surface(&moved, face).certificate(),
    );
    eprintln!(
        "shell-2-r1 Q3: hull_sup {:e} -> {:e}; on_locus_max {:e} -> {:e}; row slack {:e}",
        c0.hull_sup, c1.hull_sup, c0.on_locus_max, c1.on_locus_max, tol
    );
    assert!(
        c0.hull_sup <= tol && c1.hull_sup <= tol,
        "both limbs are certified at or under the mint tolerance"
    );
    // The row's own assertion, re-stated: it holds for ANY pair of
    // certified limbs at this tolerance, mapped or not.
    assert!((c1.hull_sup - c0.hull_sup).abs() <= tol);
    assert!((c1.hull_sup - 0.0_f64).abs() <= tol, "…including against 0");
}
