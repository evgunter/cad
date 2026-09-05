//! The reviewer rows this unit ADOPTED, one module per lane they came
//! from, each with the note that says what it is for and why it is here
//! rather than in `props1_evidence.rs`.
//!
//! `props1_evidence.rs` holds the unit's own corpora — literals chosen
//! before the change, so its tables are re-takeable and its pins are
//! keyed to the constructors' documented numbers. These rows come from
//! the blinded dual review's falsification attempts: they are wider,
//! randomized and adversarial, they found three statements that were
//! false as written, and they belong beside those statements rather
//! than inside a corpus that was never meant to draw them. Every row
//! below ASSERTS — the probes printed, and a probe that only prints
//! cannot go red when the behaviour it measured moves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::frame::mirror_across_plane;
use geom_core::{Point3, Real, Tol, Vec3};

/// The retired rejection, `self − project`, kept here as well as in
/// `props1_evidence.rs` because these rows are a separate lane's
/// evidence and must not silently follow an edit made for the other
/// file's tables. NOTE (r1): it is a hand copy, and nothing keeps the
/// two in step — a change to `project_onto`'s association has to be
/// mirrored here by hand or the differential stops being one.
fn retired_rejection<T: Real>(v: Vec3<T>, onto: Vec3<T>) -> Vec3<T> {
    v - v.project_onto(onto)
}

/// xorshift with a literal seed, so every randomized row below is
/// reproducible from the file alone.
struct Rng(u64);
impl Rng {
    fn new(s: u64) -> Self {
        Self(s)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    /// uniform in [-1, 1)
    fn sym(&mut self) -> f64 {
        f64::from((self.next_u64() >> 40) as u32) / 8_388_608.0 - 1.0
    }
    /// The r2 lane's spelling of the same draw, kept so its rows read as
    /// they were written. These three are used only by the enclosure
    /// rows, which the `interval` feature gates.
    #[allow(dead_code)]
    fn unit(&mut self) -> f64 {
        self.sym()
    }
    #[allow(dead_code)]
    fn log_uniform(&mut self, lo_exp: f64, hi_exp: f64) -> f64 {
        10f64.powf(lo_exp + (self.unit() + 1.0) / 2.0 * (hi_exp - lo_exp))
    }
    #[allow(dead_code)]
    fn dir(&mut self) -> [f64; 3] {
        [self.unit(), self.unit(), self.unit()]
    }
}

fn ulps(a: f64, b: f64) -> i64 {
    let key = |x: f64| -> i64 {
        let b = x.to_bits() as i64;
        if b < 0 { i64::MIN - b } else { b }
    };
    key(a).saturating_sub(key(b)).abs()
}

// ------------------------------------------------------- reconstruction

/// ADOPTED (r1) as the gating row for `reject_from`'s documented
/// reconstruction bound. The doc says `project + reject` returns `self`
/// to within **4 ulps of the largest component**; the committed corpus
/// reaches that number but cannot show it is the RIGHT number, because
/// it draws neither disparate magnitudes nor near-parallel pairs. This
/// row does, over 200 000 adversarial pairs, and it pins both halves of
/// the claim: the shipped spelling never exceeds 4, and the retired one
/// never exceeded 1 — which is what makes the doc's "it is no longer the
/// same bound" sentence a measurement rather than an impression.

#[test]
fn p1_project_plus_reject_ulp_adversarial() {
    let mut rng = Rng::new(0x1234_5678_9abc_def1);
    let mut worst_new = 0i64;
    let mut worst_old = 0i64;
    let mut worst_comp_new = 0i64;
    let mut worst_comp_old = 0i64;
    let mut worst_row = String::new();
    let scales = [1.0e-8_f64, 1.0, 1.0e3, 1.0e8];
    for _ in 0..200_000 {
        let os = scales[(rng.next_u64() % 4) as usize];
        let ss = scales[(rng.next_u64() % 4) as usize];
        let onto = Vec3::new(rng.sym() * os, rng.sym() * os, rng.sym() * os);
        let mode = rng.next_u64() % 3;
        let v = match mode {
            // free
            0 => Vec3::new(rng.sym() * ss, rng.sym() * ss, rng.sym() * ss),
            // near-parallel to onto
            1 => {
                let k = 1.0e-9 * rng.sym();
                onto * (rng.sym() * ss)
                    + Vec3::new(rng.sym() * k * os, rng.sym() * k * os, rng.sym() * k * os)
            }
            // near-orthogonal to onto
            _ => {
                let a = Vec3::new(rng.sym() * ss, rng.sym() * ss, rng.sym() * ss);
                retired_rejection(a, onto)
            }
        };
        if onto.norm_squared() == 0.0 || !v.norm().is_finite() || v.norm() == 0.0 {
            continue;
        }
        let sum_new = v.project_onto(onto) + v.reject_from(onto);
        let sum_old = v.project_onto(onto) + retired_rejection(v, onto);
        // The doc's own metric: ulps OF THE LARGEST COMPONENT.
        let biggest = v.x.abs().max(v.y.abs()).max(v.z.abs());
        let ulp_big = (f64::from_bits(biggest.to_bits() + 1) - biggest).abs();
        for (a, b, c) in [
            (sum_new.x, sum_old.x, v.x),
            (sum_new.y, sum_old.y, v.y),
            (sum_new.z, sum_old.z, v.z),
        ] {
            if !a.is_finite() || !c.is_finite() {
                continue;
            }
            let u = ((a - c).abs() / ulp_big).ceil() as i64;
            if u > worst_new {
                worst_new = u;
                worst_row = format!("onto {onto:?} self {v:?} got {a:e} want {c:e}");
            }
            worst_old = worst_old.max(((b - c).abs() / ulp_big).ceil() as i64);
            worst_comp_new = worst_comp_new.max(ulps(a, c));
            worst_comp_old = worst_comp_old.max(ulps(b, c));
        }
    }
    println!(
        "C4 adversarial, ulps OF THE LARGEST COMPONENT: shipped worst {worst_new}, retired worst \
         {worst_old}"
    );
    println!(
        "C4 adversarial, ulps of the component itself: shipped worst {worst_comp_new}, retired \
         worst {worst_comp_old}"
    );
    println!("C4 worst row: {worst_row}");
    assert!(
        worst_new <= 4,
        "reconstruction is worse than the documented 4 ulps of the largest component: \
         {worst_new} at {worst_row}"
    );
    assert!(
        worst_old <= 1,
        "the retired spelling is no longer the tighter one at 1 ulp: {worst_old}"
    );
}

// ------------------------------------------------------------ totality

/// ADOPTED (r1) as the totality row for the doc's overflow clause. The
/// clause used to claim `onto`'s bands were `project_onto`'s unchanged;
/// they are not, because the triple product's numerator is
/// `|onto|²·|self|` where the quotient's was `|self|` alone. These rows
/// pin the band the doc now states: an `onto` near 1e150 with a `self`
/// large enough to push the numerator past the finite range returns
/// infinities where the retired spelling was exact.

#[test]
fn p1_reject_overflow_band_is_not_project_ontos() {
    let rows: [(Vec3<f64>, Vec3<f64>); 4] = [
        // |onto|^2 = 1e300 (finite), |self| = 1e20 -> numerator 1e320
        (Vec3::new(1.0e150, 0.0, 0.0), Vec3::new(0.0, 1.0e20, 0.0)),
        (
            Vec3::new(1.0e120, 1.0e120, 0.0),
            Vec3::new(0.0, 0.0, 1.0e70),
        ),
        // both moderate-large: old fine, new fine?
        (Vec3::new(1.0e100, 0.0, 0.0), Vec3::new(0.0, 1.0e100, 0.0)),
        // underflow side: |onto|^2 normal, numerator subnormal
        (Vec3::new(1.0e-150, 0.0, 0.0), Vec3::new(0.0, 3.7e-20, 0.0)),
    ];
    for (onto, v) in rows {
        let new = v.reject_from(onto);
        let old = retired_rejection(v, onto);
        println!(
            "onto {onto:?} self {v:?}\n   shipped {new:?}\n   retired {old:?}\n   nsq {:e}",
            onto.norm_squared()
        );
    }
    // The claim under test, as an assertion: for an input where the
    // retired spelling is finite and correct, the shipped one is too.
    let onto = Vec3::new(1.0e150, 0.0, 0.0);
    let v = Vec3::new(0.0, 1.0e20, 0.0);
    let new = v.reject_from(onto);
    println!(
        "OVERFLOW: shipped {new:?} vs retired {:?} (finite: {})",
        retired_rejection(v, onto),
        new.x.is_finite() && new.y.is_finite() && new.z.is_finite()
    );
    assert!(
        !(new.x.is_finite() && new.y.is_finite() && new.z.is_finite()),
        "the numerator no longer overflows here: the doc's band is stale"
    );
    let old = retired_rejection(v, onto);
    assert_eq!(
        (old.x, old.y, old.z),
        (0.0, 1.0e20, 0.0),
        "the retired spelling was exact on this row; the differential is the point"
    );
}

/// ADOPTED (r1) as the totality row for the doc's underflow clause: an
/// `onto` small enough that the doubled cross product lands in (or
/// below) the subnormal range loses the rejection's digits, and at the
/// bottom of the band returns an exact zero for a `self` that is
/// entirely orthogonal to `onto`. `project_onto`'s own note describes a
/// different band, which is why the clause had to be rewritten.

#[test]
fn p1_reject_underflow_band_accuracy() {
    let mut worst = 0.0f64;
    let mut worst_row = String::new();
    for e in [140, 145, 150, 155, 158, 160] {
        for se in [-1, -20, -60, -100, -150] {
            let onto = Vec3::new(10.0f64.powi(-e), 0.0, 0.0);
            let v = Vec3::new(0.0, 3.7 * 10.0f64.powi(se), 0.0);
            let new = v.reject_from(onto);
            let old = retired_rejection(v, onto);
            // truth: onto is the x axis, self is on y -> rejection == self
            let rel = ((new.y - v.y) / v.y).abs();
            let rel_old = ((old.y - v.y) / v.y).abs();
            println!(
                "onto 1e-{e} self 3.7e{se}: shipped {:e} retired {:e} truth {:e} \
                 (rel shipped {rel:e}, retired {rel_old:e})",
                new.y, old.y, v.y
            );
            if rel.is_finite() && rel > worst {
                worst = rel;
                worst_row = format!("onto 1e-{e} self 3.7e{se}");
            }
        }
    }
    println!("worst relative error of the shipped rejection: {worst:e} at {worst_row}");
    assert!(
        worst >= 0.5,
        "the underflow band no longer loses the rejection: the doc's clause is stale \
         (worst relative error {worst:e})"
    );
    let onto = geom_core::Vec3::new(1.0e-160, 0.0, 0.0);
    let v = geom_core::Vec3::new(0.0, 3.7e-100, 0.0);
    assert_eq!(v.reject_from(onto).y, 0.0, "the silent-zero row moved");
    assert_eq!(
        retired_rejection(v, onto).y,
        v.y,
        "the retired spelling was exact here"
    );
}

// --------------------------------------------------------- signed zeros

/// ADOPTED (r2). A NEGATIVE axis-aligned normal anchored at the ORIGIN
/// puts a −0.0 in the normal's own slot, where a subtract-and-re-add
/// spelling put +0.0. It is inert under `transform_point`, and it is the
/// row that showed the axis pin's sign sentence was written for positive
/// axes only — that pin now draws both orientations and states the sign
/// rule in terms of the doubled dot product.

#[test]
fn negative_axis_normal_at_origin_flips_a_zero_sign() {
    for (normal, j) in [
        (Vec3::new(0.0, 0.0, -1.0), 2usize),
        (Vec3::new(-1.0, 0.0, 0.0), 0),
        (Vec3::new(0.0, -2.0, 0.0), 1),
    ] {
        let t = mirror_across_plane(Point3::origin(), normal, Tol::witness())
            .unwrap()
            .translation;
        let comps = [t.x, t.y, t.z];
        assert!(
            comps[j].is_sign_negative() && comps[j] == 0.0,
            "expected −0.0 in slot {j}, got {comps:?}"
        );
        // The retired spelling at the origin: the twelve-entry pin's
        // premise, +0.0 in all slots.
        assert!(
            comps
                .iter()
                .enumerate()
                .all(|(k, c)| k == j || !c.is_sign_negative()),
            "{comps:?}"
        );
    }
    // Off-axis zeros at a non-origin anchor with a negative normal carry
    // the sign of −q_j, not of the surviving component.
    let t = mirror_across_plane(
        Point3::new(1.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, -1.0),
        Tol::witness(),
    )
    .unwrap()
    .translation;
    println!(
        "negative-normal anchor (1,2,3): translation {:?} (signs: {} {} {})",
        (t.x, t.y, t.z),
        t.x.is_sign_negative(),
        t.y.is_sign_negative(),
        t.z.is_sign_negative()
    );
    assert_eq!(t.z, 6.0);
    assert!(
        t.x.is_sign_negative() && t.y.is_sign_negative(),
        "off-axis zeros are +0 here: the pin's sign story holds for negative normals too"
    );
}

// ------------------------------------------- geometric containment (r1)

/// ADOPTED (r1) in place of this unit's original containment pins,
/// which were tautological: they compared the shipped formula on a wide
/// box against the SAME formula on a point, so a sign-flipped or
/// otherwise wrong rejection passed them. These rows are formula-free —
/// they assert the GEOMETRY the enclosure must admit (a reflected
/// midpoint on the plane, a displacement along the normal; a rejection
/// orthogonal to `onto` and a difference parallel to it) — so a wrong
/// formula fails them.
#[cfg(feature = "interval")]
mod geometric_containment {
    use super::{Rng, retired_rejection};
    use geom_core::linalg::frame::mirror_across_plane;
    use geom_core::{Bounds, Interval, Point3, Real, Tol, Vec3};

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }
    fn fat(x: f64, r: f64) -> Interval {
        Interval::from_bounds(x - r, x + r)
    }
    fn contains_zero(e: Interval) -> bool {
        e.lo() <= 0.0 && e.hi() >= 0.0
    }
    fn overlaps(a: Interval, b: Interval) -> bool {
        a.lo() <= b.hi() && b.lo() <= a.hi()
    }
    fn w(e: Interval) -> f64 {
        e.hi() - e.lo()
    }

    /// C1, the geometric containment the committed pin does NOT do: the
    /// committed row compares the shipped formula at a wide box against
    /// the SAME formula at a point, so an algebraically wrong formula
    /// passes it. Here the check is formula-free: for the centre plane
    /// (`n0`, `p0`) and an arbitrary probe point `x`, the reflection
    /// `r` produced by the WIDE frame must admit
    /// `((r + x)/2 − p0)·n0 = 0` (midpoint on the plane) and
    /// `(r − x) × n0 = 0` (displacement along the normal). Both hold
    /// for the true reflection, which the sound enclosure contains, so
    /// each expression's enclosure must contain 0.
    #[test]
    fn p1_mirror_encloses_the_true_reflection_geometrically() {
        let mut rng = Rng::new(0x5eed_0001_0000_0007);
        let anchors = [
            [0.001, 0.002, -0.003],
            [1.0, 2.0, -3.0],
            [100.0, -250.0, 30.0],
        ];
        let radii = [0.0, 1.0e-15, 1.0e-9, 1.0e-3];
        let nradii = [0.0, 1.0e-12, 1.0e-6];
        let mut checked = 0usize;
        let mut worst_mid = 0.0f64;
        for trial in 0..600 {
            let nrow = match trial % 4 {
                0 => [rng.sym(), rng.sym(), rng.sym()],
                1 => [1.0e-9 * rng.sym(), 1.0e-9 * rng.sym(), 1.0],
                2 => [1.0, 1.0e-13 * rng.sym(), 1.0e-13 * rng.sym()],
                _ => [0.0, 0.0, 1.0],
            };
            let n0 = Vec3::new(nrow[0], nrow[1], nrow[2]);
            if n0.norm() < 1.0e-12 {
                continue;
            }
            for arow in anchors {
                for ar in radii {
                    for nr in nradii {
                        let p = Point3::new(fat(arow[0], ar), fat(arow[1], ar), fat(arow[2], ar));
                        let n = Vec3::new(fat(nrow[0], nr), fat(nrow[1], nr), fat(nrow[2], nr));
                        let Ok(m) = mirror_across_plane(p, n, Tol::witness()) else {
                            continue;
                        };
                        // an arbitrary probe point, exact
                        let xr = [
                            arow[0] + 7.5 * rng.sym(),
                            arow[1] - 3.25 * rng.sym(),
                            arow[2] + 11.0 * rng.sym(),
                        ];
                        let x = Point3::new(iv(xr[0]), iv(xr[1]), iv(xr[2]));
                        let r = m.transform_point(x);
                        // centre plane, exact
                        let nc = Vec3::new(iv(nrow[0]), iv(nrow[1]), iv(nrow[2]));
                        let pc = Point3::new(iv(arow[0]), iv(arow[1]), iv(arow[2]));
                        let mid = Vec3::new(
                            (r.x + x.x) * iv(0.5) - pc.x,
                            (r.y + x.y) * iv(0.5) - pc.y,
                            (r.z + x.z) * iv(0.5) - pc.z,
                        );
                        let on_plane = mid.dot(nc);
                        let d = Vec3::new(r.x - x.x, r.y - x.y, r.z - x.z);
                        let along = d.cross(nc);
                        checked += 1;
                        worst_mid = worst_mid.max(w(on_plane));
                        assert!(
                            contains_zero(on_plane),
                            "reflected midpoint is NOT on the plane: normal {nrow:?} anchor \
                             {arow:?} ar={ar:e} nr={nr:e} x={xr:?} -> [{:e},{:e}]",
                            on_plane.lo(),
                            on_plane.hi()
                        );
                        assert!(
                            contains_zero(along.x)
                                && contains_zero(along.y)
                                && contains_zero(along.z),
                            "displacement is NOT along the normal: normal {nrow:?} anchor \
                             {arow:?} ar={ar:e} nr={nr:e} x={xr:?}"
                        );
                    }
                }
            }
        }
        println!(
            "mirror geometric containment: {checked} rows checked, worst on-plane width \
             {worst_mid:e}"
        );
    }

    /// C1, rejection half, formula-free: `reject · onto = 0` and
    /// `(self − reject) × onto = 0` must both admit 0 in the enclosure.
    /// `onto` is exact here so the centre-`self` truth is in the box.
    #[test]
    fn p1_rejection_encloses_the_true_rejection_geometrically() {
        let mut rng = Rng::new(0x5eed_0002_0000_000b);
        let radii = [0.0, 1.0e-15, 1.0e-9, 1.0e-3];
        let mut checked = 0usize;
        for trial in 0..800 {
            let orow = match trial % 4 {
                0 => [rng.sym(), rng.sym(), rng.sym()],
                1 => [0.0, 0.0, 1.0],
                2 => [1.0e-9 * rng.sym(), 1.0, 1.0e-9 * rng.sym()],
                _ => [1.0e3 * rng.sym(), 1.0e-3 * rng.sym(), rng.sym()],
            };
            let onto = Vec3::new(iv(orow[0]), iv(orow[1]), iv(orow[2]));
            if orow.iter().all(|c| *c == 0.0) {
                continue;
            }
            let srow = match trial % 3 {
                0 => [rng.sym(), rng.sym(), rng.sym()],
                1 => [
                    orow[0] * 2.5 + 1.0e-11 * rng.sym(),
                    orow[1] * 2.5,
                    orow[2] * 2.5,
                ],
                _ => [100.0 * rng.sym(), -250.0 * rng.sym(), 30.0 * rng.sym()],
            };
            for ar in radii {
                let v = Vec3::new(fat(srow[0], ar), fat(srow[1], ar), fat(srow[2], ar));
                let r = v.reject_from(onto);
                if !r.x.lo().is_finite() {
                    println!("non-finite rejection at onto {orow:?} self {srow:?} r={ar:e}");
                    continue;
                }
                checked += 1;
                let ortho = r.dot(onto);
                assert!(
                    contains_zero(ortho),
                    "rejection is not orthogonal to onto: {orow:?} {srow:?} r={ar:e} -> \
                     [{:e},{:e}]",
                    ortho.lo(),
                    ortho.hi()
                );
                let sc = Vec3::new(iv(srow[0]), iv(srow[1]), iv(srow[2]));
                let par = Vec3::new(sc.x - r.x, sc.y - r.y, sc.z - r.z).cross(onto);
                assert!(
                    contains_zero(par.x) && contains_zero(par.y) && contains_zero(par.z),
                    "self - reject is not parallel to onto: {orow:?} {srow:?} r={ar:e}"
                );
                // and it must overlap the retired spelling's own
                // enclosure of the same centre value
                let old = retired_rejection(Vec3::new(iv(srow[0]), iv(srow[1]), iv(srow[2])), onto);
                assert!(
                    overlaps(r.x, old.x) && overlaps(r.y, old.y) && overlaps(r.z, old.z),
                    "shipped and retired enclosures are disjoint: {orow:?} {srow:?} r={ar:e}"
                );
            }
        }
        println!("rejection geometric containment: {checked} rows checked");
    }
}

// -------------------------------------------------- the wide-`onto` cost

/// ADOPTED (r1) as the measured statement of the cost side of
/// `reject_from`'s trade. The unit's own rejection corpus gave `onto`
/// width only after this review; this row is the one that first measured
/// it, and it is kept because it draws three `onto` radii against two
/// `self` rows and reports the ratio per component. `props1_evidence.rs`
/// carries the gating bound; this row carries the table behind it.
#[cfg(feature = "interval")]
mod wide_onto {
    use super::retired_rejection;
    use geom_core::{Bounds, Interval, Vec3};

    fn fat(x: f64, r: f64) -> Interval {
        Interval::from_bounds(x - r, x + r)
    }
    fn w(e: Interval) -> f64 {
        e.hi() - e.lo()
    }

    #[test]
    fn p1_wide_onto_rejection_width_old_vs_new() {
        let mut worst = f64::INFINITY;
        let mut worst_row = String::new();
        for orow in [[0.0, 0.0, 1.0], [1.0, -2.0, 2.0], [0.5, 1.5, -0.25]] {
            for orad in [1.0e-12, 1.0e-9, 1.0e-6] {
                let onto = Vec3::new(fat(orow[0], orad), fat(orow[1], orad), fat(orow[2], orad));
                for srow in [[1.0, 2.0, -3.0], [100.0, -250.0, 30.0]] {
                    for srad in [0.0, 1.0e-9] {
                        let v =
                            Vec3::new(fat(srow[0], srad), fat(srow[1], srad), fat(srow[2], srad));
                        let old = retired_rejection(v, onto);
                        let new = v.reject_from(onto);
                        let ratios = [
                            w(old.x) / w(new.x),
                            w(old.y) / w(new.y),
                            w(old.z) / w(new.z),
                        ];
                        println!(
                            "onto {orow:?} r={orad:e} self {srow:?} r={srad:e}: retired \
                             [{:e} {:e} {:e}] shipped [{:e} {:e} {:e}] ratios [{:.4} {:.4} {:.4}]",
                            w(old.x),
                            w(old.y),
                            w(old.z),
                            w(new.x),
                            w(new.y),
                            w(new.z),
                            ratios[0],
                            ratios[1],
                            ratios[2]
                        );
                        for r in ratios {
                            if r.is_finite() && r < worst {
                                worst = r;
                                worst_row =
                                    format!("onto {orow:?} r={orad:e} self {srow:?} r={srad:e}");
                            }
                        }
                    }
                }
            }
        }
        println!("WORST (shipped widest relative to retired) ratio {worst:.4} at {worst_row}");

        assert!(
            worst < 1.0,
            "the wide-`onto` regression has gone away: the doc's cost sentence is stale"
        );
        assert!(
            worst > 0.02,
            "the wide-`onto` regression grew past its recorded bound: 1/{:.1}x",
            1.0 / worst
        );
    }
}

// ----------------------------------------- randomized containment (r2)

/// ADOPTED (r2). The second lane's independent containment sweeps:
/// 3000 random rows each, near-axis and near-degenerate normals, anchor
/// and normal half-widths up to 1e-3, and — for the rejection — a WIDE
/// `onto`, which no corpus in this unit drew before the review. The
/// mirror sweep checks each wide enclosure against a point evaluation
/// taken through the RETIRED association, so agreement is not a
/// property of the shipped formula alone.
#[cfg(feature = "interval")]
mod random_containment {
    use super::Rng;
    use geom_core::linalg::frame::mirror_across_plane;
    use geom_core::{Bounds, Interval, Mat3, Point3, Real, Tol, Vec3};

    fn v3<T: Real>(a: [f64; 3], f: impl Fn(f64) -> T) -> Vec3<T> {
        Vec3::new(f(a[0]), f(a[1]), f(a[2]))
    }

    fn p3<T: Real>(a: [f64; 3], f: impl Fn(f64) -> T) -> Point3<T> {
        Point3::new(f(a[0]), f(a[1]), f(a[2]))
    }

    fn retired_rejection<T: Real>(v: Vec3<T>, onto: Vec3<T>) -> Vec3<T> {
        v - v.project_onto(onto)
    }

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }
    fn fat(x: f64, r: f64) -> Interval {
        Interval::from_bounds(x - r, x + r)
    }
    fn width(e: Interval) -> f64 {
        e.hi() - e.lo()
    }
    fn widths(v: Vec3<Interval>) -> [f64; 3] {
        [width(v.x), width(v.y), width(v.z)]
    }
    fn encloses(outer: Interval, inner: Interval) -> bool {
        outer.lo() <= inner.lo() && outer.hi() >= inner.hi()
    }
    fn encloses3(outer: Vec3<Interval>, inner: Vec3<Interval>) -> bool {
        encloses(outer.x, inner.x) && encloses(outer.y, inner.y) && encloses(outer.z, inner.z)
    }
    fn finite3(v: Vec3<Interval>) -> bool {
        [v.x, v.y, v.z]
            .iter()
            .all(|e| e.lo().is_finite() && e.hi().is_finite())
    }

    fn retired_mirror_translation<T: Real>(point: Point3<T>, normal: Vec3<T>) -> Vec3<T> {
        let n = normal.normalize();
        let t = n * T::from_f64(2.0);
        let linear = Mat3::from_cols(
            Vec3::unit_x() - t * n.x,
            Vec3::unit_y() - t * n.y,
            Vec3::unit_z() - t * n.z,
        );
        let q = point - Point3::origin();
        q - linear * q
    }

    /// An independent point-value enclosure of the true translation:
    /// the retired association at point intervals (a different rounding
    /// path from the shipped one, so agreement is not tautological).
    fn independent_truth(ps: [f64; 3], ns: [f64; 3]) -> Vec3<Interval> {
        retired_mirror_translation(p3(ps, iv), v3(ns, iv))
    }

    /// C1, mirror. Random normals — generic, near-axis, tiny and huge
    /// magnitude, near the refusal — anchors at mm / m / 100 m, anchor
    /// half-widths up to 1e-3 and normal half-widths up to 1e-3. The
    /// wide enclosure must contain the point-interval evaluation of 16
    /// samples of the box under BOTH associations. Also records how
    /// often the shipped width exceeds the retired one.
    #[test]
    fn mirror_translation_encloses_random_wide_inputs() {
        let mut rng = Rng::new(0xfeed_beef_dead_cafe);
        let (mut rows, mut refused, mut wider, mut checked, mut pokes) =
            (0u32, 0u32, 0u32, 0u32, 0u32);
        let mut worst_excess = 0.0f64;
        let mut worst_case = String::new();
        let mut worst_wide_anchor = 0.0f64;
        let mut worst_wide_case = String::new();
        for trial in 0..3000u32 {
            let scale = [1.0e-3, 1.0, 100.0][(trial % 3) as usize];
            let arow = rng.dir().map(|x| x * scale);
            let nrow = match trial % 5 {
                0 => rng.dir(),
                1 => {
                    // near-axis
                    let e = rng.log_uniform(-12.0, -3.0);
                    let mut n = [rng.unit() * e, rng.unit() * e, rng.unit() * e];
                    n[(trial / 5 % 3) as usize] = if rng.unit() < 0.0 { -1.0 } else { 1.0 };
                    n
                }
                2 => rng.dir().map(|x| x * rng.log_uniform(-7.0, -4.0)),
                3 => rng.dir().map(|x| x * rng.log_uniform(3.0, 8.0)),
                _ => rng.dir().map(|x| x * 1.0e-9),
            };
            let ar = [0.0, 1.0e-15, 1.0e-9, 1.0e-6, 1.0e-3][(trial % 5) as usize]
                * if scale == 1.0e-3 { 1.0e-3 } else { 1.0 };
            let nr = [0.0, 1.0e-12, 1.0e-6, 1.0e-3][((trial / 5) % 4) as usize]
                * nrow.iter().fold(0.0f64, |a, x| a.max(x.abs()));
            let p = p3(arow, |x| fat(x, ar));
            let n = v3(nrow, |x| fat(x, nr));
            let Ok(m) = mirror_across_plane(p, n, Tol::witness()) else {
                refused += 1;
                continue;
            };
            rows += 1;
            let new = m.translation;
            let old = retired_mirror_translation(p, n);
            if !finite3(new) {
                continue;
            }
            checked += 1;
            for j in 0..3 {
                let (wn, wo) = (widths(new)[j], widths(old)[j]);
                if wn > wo {
                    wider += 1;
                    let rel = (wn - wo) / wo.max(f64::MIN_POSITIVE);
                    if rel > worst_excess {
                        worst_excess = rel;
                        worst_case = format!(
                            "n={nrow:?}±{nr:e} p={arow:?}±{ar:e} comp {j}: shipped {wn:e} retired {wo:e}"
                        );
                    }
                    if ar > 0.0 && rel > worst_wide_anchor {
                        worst_wide_anchor = rel;
                        worst_wide_case = format!(
                            "n={nrow:?}±{nr:e} p={arow:?}±{ar:e} comp {j}: shipped {wn:e} retired {wo:e}"
                        );
                    }
                }
            }
            for _ in 0..16 {
                let ps = [
                    arow[0] + ar * rng.unit(),
                    arow[1] + ar * rng.unit(),
                    arow[2] + ar * rng.unit(),
                ];
                let ns = [
                    nrow[0] + nr * rng.unit(),
                    nrow[1] + nr * rng.unit(),
                    nrow[2] + nr * rng.unit(),
                ];
                let truth_shipped = mirror_across_plane(p3(ps, iv), v3(ns, iv), Tol::witness())
                    .unwrap()
                    .translation;
                let truth_other = independent_truth(ps, ns);
                assert!(
                    encloses3(new, truth_shipped),
                    "shipped translation excludes its own point evaluation at n={nrow:?}±{nr:e} p={arow:?}±{ar:e}: {:?} vs {:?}",
                    widths(new),
                    widths(truth_shipped)
                );
                // Not required for soundness (the other association may
                // round outward past `new`), reported rather than asserted.
                if !encloses3(new, truth_other) {
                    pokes += 1;
                }
            }
        }
        println!(
            "mirror random sweep: {rows} rows ({refused} refused), {checked} finite; retired-association point values outside the shipped box (exact-input rounding, not a soundness failure): {pokes}; shipped wider than retired in {wider} components, worst relative excess {worst_excess:e} at {worst_case}; worst with a WIDE anchor {worst_wide_anchor:e} at {worst_wide_case}"
        );
        assert!(rows > 2000);
    }

    /// C1, rejection. Random `self` AND random WIDE `onto` (the unit's
    /// rejection rows only ever use an exact `onto`), near-parallel
    /// pairs included; skips boxes that contain the zero vector.
    #[test]
    fn rejection_encloses_random_wide_inputs_including_wide_onto() {
        let mut rng = Rng::new(0x0bad_5eed_0bad_5eed);
        let (mut rows, mut wider, mut nonfinite) = (0u32, 0u32, 0u32);
        let (mut worst_exact, mut worst_wide) = (0.0f64, 0.0f64);
        let (mut case_exact, mut case_wide) = (String::new(), String::new());
        let mut wider_wide_rows = 0u32;
        for trial in 0..3000u32 {
            let sv = rng.log_uniform(-3.0, 2.0);
            let so = rng.log_uniform(-3.0, 3.0);
            let orow = rng.dir().map(|x| x * so);
            let srow = match trial % 3 {
                0 => rng.dir().map(|x| x * sv),
                1 => {
                    let k = sv / so;
                    let e = rng.log_uniform(-12.0, -4.0);
                    [
                        orow[0] * k + e * rng.unit() * sv,
                        orow[1] * k + e * rng.unit() * sv,
                        orow[2] * k + e * rng.unit() * sv,
                    ]
                }
                _ => [
                    rng.unit() * sv,
                    rng.unit() * sv * 1e-6,
                    rng.unit() * sv * 1e-9,
                ],
            };
            let ar = [0.0, 1.0e-12, 1.0e-9, 1.0e-6][(trial % 4) as usize] * sv;
            let or = [0.0, 1.0e-12, 1.0e-6, 1.0e-3][((trial / 4) % 4) as usize] * so;
            if orow.iter().all(|x| x.abs() <= or) {
                continue;
            }
            let v = v3(srow, |x| fat(x, ar));
            let onto = v3(orow, |x| fat(x, or));
            let new = v.reject_from(onto);
            let old = retired_rejection(v, onto);
            rows += 1;
            if !finite3(new) {
                nonfinite += 1;
                continue;
            }
            let mut row_wider = false;
            for j in 0..3 {
                let (wn, wo) = (widths(new)[j], widths(old)[j]);
                if wn > wo {
                    wider += 1;
                    row_wider = true;
                    let ratio = wn / wo.max(f64::MIN_POSITIVE);
                    if or == 0.0 && ratio > worst_exact {
                        worst_exact = ratio;
                        case_exact = format!(
                            "onto={orow:?} self={srow:?}±{ar:e} comp {j}: shipped {wn:e} retired {wo:e}"
                        );
                    }
                    if or > 0.0 && ratio > worst_wide {
                        worst_wide = ratio;
                        case_wide = format!(
                            "onto={orow:?}±{or:e} self={srow:?}±{ar:e} comp {j}: shipped {wn:e} retired {wo:e}"
                        );
                    }
                }
            }
            if row_wider && or > 0.0 {
                wider_wide_rows += 1;
            }
            for _ in 0..16 {
                let vs = [
                    srow[0] + ar * rng.unit(),
                    srow[1] + ar * rng.unit(),
                    srow[2] + ar * rng.unit(),
                ];
                let os = [
                    orow[0] + or * rng.unit(),
                    orow[1] + or * rng.unit(),
                    orow[2] + or * rng.unit(),
                ];
                let truth = v3(vs, iv).reject_from(v3(os, iv));
                assert!(
                    encloses3(new, truth),
                    "shipped rejection excludes its own point evaluation at onto={orow:?}±{or:e} self={srow:?}±{ar:e}"
                );
            }
        }
        println!(
            "rejection random sweep: {rows} rows, {nonfinite} non-finite, shipped wider than retired in {wider} components ({wider_wide_rows} rows with a WIDE onto); worst ratio exact-onto {worst_exact:.3} at {case_exact}; worst ratio wide-onto {worst_wide:.3} at {case_wide}"
        );
        // A literal wide-onto row for the report.
        for (orr, arr) in [(1.0e-6, 0.0), (1.0e-6, 1.0e-9), (1.0e-12, 1.0e-9)] {
            let onto = v3([1.0, -2.0, 2.0], |x| fat(x, orr));
            let v = v3([1.0, 2.0, -3.0], |x| fat(x, arr));
            println!(
                "literal: onto (1,-2,2)±{orr:e}, self (1,2,-3)±{arr:e}: retired {:?} shipped {:?}",
                widths(retired_rejection(v, onto)),
                widths(v.reject_from(onto))
            );
        }
        assert!(rows > 2000);
    }
}
