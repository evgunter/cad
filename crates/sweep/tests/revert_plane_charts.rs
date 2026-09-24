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
//! and were fixed by the mirror all along). The other image kinds a
//! plane can carry — an iso line, a fitted or general NURBS image —
//! have no producer on a plane, so their body-level rows build the
//! face by hand through the Euler door with the image given.
//!
//! Authored across the unit's lane and its review lanes; a reviewer's
//! rows are ordinary rows. The fixtures are
//! `common::latitude_seam`'s, shared with the SHELL-9 suites so every
//! row here measures the body those rows measure.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, Surface};
use geom_brep::{
    CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, PcurveCache,
    PcurveCertifyError, PcurveCheck,
};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Point3, Vec2, Vec3};
use topo::{Body, EdgeKey, FaceSurface, HalfEdgeKey, MevSite, ValidationError};

use super::common::latitude_seam::{
    collinear_cap_drum, door_cavity, graft_recertify_failures, plane_images, void_evidence,
};
use super::shell7_common::*;

const T: f64 = 0.05;

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

/// **The red-first row.** Every plane chart image of the reverted
/// cavity is the cavity's image with `v` negated, bit for bit, at all
/// nine schedule samples — the ring's two half-circles included, whose
/// `v` channel is where the unmirrored image used to miss by
/// `2·r·sin(π/8)`. On the merge base every plane image travels
/// verbatim: the radial lines pass (a zero is its own negation up to
/// sign) and this row fails at the half-circles' first interior sample.
#[test]
fn reverted_drum_cavity_mirrors_every_plane_chart_image_with_its_frame() {
    let cavity = door_cavity(&collinear_cap_drum(), T);
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
    let cavity = door_cavity(&collinear_cap_drum(), T);
    assert!(graft_recertify_failures(&cavity).is_empty());
    let reverted = cavity.revert().expect("revert");
    let failures = graft_recertify_failures(&reverted);
    assert!(
        failures.is_empty(),
        "the reverted body refuses: {failures:?}"
    );
    assert_eq!(
        topo::validate_geometric(&reverted, tol()),
        Err(vec![ValidationError::NegativeVolume {
            solid: reverted.solids().next().expect("one solid").0
        }]),
        "a reverted body bounds the complement and nothing else fails"
    );
}

/// **Bitwise involution and determinism** on a body that carries
/// plane chart images with a non-zero `v` channel: a sign flip
/// negated twice is the original bit pattern.
#[test]
fn revert_is_a_bitwise_involution_on_a_body_with_plane_chart_images() {
    let cavity = door_cavity(&collinear_cap_drum(), T);
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
    let mut cavity = door_cavity(&collinear_cap_drum(), T);
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
    let stale = row.recertify(&carrier3, reverted_plane, None, window, band);
    assert!(
        matches!(
            stale,
            Err(PcurveCertifyError::ResidualExceeded {
                check: PcurveCheck::MapResidual,
                sample: 1
            })
        ),
        "the stored image is wrong on the reverted plane at the first interior sample: \
         the reflection is load-bearing; got {stale:?}"
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
    let cavity = door_cavity(&body, T);
    let evidence = void_evidence(&cavity);
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

/// A plane face (`z = 0`, `u_ref = +x`) carrying one edge with the
/// given chart image, built through the Euler door alone.
fn plane_face_with(image: Pcurve<f64>, carrier: Curve3<f64>, t0: f64, t1: f64) -> Body<f64> {
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let mut body = Body::<f64>::new();
    let (start, end) = (carrier.eval(t0), carrier.eval(t1));
    let seed = body.mvfs(start).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(plane))
        .unwrap();
    let chart = body.get_face(seed.face).unwrap().surface;
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart_image(chart, image),
        carrier,
        param_start: t0,
        param_end: t1,
    };
    body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        end,
        spec,
        tol(),
    )
    .unwrap();
    body
}

/// The one edge's certified curve.
fn only_curve(b: &Body<f64>) -> EdgeCurve<f64> {
    let (_, e) = b.edges().next().unwrap();
    b.get_curve_geom(e.curve)
        .unwrap()
        .certified()
        .unwrap()
        .clone()
}

fn image_of(c: &EdgeCurve<f64>) -> Pcurve<f64> {
    c.description().chart().unwrap().pcurve.clone()
}

/// **The other image kinds a plane can carry, at the body.** An
/// `IsoLine`, a `Fitted` and a `General` image of one line carrier at
/// 60° (so `v ≠ 0`) on a plane face: after `revert` the image keeps
/// its kind and is the stored one with `v` negated at nine samples,
/// the SOURCE curve re-certified on the reverted plane refuses
/// `ChartResidual` at sample 1 (the control), the mirrored curve's
/// fresh certificate is the one it carries, and `revert ∘ revert`
/// restores the body. `IsoArc` has no plane producer and no plane
/// carrier — its parameter map is a rational-quadratic Bézier's,
/// which no `Curve3` matches — so it is pinned at the `geom-brep`
/// door only (`pcurve_mirror_v`).
#[test]
fn a_plane_face_with_an_iso_line_or_nurbs_image_reverts_and_recertifies() {
    let (c, s) = (0.5f64, 3f64.sqrt() / 2.0);
    let len = 2.0;
    let carrier = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(c, s, 0.0),
    };
    let lin = || {
        NurbsCurve2::new(
            KnotVector::clamped(vec![0.0, 0.0, len, len], 1).unwrap(),
            vec![Point2::new(0.0, 0.0), Point2::new(len * c, len * s)],
            vec![1.0, 1.0],
        )
        .unwrap()
    };
    let kinds: Vec<(&str, Pcurve<f64>)> = vec![
        (
            "IsoLine",
            Pcurve::IsoLine {
                p0: Point2::new(0.0, 0.0),
                pl: Vec2::new(c, s),
            },
        ),
        ("Fitted", Pcurve::Fitted(Arc::new(lin()))),
        ("General", Pcurve::General(Arc::new(lin()))),
    ];
    let band = Band::linear(tol()).unwrap();
    let ts: Vec<f64> = (0..9u32).map(|i| len * f64::from(i) / 8.0).collect();
    for (label, image) in kinds {
        let body = plane_face_with(image, carrier.clone(), 0.0, len);
        let source = only_curve(&body);
        let reverted = body.revert().expect(label);
        let mirrored = only_curve(&reverted);
        assert_eq!(
            std::mem::discriminant(&image_of(&mirrored)),
            std::mem::discriminant(&image_of(&source)),
            "{label}: kind kept"
        );
        assert_mirrored(label, &image_of(&source), &image_of(&mirrored), &ts);
        let (start, end) = (carrier.eval(0.0), carrier.eval(len));
        let surfaces = |k| reverted.get_surface(k).cloned();
        let control = source.recertify(start, end, surfaces, band);
        assert!(
            matches!(
                control,
                Err(CertifyError::ResidualExceeded {
                    check: CertCheck::ChartResidual,
                    sample: 1
                })
            ),
            "{label}: the source's image on the reverted plane: {control:?}"
        );
        let rerun = mirrored.recertify(start, end, surfaces, band).expect(label);
        assert_eq!(
            format!("{rerun:?}"),
            format!("{:?}", mirrored.certificate()),
            "{label}: the certificate that travelled verbatim is the fresh run's"
        );
        assert_eq!(
            format!("{:?}", reverted.revert().unwrap()),
            format!("{body:?}"),
            "{label}: involution"
        );
    }
}

/// **Where the signed zero lands.** A half-circle at rest in the
/// plane's chart has `+0` coefficients in its image; the reflection
/// turns them into `−0`, which a `Debug` comparison of the IMAGE sees
/// (so the reverted body is not `Debug`-equal to its source even where
/// only zeros moved). The CERTIFICATE never shows one: every stored
/// field is a norm, and a distance squares a signed zero away — which
/// is the argument for carrying it verbatim, measured at the one
/// place the sign could have leaked.
#[test]
fn a_signed_zero_lands_in_the_mirrored_image_and_never_in_its_certificate() {
    let circle = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let image = Pcurve::Harmonic {
        p0: Point2::new(0.0, 0.0),
        pa: Vec2::new(1.0, 0.0),
        pb: Vec2::new(0.0, 1.0),
        pl: Vec2::new(0.0, 0.0),
    };
    let body = plane_face_with(image, circle, 0.0, core::f64::consts::PI);
    let reverted = body.revert().unwrap();
    let m = only_curve(&reverted);
    let img = format!("{:?}", image_of(&m));
    assert!(
        img.contains("-0.0"),
        "the mirrored image carries signed zeros: {img}"
    );
    assert!(
        !format!("{:?}", m.certificate()).contains("-0.0"),
        "a certificate stores norms: {:?}",
        m.certificate()
    );
    assert_ne!(format!("{reverted:?}"), format!("{body:?}"));
    assert_eq!(
        format!("{:?}", reverted.revert().unwrap()),
        format!("{body:?}"),
        "involution"
    );
}
