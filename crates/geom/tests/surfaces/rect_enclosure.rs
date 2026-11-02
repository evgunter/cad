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
    "crates/geom/src/net.rs",
    "crates/geom/tests/exact_q/",
    "crates/geom-core/tests/exact_q/",
];

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz::{self, Rng};

/// `x` moved `n` ulps up (`n > 0`) or down.
fn ulps(x: f64, n: i64) -> f64 {
    (0..n.unsigned_abs()).fold(x, |x, _| if n > 0 { x.next_up() } else { x.next_down() })
}

/// A clamped knot vector of `degree` on `[0, 1]` with up to three
/// interior knot values, each repeated up to `degree` times, and
/// sometimes a further value a few ulps past one of them: a span a few
/// ulps wide.
fn knots(rng: &mut Rng, degree: usize) -> (KnotVector, Vec<f64>) {
    let mut interior: Vec<f64> = (0..rng.below(4)).map(|_| rng.range(0.05, 0.95)).collect();
    if !interior.is_empty() && rng.below(3) == 0 {
        let k = interior[rng.below(interior.len())];
        interior.push(ulps(k, 1 + i64::try_from(rng.below(4)).unwrap()));
    }
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
/// Half are moved off the origin by up to `1e3` on each axis.
fn patch(rng: &mut Rng) -> (NurbsSurface<Interval>, Vec<f64>, Vec<f64>) {
    let (pu, pv) = (1 + rng.below(5), 1 + rng.below(5));
    let (ku, iu) = knots(rng, pu);
    let (kv, iv) = knots(rng, pv);
    let n = ku.control_count() * kv.control_count();
    let off = translation(rng);
    let control: Vec<Point3<f64>> = (0..n)
        .map(|_| {
            Point3::new(
                off[0] + rng.range(-1.0, 1.0),
                off[1] + rng.range(-1.0, 1.0),
                off[2] + rng.range(-1.0, 1.0),
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

/// A translation: none half the time, else up to `1e3` on each axis,
/// log-uniform in size and either sign.
fn translation(rng: &mut Rng) -> [f64; 3] {
    if rng.below(2) == 0 {
        return [0.0; 3];
    }
    [0; 3].map(|_| {
        let x = rng.log_uniform(1e-3, 1e3);
        if rng.below(2) == 0 { x } else { -x }
    })
}

/// A rectangle side in `[0, 1]`: random, straddling a knot of
/// `interior` when there is one, ending a few ulps to either side of
/// one (so it meets the span past it over those ulps), or tiny.
fn side(rng: &mut Rng, interior: &[f64]) -> (f64, f64) {
    match rng.below(4) {
        3 if !interior.is_empty() => {
            let k = interior[rng.below(interior.len())];
            // A few ulps to either side, never on a knot: a cell
            // ending on a knot meets the span past it at one point,
            // which `rect_jet` reads as no meet
            // (`work/clear/rect-jet-reads-no-point-meet-at-a-cell-end-on-a-knot.md`).
            let off = 1 + i64::try_from(rng.below(4)).unwrap();
            let step = if rng.below(2) == 0 { 1 } else { -1 };
            let mut end = ulps(k, step * off);
            while interior.contains(&end) {
                end = ulps(end, step);
            }
            let h = rng.log_uniform(1e-9, 0.25);
            if rng.below(2) == 0 {
                ((end - h).max(0.0), end)
            } else {
                (end, (end + h).min(1.0))
            }
        }
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

/// **A cell's few-ulp meet with the span past a full-multiplicity knot
/// is enclosed, and tightly.** The ruled patch over `v`-degree 3 with a
/// triple knot at `k = ½ − 2 ulp`: along `v` its rulings step `0.1` in
/// `+x` into the knot and `+y` out of it, so `S_v` jumps there from
/// `(0.3/k, 0, 0)` to `(0, 0.3/(1 − k), 0)`. The cell `v ∈ [¼, ½]` holds the span past the
/// knot only over `(k, ½]`, two ulps.
///
/// Every sampled `S_v` meets the enclosure, the one at `v = ½` on the
/// far span included: red if the meet were dropped as too narrow to
/// matter, or bounded by the near span's jet, which the jump makes
/// unsound. And the enclosure is within an ulp-scale margin of the two
/// one-sided values: red if the derivative were read by differencing the
/// meet's own two-ulp restriction, whose rounding over its width is
/// slope enough to swamp them.
#[test]
fn a_few_ulp_meet_past_a_full_multiplicity_knot_is_enclosed_tightly() {
    let k = 0.5f64.next_down().next_down();
    // Steps of 0.1 from (0.3, 0.7): coordinates no float holds exactly,
    // so a restriction rounds.
    let s = 0.1;
    let ruled = |x: f64, y: f64| {
        let (x, y) = (0.3 + s * x, 0.7 + s * y);
        [Point3::new(x, y, 0.0), Point3::new(x, y, 1.0)]
    };
    let rulings = [
        ruled(0.0, 0.0),
        ruled(1.0, 0.0),
        ruled(2.0, 0.0),
        ruled(3.0, 0.0),
        ruled(3.0, 1.0),
        ruled(3.0, 2.0),
        ruled(3.0, 3.0),
    ];
    // Row-major, `u` rows: control `[i][j]` is ruling `j`'s end `i`.
    let control = (0..2)
        .flat_map(|i| rulings.iter().map(move |r| r[i]))
        .collect::<Vec<_>>();
    let net = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, k, k, k, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
        control,
        vec![1.0; 14],
    )
    .unwrap()
    .map_scalar(Interval::from_f64);
    let (u, v) = ((0.0, 1.0), (0.25, 0.5));
    let (_, sv, _) = net.rect_jet(u, v).expect("the cell is inside the domain");
    for (s, t) in [(0.0, 0.25), (0.5, 0.375), (1.0, k), (0.0, 0.5), (1.0, 0.5)] {
        let got = net.ders(Interval::from_f64(s), Interval::from_f64(t)).dv;
        assert!(
            meets(got.x, sv.x) && meets(got.y, sv.y) && meets(got.z, sv.z),
            "S_v at ({s}, {t}) = {got:?} misses the enclosure {sv:?} of {u:?} × {v:?}"
        );
    }
    let (near, far) = (3.0 * s / k, 3.0 * s / (1.0 - k));
    assert!(
        sv.y.lo() <= 0.0 && far <= sv.y.hi(),
        "S_v's y reaches both sides of the jump, 0 and {far}: {sv:?}"
    );
    let slack = 1e-12;
    assert!(
        sv.x.lo() > -slack
            && sv.x.hi() < near + slack
            && sv.y.lo() > -slack
            && sv.y.hi() < far + slack
            && sv.z.lo() > -slack
            && sv.z.hi() < slack,
        "S_v is enclosed within {slack} of [0, {near}] × [0, {far}] × {{0}}: {sv:?}"
    );
}

/// **`rect_jet` against the exact partials.** Rational nets with weights
/// from `1e-3` to `1e3`, interior knots of every multiplicity up to the
/// degree, moved off the origin by up to `1e3`; cells `2⁻³` to `2⁻⁴⁰`
/// wide, anywhere, against a knot, or a few ulps across one. At each
/// sampled point of the cell, each span meeting the cell over an extent
/// that holds the point gives the exact `S_u`, `S_v` and `S_uv` in exact
/// rationals off its own knots, and each must lie inside the claimed
/// enclosure, not merely meet a rounded evaluation.
#[test]
fn rect_jet_contains_the_exact_partials_of_translated_rational_nets() {
    use exact::Net;
    let mut rng = fuzz::start("rect_jet_exact");
    let (mut checked, mut jets) = (0usize, 0usize);
    for _ in 0..fuzz::scaled(6) {
        let data = Net::draw(&mut rng);
        let net = data.surface();
        for _ in 0..5 {
            let (u, v) = (data.cell(&mut rng, 0), data.cell(&mut rng, 1));
            let Some(jet) = net.rect_jet(u, v) else {
                panic!(
                    "rect_jet declined {u:?} × {v:?} inside the domain; {}",
                    fuzz::replay()
                );
            };
            jets += 1;
            let at = |(lo, hi): (f64, f64), s: f64| (lo + s * (hi - lo)).clamp(lo, hi);
            let stations = [0.0, 0.5, 1.0];
            let samples = stations
                .iter()
                .flat_map(|&a| stations.iter().map(move |&b| (a, b)))
                .chain((0..2).map(|_| (rng.unit(), rng.unit())))
                .collect::<Vec<_>>();
            for (a, b) in samples {
                let (s, t) = (at(u, a), at(v, b));
                for (cu, cv) in data
                    .spans(0, u, s)
                    .flat_map(|cu| data.spans(1, v, t).map(move |cv| (cu, cv)))
                {
                    let exact = data.partials(cu, cv, s, t);
                    for ((what, enc), want) in [("S_u", jet.0), ("S_v", jet.1), ("S_uv", jet.2)]
                        .iter()
                        .zip(&exact)
                    {
                        for (k, c) in [enc.x, enc.y, enc.z].iter().enumerate() {
                            let inside =
                                want[k].cmp_f64(c.lo()).is_ge() && want[k].cmp_f64(c.hi()).is_le();
                            assert!(
                                inside,
                                "{what}[{k}] at ({s}, {t}) on spans ({cu}, {cv}) is {} exactly, \
                                 outside the enclosure {c:?} of {u:?} × {v:?}; {}",
                                want[k].to_f64_report(),
                                fuzz::replay()
                            );
                            checked += 1;
                        }
                    }
                }
            }
        }
    }
    println!("[rect_jet exact] {jets} jets, {checked} exact coordinates checked");
}

/// The exact oracle's patch: the `f64` data a [`NurbsSurface`] is built
/// from, read again in exact rationals. Channels are indexed: one index
/// reads the control point, the homogeneous sums and the partial alike.
#[allow(clippy::needless_range_loop)]
mod exact {
    use super::{translation, ulps};
    use crate::exact_q::Q;
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;
    use geom_core::{Interval, Point3, Real};
    use test_utils::fuzz::Rng;

    pub(super) struct Net {
        degree: [usize; 2],
        knots: [Vec<f64>; 2],
        control: Vec<[f64; 3]>,
        weights: Vec<f64>,
    }

    /// A clamped knot vector on `[0, 1]` of `degree`: up to three
    /// interior values, each of multiplicity 1 to `degree`.
    fn knots(rng: &mut Rng, degree: usize) -> Vec<f64> {
        let mut interior: Vec<f64> = (0..rng.below(4)).map(|_| rng.range(0.05, 0.95)).collect();
        interior.sort_by(f64::total_cmp);
        interior.dedup();
        let mut all = vec![0.0; degree + 1];
        for &k in &interior {
            all.extend(std::iter::repeat_n(k, 1 + rng.below(degree)));
        }
        all.extend(std::iter::repeat_n(1.0, degree + 1));
        all
    }

    impl Net {
        /// The net of `degree`, `knots`, row-major `control` and `weights`
        /// as given.
        pub(super) fn new(
            degree: [usize; 2],
            knots: [Vec<f64>; 2],
            control: Vec<[f64; 3]>,
            weights: Vec<f64>,
        ) -> Self {
            Self {
                degree,
                knots,
                control,
                weights,
            }
        }

        pub(super) fn draw(rng: &mut Rng) -> Self {
            let degree = [1 + rng.below(4), 1 + rng.below(4)];
            let knots = [knots(rng, degree[0]), knots(rng, degree[1])];
            let n = (knots[0].len() - degree[0] - 1) * (knots[1].len() - degree[1] - 1);
            let off = translation(rng);
            let control = (0..n)
                .map(|_| [0, 1, 2].map(|k| off[k] + rng.range(-1.0, 1.0)))
                .collect();
            let weights = (0..n).map(|_| rng.log_uniform(1e-3, 1e3)).collect();
            Self {
                degree,
                knots,
                control,
                weights,
            }
        }

        pub(super) fn surface(&self) -> NurbsSurface<Interval> {
            let kv = |a: usize| KnotVector::clamped(self.knots[a].clone(), self.degree[a]).unwrap();
            NurbsSurface::new(
                kv(0),
                kv(1),
                self.control
                    .iter()
                    .map(|p| Point3::new(p[0], p[1], p[2]))
                    .collect(),
                self.weights.clone(),
            )
            .unwrap()
            .map_scalar(Interval::from_f64)
        }

        /// A cell side on `axis`, `2⁻³` to `2⁻⁴⁰` wide: anywhere, ending
        /// on an interior knot, or ending a few ulps past one.
        pub(super) fn cell(&self, rng: &mut Rng, axis: usize) -> (f64, f64) {
            let h = 2f64.powi(-(3 + i32::try_from(rng.below(38)).unwrap()));
            let interior: Vec<f64> = self.knots[axis]
                .iter()
                .copied()
                .filter(|&k| 0.0 < k && k < 1.0)
                .collect();
            let anchor = match rng.below(3) {
                0 if !interior.is_empty() => interior[rng.below(interior.len())],
                1 if !interior.is_empty() => {
                    let k = interior[rng.below(interior.len())];
                    let n = 1 + i64::try_from(rng.below(4)).unwrap();
                    ulps(k, if rng.below(2) == 0 { n } else { -n })
                }
                _ => rng.range(0.0, 1.0),
            };
            if rng.below(2) == 0 {
                ((anchor - h).max(0.0), anchor.max(h.min(1.0)))
            } else {
                (anchor.min(1.0 - h), (anchor + h).min(1.0))
            }
        }

        /// The nonempty spans on `axis` whose meet with `side` has
        /// extent and holds `t`.
        pub(super) fn spans(
            &self,
            axis: usize,
            (lo, hi): (f64, f64),
            t: f64,
        ) -> impl Iterator<Item = usize> + '_ {
            let k = &self.knots[axis];
            (0..k.len() - 1).filter(move |&s| {
                let (a, b) = (k[s], k[s + 1]);
                a < b && a <= t && t <= b && lo.max(a) < hi.min(b)
            })
        }

        /// The basis functions of `axis` on span `s` at `t`, and their
        /// first derivatives, exactly: `N_{s−p+r}` for `r = 0..=p`.
        fn basis(&self, axis: usize, s: usize, t: f64) -> (Vec<Q>, Vec<Q>) {
            let p = self.degree[axis];
            let u: Vec<Q> = self.knots[axis].iter().map(|&k| Q::from_f64(k)).collect();
            let t = Q::from_f64(t);
            let zero = || Q::from_f64(0.0);
            // n[k][r] = N_{s−k+r, k}.
            let mut n: Vec<Vec<Q>> = vec![vec![Q::from_f64(1.0)]];
            for k in 1..=p {
                let prev = &n[k - 1];
                let row = (0..=k)
                    .map(|r| {
                        let j = s + r - k;
                        let mut acc = zero();
                        if r >= 1 {
                            let f = t.sub(&u[j]).div(&u[j + k].sub(&u[j]));
                            acc = acc.add(&f.mul(&prev[r - 1]));
                        }
                        if r < k {
                            let g = u[j + k + 1].sub(&t).div(&u[j + k + 1].sub(&u[j + 1]));
                            acc = acc.add(&g.mul(&prev[r]));
                        }
                        acc
                    })
                    .collect();
                n.push(row);
            }
            let prev = &n[p - 1];
            let pq = Q::from_f64(p as f64);
            let d = (0..=p)
                .map(|r| {
                    let j = s + r - p;
                    let mut acc = zero();
                    if r >= 1 {
                        acc = acc.add(&prev[r - 1].div(&u[j + p].sub(&u[j])));
                    }
                    if r < p {
                        acc = acc.sub(&prev[r].div(&u[j + p + 1].sub(&u[j + 1])));
                    }
                    pq.mul(&acc)
                })
                .collect();
            (n.swap_remove(p), d)
        }

        /// The exact `[S_u, S_v, S_uv]` at `(s, t)` on spans `(cu, cv)`.
        pub(super) fn partials(&self, cu: usize, cv: usize, s: f64, t: f64) -> [[Q; 3]; 3] {
            let (pu, pv) = (self.degree[0], self.degree[1]);
            let nv = self.knots[1].len() - pv - 1;
            let (nu, du) = self.basis(0, cu, s);
            let (nvv, dv) = self.basis(1, cv, t);
            let zero = || Q::from_f64(0.0);
            // [value, ∂u, ∂v, ∂uv] of [x, y, z, w], summed along `v`
            // first: `rows[a] = [Σ_b N_b·h_ab, Σ_b N′_b·h_ab]`.
            let rows: Vec<[[Q; 4]; 2]> = (0..=pu)
                .map(|a| {
                    let mut acc: [[Q; 4]; 2] =
                        std::array::from_fn(|_| std::array::from_fn(|_| zero()));
                    for b in 0..=pv {
                        let idx = (cu - pu + a) * nv + (cv - pv + b);
                        let w = Q::from_f64(self.weights[idx]);
                        for c in 0..4 {
                            let hom = if c == 3 {
                                w.clone()
                            } else {
                                w.mul(&Q::from_f64(self.control[idx][c]))
                            };
                            acc[0][c] = acc[0][c].add(&nvv[b].mul(&hom));
                            acc[1][c] = acc[1][c].add(&dv[b].mul(&hom));
                        }
                    }
                    acc
                })
                .collect();
            let mut h: [[Q; 4]; 4] = std::array::from_fn(|_| std::array::from_fn(|_| zero()));
            for (a, row) in rows.iter().enumerate() {
                for c in 0..4 {
                    h[0][c] = h[0][c].add(&nu[a].mul(&row[0][c]));
                    h[1][c] = h[1][c].add(&du[a].mul(&row[0][c]));
                    h[2][c] = h[2][c].add(&nu[a].mul(&row[1][c]));
                    h[3][c] = h[3][c].add(&du[a].mul(&row[1][c]));
                }
            }
            let [a, au, av, auv] = &h;
            let (w, wu, wv, wuv) = (&a[3], &au[3], &av[3], &auv[3]);
            let w2 = w.mul(w);
            let w3 = w2.mul(w);
            let two = Q::from_f64(2.0);
            std::array::from_fn(|which| {
                std::array::from_fn(|c| {
                    let nu_ = au[c].mul(w).sub(&a[c].mul(wu));
                    match which {
                        0 => nu_.div(&w2),
                        1 => av[c].mul(w).sub(&a[c].mul(wv)).div(&w2),
                        _ => auv[c]
                            .mul(w)
                            .add(&au[c].mul(wv))
                            .sub(&av[c].mul(wu))
                            .sub(&a[c].mul(wuv))
                            .div(&w2)
                            .sub(&two.mul(&nu_).mul(wv).div(&w3)),
                    }
                })
            })
        }
    }
}

/// A rational quarter circle of radius `¼` about `(cx, cy)` in `u`, run
/// straight up one unit in `v`: weights `1, √½, 1`.
fn quarter_circle_wall(cx: f64, cy: f64) -> NurbsSurface<Interval> {
    let r = 0.25;
    let arc = [(cx + r, cy), (cx + r, cy + r), (cx, cy + r)];
    let control = arc
        .iter()
        .flat_map(|&(x, y)| [Point3::new(x, y, 0.0), Point3::new(x, y, 1.0)])
        .collect();
    let s = core::f64::consts::FRAC_1_SQRT_2;
    NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        vec![1.0, 1.0, s, s, 1.0, 1.0],
    )
    .unwrap()
    .map_scalar(Interval::from_f64)
}

/// The widest coordinate of each of `rect_jet`'s three partials.
fn jet_widths(net: &NurbsSurface<Interval>, u: (f64, f64), v: (f64, f64)) -> [f64; 3] {
    let (a, b, c) = net.rect_jet(u, v).expect("a cell inside the domain");
    [a, b, c].map(|e| {
        [e.x, e.y, e.z]
            .iter()
            .map(|x| x.hi() - x.lo())
            .fold(0.0, f64::max)
    })
}

/// **A rational wall's jet does not move with the wall.** The quarter
/// circle at the origin and at `(3, 3)`, on an 8 × 32 grid of cells and
/// on cells `2⁻¹⁰` and `2⁻³⁰` wide: each partial's width matches its
/// twin's within 1% plus a rounding allowance of `1e-12`. Read in the
/// absolute frame, `S_u` at `(3, 3)` is several times wider at every
/// cell size.
#[test]
fn a_rational_walls_jet_does_not_move_with_the_wall() {
    let (home, away) = (quarter_circle_wall(0.0, 0.0), quarter_circle_wall(3.0, 3.0));
    let mut cells: Vec<((f64, f64), (f64, f64))> = (0..8)
        .flat_map(|i| {
            (0..32).map(move |j| {
                let (i, j) = (f64::from(i), f64::from(j));
                ((i / 8.0, (i + 1.0) / 8.0), (j / 32.0, (j + 1.0) / 32.0))
            })
        })
        .collect();
    for k in [10, 30] {
        let h = 2f64.powi(-k);
        cells.push(((0.5, 0.5 + h), (0.25, 0.25 + h)));
    }
    let mut worst = 0.0f64;
    for (u, v) in cells {
        let (a, b) = (jet_widths(&home, u, v), jet_widths(&away, u, v));
        for (k, what) in ["S_u", "S_v", "S_uv"].iter().enumerate() {
            worst = worst.max(b[k] / a[k].max(1e-300));
            assert!(
                b[k] <= 1.01 * a[k] + 1e-12 && a[k] <= 1.01 * b[k] + 1e-12,
                "{what} over {u:?} × {v:?}: {} wide at (3, 3) against {} at the origin",
                b[k],
                a[k]
            );
        }
    }
    println!("[jet frame] quarter circle: worst width ratio (3, 3) / origin {worst}");
}

/// **A rational wall's jet is tight against the wall's own turning.**
/// The quarter circle at `(3, 3)` on cells `2⁻²` to `2⁻³⁰` along `u`:
/// `S_u`'s wider coordinate against the wider coordinate's spread over
/// 33 sampled points of the cell, which is at most its true range. A jet read
/// about a point far from the cell carries that distance times the
/// weight's slope, which shrinks no faster than the cell's own range.
#[test]
fn a_rational_walls_jet_is_tight_against_its_own_range() {
    let net = quarter_circle_wall(3.0, 3.0);
    let mut worst = 0.0f64;
    for k in [2, 4, 8, 16, 30] {
        let h = 2f64.powi(-k);
        for u in [(0.0, h), (0.5 - h, 0.5), (0.3, 0.3 + h), (1.0 - h, 1.0)] {
            let v = (0.25, 0.5);
            let (su, _, _) = net.rect_jet(u, v).expect("a cell inside the domain");
            let samples: Vec<_> = (0..=32)
                .map(|i| {
                    let s = (u.0 + f64::from(i) / 32.0 * (u.1 - u.0)).min(u.1);
                    net.ders(Interval::from_f64(s), Interval::from_f64(0.375))
                        .du
                })
                .collect();
            let spread = |pick: fn(&Vec3<Interval>) -> Interval| {
                let lo = samples
                    .iter()
                    .map(|d| pick(d).hi())
                    .fold(f64::INFINITY, f64::min);
                let hi = samples
                    .iter()
                    .map(|d| pick(d).lo())
                    .fold(f64::NEG_INFINITY, f64::max);
                (hi - lo).max(0.0)
            };
            let spread = spread(|d| d.x).max(spread(|d| d.y));
            let width = (su.x.hi() - su.x.lo()).max(su.y.hi() - su.y.lo());
            worst = worst.max(width / spread.max(1e-300));
            assert!(
                width <= 4.0 * spread + 1e-12,
                "S_u over u ∈ {u:?}: {width} wide against a sampled spread of {spread}"
            );
        }
    }
    println!("[jet frame] quarter circle at (3, 3): worst width / spread {worst}");
}

/// **`S_uv` against its exact value, on one fixed net.** A quartic ×
/// linear patch whose weights span `0.01` to `474`, moved to about
/// `(4, −7, −128)`, on a cell `1.5e-5 × 3.9e-3` whose `v` side starts on
/// an interior knot: the exact `S_uv` at the cell's corners and middle,
/// on every span that meets it there, lies inside `rect_jet`'s
/// enclosure. The weights' slopes and twist are steep enough that each of
/// the mixed partial's weight terms, `B·w_uv` included, moves `S_uv`
/// outside the enclosure when dropped, on every run. The net is the
/// exact oracle's draw at `CAD_FUZZ_SEED=0xb`, `CAD_FUZZ_EFFORT=8`,
/// written out.
#[test]
fn rect_jet_contains_the_exact_mixed_partial_of_a_fixed_rational_net() {
    let knots = [
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        vec![
            0.0,
            0.0,
            0.158_226_476_123_069_95,
            0.417_353_407_794_508_36,
            0.423_261_609_471_575_43,
            1.0,
            1.0,
        ],
    ];
    let control = vec![
        [3.050684608727739, -7.178091779965307, -127.05691323810933],
        [3.8740481326772764, -6.05589459197545, -126.93362955858143],
        [4.203129067593478, -6.591866329780057, -128.46802443939205],
        [3.8786676091911763, -6.597712431775758, -127.42304157601524],
        [4.044996355426123, -7.660184353670945, -127.31016418887981],
        [3.4467115813412144, -5.7651536390525795, -127.94727107639736],
        [3.2967088336918886, -6.9162905431774755, -127.9144226170482],
        [2.865635694996638, -7.276749283565412, -127.13856184432342],
        [4.290250853305883, -7.560079973978031, -127.75826575462008],
        [4.665713561612451, -7.15669900401047, -128.11683160390533],
        [4.172179846933471, -7.1018169317187905, -127.11205220335792],
        [3.3825904284215493, -6.751485532829372, -128.2185563131049],
        [3.8818678665045727, -5.914391892584441, -128.53339538668527],
        [2.875673697583536, -6.685454066578751, -128.12856829204156],
        [3.9718999782930036, -7.393790450216364, -127.73138178959199],
        [2.8901198717051106, -5.997698238870685, -127.00582465596217],
        [3.1110116353089263, -7.271043711538317, -128.22076286754285],
        [2.984657930262114, -7.590605128805975, -128.55126318179677],
        [2.788702100704791, -6.161700012823305, -126.82823386766917],
        [4.635549707905644, -6.427819108973203, -127.5201890173393],
        [4.045383980092756, -6.8671844222713965, -127.46014563580876],
        [3.203650205574907, -6.319781505149965, -127.27644246816584],
        [4.516658169742177, -7.27515979507934, -127.4225895090312],
        [4.67091759260741, -5.829628071187193, -127.74565022044183],
        [4.277178235047719, -6.702753974382321, -127.64319719219007],
    ];
    let weights = vec![
        16.201368860575894,
        80.11449246535871,
        0.7022726137531653,
        32.77331889076174,
        2.3513617617327927,
        270.3095951132867,
        22.585000274865024,
        0.011730963644322223,
        131.08289513497513,
        3.6143006436512173,
        473.86566487168744,
        1.2558176452317087,
        90.75678516505872,
        113.93898271748513,
        0.0257092401721693,
        0.08497622157165653,
        27.299110050291407,
        1.0211646851314597,
        0.027431508998624832,
        0.03039679866751755,
        250.65176372801014,
        0.3450485435613021,
        0.3681499449773386,
        0.010297527717472986,
        234.01393296691782,
    ];
    let data = exact::Net::new([4, 1], knots, control, weights);
    let net = data.surface();
    let (u, v) = (
        (0.324_520_691_074_959_4, 0.324_535_949_864_021_9),
        (0.417_353_407_794_508_36, 0.421_259_657_794_508_36),
    );
    let (_, _, suv) = net.rect_jet(u, v).expect("a cell inside the domain");
    let at = |(lo, hi): (f64, f64), x: f64| (lo + x * (hi - lo)).clamp(lo, hi);
    let mut checked = 0usize;
    for (a, b) in [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (0.5, 0.5)] {
        let (s, t) = (at(u, a), at(v, b));
        for (cu, cv) in data
            .spans(0, u, s)
            .flat_map(|cu| data.spans(1, v, t).map(move |cv| (cu, cv)))
        {
            let want = &data.partials(cu, cv, s, t)[2];
            for (k, c) in [suv.x, suv.y, suv.z].iter().enumerate() {
                assert!(
                    want[k].cmp_f64(c.lo()).is_ge() && want[k].cmp_f64(c.hi()).is_le(),
                    "S_uv[{k}] at ({s}, {t}) on spans ({cu}, {cv}) is {} exactly, outside \
                     the enclosure {c:?}",
                    want[k].to_f64_report()
                );
                checked += 1;
            }
        }
    }
    assert_eq!(
        checked, 15,
        "five points, one span pair each, three coordinates"
    );
}

/// **A piece whose middle evaluates to poison claims no partial.** Every
/// control point at `1.5e308` with a weight of 2: the homogeneous sums
/// overflow, so the evaluation at the cell's middle has no finite point
/// to read the net about, and `rect_jet` answers `None` rather than an
/// enclosure read about a point the cell does not have.
#[test]
fn rect_jet_refuses_a_piece_whose_middle_overflows() {
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let far = 1.5e308;
    let net = NurbsSurface::new(
        unit(),
        unit(),
        vec![Point3::new(far, far, far); 4],
        vec![2.0; 4],
    )
    .unwrap()
    .map_scalar(Interval::from_f64);
    assert!(
        net.rect_jet((0.25, 0.5), (0.25, 0.5)).is_none(),
        "a poisoned middle claims no partial: {:?}",
        net.rect_jet((0.25, 0.5), (0.25, 0.5))
    );
}

/// **A weight not certified positive claims no partial.** The bilinear
/// patch with corner weights `10, 10, 10` and `[−0.5, 1.5]` at `(1, 1)`,
/// on the cell `[½, 1]²`: the weight at the cell's middle is certainly
/// positive, so the cell has a point to be read about, but the
/// restricted weight at its far corner reaches zero, so the quotient's
/// denominator is not certified positive and `rect_jet` answers `None`.
#[test]
fn rect_jet_refuses_a_weight_not_certified_positive() {
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let net = NurbsSurface::new(
        unit(),
        unit(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
        ],
        vec![10.0, 10.0, 10.0, 0.5],
    )
    .unwrap()
    .map_scalar(|x| {
        if x == 0.5 {
            Interval::from_bounds(-0.5, 1.5)
        } else {
            Interval::from_f64(x)
        }
    });
    let middle = net.eval(Interval::from_f64(0.75), Interval::from_f64(0.75));
    assert!(
        [middle.x, middle.y, middle.z]
            .iter()
            .all(|c| c.lo().is_finite() && c.hi().is_finite()),
        "the cell's middle evaluates to a finite point: {middle:?}"
    );
    assert!(
        net.rect_jet((0.5, 1.0), (0.5, 1.0)).is_none(),
        "a weight reaching zero claims no partial: {:?}",
        net.rect_jet((0.5, 1.0), (0.5, 1.0))
    );
}

/// Every bound of a point's coordinates, as bits.
fn net_bits(points: &[Point3<Interval>]) -> Vec<[u64; 6]> {
    points
        .iter()
        .map(|p| [p.x, p.y, p.z].map(|c| [c.lo().to_bits(), c.hi().to_bits()]))
        .map(|[x, y, z]| [x[0], x[1], y[0], y[1], z[0], z[1]])
        .collect()
}

#[allow(clippy::type_complexity)]
fn jet_bits(jet: (Vec3<Interval>, Vec3<Interval>, Vec3<Interval>)) -> Vec<u64> {
    [jet.0, jet.1, jet.2]
        .iter()
        .flat_map(|e| [e.x, e.y, e.z])
        .flat_map(|c| [c.lo().to_bits(), c.hi().to_bits()])
        .collect()
}

/// **A memoised chart answers each rectangle as the plain chart does,
/// bit for bit.** A split tree of the rational quarter-circle wall, four
/// levels deep, each cell halved on `u` then `v`, read parent first, so
/// a cell is asked after the rectangle that holds it and beside its
/// sibling: the memo's `rect_net` and `rect_jet`, and its point jets at
/// the cell's corners and centre, are the plain chart's, on the first
/// read and on the second, which the memo serves. A memo
/// keyed by less than the whole rectangle serves a parent's or a
/// sibling's answer here.
#[test]
fn a_memoised_chart_restricts_each_cell_as_the_plain_chart_does() {
    let plain = quarter_circle_wall(0.0, 0.0);
    let memo = plain.with_rect_memo();
    let mut level = vec![((0.0, 1.0), (0.0, 1.0))];
    for depth in 0..4 {
        for &(u, v) in &level {
            let want_net = plain.rect_net(u, v).map(|n| net_bits(&n));
            let want_jet = plain.rect_jet(u, v).map(jet_bits);
            assert!(
                want_net.is_some() && want_jet.is_some(),
                "depth {depth}: the cell {u:?} × {v:?} is inside the domain"
            );
            for pass in ["first", "second"] {
                assert_eq!(
                    memo.rect_net(u, v).map(|n| net_bits(&n)),
                    want_net,
                    "depth {depth}, {pass} read: the memo's net of {u:?} × {v:?}"
                );
                assert_eq!(
                    memo.rect_jet(u, v).map(jet_bits),
                    want_jet,
                    "depth {depth}, {pass} read: the memo's jet of {u:?} × {v:?}"
                );
                for (a, b) in [
                    (u.0, v.0),
                    (u.1, v.1),
                    (0.5 * (u.0 + u.1), 0.5 * (v.0 + v.1)),
                ] {
                    let at = |x: f64| Interval::from_bounds(x, x);
                    let bits = |j: geom::SurfaceJet<Interval>| {
                        jet_bits((j.du, j.dv, j.duv))
                            .into_iter()
                            .chain(jet_bits((
                                Vec3::new(j.point.x, j.point.y, j.point.z),
                                j.duu,
                                j.dvv,
                            )))
                            .collect::<Vec<_>>()
                    };
                    assert_eq!(
                        bits(memo.point_ders(a, b)),
                        bits(plain.ders(at(a), at(b))),
                        "depth {depth}, {pass} read: the memo's point jet at ({a}, {b})"
                    );
                }
            }
        }
        level = level
            .iter()
            .flat_map(|&(u, v)| {
                let m = |(a, b): (f64, f64)| 0.5 * (a + b);
                let (mu, mv) = (m(u), m(v));
                [
                    ((u.0, mu), (v.0, mv)),
                    ((u.0, mu), (mv, v.1)),
                    ((mu, u.1), (v.0, mv)),
                    ((mu, u.1), (mv, v.1)),
                ]
            })
            .collect();
    }
}
