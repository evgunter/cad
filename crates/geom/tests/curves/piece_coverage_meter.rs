//! **Every piece of every span is read, and the join keeps what each
//! assembly contributes.** The integral speed meter's piece assembly
//! cuts each span into sixteen pieces, and a stall that only one piece
//! sees must still drive the meter to a non-positive answer: these rows
//! plant stalls at every piece boundary and every piece midpoint, inside
//! multi-span curves, at random interior parameters and at extreme
//! scales. The join rows pin the two chord assemblies where they beat
//! the pieces, and the one cell where only the pieces abstain.
//!
//! A stall's reference is the enclosure ceiling of `‖C′‖` at the stall
//! (`ceiling_at`): at least the true speed there, so a meter above it is
//! unsound with certainty.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// `!(m > x)` is deliberate: a NaN meter abstains, which is never a pass
// of a soundness row by accident.
#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::cast_precision_loss)]

test_utils::gated_to![
    "crates/geom/src/curves/",
    "crates/geom/src/curves.rs",
    "crates/geom-core/src/spline/",
];

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz;

fn ceiling_at(c: &NurbsCurve3<f64>, t: f64) -> f64 {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    ci.deriv(Interval::from_f64(t)).norm().hi()
}

/// The meter at `f64` and the certified lane's floor.
fn meters(c: &NurbsCurve3<f64>) -> (f64, f64) {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    (
        c.speed_lower_bound().get(),
        ci.speed_lower_bound().get().lo(),
    )
}

fn binom(n: usize, k: usize) -> f64 {
    (0..k).fold(1.0, |r, i| r * (n - i) as f64 / (i + 1) as f64)
}

/// A one-span degree-`p` curve on `[off, off + len]` whose derivative,
/// in the span's unit parameter `s`, is the power-form polynomial
/// `Σᵢ aᵢ sⁱ` (vector coefficients, degree ≤ `p − 1`): the net is the
/// running sum of its Bernstein coefficients over `p`.
fn bezier_with_derivative(p: usize, a: &[Vec3<f64>], off: f64, len: f64) -> NurbsCurve3<f64> {
    let q = p - 1;
    let mut pts = vec![Point3::new(1.0e3, -7.0, 3.0)];
    for k in 0..=q {
        let beta = a
            .iter()
            .enumerate()
            .take(k + 1)
            .fold(Vec3::new(0.0, 0.0, 0.0), |acc, (i, ai)| {
                acc + *ai * (binom(k, i) / binom(q, i))
            });
        let last = *pts.last().unwrap();
        pts.push(last + beta * (1.0 / p as f64));
    }
    let mut knots = vec![off; p + 1];
    knots.extend(std::iter::repeat_n(off + len, p + 1));
    NurbsCurve3::new(
        KnotVector::clamped(knots, p).unwrap(),
        pts,
        vec![1.0; p + 1],
    )
    .unwrap()
}

/// `C′(s) = (s − s*)·w + (s − s*)²·v` (a crossing stall) or, with
/// `touching`, `(s − s*)²·w + (s − s*)³·v` (a stall the derivative only
/// touches), in power form.
fn stall(s: f64, w: Vec3<f64>, v: Vec3<f64>, touching: bool) -> Vec<Vec3<f64>> {
    if touching {
        // (s − c)² = c² − 2cs + s²; (s − c)³ = −c³ + 3c²s − 3cs² + s³.
        vec![
            w * (s * s) - v * (s * s * s),
            w * (-2.0 * s) + v * (3.0 * s * s),
            w - v * (3.0 * s),
            v,
        ]
    } else {
        vec![v * (s * s) - w * s, w - v * (2.0 * s), v]
    }
}

/// A stall at every piece boundary `k/16` and every piece midpoint
/// `(2k+1)/32`, crossing and touching, alone and inside a refined
/// multi-span curve, on the unit span: the certified floor never
/// exceeds the speed at the stall, and the `f64` reading stays within
/// its stated posture (a relative ulp of the derivative's scale) of it.
/// A schedule that skips any one piece of a span is caught here.
#[test]
fn a_stall_in_any_piece_refuses() {
    stalls_in_every_piece(&[3, 6], &[(0.0, 1.0)]);
}

/// [`a_stall_in_any_piece_refuses`] across degrees and on short, long
/// and far-off spans.
#[test]
fn a_stall_in_any_piece_refuses_at_any_scale() {
    stalls_in_every_piece(
        &[3, 4, 6, 9],
        &[(1.0e6, 1.0), (0.0, 1.0e-6), (0.0, 1.0e6), (-3.7, 0.3)],
    );
}

fn stalls_in_every_piece(degrees: &[usize], spans: &[(f64, f64)]) {
    let w = Vec3::new(3.1, 0.7, -1.3);
    let v = Vec3::new(-0.2, 1.9, 0.6);
    let mut rows = 0;
    for &p in degrees {
        for touching in [false, true] {
            if touching && p < 5 {
                continue;
            }
            for &(off, len) in spans {
                for k in 0..=32 {
                    let s = f64::from(k) / 32.0;
                    let one = bezier_with_derivative(p, &stall(s, w, v, touching), off, len);
                    let refined = one
                        .refine_knots(&[off + 0.3 * len, off + 0.61 * len])
                        .unwrap();
                    for (shape, c) in [("one span", &one), ("three spans", &refined)] {
                        let t = off + len * s;
                        let reference = ceiling_at(c, t);
                        let (m, mi) = meters(c);
                        let label =
                            format!("p {p} touching {touching} off {off} len {len} s* {s} {shape}");
                        assert!(
                            !(mi > reference),
                            "{label}: certified floor {mi:e} above the stall's speed {reference:e}"
                        );
                        assert!(
                            !(m > reference + 1e-12 * (w.norm() + v.norm()) / len),
                            "{label}: f64 meter {m:e} above the stall's speed {reference:e}"
                        );
                        rows += 1;
                    }
                }
            }
        }
    }
    assert!(
        rows >= 66 * degrees.len() * spans.len(),
        "the enumeration ran {rows} rows"
    );
}

/// The same stall far outside unit scale: derivative coefficients from
/// `1e-150` to `1e150`, at piece boundaries and inside pieces.
#[test]
fn a_stall_at_extreme_scales_refuses() {
    let w = Vec3::new(3.1, 0.7, -1.3);
    let v = Vec3::new(-0.2, 1.9, 0.6);
    for scale in [1.0e-150, 1.0e-100, 1.0e100, 1.0e150] {
        for k in [0, 1, 7, 16, 17, 31, 32] {
            let s = f64::from(k) / 32.0;
            for p in [3, 5] {
                let c = bezier_with_derivative(p, &stall(s, w * scale, v * scale, false), 0.0, 1.0);
                let reference = ceiling_at(&c, s);
                let (m, mi) = meters(&c);
                assert!(
                    !(mi > reference) && !(m > reference + 1e-12 * scale),
                    "p {p} scale {scale:e} s* {s}: meter {m:e} / {mi:e}, speed {reference:e}"
                );
            }
        }
    }
}

/// **Counterexample search**: random multi-span nets with a stall placed
/// at a random interior parameter — one control point moved so that
/// `C′(t*)` vanishes to rounding. Neither reading may exceed the speed
/// at `t*`.
#[test]
fn a_random_interior_stall_refuses() {
    let mut rng = fuzz::start("piece_coverage_meter::interior_stall");
    let cases = fuzz::scaled(150);
    let mut built = 0_usize;
    for case in 0..cases {
        let deg = 2 + case % 7;
        let npts = deg + 1 + rng.below(8);
        let mut control: Vec<Point3<f64>> = (0..npts)
            .map(|_| {
                Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                )
            })
            .collect();
        let off = if case % 4 == 0 { 1.0e5 } else { 0.0 };
        let interior = npts - deg - 1;
        let mut knots = vec![off; deg + 1];
        let (mut u, mut i) = (off, 0);
        while i < interior {
            u += rng.range(0.05, 1.2);
            let mult = (1 + if rng.unit() < 0.3 { rng.below(deg) } else { 0 }).min(interior - i);
            knots.extend(std::iter::repeat_n(u, mult));
            i += mult;
        }
        u += rng.range(0.05, 1.2);
        knots.extend(std::iter::repeat_n(u, deg + 1));
        let Ok(kv) = KnotVector::clamped(knots, deg) else {
            continue;
        };
        let t = off + rng.unit() * (u - off);
        // `N′ⱼ(t*)` per basis function, through unit nets.
        let basis_slope: Vec<f64> = (0..npts)
            .map(|j| {
                let unit: Vec<Point3<f64>> = (0..npts)
                    .map(|k| Point3::new(if k == j { 1.0 } else { 0.0 }, 0.0, 0.0))
                    .collect();
                NurbsCurve3::new(kv.clone(), unit, vec![1.0; npts])
                    .unwrap()
                    .deriv(t)
                    .x
            })
            .collect();
        let d = NurbsCurve3::new(kv.clone(), control.clone(), vec![1.0; npts])
            .unwrap()
            .deriv(t);
        let (j, slope) = basis_slope
            .iter()
            .copied()
            .enumerate()
            .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .unwrap();
        control[j] = control[j] - d * (1.0 / slope);
        let c = NurbsCurve3::new(kv, control, vec![1.0; npts]).unwrap();
        built += 1;
        let reference = ceiling_at(&c, t);
        let (m, mi) = meters(&c);
        assert!(
            !(mi > reference) && !(m > reference + 1e-12),
            "case {case} (deg {deg}, {npts} pts, t* {t}): meter {m:e} / {mi:e}, speed {reference:e} — {}",
            fuzz::replay()
        );
    }
    assert!(
        built * 2 >= cases,
        "fuzz rot: {built} of {cases} nets — {}",
        fuzz::replay()
    );
}

/// The global-chord assembly, spelled here from its definition: the
/// first→last control chord's unit direction against every derivative
/// coefficient.
fn global_chord(c: &NurbsCurve3<f64>) -> f64 {
    let (first, last) = (c.control()[0], *c.control().last().unwrap());
    let chord = last - first;
    let d = chord / chord.norm();
    derivative_coeffs(c)
        .iter()
        .map(|q| d.dot(*q))
        .fold(f64::INFINITY, f64::min)
}

/// The per-span chord assembly, spelled here: each nonempty span's
/// control chord against its active derivative coefficients.
fn per_span_chord(c: &NurbsCurve3<f64>) -> f64 {
    let (p, knots, net) = (c.knots().degree(), c.knots().knots(), c.control());
    let q = derivative_coeffs(c);
    let mut best = f64::INFINITY;
    for s in p..net.len() {
        if knots[s] == knots[s + 1] {
            continue;
        }
        let chord = net[s] - net[s - p];
        let d = chord / chord.norm();
        for qi in &q[s - p..s] {
            best = best.min(d.dot(*qi));
        }
    }
    best
}

fn derivative_coeffs(c: &NurbsCurve3<f64>) -> Vec<Vec3<f64>> {
    let (p, knots, net) = (c.knots().degree(), c.knots().knots(), c.control());
    (0..net.len() - 1)
        .map(|i| (net[i + 1] - net[i]) * (p as f64 / (knots[i + p + 1] - knots[i + 1])))
        .collect()
}

fn planar(knots: &[f64], degree: usize, xy: &[(f64, f64)]) -> NurbsCurve3<f64> {
    let net: Vec<Point3<f64>> = xy.iter().map(|&(x, y)| Point3::new(x, y, 0.0)).collect();
    let n = net.len();
    NurbsCurve3::new(
        KnotVector::clamped(knots.to_vec(), degree).unwrap(),
        net,
        vec![1.0; n],
    )
    .unwrap()
}

/// **The two chord assemblies each still decide a carrier.** A jagged,
/// nearly straight quadratic whose pieces each lean off the chord reads
/// its best bound off the global chord, and a cubic reads it off the
/// per-span chords; on each the meter is that assembly's value bit for
/// bit, above the other chord assembly. Found by a seeded search over
/// 20 000 random, near-straight and swaying nets (the global chord won
/// on several near-straight quadratics, the per-span chords on one
/// cubic).
#[test]
fn each_chord_assembly_still_decides_a_carrier() {
    let global_wins = planar(
        &[
            0.0,
            0.0,
            0.0,
            0.5494306202116541,
            1.5333612871610138,
            2.1994419454604666,
            2.688855165735114,
            3.4721217338420827,
            3.8516209968410866,
            4.839159757995035,
            4.839159757995035,
            4.839159757995035,
        ],
        2,
        &[
            (-0.2040435399387167, -0.1857579770822676),
            (1.2931163226299747, 0.16490647565138256),
            (2.102158015062652, 0.15712719291495614),
            (3.2253447435483307, 0.18022331845149325),
            (4.183161431706938, -0.16449944791768426),
            (5.1382549659784065, -0.23442523183592373),
            (6.0705183675798855, 0.10771386107516989),
            (7.02853514002726, -0.10074544102959357),
            (8.070232897126282, -0.09780996691956376),
        ],
    );
    let span_wins = planar(
        &[
            0.0,
            0.0,
            0.0,
            0.0,
            1.1708792980142826,
            1.5793340814211976,
            1.5793340814211976,
            1.5793340814211976,
            1.5793340814211976,
        ],
        3,
        &[
            (0.2453794525925453, -0.15618792727756828),
            (0.8996062963056511, -0.1995838109320915),
            (1.8863894995744546, 0.2790848787255656),
            (3.270863202554688, -0.2561338545089181),
            (4.02716411841858, 0.26566859892552724),
        ],
    );
    let m = global_wins.speed_lower_bound().get();
    let (g, s) = (global_chord(&global_wins), per_span_chord(&global_wins));
    assert!(
        m == g && g > s,
        "global chord's carrier: meter {m}, global {g}, per-span {s}"
    );
    let m = span_wins.speed_lower_bound().get();
    let (g, s) = (global_chord(&span_wins), per_span_chord(&span_wins));
    assert!(
        m == s && s > g,
        "per-span chord's carrier: meter {m}, global {g}, per-span {s}"
    );
}

/// **Counterexample search**: the join never reads below either chord
/// assembly (each spelled here from its definition) — the meter before
/// the piece assembly is a floor the meter keeps.
#[test]
fn the_join_never_reads_below_the_chord_assemblies() {
    let mut rng = fuzz::start("piece_coverage_meter::chord_floor");
    let cases = fuzz::scaled(600);
    for case in 0..cases {
        let deg = 2 + case % 5;
        let npts = deg + 1 + rng.below(10);
        let net: Vec<Point3<f64>> = (0..npts)
            .map(|i| match case % 3 {
                0 => Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                ),
                1 => Point3::new(i as f64 + rng.range(-0.3, 0.3), rng.range(-0.3, 0.3), 0.0),
                _ => Point3::new(
                    if i % 2 == 0 { -1.0 } else { 1.0 } * rng.range(0.1, 2.0),
                    0.0,
                    i as f64,
                ),
            })
            .collect();
        let mut knots = vec![0.0; deg + 1];
        let mut u = 0.0;
        for _ in 0..npts - deg - 1 {
            u += rng.range(0.2, 1.2);
            knots.push(u);
        }
        u += rng.range(0.2, 1.2);
        knots.extend(std::iter::repeat_n(u, deg + 1));
        let c = NurbsCurve3::new(
            KnotVector::clamped(knots, deg).unwrap(),
            net,
            vec![1.0; npts],
        )
        .unwrap();
        let m = c.speed_lower_bound().get();
        let floor = global_chord(&c).max(per_span_chord(&c));
        assert!(
            !(m < floor),
            "case {case} (deg {deg}, {npts} pts): meter {m} below the chord assemblies' {floor} — {}",
            fuzz::replay()
        );
    }
}

/// **The cell where only the piece assembly abstains.** The quadratic
/// `0 → (1, 0, 0) → (−30, 0, 0)` turns back at `t = 1/32`, the middle
/// of its first piece, where `C′` runs from `2` to `−2` along `x`: the
/// piece's coefficients sum to zero exactly, its direction is `0/0`,
/// and the piece assembly abstains. Both chord assemblies stand, and
/// the join is theirs: `−2`, real and non-positive.
#[test]
fn only_the_piece_assembly_abstains() {
    let c = NurbsCurve3::<f64>::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(-30.0, 0.0, 0.0),
        ],
        vec![1.0; 3],
    )
    .unwrap();
    let (g, s) = (global_chord(&c), per_span_chord(&c));
    assert!(
        g == -2.0 && s == -2.0,
        "the chord assemblies: global {g}, per-span {s}"
    );
    let (m, mi) = meters(&c);
    assert!(
        m == -2.0,
        "the join is the chord assemblies' answer, got {m}"
    );
    assert!(
        !(mi > 0.0) && !mi.is_nan(),
        "the certified lane agrees: {mi}"
    );
}
