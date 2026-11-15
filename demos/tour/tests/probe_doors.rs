//! PROBE (never committed).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use demo_tour::plate::{RADIUS_SIGMA, SPACING_HALF_WIDTH, WEB_BOUND, cut_plate, plate};
use pncad::analysis::{AnalysisPolicy, DriveConfig, analyzed_box, drive};
use pncad::geom_core::Tol;

fn env_f64(name: &str, default: f64) -> f64 {
    std::env::var(name).map_or(default, |v| v.parse().expect("a number"))
}

#[test]
#[ignore]
fn probe_cut_plate() {
    let tol = Tol::witness();
    let s = env_f64("P_SCALE", 1.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let leaves = env_f64("P_LEAVES", 1.0) as usize;
    let which = std::env::var("P_DOC").unwrap_or_else(|_| "cut".into());
    let p = if which == "cut" {
        cut_plate(SPACING_HALF_WIDTH * s, RADIUS_SIGMA * s, WEB_BOUND, tol)
    } else {
        plate(SPACING_HALF_WIDTH * s, RADIUS_SIGMA * s, WEB_BOUND, tol)
    };
    let analyzed = analyzed_box(&p.doc, &AnalysisPolicy::default());
    let v = drive(
        &p.doc,
        &analyzed,
        &DriveConfig {
            max_leaves: leaves,
            ..DriveConfig::default()
        },
        &pncad::document::Stop::default(),
        tol,
    )
    .expect("nominal builds");
    eprintln!("[probe] {which} s={s} leaves={leaves} receipt={:?}", v.receipt());
    let mut classes = std::collections::BTreeMap::new();
    for leaf in v.refused() {
        *classes.entry(format!("{:?}", leaf.reason.class())).or_insert(0usize) += 1;
    }
    eprintln!("[probe] classes {classes:?}");
    for leaf in v.refused().iter().take(3) {
        let text = format!("{:?}", leaf.reason);
        eprintln!("[probe] refused {}", text.chars().take(3000).collect::<String>());
    }
}
