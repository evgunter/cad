//! The lost-correlation measurements for `frame::mirror_across_plane`
//! and `Vec3::reject_from`, re-takeable, and the pins that gate them.
//!
//! Both constructors used to evaluate a correlated expression naively:
//! an operand was mentioned twice where one mention suffices, so at
//! `Interval` the enclosure paid width the geometry never had. The
//! numbers deciding whether each respell was worth its `f64` bit
//! movement are measurements, and a measurement whose instrument was
//! thrown away is a claim — so the corpora are written down here as
//! literals rather than described, exactly as `cert3_evidence.rs` does
//! for the anchored rotation. Every row prints all three components:
//! the whole point of the defect is that it lands in the components
//! where the answer should be exact, and a max-over-components summary
//! hides it.
//!
//! The `#[ignore]`d rows are the instruments: they assert nothing and
//! gate nothing, they print. Run them with
//!
//! ```text
//! cargo test -p geom-core --features interval --test all \
//!     -- --ignored --nocapture props1_evidence
//! ```
//!
//! The rest of the file gates, comparing what the constructors SHIP
//! against the retired spelling written out below. The `f64` rows run
//! in both lanes; the `enclosure` module's rows need the `interval`
//! feature, and are where the width and containment properties live —
//! narrower is worthless if it is wrong, so every width row has a
//! containment row beside it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::frame::mirror_across_plane;
use geom_core::{Mat3, Point3, Real, Tol, Vec3};

// ---------------------------------------------------------------- corpora

/// Plane normals. Not unit — the constructor normalizes: an exact
/// axis, an exact-integer oblique, and one whose normalization rounds.
const NORMALS: [[f64; 3]; 3] = [[0.0, 0.0, 1.0], [1.0, -2.0, 2.0], [0.5, 1.5, -0.25]];
/// Anchors at metre, 100 m and mm scale — the three scales the
/// anchored-rotation corpus uses, so the two instruments' rows compare.
const ANCHORS: [[f64; 3]; 3] = [
    [1.0, 2.0, -3.0],
    [100.0, -250.0, 30.0],
    [0.001, 0.002, -0.003],
];
/// Half-widths carried by each anchor coordinate: exact, one part in
/// 1e15 of a metre, and a subdivision-scale box.
#[cfg(feature = "interval")]
const ANCHOR_RADII: [f64; 3] = [0.0, 1.0e-15, 1.0e-9];
/// Half-widths carried by each normal component: exact, and a normal
/// that is itself an enclosure.
#[cfg(feature = "interval")]
const NORMAL_RADII: [f64; 2] = [0.0, 1.0e-12];

/// Vectors rejected in the `reject_from` rows.
const SELVES: [[f64; 3]; 3] = [
    [1.0, 2.0, -3.0],
    [100.0, -250.0, 30.0],
    [0.001, 0.002, -0.003],
];
/// Lines to reject from: an exact axis and an exact-integer oblique.
const ONTOS: [[f64; 3]; 2] = [[0.0, 0.0, 1.0], [1.0, -2.0, 2.0]];
/// The parallel row's vector is this multiple of `onto`, so the true
/// rejection is exactly the zero vector.
const PARALLEL_SCALE: f64 = 2.5;

fn v3<T: Real>(a: [f64; 3], f: impl Fn(f64) -> T) -> Vec3<T> {
    Vec3::new(f(a[0]), f(a[1]), f(a[2]))
}

fn p3<T: Real>(a: [f64; 3], f: impl Fn(f64) -> T) -> Point3<T> {
    Point3::new(f(a[0]), f(a[1]), f(a[2]))
}

fn scaled(a: [f64; 3], k: f64) -> [f64; 3] {
    [a[0] * k, a[1] * k, a[2] * k]
}

// ------------------------------------------- the retired spellings, written out

/// The mirror translation as it used to be spelled: `q − L·q`, with
/// the anchor mentioned twice. `L` is the constructor's own linear
/// part, rebuilt here verbatim so the differential is only about the
/// translation.
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

/// The rejection as it used to be spelled: `self − self.project_onto(onto)`,
/// with `self` mentioned twice.
fn retired_rejection<T: Real>(v: Vec3<T>, onto: Vec3<T>) -> Vec3<T> {
    v - v.project_onto(onto)
}

/// What the constructor ships today.
fn shipped_mirror_translation<T: geom_core::predicate::Decide>(
    point: Point3<T>,
    normal: Vec3<T>,
) -> Vec3<T> {
    mirror_across_plane(point, normal, Tol::witness())
        .unwrap()
        .translation
}

// ------------------------------------------------------------- `f64` rows

/// Signed-magnitude ulp distance between two finite `f64`s, monotone
/// across zero.
fn ulps(a: f64, b: f64) -> i64 {
    let key = |x: f64| -> i64 {
        let b = x.to_bits() as i64;
        if b < 0 { i64::MIN - b } else { b }
    };
    key(a).saturating_sub(key(b)).abs()
}

/// How far each constructor's `f64` output moved when the repeated
/// mention was removed, over the corpora above — the cost side of the
/// trade, in ulps and relative to the operand's own magnitude.
#[test]
#[ignore = "the constructor bit-movement instrument; run explicitly"]
fn constructor_bit_movement_over_the_recorded_corpora() {
    let (mut moved, mut total, mut max_ulp, mut max_rel) = (0usize, 0usize, 0i64, 0.0f64);
    for nrow in NORMALS {
        let n = v3(nrow, |x| x);
        for arow in ANCHORS {
            let p = p3(arow, |x| x);
            let mag = (p - Point3::origin()).norm();
            let old = retired_mirror_translation(p, n);
            let new = shipped_mirror_translation(p, n);
            let mut worst = 0i64;
            for (u, v) in [(old.x, new.x), (old.y, new.y), (old.z, new.z)] {
                total += 1;
                if u.to_bits() != v.to_bits() {
                    moved += 1;
                }
                worst = worst.max(ulps(u, v));
                max_rel = max_rel.max((u - v).abs() / mag);
            }
            max_ulp = max_ulp.max(worst);
            println!("mirror {nrow:?} {arow:?}: worst {worst} ulp");
        }
    }
    println!("mirror translation: moved {moved} of {total} components");
    println!("  max ulp distance {max_ulp}; max |delta|/|q| {max_rel:e}");

    let (mut rmoved, mut rtotal, mut rmax_ulp, mut rmax_rel) = (0usize, 0usize, 0i64, 0.0f64);
    for orow in ONTOS {
        let onto = v3(orow, |x| x);
        for srow in SELVES.iter().copied().chain([scaled(orow, PARALLEL_SCALE)]) {
            let v = v3(srow, |x| x);
            let old = retired_rejection(v, onto);
            let new = v.reject_from(onto);
            let mag = v.norm();
            let mut worst = 0i64;
            for (u, w) in [(old.x, new.x), (old.y, new.y), (old.z, new.z)] {
                rtotal += 1;
                if u.to_bits() != w.to_bits() {
                    rmoved += 1;
                }
                worst = worst.max(ulps(u, w));
                rmax_rel = rmax_rel.max((u - w).abs() / mag);
            }
            rmax_ulp = rmax_ulp.max(worst);
            println!("reject {orow:?} {srow:?}: worst {worst} ulp");
        }
    }
    println!("rejection: moved {rmoved} of {rtotal} components");
    println!("  max ulp distance {rmax_ulp}; max |delta|/|self| {rmax_rel:e}");
}

/// The `f64` accuracy of the shipped mirror over the whole corpus: the
/// anchor is a point of its own plane, so it must come back fixed, and
/// the map must be an involution. Runs in both lanes — the enclosure
/// rows below cannot, and soundness is not an interval-only property.
#[test]
fn mirror_is_accurate_at_f64_over_the_corpus() {
    for nrow in NORMALS {
        let n = v3(nrow, |x| x);
        for arow in ANCHORS {
            let p = p3(arow, |x| x);
            let mag = (p - Point3::origin()).norm();
            let m = mirror_across_plane(p, n, Tol::witness()).unwrap();
            let fixed = (m.transform_point(p) - p).norm() / mag;
            assert!(
                fixed <= 1.0e-15,
                "anchor not fixed: {fixed:e} for {nrow:?} {arow:?}"
            );
            let back = (m.transform_point(m.transform_point(p)) - p).norm() / mag;
            assert!(back <= 1.0e-15, "not an involution: {back:e}");
        }
    }
}

/// The `f64` accuracy of the shipped rejection: orthogonal to `onto`,
/// and `project + reject` reconstructs `self`. Both claims the doc
/// makes, over the corpus, in both lanes.
#[test]
fn rejection_is_accurate_at_f64_over_the_corpus() {
    for orow in ONTOS {
        let onto = v3(orow, |x| x);
        for srow in SELVES.iter().copied().chain([scaled(orow, PARALLEL_SCALE)]) {
            let v = v3(srow, |x| x);
            let r = v.reject_from(onto);
            let ortho = r.dot(onto).abs() / (v.norm() * onto.norm());
            assert!(ortho <= 1.0e-16, "not orthogonal: {ortho:e} for {srow:?}");
            let rel = (v.project_onto(onto) + r - v).norm() / v.norm();
            assert!(rel <= 1.0e-15, "project + reject != self: {rel:e}");
        }
    }
}

/// The two rounding claims `Vec3::reject_from`'s doc makes, measured
/// for the shipped spelling beside the retired one: how far from
/// orthogonal to `onto` the rejection lands, relative to
/// `|self|·|onto|`, and how far `project + reject` lands from `self`,
/// in ulps of the largest component. The doc quotes these.
#[test]
#[ignore = "the rejection rounding-claim instrument; run explicitly"]
fn rejection_rounding_claims() {
    let (mut o_new, mut o_old) = (0.0f64, 0.0f64);
    let (mut r_new, mut r_old) = (0i64, 0i64);
    for orow in ONTOS {
        let onto = v3(orow, |x| x);
        let mut rows: Vec<[f64; 3]> = SELVES.to_vec();
        rows.push(scaled(orow, PARALLEL_SCALE));
        // Near-parallel: the row where a subtractive spelling cancels
        // catastrophically and a triple product does not.
        for k in [1.0e-6, 1.0e-12] {
            let s = scaled(orow, PARALLEL_SCALE);
            rows.push([s[0] + k, s[1] - k, s[2] + k]);
        }
        for srow in rows {
            let v = v3(srow, |x| x);
            let scale = v.norm() * onto.norm();
            for (r, o, rec) in [
                (v.reject_from(onto), &mut o_new, &mut r_new),
                (retired_rejection(v, onto), &mut o_old, &mut r_old),
            ] {
                *o = o.max(r.dot(onto).abs() / scale);
                let sum = v.project_onto(onto) + r;
                for (a, b) in [(sum.x, v.x), (sum.y, v.y), (sum.z, v.z)] {
                    *rec = (*rec).max(ulps(a, b));
                }
            }
        }
    }
    println!("orthogonality |r.onto|/(|self||onto|): shipped {o_new:e}, retired {o_old:e}");
    println!("project + reject vs self, worst component: shipped {r_new} ulp, retired {r_old} ulp");
}

/// The parallel row at `f64`: `self ∥ onto` has rejection exactly zero,
/// reached without a cancelling subtraction.
#[test]
fn parallel_rejection_is_exactly_zero_at_f64() {
    for orow in ONTOS {
        let onto = v3(orow, |x| x);
        let r = (onto * PARALLEL_SCALE).reject_from(onto);
        assert_eq!((r.x, r.y, r.z), (0.0, 0.0, 0.0), "for onto {orow:?}");
    }
}

// -------------------------------------------------------- enclosure rows

/// The width and containment rows. `Interval` only: an enclosure is
/// the only lane where a repeated mention costs anything.
#[cfg(feature = "interval")]
mod enclosure {
    use super::{
        ANCHOR_RADII, ANCHORS, NORMAL_RADII, NORMALS, ONTOS, PARALLEL_SCALE, SELVES, p3,
        retired_mirror_translation, retired_rejection, scaled, shipped_mirror_translation, v3,
    };
    use geom_core::{Bounds, Interval, Point3, Real, Vec3};

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    /// A scalar of stated width centred on `x` — the enclosure a
    /// subdivision driver's sub-box hands a constructor.
    fn fat(x: f64, r: f64) -> Interval {
        Interval::from_bounds(x - r, x + r)
    }

    fn width(e: Interval) -> f64 {
        e.hi() - e.lo()
    }

    fn widths(v: Vec3<Interval>) -> [f64; 3] {
        [width(v.x), width(v.y), width(v.z)]
    }

    fn fmt(w: [f64; 3]) -> String {
        format!("[{:e} {:e} {:e}]", w[0], w[1], w[2])
    }

    fn encloses(outer: Interval, inner: Interval) -> bool {
        outer.lo() <= inner.lo() && outer.hi() >= inner.hi()
    }

    fn encloses3(outer: Vec3<Interval>, inner: Vec3<Interval>) -> bool {
        encloses(outer.x, inner.x) && encloses(outer.y, inner.y) && encloses(outer.z, inner.z)
    }

    /// The unit normal at `f64`, and its L1 norm — the two numbers the
    /// true image width of the anchor box is built from.
    fn unit_and_l1(nrow: [f64; 3]) -> ([f64; 3], f64) {
        let m = (nrow[0] * nrow[0] + nrow[1] * nrow[1] + nrow[2] * nrow[2]).sqrt();
        let u = [nrow[0] / m, nrow[1] / m, nrow[2] / m];
        let l1 = u[0].abs() + u[1].abs() + u[2].abs();
        (u, l1)
    }

    /// The width of the TRUE image of the anchor box under
    /// `q ↦ 2·n̂·(n̂·q)`, component by component: the tightest any sound
    /// enclosure can be, and what the single-mention spelling attains.
    /// Built from the box the corpus ACTUALLY realizes — a half-width
    /// below the ulp of a 100 m coordinate rounds away, and a nominal
    /// radius would then overstate the bound.
    fn ideal_mirror_widths(nrow: [f64; 3], p: Point3<Interval>) -> [f64; 3] {
        let (u, _) = unit_and_l1(nrow);
        let q = [width(p.x), width(p.y), width(p.z)];
        let dot: f64 = (0..3).map(|k| u[k].abs() * q[k]).sum();
        [
            2.0 * u[0].abs() * dot,
            2.0 * u[1].abs() * dot,
            2.0 * u[2].abs() * dot,
        ]
    }

    /// The seed is a literal for the same reason the corpora are: a
    /// described random sweep reproduces the conclusion, not the rows.
    struct Rng(u64);

    impl Rng {
        fn new() -> Self {
            Self(0x9E37_79B9_7F4A_7C15)
        }
        fn unit(&mut self) -> f64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            f64::from((self.0 >> 40) as u32) / 8_388_608.0 - 1.0
        }
    }

    /// A point sampled uniformly from the box `centre ± r`.
    fn sample(centre: [f64; 3], r: f64, rng: &mut Rng) -> [f64; 3] {
        [
            centre[0] + r * rng.unit(),
            centre[1] + r * rng.unit(),
            centre[2] + r * rng.unit(),
        ]
    }

    /// THE MIRROR TABLE. Retired and shipped translation widths per
    /// corpus row, component by component, beside the true width of
    /// the image of the anchor box.
    #[test]
    #[ignore = "the mirror width instrument; run explicitly"]
    fn mirror_translation_widths() {
        println!("normal | anchor | r(anchor) | r(normal) | retired | shipped | true image");
        for nrow in NORMALS {
            for nr in NORMAL_RADII {
                let n = v3(nrow, |x| fat(x, nr));
                for arow in ANCHORS {
                    for ar in ANCHOR_RADII {
                        let p = p3(arow, |x| fat(x, ar));
                        let old = widths(retired_mirror_translation(p, n));
                        let new = widths(shipped_mirror_translation(p, n));
                        let ideal = ideal_mirror_widths(nrow, p);
                        println!(
                            "{nrow:?} | {arow:?} | {ar:e} | {nr:e} | {} | {} | {}",
                            fmt(old),
                            fmt(new),
                            fmt(ideal)
                        );
                    }
                }
            }
        }
    }

    /// THE REJECTION TABLE. Retired and shipped rejection widths per
    /// corpus row, component by component, including the parallel rows
    /// where the true rejection is the zero vector.
    #[test]
    #[ignore = "the rejection width instrument; run explicitly"]
    fn rejection_widths() {
        println!("onto | self | r(self) | retired | shipped");
        for orow in ONTOS {
            let onto = v3(orow, iv);
            for srow in SELVES.iter().copied().chain([scaled(orow, PARALLEL_SCALE)]) {
                for ar in ANCHOR_RADII {
                    let v = v3(srow, |x| fat(x, ar));
                    println!(
                        "{orow:?} | {srow:?} | {ar:e} | {} | {}",
                        fmt(widths(retired_rejection(v, onto))),
                        fmt(widths(v.reject_from(onto)))
                    );
                }
            }
        }
    }

    /// PIN (a). Two claims about the shipped mirror translation, per
    /// component: it attains the TRUE width of the image of the anchor
    /// box up to its rounding floor (the single-mention bound — no
    /// sound enclosure can be narrower), and it is never wider than the
    /// retired spelling beyond those floors. Where the plane's normal
    /// has a vanishing component the retired spelling misses that bound
    /// by the whole anchor width, which is the defect being retired.
    #[test]
    fn mirror_translation_meets_the_single_mention_bound() {
        for nrow in NORMALS {
            for nr in NORMAL_RADII {
                let n = v3(nrow, |x| fat(x, nr));
                for arow in ANCHORS {
                    let exact = p3(arow, |x| fat(x, 0.0));
                    let floor_new = widths(shipped_mirror_translation(exact, n));
                    let floor_old = widths(retired_mirror_translation(exact, n));
                    for ar in ANCHOR_RADII {
                        let p = p3(arow, |x| fat(x, ar));
                        let old = widths(retired_mirror_translation(p, n));
                        let new = widths(shipped_mirror_translation(p, n));
                        let ideal = ideal_mirror_widths(nrow, p);
                        for j in 0..3 {
                            if nr == 0.0 {
                                // The floor is allowed to double: the
                                // same roundings act on wider operands
                                // once the anchor carries width.
                                let bound = 2.0 * floor_new[j] + ideal[j] * (1.0 + 1.0e-12);
                                assert!(
                                    new[j] <= bound,
                                    "single-mention bound missed at {nrow:?} {arow:?} \
                                     r={ar:e} component {j}: {:e} > {bound:e}",
                                    new[j]
                                );
                            }
                            assert!(
                                new[j] <= old[j] + floor_new[j] + floor_old[j],
                                "shipped wider than retired at {nrow:?} {arow:?} r={ar:e} \
                                 component {j}: {:e} > {:e}",
                                new[j],
                                old[j]
                            );
                        }
                    }
                }
            }
        }
    }

    /// PIN (a), the other half: on a component where the plane's normal
    /// vanishes the true translation is exactly zero at every point of
    /// the anchor box, the shipped spelling says so, and the retired
    /// one paid the whole anchor width there.
    #[test]
    fn a_vanishing_normal_component_costs_the_retired_spelling_the_anchor_width() {
        let n = v3([0.0, 0.0, 1.0], |x| fat(x, 0.0));
        for arow in ANCHORS {
            for ar in ANCHOR_RADII {
                if ar == 0.0 {
                    continue;
                }
                let p = p3(arow, |x| fat(x, ar));
                let anchor = [width(p.x), width(p.y), width(p.z)];
                let old = widths(retired_mirror_translation(p, n));
                let new = widths(shipped_mirror_translation(p, n));
                for j in [0usize, 1] {
                    // A half-width below the ulp of the coordinate
                    // rounds away; that row has nothing to pay.
                    if anchor[j] == 0.0 {
                        continue;
                    }
                    assert_eq!(new[j], 0.0, "shipped is not exact at {arow:?} r={ar:e}");
                    assert!(
                        old[j] >= 1.99 * anchor[j],
                        "retired did not pay twice the anchor width at {arow:?} r={ar:e} \
                         component {j}: {:e} for a coordinate of width {:e}",
                        old[j],
                        anchor[j]
                    );
                }
            }
        }
    }

    /// PIN (b): `reject_from` collapses on the parallel row, where the
    /// true rejection is the zero vector — the retired spelling did
    /// not, because it subtracted a wide `self` from itself.
    #[test]
    fn parallel_rejection_is_narrower_than_the_retired_spelling() {
        for orow in ONTOS {
            let onto = v3(orow, iv);
            let srow = scaled(orow, PARALLEL_SCALE);
            for ar in ANCHOR_RADII {
                if ar == 0.0 {
                    continue;
                }
                let v = v3(srow, |x| fat(x, ar));
                let old = widths(retired_rejection(v, onto));
                let new = widths(v.reject_from(onto));
                let (so, sn): (f64, f64) = (old.iter().sum(), new.iter().sum());
                assert!(
                    sn < so,
                    "parallel row did not narrow at {orow:?} r={ar:e}: {sn:e} vs {so:e}"
                );
            }
        }
    }

    /// PIN (c): SOUNDNESS, mirror. For a sampled point of the anchor box
    /// and of the normal box, the enclosure the wide inputs produce
    /// contains the enclosure the exact sample produces — so it contains
    /// the true reflection at that sample. Narrower is worthless if it
    /// is wrong.
    #[test]
    fn mirror_translation_encloses_every_sampled_point_of_the_box() {
        let mut rng = Rng::new();
        for nrow in NORMALS {
            for nr in NORMAL_RADII {
                let n = v3(nrow, |x| fat(x, nr));
                for arow in ANCHORS {
                    for ar in ANCHOR_RADII {
                        let p = p3(arow, |x| fat(x, ar));
                        let new = shipped_mirror_translation(p, n);
                        for _ in 0..32 {
                            let ps = sample(arow, ar, &mut rng);
                            let ns = sample(nrow, nr, &mut rng);
                            let truth = shipped_mirror_translation(p3(ps, iv), v3(ns, iv));
                            assert!(
                                encloses3(new, truth),
                                "shipped mirror translation excludes a point of its own box: \
                                 {nrow:?} {arow:?} r={ar:e} rn={nr:e}"
                            );
                        }
                    }
                }
            }
        }
    }

    /// PIN (c), rejection half: the shipped triple product encloses the
    /// rejection of every sampled point of `self`'s box.
    #[test]
    fn rejection_encloses_every_sampled_point_of_the_box() {
        let mut rng = Rng::new();
        for orow in ONTOS {
            let onto = v3(orow, iv);
            for srow in SELVES.iter().copied().chain([scaled(orow, PARALLEL_SCALE)]) {
                for ar in ANCHOR_RADII {
                    let v = v3(srow, |x| fat(x, ar));
                    let new = v.reject_from(onto);
                    for _ in 0..32 {
                        let vs = sample(srow, ar, &mut rng);
                        let truth = v3(vs, iv).reject_from(onto);
                        assert!(
                            encloses3(new, truth),
                            "shipped rejection excludes a point of its own box: \
                             {orow:?} {srow:?} r={ar:e}"
                        );
                    }
                }
            }
        }
    }

    /// The doc's reconstruction claim, re-derived for the shipped
    /// spelling: `project + reject` is no longer `self` by one rounding
    /// per component, because the two are no longer built from a common
    /// subtraction. What holds is the enclosure statement — the sum
    /// still brackets `self` to within the two products' rounding.
    #[test]
    fn project_plus_reject_reconstructs_self_as_an_enclosure() {
        for orow in ONTOS {
            let onto = v3(orow, iv);
            for srow in SELVES {
                let v = v3(srow, iv);
                let sum = v.project_onto(onto) + v.reject_from(onto);
                let scale = 4.0 * f64::EPSILON * v.norm().hi();
                for (s, e) in [(sum.x, v.x), (sum.y, v.y), (sum.z, v.z)] {
                    assert!(
                        (s.lo() - e.lo()).abs() <= scale && (s.hi() - e.hi()).abs() <= scale,
                        "reconstruction drifted at {orow:?} {srow:?}: {:e}..{:e} vs {:e}..{:e}",
                        s.lo(),
                        s.hi(),
                        e.lo(),
                        e.hi()
                    );
                }
            }
        }
    }
}
