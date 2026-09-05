//! **Bit identity across the retired `(kv, span)` doors.**
//!
//! Every span-restricted door in `geom-core` and `geom` lost its knot
//! vector parameter and now reads the vector through the [`Span`] it is
//! given. The claim that motivates the change is a typing claim; the
//! claim this suite pins is the arithmetic one beside it: **nothing
//! numeric moved**. Each door reads the same knots, in the same order,
//! with the same association as before.
//!
//! The corpus is literal — three curves (uniform cubic, rational cubic
//! with an interior double knot, rational quadratic) and one rational
//! bicubic-by-quadratic surface — driven through every door that took a
//! span or a window: `eval`/`deriv`/`deriv2`, `eval_in_span`,
//! `ders_in_span`, `ders1_in_span`, `deriv_in_span`, `deriv2_in_span`,
//! `basis_funs`, `ders_basis_funs`, `span_hull`, `span_hull_rational`,
//! `derivative_span_hull`, `sup_norm_bound_span`, and the surface's
//! `eval`/`ders`/`ders3` plus their three window doors. Every `f64`
//! that comes out is recorded by its bits.
//!
//! **How the expectation was obtained**, because a self-generated
//! baseline pins nothing: the same corpus was run through the OLD
//! spellings before the change and its `(label, bits)` stream captured;
//! `DIGEST` and `SPOT` below are that capture. A door that rounds
//! differently — a knot read from a different vector, a reassociated
//! sum, a `p + 1` window off by one — moves at least one of 1001
//! values and reddens the digest.
//!
//! The digest is a fold, so it names no row. `SPOT` carries sixteen
//! individual values across all four subjects for the case where the
//! digest reds and the reader wants a foothold; the row that asserts
//! them names the label that moved.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve3, NurbsSurface};
use geom_core::spline::{KnotVector, basis, hull};
use geom_core::{Point3, Vec3};

fn kv(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).expect("valid knot vector")
}

fn curve(knots: Vec<f64>, degree: usize, rational: bool) -> NurbsCurve3<f64> {
    let k = kv(knots, degree);
    let n = k.control_count();
    #[allow(clippy::cast_precision_loss)]
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            Point3::new(
                i as f64 * 0.75 - 1.0,
                ((i * i) % 11) as f64 * 0.25,
                ((i * 5) % 7) as f64 * -0.5,
            )
        })
        .collect();
    #[allow(clippy::cast_precision_loss)]
    let weights: Vec<f64> = (0..n)
        .map(|i| {
            if rational {
                0.5 + (i % 5) as f64 * 0.375
            } else {
                1.0
            }
        })
        .collect();
    NurbsCurve3::new(k, control, weights).expect("valid curve")
}

fn surface() -> NurbsSurface<f64> {
    let ku = kv(vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0, 3.0], 3);
    let kvv = kv(vec![0.0, 0.0, 0.0, 0.5, 1.5, 2.0, 2.0, 2.0], 2);
    let (nu, nv) = (ku.control_count(), kvv.control_count());
    #[allow(clippy::cast_precision_loss)]
    let control: Vec<Point3<f64>> = (0..nu * nv)
        .map(|f| {
            let (iu, iv) = (f / nv, f % nv);
            Point3::new(
                iu as f64 * 1.25,
                iv as f64 * 0.875,
                (((iu * 3 + iv * 5) % 9) as f64) * 0.5 - 1.0,
            )
        })
        .collect();
    #[allow(clippy::cast_precision_loss)]
    let weights: Vec<f64> = (0..nu * nv)
        .map(|f| 0.5 + ((f % 6) as f64) * 0.25)
        .collect();
    NurbsSurface::new(ku, kvv, control, weights).expect("valid surface")
}

fn push_p(out: &mut Vec<(String, u64)>, tag: &str, p: Point3<f64>) {
    out.push((format!("{tag}.x"), p.x.to_bits()));
    out.push((format!("{tag}.y"), p.y.to_bits()));
    out.push((format!("{tag}.z"), p.z.to_bits()));
}

fn push_v(out: &mut Vec<(String, u64)>, tag: &str, v: Vec3<f64>) {
    out.push((format!("{tag}.x"), v.x.to_bits()));
    out.push((format!("{tag}.y"), v.y.to_bits()));
    out.push((format!("{tag}.z"), v.z.to_bits()));
}

/// The corpus, as `(label, f64 bits)` in a fixed order.
fn rows() -> Vec<(String, u64)> {
    let mut out = Vec::new();

    // ---- curves: whole-domain doors and the span doors ----
    for (name, rational, knots, degree) in [
        (
            "uni3",
            false,
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0, 3.0],
            3,
        ),
        (
            "rat3",
            true,
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            3,
        ),
        (
            "quad",
            true,
            vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0],
            2,
        ),
    ] {
        let c = curve(knots, degree, rational);
        let k = c.knots().clone();
        let (lo, hi) = k.domain();
        for step in 0..5 {
            #[allow(clippy::cast_precision_loss)]
            let t = lo + (hi - lo) * (step as f64) / 4.0;
            push_p(&mut out, &format!("{name}.eval@{step}"), c.eval(t));
            push_v(&mut out, &format!("{name}.deriv@{step}"), c.deriv(t));
            push_v(&mut out, &format!("{name}.deriv2@{step}"), c.deriv2(t));
        }
        for index in k.first_span()..=k.last_span() {
            let Some(win) = c.span(index) else { continue };
            let span = win.span();
            let t = 0.5 * (k.knots()[index] + k.knots()[index + 1]);
            push_p(
                &mut out,
                &format!("{name}.eval_in_span@{index}"),
                win.eval_in_span(t),
            );
            let (p, d, dd) = win.ders_in_span(t);
            push_p(&mut out, &format!("{name}.ders.p@{index}"), p);
            push_v(&mut out, &format!("{name}.ders.d@{index}"), d);
            push_v(&mut out, &format!("{name}.ders.dd@{index}"), dd);
            let (p1, d1) = win.ders1_in_span(t);
            push_p(&mut out, &format!("{name}.ders1.p@{index}"), p1);
            push_v(&mut out, &format!("{name}.ders1.d@{index}"), d1);
            push_v(
                &mut out,
                &format!("{name}.deriv_in_span@{index}"),
                win.deriv_in_span(t),
            );
            push_v(
                &mut out,
                &format!("{name}.deriv2_in_span@{index}"),
                win.deriv2_in_span(t),
            );

            // Basis rows and hull bounds, read through the same span.
            let row = basis::basis_funs::<f64>(span, t);
            for (j, n) in row.iter().enumerate() {
                out.push((format!("{name}.basis@{index}.{j}"), n.to_bits()));
            }
            let ders = basis::ders_basis_funs::<f64>(span, t, 2);
            for (o, r) in ders.iter().enumerate() {
                for (j, n) in r.iter().enumerate() {
                    out.push((format!("{name}.ders_basis@{index}.{o}.{j}"), n.to_bits()));
                }
            }
            #[allow(clippy::cast_precision_loss)]
            let coeffs: Vec<f64> = (0..k.control_count())
                .map(|i| (i % 5) as f64 * 0.5 - 1.0)
                .collect();
            let w: Vec<f64> = c.weights().to_vec();
            let h = hull::span_hull(&coeffs, span);
            out.push((format!("{name}.span_hull.lo@{index}"), h.lo().to_bits()));
            out.push((format!("{name}.span_hull.hi@{index}"), h.hi().to_bits()));
            let hr = hull::span_hull_rational(&coeffs, &w, span);
            out.push((
                format!("{name}.span_hull_rat.lo@{index}"),
                hr.lo().to_bits(),
            ));
            out.push((
                format!("{name}.span_hull_rat.hi@{index}"),
                hr.hi().to_bits(),
            ));
            let hd = hull::derivative_span_hull(&coeffs, span);
            out.push((
                format!("{name}.deriv_span_hull.lo@{index}"),
                hd.lo().to_bits(),
            ));
            out.push((
                format!("{name}.deriv_span_hull.hi@{index}"),
                hd.hi().to_bits(),
            ));
            out.push((
                format!("{name}.sup_norm@{index}"),
                hull::sup_norm_bound_span(&coeffs, span).to_bits(),
            ));
        }
    }

    // ---- surface ----
    let s = surface();
    let (ulo, uhi) = s.knots_u().domain();
    let (vlo, vhi) = s.knots_v().domain();
    for su in 0..3 {
        for sv in 0..3 {
            #[allow(clippy::cast_precision_loss)]
            let u = ulo + (uhi - ulo) * (su as f64) / 2.0;
            #[allow(clippy::cast_precision_loss)]
            let v = vlo + (vhi - vlo) * (sv as f64) / 2.0;
            push_p(&mut out, &format!("surf.eval@{su}{sv}"), s.eval(u, v));
            let j = s.ders(u, v);
            push_p(&mut out, &format!("surf.ders.p@{su}{sv}"), j.point);
            push_v(&mut out, &format!("surf.ders.du@{su}{sv}"), j.du);
            push_v(&mut out, &format!("surf.ders.dv@{su}{sv}"), j.dv);
            push_v(&mut out, &format!("surf.ders.duu@{su}{sv}"), j.duu);
            push_v(&mut out, &format!("surf.ders.duv@{su}{sv}"), j.duv);
            push_v(&mut out, &format!("surf.ders.dvv@{su}{sv}"), j.dvv);
            let j3 = s.ders3(u, v);
            push_v(&mut out, &format!("surf.ders3.duuu@{su}{sv}"), j3.duuu);
            push_v(&mut out, &format!("surf.ders3.duuv@{su}{sv}"), j3.duuv);
            push_v(&mut out, &format!("surf.ders3.duvv@{su}{sv}"), j3.duvv);
            push_v(&mut out, &format!("surf.ders3.dvvv@{su}{sv}"), j3.dvvv);
            let win = s.window_at(u, v);
            push_p(
                &mut out,
                &format!("surf.eval_in_span@{su}{sv}"),
                win.eval_in_span(u, v),
            );
            let jw = win.ders_in_span(u, v);
            push_p(&mut out, &format!("surf.jw.p@{su}{sv}"), jw.point);
            push_v(&mut out, &format!("surf.jw.du@{su}{sv}"), jw.du);
            push_v(&mut out, &format!("surf.jw.dvv@{su}{sv}"), jw.dvv);
            let j3w = win.ders3_in_span(u, v);
            push_v(&mut out, &format!("surf.j3w.duuu@{su}{sv}"), j3w.duuu);
            push_v(&mut out, &format!("surf.j3w.dvvv@{su}{sv}"), j3w.dvvv);
        }
    }
    out
}

/// The number of values the corpus produces. Pinned so that a corpus
/// that silently stops covering a door cannot leave the digest green
/// by producing a shorter stream.
const ROW_COUNT: usize = 1001;

/// FNV-1a 64 over `"{label} {bits:#018x}\n"` for every row in order,
/// captured from the retired spellings before the change.
const DIGEST: u64 = 0x9214_4852_a6d1_ba8a;

/// Individual values from the same capture — one foothold per subject
/// and per door family, so a red digest has a named row beside it.
const SPOT: &[(&str, u64)] = &[
    ("uni3.eval@2.x", 0x3fec_0000_0000_0000),
    ("uni3.basis@3.0", 0x3fc0_0000_0000_0000),
    ("uni3.span_hull.lo@3", 0xbff0_0000_0000_0000),
    ("uni3.ders.d@3.y", 0x3ff0_c000_0000_0000),
    ("rat3.eval@2.y", 0x3ffb_4de9_bd37_a6f5),
    ("rat3.sup_norm@3", 0x3ff0_0000_0000_0000),
    ("rat3.ders_basis@3.1.1", 0xbfe8_0000_0000_0000),
    ("rat3.deriv2_in_span@3.z", 0x4015_d3c5_6e75_8fe3),
    ("quad.deriv_span_hull.hi@2", 0x4010_0000_0000_0004),
    ("quad.span_hull_rat.hi@3", 0x3fe0_0000_0000_0000),
    ("quad.ders1.p@2.x", 0xbfcc_609a_90e7_d95c),
    ("surf.eval@11.z", 0x3fda_2f68_4bda_12f8),
    ("surf.ders.duv@11.x", 0x3fda_f1c7_1c71_c724),
    ("surf.ders3.duvv@22.y", 0xc04c_a72f_0539_782b),
    ("surf.j3w.duuu@22.z", 0x402d_55d4_b61c_5c8e),
    ("surf.jw.dvv@11.z", 0x3ff5_6c69_3ee9_25f0),
];

fn digest(rows: &[(String, u64)]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (name, bits) in rows {
        for byte in format!("{name} {bits:#018x}\n").bytes() {
            h ^= u64::from(byte);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

#[test]
fn every_span_door_is_bit_identical_to_its_retired_spelling() {
    let rows = rows();
    assert_eq!(
        rows.len(),
        ROW_COUNT,
        "the corpus changed shape — the digest below is a claim about a \
         DIFFERENT stream and proves nothing until it is re-captured"
    );
    let by_name: std::collections::BTreeMap<&str, u64> =
        rows.iter().map(|(n, b)| (n.as_str(), *b)).collect();
    for (name, want) in SPOT {
        let got = by_name
            .get(name)
            .unwrap_or_else(|| panic!("the corpus no longer produces `{name}`"));
        assert_eq!(
            *got, *want,
            "`{name}` moved: {got:#018x} against the retired spelling's {want:#018x}"
        );
    }
    assert_eq!(
        digest(&rows),
        DIGEST,
        "some value in the corpus moved; SPOT above names the ones with a \
         literal beside them, and the corpus prints under `--nocapture`"
    );
}
