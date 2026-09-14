//! **`Body::revert` mirrors a plane's chart images and rows with its
//! frame.** Negating a plane's `normal` with `u_ref` fixed negates
//! `v_ref = normal × u_ref`, so the plane's chart is reflected,
//! `(u, v) ↦ (u, −v)`; every datum stated in that chart's coordinates
//! — a `Chart` edge image, a stored pcurve row — is re-stated under
//! the reflection, and every certificate the source carried is a
//! certificate of the result (`topo::revert` module docs). Measured on
//! SHELL's drum: a cylinder whose top cap carries a collinear profile
//! vertex, so one plane holds four faces with a latitude ring between
//! them, and the ring's two half-circles are the plane images with a
//! non-zero `v` channel (the six radial lines lie on the `u_ref` axis
//! and were fixed by the mirror all along).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use geom::Surface;
use geom_brep::{Pcurve, PcurveCache};
use sweep::Revolution;
use topo::{Body, EdgeKey, HalfEdgeKey, ValidationError};

use super::shell7_common::*;

const R: f64 = 1.0;
const H: f64 = 2.0;
const T: f64 = 0.05;

fn collinear_cap_drum() -> Body<f64> {
    polyline(
        &[(0.0, 0.0), (R, 0.0), (R, H), (R / 2.0, H), (0.0, H)],
        Revolution::Full,
    )
}

/// The axial door's cavity of the drum, tier-3 valid.
fn door_cavity(body: &Body<f64>) -> Body<f64> {
    let mut cavity = body.clone();
    let band = geom_core::Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(body, T), band, tol())
        .expect("the door takes the drum");
    assert_eq!(
        topo::validate_geometric(&cavity, tol()),
        Ok(()),
        "cavity tier 3"
    );
    cavity
}

/// Every edge described as a chart image on a plane, with its image.
fn plane_images(body: &Body<f64>) -> Vec<(EdgeKey, Pcurve<f64>)> {
    body.edges()
        .filter_map(|(k, e)| {
            let curve = body.get_curve_geom(e.curve)?.certified()?;
            let c = curve.description().chart()?;
            matches!(body.get_surface(c.surface), Some(Surface::Plane { .. }))
                .then(|| (k, c.pcurve.clone()))
        })
        .collect()
}

/// The certification schedule's nine parameters of `edge`.
fn schedule(body: &Body<f64>, edge: EdgeKey) -> Vec<f64> {
    let (_, (t0, t1)) = carrier(body, edge);
    (0..9u32)
        .map(|i| t0 + (t1 - t0) * f64::from(i) / 8.0)
        .collect()
}

/// `mirrored` is `stored` with its `v` channel negated, bit for bit,
/// at every schedule sample — up to the sign of a zero: a radial
/// line's `v` coefficients are zeros, and a sum of signed zeros
/// scaled by `cos t`, `sin t` and `t` lands on whichever zero the
/// term order gives, which no metred distance can tell apart (a
/// squared zero is `+0`).
fn assert_mirrored(label: &str, stored: &Pcurve<f64>, mirrored: &Pcurve<f64>, ts: &[f64]) {
    for &t in ts {
        let (a, b) = (stored.eval(t), mirrored.eval(t));
        assert_eq!(
            a.x.to_bits(),
            b.x.to_bits(),
            "{label}: u channel moved at t = {t}"
        );
        if a.y == 0.0 {
            assert!(b.y == 0.0, "{label}: a zero v became {} at t = {t}", b.y);
        } else {
            assert_eq!(
                (-a.y).to_bits(),
                b.y.to_bits(),
                "{label}: v channel is not the negation at t = {t}"
            );
        }
    }
}

/// Re-certify every edge exactly as the void door's graft does — the
/// image verbatim, carrier and interval verbatim, endpoints from
/// `he_plus`, surfaces from the body — and return the refusals.
fn graft_recertify_failures(body: &Body<f64>) -> Vec<(EdgeKey, geom_brep::CertifyError)> {
    let band = geom_core::Band::linear(tol()).expect("band");
    body.edges()
        .filter_map(|(ek, e)| {
            let curve = body.get_curve_geom(e.curve)?.certified()?;
            let start_v = body.get_half_edge(e.he_plus)?.start;
            let end_v = body.half_edge_end(e.he_plus)?;
            geom_brep::EdgeCurve::certify(
                curve.restated_spec(),
                point(body, start_v),
                point(body, end_v),
                |sk| body.get_surface(sk).cloned(),
                band,
            )
            .err()
            .map(|err| (ek, err))
        })
        .collect()
}

/// **The red-first row.** Every plane chart image of the reverted
/// cavity is the cavity's image with `v` negated, bit for bit, at all
/// nine schedule samples — the ring's two half-circles included, whose
/// `v` channel is where the unmirrored image used to miss by
/// `2·r·sin(π/8)`. On the merge base every plane image travels
/// verbatim: the radial lines pass (a zero is its own negation up to
/// sign) and this row fails at the half-circles' first interior sample.
#[test]
fn reverted_drum_cavity_mirrors_every_plane_chart_image_with_its_frame() {
    let cavity = door_cavity(&collinear_cap_drum());
    let reverted = cavity.revert().expect("revert");
    let stored = plane_images(&cavity);
    let mirrored = plane_images(&reverted);
    assert_eq!(stored.len(), mirrored.len(), "keys are the source's");
    let mut circles = 0;
    for ((ek, before), (ek2, after)) in stored.iter().zip(&mirrored) {
        assert_eq!(ek, ek2);
        if matches!(carrier(&cavity, *ek).0, geom::Curve3::Circle { .. }) {
            circles += 1;
        }
        assert_mirrored(&format!("{ek:?}"), before, after, &schedule(&cavity, *ek));
    }
    assert_eq!(
        circles, 2,
        "the latitude ring's two half-circles are plane images"
    );
    assert!(
        stored.len() > circles,
        "the radial lines are plane images too: {} images",
        stored.len()
    );
}

/// **The contract at `revert`'s module docs, metered.** Every edge of
/// the reverted cavity re-certifies through the graft's own meter
/// (the image verbatim against the reverted plane), and tier 3 of the
/// reverted body reports exactly the complement's `NegativeVolume` —
/// no `EdgeCertification`, no pcurve finding. On the merge base the
/// two half-circles refuse `ChartResidual` at sample 1.
#[test]
fn reverted_drum_cavity_re_certifies_edge_for_edge_and_tier_3_reports_only_the_complement() {
    let cavity = door_cavity(&collinear_cap_drum());
    assert!(graft_recertify_failures(&cavity).is_empty());
    let reverted = cavity.revert().expect("revert");
    let failures = graft_recertify_failures(&reverted);
    assert!(
        failures.is_empty(),
        "the reverted body refuses: {failures:?}"
    );
    assert_eq!(
        topo::validate_geometric(&reverted, tol()),
        Err(vec![ValidationError::NegativeVolume]),
        "a reverted body bounds the complement and nothing else fails"
    );
}

/// **Bitwise involution and determinism** on a body that carries
/// plane chart images with a non-zero `v` channel: a sign flip
/// negated twice is the original bit pattern.
#[test]
fn revert_is_a_bitwise_involution_on_a_body_with_plane_chart_images() {
    let cavity = door_cavity(&collinear_cap_drum());
    let original = format!("{cavity:?}");
    let once = cavity.revert().unwrap();
    assert_ne!(
        format!("{once:?}"),
        original,
        "the reversal moved something"
    );
    assert_eq!(format!("{:?}", once.revert().unwrap()), original);
    assert_eq!(
        format!("{:?}", cavity.revert().unwrap()),
        format!("{once:?}")
    );
}

/// **A stored pcurve row on a plane face travels the same way**: no
/// producer mints one (planar faces derive on demand), so the row is
/// attached by hand from a half-circle's own image, certified against
/// the cap plane. After `revert` the row's image is the mirrored
/// one, its certificate is byte for byte the certificate a fresh run
/// on the reverted body produces, the unmirrored row refuses there,
/// and the involution restores the bits.
#[test]
fn a_stored_pcurve_row_on_a_plane_face_is_mirrored_and_its_certificate_travels_verbatim() {
    let mut cavity = door_cavity(&collinear_cap_drum());
    let band = geom_core::Band::linear(tol()).expect("band");
    let (ek, image) = plane_images(&cavity)
        .into_iter()
        .find(|(k, _)| matches!(carrier(&cavity, *k).0, geom::Curve3::Circle { .. }))
        .expect("a latitude half-circle on the plane");
    let (he, curve_key): (HalfEdgeKey, topo::CurveKey) = {
        let edge = cavity.get_edge(ek).unwrap();
        (edge.he_plus, edge.curve)
    };
    let surface_key = cavity
        .get_curve_geom(curve_key)
        .and_then(|g| g.certified())
        .and_then(|c| c.description().chart())
        .map(|c| c.surface)
        .unwrap();
    let (carrier3, (t0, t1)) = carrier(&cavity, ek);
    let plane = cavity.get_surface(surface_key).unwrap().clone();
    let window = image.chart_box(t0, t1);
    let row = PcurveCache::certify(image, t0, t1, &carrier3, &plane, window, band)
        .expect("a plane row certifies in the harmonic lane");
    assert!(cavity.attach_pcurve(he, row.clone()).is_none());

    let reverted = cavity.revert().expect("revert");
    let mirrored = reverted.pcurve(he).expect("the row keeps its key");
    assert_mirrored(
        "row",
        row.pcurve(),
        mirrored.pcurve(),
        &schedule(&cavity, ek),
    );
    assert_eq!(mirrored.params(), row.params());
    assert_eq!(
        format!("{:?}", mirrored.certificate()),
        format!("{:?}", row.certificate()),
        "the certificate travels verbatim"
    );
    // The verbatim certificate IS the fresh run's: re-certify the
    // mirrored row against the reverted plane, window mirrored with it.
    let reverted_plane = reverted.get_surface(surface_key).unwrap();
    let rerun = mirrored
        .recertify(
            &carrier3,
            reverted_plane,
            None,
            mirrored.pcurve().chart_box(t0, t1),
            band,
        )
        .expect("the mirrored row certifies on the reverted plane");
    assert_eq!(
        format!("{rerun:?}"),
        format!("{:?}", row.certificate()),
        "a fresh run on the reverted body metres the same numbers"
    );
    assert!(
        row.recertify(&carrier3, reverted_plane, None, window, band)
            .is_err(),
        "the stored image is wrong on the reverted plane: the reflection is load-bearing"
    );
    let back = reverted.revert().expect("revert");
    assert_eq!(
        format!("{:?}", back.pcurve(he).unwrap()),
        format!("{row:?}"),
        "involution on the row"
    );
}

/// **The void door takes the drum's cavity**, which is where `shell`
/// used to stop (`ShellError::Insert`, the graft's `Recertify`): the
/// graft re-runs the meter on the reverted cavity's images, and they
/// are right now. The assembled thin solid's closed form is pinned end
/// to end in `shell7_seam_corner`; this row pins the door alone.
#[test]
fn insert_voids_takes_the_reverted_drum_cavity() {
    let body = collinear_cap_drum();
    let cavity = door_cavity(&body);
    let evidence = topo::VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    topo::VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    };
    let images_before = plane_images(&body).len() + plane_images(&cavity).len();
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    topo::insert_voids(&mut out, &solids, cavity, &evidence, tol())
        .expect("the graft's meter passes on the mirrored images");
    assert_eq!(out.shells().count(), 2, "outer + cavity");
    assert_eq!(
        plane_images(&out).len(),
        images_before,
        "every plane image of both bodies was grafted"
    );
}
