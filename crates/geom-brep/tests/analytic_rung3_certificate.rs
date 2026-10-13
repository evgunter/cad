//! **The analytic rung-3 edge certificate** ([`geom_brep::analytic_rung3`]):
//! a spline carrier between two analytic faces states C2's limbs in the
//! edge's own certificate — its distance from EACH operand over the
//! whole span, whatever faces store pcurve rows, and the uniqueness
//! tube — through [`EdgeCurve::certify_via`] at construction and
//! [`EdgeCurve::recertify_via`], the call tier 3 makes at rest.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::FRAC_PI_2;
use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::recourse::Reading;
use geom_brep::{
    AnalyticRung3Refusal, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane,
    Rung3Limb, SurfaceKey, SurfacePair, chart_pcurve_over,
};
use geom_core::spline::KnotVector;
use geom_core::{
    Band, FileCoincidence, Interval, KERNEL_OR_FILE_DEFECT_ENDING, Point3, Real, Vec3,
};
use slotmap::SlotMap;

use crate::shared::tol::band;
use geom_core::test_support::upper;

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
            pair: SurfacePair::new(k1, k2),
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

/// The import door's words on a measured miss of a fitted carrier that
/// lies within the file's `eps_in`.
fn in_file_words(eps_in: f64) -> String {
    format!(
        "This miss lies beyond the tolerance and within the file's declared coincidence \
         distance ε_in = {eps_in:e} m, and may be the kernel's own approximation. Recourse: \
         re-export the file more precisely, or, as a stopgap, set the tolerance to \
         ε_in = {eps_in:e} m; this refusal may indicate a kernel bug worth reporting"
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
/// plane does. A uniform part break lands on the bulge's peak at `1/32`,
/// where the carrier's certified distance from the plane is past the
/// band, so it refuses on that measured miss, at `f64` and at
/// `Interval`, which the import door reads. The same carrier without its
/// bulge certifies.
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
    // A lower bound on the distance at a break: at most the peak's, and
    // close to it, since a break lands on the peak.
    let plane_limb = |e: &CertifyError| {
        matches!(
            e,
            CertifyError::AnalyticRung3(AnalyticRung3Refusal::Limb {
                operand: geom::SurfaceKind::Plane,
                limb: Rung3Limb::HullValue,
                margin,
                ..
            }) if upper(*margin) <= off && upper(*margin) >= 0.99 * off
        )
    };
    let at_f64 = certify(&carrier, plane(), cylinder_z(), true).map(|_| ());
    assert!(at_f64.as_ref().is_err_and(plane_limb), "f64: {at_f64:?}");
    let at_iv = certify(&lift(&carrier), plane(), cylinder_z(), true).map(|_| ());
    assert!(at_iv.as_ref().is_err_and(plane_limb), "Interval: {at_iv:?}");
    // The limb refuses on the miss itself, measured: at rest a stored
    // edge off its face is a contradiction, and the import door names
    // the fit's approximation where ε_in holds the miss and the
    // contradiction where it does not.
    for refused in [&at_f64, &at_iv] {
        let Err(e @ CertifyError::AnalyticRung3(AnalyticRung3Refusal::Limb { margin, .. })) =
            refused
        else {
            panic!("the plane limb refused above: {refused:?}")
        };
        let value = upper(*margin);
        assert_eq!(
            e.ending(Reading::AtRest).as_deref(),
            Some(KERNEL_OR_FILE_DEFECT_ENDING),
            "{e:?}"
        );
        assert_eq!(
            e.ending(FileCoincidence::new(2.0 * value)),
            Some(in_file_words(2.0 * value)),
            "{e:?}"
        );
        assert_eq!(
            e.ending(FileCoincidence::new(0.5 * value)).as_deref(),
            Some(KERNEL_OR_FILE_DEFECT_ENDING),
            "{e:?}"
        );
    }
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
    // With the lane withheld the door refuses before it reads the
    // carrier, the bulged and the flat quarter alike.
    for (what, c) in [("bulged", &carrier), ("flat", &flat)] {
        let free = certify(c, plane(), cylinder_z(), false).map(|_| ());
        assert_eq!(
            free,
            Err(CertifyError::NurbsLaneNotSupplied),
            "no lane, {what}"
        );
    }
}

/// **The refusal for want of the lane comes first.** A rung-3 edge
/// with a schedule defect — a backward interval, then an endpoint off
/// the carrier — refuses `NurbsLaneNotSupplied` at a lane-free door,
/// not the defect: the door checked nothing. With the lane in hand the
/// same edge refuses on the defect itself.
#[test]
fn a_lane_free_door_refuses_for_the_lane_before_any_schedule_defect() {
    let c = Curve3::Nurbs(Arc::new(bulged(16, 0.0)));
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (k1, k2) = (arena.insert(plane()), arena.insert(cylinder_z()));
    let spec = |t0: f64, t1: f64| EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(k1, k2),
            witness: c.eval(0.5),
        },
        carrier: c.clone(),
        param_start: t0,
        param_end: t1,
    };
    let off = Vec3::new(0.0, 0.0, 0.25);
    for (defect, spec, start, end) in [
        ("backward", spec(1.0, 0.0), c.eval(1.0), c.eval(0.0)),
        (
            "endpoint off",
            spec(0.0, 1.0),
            c.eval(0.0) + off,
            c.eval(1.0),
        ),
    ] {
        let surfaces = |k| arena.get(k).cloned();
        assert_eq!(
            EdgeCurve::certify(spec.clone(), start, end, surfaces, band()).map(|_| ()),
            Err(CertifyError::NurbsLaneNotSupplied),
            "{defect}: lane-free"
        );
        let with_lane = EdgeCurve::certify_via(
            spec,
            start,
            end,
            surfaces,
            band(),
            Some(NurbsLane::certified()),
        )
        .map(|_| ());
        let on_the_defect = match defect {
            "backward" => matches!(with_lane, Err(CertifyError::IntervalNotForward { .. })),
            _ => matches!(with_lane, Err(CertifyError::ResidualExceeded { .. })),
        };
        assert!(on_the_defect, "{defect}: with the lane: {with_lane:?}");
    }
}

/// **Every lane-free door refuses an analytic rung-3 edge, typed.** C2's
/// limbs are the edge certificate's own, so a door that cannot state
/// them certifies nothing about the edge: [`EdgeCurve::certify`],
/// [`EdgeCurve::recertify`], either `_via` door handed `None`, and a
/// scalar that holds no lane (the dual) each answer
/// `NurbsLaneNotSupplied` on a carrier the lane certifies clean.
#[test]
fn lane_free_doors_refuse_an_analytic_rung3_edge() {
    let carrier = bulged(16, 0.0);
    let c = Curve3::Nurbs(Arc::new(carrier.clone()));
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (k1, k2) = (arena.insert(plane()), arena.insert(cylinder_z()));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(k1, k2),
            witness: c.eval(0.5),
        },
        carrier: c.clone(),
        param_start: 0.0,
        param_end: 1.0,
    };
    let (start, end) = (c.eval(0.0), c.eval(1.0));
    let surfaces = |k| arena.get(k).cloned();
    let refused = Err(CertifyError::NurbsLaneNotSupplied);
    let certified = EdgeCurve::certify_via(
        spec.clone(),
        start,
        end,
        surfaces,
        band(),
        Some(NurbsLane::certified()),
    )
    .expect("the lane certifies the flat quarter");
    assert_eq!(
        EdgeCurve::certify(spec, start, end, surfaces, band()).map(|_| ()),
        refused,
        "EdgeCurve::certify"
    );
    assert_eq!(
        certified
            .recertify(start, end, surfaces, band())
            .map(|_| ()),
        refused,
        "EdgeCurve::recertify"
    );
    assert_eq!(
        certified
            .recertify_via(start, end, surfaces, band(), None)
            .map(|_| ()),
        refused,
        "recertify_via(None)"
    );
    certified
        .recertify_via(start, end, surfaces, band(), Some(NurbsLane::certified()))
        .expect("recertify_via with the lane re-derives the certificate");

    // The dual: a scalar with no lane to hand.
    use geom_core::Dual64;
    let dual = carrier.map_scalar(Dual64::from_f64);
    let dc = Curve3::Nurbs(Arc::new(dual));
    let mut dual_arena: SlotMap<SurfaceKey, Surface<Dual64>> = SlotMap::with_key();
    let (d1, d2) = (dual_arena.insert(plane()), dual_arena.insert(cylinder_z()));
    let half = Dual64::from_f64(0.5);
    let dual_spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(d1, d2),
            witness: dc.eval(half),
        },
        carrier: dc.clone(),
        param_start: Dual64::from_f64(0.0),
        param_end: Dual64::from_f64(1.0),
    };
    assert_eq!(
        EdgeCurve::certify(
            dual_spec,
            dc.eval(Dual64::from_f64(0.0)),
            dc.eval(Dual64::from_f64(1.0)),
            |k| dual_arena.get(k).cloned(),
            band(),
        )
        .map(|_| ()),
        refused,
        "Dual64"
    );
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
/// any tube around it. With the lane `certify_via` refuses on the tube;
/// withheld it, the door refuses for want of the lane, so no door mints
/// the edge.
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
    let free = certify(&carrier, s1.clone(), s2.clone(), false).map(|_| ());
    assert_eq!(free, Err(CertifyError::NurbsLaneNotSupplied), "no lane");
    let at_construction = certify(&carrier, s1, s2, true).map(|_| ());
    assert!(
        at_construction.as_ref().is_err_and(tube),
        "{at_construction:?}"
    );
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
            pair: SurfacePair::new(k1, k2),
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
/// refuses on the plane limb's measured miss. (Ported from PR 4304's
/// confirming review.)
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
                limb: Rung3Limb::HullValue,
                margin,
            })) if upper(margin) <= off && upper(margin) >= 0.99 * off
        ),
        "{past:?}"
    );
}

/// **A rational carrier's tube reads its whole net.** The Steinmetz arc
/// is rational with unequal weights, and no piece of it has `f64`
/// weights at an enclosure scalar, so the tube reads the whole carrier
/// (`edge_nurbs`'s `edge_piece`): an edge over a stretch clear of the
/// crossing still reads it, and refuses on the crossing's verdict —
/// sound, conservative, and filed
/// (`work/pcert/the-tube-reads-a-rational-carriers-whole-net.md`).
#[test]
fn a_rational_carriers_tube_reads_its_whole_net() {
    let carrier = steinmetz_arc(0.4, 0.3);
    let (s1, s2) = (
        cylinder_about(Vec3::unit_y(), Vec3::unit_x()),
        cylinder_about(Vec3::unit_x(), Vec3::unit_y()),
    );
    for stretch in [(0.0, 1.0), (0.0, 0.3), (0.7, 1.0)] {
        let got = certify_over(&carrier, stretch, s1.clone(), s2.clone());
        assert!(
            matches!(
                got,
                Err(CertifyError::AnalyticRung3(
                    AnalyticRung3Refusal::TubeStraddles { .. }
                ))
            ),
            "{stretch:?}: {got:?}"
        );
    }
}

/// The `x` axis as one degree-6 span, `x = t`, `y = 0`, its `z`
/// controls zero at the ends and alternating `∓a` between, so that
/// `z(t) = a·((1 − 2t)⁶ − (1 − t)⁶ − t⁶)`, whose largest magnitude is
/// `0.27204…·a`: a net much wider than its curve.
fn oscillating(a: f64) -> NurbsCurve3<f64> {
    let ctl: Vec<_> = (0..=6)
        .map(|i| {
            let z = match i {
                0 | 6 => 0.0,
                i if i % 2 == 0 => a,
                _ => -a,
            };
            Point3::new(f64::from(i) / 6.0, 0.0, z)
        })
        .collect();
    let knots = [[0.0; 7], [1.0; 7]].concat();
    NurbsCurve3::new(KnotVector::clamped(knots, 6).unwrap(), ctl, vec![1.0; 7]).unwrap()
}

/// **Limb 2 halves the parts it cannot clear.** The oscillating span's
/// fixed parts bound its distance from `z = 0` about 1.3 % above its
/// peak, so a peak 0.3 % inside the band is past the band on those
/// parts, and the edge certifies only on the halved ones. A peak twenty
/// times the band's zero refuses on the plane, on a break's measured
/// distance.
#[test]
fn limb_two_halves_the_parts_whose_bound_does_not_clear() {
    let zero = band().zero();
    let side = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_z(),
    };
    let peak = |c: &NurbsCurve3<f64>| {
        (0..=100_000)
            .map(|k| c.eval(f64::from(k) / 1e5).z.abs())
            .fold(0.0, f64::max)
    };
    let inside = oscillating(3.665 * zero);
    assert!(peak(&inside) < 0.998 * zero, "{:e}", peak(&inside));
    let got = geom_brep::analytic_rung3(&inside, inside.domain(), &plane(), &side, band());
    assert!(got.is_ok(), "f64: {got:?}");
    let lifted = lift(&inside);
    let (d0, d1) = lifted.domain();
    let got = geom_brep::analytic_rung3(
        &lifted,
        (Interval::from_f64(d0), Interval::from_f64(d1)),
        &plane(),
        &side.map_scalar(Interval::from_f64),
        band(),
    );
    assert!(got.is_ok(), "Interval: {got:?}");
    let past = oscillating(20.0 / 0.272_044 * zero);
    let got = geom_brep::analytic_rung3(&past, past.domain(), &plane(), &side, band());
    assert!(
        matches!(
            got,
            Err(AnalyticRung3Refusal::Limb {
                operand: geom::SurfaceKind::Plane,
                limb: Rung3Limb::HullValue,
                margin,
            }) if upper(margin) > 10.0 * zero && upper(margin) <= 20.0 * zero
        ),
        "{got:?}"
    );
}

/// The `x` axis as one degree-9 span, `x = t`, `y = 0`, with
/// `z(t) = a·∏ⱼ (t − j/8)` over `j = 0..=8`: zero at every uniform part
/// break `j/8`, and off the plane between them, its peak in `(0, 1/8)`.
fn waved_between_the_breaks(a: f64) -> NurbsCurve3<f64> {
    // Power coefficients of the product, then its Bernstein ones,
    // `bᵢ = Σ_{k ≤ i} C(i, k)/C(9, k)·cₖ`.
    let mut c = vec![a];
    for j in 0..=8 {
        let root = f64::from(j) / 8.0;
        let mut next = vec![0.0; c.len() + 1];
        for (k, ck) in c.iter().enumerate() {
            next[k + 1] += ck;
            next[k] -= root * ck;
        }
        c = next;
    }
    let choose = |n: u32, k: u32| (1..=k).fold(1.0, |m, i| m * f64::from(n + 1 - i) / f64::from(i));
    let ctl: Vec<_> = (0..=9u32)
        .map(|i| {
            let z = (0..=i)
                .map(|k| choose(i, k) / choose(9, k) * c[k as usize])
                .sum();
            Point3::new(f64::from(i) / 9.0, 0.0, z)
        })
        .collect();
    let knots = [[0.0; 10], [1.0; 10]].concat();
    NurbsCurve3::new(KnotVector::clamped(knots, 9).unwrap(), ctl, vec![1.0; 10]).unwrap()
}

/// **Limb 2 halves, then stops on a break's measured miss.** The carrier
/// is on the plane at every uniform part break and well past the band's
/// escalation between them, so the first hull neither clears nor names
/// a miss. The halved parts' breaks `(2j + 1)/16` are past it, so the
/// limb refuses on the plane there, on `HullValue`, at `f64` and at
/// `Interval`.
#[test]
fn limb_two_halves_then_stops_on_a_breaks_miss() {
    let band = band();
    let side = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_z(),
    };
    // `∏ⱼ |1/16 − j/8|`, the product at the first halved break.
    let at_a_sixteenth = (1..=8).fold(1.0 / 16.0, |m, j| m * f64::from(2 * j - 1) / 16.0);
    let carrier = waved_between_the_breaks(50.0 * band.zero() / at_a_sixteenth);
    let z = |t: f64| carrier.eval(t).z.abs();
    for j in 0..=8 {
        let t = f64::from(j) / 8.0;
        assert!(
            z(t) < 1e-3 * band.zero(),
            "on the plane at the break {t}: {:e}",
            z(t)
        );
    }
    let halved = (0..8)
        .map(|j| z(f64::from(2 * j + 1) / 16.0))
        .fold(0.0, f64::max);
    let peak = (0..=100_000)
        .map(|k| z(f64::from(k) / 1e5))
        .fold(0.0, f64::max);
    assert!(
        halved > 2.0 * band.escalate(),
        "past the band at a halved break: {halved:e}"
    );
    let refused = |got: &Result<(), AnalyticRung3Refusal>| {
        matches!(
            got,
            Err(AnalyticRung3Refusal::Limb {
                operand: geom::SurfaceKind::Plane,
                limb: Rung3Limb::HullValue,
                margin,
            }) if upper(*margin) > band.escalate() && upper(*margin) <= peak
        )
    };
    let got = geom_brep::analytic_rung3(&carrier, carrier.domain(), &plane(), &side, band);
    assert!(refused(&got), "f64: {got:?}");
    let lifted = lift(&carrier);
    let (d0, d1) = lifted.domain();
    let got = geom_brep::analytic_rung3(
        &lifted,
        (Interval::from_f64(d0), Interval::from_f64(d1)),
        &plane(),
        &side.map_scalar(Interval::from_f64),
        band,
    );
    assert!(refused(&got), "Interval: {got:?}");
}

/// **A cone or a torus operand refuses the tube by name, once limb 2
/// holds.** The unit circle in `z = 0` lies on the plane exactly, on
/// the torus `R = 5/4, r = 1/4` as its inner equator, and on the cone
/// with apex `(0, 0, −1)` and half-angle `π/4`; its limbs hold against
/// both operands of each pair. The tube's ℝ³ enclosure has no arm for
/// either kind, so every box's margin is refused at every rung; the
/// edge says which operand it has no enclosure of rather than reading
/// that refusal as a sliver.
#[test]
fn a_cone_or_torus_operand_refuses_the_tube_by_name() {
    let carrier = bulged(8, 0.0);
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 1.25,
        minor_radius: 0.25,
        u_ref: Vec3::unit_x(),
    };
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, -1.0),
        axis: Vec3::unit_z(),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: Vec3::unit_x(),
    };
    for (wall, kind) in [
        (torus, geom::SurfaceKind::Torus),
        (cone, geom::SurfaceKind::Cone),
    ] {
        let got = certify(&carrier, plane(), wall, true).map(|_| ());
        assert_eq!(
            got,
            Err(CertifyError::AnalyticRung3(
                AnalyticRung3Refusal::NoTubeEnclosure { operand: kind }
            )),
            "{kind:?}"
        );
    }
}

/// **A dense export-shaped spline certifies where the tube has an
/// enclosure.** The STEP writer's stand-in for a spiric is a cubic
/// interpolant through 513 points of the section, 510 spans, which the
/// tube reads as a chain of 510 boxes at a ladder whose widest rung is
/// an eighth of the control net's diameter. The same construction on a
/// plane × cylinder section at the vessel's scale (radius `9/128` m,
/// the arc `θ ∈ [−0.89, 0.89]`) certifies, limbs and tube: neither the
/// chain's length nor the ladder's scale refuses such a carrier.
#[test]
fn a_dense_interpolant_of_a_section_certifies_its_tube() {
    let r = 9.0 / 128.0;
    let n = Vec3::new(0.0, 0.6, 0.8);
    let oblique = Surface::Plane {
        origin: Point3::origin(),
        normal: n,
        u_ref: Vec3::unit_x(),
    };
    let wall = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let nodes = 512;
    #[allow(clippy::cast_precision_loss)]
    let params: Vec<f64> = (0..=nodes).map(|i| i as f64 / nodes as f64).collect();
    let points: Vec<Point3<f64>> = params
        .iter()
        .map(|f| {
            let a = -0.89 + 1.78 * f;
            let (x, y) = (r * a.cos(), r * a.sin());
            Point3::new(x, y, -n.y * y / n.z)
        })
        .collect();
    let spline = NurbsCurve3::interpolate_with_params(&points, 3, &params).unwrap();
    assert_eq!(spline.control().len(), nodes + 1, "the export's net size");
    let got = certify(&spline, oblique, wall, true).map(|_| ());
    assert_eq!(got, Ok(()));
}
