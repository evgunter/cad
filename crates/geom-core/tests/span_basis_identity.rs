//! `basis_funs` / `ders_basis_funs` over a [`Span`] are **bit-for-bit**
//! what they were over a raw span index.
//!
//! The fold that gave these functions a validated `Span` deleted their
//! three-way range guard and their all-poison return. That is a typing
//! change, and D9 makes "the numbers moved" a bug rather than a
//! re-baseline — so the rows below are the literal `f64` bit patterns
//! `main` produced, captured before the change and re-checked after.
//!
//! Any drift in operand form, association order, or the shift-and-add's
//! `n[j] = saved` (which is where the sign of a zero lives) shows up
//! here as a bit mismatch, not as a tolerance question. The spread is
//! five knot vectors — uniform cubic, a cubic and a quadratic with an
//! **interior multiplicity 2** (each carrying an empty span the walk
//! must skip), degree 1, and degree 5 — sampled at each nonempty span's
//! low knot, midpoint, and high knot. The high knot is outside the
//! half-open span, so a third of the rows exercise the documented
//! polynomial extension too.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::KnotVector;
use geom_core::spline::basis::{basis_funs, ders_basis_funs};

fn kv(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).expect("valid knot vector")
}

fn vectors() -> Vec<(&'static str, KnotVector)> {
    vec![
        (
            "uniform cubic",
            kv(
                vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
                3,
            ),
        ),
        (
            "cubic, interior multiplicity 2",
            kv(
                vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
                3,
            ),
        ),
        ("degree 1", kv(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1)),
        (
            "degree 5",
            kv(
                vec![0.0; 6]
                    .into_iter()
                    .chain([0.4, 0.9])
                    .chain(vec![1.0; 6])
                    .collect(),
                5,
            ),
        ),
        (
            "quadratic, interior multiplicity 2, nonuniform",
            kv(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.25, 3.0, 3.0, 3.0], 2),
        ),
    ]
}

/// Captured from `main` at this branch's point, before the fold, by the
/// exact walk `golden_rows_match_the_pre_fold_capture` performs below:
/// per vector, per **nonempty** span, per `t` in
/// `[u_s, midpoint, u_{s+1}]`, the `basis_funs` row followed by
/// `ders_basis_funs(.., 2)`'s first-derivative row.
const GOLDEN: &[&[u64]] = &[
    // uniform cubic: degree 3
    &[0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000],
    &[0xc008000000000000, 0x4008000000000000, 0x0000000000000000, 0x0000000000000000],
    &[0x3fc0000000000000, 0x3fe3000000000000, 0x3fd0aaaaaaaaaaaa, 0x3f95555555555555],
    &[0xbfe8000000000000, 0xbfc8000000000000, 0x3fea000000000000, 0x3fc0000000000000],
    &[0x0000000000000000, 0x3fd0000000000000, 0x3fe2aaaaaaaaaaaa, 0x3fc5555555555555],
    &[0x0000000000000000, 0xbfe8000000000000, 0x3fd0000000000000, 0x3fe0000000000000],
    &[0x3fd0000000000000, 0x3fe2aaaaaaaaaaaa, 0x3fc5555555555555, 0x0000000000000000],
    &[0xbfe8000000000000, 0x3fd0000000000000, 0x3fe0000000000000, 0x0000000000000000],
    &[0x3fa0000000000000, 0x3fde000000000000, 0x3fdeaaaaaaaaaaaa, 0x3f95555555555555],
    &[0xbfc8000000000000, 0xbfe2000000000000, 0x3fe4000000000000, 0x3fc0000000000000],
    &[0x0000000000000000, 0x3fc5555555555555, 0x3fe5555555555555, 0x3fc5555555555555],
    &[0x0000000000000000, 0xbfe0000000000000, 0x0000000000000000, 0x3fe0000000000000],
    &[0x3fc5555555555555, 0x3fe5555555555555, 0x3fc5555555555555, 0x0000000000000000],
    &[0xbfe0000000000000, 0x0000000000000000, 0x3fe0000000000000, 0x0000000000000000],
    &[0x3f95555555555555, 0x3fdeaaaaaaaaaaaa, 0x3fde000000000000, 0x3fa0000000000000],
    &[0xbfc0000000000000, 0xbfe4000000000000, 0x3fe2000000000000, 0x3fc8000000000000],
    &[0x0000000000000000, 0x3fc5555555555555, 0x3fe2aaaaaaaaaaaa, 0x3fd0000000000000],
    &[0x0000000000000000, 0xbfe0000000000000, 0xbfd0000000000000, 0x3fe8000000000000],
    &[0x3fc5555555555555, 0x3fe2aaaaaaaaaaaa, 0x3fd0000000000000, 0x0000000000000000],
    &[0xbfe0000000000000, 0xbfd0000000000000, 0x3fe8000000000000, 0x0000000000000000],
    &[0x3f95555555555555, 0x3fd0aaaaaaaaaaaa, 0x3fe3000000000000, 0x3fc0000000000000],
    &[0xbfc0000000000000, 0xbfea000000000000, 0x3fc8000000000000, 0x3fe8000000000000],
    &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    &[0x0000000000000000, 0x0000000000000000, 0xc008000000000000, 0x4008000000000000],
    // cubic, interior multiplicity 2: degree 3
    &[0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000],
    &[0xc024000000000000, 0x4024000000000000, 0x0000000000000000, 0x0000000000000000],
    &[0x3fc0000000000000, 0x3fd8000000000000, 0x3fde969696969697, 0x3f96969696969697],
    &[0xc004000000000000, 0xc004000000000000, 0x40123c3c3c3c3c3d, 0x3fdc3c3c3c3c3c3c],
    &[0x0000000000000000, 0x0000000000000000, 0x3fea5a5a5a5a5a5a, 0x3fc6969696969697],
    &[0x0000000000000000, 0x0000000000000000, 0xbffc3c3c3c3c3c3c, 0x3ffc3c3c3c3c3c3c],
    &[0x3fea5a5a5a5a5a5a, 0x3fc6969696969697, 0x0000000000000000, 0x0000000000000000],
    &[0xbffc3c3c3c3c3c3c, 0x3ffc3c3c3c3c3c3c, 0x0000000000000000, 0x0000000000000000],
    &[0x3fba5a5a5a5a5a5a, 0x3fdde56cf47c038c, 0x3fd61707f8e9dacc, 0x3fb5b3d1f00e2c4a],
    &[0xbfdc3c3c3c3c3c3c, 0xbfe09be7327dc916, 0x3fe319b046dd740d, 0x3fd740aa137ce650],
    &[0x0000000000000000, 0x3f9fe3a76b2ef2ba, 0x3fd29a21a930b840, 0x3fe5b3d1f00e2c4a],
    &[0x0000000000000000, 0xbfd3ee48a2fd57b3, 0xbff24517eabd9062, 0x3ff740aa137ce650],
    &[0x3f9fe3a76b2ef2ba, 0x3fd29a21a930b840, 0x3fe5b3d1f00e2c4a, 0x0000000000000000],
    &[0xbfd3ee48a2fd57b3, 0xbff24517eabd9062, 0x3ff740aa137ce650, 0x0000000000000000],
    &[0x3f6fe3a76b2ef2a5, 0x3fba3e01c5894d08, 0x3fe8985c1fe3a76a, 0x3fc000000000000a],
    &[0xbfb3ee48a2fd57ab, 0xbff2af6418cd8236, 0xbff411b75d02a861, 0x4004000000000008],
    &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    &[0x0000000000000000, 0x0000000000000000, 0xc024000000000000, 0x4024000000000000],
    // degree 1: degree 1
    &[0x3ff0000000000000, 0x0000000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    &[0x3fe0000000000000, 0x3fe0000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    &[0x0000000000000000, 0x3ff0000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    &[0x3ff0000000000000, 0x0000000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    &[0x3fe0000000000000, 0x3fe0000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    &[0x0000000000000000, 0x3ff0000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    &[0x3ff0000000000000, 0x0000000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    &[0x3fe0000000000000, 0x3fe0000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    &[0x0000000000000000, 0x3ff0000000000000],
    &[0xbff0000000000000, 0x3ff0000000000000],
    // degree 5: degree 5
    &[0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000],
    &[0xc029000000000000, 0x4029000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000],
    &[0x3fa0000000000000, 0x3fdd306c00c7c641, 0x3fd84ceee1ab1677, 0x3fbda11864b572ea, 0x3f90bcebf327df70, 0x3f4d208a5a912e34],
    &[0xbfe9000000000000, 0xc00206a992fb834c, 0x3ff6b872aed97525, 0x3ff4b72b0d8e14ee, 0x3fd30abee4d1db58, 0x3f96c16c16c16c18],
    &[0x0000000000000000, 0x3fb862f365a49e69, 0x3fd5fa497dcbe081, 0x3fd7628533939238, 0x3fc570d79f1ca590, 0x3f9d208a5a912e34],
    &[0x0000000000000000, 0xbfee7bb03f0dc604, 0xbff4625b9187eeb3, 0x3fe550d6a1f52f1b, 0x3ff3476d5a63df22, 0x3fd6c16c16c16c18],
    &[0x3fb862f365a49e69, 0x3fd5fa497dcbe081, 0x3fd7628533939238, 0x3fc570d79f1ca590, 0x3f9d208a5a912e34, 0x0000000000000000],
    &[0xbfee7bb03f0dc604, 0xbff4625b9187eeb3, 0x3fe550d6a1f52f1b, 0x3ff3476d5a63df22, 0x3fd6c16c16c16c18, 0x0000000000000000],
    &[0x3f6862f365a49e69, 0x3fad93bc03f8c59c, 0x3fce066d8081d03c, 0x3fd9eb4e1a24c34c, 0x3fd23753ddeacfe4, 0x3f8edd3c0ca45881],
    &[0xbfae7bb03f0dc604, 0xbfe4f6f61cdba881, 0xbff4f1eb120cc84f, 0xbf6bcff5e2ec6800, 0x3ffb9c9a3b6ad320, 0x3fd34a4587e6b751],
    &[0x0000000000000000, 0x3f25d867c3ece29f, 0x3f781ef293003a3d, 0x3fb3bb206384835e, 0x3fdbd0c503360824, 0x3fdedd3c0ca45881],
    &[0x0000000000000000, 0xbf8111111111110e, 0xbfcb05b05b05b059, 0xbffa8c536fe1a8c6, 0xc0078cf5411b2e27, 0x40134a4587e6b751],
    &[0x3f25d867c3ece29f, 0x3f781ef293003a3d, 0x3fb3bb206384835e, 0x3fdbd0c503360824, 0x3fdedd3c0ca45881, 0x0000000000000000],
    &[0xbf8111111111110e, 0xbfcb05b05b05b059, 0xbffa8c536fe1a8c6, 0xc0078cf5411b2e27, 0x40134a4587e6b751, 0x0000000000000000],
    &[0x3ed5d867c3ece2c2, 0x3f3d208a5a912e4f, 0x3f8da4d379972c4d, 0x3fcb79bac5c56456, 0x3fe7a74f0329161e, 0x3f9fffffffffffce],
    &[0xbf41111111111124, 0xbfa16c16c16c16ce, 0xbfe83518a6dfc35a, 0xc016941bcff5e2eb, 0x400a7c3f35ba7839, 0x4008ffffffffffe1],
    &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xc049000000000001, 0x4049000000000001],
    // quadratic, interior multiplicity 2, nonuniform: degree 2
    &[0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
    &[0xc010000000000000, 0x4010000000000000, 0x0000000000000000],
    &[0x3fd0000000000000, 0x3fe0000000000000, 0x3fd0000000000000],
    &[0xc000000000000000, 0x0000000000000000, 0x4000000000000000],
    &[0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    &[0x0000000000000000, 0xc010000000000000, 0x4010000000000000],
    &[0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
    &[0xc005555555555555, 0x4005555555555555, 0x0000000000000000],
    &[0x3fd0000000000000, 0x3fe599999999999a, 0x3fb3333333333334],
    &[0xbff5555555555555, 0x3feddddddddddddd, 0x3fd999999999999a],
    &[0x0000000000000000, 0x3fe6666666666667, 0x3fd3333333333334],
    &[0x0000000000000000, 0xbfe999999999999a, 0x3fe999999999999a],
    &[0x3fe6666666666667, 0x3fd3333333333334, 0x0000000000000000],
    &[0xbfe999999999999a, 0x3fe999999999999a, 0x0000000000000000],
    &[0x3fc6666666666667, 0x3fe2666666666666, 0x3fd0000000000000],
    &[0xbfd999999999999a, 0xbfc5f15f15f15f14, 0x3fe2492492492492],
    &[0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    &[0x0000000000000000, 0xbff2492492492492, 0x3ff2492492492492],
];

#[test]
fn golden_rows_match_the_pre_fold_capture() {
    let mut n = 0usize;
    let mut next = |row: &[f64], what: &str, name: &str, index: usize, t: f64| {
        let want = GOLDEN.get(n).unwrap_or_else(|| {
            panic!("golden table is short: row {n} ({what}, {name}, span {index}, t {t})")
        });
        assert_eq!(
            row.len(),
            want.len(),
            "row {n} ({what}, {name}, span {index}, t {t}) changed LENGTH"
        );
        for (j, (got, expect)) in row.iter().zip(*want).enumerate() {
            assert_eq!(
                got.to_bits(),
                *expect,
                "row {n} ({what}, {name}, span {index}, t {t}), entry {j}: \
                 got {got} (0x{:016x}), pre-fold 0x{expect:016x}",
                got.to_bits()
            );
        }
        n += 1;
    };
    for (name, k) in vectors() {
        for index in k.first_span()..=k.last_span() {
            let Some(span) = k.span(index) else { continue };
            let (a, b) = (k.knots()[index], k.knots()[index + 1]);
            for t in [a, 0.5 * (a + b), b] {
                next(&basis_funs::<f64>(&k, span, t), "basis", name, index, t);
                next(
                    &ders_basis_funs::<f64>(&k, span, t, 2)[1],
                    "ders[1]",
                    name,
                    index,
                    t,
                );
            }
        }
    }
    assert_eq!(n, GOLDEN.len(), "golden table has unconsumed rows");
}
