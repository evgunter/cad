//! Review probes for PR 4304 (reviewer lane, frozen head fdc8ea9b).

use std::f64::consts::FRAC_PI_2;
use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::ssi::{SsiOperand, TubeScale};
use geom_brep::{
    EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, FittedLane, NurbsLane, Pcurve, PcurveCache,
    SurfaceKey, chart_pcurve_over,
};
use geom_core::spline::KnotVector;
use geom_core::tolerance::Tol;
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};
use slotmap::SlotMap;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The unit quarter circle in `z = 0`, as `n` rational quadratic
/// sub-arcs joined at double knots `j/n`, with each sub-arc's middle
/// control lifted to `z = h`. Exactly on the cylinder `ρ = 1`; on the
/// plane `z = 0` at every knot `j/n` (so at every schedule parameter
/// `k/8` when `n` is a multiple of 8) and `≈ h/2` off it between.
fn bulged(n: usize, h: f64) -> NurbsCurve3<f64> {
    let span = FRAC_PI_2 / n as f64;
    let half = 0.5 * span;
    let mut ctl = Vec::new();
    let mut w = Vec::new();
    let mut knots = vec![0.0, 0.0, 0.0];
    for j in 0..n {
        let a = span * j as f64;
        if j == 0 {
            ctl.push(Point3::new(a.cos(), a.sin(), 0.0));
            w.push(1.0);
        }
        let m = a + half;
        ctl.push(Point3::new(m.cos() / half.cos(), m.sin() / half.cos(), h));
        w.push(half.cos());
        let b = a + span;
        ctl.push(Point3::new(b.cos(), b.sin(), 0.0));
        w.push(1.0);
        if j + 1 < n {
            let k = (j + 1) as f64 / n as f64;
            knots.push(k);
            knots.push(k);
        }
    }
    knots.extend([1.0, 1.0, 1.0]);
    NurbsCurve3::new(KnotVector::clamped(knots, 2).unwrap(), ctl, w).unwrap()
}

fn plane<T: Real>() -> Surface<T> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn cylinder<T: Real>() -> Surface<T> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: T::one(),
        u_ref: Vec3::unit_x(),
    }
}

/// **P1 — the plane side of a plane × analytic rung-3 edge has no
/// between-samples statement anywhere.** `rung3_tube`'s docs say limbs 1
/// and 2 are "each face's pcurve row's incidence term"; a planar face
/// stores no row, so the carrier's distance from the PLANE is read at the
/// nine schedule samples only. Before the PR the analytic face's
/// `OnLocusHull` row ran `certify_rung3` (limbs All) against the plane
/// mate and its hull limb refused this carrier.
#[test]
fn p1_plane_side_hull_is_unasked() {
    let b = band();
    let h = 2e-3;
    let carrier = bulged(16, h);
    // The bulge is real: the carrier's distance from the plane at a
    // sub-arc's middle.
    let off = carrier.eval(1.0 / 32.0).z;
    eprintln!("[p1] distance from the plane between samples: {off:e} (band {:e})", b.zero());
    assert!(off > 1e-4);
    let c3 = Curve3::Nurbs(Arc::new(carrier.clone()));
    // On the plane at every schedule parameter.
    for k in 0..=8 {
        let t = f64::from(k) / 8.0;
        assert!(carrier.eval(t).z.abs() < 1e-15, "sample {k} off the plane");
    }

    // The edge certificate, lane in hand (the tube runs), at f64 and Interval.
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let s1 = arena.insert(plane());
    let s2 = arena.insert(cylinder());
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1,
            s2,
            witness: c3.eval(0.5),
        },
        carrier: c3.clone(),
        param_start: 0.0,
        param_end: 1.0,
    };
    let edge = EdgeCurve::certify_via(
        spec,
        c3.eval(0.0),
        c3.eval(1.0),
        |k| arena.get(k).cloned(),
        b,
        Some(NurbsLane::certified()),
    );
    eprintln!("[p1] f64 edge certify_via(lane): {:?}", edge.as_ref().map(|_| "Ok"));

    let lift = |x: f64| Interval::from_f64(x);
    let ci = c3.map_scalar(lift);
    let mut arena_i: SlotMap<SurfaceKey, Surface<Interval>> = SlotMap::with_key();
    let i1 = arena_i.insert(plane());
    let i2 = arena_i.insert(cylinder());
    let spec_i = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: i1,
            s2: i2,
            witness: ci.eval(lift(0.5)),
        },
        carrier: ci.clone(),
        param_start: lift(0.0),
        param_end: lift(1.0),
    };
    let edge_i = EdgeCurve::certify_via(
        spec_i,
        ci.eval(lift(0.0)),
        ci.eval(lift(1.0)),
        |k| arena_i.get(k).cloned(),
        b,
        Some(NurbsLane::certified()),
    );
    eprintln!("[p1] Interval edge certify_via(lane): {:?}", edge_i.as_ref().map(|_| "Ok"));

    // The cylinder face's projected row, at Interval with the door.
    let (t0, t1) = (lift(0.0), lift(1.0));
    let cyl_i: Surface<Interval> = cylinder();
    let row = chart_pcurve_over(&ci, t0, t1, &cyl_i, b).expect("cylinder image");
    let Pcurve::Projected(p) = row else { panic!("{row:?}") };
    let cache = PcurveCache::certify_projected(*p, t0, t1, &ci, &cyl_i, b, Some(FittedLane::certified()));
    eprintln!(
        "[p1] Interval cylinder row: {:?}",
        cache.as_ref().map(|c| c.certificate().envelope.hi())
    );

    // What the retired OnLocusHull row asked: limbs All against the plane mate.
    let pl: Surface<f64> = plane();
    let cy: Surface<f64> = cylinder();
    let full = geom_brep::certify_rung3(
        &carrier,
        None,
        &SsiOperand::Analytic(&pl),
        &SsiOperand::Analytic(&cy),
        TubeScale::uniform(1.5),
        b,
    );
    eprintln!("[p1] certify_rung3 (limbs All): {:?}", full.as_ref().map(|_| "Ok"));
    let tube = geom_brep::rung3_tube(&carrier, &pl, &cy, b);
    eprintln!("[p1] rung3_tube (limb 3 alone): {tube:?}");

    assert!(edge.is_ok() && edge_i.is_ok() && cache.is_ok() && full.is_err(),
        "P1 premise did not hold — see the printed lines");
}
