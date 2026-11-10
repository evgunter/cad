//! **A loft wall's row read in band takes the lane a definite miss
//! takes** (D3: a lane choice records nothing, and in band takes the
//! lane definite takes).
//!
//! The fixture is the loft of a profile with one semicircular edge, at
//! `f64` and at `Interval`: its arc wall a rational-quadratic spline
//! chart whose seams are described as `IsoLine` images on the
//! neighbouring walls and whose rims are its `IsoArc` rows. The wall is
//! re-charted in place onto its surface moved by a distance inside the
//! band, its edges' descriptions left as they were:
//!
//! - a seam's pick (`pcurve_iso_wall_seam`) shows no column, and the
//!   seam takes its `General` image;
//! - a rim's pick (`pcurve_iso_arc_row_direction`) shows no row, and the
//!   rim ends as the wall's incidence read: in band, in its lever; moved
//!   past the band, off the chart.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::semicircle_wall::{
    arc_wall, band, derived, in_band, recharted, semicircle_prism, wall_half_edges,
};
use geom_core::{Bounds, Vec3};
use topo::{AtRestPolicy, Body, FaceKey, HalfEdgeKey, Pcurve, PcurveMintError};

/// The arc wall, its rims (the `IsoArc` rows) and its seams (the
/// columns, `u` held).
fn wall_rows<T: AtRestPolicy + Bounds>(
    body: &Body<T>,
) -> (FaceKey, Vec<HalfEdgeKey>, Vec<HalfEdgeKey>) {
    let wall = arc_wall(body);
    let (mut rims, mut seams) = (Vec::new(), Vec::new());
    for h in wall_half_edges(body, wall) {
        match body.pcurve(h).map(|c| c.pcurve()) {
            Some(Pcurve::IsoArc { .. }) => rims.push(h),
            Some(Pcurve::IsoLine { pl, .. }) if pl.x.lo() == 0.0 && pl.x.hi() == 0.0 => {
                seams.push(h);
            }
            other => panic!("{h:?}: the wall's rows are exact iso images: {other:?}"),
        }
    }
    assert_eq!((rims.len(), seams.len()), (2, 2), "two rims and two seams");
    (wall, rims, seams)
}

/// `body` with `wall` re-charted in place onto its surface moved by
/// `lift`.
fn moved<T: AtRestPolicy>(body: &Body<T>, wall: FaceKey, lift: Vec3<f64>) -> Body<T> {
    let lift = lift.map(T::from_f64);
    recharted(body, wall, |net| net.map_points(|p| p + lift), false)
}

/// The carrier and interval of `h`'s edge, and the surface of the face
/// it bounds.
fn edge_on_wall<T: AtRestPolicy>(
    body: &Body<T>,
    h: HalfEdgeKey,
) -> (geom::Curve3<T>, (T, T), geom::Surface<T>) {
    let edge = body.get_half_edge(h).unwrap().edge;
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap();
    let surface = body
        .face_surface(body.face_of_half_edge(h).unwrap())
        .unwrap()
        .clone();
    (curve.carrier().clone(), curve.params(), surface)
}

/// **A seam in the band takes its `General` image.** The wall moved
/// `in_band()` along `y`, its normal at both seams: each seam's stated
/// columns, the closed-form columns and the column the foot names are
/// all undecided at the probe, and the seam takes the general lane.
fn a_seam_in_the_band_takes_its_general_image<T: AtRestPolicy + Bounds>() {
    let body = semicircle_prism::<T>();
    let (wall, _, seams) = wall_rows(&body);
    let lifted = moved(&body, wall, Vec3::new(0.0, in_band(), 0.0));
    for h in seams {
        // The pose: the seam's own column, stored on the native wall, is
        // undecided at the probe on the moved one.
        let column = body.pcurve(h).unwrap().pcurve().clone();
        let (carrier, (t0, t1), surface) = edge_on_wall(&lifted, h);
        assert!(
            geom_brep::probe_pick(
                "pcurve_iso_wall_seam",
                &carrier,
                (t0, t1 - t0),
                &surface,
                band(),
                [column],
            )
            .is_err(),
            "{h:?}: the seam's column reads in band, at {}",
            T::NAME
        );
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

/// **A rim in the band ends in its incidence read's lever.** The wall
/// moved `in_band()` along `z`, its rows' normal: every arc-rim candidate
/// is undecided at the probe, so the rim is not shown on a row, and the
/// lane choice ends as the wall's incidence read
/// (`geom_brep::read_edge_incidence`), which is in band too
/// (`IncidenceRead::Undecided`), its lever at every door, never a defect, for the
/// bottom rim, which the wall has left by the move. The top rim lies
/// inside the moved wall, on its chart where no row holds it, and refuses
/// "not yet". Moved a millimetre instead, the bottom rim's read is decided
/// off (`IncidenceRead::Off`), the defect ending of the door that
/// reads it, and the top rim still refuses "not yet".
fn a_rim_in_the_band_ends_as_its_incidence_read<T: AtRestPolicy + Bounds>() {
    let body = semicircle_prism::<T>();
    let (wall, rims, _) = wall_rows(&body);
    let near = moved(&body, wall, Vec3::new(0.0, 0.0, in_band()));
    let off = moved(&body, wall, Vec3::new(0.0, 0.0, 1e-3));
    let height = |h: HalfEdgeKey| {
        let (carrier, (t0, _), _) = edge_on_wall(&body, h);
        carrier.eval(t0).z.lo()
    };
    let bottom = rims
        .iter()
        .map(|&h| height(h))
        .fold(f64::INFINITY, f64::min);
    let not_yet = |got: &Result<Pcurve<T>, PcurveMintError>| {
        matches!(
            got,
            Err(PcurveMintError::Certify { error, .. })
                if error.kinds() == Some((geom::SurfaceKind::Nurbs, geom::CurveKind::Circle))
                    && error.refused_check() == Some(geom_brep::PcurveCheck::ChartClass)
        )
    };
    for h in rims {
        let (carrier, (t0, t1), surface) = edge_on_wall(&near, h);
        assert!(
            geom_brep::pick_arc_rim(&carrier, (t0, t1 - t0), &surface, band()).is_err(),
            "{h:?}: the pick reads in band, at {}",
            T::NAME
        );
        let got = derived(&near, h);
        if height(h) > bottom {
            assert!(
                not_yet(&got) && not_yet(&derived(&off, h)),
                "{h:?}: the top rim is on the moved wall, no row holding it, at {}: {got:?}",
                T::NAME
            );
            continue;
        }
        let Err(ref refused @ PcurveMintError::Certify { ref error, .. }) = got else {
            panic!(
                "{h:?}: a rim in the band ends as its incidence read, at {}: {got:?}",
                T::NAME
            )
        };
        let read = geom_brep::IncidenceRead::of(error).expect("an incidence refusal");
        assert!(
            matches!(read, geom_brep::IncidenceRead::Undecided(_)),
            "{h:?}: the read is in band, at {}: {read:?}",
            T::NAME
        );
        for at in [
            geom_brep::recourse::Reading::Build,
            geom_brep::recourse::Reading::AtRest,
        ] {
            let ending = refused.ending(at).expect("an incidence read has an ending");
            assert!(
                ending.ends_with(&read.ending(at)) && !ending.contains("defect"),
                "{h:?}: the read's lever, at {} {at:?}: {ending}",
                T::NAME
            );
        }
        let got = derived(&off, h);
        let kinds = (
            topo::readback::face_carrier_kind(&off, off.face_of_half_edge(h).unwrap()).unwrap(),
            topo::readback::edge_carrier_kind(&off, off.get_half_edge(h).unwrap().edge).unwrap(),
        );
        assert_eq!(
            kinds,
            (geom::SurfaceKind::Nurbs, geom::CurveKind::Circle),
            "{h:?}: the rim is a circle on the spline wall"
        );
        assert!(
            matches!(
                &got,
                Err(PcurveMintError::Certify { error, .. })
                    if matches!(geom_brep::IncidenceRead::of(error), Some(geom_brep::IncidenceRead::Off(_)))
                        && error.kinds() == Some(kinds)
            ),
            "{h:?}: a rim its wall moved a millimetre from ends off the chart, at {}: {got:?}",
            T::NAME
        );
    }
}

#[test]
fn a_rim_in_the_band_ends_as_its_incidence_read_at_f64() {
    a_rim_in_the_band_ends_as_its_incidence_read::<f64>();
}

#[test]
fn a_rim_in_the_band_ends_as_its_incidence_read_at_interval() {
    a_rim_in_the_band_ends_as_its_incidence_read::<geom_core::Interval>();
}
