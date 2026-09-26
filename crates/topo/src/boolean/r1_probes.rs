//! Ray-torus root certification held against oracles that share no
//! code with it.
//!
//! * The CERTIFIED ROOT COUNT and the roots themselves answer to a
//!   geometric counter that never touches the quartic's coefficients:
//!   it samples the torus's own implicit `F` along the ray and bisects
//!   every sign change. Where the two disagree in count, in a root's
//!   value, or over a miss the oracle can see through, the rows say so
//!   by pose regime.
//! * The `sqrt`-chain CUBE ROOT answers to `f64::cbrt` across twelve
//!   decades either side of 1 and both signs, and its truncation is
//!   registered as a systematic bias rather than an enclosure — the
//!   magnitude argument `solid_contain`'s `cbrt` docs rest on.
//! * The BIQUADRATIC SIGN answers to the same geometric oracle, on the
//!   rays that reach that arm.
//!
//! The independence is the content: an oracle built from the quartic's
//! own algebra would agree with a wrong certifier. Two rows carry the
//! count claim between them — the deterministic enumeration of the
//! pose regimes the geometry names, HERE, and the sweep over poses
//! nobody chose, which lives in `solid_contain/r1_generic_poses.rs`.
//! The sweep is a file of its own because a gate restricting it to
//! the code it tests gates a whole file's module, and these pins run
//! on every leg.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use super::*;
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The torus's own implicit function along the ray — the geometry, not
/// the quartic. `F(t) = (|w|² + R² − r²)² − 4R²ρ²`.
fn f_geom(
    q: Point3<f64>,
    d: Vec3<f64>,
    c: Point3<f64>,
    a: Vec3<f64>,
    rr: f64,
    r: f64,
    t: f64,
) -> f64 {
    let w = (q - c) + d * t;
    let h = w.dot(a);
    let rho2 = w.norm_squared() - h * h;
    (w.norm_squared() + rr * rr - r * r).powi(2) - 4.0 * rr * rr * rho2
}

/// An INDEPENDENT root counter: dense sampling of `f_geom` plus
/// bisection. Returns the sorted roots, and the smallest gap between
/// consecutive ones (a proxy for how near a tangency the pose is).
fn oracle_roots(
    q: Point3<f64>,
    d: Vec3<f64>,
    c: Point3<f64>,
    a: Vec3<f64>,
    rr: f64,
    r: f64,
) -> (Vec<f64>, f64) {
    let b = (q - c).dot(d);
    let ext = rr + r;
    let (lo, hi) = (-b - ext * 1.5, -b + ext * 1.5);
    let n = 400_000usize;
    let mut roots = Vec::new();
    let mut prev_t = lo;
    let mut prev = f_geom(q, d, c, a, rr, r, lo);
    for i in 1..=n {
        let t = lo + (hi - lo) * (i as f64) / (n as f64);
        let cur = f_geom(q, d, c, a, rr, r, t);
        if (prev < 0.0 && cur > 0.0) || (prev > 0.0 && cur < 0.0) {
            // bisect
            let (mut x0, mut x1) = (prev_t, t);
            let f0 = prev;
            for _ in 0..200 {
                let mid = 0.5 * (x0 + x1);
                let fm = f_geom(q, d, c, a, rr, r, mid);
                if (fm < 0.0) == (f0 < 0.0) {
                    x0 = mid;
                } else {
                    x1 = mid;
                }
            }
            roots.push(0.5 * (x0 + x1));
        } else if cur == 0.0 {
            roots.push(t);
        }
        prev_t = t;
        prev = cur;
    }
    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut gap = f64::INFINITY;
    for w in roots.windows(2) {
        gap = gap.min(w[1] - w[0]);
    }
    (roots, gap)
}

/// CLAIM 2 — the `sqrt`-chain cube root. Compared against `f64::cbrt`
/// over 12 decades either side of 1 and both signs.
#[test]
fn r1_cbrt_chain_tracks_the_true_cube_root() {
    let mut worst = 0.0f64;
    let mut worst_x = 0.0f64;
    for e in -30..=30 {
        for mant in [1.0, 1.7, 3.3, 6.1, 9.4] {
            for sgn in [1.0, -1.0] {
                let x = sgn * mant * 10f64.powi(e);
                let got = cbrt(x);
                let want = x.cbrt();
                let rel = ((got - want) / want).abs();
                if rel > worst {
                    worst = rel;
                    worst_x = x;
                }
            }
        }
    }
    println!("R1 cbrt: worst relative error {worst:e} at x = {worst_x:e}");
    assert_eq!(cbrt(0.0), 0.0, "cbrt(0) must be 0");
    assert!(
        worst < 1e-12,
        "the sqrt-chain cube root drifts from the true one by {worst:e} at {worst_x:e}"
    );
}

/// The truncation is a SYSTEMATIC offset, not an enclosure: the chain
/// computes `x^((1−4^-27)/3)`, and this measures how far that is from
/// `x^(1/3)` at the magnitudes the resolvent actually feeds it.
#[test]
fn r1_cbrt_truncation_is_a_bias_not_a_containment() {
    for x in [1e-18f64, 1e-6, 1.0, 1e6, 1e18, 1e36] {
        let exact = x.powf((1.0 - 4f64.powi(-27)) / 3.0);
        let truth = x.cbrt();
        println!(
            "R1 cbrt truncation: x={x:e} chain-exponent value={exact:e} true={truth:e} rel={:e}",
            ((exact - truth) / truth).abs()
        );
    }
}

/// The torus centre and axis every probe below sweeps about.
fn centre_and_axis() -> (Point3<f64>, Vec3<f64>) {
    (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))
}

/// The five `(R, r)` shapes the probes sweep — one per conditioning
/// regime of the quartic: generic; a fat ring; a thin ring; a large-`R`
/// thin ring; and a ring whose inner equator all but closes
/// (`R - r = 0.01`, so the near-tangency is built into the shape rather
/// than into the pose).
pub(super) const SHAPES: [(f64, f64); 5] =
    [(1.0, 0.3), (1.0, 0.9), (1.0, 0.02), (5.0, 0.1), (0.2, 0.19)];

/// The running census of one sweep: how the certifier answered, and
/// every disagreement with the geometric oracle.
///
/// `undecided` carries the LABEL of every ray the certifier escalated
/// or refused rather than a bare count, because the count alone cannot
/// say which pose class stopped deciding — and that is the question a
/// shrinking decided-ray floor asks.
pub(super) struct Tally {
    checked: usize,
    certified: usize,
    misses: usize,
    undecided: Vec<String>,
    pub(super) bad: Vec<String>,
}

impl Tally {
    pub(super) fn new() -> Self {
        Self {
            checked: 0,
            certified: 0,
            misses: 0,
            undecided: Vec::new(),
            bad: Vec::new(),
        }
    }

    /// Rays the certifier answered outright — a certified root set or a
    /// miss. Read together with `bad`, which holds every answer the
    /// oracle contradicted: `bad` empty and `decided` at its floor is
    /// the pair that says the rows tested something.
    fn decided(&self) -> usize {
        self.certified + self.misses
    }

    /// One pose against both counters. Every line `bad` collects opens
    /// with the REGIME that produced it, so a red names the pose class
    /// and not only its coordinates.
    pub(super) fn compare(
        &mut self,
        regime: &str,
        rr: f64,
        r: f64,
        o: Point3<f64>,
        dir: Vec3<f64>,
    ) {
        let (c, a) = centre_and_axis();
        let d = dir.normalize();
        self.checked += 1;
        let got = line_torus_roots(o, d, c, a, rr, r, band());
        let (oracle, gap) = oracle_roots(o, d, c, a, rr, r);
        match got {
            Ok(TorusRoots::Certified { count, ts }) => {
                self.certified += 1;
                if count != oracle.len() {
                    self.bad.push(format!(
                        "COUNT [{regime}] R={rr} r={r} o={o:?} d={d:?}: certified {count}, \
                         oracle {} (gap {gap:e}) oracle roots {oracle:?} code roots {:?}",
                        oracle.len(),
                        &ts[..count]
                    ));
                    return;
                }
                let mut mine: Vec<f64> = ts[..count].to_vec();
                mine.sort_by(|x, y| x.partial_cmp(y).unwrap());
                for (m, ok) in mine.iter().zip(oracle.iter()) {
                    if (m - ok).abs() > 1e-6 {
                        self.bad.push(format!(
                            "ROOT [{regime}] R={rr} r={r} o={o:?} d={d:?}: code {m:e} vs \
                             oracle {ok:e} (gap {gap:e})"
                        ));
                    }
                }
            }
            Ok(TorusRoots::Miss) => {
                self.misses += 1;
                if !oracle.is_empty() && gap > 1e-3 {
                    self.bad.push(format!(
                        "MISS [{regime}] R={rr} r={r} o={o:?} d={d:?}: code says miss, oracle \
                         found {oracle:?} (gap {gap:e})"
                    ));
                }
            }
            Ok(TorusRoots::Uncertain) | Err(_) => {
                self.undecided.push(format!("[{regime}] R={rr} r={r}"));
            }
        }
    }

    pub(super) fn report(&self, label: &str) {
        println!(
            "R1 root-count probe ({label}): {} rays, {} certified, {} miss, {} \
             uncertain/escalated, {} disagreements",
            self.checked,
            self.certified,
            self.misses,
            self.undecided.len(),
            self.bad.len()
        );
        for line in self.undecided.iter().take(25) {
            println!("  UNDECIDED {line}");
        }
        for line in self.bad.iter().take(25) {
            println!("  {line}");
        }
    }
}

/// The pose REGIMES of a `(R, r)` torus: the rays whose root structure
/// the geometry names, rather than rays drawn at large. Every entry is
/// a distinct configuration of the quartic — a ray through the hole, a
/// ray tangent to one of the two equators, a ray down the tube's
/// centre, a ray that clears the shape — in each of the two symmetry
/// planes plus two obliques through the centre.
fn regime_poses(rr: f64, r: f64) -> Vec<(&'static str, Point3<f64>, Vec3<f64>)> {
    vec![
        // Through the centre: the four-root rays.
        (
            "centre, midplane, axis-aligned",
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "centre, midplane, oblique",
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.3, 0.0, 0.954),
        ),
        (
            "centre, out of the midplane",
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.3, 0.5, 0.812),
        ),
        // Parallel to the axis: the offset walks the hole, the inner
        // equator, the tube's centre, the outer equator, and clear.
        (
            "axis-parallel, through the hole",
            Point3::new(0.0, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            "axis-parallel, inner-equator tangent",
            Point3::new(rr - r, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            "axis-parallel, through the tube centre",
            Point3::new(rr, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            "axis-parallel, outer-equator tangent",
            Point3::new(rr + r, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            "axis-parallel, clear of the shape",
            Point3::new(rr + r + 0.4, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        // In the midplane, offset from the centre: the same walk with
        // the ray in the plane of the ring instead of along its axis.
        (
            "midplane, through the hole",
            Point3::new(-9.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "midplane, inner-equator tangent",
            Point3::new(-9.0, 0.0, rr - r),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "midplane, through the tube centre",
            Point3::new(-9.0, 0.0, rr),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "midplane, outer-equator tangent",
            Point3::new(-9.0, 0.0, rr + r),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "midplane, beyond the outer equator",
            Point3::new(-9.0, 0.0, 2.0 * rr),
            Vec3::new(1.0, 0.0, 0.0),
        ),
    ]
}

/// How many of the enumeration's 65 rays the certifier must answer
/// outright. The table is static, so this is a WITNESS, not a budget,
/// and it exists because agreement with the oracle is vacuous over rays
/// that never decided: a certifier that escalated on everything would
/// satisfy the disagreement assertion perfectly.
///
/// The rays that legitimately do not decide are the ones carrying a
/// double root inside the band — the two equator tangents in each of
/// the two symmetry planes, and the axis-parallel ray up the hole,
/// which grazes the inner equator — where escalating is the right
/// answer. Which of them escalate MOVES WITH ε, and so does the count:
/// 40 rays decided at the default ε, 41 at 1e-12, 38 at 1e-6. The
/// membership moves too, not just the total — at 1e-6 the two midplane
/// tangents on the `(0.2, 0.19)` torus decide while the tube-centre ray
/// through the two thinnest tubes does not — so this is a FLOOR over
/// the three ε rows and not a per-regime pin. Lower it only with the
/// regime that stopped deciding named, and only once escalating there
/// is the right answer for it.
const DECIDED_FLOOR: usize = 38;

/// CLAIM 1 + 5 — the certified count and the biquadratic sign, against
/// the geometric oracle, over the ENUMERATION of pose regimes: for each
/// of the five torus shapes, every ray configuration the geometry names
/// (`regime_poses`). Deterministic and exhaustive over that table;
/// generic poses are the sibling sweep's, not this row's.
///
/// Two assertions, labelled: **AGREEMENT**, that no answer contradicts
/// the oracle, and **DECIDEDNESS**, that the table still drives the
/// certifier to an answer on at least [`DECIDED_FLOOR`] of its rays.
/// Neither carries the claim alone.
#[test]
fn r1_certified_counts_agree_with_a_geometric_oracle() {
    let mut tally = Tally::new();
    for (rr, r) in SHAPES {
        for (regime, o, dir) in regime_poses(rr, r) {
            tally.compare(regime, rr, r, o, dir);
        }
    }
    tally.report("pose regimes");
    assert!(
        tally.bad.is_empty(),
        "AGREEMENT: {} disagreements with the oracle over the pose-regime enumeration",
        tally.bad.len()
    );
    assert!(
        tally.decided() >= DECIDED_FLOOR,
        "DECIDEDNESS: only {} of {} enumerated rays were decided (floor {DECIDED_FLOOR}); \
         the regimes that escalated or refused are {:?}",
        tally.decided(),
        tally.checked,
        tally.undecided
    );
}

/// The four-root ray through the hole, asserted at the ROOT level
/// rather than through `point_in_solid` — the shipped suite pins the
/// consequence, not the count.
#[test]
fn r1_the_hole_ray_certifies_exactly_four_roots() {
    let (rr, r) = (1.0f64, 0.3f64);
    let c = Point3::new(0.0, 0.0, 0.0);
    let a = Vec3::new(0.0, 1.0, 0.0);
    let d = Vec3::new(1.0, 0.0, 0.0);
    match line_torus_roots(Point3::new(0.0, 0.0, 0.0), d, c, a, rr, r, band()) {
        Ok(TorusRoots::Certified { count, ts }) => {
            let mut v = ts[..count].to_vec();
            v.sort_by(|x, y| x.partial_cmp(y).unwrap());
            println!("R1 hole ray: count={count} ts={v:?}");
            assert_eq!(count, 4);
            for (got, want) in v.iter().zip([-(rr + r), -(rr - r), rr - r, rr + r]) {
                assert!((got - want).abs() < 1e-9, "{got} != {want}");
            }
        }
        other => panic!("the four-root ray through the hole did not certify: {other:?}"),
    }
}

/// The pose the generic-pose sweep found (seed `0x2ce3095461764e3a`, at
/// ε = 1e-12): a ray all but perpendicular to the axis, whose odd
/// coefficient `q̂ ≈ −1.2e-4` is decided nonzero, so the resolvent's one
/// real root is `z ≈ q̂²/c1 ≈ 1.3e-9` — the root Cardano's form would
/// assemble as a difference of order-one terms.
///
/// The truth is the pose's own quartic solved in exact rational
/// arithmetic on these f64 inputs: its Sturm count is 2 and bisection
/// brackets each root to 1e-45. The row holds both scalars to it at the
/// band the sweep drew — a FIXED band, so every ε leg asserts the same
/// pose, which the run's band would escalate at 1e-9 and 1e-6 — the f64
/// roots to within 1e-12, and the `Interval` roots to an enclosure that
/// contains the truth and is no wider than 1e-9.
#[test]
fn r1_the_near_perpendicular_ray_keeps_its_roots() {
    use geom_core::{Bounds, Interval, Real};
    const TRUTH: [f64; 2] = [-0.932_255_799_041_234, -0.320_660_420_293_933_6];
    let o = [0.6165109851873778, -0.4322368608327216, -1.7665919240171966];
    let d = [
        -0.7793351131793784,
        3.340316168992811e-5,
        -0.6266073573218832,
    ];
    let pinned = Band::new(1e-12, 1e-11).unwrap();
    let (c, a) = centre_and_axis();
    let f64_roots = match line_torus_roots(
        Point3::new(o[0], o[1], o[2]),
        Vec3::new(d[0], d[1], d[2]),
        c,
        a,
        1.0,
        0.9,
        pinned,
    ) {
        Ok(TorusRoots::Certified { count, ts }) => {
            assert_eq!(count, 2, "F64 COUNT: the exact Sturm count is 2");
            let mut v = ts[..count].to_vec();
            v.sort_by(|x, y| x.partial_cmp(y).unwrap());
            v
        }
        other => panic!("F64: the near-perpendicular ray did not certify: {other:?}"),
    };
    for (got, want) in f64_roots.iter().zip(TRUTH) {
        assert!(
            (got - want).abs() < 1e-12,
            "F64 ROOT: {got:e} against the exact {want:e}, off by {:e}",
            (got - want).abs()
        );
    }
    let iv = Interval::from_f64;
    match line_torus_roots(
        Point3::new(iv(o[0]), iv(o[1]), iv(o[2])),
        Vec3::new(iv(d[0]), iv(d[1]), iv(d[2])),
        Point3::new(iv(0.0), iv(0.0), iv(0.0)),
        Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
        iv(1.0),
        iv(0.9),
        pinned,
    ) {
        Ok(TorusRoots::Certified { count, ts }) => {
            assert_eq!(count, 2, "INTERVAL COUNT: the exact Sturm count is 2");
            let mut v = ts[..count].to_vec();
            v.sort_by(|x, y| x.lo().partial_cmp(&y.lo()).unwrap());
            for (got, want) in v.iter().zip(TRUTH) {
                assert!(
                    got.lo() <= want && want <= got.hi(),
                    "INTERVAL CONTAINMENT: [{:e}, {:e}] excludes the exact root {want:e}",
                    got.lo(),
                    got.hi()
                );
                assert!(
                    got.hi() - got.lo() < 1e-9,
                    "INTERVAL WIDTH: the enclosure of {want:e} is {:e} wide",
                    got.hi() - got.lo()
                );
            }
        }
        other => panic!("INTERVAL: the near-perpendicular ray did not certify: {other:?}"),
    }
}

/// Review MINOR-1 on PR 3255, as the reviewer wrote it: a generic ray
/// whose resolvent's depressed constant `Q` is a few ulps from zero, so
/// its `Interval` enclosure straddles zero. Main certified it; a Cardano
/// radicand chosen by `copysign(…, Q)` turned the straddle into a
/// whole-line root and escalated `Invalid`. The truth is the pose's
/// exact root pair; the ±1e-12 is `d`'s own 5.9e-16 departure from unit.
#[test]
fn r1_a_ray_on_the_resolvents_q_zero_surface_still_certifies_at_interval() {
    use geom_core::{Bounds, Interval, Real};
    const ROOTS: [f64; 2] = [2.198_813_713_696_078, 4.602_155_907_842_552];
    let (o, d) = (
        [-3.747999552668488, -0.6565569277642699, -0.832630206695418],
        [
            0.9412606460021874,
            0.3369297354810353,
            -0.022511100289074278,
        ],
    );
    let iv = Interval::from_f64;
    match line_torus_roots(
        Point3::new(iv(o[0]), iv(o[1]), iv(o[2])),
        Vec3::new(iv(d[0]), iv(d[1]), iv(d[2])),
        Point3::new(iv(0.0), iv(0.0), iv(0.0)),
        Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
        iv(1.0),
        iv(0.9),
        Band::new(1e-12, 1e-11).unwrap(),
    ) {
        Ok(TorusRoots::Certified { count, ts }) => {
            assert_eq!(count, 2);
            let mut v = ts[..count].to_vec();
            v.sort_by(|x, y| x.lo().partial_cmp(&y.lo()).unwrap());
            for (got, want) in v.iter().zip(ROOTS) {
                assert!(
                    got.lo() - 1e-12 <= want
                        && want <= got.hi() + 1e-12
                        && got.hi() - got.lo() < 1e-9,
                    "[{:e}, {:e}] vs {want:e}",
                    got.lo(),
                    got.hi()
                );
            }
        }
        other => panic!("Q straddles zero here and the ray must still certify: {other:?}"),
    }
}

/// The resolvent's depressed constant `Q`, spelled as
/// `cubic_largest_real_root` spells it from `line_torus_roots`'
/// coefficients — here only to check the walk below crosses `Q = 0`,
/// which is that row's premise rather than its claim.
fn resolvent_q<T: geom_core::Real>(o: [T; 3], d: [T; 3], rr: T, r: T) -> T {
    let dot = |x: [T; 3], y: [T; 3]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    let (two, three, four) = (T::from_f64(2.0), T::from_f64(3.0), T::from_f64(4.0));
    let b = dot(o, d);
    let perp = [o[0] - d[0] * b, o[1] - d[1] * b, o[2] - d[2] * b];
    let (m, n, e) = (dot(perp, perp), perp[1], d[1]);
    let big_m = m + rr.powi(2) - r.powi(2);
    let p = two * big_m - four * rr.powi(2) * (T::one() - e.powi(2));
    let q_hat = T::from_f64(8.0) * rr.powi(2) * e * n;
    let s = big_m.powi(2) - four * rr.powi(2) * (m - n.powi(2));
    let (c2, c1, c0) = (two * p, p.powi(2) - four * s, T::zero() - q_hat.powi(2));
    c2.powi(3) * two / T::from_f64(27.0) - c2 * c1 / three + c0
}

/// `Q = 0` is a codimension-one surface of GENERIC rays, so one pose on
/// it is weak evidence. Five unrelated rays found on it (by bisecting
/// `Q` along the origin's axial coordinate, each with a definite
/// discriminant and an order-one `q̂`) are each walked across it by
/// offsets from 1e-8 down to 1e-15 on either side. Every ray of every
/// walk must certify at BOTH scalars, agree in count with the geometric
/// oracle, and — at `Interval` — enclose the oracle's roots in less
/// than 1e-9. The walk asserts its own premise: `Q` changes sign along
/// it at `f64`, and at least one of its `Interval` enclosures of `Q`
/// straddles zero.
#[test]
fn r1_the_q_zero_surface_certifies_on_both_sides() {
    use geom_core::{Bounds, Interval, Real};
    const BASES: [([f64; 3], [f64; 3]); 5] = [
        (
            [-1.0570034110010258, -1.2591002478973186, 0.9056068382391222],
            [0.8831526396789883, 0.43221370376568374, -0.1822984621580361],
        ),
        (
            [-0.6199171520953191, 2.16351195309624, -2.7205039162934623],
            [-0.45567824377849847, 0.5607732491905882, -0.691296391672323],
        ),
        (
            [1.1939666023774276, -1.6847529160703743, 0.44654226155202625],
            [0.638505070624459, -0.7599931189159066, -0.12133315287804632],
        ),
        (
            [0.6751673060667622, -0.2987541742458132, 0.07296883461191639],
            [
                -0.24693981883060998,
                -0.6563869361596056,
                -0.7128652859516386,
            ],
        ),
        (
            [-0.46647491233403304, 1.5667199101032885, 1.9138738786876903],
            [
                -0.02420391296253917,
                0.44702148428280625,
                0.8941957074303694,
            ],
        ),
    ];
    const STEPS: [f64; 11] = [
        -1e-8, -1e-11, -1e-13, -1e-14, -1e-15, 0.0, 1e-15, 1e-14, 1e-13, 1e-11, 1e-8,
    ];
    let pinned = Band::new(1e-12, 1e-11).unwrap();
    let (c, a) = centre_and_axis();
    let iv = Interval::from_f64;
    for (k, (base, d)) in BASES.into_iter().enumerate() {
        let (mut signs, mut straddles) = (Vec::new(), 0usize);
        for h in STEPS {
            let o = [base[0], base[1] + h, base[2]];
            signs.push(resolvent_q(o, d, 1.0, 0.9).signum());
            let q_iv = resolvent_q(o.map(iv), d.map(iv), iv(1.0), iv(0.9));
            if q_iv.lo() <= 0.0 && 0.0 <= q_iv.hi() {
                straddles += 1;
            }
            let (truth, _) = oracle_roots(
                Point3::new(o[0], o[1], o[2]),
                Vec3::new(d[0], d[1], d[2]),
                c,
                a,
                1.0,
                0.9,
            );
            assert_eq!(
                truth.len(),
                2,
                "PREMISE: base {k} step {h:e} is a two-root ray"
            );
            let at = format!("base {k}, step {h:e}");
            match line_torus_roots(
                Point3::new(o[0], o[1], o[2]),
                Vec3::new(d[0], d[1], d[2]),
                c,
                a,
                1.0,
                0.9,
                pinned,
            ) {
                Ok(TorusRoots::Certified { count, ts }) => {
                    assert_eq!(count, 2, "F64 COUNT at {at}");
                    let mut v = ts[..count].to_vec();
                    v.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    for (got, want) in v.iter().zip(&truth) {
                        assert!(
                            (got - want).abs() < 1e-11,
                            "F64 ROOT at {at}: {got:e} vs {want:e}"
                        );
                    }
                }
                other => panic!("F64 at {at} did not certify: {other:?}"),
            }
            match line_torus_roots(
                Point3::new(iv(o[0]), iv(o[1]), iv(o[2])),
                Vec3::new(iv(d[0]), iv(d[1]), iv(d[2])),
                Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
                iv(1.0),
                iv(0.9),
                pinned,
            ) {
                Ok(TorusRoots::Certified { count, ts }) => {
                    assert_eq!(count, 2, "INTERVAL COUNT at {at}");
                    let mut v = ts[..count].to_vec();
                    v.sort_by(|x, y| x.lo().partial_cmp(&y.lo()).unwrap());
                    for (got, want) in v.iter().zip(&truth) {
                        assert!(
                            got.lo() - 1e-11 <= *want
                                && *want <= got.hi() + 1e-11
                                && got.hi() - got.lo() < 1e-9,
                            "INTERVAL at {at}: [{:e}, {:e}] vs {want:e}",
                            got.lo(),
                            got.hi()
                        );
                    }
                }
                other => panic!("INTERVAL at {at} did not certify: {other:?}"),
            }
        }
        assert!(
            signs.first() != signs.last(),
            "PREMISE: base {k}'s walk does not cross Q = 0 ({signs:?})"
        );
        assert!(
            straddles > 0,
            "PREMISE: no Interval Q straddles zero on base {k}'s walk"
        );
    }
}
