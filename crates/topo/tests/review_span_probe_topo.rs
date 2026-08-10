//! R1 review probe for PR #306, claim 3 (the deviation's blast
//! radius): a REGULAR rational carrier that returns near its start in
//! space must meter positively (reversal is not disqualifying), must
//! certify through the ParamSpan gate, and `split_edge`'s interiority
//! must stay honest in ARC LENGTH — including a split parameter whose
//! point is spatially near the far endpoint.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeCurveSpec;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use geom_curves::{Curve3, NurbsCurve3};
use geom_surfaces::Surface;

/// A 359-degree arc of the unit circle in z = 0: three rational
/// quadratic spans of 119.67 degrees each. Speed is bounded away from
/// zero everywhere, yet the end point sits 0.0175 from the start.
fn near_closed_arc() -> NurbsCurve3<f64> {
    let total = 359.0f64.to_radians();
    let seg = total / 3.0;
    let w = (seg / 2.0).cos();
    let pt = |a: f64| Point3::new(a.cos(), a.sin(), 0.0);
    let apex = |a0: f64, a1: f64| {
        let m = (a0 + a1) / 2.0;
        Point3::new(m.cos() / w, m.sin() / w, 0.0)
    };
    let kv = KnotVector::clamped(
        vec![
            0.0,
            0.0,
            0.0,
            1.0 / 3.0,
            1.0 / 3.0,
            2.0 / 3.0,
            2.0 / 3.0,
            1.0,
            1.0,
            1.0,
        ],
        2,
    )
    .unwrap();
    NurbsCurve3::new(
        kv,
        vec![
            pt(0.0),
            apex(0.0, seg),
            pt(seg),
            apex(seg, 2.0 * seg),
            pt(2.0 * seg),
            apex(2.0 * seg, total),
            pt(total),
        ],
        vec![1.0, w, 1.0, w, 1.0, w, 1.0],
    )
    .unwrap()
}

#[test]
fn a_returning_regular_carrier_certifies_and_splits_honestly() {
    let carrier = std::sync::Arc::new(near_closed_arc());
    let (h0, h1) = carrier.domain();
    let (p0, p1) = (carrier.eval(h0), carrier.eval(h1));
    assert!(
        (p1 - p0).norm() < 0.02,
        "fixture: the end must sit close to the start, got {}",
        (p1 - p0).norm()
    );
    // Regular: dense-sampled speed stays away from zero; the meter is
    // positive and sound.
    let m = carrier.speed_lower_bound();
    assert!(m > 0.0, "reversal must not disqualify a regular carrier: {m}");
    let mut lo = f64::INFINITY;
    for i in 0..=8000 {
        let t = h0 + (h1 - h0) * f64::from(i) / 8000.0;
        lo = lo.min(carrier.deriv(t).norm());
    }
    assert!(m <= lo + 1e-9, "unsound: meter {m} > true min {lo}");

    // ParamSpan consumer: the edge certifies end to end (mev runs the
    // full certify gate, `nurbs_span_meter` included).
    let mut body = topo::Body::<f64>::new();
    let seed = body.mvfs(p0).unwrap();
    let sph = body
        .set_face_surface(
            seed.face,
            topo::FaceSurface::New(Surface::Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
    let anchor = body.mvfs(p1).unwrap();
    let plane = body
        .set_face_surface(
            anchor.face,
            topo::FaceSurface::New(Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
    let mid = h0 + (h1 - h0) * 0.5;
    let made = body
        .mev(
            topo::MevSite::Lone { r#loop: seed.r#loop },
            p1,
            EdgeCurveSpec {
                description: geom_brep::EdgeGeometry::Intersection {
                    s1: sph,
                    s2: plane,
                    witness: carrier.eval(mid),
                },
                carrier: Curve3::Nurbs(carrier.clone()),
                param_start: h0,
                param_end: h1,
            },
        )
        .expect("a returning-but-regular rational edge certifies under the ratified contract");

    // Split near the far end: t = 0.995 puts the new vertex at arc
    // angle ~357.2 deg — spatially 0.049 from the START vertex, yet
    // arc-length-interior by a wide metered margin. The contract says
    // this must be accepted, and both children must re-certify with
    // exact endpoint pinning.
    let t_split = h0 + (h1 - h0) * 0.995;
    let created = body
        .split_edge(made.edge, t_split)
        .expect("arc-length interiority is the contract, not chord distance");
    let c1 = body
        .get_curve_geom(created.first_curve)
        .and_then(|g| g.certified())
        .unwrap();
    let c2 = body
        .get_curve_geom(created.second_curve)
        .and_then(|g| g.certified())
        .unwrap();
    assert_eq!(c1.params(), (h0, t_split));
    assert_eq!(c2.params(), (t_split, h1));
    let p_split = *body
        .get_point(body.get_vertex(created.vertex).unwrap().point)
        .unwrap();
    assert!(p_split.distance(carrier.eval(t_split)) == 0.0);
    // The honest-margin invariant on the SHORT child: its parameter
    // span, metered, must not overstate its true arc.
    let m2 = match c2.carrier() {
        Curve3::Nurbs(n) => n.speed_lower_bound(),
        _ => panic!("child carrier stays NURBS"),
    };
    assert!(m2 > 0.0);
    let span = h1 - t_split;
    let arc: f64 = (0..2000)
        .map(|i| {
            let a = t_split + span * f64::from(i) / 2000.0;
            let b = t_split + span * f64::from(i + 1) / 2000.0;
            (carrier.eval(b) - carrier.eval(a)).norm()
        })
        .sum();
    assert!(
        span * m2 <= arc + 1e-9,
        "metered margin {} overstates the child arc {arc}",
        span * m2
    );
}
