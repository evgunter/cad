//! The demo K sweep (`demo-tour k-probe`) at each tolerance row: it
//! exits 0, every volume it reads is an honest enclosure, the lily's
//! swept leaves' enclosures hold their closed-form volume to within the
//! leaves' own fit ([`FIT_REL`]), and the bodies it reads as a BRACKET
//! are exactly the ones listed per ε.
//!
//! A body comes back as a bracket when its gate certified the sign and
//! a face's quadrature lane stops (its floor or its work wall) short of
//! the reporting target `1024·ε`. At 1e-9 and 1e-6 that is the long
//! lofted leaf alone. At 1e-12 the sweep does not finish: `probe.rs`
//! builds `lily::plant` without reading the lily's declared stem
//! frontier, and the lower stem refuses at that row, so the 1e-12 test
//! is red (`work/ciw/the-tours-k-probe-brackets-suite-runs-in-no-ci-row.md`).
//! The set it pins there (the long leaf and the three sepals, with the
//! swept leaves `b` and `c` certifying a number) cannot be re-read until
//! that row runs.
//!
//! The sweep writes one stderr record per body (`probe.rs`'s module
//! doc): `k_probe(demo)\tvolume\t<scene>\t<label>\t<kind>\t<lo>\t<hi>`.
//!
//! One process per ε (`Tolerance` is a OnceLock), so each row spawns
//! the binary, as `eps_regression.rs` does.
#![cfg(feature = "probe")]

use std::collections::BTreeSet;
use std::process::Command;

/// One body's volume reading as the sweep recorded it.
struct Reading {
    label: String,
    bracket: bool,
    lo: f64,
    hi: f64,
}

/// Runs the sweep at `eps`; returns every body's reading.
fn sweep(eps: &str) -> Vec<Reading> {
    let csv = std::env::temp_dir().join(format!(
        "demo-tour-k-probe-brackets-{eps}-{}.csv",
        std::process::id()
    ));
    let out = Command::new(env!("CARGO_BIN_EXE_demo-tour"))
        .arg("k-probe")
        .arg(&csv)
        .env("CAD_TOLERANCE_EPS", eps)
        .output()
        .expect("spawn demo-tour");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let lines: Vec<&str> = stderr.lines().collect();
    assert!(
        out.status.success(),
        "k-probe at eps {eps} exited {:?}; stderr tail:\n{}",
        out.status.code(),
        lines[lines.len().saturating_sub(20)..].join("\n")
    );
    let readings = lines
        .iter()
        .filter_map(|l| l.strip_prefix("k_probe(demo)\tvolume\t"))
        .map(|rec| {
            let f: Vec<&str> = rec.split('\t').collect();
            let [_scene, label, kind, lo, hi] = f[..] else {
                panic!("a volume record with {} fields: {rec}", f.len());
            };
            let num = |x: &str| {
                x.parse::<f64>()
                    .unwrap_or_else(|e| panic!("{rec}: {x}: {e}"))
            };
            Reading {
                label: label.to_string(),
                bracket: match kind {
                    "number" => false,
                    "bracket" => true,
                    other => panic!("{rec}: unknown reading kind {other}"),
                },
                lo: num(lo),
                hi: num(hi),
            }
        })
        .collect();
    let _ = std::fs::remove_file(&csv);
    readings
}

/// How far a swept leaf's volume sits from its Pappus volume, relative:
/// the leaf is a FIT (a cubic spine through nine exact points of the
/// arc, a degree-3 skin through nine stations), so its volume is the
/// Pappus volume only to the fit's order. Measured at 1e-12, where both
/// leaves certify a number: the fitted volume sits ABOVE Pappus by
/// 9.8e-8 (`b`) and 7.9e-8 (`c`) relative, and the same leaves on five
/// stations sit 14× and 12× further off, the fourth order of a cubic
/// fit (2⁴ = 16) — a fit's offset, not an enclosure's miss. Twice the
/// larger is the slack, at 1e-12 only.
const FIT_REL: f64 = 2e-7;

/// The swept leaves' Pappus volumes: a rigid lens carried along the
/// spine's normal frame sweeps `A·(len + |curl|·rise)`, the lens being
/// two circular segments on one chord (`lily.rs`'s `Lance::area` and
/// `Lance::centroid_rise`). The numbers are `LEAF_B` and `LEAF_C`'s.
fn pappus() -> [(&'static str, f64); 2] {
    let segment = |c: f64, h: f64| {
        let r = (0.25 * c * c + h * h) / (2.0 * h);
        let th = 4.0 * (2.0 * h / c).atan();
        let area = 0.5 * r * r * (th - th.sin());
        let rise =
            4.0 * r * (0.5 * th).sin().powi(3) / (3.0 * (th - th.sin())) - r * (0.5 * th).cos();
        (area, area * rise)
    };
    let leaf = |width: f64, ridge: f64, keel: f64, len: f64, curl: f64| {
        let ((a_r, m_r), (a_k, m_k)) = (segment(width, ridge), segment(width, keel));
        let area = a_r + a_k;
        area * curl.abs().mul_add((m_r - m_k) / area, len)
    };
    [
        ("lily_leaf_b", leaf(0.170, 0.015, 0.007, 1.25, -0.40)),
        ("lily_leaf_c", leaf(0.140, 0.013, 0.006, 0.95, -0.35)),
    ]
}

/// Every reading is an enclosure (`lo ≤ hi`, strictly for a bracket),
/// both leaves are read and hold their Pappus volume to [`FIT_REL`], and the
/// bracketed set is `expect`.
fn check(eps: &str, readings: &[Reading], expect: &[&str]) {
    for r in readings {
        assert!(
            r.lo <= r.hi && (!r.bracket || r.lo < r.hi),
            "eps {eps}: {}: [{:e}, {:e}] is not an enclosure",
            r.label,
            r.lo,
            r.hi
        );
    }
    for (name, v) in pappus() {
        let r = readings
            .iter()
            .find(|r| r.label == name)
            .unwrap_or_else(|| panic!("eps {eps}: no volume record for {name}"));
        // The fit's offset shows only where the enclosure is narrower
        // than it: at 1e-12. Every looser row holds Pappus outright.
        let slack = if eps == "1e-12" { FIT_REL * v } else { 0.0 };
        assert!(
            r.lo - slack <= v && v <= r.hi + slack,
            "eps {eps}: {name}: Pappus {v:e} outside [{:e}, {:e}] by more than the fit's {slack:e}",
            r.lo,
            r.hi
        );
    }
    let bracketed: BTreeSet<&str> = readings
        .iter()
        .filter(|r| r.bracket)
        .map(|r| r.label.as_str())
        .collect();
    assert_eq!(
        bracketed,
        expect.iter().copied().collect(),
        "bracketed bodies at eps {eps}"
    );
}

#[test]
fn k_probe_brackets_the_lofted_leaf_and_the_sepals_at_eps_1e_12() {
    let readings = sweep("1e-12");
    check(
        "1e-12",
        &readings,
        &[
            "lily_leaf_a",
            "lily_sepal_a",
            "lily_sepal_b",
            "lily_sepal_c",
        ],
    );
}

#[test]
fn k_probe_brackets_the_lofted_leaf_at_eps_1e_9() {
    let readings = sweep("1e-9");
    check("1e-9", &readings, &["lily_leaf_a"]);
}

#[test]
fn k_probe_brackets_the_lofted_leaf_at_eps_1e_6() {
    let readings = sweep("1e-6");
    check("1e-6", &readings, &["lily_leaf_a"]);
}
