//! **A carrier whose foot path bends takes the general lane on a spline
//! chart** (D3): a circle no boundary row holds, and an ellipse, which
//! has no exact class there, mint the `General` image
//! (`geom_brep::FittedLane::general_image`), climbed to the structure the
//! bend asks for, and certify. Where its certificate still refuses on a
//! rim the wall holds, the refusal is the lane's gap, "not yet", never a
//! tolerance offer; a rim in band of the wall ends in its incidence read's
//! lever. A stated row, and a stored row read at rest, keep their own
//! term.
//!
//! The fixture is the unit cylinder sheet over `[0, TURN] × [0, 1]`
//! (`topo::test_support::cyl_wall_sheet_tilted`) with its analytic
//! cylinder swapped for a rational ring net in two sub-arcs over
//! `z ∈ [−½, 1]`: the top rim is the net's `v = 1` row, and the bottom
//! rim at `z = 0` lies on the wall's interior, crossing the knot line
//! between the sub-arcs. The bottom rim is the cylinder's circle at
//! that height, in two description forms, or the ellipse the cylinder
//! is cut in by a plane through the rim's two ends, tilted up between
//! them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, LaneGap, Pcurve, PcurveCertifyError};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol};
use std::sync::Arc;
use topo::construction::Construct;
use topo::test_support::{CylFrame, CylKey, cyl_wall_sheet_tilted};
use topo::{AtRestPolicy, Body, FaceKey, HalfEdgeKey};

/// The sheet's turn.
const TURN: f64 = 1.2;

/// The ring's lowest height: the bottom rim at `z = 0` is interior.
const LOW: f64 = -0.5;

/// The ellipse's tilt: its plane rises by this much per metre across
/// the chord between the rim's ends.
const TILT: f64 = 1.0;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A bend ([`ring`]) that leaves the wall within the band of the bottom
/// rim: past `zero()`, short of `escalate()`, at its widest.
fn in_band_push() -> f64 {
    let b = band();
    1.5 * 0.5 * (b.zero() + b.escalate()) * (1.0 + (TURN / 4.0).cos())
}

/// The unit cylinder over `z ∈ [LOW, 1]` through `TURN` from `x̂`, in
/// two rational quadratic sub-arcs, the second's middle control point on
/// the `z = LOW` row pushed `push` off the axis: at `z = 0`, two thirds
/// of the way down, the wall parts from the unit circle by
/// `(2/3)·push / (1 + cos(TURN/4))` at that sub-arc's middle, and the top
/// rim's row is the circle's.
fn ring(push: f64) -> NurbsSurface<f64> {
    ring_split_at(push, 0.5)
}

/// [`ring`]`(push)` with its sub-arcs' shared knot at `knot`: the same
/// wall, each sub-arc's span moved affinely, so a point of the wall
/// reads at another `u`.
fn ring_split_at(push: f64, knot: f64) -> NurbsSurface<f64> {
    let h = TURN / 2.0;
    let mut ku = vec![0.0; 3];
    ku.extend([knot; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = f64::from(j) * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for (z, bend) in [(LOW, push), (1.0, 0.0)] {
            let r = reach * if j == 3 { 1.0 + bend } else { 1.0 };
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap()
}

/// The bottom rim's carrier and description form.
#[derive(Clone, Copy, Debug)]
enum Rim {
    /// The circle, as the cylinder cut by the rim's plane.
    Circle,
    /// The same circle restated through the scaffolding door.
    ScaffoldCircle,
    /// The ellipse the cylinder is cut in by the rim's plane tilted
    /// [`TILT`] about the chord between the rim's ends.
    Ellipse,
}

const RIMS: [Rim; 3] = [Rim::Circle, Rim::ScaffoldCircle, Rim::Ellipse];

/// The bottom rim's edge, its carrier and its interval.
fn bottom_curve<T: AtRestPolicy>(
    body: &Body<T>,
    bottom: HalfEdgeKey,
) -> (topo::EdgeKey, topo::EdgeCurve<T>) {
    let edge = body.get_half_edge(bottom).unwrap().edge;
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .clone();
    (edge, curve)
}

/// The sheet at `T` with its bottom rim as `rim`, the cylinder swapped
/// for [`ring`]`(push)`: the wall face and the bottom rim's half-edge on
/// it.
fn sheet<T: AtRestPolicy + geom_core::Bounds>(
    rim: Rim,
    push: f64,
) -> (Body<T>, FaceKey, HalfEdgeKey) {
    let mut body = Body::<T>::new();
    let tilt = match rim {
        Rim::Ellipse => TILT,
        Rim::Circle | Rim::ScaffoldCircle => 0.0,
    };
    let (wall, cyl) = cyl_wall_sheet_tilted(
        &mut body,
        CylFrame::canonical(1.0),
        CylKey::Bare,
        (0.0, TURN),
        (0.0, 1.0),
        tilt,
        tol(),
    );
    let bottom = bottom_rim(&body, wall);
    if let Rim::ScaffoldCircle = rim {
        let (edge, curve) = bottom_curve(&body, bottom);
        let (t0, t1) = curve.params();
        let spec = EdgeCurveSpec::arc_of_circle(curve.carrier().clone(), t0, t1)
            .expect("the rim is a circle");
        body.set_edge_curve(edge, spec, tol())
            .expect("the rim restates through the scaffolding door");
    }
    let net = Surface::Nurbs(Arc::new(ring(push).map_scalar(T::from_f64)));
    let body = body.with_surface_moved_for_tests(cyl, net).unwrap();
    (body, wall, bottom)
}

/// The bottom rim's half-edge on `wall`.
fn bottom_rim<T: AtRestPolicy + geom_core::Bounds>(body: &Body<T>, wall: FaceKey) -> HalfEdgeKey {
    body.half_edges()
        .map(|(k, _)| k)
        .filter(|&h| body.face_of_half_edge(h) == Some(wall))
        .find(|&h| {
            let (_, c) = bottom_curve(body, h);
            c.carrier().is_curved() && c.carrier().eval(c.params().0).z.lo() < 0.5
        })
        .expect("the bottom rim bounds the wall")
}

/// The wall's rows minted: the body, the bottom rim's half-edge, and its
/// row or its refusal.
fn minted<T: AtRestPolicy + geom_core::Bounds>(
    rim: Rim,
    push: f64,
) -> (
    Body<T>,
    HalfEdgeKey,
    Result<geom_brep::PcurveCache<T>, PcurveCertifyError>,
) {
    let (mut body, wall, bottom) = sheet::<T>(rim, push);
    let got = match topo::mint_pcurves_of(&mut body, &[wall], tol()) {
        Ok(_) => Ok(body
            .pcurve(bottom)
            .expect("the mint writes the rim's row")
            .clone()),
        Err(topo::PcurveMintError::Certify { half_edge, error }) => {
            assert_eq!(
                half_edge, bottom,
                "{rim:?}: only the bottom rim refuses: {error:?}"
            );
            Err(error)
        }
        Err(other) => panic!("{rim:?}: the mint refuses the bottom rim's row: {other:?}"),
    };
    (body, bottom, got)
}

/// Whether `error` is the general lane's gap: "not yet", decided or in
/// band, and its ending at a build that and no tolerance offer.
fn is_the_lanes_gap(error: &PcurveCertifyError) -> bool {
    let gap = matches!(
        error,
        PcurveCertifyError::NoGeneralLane {
            chart: geom::SurfaceKind::Nurbs,
            gap: LaneGap::Image,
            ..
        } | PcurveCertifyError::Escalated {
            check: geom_brep::PcurveCheck::GeneralLane,
            ..
        }
    );
    let ending = error.ending(geom_brep::recourse::Reading::Build);
    gap && ending.contains(geom_core::NOT_YET_ENDING) && !ending.contains("loosen")
}

/// The finest `ε` at which an `Interval` certificate reads the image's
/// samples in band: a degree-5 image evaluated over an interval
/// parameter is about `1e-12` m wide at a sample, which check 3's
/// enclosure carries.
const INTERVAL_SAMPLE_WIDTH: f64 = 2e-12;

/// Whether `T`'s certificate reads the image's samples under its width
/// ([`INTERVAL_SAMPLE_WIDTH`]) in this run.
fn under_the_sample_width<T: AtRestPolicy>() -> bool {
    T::NAME == "interval" && band().zero() < INTERVAL_SAMPLE_WIDTH
}

/// Every rim on the true ring mints its `General` image and certifies:
/// the circle in both forms, which no row holds, and the ellipse, which
/// has no exact class. At `Interval` where the band is under the image's
/// sample width, the certificate reads in band and the row ends "not yet".
fn a_bending_rim_certifies<T: AtRestPolicy + geom_core::Bounds>() {
    for rim in RIMS {
        let (_, _, got) = minted::<T>(rim, 0.0);
        match got {
            Ok(row) => {
                assert!(
                    matches!(row.pcurve(), Pcurve::General(_)),
                    "{rim:?}: the row is the general image, at {}: {:?}",
                    T::NAME,
                    row.pcurve()
                );
                let envelope = row.certificate().envelope;
                assert!(
                    envelope.hi() <= band().zero(),
                    "{rim:?}: the hull bound is within ε, at {}: {envelope:?}",
                    T::NAME
                );
            }
            Err(error) => assert!(
                under_the_sample_width::<T>() && is_the_lanes_gap(&error),
                "{rim:?}: the general image certifies, at {}: {error:?}",
                T::NAME
            ),
        }
    }
}

#[test]
fn a_bending_rim_certifies_at_f64() {
    a_bending_rim_certifies::<f64>();
}

#[test]
fn a_bending_rim_certifies_at_interval() {
    a_bending_rim_certifies::<geom_core::Interval>();
}

/// A ring bent within the band of the rim: the chart's incidence read
/// does not show the rim off, so it takes the general image. The image's
/// certificate refuses, and on a spline chart a refused certificate ends
/// by the carrier's incidence read (`geom_brep`'s `incidence_ended`): in
/// band, in its lever, which moves the geometry. It is the carrier's own
/// pose, so it is neither the lane's gap nor a tolerance offer nor a
/// defect, in every form.
fn a_band_scale_pose_ends_in_its_incidence_lever<T: AtRestPolicy + geom_core::Bounds>() {
    for rim in RIMS {
        let (_, _, got) = minted::<T>(rim, in_band_push());
        let Err(error) = got else {
            panic!(
                "{rim:?}: a rim in band of the wall does not certify, at {}",
                T::NAME
            )
        };
        let read = geom_brep::IncidenceRead::of(&error);
        let ending = error.ending(geom_brep::recourse::Reading::Build);
        assert!(
            read.is_some_and(|r| !r.is_definite())
                && !is_the_lanes_gap(&error)
                && read.is_some_and(
                    |r| ending.ends_with(&r.ending(geom_brep::recourse::Reading::Build))
                )
                && !ending.contains("loosen"),
            "{rim:?}: a rim in band of the wall ends in its incidence read's lever, at {}: \
             {error:?}: {ending}",
            T::NAME
        );
    }
}

#[test]
fn a_band_scale_pose_ends_in_its_incidence_lever_at_f64() {
    a_band_scale_pose_ends_in_its_incidence_lever::<f64>();
}

#[test]
fn a_band_scale_pose_ends_in_its_incidence_lever_at_interval() {
    a_band_scale_pose_ends_in_its_incidence_lever::<geom_core::Interval>();
}

/// A `General` row the mint certified, read at rest where it no longer
/// holds: its wall bent in band of its carrier, and its carrier lifted a
/// quarter up the wall, which the row then misses. The stored row keeps
/// its own term, never the lane's gap, which is a statement about a
/// fresh image alone.
fn a_stored_row_off_its_wall_keeps_its_term<T: AtRestPolicy + geom_core::Bounds>() {
    for rim in RIMS {
        let (body, bottom, got) = minted::<T>(rim, 0.0);
        let row = match got {
            Ok(row) => row,
            Err(error) => {
                assert!(
                    under_the_sample_width::<T>() && is_the_lanes_gap(&error),
                    "{rim:?}: the mint stores a row, at {}: {error:?}",
                    T::NAME
                );
                println!(
                    "SKIPPED: {rim:?} at {} under its sample width stores no row to read at rest",
                    T::NAME
                );
                continue;
            }
        };
        assert!(
            matches!(row.pcurve(), Pcurve::General(_)),
            "{rim:?}: a General row"
        );
        let (_, curve) = bottom_curve(&body, bottom);
        let wall = |push: f64| Surface::Nurbs(Arc::new(ring(push).map_scalar(T::from_f64)));
        let lifted = curve
            .carrier()
            .translated(geom_core::Vec3::new(0.0, 0.0, 0.25).map(T::from_f64))
            .expect("a conic moves");
        for (pose, carrier, surface) in [
            (
                "its wall bent in band",
                curve.carrier(),
                wall(in_band_push()),
            ),
            ("its carrier lifted", &lifted, wall(0.0)),
        ] {
            let refused =
                row.recertify(carrier, curve.params(), &surface, band(), T::fitted_lane());
            assert!(
                refused.as_ref().is_err_and(|e| !is_the_lanes_gap(e)),
                "{rim:?}, {pose}: the stored row refuses at rest by its own term, at {}: \
                 {refused:?}",
                T::NAME
            );
        }
    }
}

#[test]
fn a_stored_row_off_its_wall_keeps_its_term_at_f64() {
    a_stored_row_off_its_wall_keeps_its_term::<f64>();
}

#[test]
fn a_stored_row_off_its_wall_keeps_its_term_at_interval() {
    a_stored_row_off_its_wall_keeps_its_term::<geom_core::Interval>();
}

/// A construction's STATED `General` row of a bending rim that is wrong
/// for its wall keeps its own term wherever it is read, never the lane's
/// gap: nothing derived it.
///
/// - **At a build**: wrong by `1e-4` m in `v` at one control point whose
///   basis vanishes at every sample of check 3, the edge door certifies
///   it over its span and refuses it as the description's own image,
///   `ChartImageUnavailable`, before it is stored.
/// - **At rest**: the true row, stated and certified, then its wall
///   re-parametrized under it ([`ring_split_at`]), so the carrier still
///   lies on the wall and the row maps it elsewhere. The mint reads the
///   row off the description and derives nothing, so its certificate's
///   refusal stands.
fn a_stated_wrong_general_row_keeps_its_term<T: AtRestPolicy + geom_core::Bounds>() {
    for rim in [Rim::Circle, Rim::Ellipse] {
        let (_, _, got) = minted::<T>(rim, 0.0);
        let row = match got {
            Ok(row) => row,
            Err(error) => {
                assert!(
                    under_the_sample_width::<T>() && is_the_lanes_gap(&error),
                    "{rim:?}: the mint stores a row, at {}: {error:?}",
                    T::NAME
                );
                println!(
                    "SKIPPED: {rim:?} at {} under its sample width mints no row to state",
                    T::NAME
                );
                continue;
            }
        };
        let Pcurve::General(image) = row.pcurve() else {
            panic!("{rim:?}: a General row")
        };
        let (a, b) = image.domain();
        let samples: Vec<T> = (0..9)
            .map(|i| T::from_f64(a + (b - a) * f64::from(i) / 8.0))
            .collect();
        let bumped = |at: usize| {
            let mut control = image.control().to_vec();
            control[at].y = control[at].y + T::from_f64(1e-4);
            geom::NurbsCurve2::new(
                image.knot_values().clone(),
                control,
                image.weights().to_vec(),
            )
            .unwrap()
        };
        let n = image.control().len();
        let at = (n / 3..n)
            .find(|&at| {
                let wrong = bumped(at);
                samples.iter().all(|&t| {
                    let (p, q) = (wrong.eval(t), image.eval(t));
                    (p.y - q.y).hi().abs().max((p.y - q.y).lo().abs()) < 1e-11
                })
            })
            .expect("a control point whose basis vanishes at every sample");
        let state = |body: &mut Body<T>, wall: FaceKey, image: &Arc<geom::NurbsCurve2<T>>| {
            let (edge, curve) = bottom_curve(body, bottom_rim(body, wall));
            let (t0, t1) = curve.params();
            let key = body.get_face(wall).unwrap().surface;
            let spec = EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::chart_image(
                    key,
                    Pcurve::General(Arc::clone(image)),
                ),
                carrier: curve.carrier().clone(),
                param_start: t0,
                param_end: t1,
            };
            body.set_edge_curve(edge, spec, tol())
                .map(|_| curve.carrier().kind())
                .map_err(|e| (e, curve.carrier().kind()))
        };

        let (mut body, wall, _) = sheet::<T>(rim, 0.0);
        let stated = state(&mut body, wall, &Arc::new(bumped(at)));
        let Err((topo::EulerOpError::Certification { error }, carrier)) = &stated else {
            panic!(
                "{rim:?}: the edge door refuses the wrong stated row, at {}: {stated:?}",
                T::NAME
            )
        };
        assert_eq!(
            *error,
            geom_brep::CertifyError::ChartImageUnavailable {
                chart: geom::SurfaceKind::Nurbs,
                carrier: *carrier,
            },
            "{rim:?}: a stated row is refused as its own image, at {}",
            T::NAME
        );
        let ending = error
            .ending(geom_brep::recourse::Reading::Build)
            .expect("the image's refusal is decided");
        assert!(
            !ending.contains(geom_core::NOT_YET_ENDING),
            "{rim:?}: the stated row's refusal is not the lane's gap, at {}: {ending}",
            T::NAME
        );

        let (mut body, wall, bottom) = sheet::<T>(rim, 0.0);
        state(&mut body, wall, image).expect("the true row states");
        let key = body.get_face(wall).unwrap().surface;
        let split = Surface::Nurbs(Arc::new(ring_split_at(0.0, 0.4).map_scalar(T::from_f64)));
        let mut body = body.with_surface_moved_for_tests(key, split).unwrap();
        let got = topo::mint_pcurves_of(&mut body, &[wall], tol());
        let Err(topo::PcurveMintError::Certify { half_edge, error }) = got else {
            panic!(
                "{rim:?}: the stated row refuses at rest, at {}: {got:?}",
                T::NAME
            )
        };
        assert_eq!(half_edge, bottom, "{rim:?}: the bottom rim's row refuses");
        assert!(
            matches!(
                error,
                PcurveCertifyError::ResidualExceeded {
                    check: geom_brep::PcurveCheck::MapResidual,
                    ..
                }
            ) && !is_the_lanes_gap(&error),
            "{rim:?}: a stated row keeps its own term at rest, check 3's map residual, at {}: \
             {error:?}",
            T::NAME
        );
    }
}

#[test]
fn a_stated_wrong_general_row_keeps_its_term_at_f64() {
    a_stated_wrong_general_row_keeps_its_term::<f64>();
}

#[test]
fn a_stated_wrong_general_row_keeps_its_term_at_interval() {
    a_stated_wrong_general_row_keeps_its_term::<geom_core::Interval>();
}
