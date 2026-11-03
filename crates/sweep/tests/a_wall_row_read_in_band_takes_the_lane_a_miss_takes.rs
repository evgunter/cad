//! **A loft wall's row read in band takes the lane a definite miss
//! takes** (D3: a lane choice records nothing, and in band takes the
//! lane definite takes).
//!
//! The fixture is the loft of a profile with one semicircular edge, at
//! `f64` and at `Interval`: its arc wall a rational-quadratic spline
//! chart whose seams are described as `IsoLine` images on the
//! neighbouring walls and whose rims are its `IsoArc` rows. The wall is
//! re-charted in place onto its surface moved by a distance inside the
//! band, or past it, its edges' descriptions left as they were:
//!
//! - a seam's pick (`pcurve_iso_wall_seam`) shows no column, and the
//!   seam takes its `General` image;
//! - a rim's pick (`pcurve_iso_arc_row_direction`) shows no row, and a
//!   circle on a spline chart has no general lane yet: the rim refuses
//!   "not yet", the same refusal in band as decided off.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::PcurveCertifyError;
use geom_core::{Bounds, Point2, Tol, Vec3};
use profile::test_support::bulge_loop;
use sweep::test_support::{lift_places, lift_sections, stacked_at};
use topo::construction::Construct;
use topo::construction::FaceSurface;
use topo::{AtRestPolicy, Body, FaceKey, HalfEdgeKey, Pcurve, PcurveMintError};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(tol()).unwrap()
}

/// The band's interior: a distance the probe can neither confirm nor
/// rule out (between ε and K·ε).
fn in_band() -> f64 {
    tol().eps() * tol().k().sqrt()
}

/// A distance decided off: far past `K·ε` at every tolerance row.
const OFF: f64 = 1e-3;

/// The semicircle prism at `T`, its wall rows minted.
fn semicircle_prism<T: AtRestPolicy>() -> Body<T> {
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let profile = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 1.0),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    sweep::loft_body::<T>(
        &lift_sections::<T>(&[profile(), profile()]),
        &lift_places::<T>(&stacked_at(&[0.0, 1.0]), tol()),
        1,
        tol(),
    )
    .expect("the prism builds")
    .body
}

/// The arc wall, its rims (the `IsoArc` rows) and its seams (the
/// columns, `u` held).
fn arc_wall<T: AtRestPolicy + Bounds>(
    body: &Body<T>,
) -> (FaceKey, Vec<HalfEdgeKey>, Vec<HalfEdgeKey>) {
    let rim = body
        .pcurves()
        .find(|(_, c)| matches!(c.pcurve(), Pcurve::IsoArc { .. }))
        .map(|(h, _)| h)
        .expect("the prism has an arc rim");
    let wall = body.face_of_half_edge(rim).unwrap();
    let (mut rims, mut seams) = (Vec::new(), Vec::new());
    for (h, c) in body.pcurves() {
        if body.face_of_half_edge(h) != Some(wall) {
            continue;
        }
        match c.pcurve() {
            Pcurve::IsoArc { .. } => rims.push(h),
            Pcurve::IsoLine { pl, .. } if pl.x.lo() == 0.0 && pl.x.hi() == 0.0 => seams.push(h),
            other => panic!("{h:?}: the wall's rows are exact iso images: {other:?}"),
        }
    }
    assert_eq!((rims.len(), seams.len()), (2, 2), "two rims and two seams");
    (wall, rims, seams)
}

/// `body` with `wall` re-charted in place onto its surface moved by
/// `lift`.
fn moved<T: AtRestPolicy>(body: &Body<T>, wall: FaceKey, lift: Vec3<f64>) -> Body<T> {
    let mut out = body.clone();
    let face = out.get_face(wall).unwrap();
    let (key, sense) = (face.surface, face.sense);
    let Some(Surface::Nurbs(net)) = out.get_surface(key).cloned() else {
        panic!("the arc wall is a spline chart")
    };
    let lift = lift.map(T::from_f64);
    let net: NurbsSurface<T> = net.map_points(|p| p + lift);
    // Lifts RechartUnvouched and RechartStrandsDescriptions: the wall
    // re-charted in place onto its surface moved, its edges'
    // descriptions left as they were, is the row's premise.
    out.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New {
            surface: Surface::Nurbs(std::sync::Arc::new(net)),
            sense,
        },
    )
    .unwrap();
    out
}

/// What `nurbs_iso_derive` answers for `h`, the stored row detached:
/// `pcurve_of` answers a cached image first.
fn derived<T: AtRestPolicy>(body: &Body<T>, h: HalfEdgeKey) -> Result<Pcurve<T>, PcurveMintError> {
    let mut fresh = body.clone();
    fresh.detach_pcurve(h);
    topo::pcurve_of(&fresh, h, band())
}

/// **A seam in the band takes its `General` image.** The wall moved
/// `in_band()` along `y`, its normal at both seams: each seam's stated
/// columns, the closed-form columns and the column the foot names are
/// all undecided at the probe, and the seam takes the general lane.
fn a_seam_in_the_band_takes_its_general_image<T: AtRestPolicy + Bounds>() {
    let body = semicircle_prism::<T>();
    let (wall, _, seams) = arc_wall(&body);
    let lifted = moved(&body, wall, Vec3::new(0.0, in_band(), 0.0));
    for h in seams {
        let got = derived(&lifted, h);
        assert!(
            matches!(got, Ok(Pcurve::General(_))),
            "{h:?}: a seam in the band takes its General image, at {}: {got:?}",
            T::NAME
        );
    }
}

#[test]
fn a_seam_in_the_band_takes_its_general_image_at_f64() {
    a_seam_in_the_band_takes_its_general_image::<f64>();
}

#[test]
fn a_seam_in_the_band_takes_its_general_image_at_interval() {
    a_seam_in_the_band_takes_its_general_image::<geom_core::Interval>();
}

/// **A rim in the band refuses "not yet", as one decided off does.** The
/// wall moved along `z`, its rows' normal: `in_band()`, where every
/// arc-rim candidate is undecided at the probe, and `OFF`, where every
/// one is decided off. Either way the rim is not shown on a row, a
/// circle on a spline chart has no general lane, and the rim refuses
/// that gap: the same refusal, ending "not yet" with no tolerance offer.
fn a_rim_in_the_band_refuses_as_one_off_its_row<T: AtRestPolicy + Bounds>() {
    let body = semicircle_prism::<T>();
    let (wall, rims, _) = arc_wall(&body);
    let near = moved(&body, wall, Vec3::new(0.0, 0.0, in_band()));
    let far = moved(&body, wall, Vec3::new(0.0, 0.0, OFF));
    for h in rims {
        let got = derived(&near, h);
        let Err(
            ref refused @ PcurveMintError::Certify {
                error:
                    PcurveCertifyError::NoGeneralLane {
                        chart: geom::SurfaceKind::Nurbs,
                        carrier: geom::CurveKind::Circle,
                        gap: geom_brep::LaneGap::Unwritten,
                    },
                ..
            },
        ) = got
        else {
            panic!(
                "{h:?}: a rim in the band has no general lane, at {}: {got:?}",
                T::NAME
            )
        };
        assert_eq!(
            derived(&far, h).err().as_ref(),
            Some(refused),
            "{h:?}: in band refuses as decided off, at {}",
            T::NAME
        );
        for at in [
            geom_brep::recourse::Reading::Build,
            geom_brep::recourse::Reading::AtRest,
        ] {
            let ending = refused.ending(at).expect("a lane gap has an ending");
            assert!(
                ending.ends_with(geom_core::NOT_YET_ENDING) && !ending.contains("tolerance"),
                "{h:?}: \"not yet\", no tolerance offer, at {} {at:?}: {ending}",
                T::NAME
            );
        }
    }
}

#[test]
fn a_rim_in_the_band_refuses_as_one_off_its_row_at_f64() {
    a_rim_in_the_band_refuses_as_one_off_its_row::<f64>();
}

#[test]
fn a_rim_in_the_band_refuses_as_one_off_its_row_at_interval() {
    a_rim_in_the_band_refuses_as_one_off_its_row::<geom_core::Interval>();
}
