//! **A rigid map moves an `Approx` face the kernel minted near ε, and
//! the image re-derives** (ONEHOME fork 259, PR 6824's
//! `[tolerance.residuals]`: whatever a certificate or a meter reads is a
//! function of the geometry, so a rigid map moves it only by its rounding
//! width).
//!
//! An offset fit's certificate reads every vector-valued bound from its
//! coefficient vectors: their norms from above, their hull's distance
//! from the origin from below (`‖E‖`'s floor, `PatchSpans::cell_norm_inf`),
//! and balls about them for the curvature meter's forms
//! (`PatchCell::balls`), never a per-coordinate box folded into a norm. So a rotation moves the
//! certificate only by the rounding of the image itself: the image's
//! control points are rounded at the patch's own extent, which moves the
//! residual by a few ulps of it. Measured on the faces below at the
//! default ε (while `‖E‖`'s floor read a ball about the vectors; the
//! rows hold with the hull's): under these rotations the hull bound moves by up to
//! `6.2e-4` of itself, and under ±2-ulp noise on the control points with
//! no rotation at all by up to `3.9e-4` — the same size, so that drift is
//! the image's rounding, not a frame reading. The curvature meter's reach
//! moves by `9e-11` (it moved by `2.7e-3` while its forms were boxes).
//!
//! **A face within rounding of ε may flip** (`[simple.conditioned]`): a
//! face minted closer to ε than its image's rounding is one whose
//! re-derivation may land on either side, and nothing in the certifier
//! can say otherwise, because the image is a different surface by that
//! much. These rows mint the face [`MARGIN`] under ε, past that rounding,
//! and assert that EVERY rotation's image re-derives and that the moved
//! face re-certifies at a later finish.
//!
//! **The subject is one shape at a scale.** Every rigid map commutes
//! with a uniform scale about the origin, and so does the offset
//! (`s·(S + d·n) = s·S + (s·d)·n`), so every round's fit of the patch
//! scaled by `s` is that round's fit of the patch, scaled, and its
//! bound is `s` times the patch's up to the arithmetic. Choosing `s`
//! from the patch's own round-[`ROUND`] bound, which is below every
//! earlier round's, lands the minted face on that round a chosen
//! fraction under the run's ε at every eps row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;
use topo::construction::Construct;

use geom::{NurbsSurface, Surface};
use geom_brep::OffsetFitLane;
use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use topo::construction::FaceSurface;
use topo::{Body, FaceKey};

/// How far under ε the minted face's bound is aimed, as a fraction of
/// ε: past the image's rounding (module docs, `6.2e-4` measured), so a
/// rotation's image cannot cross ε.
const MARGIN: f64 = 1.0 / 256.0;

/// The fit round the minted face certifies on (module docs).
const ROUND: u32 = 2;

/// A target the unscaled patch's round-[`ROUND`] fit meets and its
/// earlier rounds do not (their bounds are `4.2e-11` and above, its
/// own `1.27e-12`).
const ROUND_TARGET: f64 = 1e-11;

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
/// minted by the production door at the run's ε on round [`ROUND`],
/// [`MARGIN`] under ε.
fn near_eps_face(d: f64) -> geom::ApproxSurface<f64> {
    let tol = Tol::witness();
    let eps = tol.eps();
    // The unscaled patch's round-`ROUND` bound, through the
    // numeric-target door.
    let (_, unscaled) = geom_brep::offset_fit::fit_offset_at(
        &bowed(1.0),
        d,
        ROUND_TARGET,
        Band::linear(tol).unwrap(),
    )
    .unwrap();
    assert_eq!(
        unscaled.rounds, ROUND,
        "the unscaled patch meets {ROUND_TARGET:e} on round {ROUND}"
    );
    let s = eps * (1.0 - MARGIN) / unscaled.hull_sup;
    let face =
        geom_brep::approx_offset_surface(Arc::new(bowed(s)), d * s, tol).unwrap_or_else(|e| {
            panic!(
                "d = {d}: the scaled patch mints at the run's ε: {e_at}",
                e_at = e.render(geom_brep::recourse::Reading::Build)
            )
        });
    let c = face.certificate();
    // The subject is what the file says it is: a round-`ROUND` face whose
    // bound sits just under ε. A scaled fit that stopped elsewhere makes
    // every row below vacuous.
    assert!(
        c.rounds == ROUND && c.hull_sup >= eps * (1.0 - 4.0 * MARGIN) && c.hull_sup <= eps,
        "d = {d}: the minted face is round {ROUND}, just under ε: rounds {}, hull_sup/ε {}",
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
    let created = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    body.set_face_surface(
        created.face,
        FaceSurface::New {
            surface: Surface::Approx(Arc::new(face)),
            sense: true,
        },
    )
    .unwrap();
    (body, created.face)
}

fn approx_on(body: &Body<f64>, face: FaceKey) -> Arc<geom::ApproxSurface<f64>> {
    match body.face_surface(face) {
        Some(Surface::Approx(a)) => Arc::clone(a),
        other => panic!("{face:?} must wear an approximating surface, got {other:?}"),
    }
}

/// Two nets agree bit for bit — control points, weights and knots.
fn same_net(a: &NurbsSurface<f64>, b: &NurbsSurface<f64>) -> bool {
    a.weights() == b.weights()
        && a.knots_u_f64_interim().knots() == b.knots_u_f64_interim().knots()
        && a.knots_v_f64_interim().knots() == b.knots_v_f64_interim().knots()
        && a.control().len() == b.control().len()
        && a.control()
            .iter()
            .zip(b.control())
            .all(|(p, q)| p.same_bits(*q))
}

/// **Every image re-derives.** The face certifies at the run's ε where
/// it stands, and so does its image under every rotation: the
/// certificate is a function of the geometry, and the face is minted
/// past its image's rounding of ε (module docs). This row once pinned
/// the opposite — some image refusing on the hull limb — while the
/// certificate's lower bounds and the curvature meter were assembled in
/// the ambient frame.
#[test]
fn every_image_of_a_face_minted_near_eps_re_derives() {
    let tol = Tol::witness();
    let lane = OffsetFitLane::fit();
    for d in [0.05_f64, -0.05] {
        let face = near_eps_face(d);
        lane.recertify(&face, tol).unwrap_or_else(|e| {
            panic!(
                "d = {d}: the minted face certifies where it stands: {e_at}",
                e_at = e.render(geom_brep::recourse::Reading::Build)
            )
        });
        let refused: Vec<String> = rotations()
            .into_iter()
            .filter_map(|(name, map)| {
                let (description, fit) = image(&face, &map);
                lane.remap(&description, &fit, face.window(), tol)
                    .err()
                    .map(|e| format!("{name}: {}", e.render(geom_brep::recourse::Reading::Build)))
            })
            .collect();
        assert!(
            refused.is_empty(),
            "d = {d}: every image re-derives, none refuses: {refused:#?}"
        );
        // The meters are functions of the geometry: an image's regularity
        // floor and curvature reach are the source's up to rounding (the
        // reach moved by `2.7e-3` of itself while the curvature forms were
        // per-coordinate boxes).
        let band = Band::linear(tol).unwrap();
        let read = |description: &geom::SurfaceDescription<f64>, fit: &NurbsSurface<f64>| {
            geom_brep::offset_fit::certify_offset_over_at(
                description,
                fit,
                face.window(),
                1.0,
                band,
            )
            .unwrap_or_else(|e| panic!("d = {d}: the meters read: {e:?}"))
        };
        let source = read(face.description(), face.fit());
        for (name, map) in rotations() {
            let (description, fit) = image(&face, &map);
            let moved = read(&description, &fit);
            let drift = |a: f64, b: f64| ((a - b) / b).abs();
            assert!(
                drift(moved.normal_floor, source.normal_floor) <= 1e-9
                    && drift(moved.curvature_reach, source.curvature_reach) <= 1e-9,
                "d = {d}, {name}: the meters moved past rounding: floor {} -> {}, reach {} -> {}",
                source.normal_floor,
                moved.normal_floor,
                source.curvature_reach,
                moved.curvature_reach
            );
        }
    }
}

/// **The move carries the verdict, and a later finish agrees.** Every
/// rotation moves the body, and the moved face IS the image, bit for
/// bit, its certificate the operand's; tier 3's door then re-certifies
/// the moved face afresh (`finish(b.moved(iso)) ≡ b.moved(iso)`, one
/// face wide: no closed body the tree builds carries a curved `Approx`
/// face, so the verdict pin's offset cap,
/// `sweep`'s `a_moved_body_keeps_its_verdict`, is a planar patch whose
/// fit is exact).
#[test]
fn a_rigid_map_moves_a_face_minted_near_eps() {
    let tol = Tol::witness();
    let lane = OffsetFitLane::fit();
    for d in [0.05_f64, -0.05] {
        let face = near_eps_face(d);
        let (body, key) = body_with(face.clone());
        for (name, map) in rotations() {
            let moved = topo::transform_rigid(&body, &geom_core::test_support::rigid(&map, tol))
                .unwrap_or_else(|e| {
                    panic!(
                        "d = {d}, {name}: a sound face must move: {e_at}",
                        e_at = e.render(geom_brep::recourse::Reading::Build)
                    )
                });
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
                "d = {d}, {name}: the description maps exactly"
            );
            assert!(
                same_net(after.fit(), &fit),
                "d = {d}, {name}: the fit is the image, not a re-fit"
            );
            assert_eq!(
                format!("{:?}", after.certificate()),
                format!("{:?}", face.certificate()),
                "d = {d}, {name}: the certificate is carried"
            );
            lane.recertify(&after, tol).unwrap_or_else(|e| {
                panic!(
                    "d = {d}, {name}: the moved face re-certifies at a later finish: {e_at}",
                    e_at = e.render(geom_brep::recourse::Reading::Build)
                )
            });
        }
    }
}

/// **The map carries what it is handed; a finish decides.** The near-ε
/// face with an interior fit control point nudged by ten times ε,
/// behind the certificate it had before, is a planted face that no
/// longer certifies where it stands. The map is construction state: it
/// moves the planted face as it moves any, and tier 3's own door
/// refuses the image as it refuses the source.
#[test]
fn a_planted_face_is_moved_and_refused_by_the_finishs_door() {
    let tol = Tol::witness();
    let d = 0.05;
    let face = near_eps_face(d);
    let fit = face.fit();
    let mut control = fit.control().to_vec();
    let mid = control.len() / 2;
    control[mid] = control[mid] + Vec3::new(0.0, 0.0, 10.0 * tol.eps());
    let nudged = NurbsSurface::new(
        fit.knots_u_f64_interim().clone(),
        fit.knots_v_f64_interim().clone(),
        control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let spec = geom::SurfaceSpec {
        description: face.description().clone(),
        fit: nudged,
        window: face.window(),
    };
    let planted = geom::ApproxSurface::certify(spec, |_, _, _| {
        Ok::<_, std::convert::Infallible>(*face.certificate())
    })
    .unwrap();
    assert!(
        OffsetFitLane::fit().recertify(&planted, tol).is_err(),
        "the planted fit must fail where it stands, or this row tests nothing"
    );
    let (body, key) = body_with(planted);
    for (name, map) in rotations() {
        let moved = topo::transform_rigid(&body, &geom_core::test_support::rigid(&map, tol))
            .unwrap_or_else(|e| panic!("{name}: the map moves what it is handed: {e:?}"));
        assert!(
            OffsetFitLane::fit()
                .recertify(&approx_on(&moved, key), tol)
                .is_err(),
            "{name}: tier 3's door refuses the planted face's image"
        );
    }
}
