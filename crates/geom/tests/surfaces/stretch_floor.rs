//! **The rational stretch and area floors**
//! (`NurbsSurface::stretch_floor` / `area_floor`, the quotient bound of
//! `geom_core::spline::derivative` on each cell's dyadic pieces): one
//! named row per clause of the proof.
//!
//! A clause whose removal makes a floor UNSOUND has a row whose fixture
//! puts the true minimum below what the floor without that clause would
//! claim; a LOCALITY or tightness clause, whose removal only weakens the
//! floor, has a row that pins the floor near the true minimum, so a
//! floor that drops (to zero everywhere, at the extreme) goes red.
//! Random fuzz witnesses none of this: each fixture is written down.

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
    let (u0, u1) = s.knots_u_f64_interim().domain();
    let (v0, v1) = s.knots_v_f64_interim().domain();
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

/// `min ‖S_u‖` (`along_u`) or `min ‖S_v‖`, sampled.
fn sampled_stretch(s: &NurbsSurface<f64>, along_u: bool) -> f64 {
    sampled_min(s, 64, |du, dv| if along_u { du.norm() } else { dv.norm() })
}

/// `min ‖S_u × S_v‖`, sampled.
fn sampled_area(s: &NurbsSurface<f64>) -> f64 {
    sampled_min(s, 64, |du, dv| du.cross(dv).norm())
}

/// The floor is sound against the sampled minimum, and at least
/// `share` of it (`None`: soundness only).
#[track_caller]
fn floor_holds(name: &str, floor: f64, sampled: f64, share: Option<f64>) {
    assert!(
        floor <= sampled,
        "{name}: floor {floor} is above the sampled minimum {sampled}"
    );
    if let Some(share) = share {
        assert!(
            floor >= share * sampled,
            "{name}: floor {floor} is below {share} of the minimum {sampled}"
        );
    }
}

/// The rational linear `x(u)` from `0` to `4` with weights `(1, 4)`:
/// `x′ = w₀w₁·4/W²` runs from `16` at `u = 0` down to `1` at `u = 1`,
/// and the area element with it (`S_v = ẑ`).
fn rising() -> NurbsSurface<f64> {
    ruled(vec![0.0, 1.0], 1, &[0.0, 4.0], &[1.0, 4.0])
}

/// A circular arc of radius 1 ruled along `z`, as quadratic rational
/// spans of `2·half` radians each, `spans` of them from angle
/// `start`: span `i`'s middle point at the tangents' meet, weight
/// `cos(half)`.
fn arc(start: f64, half: f64, spans: usize) -> NurbsSurface<f64> {
    let mut xs = Vec::new();
    let mut ws = Vec::new();
    let mut breaks = vec![0.0];
    for i in 0..spans {
        #[allow(clippy::cast_precision_loss)]
        let a = start + 2.0 * half * i as f64;
        if i == 0 {
            xs.push(Point3::new(a.cos(), a.sin(), 0.0));
            ws.push(1.0);
        }
        let m = a + half;
        xs.push(Point3::new(m.cos() / half.cos(), m.sin() / half.cos(), 0.0));
        ws.push(half.cos());
        let e = a + 2.0 * half;
        xs.push(Point3::new(e.cos(), e.sin(), 0.0));
        ws.push(1.0);
        #[allow(clippy::cast_precision_loss)]
        breaks.push((i + 1) as f64);
    }
    // Double interior knots: each span is its own quadratic arc.
    let mut ku = vec![0.0; 3];
    for b in &breaks[1..breaks.len() - 1] {
        ku.extend([*b, *b]);
    }
    ku.extend([breaks[breaks.len() - 1]; 3]);
    let (mut pts, mut w) = (Vec::new(), Vec::new());
    for (x, wi) in xs.iter().zip(&ws) {
        for z in [0.0, 1.0] {
            pts.push(*x + Vec3::new(0.0, 0.0, z));
            w.push(*wi);
        }
    }
    surface((ku, 2), (vec![0.0, 0.0, 1.0, 1.0], 1), pts, w)
}

/// **The floors are not vacuous at unit weights.** A bilinear rectangle
/// `[0, 2] × [0, 3]` has `S_u = 2x̂`, `S_v = 3ŷ` and area element `6`
/// everywhere, and the floors read exactly that: every ratio is the
/// constant itself, and a floor that reads zero everywhere goes red.
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
    assert_eq!(
        (
            s.stretch_floor(true),
            s.stretch_floor(false),
            s.area_floor()
        ),
        (2.0, 3.0, 6.0)
    );
}

/// **Each ratio divides by its own weight power, not the least.** On
/// [`rising`] the numerator is the constant `16x̂` and `W²` runs from
/// `1` to `16`: the last piece's ratios are `16/6.25`, `16/10` and
/// `16/16`, so the floor is the true `1`. Over the piece's least
/// weight power every ratio would read `16/6.25 = 2.56`.
#[test]
fn each_ratio_divides_by_its_own_weight_power() {
    let s = rising();
    floor_holds(
        "S_u",
        s.stretch_floor(true),
        sampled_stretch(&s, true),
        Some(1.0 - 1e-9),
    );
    floor_holds("area", s.area_floor(), sampled_area(&s), Some(1.0 - 1e-9));
}

/// **The weight power is the derivative's own.** Scaling every weight
/// leaves the surface unchanged and scales the numerators and the
/// weight powers alike, so the floors are invariant to rounding: `N_u`
/// over `W²`, `M̃` over `W³`. A floor over the wrong power of the
/// weight moves with the scale.
#[test]
fn uniformly_scaled_weights_read_the_same_floors() {
    let base = arc(0.3, 0.6, 2);
    let f0 = [
        base.stretch_floor(true),
        base.stretch_floor(false),
        base.area_floor(),
    ];
    for k in [1e-3, 1e3] {
        let s = NurbsSurface::new(
            base.knots_u_f64_interim().clone(),
            base.knots_v_f64_interim().clone(),
            base.control().to_vec(),
            base.weights().iter().map(|w| w * k).collect(),
        )
        .unwrap();
        let f = [
            s.stretch_floor(true),
            s.stretch_floor(false),
            s.area_floor(),
        ];
        for (which, (a, b)) in ["u", "v", "area"].iter().zip(f.iter().zip(&f0)) {
            assert!(
                (a - b).abs() <= 1e-12 * b.abs(),
                "{which} at ×{k}: {a} against {b}"
            );
        }
    }
}

/// **A rational half-cylinder reads positive.** Its `S_u` turns
/// through half a revolution, so one direction for the whole chart
/// meets it at a right angle or worse somewhere and floors it at zero;
/// each piece's own direction (the sum of its numerator's
/// coefficients) floors every piece's quarter-turn at most well above
/// zero. The floors read above `0.9` of the arc's least speed (`√2`);
/// one direction for the chart reads `0`.
#[test]
fn a_rational_half_cylinder_reads_positive() {
    let s = arc(0.0, core::f64::consts::FRAC_PI_4, 2);
    let (fu, fa) = (s.stretch_floor(true), s.area_floor());
    floor_holds("S_u", fu, sampled_stretch(&s, true), Some(0.9));
    floor_holds("area", fa, sampled_area(&s), Some(0.9));
    floor_holds(
        "S_v",
        s.stretch_floor(false),
        sampled_stretch(&s, false),
        Some(1.0 - 1e-9),
    );
}

/// **The piece schedule reaches a wide arc.** One quadratic span
/// turning through `170°`: read as the cell's one Bézier piece, its
/// ratios fan out around the cell's mean direction and the floor reads
/// `0.17` of the arc's least speed; on the schedule's pieces each turns
/// through a fraction of it and the floor reads above `0.8` of it,
/// below the sampled truth.
#[test]
fn the_piece_schedule_reaches_a_wide_arc() {
    let s = arc(0.0, 85f64.to_radians(), 1);
    floor_holds(
        "S_u",
        s.stretch_floor(true),
        sampled_stretch(&s, true),
        Some(0.8),
    );
    floor_holds("area", s.area_floor(), sampled_area(&s), Some(0.8));
}

/// **The floors are sound on the named carriers** and on the bilinear
/// patches whose weights differ along both directions (fixtures a
/// search found against an earlier floor's cross clause): each floor at
/// most the sampled minimum.
#[test]
fn the_floors_hold_on_curved_rational_nets() {
    let bilinear = |pts: [[f64; 3]; 4], w: [f64; 4]| {
        surface(
            (vec![0.0, 0.0, 1.0, 1.0], 1),
            (vec![0.0, 0.0, 1.0, 1.0], 1),
            pts.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(),
            w.to_vec(),
        )
    };
    let nets = [
        ("rising", rising()),
        (
            "quadratic",
            ruled(vec![0.0, 1.0], 2, &[0.0, 1.0, 1.2], &[1.0, 3.0, 9.0]),
        ),
        (
            "two cells, own weights",
            ruled(vec![0.0, 1.0, 2.0], 1, &[0.0, 2.0, 6.0], &[1.0, 1.0, 4.0]),
        ),
        (
            "two cells, own points",
            ruled(vec![0.0, 1.0, 2.0], 1, &[0.0, 2.0, 6.0], &[4.0, 4.0, 1.0]),
        ),
        ("quarter arcs", arc(0.2, 0.4, 3)),
        (
            "bilinear, v side",
            bilinear(
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
            ),
        ),
        (
            "bilinear, u side",
            bilinear(
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
            ),
        ),
    ];
    for (name, s) in &nets {
        for (which, f, t) in [
            ("S_u", s.stretch_floor(true), sampled_stretch(s, true)),
            ("S_v", s.stretch_floor(false), sampled_stretch(s, false)),
            ("area", s.area_floor(), sampled_area(s)),
        ] {
            floor_holds(&format!("{name}: {which}"), f, t, None);
        }
    }
}

/// **The per-axis floors reach past the area floor's ceiling.** A
/// rational Bézier sheet of degree `p` along `u`, ruled along `v`: the
/// area quotient `M̃/W³` sits at degree `3p`, past
/// `QUOTIENT_MAX_DEGREE` (56) from `p = 19`, so the area floor refuses
/// there; `N_u/W²` and `N_v/W²` sit at `2p` and read to `p = 28`, below
/// the sampled truth. At `p = 18` the area floor still reads.
#[test]
fn the_per_axis_floors_reach_past_the_area_ceiling() {
    let sheet = |p: usize| {
        let xs: Vec<f64> = (0..=p).map(|i| i as f64 / p as f64).collect();
        let ws: Vec<f64> = (0..=p).map(|i| 1.0 + 0.3 * (i % 2) as f64).collect();
        let mut k = vec![0.0; p + 1];
        k.extend(vec![1.0; p + 1]);
        let (mut pts, mut w) = (Vec::new(), Vec::new());
        for (i, (x, wi)) in xs.iter().zip(&ws).enumerate() {
            for z in [0.0, 1.0] {
                pts.push(Point3::new(*x, 0.1 * (i % 3) as f64, z));
                w.push(*wi);
            }
        }
        surface((k, p), (vec![0.0, 0.0, 1.0, 1.0], 1), pts, w)
    };
    let s = sheet(18);
    floor_holds("p 18 area", s.area_floor(), sampled_area(&s), None);
    assert!(s.area_floor() > 0.0, "p 18: area floor {}", s.area_floor());
    for p in [19, 28] {
        let s = sheet(p);
        for (which, along_u) in [("S_u", true), ("S_v", false)] {
            let f = s.stretch_floor(along_u);
            assert!(f > 0.0, "p {p} {which}: floor {f}");
            floor_holds(
                &format!("p {p} {which}"),
                f,
                sampled_stretch(&s, along_u),
                None,
            );
        }
        assert!(
            s.area_floor().is_nan(),
            "p {p}: area floor {} past its ceiling",
            s.area_floor()
        );
    }
}
