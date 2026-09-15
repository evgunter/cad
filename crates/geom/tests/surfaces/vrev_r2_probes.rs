//! **VREV R2 review probes.** Not shipped rows — a reviewer's
//! instrument for PR 2627's `NurbsSurface::reversed_v` / `reversed_u`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{KnotMirrorError, NurbsSurface};
use geom_core::spline::basis::basis_funs;
use geom_core::{KnotVector, Point3};

fn net(n: usize) -> Vec<Point3<f64>> {
    (0..n)
        .map(|i| {
            let f = i as f64;
            Point3::new(f * 0.7 + 0.3, (f * 1.31).sin() * 2.0, (f * 0.77).cos() * 3.0)
        })
        .collect()
}
fn wts(n: usize) -> Vec<f64> {
    (0..n).map(|i| 0.25 + ((i * 7) % 13) as f64 * 0.31).collect()
}

fn sym_surface() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.75, 1.0, 1.0, 1.0], 2).unwrap();
    NurbsSurface::new(ku, kv, net(15), wts(15)).unwrap()
}

/// P1. Is the ulp band really only the SUMMATION ORDER? If the mirrored
/// basis values are already not bit-identical, then "evaluate both sides
/// in the same order" cannot buy bit-exactness.
#[test]
fn p1_mirrored_basis_values_are_not_bit_identical() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.75, 1.0, 1.0, 1.0], 2).unwrap();
    let mut differing = 0usize;
    let mut checked = 0usize;
    let mut slots = 0usize;
    for i in 0..=64u32 {
        let v = f64::from(i) / 64.0;
        let a: Vec<f64> = basis_funs(kv.span_at(v), v);
        let b: Vec<f64> = basis_funs(kv.span_at(1.0 - v), 1.0 - v);
        if a.len() != b.len() {
            continue;
        }
        checked += 1;
        for (x, y) in a.iter().zip(b.iter().rev()) {
            slots += 1;
            if x.to_bits() != y.to_bits() {
                differing += 1;
            }
        }
    }
    println!("P1: windows compared {checked}, slots {slots}, bit mismatches {differing}");
    assert!(checked > 0, "P1 compared something");
}

/// P2. How wide is the point-set ulp band really? The shipped row pins
/// `<= 4` on 35 dyadic parameters. Scan a much denser grid, dyadic and
/// not, and report the max.
#[test]
fn p2_point_set_ulp_band_on_a_dense_grid() {
    let s = sym_surface();
    let r = s.reversed_v().unwrap();
    let ulps = |a: f64, b: f64| -> i128 {
        i128::from(a.to_bits() as i64) - i128::from(b.to_bits() as i64)
    };
    for (label, n, dyadic) in [("dyadic", 256usize, true), ("non-dyadic", 257usize, false)] {
        let mut worst = 0i128;
        let mut worst_at = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        let mut sign_cross = 0usize;
        let mut exact_reflect = true;
        for iu in 0..=n {
            let u = iu as f64 / n as f64;
            for iv in 0..=n {
                let v = iv as f64 / n as f64;
                let vr = 1.0 - v;
                if dyadic {
                    // 1 - v exact iff (1 - v) + v == 1 recovers v.
                    if 1.0 - vr != v {
                        exact_reflect = false;
                    }
                }
                let got = r.eval(u, v);
                let want = s.eval(u, vr);
                for (a, b) in [(got.x, want.x), (got.y, want.y), (got.z, want.z)] {
                    if a.is_sign_negative() != b.is_sign_negative() {
                        sign_cross += 1;
                    }
                    let g = ulps(a, b).abs();
                    if g > worst {
                        worst = g;
                        worst_at = (u, v, a, b);
                    }
                }
            }
        }
        println!(
            "P2[{label}]: grid {}^2, worst ulp gap {worst} at u={} v={} got={:e} want={:e}, \
             sign-straddles {sign_cross}, reflection-exact {exact_reflect}",
            n + 1,
            worst_at.0,
            worst_at.1,
            worst_at.2,
            worst_at.3
        );
    }
}

/// P9. The SHIPPED fixture (`reversal_tests::NET` / `WEIGHTS`, copied
/// verbatim), on a much denser DYADIC grid than the row's 35 points.
/// Does the row's `gap <= 4` bound survive, or is it a property of the
/// 35 chosen parameters?
#[test]
fn p9_shipped_fixture_on_a_dense_dyadic_grid() {
    const NET: [(f64, f64, f64); 15] = [
        (0.0, 0.0, 0.0),
        (1.0, 2.0, -1.0),
        (2.0, -1.0, 3.0),
        (3.0, 1.0, -2.0),
        (4.0, 0.5, 1.0),
        (0.5, 3.0, 1.0),
        (1.5, -2.0, 2.0),
        (2.5, 2.0, 0.0),
        (3.5, 0.0, 3.0),
        (4.5, 1.0, -1.0),
        (1.0, -1.5, 2.5),
        (2.0, 0.0, -3.0),
        (3.0, 2.5, 1.5),
        (4.0, -1.0, 0.0),
        (5.0, 1.5, 2.0),
    ];
    const WEIGHTS: [f64; 15] = [
        1.0, 2.0, 0.5, 4.0, 1.5, 0.25, 3.0, 1.0, 2.5, 0.75, 1.25, 0.5, 2.0, 1.0, 3.5,
    ];
    let s = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.75, 1.0, 1.0, 1.0], 2).unwrap(),
        NET.iter().map(|(x, y, z)| Point3::new(*x, *y, *z)).collect(),
        WEIGHTS.to_vec(),
    )
    .unwrap();
    let r = s.reversed_v().unwrap();
    let ulps =
        |a: f64, b: f64| -> i128 { i128::from(a.to_bits() as i64) - i128::from(b.to_bits() as i64) };
    let mut worst = 0i128;
    let mut worst_at = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut over4 = 0usize;
    let mut sign_cross = 0usize;
    let n = 256usize;
    for iu in 0..=n {
        let u = iu as f64 / n as f64;
        for iv in 0..=n {
            let v = iv as f64 / n as f64;
            let got = r.eval(u, v);
            let want = s.eval(u, 1.0 - v);
            for (a, b) in [(got.x, want.x), (got.y, want.y), (got.z, want.z)] {
                if a.is_sign_negative() != b.is_sign_negative() {
                    sign_cross += 1;
                    println!("P9: SIGN STRADDLE at ({u}, {v}): {a:?} vs {b:?}");
                }
                let g = ulps(a, b).abs();
                if g > 4 {
                    over4 += 1;
                }
                if g > worst {
                    worst = g;
                    worst_at = (u, v, a, b);
                }
            }
        }
    }
    println!(
        "P9: shipped fixture, dyadic {}^2 grid: worst ulp gap {worst} at u={} v={} \
         got={:e} want={:e}; slots over the row's bound of 4: {over4}; sign-straddles {sign_cross}",
        n + 1,
        worst_at.0,
        worst_at.1,
        worst_at.2,
        worst_at.3
    );
    // The row's own 35 points, for comparison.
    let mut row_worst = 0i128;
    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        for &v in &[0.0, 0.125, 0.25, 0.5, 0.75, 0.875, 1.0] {
            let got = r.eval(u, v);
            let want = s.eval(u, 1.0 - v);
            for (a, b) in [(got.x, want.x), (got.y, want.y), (got.z, want.z)] {
                row_worst = row_worst.max(ulps(a, b).abs());
            }
        }
    }
    println!("P9: the row's own 35-point grid: worst ulp gap {row_worst}");
    // How close does the row's grid come to a bad neighbourhood? Perturb
    // v by one grid step of 1/16 (still dyadic, still in the spec's
    // "exact reflection" class).
    let mut near_worst = 0i128;
    let mut near_at = (0.0f64, 0.0f64);
    for iu in 0..=16 {
        let u = f64::from(iu) / 16.0;
        for iv in 0..=16 {
            let v = f64::from(iv) / 16.0;
            let got = r.eval(u, v);
            let want = s.eval(u, 1.0 - v);
            for (a, b) in [(got.x, want.x), (got.y, want.y), (got.z, want.z)] {
                let g = ulps(a, b).abs();
                if g > near_worst {
                    near_worst = g;
                    near_at = (u, v);
                }
            }
        }
    }
    println!("P9: a 17x17 dyadic grid (step 1/16): worst ulp gap {near_worst} at {near_at:?}");
}

/// P3. The shipped row's `ulps` helper subtracts raw bit patterns as
/// `i64`. That overflows when the two values straddle zero in SIGN —
/// a debug-build PANIC, not a wrong number.
#[test]
fn p3_the_rows_ulp_helper_overflows_across_signed_zero() {
    let a = 0.0f64;
    let b = -0.0f64;
    let d = (a.to_bits() as i64).checked_sub(b.to_bits() as i64);
    println!("P3: ulps(0.0, -0.0) as the row computes it = {d:?}");
    assert!(
        d.is_none(),
        "P3: the row's ulp subtraction overflows i64 on a signed-zero pair"
    );
    let c = 1e-300f64;
    let e = -1e-300f64;
    println!(
        "P3: ulps(1e-300, -1e-300) = {:?}",
        (c.to_bits() as i64).checked_sub(e.to_bits() as i64)
    );
}

/// P4. A knot vector that IS its own reflection in ℝ but whose domain
/// sum overflows is refused.
#[test]
fn p4_a_genuinely_symmetric_vector_is_refused_when_lo_plus_hi_overflows() {
    let (lo, mid, hi) = (1e308f64, 1.25e308f64, 1.5e308f64);
    assert_eq!(lo + hi, f64::INFINITY);
    let kv = KnotVector::clamped(vec![lo, lo, lo, mid, hi, hi, hi], 2).unwrap();
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let s = NurbsSurface::new(ku, kv, net(12), wts(12)).unwrap();
    let err = s.reversed_v().unwrap_err();
    println!("P4: {err} / {err:?}");
    assert!(matches!(err, KnotMirrorError::ReflectionNotFinite { .. }));
}

/// P5. `reversed_u` on an asymmetric `knots_u` reports indices into
/// `knots_u`. And `reversed_u` is an involution.
#[test]
fn p5_reversed_u_refusal_indices_and_involution() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let s = NurbsSurface::new(ku, kv, net(12), wts(12)).unwrap();
    let err = s.reversed_u().unwrap_err();
    println!("P5: reversed_u on asymmetric knots_u -> {err:?}");
    assert_eq!(
        err,
        KnotMirrorError::AsymmetricPair {
            index: 3,
            mirror_index: 3,
            knot: 0.25,
            mirror_knot: 0.25,
            lo: 0.0,
            hi: 1.0,
        }
    );
    let t = sym_surface();
    let back = t.reversed_u().unwrap().reversed_u().unwrap();
    assert_eq!(
        back.control()
            .iter()
            .map(|p| (p.x.to_bits(), p.y.to_bits(), p.z.to_bits()))
            .collect::<Vec<_>>(),
        t.control()
            .iter()
            .map(|p| (p.x.to_bits(), p.y.to_bits(), p.z.to_bits()))
            .collect::<Vec<_>>(),
        "P5: reversed_u is an involution bit for bit"
    );
    assert_eq!(back.weights(), t.weights());
}

/// P6. Display strings, read as a user would see them.
#[test]
fn p6_display_reads() {
    let a = KnotMirrorError::AsymmetricPair {
        index: 3,
        mirror_index: 3,
        knot: 0.25,
        mirror_knot: 0.25,
        lo: 0.0,
        hi: 1.0,
    };
    let b = KnotMirrorError::ReflectionNotFinite {
        lo: 1e308,
        hi: 1.5e308,
    };
    println!("P6 asym   : {a}");
    println!("P6 notfin : {b}");
    let c = KnotMirrorError::AsymmetricPair {
        index: 3,
        mirror_index: 4,
        knot: 0.5,
        mirror_knot: f64::from_bits(0.5f64.to_bits() + 1),
        lo: 0.0,
        hi: 1.0,
    };
    println!("P6 midline: {c}");
}

/// P7. An end run whose bits are NOT all identical: `clamped` compares
/// with `==`, so `-0.0` and `0.0` may share a run. The door's doc claims
/// the clamp pairs are "the pair (lo, hi) BIT FOR BIT".
#[test]
fn p7_signed_zero_in_the_clamp_run() {
    let kv = KnotVector::clamped(vec![-0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    println!(
        "P7: knots {:?}, k0 bits {:#x}, k2 bits {:#x}, domain {:?}",
        kv.knots(),
        kv.knots()[0].to_bits(),
        kv.knots()[2].to_bits(),
        kv.domain()
    );
    assert_ne!(
        kv.knots()[0].to_bits(),
        kv.knots()[2].to_bits(),
        "P7: the clamp run is NOT bit-uniform, only ==-uniform"
    );
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let s = NurbsSurface::new(ku, kv, net(12), wts(12)).unwrap();
    let r = s.reversed_v();
    println!("P7: reversed_v -> ok={}", r.is_ok());
    assert!(r.is_ok(), "P7: a -0.0 in the start run must not refuse");
}

/// P8. Randomized differential: the door's verdict against an exact
/// check done in integer arithmetic on the knots' mantissas.
#[test]
fn p8_verdict_agrees_with_exact_rational_symmetry() {
    // Exact (mantissa, exponent) for a finite f64: x == m * 2^e.
    let parts = |x: f64| -> (i128, i64) {
        if x == 0.0 {
            return (0, 0);
        }
        let bits = x.to_bits();
        let sign = if bits >> 63 == 1 { -1i128 } else { 1i128 };
        let exp = ((bits >> 52) & 0x7ff) as i64;
        let frac = i128::from(bits & 0xf_ffff_ffff_ffff);
        let (m, e) = if exp == 0 {
            (frac, -1074i64)
        } else {
            (frac | (1i128 << 52), exp - 1075)
        };
        (sign * m, e)
    };
    // a + b == c + d exactly, all four finite and of comparable scale.
    let sums_equal = |a: f64, b: f64, c: f64, d: f64| -> Option<bool> {
        let p = [parts(a), parts(b), parts(c), parts(d)];
        let emin = p.iter().filter(|q| q.0 != 0).map(|q| q.1).min()?;
        let mut v = [0i128; 4];
        for (i, (m, e)) in p.iter().enumerate() {
            let sh = e - emin;
            if !(0..=64).contains(&sh) {
                return None;
            }
            v[i] = m << sh;
        }
        Some(v[0] + v[1] == v[2] + v[3])
    };
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
        | 1;
    println!("P8 seed = {seed}");
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mut cases = 0usize;
    let mut agreed = 0usize;
    let mut mirrored_cases = 0usize;
    for _ in 0..4000 {
        let a = (next() % 1000) as f64 / 2048.0 + 1.0 / 4096.0;
        let mirrored = 1.0 - a;
        let kind = next() % 3;
        let b = match kind {
            0 => mirrored,
            1 => f64::from_bits(mirrored.to_bits() + 1),
            _ => (next() % 2000) as f64 / 2048.0 + 1.0 / 4096.0,
        };
        if !(a <= b && a > 0.0 && b < 1.0) {
            continue;
        }
        let Ok(kv) = KnotVector::clamped(vec![0.0, 0.0, 0.0, a, b, 1.0, 1.0, 1.0], 2) else {
            continue;
        };
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let s = NurbsSurface::new(ku, kv, net(15), wts(15)).unwrap();
        let door_ok = s.reversed_v().is_ok();
        let Some(truth) = sums_equal(a, b, 0.0, 1.0) else {
            continue;
        };
        if truth {
            mirrored_cases += 1;
        }
        cases += 1;
        if door_ok == truth {
            agreed += 1;
        } else {
            panic!("P8: door {door_ok} vs exact {truth} on a={a:?} b={b:?} (seed printed above)");
        }
    }
    println!("P8: {agreed}/{cases} verdicts agree ({mirrored_cases} genuinely symmetric)");
    assert!(cases > 100, "P8 exercised something");
}
