//! **A rigid map does not refuse an `Approx` face the kernel minted.**
//!
//! An offset fit's `hull_sup` is a bound assembled from control-hull
//! enclosures in the ambient frame, so a rotation moves it, and the
//! fit loop stops at the first round that certifies, so a minted face
//! can sit anywhere under ε. A face minted within the rotation's drift
//! of ε therefore has an IMAGE that re-derives above ε. These rows
//! build that face through the production mint door at the run's ε,
//! show that its image refuses the re-derivation `transform_rigid`
//! runs, and show that the body moves anyway: the transform re-fits the
//! mapped description and ships a face tier 3's own door accepts.
//!
//! **The subject is one shape at a scale.** Every rigid map commutes
//! with a uniform scale about the origin, and so does the offset
//! (`s·(S + d·n) = s·S + (s·d)·n`), so the round-0 fit of the patch
//! scaled by `s` is the round-0 fit of the patch, scaled, and its bound
//! is `s` times the patch's to rounding. Choosing `s` from the
//! patch's own round-0 bound lands the minted face a fixed fraction
//! [`MARGIN`] under the run's ε at every eps row, with the same rigid
//! drift at every row, which is what lets one row reproduce the
//! refusal whatever ε the run committed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_brep::{OffsetFitError, OffsetFitLane, OffsetLimb};
use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use topo::{Body, FaceKey, FaceSurface};

/// How far under ε the minted face's bound is placed, as a fraction of
/// ε. Well inside the drift the maps below produce on this shape (about
/// half a percent), and well outside the rounding of the scaling
/// itself (a few parts per million).
const MARGIN: f64 = 1.0 / 512.0;

/// A gently bowed biquadratic patch over `[0,1]²`, scaled by `s` about
/// the origin — a base whose offset is not a NURBS, so the fit has
/// real work to do.
fn bowed(s: f64) -> NurbsSurface<f64> {
    const BOW: f64 = 1.5e-2;
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            let z = BOW * u * (1.0 - u) + (BOW * 2.0 / 3.0) * v * v;
            control.push(Point3::new(s * u, s * v, s * z));
        }
    }
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
}

/// The certified `Approx` face of the scaled patch's offset at `d`,
/// minted by the production door at the run's ε, with its bound
/// `MARGIN·ε` under ε.
fn near_eps_face(d: f64) -> geom::ApproxSurface<f64> {
    let tol = Tol::witness();
    let eps = tol.eps();
    // The unscaled patch's round-0 bound: the numeric-target door with
    // a target every round-0 fit meets, so the loop stops there.
    let (_, unscaled) =
        geom_brep::offset_fit::fit_offset_at(&bowed(1.0), d, 1.0, Band::linear(tol).unwrap())
            .unwrap();
    assert_eq!(
        unscaled.rounds, 0,
        "a target of a metre certifies at round 0"
    );
    let s = eps * (1.0 - MARGIN) / unscaled.hull_sup;
    let minted = geom_brep::approx_offset_surface(Arc::new(bowed(s)), d * s, tol)
        .unwrap_or_else(|e| panic!("d = {d}: the scaled patch mints at the run's ε: {e}"));
    let Surface::Approx(face) = minted else {
        panic!("the mint door produces `Surface::Approx`")
    };
    let c = face.certificate();
    // The subject is what the file says it is: a round-0 face whose
    // bound sits within twice the margin of ε. A scaled fit that
    // refined, or landed elsewhere, makes every row below vacuous.
    assert!(
        c.rounds == 0 && c.hull_sup <= eps && c.hull_sup >= eps * (1.0 - 2.0 * MARGIN),
        "d = {d}: the minted face must sit just under ε — rounds {}, hull_sup/ε {}",
        c.rounds,
        c.hull_sup / eps
    );
    Arc::unwrap_or_clone(face)
}

/// Rigid maps about four axes at eight angles each — the oblique ones
/// are where a rotation re-splits the geometry across all three axes.
fn rotations() -> Vec<(String, Affine3<f64>)> {
    let axes = [
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.3, -0.4, 0.8),
    ];
    let mut out = Vec::new();
    for axis in axes {
        for k in 1..=8 {
            let angle = f64::from(k) * 0.135;
            out.push((
                format!("{axis:?} by {angle:.3}"),
                Affine3::rotation_about_axis(Point3::origin(), axis.normalize(), angle),
            ));
        }
    }
    out
}

/// The image of `face` under `map`: both nets mapped control-point-wise.
fn image(
    face: &geom::ApproxSurface<f64>,
    map: &Affine3<f64>,
) -> (geom::SurfaceDescription<f64>, NurbsSurface<f64>) {
    let geom::SurfaceDescription::Offset { base, d } = face.description();
    (
        geom::SurfaceDescription::Offset {
            base: Arc::new(base.map_points(|p| map.transform_point(p))),
            d: *d,
        },
        face.fit().map_points(|p| map.transform_point(p)),
    )
}

/// The seed body with `face` on its one face.
fn body_with(face: geom::ApproxSurface<f64>) -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let created = body.mvfs(Point3::new(0.0, 0.0, 0.0)).unwrap();
    body.set_face_surface(
        created.face,
        FaceSurface::New(Surface::Approx(Arc::new(face))),
    )
    .unwrap();
    (body, created.face)
}

fn approx_on(body: &Body<f64>, face: FaceKey) -> Arc<geom::ApproxSurface<f64>> {
    match body.get_surface(body.get_face(face).unwrap().surface) {
        Some(Surface::Approx(a)) => Arc::clone(a),
        other => panic!("{face:?} must wear an approximating surface, got {other:?}"),
    }
}

/// Two nets agree bit for bit — control points, weights and knots.
fn same_net(a: &NurbsSurface<f64>, b: &NurbsSurface<f64>) -> bool {
    a.weights() == b.weights()
        && a.knots_u().knots() == b.knots_u().knots()
        && a.knots_v().knots() == b.knots_v().knots()
        && a.control().len() == b.control().len()
        && a.control().iter().zip(b.control()).all(|(p, q)| {
            p.x.to_bits() == q.x.to_bits()
                && p.y.to_bits() == q.y.to_bits()
                && p.z.to_bits() == q.z.to_bits()
        })
}

/// **The reproduction.** The face certifies at the run's ε where it
/// stands — tier 3's door accepts it — and its image under some
/// rotation refuses the same door at the same ε, on the hull limb. That
/// refusal is what `transform_rigid` used to hand back as
/// `ApproxRecertify` for a body that validates.
#[test]
fn the_image_of_a_face_minted_near_eps_refuses_re_derivation() {
    let tol = Tol::witness();
    let lane = OffsetFitLane::fit();
    for d in [0.05_f64, -0.05] {
        let face = near_eps_face(d);
        lane.recertify(&face, tol)
            .unwrap_or_else(|e| panic!("d = {d}: the minted face certifies where it stands: {e}"));
        let refused: Vec<String> = rotations()
            .into_iter()
            .filter_map(|(name, map)| {
                let (description, fit) = image(&face, &map);
                match lane.remap(&description, &fit, face.window(), tol) {
                    Ok(_) => None,
                    Err(OffsetFitError::Limb {
                        limb: OffsetLimb::HullSup,
                        ..
                    }) => Some(name),
                    Err(e) => panic!("d = {d}, {name}: only the hull limb moves here: {e}"),
                }
            })
            .collect();
        assert!(
            !refused.is_empty(),
            "d = {d}: no rotation moved the bound past ε, so the face does not reproduce the \
             refusal"
        );
    }
}

/// **The fix.** Every rotation moves the body. Where the image
/// certifies, the moved face IS the image, bit for bit, with the
/// operand's `rounds` carried; where it refuses, the moved face is a
/// fresh fit of the mapped description — the base mapped bit for bit,
/// a different net — and tier 3's door accepts it at the run's ε. Both
/// classes occur, so neither half of the row is vacuous.
#[test]
fn a_rigid_map_moves_a_face_minted_near_eps() {
    let tol = Tol::witness();
    let lane = OffsetFitLane::fit();
    for d in [0.05_f64, -0.05] {
        let face = near_eps_face(d);
        let (body, key) = body_with(face.clone());
        let (mut imaged, mut refitted) = (0_usize, 0_usize);
        for (name, map) in rotations() {
            let moved = topo::transform_rigid(&body, &map, tol)
                .unwrap_or_else(|e| panic!("d = {d}, {name}: a sound face must move: {e}"));
            let after = approx_on(&moved, key);
            let (description, fit) = image(&face, &map);
            let geom::SurfaceDescription::Offset {
                base: want,
                d: want_d,
            } = &description;
            let geom::SurfaceDescription::Offset {
                base: got,
                d: got_d,
            } = after.description();
            assert!(
                same_net(got, want) && got_d.to_bits() == want_d.to_bits(),
                "d = {d}, {name}: the description maps exactly either way"
            );
            lane.recertify(&after, tol).unwrap_or_else(|e| {
                panic!("d = {d}, {name}: tier 3's door must accept the moved face: {e}")
            });
            if lane.remap(&description, &fit, face.window(), tol).is_ok() {
                imaged += 1;
                assert!(
                    same_net(after.fit(), &fit),
                    "d = {d}, {name}: an image that certifies is shipped as the image"
                );
                assert_eq!(after.certificate().rounds, face.certificate().rounds);
            } else {
                refitted += 1;
                assert!(
                    !same_net(after.fit(), &fit),
                    "d = {d}, {name}: a refused image must not be what ships"
                );
            }
        }
        assert!(
            imaged > 0 && refitted > 0,
            "d = {d}: both answers must occur — {imaged} imaged, {refitted} re-fitted"
        );
    }
}

/// **A face that does not certify where it stands is not re-fitted.**
/// The near-ε face with an interior fit control point nudged by ten
/// times ε, behind the certificate it had before, refuses
/// `ApproxRecertify` on the image's limb under every rotation: the map
/// re-fits only a sound face, so it cannot launder a body tier 3
/// rejects into one it accepts.
#[test]
fn a_face_that_fails_where_it_stands_is_not_re_fitted() {
    let tol = Tol::witness();
    let d = 0.05;
    let face = near_eps_face(d);
    let fit = face.fit();
    let mut control = fit.control().to_vec();
    let mid = control.len() / 2;
    control[mid] = control[mid] + Vec3::new(0.0, 0.0, 10.0 * tol.eps());
    let nudged = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let spec = geom::SurfaceSpec {
        description: face.description().clone(),
        fit: nudged,
        window: face.window(),
    };
    // Planted with the operand's certificate: the surface claims what
    // it no longer is, and only a re-derivation can tell.
    let planted = geom::ApproxSurface::certify(spec, |_, _, _| {
        Ok::<_, std::convert::Infallible>(*face.certificate())
    })
    .unwrap();
    assert!(
        OffsetFitLane::fit().recertify(&planted, tol).is_err(),
        "the planted fit must fail where it stands, or this row tests nothing"
    );
    let (body, _) = body_with(planted);
    for (name, map) in rotations() {
        match topo::transform_rigid(&body, &map, tol) {
            Err(topo::TransformError::ApproxRecertify {
                source: OffsetFitError::Limb { .. },
            }) => {}
            other => panic!("{name}: expected the image's limb refusal, got {other:?}"),
        }
    }
}
