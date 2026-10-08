//! **The analytic rung-3 edge certificate** ([`geom_brep::analytic_rung3`]):
//! a spline carrier between two analytic faces states C2's limbs in the
//! edge's own certificate — its distance from EACH operand over the
//! whole span, whatever faces store pcurve rows, and the uniqueness
//! tube — through [`EdgeCurve::certify_via`] at construction and
//! [`EdgeCurve::recertify_via`], the call tier 3 makes at rest.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};
use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::{
    AnalyticRung3Refusal, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane,
    SsiLimb, SurfaceKey, chart_pcurve_over,
};
use geom_core::spline::KnotVector;
use geom_core::{Band, Interval, Point3, Real, Vec3};
use slotmap::SlotMap;

use crate::shared::tol::band;

/// The unit quarter circle in `z = 0`, as `n` rational quadratic
/// sub-arcs joined at double knots `j/n`, each sub-arc's middle control
/// lifted to `z = h`: exactly on the cylinder `ρ = 1`, on the plane
/// `z = 0` at every knot `j/n` (every schedule parameter `k/8` when `n`
/// is a multiple of 8), and about `h/2` off it between.
fn bulged(n: usize, h: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_precision_loss)]
    let span = FRAC_PI_2 / n as f64;
    let half = 0.5 * span;
    let mut ctl = Vec::new();
    let mut w = Vec::new();
    let mut knots = vec![0.0, 0.0, 0.0];
    for j in 0..n {
        #[allow(clippy::cast_precision_loss)]
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
            #[allow(clippy::cast_precision_loss)]
            let k = (j + 1) as f64 / n as f64;
            knots.extend([k, k]);
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

fn cylinder_z<T: Real>() -> Surface<T> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: T::one(),
        u_ref: Vec3::unit_x(),
    }
}

/// `EdgeCurve::certify_via` of an `Intersection` of `s1`, `s2` over
/// `carrier`, at the scalar of its arguments, the plane × NURBS lane's
/// door value in hand or withheld.
fn certify<T: geom_core::Decide + geom_core::CertifiedBounds>(
    carrier: &NurbsCurve3<T>,
    s1: Surface<T>,
    s2: Surface<T>,
    lane: bool,
) -> Result<EdgeCurve<T>, CertifyError> {
    let c = Curve3::Nurbs(Arc::new(carrier.clone()));
    let (d0, d1) = carrier.domain();
    let (t0, t1) = (T::from_f64(d0), T::from_f64(d1));
    let mut arena: SlotMap<SurfaceKey, Surface<T>> = SlotMap::with_key();
    let (k1, k2) = (arena.insert(s1), arena.insert(s2));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: k1,
            s2: k2,
            witness: c.eval(T::from_f64(0.5 * (d0 + d1))),
        },
        carrier: c.clone(),
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify_via(
        spec,
        c.eval(t0),
        c.eval(t1),
        |k| arena.get(k).cloned(),
        band(),
        lane.then(NurbsLane::certified),
    )
}

fn lift(c: &NurbsCurve3<f64>) -> NurbsCurve3<Interval> {
    c.map_scalar(Interval::from_f64)
}

/// **The plane side of a plane × cylinder rung-3 edge is stated by the
/// edge.** The carrier is on the cylinder exactly and on the plane at
/// every schedule sample, and leaves the plane by about `h/2` between
/// them. The planar face stores no pcurve row, so no row's incidence
/// term reads that distance: the edge certificate's limb 2 against the
/// plane does, and refuses it with the measurement, at `f64` and at
/// `Interval`. The same carrier without its bulge certifies. (Ported
/// from PR 4304's review, both reviewers' plane-limb probes.)
#[test]
fn a_carrier_off_the_plane_between_samples_refuses_on_the_plane_limb() {
    let h = 2e-3;
    let carrier = bulged(16, h);
    for k in 0..=8 {
        let t = f64::from(k) / 8.0;
        assert!(
            carrier.eval(t).z.abs() < 1e-15,
            "sample {k} is on the plane"
        );
    }
    let off = carrier.eval(1.0 / 32.0).z;
    assert!(off > 5e-4, "the bulge is real between samples: {off:e}");
    let plane_limb = |e: &CertifyError| {
        matches!(
            e,
            CertifyError::AnalyticRung3(AnalyticRung3Refusal::Limb {
                operand: geom::SurfaceKind::Plane,
                limb: SsiLimb::HullSup,
                value,
            }) if *value >= off
        )
    };
    let at_f64 = certify(&carrier, plane(), cylinder_z(), true).map(|_| ());
    assert!(at_f64.as_ref().is_err_and(plane_limb), "f64: {at_f64:?}");
    let at_iv = certify(&lift(&carrier), plane(), cylinder_z(), true).map(|_| ());
    assert!(at_iv.as_ref().is_err_and(plane_limb), "Interval: {at_iv:?}");
    let direct =
        geom_brep::analytic_rung3(&carrier, carrier.domain(), &plane(), &cylinder_z(), band());
    assert!(
        matches!(
            direct,
            Err(AnalyticRung3Refusal::Limb {
                operand: geom::SurfaceKind::Plane,
                ..
            })
        ),
        "{direct:?}"
    );
    // The cylinder's own row is clean: the carrier is on the cylinder.
    let row = chart_pcurve_over(
        &Curve3::Nurbs(Arc::new(carrier.clone())),
        0.0,
        1.0,
        &cylinder_z(),
        band(),
    );
    assert!(row.is_ok(), "{row:?}");
    // Without the bulge the edge certifies at both scalars.
    let flat = bulged(16, 0.0);
    certify(&flat, plane(), cylinder_z(), true).expect("f64: the flat quarter certifies");
    certify(&lift(&flat), plane(), cylinder_z(), true)
        .expect("Interval: the flat quarter certifies");
    // With the lane withheld the limbs are not stated, and the bulge
    // passes the schedule (`work/pcert/lane-free-doors-skip-the-
    // analytic-rung3-limbs.md` holds the policy open).
    certify(&carrier, plane(), cylinder_z(), false)
        .expect("no lane: the schedule alone, which the bulge passes");
}

/// One branch of the Steinmetz pair `x² + z² = 1`, `y² + z² = 1`: the
/// ellipse `(cos θ, cos θ, sin θ)` in the plane `x = y`, over
/// `[π/2 − a, π/2 + b]`, as the affine image of a rational quadratic
/// unit-circle arc. It passes through `(0, 0, 1)`, where the other
/// branch crosses it and the two cylinders share their normal.
fn steinmetz_arc(a: f64, b: f64) -> NurbsCurve3<f64> {
    let (lo, hi) = (FRAC_PI_2 - a, FRAC_PI_2 + b);
    let (m, half) = (0.5 * (lo + hi), 0.5 * (hi - lo));
    let map = |c: f64, s: f64| Point3::new(c, c, s);
    NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            map(lo.cos(), lo.sin()),
            map(m.cos() / half.cos(), m.sin() / half.cos()),
            map(hi.cos(), hi.sin()),
        ],
        vec![1.0, half.cos(), 1.0],
    )
    .unwrap()
}

fn cylinder_about(axis: Vec3<f64>, u_ref: Vec3<f64>) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis,
        radius: 1.0,
        u_ref,
    }
}

/// **The uniqueness tube is the edge's, at construction and at rest.**
/// A carrier along one Steinmetz branch through the point where the
/// other branch crosses it: on both cylinders exactly, transverse at
/// every schedule sample, and two arcs of the pair's crossing inside
/// any tube around it. Withheld the lane, the edge certifies at the
/// schedule; with it, `certify_via` refuses on the tube, and so does
/// `recertify_via` of the lane-free certificate — the call tier 3
/// makes on a body at rest.
#[test]
fn a_carrier_through_a_crossing_of_two_branches_refuses_on_the_tube() {
    let carrier = steinmetz_arc(0.4, 0.3);
    let (s1, s2) = (
        cylinder_about(Vec3::unit_y(), Vec3::unit_x()),
        cylinder_about(Vec3::unit_x(), Vec3::unit_y()),
    );
    for k in 0..=64 {
        let p = carrier.eval(f64::from(k) / 64.0);
        let on = |r: f64| (r - 1.0).abs() < 1e-14;
        assert!(on(p.x.hypot(p.z)) && on(p.y.hypot(p.z)), "on both: {p:?}");
    }
    let tube = |e: &CertifyError| {
        matches!(
            e,
            CertifyError::AnalyticRung3(
                AnalyticRung3Refusal::TubeStraddles { .. }
                    | AnalyticRung3Refusal::TubeNotOneArc { .. }
            )
        )
    };
    let free = certify(&carrier, s1.clone(), s2.clone(), false)
        .expect("no lane: the schedule alone, where the pair crosses transversely");
    let at_construction = certify(&carrier, s1.clone(), s2.clone(), true).map(|_| ());
    assert!(
        at_construction.as_ref().is_err_and(tube),
        "{at_construction:?}"
    );
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (k1, k2) = (arena.insert(s1.clone()), arena.insert(s2.clone()));
    let _ = k2;
    let resolve = |k: SurfaceKey| Some(if k == k1 { s1.clone() } else { s2.clone() });
    let c = Curve3::Nurbs(Arc::new(carrier.clone()));
    let at_rest = free
        .recertify_via(
            c.eval(0.0),
            c.eval(1.0),
            resolve,
            band(),
            Some(NurbsLane::certified()),
        )
        .map(|_| ());
    assert!(at_rest.as_ref().is_err_and(tube), "{at_rest:?}");
    let _ = FRAC_PI_4;
}

/// **The refusal speaks of the analytic pair.** Its words name the
/// tube's verdict on the two surfaces, not another lane's pair.
#[test]
fn the_analytic_refusal_names_its_own_pair() {
    let carrier = steinmetz_arc(0.4, 0.3);
    let err = geom_brep::analytic_rung3(
        &carrier,
        carrier.domain(),
        &cylinder_about(Vec3::unit_y(), Vec3::unit_x()),
        &cylinder_about(Vec3::unit_x(), Vec3::unit_y()),
        band(),
    )
    .expect_err("two branches in the tube");
    let words = CertifyError::AnalyticRung3(err).to_string();
    assert!(
        words.contains("analytic") && !words.contains("NURBS") && !words.contains("plane"),
        "{words}"
    );
    let _: Band = band();
}

/// `EdgeCurve::certify_via` of an `Intersection` of `s1`, `s2` over
/// `carrier` restricted to `[t0, t1]`, the lane in hand.
fn certify_over(
    carrier: &NurbsCurve3<f64>,
    (t0, t1): (f64, f64),
    s1: Surface<f64>,
    s2: Surface<f64>,
) -> Result<(), CertifyError> {
    let c = Curve3::Nurbs(Arc::new(carrier.clone()));
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (k1, k2) = (arena.insert(s1), arena.insert(s2));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: k1,
            s2: k2,
            witness: c.eval(0.5 * (t0 + t1)),
        },
        carrier: c.clone(),
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify_via(
        spec,
        c.eval(t0),
        c.eval(t1),
        |k| arena.get(k).cloned(),
        band(),
        Some(NurbsLane::certified()),
    )
    .map(|_| ())
}

/// **The edge's limbs read the edge's own interval.** A carrier flat on
/// the plane over `[0, ½]` and bulged off it over `(½, 1]`: the edge
/// over `[0, ½]` certifies (the net past its end is not the edge's — a
/// split edge keeps its parent's carrier), and the edge over `[½, 1]`
/// refuses on the plane limb. (Ported from PR 4304's confirming
/// review.)
#[test]
fn an_edges_limbs_read_only_its_own_interval_of_the_carrier() {
    let bumped = bulged(16, 2e-3);
    let flat = bulged(16, 0.0);
    // Sub-arcs 0..8 (controls 0..=16) from the flat quarter, the rest
    // from the bulged one: they share the knot vector and weights.
    let ctl: Vec<_> = flat.control()[..17]
        .iter()
        .chain(&bumped.control()[17..])
        .copied()
        .collect();
    let carrier = NurbsCurve3::new(flat.knots().clone(), ctl, flat.weights().to_vec()).unwrap();
    for k in 0..=64 {
        let t = 0.5 * f64::from(k) / 64.0;
        assert!(carrier.eval(t).z.abs() < 1e-15, "flat on [0, ½] at {t}");
    }
    let off = carrier.eval(0.5 + 1.0 / 32.0).z;
    assert!(off > 5e-4, "bulged past ½: {off:e}");
    certify_over(&carrier, (0.0, 0.5), plane(), cylinder_z())
        .expect("the edge's own interval is on both surfaces");
    let past = certify_over(&carrier, (0.5, 1.0), plane(), cylinder_z());
    assert!(
        matches!(
            past,
            Err(CertifyError::AnalyticRung3(AnalyticRung3Refusal::Limb {
                operand: geom::SurfaceKind::Plane,
                value,
                ..
            })) if value >= off
        ),
        "{past:?}"
    );
}

/// **The tube reads the edge's own interval.** The Steinmetz arc whose
/// whole carrier passes the crossing of the two branches refuses on the
/// crossing's verdict (`TubeStraddles`); an edge over the stretch
/// before the crossing never reads it. That stretch does not certify
/// either: the tube's one-arc check reads an invalid margin on every
/// Steinmetz stretch, the crossing's or not
/// (`work/pcert/the-tube-reads-an-invalid-margin-on-a-steinmetz-branch-clear-of-its-crossing.md`).
#[test]
fn the_tube_reads_only_the_edges_own_interval_of_the_carrier() {
    let carrier = steinmetz_arc(0.4, 0.3);
    let (s1, s2) = (
        cylinder_about(Vec3::unit_y(), Vec3::unit_x()),
        cylinder_about(Vec3::unit_x(), Vec3::unit_y()),
    );
    let whole = certify_over(&carrier, (0.0, 1.0), s1.clone(), s2.clone());
    assert!(
        matches!(
            whole,
            Err(CertifyError::AnalyticRung3(
                AnalyticRung3Refusal::TubeStraddles { .. }
            ))
        ),
        "{whole:?}"
    );
    for stretch in [(0.0, 0.3), (0.7, 1.0)] {
        let cut = certify_over(&carrier, stretch, s1.clone(), s2.clone());
        assert!(
            !matches!(
                cut,
                Err(CertifyError::AnalyticRung3(
                    AnalyticRung3Refusal::TubeStraddles { .. }
                ))
            ),
            "{stretch:?} holds no crossing: {cut:?}"
        );
    }
}

/// Confirming-review probe 2 (4304): a Steinmetz arc clear of the
/// crossing, as its OWN carrier (no cut): does the tube certify, or is
/// the invalid margin pre-existing (not the cut's)?
#[test]
fn confirm_probe_steinmetz_stretch_as_its_own_carrier() {
    let (s1, s2) = (
        cylinder_about(Vec3::unit_y(), Vec3::unit_x()),
        cylinder_about(Vec3::unit_x(), Vec3::unit_y()),
    );
    for (a, b) in [(-0.1, 0.4), (0.6, -0.2), (-0.2, 0.9)] {
        let carrier = steinmetz_arc(a, b);
        let got = certify_over(&carrier, carrier.domain(), s1.clone(), s2.clone());
        let direct = geom_brep::analytic_rung3(&carrier, carrier.domain(), &s1, &s2, band());
        eprintln!("CONFIRM-PROBE-2 steinmetz({a},{b}) whole domain: certify_via={got:?} direct={direct:?}");
    }
}
