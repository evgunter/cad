//! Review r2 probes for PR 5959 (the shared row's certificate).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::tol::band;
use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::edge_nurbs::{pick_arc_rim, sub_arc_joint};
use geom_brep::keys::SurfaceKey;
use geom_brep::pcurve_cache::Pcurve;
use geom_brep::{
    CertifyError, CurveSelector, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane,
    SharedRow, Structural, SurfacePair,
};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use slotmap::SlotMap;

const TURN: f64 = 1.2;
const M: usize = 24;

/// A ring about `z` through `TURN` in `M` rational quadratic sub-arcs:
/// its `v = 0` row the unit circle, its `v = 1` row column `j`'s control
/// point plus `offset(j)`.
fn ring(offset: impl Fn(usize, Vec3<f64>) -> Vec3<f64>) -> Surface<f64> {
    let h = TURN / M as f64;
    let mut ku = vec![0.0; 3];
    for k in 1..M {
        ku.extend([sub_arc_joint(k, M); 2]);
    }
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=2 * M {
        let theta = j as f64 * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        let a = Point3::new(reach * theta.cos(), reach * theta.sin(), 0.0);
        let radial = Vec3::new(reach * theta.cos(), reach * theta.sin(), 0.0);
        control.push(a);
        control.push(a + offset(j, radial));
        weights.extend([w, w]);
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            KnotVector::clamped(ku, 2).unwrap(),
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            control,
            weights,
        )
        .unwrap(),
    ))
}

/// **Walls tangent along a sub-span the row's transversality feet step
/// over.** The cylinder `z ∈ [0, 1]` and a flange that runs outward in
/// `z = 0` except on sub-arc 4 (carrier `t ∈ [0.2, 0.25]`), where its
/// columns 8–10 turn straight down: there `∂S/∂v = −ẑ` exactly, so the
/// flange's normal is the cylinder's and the walls are tangent along the
/// rim on a whole sub-span. The shared row's transversality reads only
/// the 7 interior CERT_SAMPLES feet, `t = 0.15·i`, which are sub-arc
/// joints 3, 6, …, 21, all outward. A sound certificate refuses
/// (`NotTransverse` / escalation); this row asserts that.
fn tangent_between_feet(witness: bool) -> Result<EdgeCurve<f64>, CertifyError> {
    let cylinder = ring(|_, _| Vec3::new(0.0, 0.0, 1.0));
    let flange = ring(|j, radial| {
        if (8..=10).contains(&j) {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            radial
        }
    });
    // The tangency is real: on sub-arc 4 the flange's normal is radial.
    if let Surface::Nurbs(f) = &flange {
        let u = (sub_arc_joint(4, M) + sub_arc_joint(5, M)) / 2.0;
        let d = f.ders(u, 0.0);
        let n = d.du.cross(d.dv);
        let n = n * (1.0 / n.norm());
        assert!(
            n.z.abs() < 1e-12,
            "the flange's normal is horizontal there: {n:?}"
        );
    } else {
        unreachable!()
    }
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let keys = [arena.insert(cylinder), arena.insert(flange)];
    let pair = SurfacePair::new(keys[0], keys[1]);
    let carrier = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (t0, t1) = (0.0, TURN);
    let images: [Pcurve<f64>; 2] = [0, 1].map(|i| {
        pick_arc_rim(&carrier, (t0, t1 - t0), &arena[keys[i]], band())
            .expect("the pick decides")
            .expect("the rim is a row of the wall")
    });
    let [ia, ib] = images;
    let row = SharedRow::new(pair, (keys[0], ia), (keys[1], ib)).unwrap();
    let selector = if witness {
        CurveSelector::Witness(carrier.mid_point(t0, t1))
    } else {
        CurveSelector::Structural(Structural::SharedRow(row))
    };
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection { pair, selector },
        carrier: carrier.clone(),
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify(
        spec,
        carrier.eval(t0),
        carrier.eval(t1),
        |k| arena.get(k).cloned(),
        1.0,
        band(),
        NurbsLane::certified(),
    )
}

/// See [`tangent_between_feet`]: the shared row certifies the tangent
/// walls as transverse.
#[test]
fn a_shared_row_tangent_between_its_feet_refuses() {
    let got = tangent_between_feet(false);
    assert!(
        matches!(
            got,
            Err(CertifyError::NotTransverse { .. } | CertifyError::Escalated { .. })
        ),
        "walls tangent along t ∈ [0.2, 0.25] must not certify as a transverse shared row: {:?}",
        got.map(|e| e.certificate().max_residual)
    );
}

/// The same walls under the witness selector, for comparison: the
/// NURBS-pair lane reads transversality at 31 interior feet (spacing
/// 0.0375, one of them at t = 0.225 inside the tangent sub-span).
#[test]
fn the_witness_lane_on_the_same_walls() {
    let got = tangent_between_feet(true);
    println!(
        "witness lane: {:?}",
        got.as_ref().map(|e| e.certificate().max_residual)
    );
    assert!(got.is_err(), "the witness lane refuses: {got:?}");
}
