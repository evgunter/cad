//! R2 (MESH-4) review probe — ADDITIVE, not part of any cargo target.
//!
//! Bitwise differential of PR #1517's six ported terminal ε reads
//! against the bare spellings the merge base `ba0a90a08` wrote, over a
//! boundary battery centred on the band edge. The point is the class
//! the byte-identity digest CANNOT see: an off-by-inclusivity at a band
//! edge no corpus body lands on.
//!
//! Build and run standalone:
//!     rustc -O probes/r2_band_edge_differential.rs -o /tmp/r2probe && /tmp/r2probe
//!
//! The four op bodies are transcribed from the head's
//! `crates/mesh/src/sizing.rs`; the six bare spellings from
//! `git show ba0a90a08:crates/mesh/src/{walk,trimmed}.rs`.

// ---- HEAD: the four operations, bodies verbatim from sizing.rs ----
fn separates(band: f64, length: f64) -> bool {
    length > band
}
fn coincident(band: f64, length: f64) -> bool {
    length <= band
}
fn dominates(band: f64, scaled: f64) -> bool {
    scaled < band
}
fn pad(band: f64, bound: f64) -> f64 {
    bound + band
}

// ---- MERGE BASE: the bare spellings, verbatim from ba0a90a08 ----
// walk.rs:607   `gap * lever < eps`            (gap_is_noise)
fn base_gap_is_noise(gap: f64, lever: f64, eps: f64) -> bool {
    gap * lever < eps
}
// walk.rs:881   `chart.radial(junction) > eps` (iso_side_starts)
fn base_iso_side(radial: f64, eps: f64) -> bool {
    radial > eps
}
// walk.rs:1098  `(p - pp).norm() <= eps`       (pole_index)
fn base_pole_index(norm: f64, eps: f64) -> bool {
    norm <= eps
}
// walk.rs:1075  `d <= eps`                     (coincident_declared)
fn base_coincident_declared(d: f64, eps: f64) -> bool {
    d <= eps
}
// walk.rs:1164  `gap <= eps`                   (issue-896 pole guard)
fn base_pole_guard(gap: f64, eps: f64) -> bool {
    gap <= eps
}
// trimmed.rs:594 `d / (bound + tol.eps)`       (deviation probe)
fn base_trimmed_ratio(d: f64, bound: f64, eps: f64) -> f64 {
    d / (bound + eps)
}

// ---- HEAD call-site expressions, as the head actually spells them ----
fn head_gap_is_noise(gap: f64, lever: f64, eps: f64) -> bool {
    dominates(eps, gap * lever)
}
fn head_iso_side(radial: f64, eps: f64) -> bool {
    separates(eps, radial)
}
fn head_pole_index(norm: f64, eps: f64) -> bool {
    coincident(eps, norm)
}
fn head_coincident_declared(d: f64, eps: f64) -> bool {
    coincident(eps, d)
}
fn head_pole_guard(gap: f64, eps: f64) -> bool {
    coincident(eps, gap)
}
fn head_trimmed_ratio(d: f64, bound: f64, eps: f64) -> f64 {
    d / pad(eps, bound)
}

fn ulp_up(x: f64) -> f64 {
    f64::from_bits(x.to_bits() + 1)
}
fn ulp_down(x: f64) -> f64 {
    f64::from_bits(x.to_bits() - 1)
}

/// Lengths to probe against a band, centred hard on the edge.
fn battery(band: f64) -> Vec<(&'static str, f64)> {
    let mut v: Vec<(&'static str, f64)> = vec![
        ("exactly the band", band),
        ("band - 1ulp", ulp_down(band)),
        ("band + 1ulp", ulp_up(band)),
        ("+0.0", 0.0),
        ("-0.0", -0.0),
        ("smallest subnormal", f64::from_bits(1)),
        ("-subnormal", -f64::from_bits(1)),
        ("half band", band / 2.0),
        ("double band", band * 2.0),
        ("1.0", 1.0),
        ("-1.0", -1.0),
        ("+inf", f64::INFINITY),
        ("-inf", f64::NEG_INFINITY),
        ("NaN", f64::NAN),
        ("-NaN", -f64::NAN),
        ("MAX", f64::MAX),
        ("MIN", f64::MIN),
    ];
    // A deterministic spread of ordinary magnitudes either side.
    let mut s: u64 = 0x2545_F491_4F6C_DD1D;
    for _ in 0..2000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        // Values in [0, 4*band], so a good share land near the edge.
        let f = (s >> 11) as f64 / (1u64 << 53) as f64;
        v.push(("random near band", f * 4.0 * band));
    }
    v
}

fn main() {
    // Bands: the three suite ε, plus zero, plus a denormal and a NaN band.
    let bands: Vec<(&str, f64)> = vec![
        ("1e-9 (default)", 1e-9),
        ("1e-6", 1e-6),
        ("1e-12", 1e-12),
        ("0.0 (curved's band fixtures)", 0.0),
        ("0.15 (walk's pole-guard row)", 0.15),
        ("3.38e-5 (closure-bar row)", 3.38e-5),
        ("subnormal", f64::from_bits(1)),
        ("NaN band", f64::NAN),
    ];
    let mut checks = 0u64;
    let mut fails = 0u64;
    for (bname, band) in &bands {
        for (lname, x) in battery(*band) {
            // 1. gap_is_noise: probed as (gap, lever) pairs whose
            //    product is x, plus the lever==0 limit the doc claims.
            for (gap, lever) in [
                (x, 1.0),
                (1.0, x),
                (x, 0.0),
                (0.0, x),
                (x, 2.0),
                (x / 3.0, 3.0),
            ] {
                let b = base_gap_is_noise(gap, lever, *band);
                let h = head_gap_is_noise(gap, lever, *band);
                checks += 1;
                if b != h {
                    fails += 1;
                    println!(
                        "MISMATCH gap_is_noise band={bname} {lname} gap={gap:e} lever={lever:e}: base={b} head={h}"
                    );
                }
            }
            // 2..5: the single-length predicates.
            let pairs: [(&str, bool, bool); 4] = [
                (
                    "iso_side_starts",
                    base_iso_side(x, *band),
                    head_iso_side(x, *band),
                ),
                (
                    "pole_index",
                    base_pole_index(x, *band),
                    head_pole_index(x, *band),
                ),
                (
                    "coincident_declared",
                    base_coincident_declared(x, *band),
                    head_coincident_declared(x, *band),
                ),
                (
                    "pole_guard",
                    base_pole_guard(x, *band),
                    head_pole_guard(x, *band),
                ),
            ];
            for (name, b, h) in pairs {
                checks += 1;
                if b != h {
                    fails += 1;
                    println!("MISMATCH {name} band={bname} {lname} x={x:e}: base={b} head={h}");
                }
            }
            // 6. trimmed's ratio, compared BITWISE (it is arithmetic,
            //    not a predicate — an operand-order change would show
            //    here and nowhere else).
            for d in [1.0, 0.0, x, 5e-17, f64::NAN] {
                let b = base_trimmed_ratio(d, x, *band);
                let h = head_trimmed_ratio(d, x, *band);
                checks += 1;
                let same = b.to_bits() == h.to_bits() || (b.is_nan() && h.is_nan());
                if !same {
                    fails += 1;
                    println!(
                        "MISMATCH trimmed_ratio band={bname} bound={lname}({x:e}) d={d:e}: base={b:e}({:016x}) head={h:e}({:016x})",
                        b.to_bits(),
                        h.to_bits()
                    );
                }
            }
        }
    }
    println!("R2 band-edge differential: {checks} checks, {fails} mismatches");

    // ---- The claims the PR makes ABOUT the ops, tested directly ----
    let mut claim_fails = 0u64;
    for (bname, band) in &bands {
        for (lname, x) in battery(*band) {
            // "separates and coincident are exact negations on ordered input"
            if !x.is_nan() && !band.is_nan() {
                if separates(*band, x) == coincident(*band, x) {
                    claim_fails += 1;
                    println!("CLAIM FAIL negation band={bname} {lname} x={x:e}");
                }
            }
            // "both are FALSE on a NaN"
            if separates(*band, f64::NAN)
                || coincident(*band, f64::NAN)
                || dominates(*band, f64::NAN)
            {
                claim_fails += 1;
                println!("CLAIM FAIL NaN band={bname}");
            }
            // "dominates differs from coincident ONLY at the edge"
            let differ = dominates(*band, x) != coincident(*band, x);
            if differ && !(x == *band) {
                claim_fails += 1;
                println!(
                    "CLAIM FAIL edge-only band={bname} {lname} x={x:e}: dominates={} coincident={}",
                    dominates(*band, x),
                    coincident(*band, x)
                );
            }
            // "pad widens UP by exactly one band"
            let p = pad(*band, x);
            if !band.is_nan() && !x.is_nan() && *band > 0.0 && x.is_finite() && p < x {
                claim_fails += 1;
                println!("CLAIM FAIL pad-direction band={bname} {lname} x={x:e} -> {p:e}");
            }
        }
    }
    println!("R2 op-claim battery: {claim_fails} claim failures");
}
