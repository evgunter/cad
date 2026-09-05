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
/// translation — and it is a HAND COPY that nothing keeps in step: a
/// change to the constructor's columns has to be mirrored here by hand
/// or the differential stops being one.
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

/// ONE METRIC, defined here and used by every statement about
/// `project + reject`: the error is measured in **ulps of the vector's
/// largest component**, so a 100 m x-coordinate and a 1 mm z-coordinate
/// are held to the same absolute scale rather than the z one being
/// judged against its own tiny ulp. The doc quotes this metric and the
/// adversarial row in `props1_review_rows.rs` gates its worst value.
fn reconstruction_ulps(sum: Vec3<f64>, v: Vec3<f64>) -> f64 {
    let biggest = v.x.abs().max(v.y.abs()).max(v.z.abs());
    let ulp = f64::from_bits(biggest.to_bits() + 1) - biggest;
    [(sum.x - v.x), (sum.y - v.y), (sum.z - v.z)]
        .into_iter()
        .fold(0.0f64, |a, d| a.max(d.abs() / ulp))
}

/// The `f64` accuracy of the shipped rejection: orthogonal to `onto`,
/// and `project + reject` reconstructs `self`. Both claims the doc
/// makes, over the corpus, in both lanes, at the numbers the doc quotes
/// rather than at a round number a decade looser.
#[test]
fn rejection_is_accurate_at_f64_over_the_corpus() {
    let (mut worst_ortho, mut worst_rec) = (0.0f64, 0.0f64);
    for orow in ONTOS {
        let onto = v3(orow, |x| x);
        for srow in SELVES.iter().copied().chain([scaled(orow, PARALLEL_SCALE)]) {
            let v = v3(srow, |x| x);
            let r = v.reject_from(onto);
            worst_ortho = worst_ortho.max(r.dot(onto).abs() / (v.norm() * onto.norm()));
            worst_rec = worst_rec.max(reconstruction_ulps(v.project_onto(onto) + r, v));
        }
    }
    // The doc quotes 3.5e-17; the slack is one binade, not a decade.
    assert!(
        worst_ortho <= 5.0e-17,
        "orthogonality is worse than the documented 3.5e-17: {worst_ortho:e}"
    );
    assert!(
        worst_rec <= 4.0,
        "reconstruction is worse than the documented 4 ulps of the largest component: \
         {worst_rec}"
    );
    println!("corpus: orthogonality {worst_ortho:e}, reconstruction {worst_rec} ulp");
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
    let (mut r_new, mut r_old) = (0.0f64, 0.0f64);
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
                *rec = rec.max(reconstruction_ulps(v.project_onto(onto) + r, v));
            }
        }
    }
    println!("orthogonality |r.onto|/(|self||onto|): shipped {o_new:e}, retired {o_old:e}");
    println!(
        "project + reject vs self, ulps of the largest component: shipped {r_new}, retired {r_old}"
    );
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
        ANCHORS, NORMALS, ONTOS, PARALLEL_SCALE, SELVES, p3, retired_mirror_translation,
        retired_rejection, scaled, shipped_mirror_translation, v3,
    };
    use geom_core::{Bounds, Interval, Point3, Real, Vec3};

    /// Half-widths carried by each anchor coordinate: exact, one part in
    /// 1e15 of a metre, and a subdivision-scale box.
    const ANCHOR_RADII: [f64; 3] = [0.0, 1.0e-15, 1.0e-9];

    /// Half-widths carried by each normal component: exact, and a normal
    /// that is itself an enclosure.
    const NORMAL_RADII: [f64; 2] = [0.0, 1.0e-12];

    /// Half-widths carried by each `onto` component. The shipped
    /// rejection amplifies `onto`'s width through two cross products, so
    /// a corpus whose `onto` is always exact cannot see the cost side of
    /// the trade at all; these are the rows that can.
    const ONTO_RADII: [f64; 3] = [0.0, 1.0e-12, 1.0e-6];

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
    fn ideal_mirror_widths(nrow: [f64; 3], p: Point3<Interval>, nr: f64) -> [f64; 3] {
        let (u, _) = unit_and_l1(nrow);
        // A wide normal enlarges the image: bound each unit component by
        // its midpoint plus the half-width the enclosure could add.
        let m = (nrow[0] * nrow[0] + nrow[1] * nrow[1] + nrow[2] * nrow[2]).sqrt();
        let d = 2.0 * nr / m;
        let b = [u[0].abs() + d, u[1].abs() + d, u[2].abs() + d];
        let q = [width(p.x), width(p.y), width(p.z)];
        let dot: f64 = (0..3).map(|k| b[k] * q[k]).sum();
        [2.0 * b[0] * dot, 2.0 * b[1] * dot, 2.0 * b[2] * dot]
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
                        let ideal = ideal_mirror_widths(nrow, p, nr);
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
        println!("onto | r(onto) | self | r(self) | retired | shipped");
        for orow in ONTOS {
            for or in ONTO_RADII {
                let onto = v3(orow, |x| fat(x, or));
                for srow in SELVES.iter().copied().chain([scaled(orow, PARALLEL_SCALE)]) {
                    for ar in ANCHOR_RADII {
                        let v = v3(srow, |x| fat(x, ar));
                        println!(
                            "{orow:?} | {or:e} | {srow:?} | {ar:e} | {} | {}",
                            fmt(widths(retired_rejection(v, onto))),
                            fmt(widths(v.reject_from(onto)))
                        );
                    }
                }
            }
        }
    }

    /// PIN (a). THE TIGHTNESS CLAIM, and it can fail. Per component,
    /// the shipped mirror translation is at most its own rounding floor
    /// (measured on the same row with an exact anchor) plus the width of
    /// the TRUE image of the anchor box under `q ↦ 2·n̂·(n̂·q)`. No sound
    /// enclosure of that image can be narrower, so a spelling that
    /// carries dependency width misses this bound — and the row proves
    /// the bound is not vacuous by counting the rows where the RETIRED
    /// spelling misses it and requiring that count to be non-zero.
    ///
    /// Asserted at every normal width, not only the exact one: a normal
    /// carrying half-width `nr` enlarges the image, and the bound widens
    /// each unit component by `2·nr/|normal|` to account for it.
    #[test]
    fn mirror_translation_meets_the_single_mention_bound() {
        let mut retired_misses = 0usize;
        let mut rows_with_anchor_width = 0usize;
        for nrow in NORMALS {
            for nr in NORMAL_RADII {
                let n = v3(nrow, |x| fat(x, nr));
                for arow in ANCHORS {
                    let exact = p3(arow, |x| fat(x, 0.0));
                    let floor_new = widths(shipped_mirror_translation(exact, n));
                    let floor_old = widths(retired_mirror_translation(exact, n));
                    for ar in ANCHOR_RADII {
                        let p = p3(arow, |x| fat(x, ar));
                        if [width(p.x), width(p.y), width(p.z)] == [0.0; 3] {
                            continue;
                        }
                        rows_with_anchor_width += 1;
                        let old = widths(retired_mirror_translation(p, n));
                        let new = widths(shipped_mirror_translation(p, n));
                        let ideal = ideal_mirror_widths(nrow, p, nr);
                        for j in 0..3 {
                            // A normal that is itself an enclosure gets
                            // twice the image width: `n̂`'s own width
                            // multiplies the anchor's in two places, the
                            // dot product and the final scale. Measured
                            // at 1.5x; the bound is 2x.
                            let slack = if nr == 0.0 { 1.0 } else { 2.0 };
                            // The floor is allowed to double as well:
                            // the same roundings act on wider operands
                            // once the anchor carries width.
                            let bound = 2.0 * floor_new[j] + slack * ideal[j] * (1.0 + 1.0e-9);
                            assert!(
                                new[j] <= bound,
                                "single-mention bound missed at {nrow:?}±{nr:e} {arow:?} \
                                 r={ar:e} component {j}: {:e} > {bound:e}",
                                new[j]
                            );
                            if old[j] > floor_old[j] + ideal[j] * (1.0 + 1.0e-9) {
                                retired_misses += 1;
                            }
                        }
                    }
                }
            }
        }
        assert!(
            retired_misses > 0,
            "the bound is vacuous: the retired spelling meets it on all \
             {rows_with_anchor_width} rows"
        );
        println!(
            "single-mention bound: {rows_with_anchor_width} rows with anchor width; the retired \
             spelling misses the bound in {retired_misses} components"
        );
    }

    /// PIN (a)'s FLOOR, measured rather than claimed. The spec asked for
    /// "narrower than the old form on every corpus row"; that is FALSE
    /// per component once the anchor is exact, where both widths are
    /// pure rounding floor and which floor is smaller is a rounding
    /// accident. Over the committed corpus the shipped floor is wider in
    /// some components, by at most the ratio pinned here — small in
    /// absolute terms (1e-17 to 1e-13 m) and independent of the anchor
    /// width, which is what the respell was about.
    #[test]
    fn the_exact_anchor_rounding_floor_can_favour_either_spelling() {
        let (mut wider, mut total) = (0usize, 0usize);
        let mut worst = 0.0f64;
        for nrow in NORMALS {
            for nr in NORMAL_RADII {
                let n = v3(nrow, |x| fat(x, nr));
                for arow in ANCHORS {
                    let p = p3(arow, |x| fat(x, 0.0));
                    let old = widths(retired_mirror_translation(p, n));
                    let new = widths(shipped_mirror_translation(p, n));
                    for j in 0..3 {
                        total += 1;
                        if new[j] > old[j] {
                            wider += 1;
                            worst = worst.max(new[j] / old[j]);
                        }
                        // With an exact normal the floor is pure
                        // rounding on a 100 m anchor; a normal carrying
                        // 1e-12 of its own contributes far more, and
                        // that is the normal's width, not this floor.
                        if nr == 0.0 {
                            assert!(
                                new[j] <= 2.0e-12,
                                "the exact-anchor floor is no longer a floor at {nrow:?} \
                                 {arow:?} component {j}: {:e}",
                                new[j]
                            );
                        }
                    }
                }
            }
        }
        assert!(
            wider > 0,
            "the shipped floor is never wider: restate the deviation, it has gone away"
        );
        assert!(
            worst <= 1.6,
            "the shipped exact-anchor floor got worse: {worst:.3}x the retired one"
        );
        println!(
            "exact-anchor floor: shipped wider in {wider} of {total} components, worst ratio \
             {worst:.3}x"
        );
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

    /// PIN (A): THE COST SIDE, pinned. The shipped rejection names
    /// `self` once and `onto` three times THROUGH TWO CROSS PRODUCTS,
    /// so where `onto` itself carries width the shipped enclosure is
    /// WIDER than the retired one — the retired spelling's `onto`
    /// mentions sit inside one scalar quotient instead. Every in-tree
    /// consumer passes an exact stored axis as `onto`, which is why the
    /// shipped form is the right trade, but the regression is real and
    /// this row holds it to its measured bound. It also requires the
    /// regression to still be there, so the row fails if the amplifier
    /// is ever removed and the doc is left stale.
    #[test]
    fn a_wide_onto_costs_the_shipped_rejection_width() {
        let (mut wider, mut total) = (0usize, 0usize);
        let mut worst = 0.0f64;
        let mut worst_row = String::new();
        for orow in ONTOS {
            for or in ONTO_RADII {
                let onto = v3(orow, |x| fat(x, or));
                for srow in SELVES {
                    for ar in ANCHOR_RADII {
                        let v = v3(srow, |x| fat(x, ar));
                        let old = widths(retired_rejection(v, onto));
                        let new = widths(v.reject_from(onto));
                        for j in 0..3 {
                            total += 1;
                            if new[j] > old[j] {
                                wider += 1;
                                let ratio = new[j] / old[j].max(f64::MIN_POSITIVE);
                                if ratio > worst {
                                    worst = ratio;
                                    worst_row = format!(
                                        "onto {orow:?}±{or:e} self {srow:?}±{ar:e} component {j}"
                                    );
                                }
                            }
                            // With an exact `onto` and a `self` that
                            // carries width, the shipped spelling is the
                            // narrower one — the whole point. With BOTH
                            // exact, both widths are rounding floor and
                            // which is smaller is an accident, so that
                            // row is bounded absolutely instead.
                            if or == 0.0 && ar > 0.0 {
                                assert!(
                                    new[j] <= old[j] * 1.1 + 1.0e-300,
                                    "an EXACT onto cost width beyond the rounding floor: \
                                     {orow:?} {srow:?} r={ar:e} component {j}: {:e} vs {:e}",
                                    new[j],
                                    old[j]
                                );
                            } else if or == 0.0 {
                                assert!(
                                    new[j] <= 1.0e-13,
                                    "the exact-input rejection floor moved at {orow:?} \
                                     {srow:?} component {j}: {:e}",
                                    new[j]
                                );
                            }
                        }
                    }
                }
            }
        }
        assert!(
            wider > 0,
            "no wide-`onto` regression measured: the doc's cost sentence is stale"
        );
        assert!(
            worst <= 40.0,
            "the wide-`onto` regression grew past its recorded bound: {worst:.1}x at {worst_row}"
        );
        println!(
            "wide onto: shipped wider than retired in {wider} of {total} components, worst \
             {worst:.1}x at {worst_row}"
        );
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
