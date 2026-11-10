//! Counterexample search: no arc rim certifies with an envelope under
//! its miss. Rows draw the radius, one to four sub-arcs of up to 179°
//! (a half turn is where the arc has no positive weight), held end
//! weights from 1e-3 to 1e3 or one value, and each sub-arc's normalized
//! weight at `cos(h/2)` or off it by up to a few bands in metres, either
//! side. Each row is certified at `f64` and at `Interval`, and every
//! certificate's envelope must hold the miss sampled off the geometry
//! (`iso_arc_weight_rows`).
//!
//! `CAD_ISO_ARC_DUMP=<file>` appends each certified row as a JSON line
//! for `scripts/oracles/iso_arc_weight_miss_mpmath.py`, which re-derives
//! the miss at 50 digits.
test_utils::gated_to![
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom-brep/src/edge_nurbs.rs",
    "crates/geom-brep/src/pcurve_cache/iso_arc_weight_rows.rs",
];

use std::io::Write as _;

use test_utils::fuzz;

use super::harmonic_sup_rows::zero;
use super::iso_arc_weight_rows::{ArcRow, Verdict};

fn draw(g: &mut fuzz::Rng) -> ArcRow {
    let radius = g.log_uniform(1e-3, 1.0);
    let n = 1 + g.below(4);
    #[allow(clippy::cast_precision_loss)]
    let top = 179.0_f64.min(359.0 / n as f64);
    let h = if g.below(4) == 0 {
        g.range(top - 10.0, top)
    } else {
        g.range(1.0, top)
    }
    .to_radians();
    let c = (h / 2.0).cos();
    let one_end = g.below(2) == 0;
    let ends: Vec<f64> = (0..=n)
        .map(|_| {
            if one_end {
                1.0
            } else {
                g.log_uniform(1e-3, 1e3)
            }
        })
        .collect();
    let normal: Vec<f64> = (0..n)
        .map(|_| {
            if g.below(4) == 0 {
                c
            } else {
                let side = if g.below(2) == 0 { 1.0 } else { -1.0 };
                c + side * g.log_uniform(1e-4, 3.0) * zero() / radius
            }
        })
        .collect();
    ArcRow::uniform(radius, h, &ends, &normal)
}

fn dump(row: &ArcRow, scalar: &str, envelope: f64) {
    let Some(path) = std::env::var_os("CAD_ISO_ARC_DUMP") else {
        return;
    };
    let (pts, w) = row.column();
    let line = format!(
        "{{\"scalar\":\"{scalar}\",\"radius\":{:e},\"h\":{:e},\"spans\":{},\"control\":{:?},\"weights\":{:?},\"envelope\":{:e}}}\n",
        row.radius,
        row.h,
        row.spans(),
        pts,
        w,
        envelope
    );
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap()
        .write_all(line.as_bytes())
        .unwrap();
}

#[test]
fn no_arc_rim_certifies_under_its_miss() {
    let mut g = fuzz::start("iso_arc_weight_miss_search");
    let mut certified = 0;
    let mut not = std::collections::BTreeMap::<String, usize>::new();
    for _ in 0..fuzz::scaled(30) {
        let row = draw(&mut g);
        for (scalar, verdict) in [
            ("f64", row.certify::<f64>()),
            ("interval", row.certify::<geom_core::Interval>()),
        ] {
            match verdict {
                Verdict::Certified { envelope } => {
                    certified += 1;
                    dump(&row, scalar, envelope);
                    let miss = row.sampled_miss();
                    assert!(
                        envelope >= miss - row.eval_rounding(),
                        "{scalar}: envelope {envelope:e} under the sampled miss {miss:e} \
                         ({row:?}); {}",
                        fuzz::replay()
                    );
                }
                Verdict::Refused(why) | Verdict::NoPick(why) => {
                    let kind = why.split([' ', '{', '(']).next().unwrap_or("").to_owned();
                    *not.entry(kind).or_default() += 1;
                }
            }
        }
    }
    eprintln!("iso_arc_weight_miss_search: {certified} certified; not: {not:?}");
}
