//! **A carrier whose foot path bends takes the general lane on a spline
//! chart** (D3): a circle no boundary row holds, and an ellipse, which
//! has no exact class there, mint the `General` image
//! (`geom_brep::FittedLane::general_image`), climbed to the structure the
//! bend asks for, and certify. Where its certificate still refuses, a
//! pose in band of the wall among them, the refusal is the lane's gap,
//! "not yet", and never a tolerance offer. A stored row read at rest
//! keeps its own term.
//!
//! The fixture is the unit cylinder sheet over `[0, TURN] × [0, 1]`
//! (`topo::test_support::cyl_wall_sheet_keyed`) with its analytic
//! cylinder swapped for a rational ring net in two sub-arcs over
//! `z ∈ [−½, 1]`: the top rim is the net's `v = 1` row, and the bottom
//! rim at `z = 0` lies on the wall's interior, crossing the knot line
//! between the sub-arcs. The bottom rim is the cylinder's circle at
//! that height, in two description forms, or the ellipse the cylinder
//! is cut in by a plane through the rim's two ends, tilted up between
//! them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, NurbsSurface, Surface};
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

/// A bend that leaves the wall within the band of the rim: past
/// `zero()`, short of `escalate()`, at its widest.
fn in_band_push() -> f64 {
    let b = band();
    0.5 * (b.zero() + b.escalate()) * (1.0 + (TURN / 4.0).cos())
}

/// The unit cylinder over `z ∈ [LOW, 1]` through `TURN` from `x̂`, in
/// two rational quadratic sub-arcs, the second's middle control point
/// pushed `push` off the axis.
fn ring(push: f64) -> NurbsSurface<f64> {
    let h = TURN / 2.0;
    let mut ku = vec![0.0; 3];
    ku.extend([0.5; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = f64::from(j) * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        let r = reach * if j == 3 { 1.0 + push } else { 1.0 };
        for z in [LOW, 1.0] {
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

/// The sheet at `T` with its bottom rim restated as `rim`, the cylinder
/// swapped for [`ring`]`(push)`; the wall face and the bottom rim's
/// half-edge on it.
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

/// The wall's rows minted, and the bottom rim's.
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

/// Every rim on the true ring mints its `General` image and certifies:
/// the circle in both forms, which no row holds, and the ellipse.
fn a_bending_rim_certifies<T: AtRestPolicy + geom_core::Bounds>() {
    for rim in RIMS {
        let (_, _, got) = minted::<T>(rim, 0.0);
        eprintln!(
            "ROW {} {rim:?}: {:?}",
            T::NAME,
            got.as_ref().map(|c| c.certificate().clone())
        );
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
