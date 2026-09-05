//! **Bit identity across the coefficient-hull doors.**
//!
//! The coefficient array a `hull` door bounds travels with the knot
//! vector it was fitted against, and a span of that vector is taken
//! from the pair. That is a typing claim; the claim this suite pins is
//! the arithmetic one beside it: **nothing numeric moved**. Every door
//! reads the same coefficients and the same knots, in the same order,
//! with the same association it had when the coefficients were a loose
//! slice beside the span.
//!
//! The corpus is literal — six clamped vectors at degrees 1 to 4, with
//! interior multiplicities up to the degree (empty spans the walk must
//! skip, a C⁰ kink), each driven non-rationally (unit weights) and
//! rationally, through every door: per span, `hull`, `hull_rational`,
//! `derivative_hull` and `sup_norm_bound`; over the whole domain,
//! `domain_hull`, `domain_hull_rational`, every `derivative_coeffs`
//! entry, `derivative_domain_hull`, `sup_norm_bound` and
//! `sup_norm_bound_rational`. Three coefficient lanes: `f64` brackets,
//! `RingInterval` brackets (what the consumers actually hand in), and —
//! under the `interval` feature — `Interval` brackets. Every `f64`
//! that comes out is recorded by its bits.
//!
//! **How the expectation was obtained**, because a self-generated
//! baseline pins nothing: the same corpus was run through the free
//! `(coeffs, span)` and `(kv, coeffs)` spellings at the merge base and
//! its `(label, bits)` stream captured; `DIGEST` and `SPOT` below are
//! that capture, one pair per lane. A door that rounds differently — a
//! reassociated fold, a window off by one, a knot read from the wrong
//! end — moves at least one value and reddens the digest.
//!
//! The digest is a fold, so it names no row. `SPOT` carries individual
//! values across the doors for the case where the digest reds and the
//! reader wants a foothold; the row that asserts them names the label
//! that moved.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::{KnotVector, hull};
use geom_core::{CertifiedEnclosure, RingInterval};

type Rows = Vec<(String, u64)>;

/// The six vectors: degrees 1–4, interior multiplicities from 1 up to
/// the degree.
fn vectors() -> Vec<(&'static str, KnotVector)> {
    let kv = |k: &[f64], p: usize| KnotVector::clamped(k.to_vec(), p).expect("valid");
    vec![
        ("d1", kv(&[0.0, 0.0, 0.5, 1.5, 2.0, 2.0], 1)),
        ("d2m2", kv(&[0.0, 0.0, 0.0, 0.5, 0.5, 1.25, 3.0, 3.0, 3.0], 2)),
        ("d3", kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0], 3)),
        (
            "d3m2",
            kv(&[0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0], 3),
        ),
        (
            "d3m3",
            kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0], 3),
        ),
        (
            "d4",
            kv(
                &[0.0, 0.0, 0.0, 0.0, 0.0, 0.4, 0.9, 0.9, 1.0, 1.0, 1.0, 1.0, 1.0],
                4,
            ),
        ),
    ]
}

/// The `f64` coefficient values: distinct, signed, not on a grid.
#[allow(clippy::cast_precision_loss)]
fn values(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| ((i * 7) % 11) as f64 * 0.375 - 1.5 + i as f64 * 0.013)
        .collect()
}

/// Weights: unit for the non-rational pass, varied and positive for the
/// rational pass.
#[allow(clippy::cast_precision_loss)]
fn weights(n: usize, rational: bool) -> Vec<f64> {
    (0..n)
        .map(|i| {
            if rational {
                0.5 + (i % 4) as f64 * 0.375
            } else {
                1.0
            }
        })
        .collect()
}

fn ri(o: &mut Rows, tag: &str, r: RingInterval) {
    o.push((format!("{tag}.lo"), r.lo().to_bits()));
    o.push((format!("{tag}.hi"), r.hi().to_bits()));
}

/// Every door, for one vector, one coefficient array and one weight
/// vector. The doors' spellings are the only thing that changes when
/// the coefficients start carrying their vector; the labels do not.
fn drive<E: CertifiedEnclosure>(o: &mut Rows, name: &str, kv: &KnotVector, coeffs: &[E], w: &[f64]) {
    for index in kv.first_span()..=kv.last_span() {
        let Some(span) = kv.span(index) else { continue };
        ri(o, &format!("{name}.hull@{index}"), hull::span_hull(coeffs, span));
        ri(
            o,
            &format!("{name}.hull_rat@{index}"),
            hull::span_hull_rational(coeffs, w, span),
        );
        ri(
            o,
            &format!("{name}.dhull@{index}"),
            hull::derivative_span_hull(coeffs, span),
        );
        o.push((
            format!("{name}.sup@{index}"),
            hull::sup_norm_bound_span(coeffs, span).to_bits(),
        ));
    }
    ri(o, &format!("{name}.domain"), hull::domain_hull(kv, coeffs));
    ri(
        o,
        &format!("{name}.domain_rat"),
        hull::domain_hull_rational(kv, coeffs, w),
    );
    for (i, q) in hull::derivative_coeffs(kv, coeffs).iter().enumerate() {
        ri(o, &format!("{name}.dcoeff.{i}"), *q);
    }
    ri(
        o,
        &format!("{name}.ddomain"),
        hull::derivative_domain_hull(kv, coeffs),
    );
    o.push((
        format!("{name}.sup_domain"),
        hull::sup_norm_bound(kv, coeffs).to_bits(),
    ));
    o.push((
        format!("{name}.sup_domain_rat"),
        hull::sup_norm_bound_rational(kv, coeffs, w).to_bits(),
    ));
}

/// The default-lane corpus: `f64` and `RingInterval` brackets.
fn rows() -> Rows {
    let mut o = Vec::new();
    for (vname, kv) in vectors() {
        let n = kv.control_count();
        let c = values(n);
        #[allow(clippy::cast_precision_loss)]
        let rings: Vec<RingInterval> = c
            .iter()
            .enumerate()
            .map(|(i, x)| RingInterval::from_bounds(x - 0.01 * i as f64, x + 0.005))
            .collect();
        for rational in [false, true] {
            let w = weights(n, rational);
            let tag = if rational { "rat" } else { "nr" };
            drive(&mut o, &format!("{vname}.{tag}.f64"), &kv, &c, &w);
            drive(&mut o, &format!("{vname}.{tag}.ring"), &kv, &rings, &w);
        }
    }
    o
}

/// The `Interval`-lane corpus: `Interval` brackets of the same values.
#[cfg(feature = "interval")]
fn rows_interval() -> Rows {
    use geom_core::Interval;
    let mut o = Vec::new();
    for (vname, kv) in vectors() {
        let n = kv.control_count();
        #[allow(clippy::cast_precision_loss)]
        let c: Vec<Interval> = values(n)
            .iter()
            .enumerate()
            .map(|(i, x)| Interval::from_bounds(x - 0.02, x + 0.01 * i as f64))
            .collect();
        for rational in [false, true] {
            let w = weights(n, rational);
            let tag = if rational { "rat" } else { "nr" };
            drive(&mut o, &format!("{vname}.{tag}.interval"), &kv, &c, &w);
        }
    }
    o
}

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

fn check(rows: &[(String, u64)], row_count: usize, want: u64, spot: &[(&str, u64)]) {
    if std::env::var_os("COEFFS_DUMP").is_some() {
        for (name, bits) in rows {
            println!("{name} {bits:#018x}");
        }
        println!("rows {} digest {:#018x}", rows.len(), digest(rows));
    }
    assert_eq!(
        rows.len(),
        row_count,
        "the corpus changed shape — the digest is a claim about a DIFFERENT \
         stream and proves nothing until it is re-captured"
    );
    let by_name: std::collections::BTreeMap<&str, u64> =
        rows.iter().map(|(n, b)| (n.as_str(), *b)).collect();
    for (name, bits) in spot {
        let got = by_name
            .get(name)
            .unwrap_or_else(|| panic!("the corpus no longer produces `{name}`"));
        assert_eq!(
            *got, *bits,
            "`{name}` moved: {got:#018x} against the retired spelling's {bits:#018x}"
        );
    }
    assert_eq!(
        digest(rows),
        want,
        "some value in the corpus moved; SPOT names the ones with a literal \
         beside them, and the corpus prints under COEFFS_DUMP=1 --nocapture"
    );
}

/// The number of values the default-lane corpus produces, pinned so a
/// corpus that silently stops covering a door cannot leave the digest
/// green by producing a shorter stream.
const ROW_COUNT: usize = 960;

/// FNV-1a 64 over `"{label} {bits:#018x}\n"` for every row in order,
/// captured through the retired spellings at the merge base.
const DIGEST: u64 = 0xdedc_bc91_037f_daab;

/// Individual values from the same capture — one foothold per door
/// family and per lane.
const SPOT: &[(&str, u64)] = &[
    ("d1.nr.f64.hull@1.lo", 0xbff8_0000_0000_0000),
    ("d1.rat.ring.hull_rat@2.lo", 0xbfd7_9db2_2d0e_5604),
    ("d2m2.nr.f64.sup_domain", 0x4002_4fdf_3b64_5a1d),
    ("d2m2.rat.f64.hull_rat@4.hi", 0x4002_4fdf_3b64_5a1d),
    ("d3.nr.ring.dhull@5.lo", 0xc002_8106_24dd_2f1f),
    ("d3.rat.ring.ddomain.hi", 0x4020_2872_b020_c49f),
    ("d3m2.rat.f64.sup@6", 0x4002_4fdf_3b64_5a1d),
    ("d3m3.nr.f64.domain.hi", 0x4002_4fdf_3b64_5a1d),
    ("d3m3.rat.f64.sup_domain_rat", 0x4002_4fdf_3b64_5a1d),
    ("d4.nr.f64.hull@7.hi", 0x4002_4fdf_3b64_5a1d),
    ("d4.nr.f64.dcoeff.3.hi", 0xc017_cac0_8312_6e94),
    ("d4.rat.ring.domain_rat.lo", 0xbff8_0000_0000_0000),
];

#[test]
fn every_coefficient_door_is_bit_identical_to_its_retired_spelling() {
    check(&rows(), ROW_COUNT, DIGEST, SPOT);
}

#[cfg(feature = "interval")]
const ROW_COUNT_INTERVAL: usize = 480;
#[cfg(feature = "interval")]
const DIGEST_INTERVAL: u64 = 0x077f_2c93_d2ac_b9b5;
#[cfg(feature = "interval")]
const SPOT_INTERVAL: &[(&str, u64)] = &[
    ("d1.nr.interval.hull@1.lo", 0xbff8_51eb_851e_b852),
    ("d2m2.nr.interval.sup_domain", 0x4002_8d4f_df3b_645a),
    ("d2m2.rat.interval.hull_rat@4.hi", 0x4002_8d4f_df3b_645a),
    ("d3.nr.interval.dhull@5.lo", 0xc002_9062_4dd2_f1ae),
    ("d3.rat.interval.ddomain.hi", 0x4020_4ed9_1687_2b06),
    ("d3m2.rat.interval.sup@6", 0x4002_8d4f_df3b_645a),
    ("d3m3.nr.interval.domain.hi", 0x4002_8d4f_df3b_645a),
    ("d3m3.rat.interval.sup_domain_rat", 0x4002_8d4f_df3b_645a),
    ("d4.nr.interval.dcoeff.3.hi", 0xc016_d4fd_f3b6_459e),
    ("d4.rat.interval.domain_rat.lo", 0xbff8_51eb_851e_b852),
];

#[cfg(feature = "interval")]
#[test]
fn every_coefficient_door_is_bit_identical_on_interval_brackets() {
    check(
        &rows_interval(),
        ROW_COUNT_INTERVAL,
        DIGEST_INTERVAL,
        SPOT_INTERVAL,
    );
}
