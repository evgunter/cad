//! The anchored-rotation measurements, re-takeable.
//!
//! The numbers behind `Affine3::rotation_about_axis`'s change live in
//! three places — the constructor's doc comments, the
//! `revolved_point_anchor` rows, and the `editor-core` m10-p fence
//! header. Every one of them is a measurement, and a measurement whose
//! instrument was thrown away is a claim: the same sweep re-derived
//! from a prose description of its corpus ("3 axes x 3 anchors x 9
//! angles") reproduces the CONCLUSION and not the DIGITS, because the
//! corpus is not the description.
//!
//! So the instruments are here, with their corpora written down as
//! literals rather than described. `#[ignore]`d — they assert nothing
//! and gate nothing; they print, and their output is what the prose
//! elsewhere quotes. Run one with
//!
//! ```text
//! cargo test -p geom-core --features interval --test all \
//!     -- --ignored --nocapture cert3_evidence
//! ```
//!
//! The kernel-wide half of the measurement is not here: the corpus
//! coordinate-dump differential lives in
//! `crates/editor-core/tests/cert3r1_dump.rs`, because it needs the
//! evaluator. The m10-p fence header names it.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Bounds, Interval, Mat3, Point3, Real, Vec3};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}

/// THE CORPUS, written down. Three axes (`+z`, oblique, `+x`), three
/// anchors (metre, 100 m, mm scale), nine angles — 81 triples, 243
/// components. Changing any literal here changes the digits the prose
/// quotes, which is the point of it being a literal.
const AXES: [[f64; 3]; 3] = [[0.0, 0.0, 1.0], [1.0, -2.0, 2.0], [1.0, 0.0, 0.0]];
const ANCHORS: [[f64; 3]; 3] = [
    [1.0, 2.0, -3.0],
    [100.0, -250.0, 30.0],
    [0.001, 0.002, -0.003],
];
const ANGLES: [f64; 9] = [
    1.0e-9,
    1.0e-6,
    0.001,
    0.1,
    core::f64::consts::FRAC_PI_2,
    core::f64::consts::PI,
    2.0,
    5.0,
    core::f64::consts::TAU,
];

/// How far the constructor's `f64` output moved when the translation
/// stopped being `q − R·q`, over the corpus above.
///
/// Also the fixed-point residual both spellings leave on the anchor —
/// the one property where the retired form is BETTER, because
/// mentioning `q` twice buys a correlated-rounding advantage there,
/// which is exactly the mention being removed.
#[test]
#[ignore = "the constructor bit-movement instrument; run explicitly"]
fn constructor_bit_movement_over_the_recorded_corpus() {
    let mut moved = 0usize;
    let mut total = 0usize;
    let mut max_abs = 0.0f64;
    let mut max_rel = 0.0f64;
    let (mut res_old, mut res_new) = (0.0f64, 0.0f64);

    for ax in AXES {
        let axis = Vec3::new(ax[0], ax[1], ax[2]);
        for an in ANCHORS {
            let p = Point3::new(an[0], an[1], an[2]);
            let q = p - Point3::origin();
            let mag = q.norm();
            for angle in ANGLES {
                let r = Mat3::rotation_about(axis, angle);
                let old = q - r * q;
                let new = Mat3::identity_minus_rotation_about(axis, angle) * q;
                for (u, v) in [(old.x, new.x), (old.y, new.y), (old.z, new.z)] {
                    total += 1;
                    if u.to_bits() != v.to_bits() {
                        moved += 1;
                    }
                    max_abs = max_abs.max((u - v).abs());
                    max_rel = max_rel.max((u - v).abs() / mag);
                }
                for (t, acc) in [(old, &mut res_old), (new, &mut res_new)] {
                    let img = Affine3::from_parts(r, t).transform_point(p);
                    *acc = acc.max((img - p).norm() / mag);
                }
            }
        }
    }
    println!("corpus: {} axes x {} anchors x {} angles = {total} components",
             AXES.len(), ANCHORS.len(), ANGLES.len());
    println!("moved {moved} of {total} ({}%)", moved * 100 / total);
    println!("max |delta| {max_abs:e} m; max relative {max_rel:e}");
    println!("anchor fixed-point residual, relative: retired {res_old:e}, new {res_new:e}");
}

/// Where the `RevolvedPoint` start sample's residual 2.66e-15 actually
/// comes from, decomposed against two counterfactual respellings of
/// `Mat3::rotation_about`.
///
/// The question this answers is whether retiring `1 − cos θ` — the
/// audit member on issue 1143 — would buy the residue back. It prints
/// the diagonal entry widths and the resulting `R·p` widths for the
/// shipped form, for `t` alone respelled to the half angle, and for `t`
/// and `c` both.
#[test]
#[ignore = "the residue-decomposition instrument; run explicitly"]
fn start_sample_residue_decomposition() {
    // The `revolved_point_anchor` fixture's placed sketch point.
    let p = Point3::new(iv(2.0), iv(2.0), iv(3.0));
    let q = p - Point3::origin();
    let axis = Vec3::new(Interval::zero(), Interval::zero(), Interval::one());

    // `rotation_about` verbatim, parameterized by how `t` and `c` are
    // built — the only thing the counterfactuals change.
    let build = |angle: Interval, half_t: bool, half_c: bool| {
        let n = axis.normalize();
        let (hs, hc) = (angle * iv(0.5)).sin_cos();
        let (s_full, c_full) = angle.sin_cos();
        let t = if half_t {
            iv(2.0) * hs.powi(2)
        } else {
            Interval::one() - c_full
        };
        let c = if half_c { Interval::one() - t } else { c_full };
        let s = if half_t { iv(2.0) * hs * hc } else { s_full };
        let (x, y, z) = (n.x, n.y, n.z);
        Mat3::from_cols(
            Vec3::new(t * x.powi(2) + c, t * x * y + s * z, t * x * z - s * y),
            Vec3::new(t * x * y - s * z, t * y.powi(2) + c, t * y * z + s * x),
            Vec3::new(t * x * z + s * y, t * y * z - s * x, t * z.powi(2) + c),
        )
    };

    for (name, angle) in [
        ("theta = 0 (start sample)", Interval::zero()),
        ("theta = TAU (full period)", iv(core::f64::consts::TAU)),
    ] {
        println!("--- {name} ---");
        let (_, c) = angle.sin_cos();
        let t_full = Interval::one() - c;
        let (hs, _) = (angle * iv(0.5)).sin_cos();
        let t_half = iv(2.0) * hs.powi(2);
        println!(
            "  factors: width(cos) {:e}, width(1-cos) {:e}, width(2sin^2) {:e}",
            width(c),
            width(t_full),
            width(t_half),
        );
        let mut baseline = 0.0f64;
        for (tag, half_t, half_c) in [
            ("shipped (1-cos)", false, false),
            ("half-angle t only", true, false),
            ("half-angle t and c", true, true),
        ] {
            let m = build(angle, half_t, half_c);
            let rq = m * q;
            let w = width(rq.x).max(width(rq.y)).max(width(rq.z));
            if tag.starts_with("shipped") {
                baseline = w;
            }
            println!(
                "  {tag:20} diag widths [{:e} {:e} {:e}] -> width(R*p) {w:e} \
                 ({}% of shipped)",
                width(m.c0.x),
                width(m.c1.y),
                width(m.c2.z),
                (w / baseline * 100.0).round(),
            );
        }
        println!(
            "  the multiplier is the Z COORDINATE ({}), not |p| ({:e})",
            p.z.lo(),
            q.norm().lo(),
        );
    }
}
