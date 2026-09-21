//! R2 review probe for RING-0: hand-built inputs per allowlist class at
//! both arithmetics, the division cases, and a direction census of every
//! verdict disagreement the corner corpus forms. Reporting only.

test_utils::gated_to![
    "crates/geom-core/src/ring_interval.rs",
    "crates/geom-core/src/interval.rs",
    "interval-transcendentals/src/",
];

use geom_core::RingInterval;
use interval_transcendentals::{DInterval, Decoration};

const CORNERS: [f64; 16] = [
    f64::NEG_INFINITY,
    -f64::MAX,
    -1e300,
    -1.0,
    -f64::MIN_POSITIVE,
    -5e-324,
    -0.0,
    0.0,
    5e-324,
    f64::MIN_POSITIVE,
    1e-160,
    1.0,
    1e160,
    1e300,
    f64::MAX,
    f64::INFINITY,
];

fn ring(lo: f64, hi: f64) -> RingInterval {
    RingInterval::from_bounds(lo, hi)
}
fn dint(lo: f64, hi: f64) -> DInterval {
    DInterval::from_bounds(lo, hi)
}
fn d_refuses(d: DInterval) -> bool {
    d.is_nai() || d.is_empty() || d.decoration() < Decoration::Def
}
fn show_ring(r: RingInterval) -> String {
    if r.is_poison() {
        "POISON".into()
    } else {
        format!("[{:e}, {:e}]", r.lo(), r.hi())
    }
}
fn show_d(d: DInterval) -> String {
    if d.is_nai() {
        "NaI".into()
    } else if d.is_empty() {
        format!("EMPTY dec={:?}", d.decoration())
    } else {
        format!("[{:e}, {:e}] dec={:?}", d.lo(), d.hi(), d.decoration())
    }
}

#[test]
fn probe_hand_built_class_witnesses() {
    println!("--- class 1: mul-zero-times-infinite");
    for (a, b) in [
        ((-0.0f64, 0.0f64), (1.0f64, f64::INFINITY)),
        ((0.0, 1.0), (f64::NEG_INFINITY, -1.0)),
        ((-1.0, 0.0), (f64::NEG_INFINITY, f64::INFINITY)),
    ] {
        let r = ring(a.0, a.1) * ring(b.0, b.1);
        let d = dint(a.0, a.1) * dint(b.0, b.1);
        println!(
            "  [{:e},{:e}] * [{:e},{:e}] -> ring {} | backend {} | ring_poison={} backend_refuses={}",
            a.0,
            a.1,
            b.0,
            b.1,
            show_ring(r),
            show_d(d),
            r.is_poison(),
            d_refuses(d)
        );
    }
    println!("--- the exact-zero annihilator (excluded from class 1)");
    let r = ring(0.0, 0.0) * ring(1.0, f64::INFINITY);
    let d = dint(0.0, 0.0) * dint(1.0, f64::INFINITY);
    println!(
        "  [0,0] * [1,inf] -> ring {} | backend {} | poison={} refuses={}",
        show_ring(r),
        show_d(d),
        r.is_poison(),
        d_refuses(d)
    );

    println!("--- class 2: div-infinite-over-infinite");
    for (a, b) in [
        ((1.0f64, f64::INFINITY), (1.0f64, f64::INFINITY)),
        ((f64::NEG_INFINITY, 5.0), (1.0, f64::INFINITY)),
        (
            (f64::NEG_INFINITY, f64::INFINITY),
            (f64::NEG_INFINITY, -1.0),
        ),
    ] {
        let r = ring(a.0, a.1) / ring(b.0, b.1);
        let d = dint(a.0, a.1) / dint(b.0, b.1);
        println!(
            "  [{:e},{:e}] / [{:e},{:e}] -> ring {} | backend {} | poison={} refuses={}",
            a.0,
            a.1,
            b.0,
            b.1,
            show_ring(r),
            show_d(d),
            r.is_poison(),
            d_refuses(d)
        );
    }

    println!("--- class 3: powi-chain-spans-zero-and-infinity");
    for (a, n) in [
        ((5e-324f64, f64::INFINITY), 3i32),
        ((-f64::MAX, -0.0), 3),
        ((-f64::MAX, -0.0), 5),
        ((-f64::MAX, -0.0), 4),  // power of two: NOT in the class
        ((-f64::MAX, -0.0), -3), // negative: NOT in the class
    ] {
        let r = ring(a.0, a.1).powi(n);
        let d = dint(a.0, a.1).powi(n);
        println!(
            "  [{:e},{:e}].powi({n}) -> ring {} | backend {} | poison={} refuses={}",
            a.0,
            a.1,
            show_ring(r),
            show_d(d),
            r.is_poison(),
            d_refuses(d)
        );
    }
}

#[test]
fn probe_division_by_hand() {
    println!("--- division, divisor cases");
    let num = (-2.0f64, -1.0f64);
    for b in [
        (-1.0f64, 1.0f64),
        (0.0, 0.0),
        (-0.0, 0.0),
        (0.0, 5e-324),
        (-5e-324, 0.0),
        (1.0, 2.0),
    ] {
        let r = ring(num.0, num.1) / ring(b.0, b.1);
        let d = dint(num.0, num.1) / dint(b.0, b.1);
        println!(
            "  [-2,-1] / [{:e},{:e}] -> ring {} | backend {} | poison={} refuses={}",
            b.0,
            b.1,
            show_ring(r),
            show_d(d),
            r.is_poison(),
            d_refuses(d)
        );
    }
}

#[test]
fn probe_disagreement_direction_census() {
    let mut brackets = Vec::new();
    for (i, &x) in CORNERS.iter().enumerate() {
        for &y in &CORNERS[i..] {
            brackets.push(if x <= y { (x, y) } else { (y, x) });
        }
    }
    let mut ring_poisons_backend_certifies = std::collections::BTreeMap::new();
    let mut backend_refuses_ring_certifies = std::collections::BTreeMap::new();
    let mut ex: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    fn bump(m: &mut std::collections::BTreeMap<String, u64>, k: &str) {
        *m.entry(k.to_string()).or_insert(0) += 1;
    }
    for &a in &brackets {
        for &b in &brackets {
            let (ra, rb) = (ring(a.0, a.1), ring(b.0, b.1));
            let (da, db) = (dint(a.0, a.1), dint(b.0, b.1));
            let mut cases: Vec<(String, RingInterval, DInterval)> = vec![
                ("add".into(), ra + rb, da + db),
                ("sub".into(), ra - rb, da - db),
                ("mul".into(), ra * rb, da * db),
                ("div".into(), ra / rb, da / db),
                ("neg".into(), -ra, -da),
                ("sqr".into(), ra.sqr(), da.powi(2)),
            ];
            for n in [-3i32, 0, 1, 2, 3, 5] {
                cases.push((format!("powi({n})"), ra.powi(n), da.powi(n)));
            }
            for (name, r, d) in cases {
                let rp = r.is_poison();
                let dr = d_refuses(d);
                if rp == dr {
                    continue;
                }
                let m = if rp {
                    &mut ring_poisons_backend_certifies
                } else {
                    &mut backend_refuses_ring_certifies
                };
                bump(m, &name);
                let key = format!("{name}|{rp}");
                ex.entry(key).or_insert_with(|| {
                    format!(
                        "a=[{:e},{:e}] b=[{:e},{:e}] ring {} backend {}",
                        a.0,
                        a.1,
                        b.0,
                        b.1,
                        show_ring(r),
                        show_d(d)
                    )
                });
            }
        }
    }
    println!("brackets: {}", brackets.len());
    println!("RING POISONS, BACKEND CERTIFIES: {ring_poisons_backend_certifies:?}");
    println!("BACKEND REFUSES, RING CERTIFIES: {backend_refuses_ring_certifies:?}");
    for (k, v) in &ex {
        println!("  first {k}: {v}");
    }
}

// ---- precision of each allowlist predicate over the corner corpus, and
// ---- an exponent sweep wider than the differential's fixed [-3,0,1,2,3,5].

fn brackets() -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for (i, &x) in CORNERS.iter().enumerate() {
        for &y in &CORNERS[i..] {
            out.push(if x <= y { (x, y) } else { (y, x) });
        }
    }
    out
}

fn is_exact_zero(a: (f64, f64)) -> bool {
    a.0 == 0.0 && a.1 == 0.0
}
fn has_zero_endpoint(a: (f64, f64)) -> bool {
    a.0 == 0.0 || a.1 == 0.0
}
fn is_unbounded(a: (f64, f64)) -> bool {
    a.0.is_infinite() || a.1.is_infinite()
}
fn spans_zero(a: (f64, f64)) -> bool {
    a.0 <= 0.0 && a.1 >= 0.0
}
fn strictly_one_signed(a: (f64, f64)) -> bool {
    a.0 > 0.0 || a.1 < 0.0
}
fn chain_spans_zero_and_infinity(a: (f64, f64), n: i32) -> bool {
    let mut small = if spans_zero(a) {
        0.0
    } else {
        a.0.abs().min(a.1.abs())
    };
    let mut large = a.0.abs().max(a.1.abs());
    let mut rz = small == 0.0;
    let mut ri = large.is_infinite();
    let sq = (i32::BITS - 1) - (n.unsigned_abs() | 1).leading_zeros();
    for _ in 0..sq {
        small = (small * small).next_down().max(0.0);
        large = (large * large).next_up();
        rz |= small == 0.0;
        ri |= large.is_infinite();
    }
    rz && ri
}

#[test]
fn probe_allowlist_precision() {
    let bs = brackets();
    let (mut hold, mut hold_and_dis, mut dis) = (0u64, 0u64, 0u64);
    for &a in &bs {
        for &b in &bs {
            let h = !is_exact_zero(a)
                && !is_exact_zero(b)
                && ((has_zero_endpoint(a) && is_unbounded(b))
                    || (has_zero_endpoint(b) && is_unbounded(a)));
            let d = (ring(a.0, a.1) * ring(b.0, b.1)).is_poison()
                != d_refuses(dint(a.0, a.1) * dint(b.0, b.1));
            hold += h as u64;
            dis += d as u64;
            hold_and_dis += (h && d) as u64;
        }
    }
    println!(
        "class1 mul-zero-times-infinite: predicate holds {hold}, disagreements {dis}, both {hold_and_dis}"
    );
    let (mut hold, mut hold_and_dis, mut dis) = (0u64, 0u64, 0u64);
    for &a in &bs {
        for &b in &bs {
            let h = is_unbounded(a) && is_unbounded(b) && strictly_one_signed(b);
            let d = (ring(a.0, a.1) / ring(b.0, b.1)).is_poison()
                != d_refuses(dint(a.0, a.1) / dint(b.0, b.1));
            hold += h as u64;
            dis += d as u64;
            hold_and_dis += (h && d) as u64;
        }
    }
    println!(
        "class2 div-infinite-over-infinite: predicate holds {hold}, disagreements {dis}, both {hold_and_dis}"
    );
    for n in [-8i32, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 16, 31] {
        let (mut hold, mut hold_and_dis, mut dis, mut unmatched) = (0u64, 0u64, 0u64, 0u64);
        let mut first_unmatched = String::new();
        for &a in &bs {
            let h = n >= 3 && n.count_ones() >= 2 && chain_spans_zero_and_infinity(a, n);
            let rr = ring(a.0, a.1).powi(n);
            let dd = dint(a.0, a.1).powi(n);
            let d = rr.is_poison() != d_refuses(dd);
            hold += h as u64;
            dis += d as u64;
            hold_and_dis += (h && d) as u64;
            if d && !h {
                unmatched += 1;
                if first_unmatched.is_empty() {
                    first_unmatched = format!(
                        "a=[{:e},{:e}] ring {} backend {}",
                        a.0,
                        a.1,
                        show_ring(rr),
                        show_d(dd)
                    );
                }
            }
        }
        println!(
            "powi({n}): predicate holds {hold}, disagreements {dis}, both {hold_and_dis}, UNMATCHED {unmatched} {first_unmatched}"
        );
    }
}

#[test]
fn probe_negative_exponents() {
    let bs = brackets();
    for n in (-12i32..=-1).rev() {
        let mut rows = Vec::new();
        for &a in &bs {
            let rr = ring(a.0, a.1).powi(n);
            let dd = dint(a.0, a.1).powi(n);
            if rr.is_poison() != d_refuses(dd) {
                rows.push(format!(
                    "a=[{:e},{:e}] ring {} backend {}",
                    a.0,
                    a.1,
                    show_ring(rr),
                    show_d(dd)
                ));
            }
        }
        println!("powi({n}): {} disagreements", rows.len());
        for r in rows.iter().take(20) {
            println!("    {r}");
        }
    }
}

#[test]
fn probe_negative_exponents_finite_bases() {
    // Are the n = -1 disagreements only at unbounded brackets?
    let vals = [
        -1e300f64, -1.0, -1e-160, -5e-324, 5e-324, 1e-160, 1.0, 1e300,
    ];
    for &lo in &vals {
        for &hi in &vals {
            if lo > hi {
                continue;
            }
            for n in [-1i32, -2, -3] {
                let rr = ring(lo, hi).powi(n);
                let dd = dint(lo, hi).powi(n);
                if rr.is_poison() != d_refuses(dd) {
                    println!(
                        "FINITE powi({n}) a=[{lo:e},{hi:e}] ring {} backend {}",
                        show_ring(rr),
                        show_d(dd)
                    );
                }
            }
        }
    }
    println!("finite-base negative-exponent scan done");
}

#[test]
fn probe_why_powi_minus_one() {
    let a = (5e-324f64, 1.0f64);
    println!("dint(a)            = {}", show_d(dint(a.0, a.1)));
    println!("dint(a).powi(1)    = {}", show_d(dint(a.0, a.1).powi(1)));
    println!("dint(a).powi(-1)   = {}", show_d(dint(a.0, a.1).powi(-1)));
    println!(
        "dint(1,1)/dint(a)  = {}",
        show_d(dint(1.0, 1.0) / dint(a.0, a.1))
    );
    println!("ring(a).powi(1)    = {}", show_ring(ring(a.0, a.1).powi(1)));
    println!(
        "ring(a).powi(-1)   = {}",
        show_ring(ring(a.0, a.1).powi(-1))
    );
    println!(
        "ring(1,1)/ring(a)  = {}",
        show_ring(ring(1.0, 1.0) / ring(a.0, a.1))
    );
    // Does plain division overflow disagree anywhere?
    let vals = [5e-324f64, 1e-320, f64::MIN_POSITIVE, 1e-300, 1.0, 1e300];
    for &x in &vals {
        for &y in &vals {
            if x > y {
                continue;
            }
            let r = ring(1.0, 1e300) / ring(x, y);
            let d = dint(1.0, 1e300) / dint(x, y);
            if r.is_poison() != d_refuses(d) {
                println!(
                    "DIV DISAGREE [1,1e300]/[{x:e},{y:e}] ring {} backend {}",
                    show_ring(r),
                    show_d(d)
                );
            }
        }
    }
    println!("div overflow scan done");
}

/// The three end-to-end witnesses, at the ring and at the newtype
/// (`DInterval` IS the newtype: the dry run forwards every op to it).
#[test]
fn probe_end_to_end_witnesses() {
    println!("--- certified_door: poison reachable by arithmetic");
    for (tag, r, d) in [
        (
            "[1,2]/[0,0]",
            ring(1.0, 2.0) / ring(0.0, 0.0),
            dint(1.0, 2.0) / dint(0.0, 0.0),
        ),
        (
            "[0,1]*[0,inf]",
            ring(0.0, 1.0) * ring(0.0, f64::INFINITY),
            dint(0.0, 1.0) * dint(0.0, f64::INFINITY),
        ),
        (
            "[-2,-1]/[0,5e-324]  (a Trv with REAL endpoints)",
            ring(-2.0, -1.0) / ring(0.0, 5e-324),
            dint(-2.0, -1.0) / dint(0.0, 5e-324),
        ),
        (
            "([-2,-1]/[-1,1])*[0,0]  (a Trv with FINITE endpoints)",
            (ring(-2.0, -1.0) / ring(-1.0, 1.0)) * ring(0.0, 0.0),
            (dint(-2.0, -1.0) / dint(-1.0, 1.0)) * dint(0.0, 0.0),
        ),
    ] {
        println!(
            "  {tag}: ring {} (poison={}) | newtype {} (poison={}) endpoints ({:e},{:e})",
            show_ring(r),
            r.is_poison(),
            show_d(d),
            d_refuses(d),
            d.lo(),
            d.hi()
        );
    }

    println!("--- the sign clamp (review_m5_pr2_scratch::lane_sign_clamp)");
    let (a, b) = (2.2250738585072014e-308f64, -1.8669573922462645e-308f64);
    let rp = ring(a, a) * ring(b, b);
    let dp = dint(a, a) * dint(b, b);
    println!(
        "  [a,a]*[b,b] with a>0>b: ring {} | newtype {}",
        show_ring(rp),
        show_d(dp)
    );
    let (c, e) = (1.902e-308f64, 1.902e-308f64);
    println!(
        "  same-sign [c,c]*[e,e]: ring {} | newtype {}",
        show_ring(ring(c, c) * ring(e, e)),
        show_d(dint(c, c) * dint(e, e))
    );

    println!("--- the zero annihilator");
    println!(
        "  [0,0]*[-inf,inf]: ring {} | newtype {}",
        show_ring(ring(0.0, 0.0) * ring(f64::NEG_INFINITY, f64::INFINITY)),
        show_d(dint(0.0, 0.0) * dint(f64::NEG_INFINITY, f64::INFINITY))
    );

    println!("--- powi(3) on a production-shaped weight hull (quad.rs `wm.powi(3)`)");
    for w in [(0.5f64, 2.0f64), (1e-200, 1e200), (0.0, 1.0)] {
        println!(
            "  [{:e},{:e}].powi(3): ring {} | newtype {}",
            w.0,
            w.1,
            show_ring(ring(w.0, w.1).powi(3)),
            show_d(dint(w.0, w.1).powi(3))
        );
    }
}
