//! **`NurbsSurface::rect_enclosure` contains the patch over its
//! rectangle, and `rect_jet` its partials** — counterexample searches,
//! and the door's edge rows.
//!
//! The search draws patches of degree 1–5 in each direction, interior
//! knots of multiplicity up to the degree, and half of them rational
//! with weights down to `1e-6`; on each, rectangles drawn at random,
//! straddling an interior knot, and tiny. Every sampled point of the
//! rectangle, evaluated at `Interval` at a point parameter, must meet
//! the box. A box that under-encloses by more than an evaluation's
//! rounding fails here with the seed that drew it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the door and what it is built from: the restriction runs on
// the blossom door and the span windows, the hull and the evaluation it
// is checked against on the interval scalar.
test_utils::gated_to![
    "crates/geom/src/surfaces/nurbs.rs",
    "crates/geom/src/surfaces.rs",
    "crates/geom-core/src/spline/",
    "crates/geom-core/src/interval.rs",
];

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real};
use test_utils::fuzz::{self, Rng};

/// A clamped knot vector of `degree` on `[0, 1]` with up to three
/// interior knot values, each repeated up to `degree` times.
fn knots(rng: &mut Rng, degree: usize) -> (KnotVector, Vec<f64>) {
    let mut interior: Vec<f64> = (0..rng.below(4)).map(|_| rng.range(0.05, 0.95)).collect();
    interior.sort_by(f64::total_cmp);
    interior.dedup();
    let mut all = vec![0.0; degree + 1];
    for &k in &interior {
        all.extend(std::iter::repeat_n(k, 1 + rng.below(degree)));
    }
    all.extend(std::iter::repeat_n(1.0, degree + 1));
    (KnotVector::clamped(all, degree).unwrap(), interior)
}

/// A random patch at `Interval`, and its two directions' interior knots.
fn patch(rng: &mut Rng) -> (NurbsSurface<Interval>, Vec<f64>, Vec<f64>) {
    let (pu, pv) = (1 + rng.below(5), 1 + rng.below(5));
    let (ku, iu) = knots(rng, pu);
    let (kv, iv) = knots(rng, pv);
    let n = ku.control_count() * kv.control_count();
    let control: Vec<Point3<f64>> = (0..n)
        .map(|_| {
            Point3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            )
        })
        .collect();
    let rational = rng.below(2) == 0;
    let weights: Vec<f64> = (0..n)
        .map(|_| {
            if rational {
                rng.log_uniform(1e-6, 1.0)
            } else {
                1.0
            }
        })
        .collect();
    let net = NurbsSurface::new(ku, kv, control, weights).unwrap();
    (net.map_scalar(Interval::from_f64), iu, iv)
}

/// A rectangle side in `[0, 1]`: random, straddling a knot of
/// `interior` when there is one, or tiny.
fn side(rng: &mut Rng, interior: &[f64]) -> (f64, f64) {
    match rng.below(3) {
        0 if !interior.is_empty() => {
            let k = interior[rng.below(interior.len())];
            let h = rng.log_uniform(1e-9, 0.05);
            ((k - h).max(0.0), (k + h).min(1.0))
        }
        1 => {
            let a = rng.range(0.0, 1.0);
            (a, (a + rng.log_uniform(1e-12, 1e-6)).min(1.0))
        }
        _ => {
            let (a, b) = (rng.range(0.0, 1.0), rng.range(0.0, 1.0));
            (a.min(b), a.max(b))
        }
    }
}

/// Whether the point's enclosure meets the box's: the true point lies
/// in both, so an empty meet proves the box misses it. Containment of
/// the whole enclosure is not owed: at a corner the box's coefficient
/// can be tighter than the evaluation's rounding.
fn meets(x: Interval, b: Interval) -> bool {
    b.lo() <= x.hi() && x.lo() <= b.hi()
}

#[test]
fn rect_enclosure_contains_every_sampled_point_of_its_rectangle() {
    let mut rng = fuzz::start("rect_enclosure_contains");
    let (mut boxes, mut poisoned) = (0usize, 0usize);
    for _ in 0..fuzz::scaled(15) {
        let (net, iu, iv) = patch(&mut rng);
        for _ in 0..8 {
            let (u, v) = (side(&mut rng, &iu), side(&mut rng, &iv));
            let enc = net.rect_enclosure(u, v);
            if enc.x.is_poison() {
                // An unclaimable extent is never a smaller one.
                poisoned += 1;
                continue;
            }
            boxes += 1;
            let at = |(lo, hi): (f64, f64), s: f64| lo + s * (hi - lo);
            let stations = [0.0, 0.5, 1.0];
            let samples = stations
                .iter()
                .flat_map(|&a| stations.iter().map(move |&b| (a, b)))
                .chain((0..4).map(|_| (rng.unit(), rng.unit())))
                .collect::<Vec<_>>();
            for (a, b) in samples {
                let (su, sv) = (at(u, a).min(u.1), at(v, b).min(v.1));
                let p = net.eval(Interval::from_f64(su), Interval::from_f64(sv));
                assert!(
                    meets(p.x, enc.x) && meets(p.y, enc.y) && meets(p.z, enc.z),
                    "the point at ({su}, {sv}) = {p:?} misses the enclosure {enc:?} of \
                     {u:?} × {v:?}; {}",
                    fuzz::replay()
                );
            }
        }
    }
    println!("[rect_enclosure] {boxes} boxes checked, {poisoned} poison");
}

/// **`NurbsSurface::rect_jet` encloses the partials over its
/// rectangle**: on the same draws, every sampled `S_u`, `S_v` and
/// `S_uv`, evaluated at `Interval` at a point parameter, meets the
/// enclosure. A rectangle the door declines answers `None`, never a
/// smaller box.
#[test]
fn rect_jet_contains_every_sampled_partial_of_its_rectangle() {
    let mut rng = fuzz::start("rect_jet_contains");
    let (mut jets, mut declined) = (0usize, 0usize);
    for _ in 0..fuzz::scaled(15) {
        let (net, iu, iv) = patch(&mut rng);
        for _ in 0..8 {
            let (u, v) = (side(&mut rng, &iu), side(&mut rng, &iv));
            let Some((eu, ev, euv)) = net.rect_jet(u, v) else {
                declined += 1;
                continue;
            };
            jets += 1;
            // A claimed enclosure is finite: the whole line contains every
            // sample and would pass containment, so it is refused here.
            for e in [eu, ev, euv] {
                assert!(
                    [e.x, e.y, e.z]
                        .iter()
                        .all(|c| c.lo().is_finite() && c.hi().is_finite()),
                    "rect_jet over {u:?} × {v:?} claims an unbounded partial {e:?}; {}",
                    fuzz::replay()
                );
            }
            let at = |(lo, hi): (f64, f64), s: f64| lo + s * (hi - lo);
            let stations = [0.0, 0.5, 1.0];
            let samples = stations
                .iter()
                .flat_map(|&a| stations.iter().map(move |&b| (a, b)))
                .chain((0..4).map(|_| (rng.unit(), rng.unit())))
                .collect::<Vec<_>>();
            for (a, b) in samples {
                let (su, sv) = (at(u, a).min(u.1), at(v, b).min(v.1));
                let j = net.ders(Interval::from_f64(su), Interval::from_f64(sv));
                for (what, got, enc) in [("S_u", j.du, eu), ("S_v", j.dv, ev), ("S_uv", j.duv, euv)]
                {
                    assert!(
                        meets(got.x, enc.x) && meets(got.y, enc.y) && meets(got.z, enc.z),
                        "{what} at ({su}, {sv}) = {got:?} misses the enclosure {enc:?} of \
                         {u:?} × {v:?}; {}",
                        fuzz::replay()
                    );
                }
            }
        }
    }
    println!("[rect_jet] {jets} jets checked, {declined} declined");
    // The door reads every rectangle inside the domain with extent on
    // both axes, which is every draw here: a decline is a door that
    // stopped reading, and a run of them would pass the row vacuously.
    assert_eq!(
        declined,
        0,
        "rect_jet declined {declined} of {} rectangles",
        jets + declined
    );
}

/// The unit bilinear patch `(u, v) ↦ (u, v, u·v)` at `Interval`.
fn bilinear() -> NurbsSurface<Interval> {
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(
        unit(),
        unit(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
        ],
        vec![1.0; 4],
    )
    .unwrap()
    .map_scalar(Interval::from_f64)
}

/// **A rectangle reaching past the knot domain is poison**: the patch
/// has no points there, and a clipped box would be smaller than what
/// `eval` reads off the end pieces' extension. A rectangle inside the
/// domain, its edges included, encloses.
#[test]
fn a_rectangle_past_the_knot_domain_is_poison() {
    let net = bilinear();
    for (u, v) in [
        ((-0.25, 0.5), (0.0, 1.0)),
        ((0.5, 1.25), (0.0, 1.0)),
        ((0.0, 1.0), (-1e-300, 0.5)),
        ((0.0, 1.0), (0.5, 1.0f64.next_up())),
    ] {
        assert!(
            net.rect_enclosure(u, v).x.is_poison(),
            "{u:?} × {v:?} reaches past [0, 1]²"
        );
    }
    let whole = net.rect_enclosure((0.0, 1.0), (0.0, 1.0));
    assert!(
        !whole.z.is_poison() && whole.z.lo() <= 0.0 && 1.0 <= whole.z.hi(),
        "the whole domain encloses the patch's z range [0, 1]: {whole:?}"
    );
}
