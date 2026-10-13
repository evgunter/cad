//! **The rational stretch and area floors**
//! (`NurbsSurface::stretch_floor` / `area_floor`, the tensor-cell
//! pair-term floors of `geom_core::spline::rational_stretch_floor` and
//! `rational_area_floor`): one named row per clause of the proof.
//!
//! A clause whose removal makes a floor UNSOUND has a row whose fixture
//! puts the true minimum below what the floor without that clause would
//! claim; a LOCALITY clause, whose removal only weakens the floor, has a
//! row that pins the floor at the true minimum, so a floor that drops
//! (to zero everywhere, at the extreme) goes red. Random fuzz witnesses
//! none of this: each fixture is written down.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom/src/surfaces/", "crates/geom-core/src/spline/",];

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

fn surface(
    (ku, pu): (Vec<f64>, usize),
    (kv, pv): (Vec<f64>, usize),
    pts: Vec<Point3<f64>>,
    w: Vec<f64>,
) -> NurbsSurface<f64> {
    NurbsSurface::new(
        KnotVector::clamped(ku, pu).unwrap(),
        KnotVector::clamped(kv, pv).unwrap(),
        pts,
        w,
    )
    .unwrap()
}

/// The clamped knot vector of degree `p` over the `breaks`, each
/// interior one simple.
fn clamped(breaks: &[f64], p: usize) -> Vec<f64> {
    let (first, last) = (breaks[0], breaks[breaks.len() - 1]);
    let mut k = vec![first; p];
    k.extend_from_slice(breaks);
    k.extend(std::iter::repeat_n(last, p));
    k
}

/// A patch ruled along `z`: degree `p` in `u` over the breaks `ku` through the
/// abscissae `xs` with weights `ws`, and two identical columns `z = 0`
/// and `z = 1` (degree 1 in `v`). Its `S_u` is the rational curve's
/// derivative along `x`, independent of `v`, and `S_v = ẑ`.
fn ruled(ku: Vec<f64>, p: usize, xs: &[f64], ws: &[f64]) -> NurbsSurface<f64> {
    let mut pts = Vec::new();
    let mut w = Vec::new();
    for (x, wi) in xs.iter().zip(ws) {
        for z in [0.0, 1.0] {
            pts.push(Point3::new(*x, 0.0, z));
            w.push(*wi);
        }
    }
    surface((clamped(&ku, p), p), (vec![0.0, 0.0, 1.0, 1.0], 1), pts, w)
}

/// The least certified UPPER end of `f(jet)` over an `(n + 1)²` grid:
/// at least the true minimum, so a floor above it is unsound with
/// certainty.
fn sampled_min(
    s: &NurbsSurface<f64>,
    n: usize,
    f: impl Fn(Vec3<Interval>, Vec3<Interval>) -> Interval,
) -> f64 {
    let si = s.map_scalar(<Interval as Real>::from_f64);
    let (u0, u1) = s.knots_u().domain();
    let (v0, v1) = s.knots_v().domain();
    let mut least = f64::INFINITY;
    for i in 0..=n {
        for j in 0..=n {
            #[allow(clippy::cast_precision_loss)]
            let (u, v) = (
                (u0 + (u1 - u0) * i as f64 / n as f64).clamp(u0, u1),
                (v0 + (v1 - v0) * j as f64 / n as f64).clamp(v0, v1),
            );
            let jet = si.ders(Interval::from_f64(u), Interval::from_f64(v));
            least = least.min(f(jet.du, jet.dv).hi());
        }
    }
    least
}

fn lift(d: Vec3<f64>) -> Vec3<Interval> {
    d.map(<Interval as Real>::from_f64)
}

/// `min S_u·d` (`along_u`) or `min S_v·d`, sampled.
fn sampled_stretch(s: &NurbsSurface<f64>, along_u: bool, d: Vec3<f64>) -> f64 {
    let d = lift(d);
    sampled_min(s, 64, |du, dv| if along_u { du.dot(d) } else { dv.dot(d) })
}

/// `min (S_u × S_v)·d`, sampled.
fn sampled_area(s: &NurbsSurface<f64>, d: Vec3<f64>) -> f64 {
    let d = lift(d);
    sampled_min(s, 64, |du, dv| du.cross(dv).dot(d))
}

/// The floor is sound against the sampled minimum, and within `slack`
/// of it (`slack = None`: soundness only).
#[track_caller]
fn floor_holds(name: &str, floor: f64, sampled: f64, slack: Option<f64>) {
    assert!(
        floor <= sampled,
        "{name}: floor {floor} is above the sampled minimum {sampled}"
    );
    if let Some(slack) = slack {
        assert!(
            floor >= sampled - slack,
            "{name}: floor {floor} is more than {slack} below the minimum {sampled}"
        );
    }
}

/// The rational linear `x(u)` from `0` to `4` with weights `(1, 4)`:
/// `x′ = w₀w₁·4/W²` runs from `16` at `u = 0` down to `1` at `u = 1`.
fn rising() -> NurbsSurface<f64> {
    ruled(vec![0.0, 1.0], 1, &[0.0, 4.0], &[1.0, 4.0])
}

/// **A negative projection divides by the LEAST weight.** Along `−x`
/// the term values are `−16` and `−4`, and the true minimum is `−16`,
/// at `W = 1`: over the greatest weight the floor would read `−4`.
#[test]
fn a_negative_projection_divides_by_the_least_weight() {
    let s = rising();
    let d = Vec3::new(-1.0, 0.0, 0.0);
    floor_holds(
        "S_u·(−x)",
        s.stretch_floor(true, d),
        sampled_stretch(&s, true, d),
        Some(1e-9),
    );
}

/// **A non-negative projection divides by the GREATEST weight.** Along
/// `x` the least term value is `4` and the true minimum `1`, at
/// `W = 4`: over the least weight the floor would read `4`.
#[test]
fn a_positive_projection_divides_by_the_greatest_weight() {
    let s = rising();
    let d = Vec3::new(1.0, 0.0, 0.0);
    floor_holds(
        "S_u·x",
        s.stretch_floor(true, d),
        sampled_stretch(&s, true, d),
        Some(1e-9),
    );
}

/// **Every active column's pairs are read.** On this bilinear patch the
/// `u` edges differ, `4x̂` at `v = 0` and `x̂ − 2ŷ` at `v = 1`, so
/// `S_u·x̂ = 4 − 3v` falls to `1` at the second column; the `v` edges
/// read `3ŷ` at `u = 0` and `−3x̂ + ŷ` at `u = 1`, so `S_v·ŷ` falls to
/// `1` at the second row. Read over the first column (row) alone the
/// floors claim `4` and `3`.
#[test]
fn every_active_column_is_read() {
    let s = surface(
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 3.0, 0.0),
            Point3::new(4.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ],
        vec![1.0; 4],
    );
    for (name, along_u, d) in [
        ("S_u·x", true, Vec3::new(1.0, 0.0, 0.0)),
        ("S_v·y", false, Vec3::new(0.0, 1.0, 0.0)),
    ] {
        floor_holds(
            name,
            s.stretch_floor(along_u, d),
            sampled_stretch(&s, along_u, d),
            Some(1e-9),
        );
    }
}

/// **Every term is read at every active point.** The least term value
/// along `x` is the pair's term at its FAR end (`w₀·4` at `P₁`); read
/// at the pair's own start, or at the cell's first active point only,
/// it is `w₁·4 = 16`, a floor of `4` against the true `1`. The degree-2
/// arc is the case where the least value sits at a point that is not
/// either end of the pair whose term it is.
#[test]
fn the_term_is_read_at_every_active_point() {
    let s = rising();
    let d = Vec3::new(1.0, 0.0, 0.0);
    floor_holds(
        "linear",
        s.stretch_floor(true, d),
        sampled_stretch(&s, true, d),
        None,
    );
    let arc = ruled(vec![0.0, 1.0], 2, &[0.0, 1.0, 1.2], &[1.0, 3.0, 9.0]);
    floor_holds(
        "quadratic",
        arc.stretch_floor(true, d),
        sampled_stretch(&arc, true, d),
        None,
    );
}

/// **The weight divides once per derivative factor.** Scaling every
/// weight leaves the surface unchanged and scales every term value with
/// it (and every cross with its square), so the floors are invariant:
/// `S_u` over `W`, the area element over `W²`. A floor over the wrong
/// power of the weight moves with the scale.
#[test]
fn uniformly_scaled_weights_read_the_same_floors() {
    let base = rising();
    let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
    let (f0, a0) = (base.stretch_floor(true, x), base.area_floor(y));
    for k in [1e-3, 1e3] {
        let s = ruled(vec![0.0, 1.0], 1, &[0.0, 4.0], &[k, 4.0 * k]);
        let (f, a) = (s.stretch_floor(true, x), s.area_floor(y));
        assert!(
            (f - f0).abs() <= 1e-12 * f0.abs(),
            "stretch at ×{k}: {f} vs {f0}"
        );
        assert!(
            (a - a0).abs() <= 1e-12 * a0.abs(),
            "area at ×{k}: {a} vs {a0}"
        );
        floor_holds("stretch", f, sampled_stretch(&s, true, x), None);
        floor_holds("area", a, sampled_area(&s, y), None);
    }
}

/// **The area floor divides by its weight squared by the sign.** On the
/// ruled patch `S_u × S_v = −x′·ŷ`, `x′ ∈ [1, 16]`: along `−ŷ` the
/// least cross is `4·1` over `w_max² = 16`, and along `ŷ` it is
/// `−16·4` over `w_min² = 1` against the true `−16`. Either divisor on
/// the wrong side claims above the true minimum.
#[test]
fn the_area_floor_divides_by_the_squared_weight_by_its_sign() {
    let s = rising();
    for (name, d) in [
        ("along −y", Vec3::new(0.0, -1.0, 0.0)),
        ("along y", Vec3::new(0.0, 1.0, 0.0)),
    ] {
        floor_holds(name, s.area_floor(d), sampled_area(&s, d), None);
    }
}

/// **Both factors of a cross are read at one point.** `T(S) × U(S)` is
/// affine in `S` only with `S` the same in both factors. On these two
/// bilinear patches, whose weights differ along both directions, the
/// floor with the `v` term (first) or the `u` term (second) read at the
/// cell's first point instead claims `0.461` against a sampled `0.398`
/// along `ẑ`, and `−1.80` against `−2.04` along `−ẑ` (fixtures a search
/// over random bilinear patches found).
#[test]
fn both_factors_of_a_cross_are_read_at_one_point() {
    let bilinear = |pts: [[f64; 3]; 4], w: [f64; 4]| {
        surface(
            (vec![0.0, 0.0, 1.0, 1.0], 1),
            (vec![0.0, 0.0, 1.0, 1.0], 1),
            pts.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(),
            w.to_vec(),
        )
    };
    let v_side = bilinear(
        [
            [
                0.05115414404285357,
                0.04020158133436242,
                0.21194913761987055,
            ],
            [0.13212597668765905, 1.0265534146353426, 0.13013743691059915],
            [1.1637266203245495, 0.20790745013085654, 0.3805128156857206],
            [1.1766205467159978, 1.0392927022611977, 0.4327114026119544],
        ],
        [
            1.419592622849598,
            2.3982519986774027,
            1.363705211963951,
            2.671366776744994,
        ],
    );
    let u_side = bilinear(
        [
            [
                0.0007061127840568227,
                0.017130348865978882,
                0.3848033683622043,
            ],
            [0.14647573952206644, 1.1149017961519276, 0.21109946641136085],
            [1.1758969642004764, 0.010161451808882194, 0.3828131654380899],
            [1.235183960617558, 1.1958826043902469, 0.39393879758853545],
        ],
        [
            3.4566489259400695,
            2.858855350262942,
            2.5153666820030725,
            2.6793620497527,
        ],
    );
    for (name, s, d) in [
        ("v term, along z", v_side, Vec3::new(0.0, 0.0, 1.0)),
        ("u term, along −z", u_side, Vec3::new(0.0, 0.0, -1.0)),
    ] {
        floor_holds(name, s.area_floor(d), sampled_area(&s, d), None);
    }
}

/// **A cell reads its own weights.** Over knots `[0, 1, 2]`, `x` runs
/// `0 → 2` at unit weights (`x′ = 2`) and then `2 → 6` with weights
/// `(1, 4)` (`x′` down to `1`). The floor is the true `1`; over the
/// patch's weight range the unit-weight cell would read `2/4`.
#[test]
fn a_cell_reads_its_own_weights() {
    let s = ruled(vec![0.0, 1.0, 2.0], 1, &[0.0, 2.0, 6.0], &[1.0, 1.0, 4.0]);
    let d = Vec3::new(1.0, 0.0, 0.0);
    floor_holds(
        "two cells",
        s.stretch_floor(true, d),
        sampled_stretch(&s, true, d),
        Some(1e-9),
    );
}

/// **A cell reads its own points.** `x` runs `0 → 2` at weight 4
/// (`x′ = 2`) and then `2 → 6` with weights `(4, 1)` (`x′ = 16/W²`, `1`
/// at `u = 1`). The floor is the true `1`; with the first cell's `x = 0`
/// standing for `S`, the second cell's term reads `4 + 3·(0 − 2) < 0`.
#[test]
fn a_cell_reads_its_own_points() {
    let s = ruled(vec![0.0, 1.0, 2.0], 1, &[0.0, 2.0, 6.0], &[4.0, 4.0, 1.0]);
    let d = Vec3::new(1.0, 0.0, 0.0);
    floor_holds(
        "two cells",
        s.stretch_floor(true, d),
        sampled_stretch(&s, true, d),
        Some(1e-9),
    );
}

/// **The floors are not vacuous at unit weights.** A bilinear rectangle
/// `[0, 2] × [0, 3]` has `S_u = 2x̂`, `S_v = 3ŷ` and area element `6`
/// everywhere, and the floors read exactly that.
#[test]
fn a_unit_weight_rectangle_reads_its_exact_floors() {
    let s = surface(
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 3.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 3.0, 0.0),
        ],
        vec![1.0; 4],
    );
    assert_eq!(s.stretch_floor(true, Vec3::new(1.0, 0.0, 0.0)), 2.0);
    assert_eq!(s.stretch_floor(false, Vec3::new(0.0, 1.0, 0.0)), 3.0);
    assert_eq!(s.area_floor(Vec3::new(0.0, 0.0, 1.0)), 6.0);
}
