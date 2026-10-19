//! **A curve's knot algebra at an enclosure scalar (C6 S4).** At
//! `Interval` a knot operation runs homogeneous, on `(w·P, w)`, and is
//! de-homogenized once into the carrier's door. Each row lifts a curve
//! to `Interval` at its points, runs an operation at both scalars, and
//! reads the `Interval` result against the `f64` one: the same
//! structure, every pole and weight within rounding of the `f64` value,
//! and no more width than rounding. A wrong blend orientation, a wrong unblend,
//! or a missing de-homogenization moves a pole by the net's own scale,
//! which these see; a symmetric blend or unit weights alone would not.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::num::NonZeroUsize;

use geom::NurbsCurve3;
use geom_core::spline::{KnotAlgebraError, Knots};
use geom_core::{Bounds, Interval, Point3, Real};

fn m(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

/// A cubic with interior knots at 0.3 and 0.55 (so no blend is
/// symmetric), at `f64`, with `weights`.
fn cubic(weights: &[f64]) -> NurbsCurve3<f64> {
    let knots = Knots::new(vec![(0.0, m(4)), (0.3, m(1)), (0.55, m(1)), (1.0, m(4))], 3).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.4, 1.1, 0.2),
        Point3::new(1.3, 1.6, -0.3),
        Point3::new(2.1, 0.7, 0.4),
        Point3::new(2.9, -0.2, 0.1),
        Point3::new(3.5, 0.9, -0.6),
    ];
    NurbsCurve3::new(knots, control, weights.to_vec()).unwrap()
}

fn lift(c: &NurbsCurve3<f64>) -> NurbsCurve3<Interval> {
    let i = Interval::from_f64;
    let knots = Knots::new(
        c.knot_values()
            .runs()
            .map(|(v, mult)| (i(v), m(mult)))
            .collect(),
        c.degree(),
    )
    .unwrap();
    NurbsCurve3::new(
        knots,
        c.control()
            .iter()
            .map(|p| Point3::new(i(p.x), i(p.y), i(p.z)))
            .collect(),
        c.weights().iter().map(|w| i(*w)).collect(),
    )
    .unwrap()
}

/// `got` reaches `want` pole for pole and weight for weight, each no
/// wider than `width` and no farther than `width` from it: the `f64`
/// operation rounds too (an elevation's unit weight can come out
/// `1 − 2⁻⁵³` at `f64`, exactly `1` at `Interval`).
fn encloses(tag: &str, got: &NurbsCurve3<Interval>, want: &NurbsCurve3<f64>, width: f64) {
    assert_eq!(got.degree(), want.degree(), "{tag}: degree");
    assert_eq!(
        got.control().len(),
        want.control().len(),
        "{tag}: pole count"
    );
    let within = |what: &str, k: usize, v: Interval, f: f64| {
        assert!(
            v.lo() - width <= f && f <= v.hi() + width && v.hi() - v.lo() <= width,
            "{tag}: {what} {k} = [{:e}, {:e}] against the f64 {f:e}",
            v.lo(),
            v.hi()
        );
    };
    for (k, (p, q)) in got.control().iter().zip(want.control()).enumerate() {
        within("pole x", k, p.x, q.x);
        within("pole y", k, p.y, q.y);
        within("pole z", k, p.z, q.z);
    }
    for (k, (w, f)) in got.weights().iter().zip(want.weights()).enumerate() {
        within("weight", k, *w, *f);
    }
}

/// **Every knot operation at `Interval` encloses its `f64` result.** On
/// a polynomial cubic: an insertion at 0.42, a refinement, an elevation
/// by one and by three, a split, the removal of a knot an insertion put
/// in, and the removal of a knot that is not exactly removable. On a
/// rational quadratic with weights `1, 2, 2, 1`: an insertion at 0.5
/// and a removal, whose weights stay points and whose poles depend on
/// the de-homogenization. Red where a blend or an unblend takes the
/// wrong orientation, or a pole is not divided back by its weight.
#[test]
fn every_knot_operation_at_interval_encloses_its_f64_result() {
    let w = 1e-12;
    let poly = cubic(&[1.0; 6]);
    let at = lift(&poly);
    encloses(
        "insert 0.42",
        &at.insert_knot(0.42, 1).unwrap(),
        &poly.insert_knot(0.42, 1).unwrap(),
        w,
    );
    encloses(
        "refine",
        &at.refine_knots(&[0.1, 0.42, 0.8]).unwrap(),
        &poly.refine_knots(&[0.1, 0.42, 0.8]).unwrap(),
        w,
    );
    encloses(
        "elevate 1",
        &at.elevate_degree(1).unwrap(),
        &poly.elevate_degree(1).unwrap(),
        w,
    );
    encloses(
        "elevate 3",
        &at.elevate_degree(3).unwrap(),
        &poly.elevate_degree(3).unwrap(),
        w,
    );
    let (a, b) = at.split_at(0.42).unwrap();
    let (fa, fb) = poly.split_at(0.42).unwrap();
    encloses("split left", &a, &fa, w);
    encloses("split right", &b, &fb, w);
    let put_in = poly.insert_knot(0.7, 1).unwrap();
    let (got, _) = lift(&put_in).remove_knot(0.7, 1).unwrap();
    let (want, _) = put_in.remove_knot(0.7, 1).unwrap();
    encloses("remove a knot put in", &got, &want, w);
    let (got, _) = at.remove_knot(0.3, 1).unwrap();
    let (want, _) = poly.remove_knot(0.3, 1).unwrap();
    encloses("remove 0.3", &got, &want, w);

    let knots = Knots::new(vec![(0.0, m(3)), (0.5, m(1)), (1.0, m(3))], 2).unwrap();
    let rational = NurbsCurve3::new(
        knots,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 2.0, 0.5),
            Point3::new(2.5, 1.5, -0.5),
            Point3::new(3.0, 0.0, 0.0),
        ],
        vec![1.0, 2.0, 2.0, 1.0],
    )
    .unwrap();
    let at = lift(&rational);
    encloses(
        "rational insert 0.25",
        &at.insert_knot(0.25, 1).unwrap(),
        &rational.insert_knot(0.25, 1).unwrap(),
        w,
    );
    let (got, _) = at.remove_knot(0.5, 1).unwrap();
    let (want, _) = rational.remove_knot(0.5, 1).unwrap();
    encloses("rational remove 0.5", &got, &want, w);
}

/// **A chain of blends meets its inputs' widths, it does not multiply
/// them.** A polynomial cubic whose x coordinates are each `±1e-6` wide:
/// elevated by four, and refined at 40 points, every pole stays within
/// a few times the input width. The plain step `x + (y − x)·t` reads
/// `x` twice and inflates per step (about 0.12 after elevating by four,
/// 7e-3 after the refinement); Red where `Interval`'s blend takes it.
#[test]
fn a_chain_of_blends_at_interval_does_not_multiply_its_width() {
    let poly = cubic(&[1.0; 6]);
    let at =
        lift(&poly).map_points(|p| Point3::new(p.x + Interval::from_bounds(-1e-6, 1e-6), p.y, p.z));
    let widest = |c: &NurbsCurve3<Interval>| {
        c.control()
            .iter()
            .fold(0.0f64, |m, p| m.max(p.x.hi() - p.x.lo()))
    };
    let elevated = widest(&at.elevate_degree(4).unwrap());
    assert!(
        elevated <= 1e-3,
        "elevated by four: widest pole {elevated:e}"
    );
    #[allow(clippy::cast_precision_loss)]
    let add: Vec<f64> = (1..=40).map(|k| k as f64 / 41.0).collect();
    let refined = widest(&at.refine_knots(&add).unwrap());
    assert!(
        refined <= 1e-5,
        "refined at 40 points: widest pole {refined:e}"
    );
}

/// **A removal's bound at `Interval` is an enclosure of the `f64` one.**
/// It was refused outright (`NaI`): the weights' minimum was seeded with
/// `+∞`, which `Interval` holds as no value. Red where the seed is not a
/// held weight.
#[test]
fn a_removal_bound_at_interval_encloses_the_f64_bound() {
    let poly = cubic(&[1.0; 6]);
    let (_, got) = lift(&poly).remove_knot(0.3, 1).unwrap();
    let (_, want) = poly.remove_knot(0.3, 1).unwrap();
    assert!(
        got.lo() <= want && want <= got.hi(),
        "bound [{:e}, {:e}] against the f64 {want:e}",
        got.lo(),
        got.hi()
    );
}

/// **A removal whose ratio's enclosure reaches 1 refuses, typed.** Knots
/// a few ulps apart (`1.1358116413770405`, `…412`, `…420`) make the
/// backward unblend divide by an enclosure of `1 − a` that reaches
/// zero; the pole it writes is `[−∞, +∞]`, which the door's poison test
/// does not see. Red where an unbounded pole is admitted.
#[test]
fn a_removal_whose_ratio_reaches_one_refuses_typed() {
    let i = Interval::from_f64;
    let (u, h, top) = (
        1.135_811_641_377_040_5,
        1.135_811_641_377_041_2,
        1.135_811_641_377_042,
    );
    let knots = Knots::new(
        vec![(i(-30.0), m(5)), (i(u), m(1)), (i(h), m(1)), (i(top), m(5))],
        4,
    )
    .unwrap();
    let n = 7;
    #[allow(clippy::cast_precision_loss)]
    let control: Vec<Point3<Interval>> = (0..n)
        .map(|k| Point3::new(i(k as f64), i(((k * 7) % 5) as f64 - 2.0), i(0.0)))
        .collect();
    let c = NurbsCurve3::new(knots, control, vec![i(1.0); n]).unwrap();
    match c.remove_knot(u, 1) {
        Err(KnotAlgebraError::UnboundedAtEnclosure { .. }) => {}
        other => panic!("a removal whose ratio reaches 1: {:?}", other.map(|_| ())),
    }
}

/// **An uncertified pole stays uncertified through the knot algebra.**
/// A pole computed outside its domain (`√[−1, 4]`, decorated below
/// certified) enters a quadratic at `Interval`; after an insertion and
/// after an elevation every pole it reaches still reads uncertified, so
/// the refusal signal (D4) is not laundered into a certified carrier.
/// Red where the blend step's corner hull carries its fresh corners'
/// decoration rather than its inputs'.
#[test]
fn an_uncertified_pole_stays_uncertified_through_the_knot_algebra() {
    let i = Interval::from_f64;
    let bad = <Interval as Real>::sqrt(Interval::from_bounds(-1.0, 4.0));
    assert!(!bad.is_certified(), "the fixture's pole is uncertified");
    let knots = Knots::new(vec![(i(0.0), m(3)), (i(1.0), m(3))], 2).unwrap();
    let c = NurbsCurve3::new(
        knots,
        vec![
            Point3::new(i(0.0), i(0.0), i(0.0)),
            Point3::new(bad, i(1.0), i(0.0)),
            Point3::new(i(3.0), i(0.0), i(0.0)),
        ],
        vec![i(1.0); 3],
    )
    .unwrap();
    let inserted = c.insert_knot(0.3, 1).unwrap();
    let elevated = c.elevate_degree(1).unwrap();
    for (tag, curve) in [("insert", &inserted), ("elevate", &elevated)] {
        let poles = curve.control();
        let inner = &poles[1..poles.len() - 1];
        for (k, p) in inner.iter().enumerate() {
            assert!(
                !p.x.is_certified(),
                "{tag}: inner pole {k}'s x reads certified"
            );
        }
    }
}
