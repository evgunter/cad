//! The demo K sweep (`demo-tour k-probe`) at each tolerance row: it
//! exits 0, every volume it reads is an honest enclosure, the lily's
//! swept leaves' enclosures contain their closed-form volume, and the
//! bodies it reads as a BRACKET are exactly the ones listed per ε.
//!
//! A body comes back as a bracket when its gate certified the sign and
//! its quadrature schedule cannot reach the reporting target
//! `1024·ε`; the lane says so after round 0 (`props::quad`'s
//! `last_round_refuses`), and that decision is metered as a
//! `props_quad_last_round` sample, so a bracket shows in the CSV as a
//! `negative` row of that predicate. The lily's swept leaves are
//! rational walls along a cubic spine whose last-round width is about
//! 1.5e-8 m of mean displacement, so they certify at 1e-6 and 1e-9 and
//! bracket at 1e-12.
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

/// Runs the sweep at `eps`; returns every body's reading and the
/// outcomes of `demo/lily`'s `props_quad_last_round` samples.
fn sweep(eps: &str) -> (Vec<Reading>, Vec<String>) {
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
    let rows = std::fs::read_to_string(&csv).expect("read the k-probe csv");
    let _ = std::fs::remove_file(&csv);
    let last_round = rows
        .lines()
        .filter(|r| r.starts_with("demo/lily,props_quad_last_round,"))
        .map(|r| r.rsplit(',').next().unwrap_or_default().to_string())
        .collect();
    (readings, last_round)
}

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
/// both leaves are read and contain their Pappus volume, and the
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
        assert!(
            r.lo <= v && v <= r.hi,
            "eps {eps}: {name}: Pappus {v:e} outside [{:e}, {:e}]",
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
fn k_probe_brackets_the_swept_leaves_at_eps_1e_12() {
    let (readings, last_round) = sweep("1e-12");
    check("1e-12", &readings, &["lily_leaf_b", "lily_leaf_c"]);
    assert!(
        !last_round.is_empty() && last_round.iter().all(|o| o == "negative"),
        "the lily's last-round samples at 1e-12: {last_round:?}"
    );
}

#[test]
fn k_probe_measures_every_body_at_eps_1e_9() {
    let (readings, last_round) = sweep("1e-9");
    check("1e-9", &readings, &[]);
    assert!(
        last_round.iter().all(|o| o != "negative"),
        "the lily's last-round samples at 1e-9: {last_round:?}"
    );
}

#[test]
fn k_probe_measures_every_body_at_eps_1e_6() {
    let (readings, _) = sweep("1e-6");
    check("1e-6", &readings, &[]);
}
