//! The refinement loop's four faces, pinned from outside the module
//! over a corpus rather than one instance each: every refusal the
//! corpus reaches keeps its face's own payload invariants — the two
//! faces that carry a bound carry a finite one, the cap face never
//! fires on the round budget's last round (the budget test precedes
//! the marking), `rounds` never exceeds the budget, a not-finite face
//! carries the last finite bound or none — and every message names the
//! lever its face's doc claims.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_brep::offset_fit::{
    OFFSET_FIT_BUDGET, OFFSET_FIT_SAMPLE_CAP, OffsetFitError, fit_offset_at,
};

use crate::shared::fixture::{bumpy_patch, quarter_cylinder};
use crate::shared::tol::band;

/// Every refusal the corpus reaches, with its face's own invariants.
#[test]
fn every_budget_face_keeps_its_own_payload_invariants() {
    let bases: Vec<(&str, NurbsSurface<f64>)> = vec![
        ("quarter_cylinder", quarter_cylinder(1.0, 1.0)),
        ("bumpy", bumpy_patch()),
    ];
    let ds = [1e-8_f64, 1e-7, 1e-6, 1e-5, 0.01, 0.05, -0.05];
    let tols = [1e-15_f64, 1e-12, 1e-9, 1e-6, 1e-3];
    // budget, cap, not-finite with no finite round, not-finite with a
    // lost finite round, stalled.
    let mut seen = [0u32; 5];
    for (name, base) in &bases {
        for d in ds {
            for t in tols {
                match fit_offset_at(base, d, t, band()) {
                    Ok(_) => {}
                    Err(OffsetFitError::BudgetExhausted {
                        budget,
                        grid,
                        achieved,
                        tolerance,
                    }) => {
                        seen[0] += 1;
                        assert_eq!(budget, OFFSET_FIT_BUDGET, "{name} d={d} t={t}");
                        assert!(
                            achieved.is_finite(),
                            "{name} d={d} t={t}: BudgetExhausted carries a non-finite bound"
                        );
                        assert!(achieved > tolerance, "{name} d={d} t={t}");
                        assert!(
                            grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP,
                            "{name} d={d} t={t}: grid past the cap"
                        );
                    }
                    Err(OffsetFitError::SampleCapReached {
                        cap,
                        rounds,
                        grid,
                        achieved,
                        tolerance,
                    }) => {
                        seen[1] += 1;
                        assert_eq!(cap, OFFSET_FIT_SAMPLE_CAP, "{name} d={d} t={t}");
                        assert!(
                            achieved.is_finite(),
                            "{name} d={d} t={t}: SampleCapReached carries a non-finite bound"
                        );
                        assert!(achieved > tolerance, "{name} d={d} t={t}");
                        // THE TIE: the budget test runs before the
                        // marking, so a cap stop can never be the
                        // budget's last round.
                        assert!(
                            (rounds as usize) < OFFSET_FIT_BUDGET,
                            "{name} d={d} t={t}: the cap face fired on the budget's last round \
                             (rounds={rounds}, budget={OFFSET_FIT_BUDGET})"
                        );
                        assert!(
                            grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP,
                            "{name} d={d} t={t}"
                        );
                    }
                    Err(OffsetFitError::BoundNotFinite {
                        rounds,
                        grid,
                        d: dd,
                        tolerance,
                        last_finite,
                    }) => {
                        assert_eq!(dd, d, "{name} d={d} t={t}");
                        assert_eq!(tolerance, t, "{name} d={d} t={t}");
                        assert!(
                            (rounds as usize) <= OFFSET_FIT_BUDGET,
                            "{name} d={d} t={t}: rounds past the budget"
                        );
                        assert!(
                            grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP,
                            "{name} d={d} t={t}"
                        );
                        match last_finite {
                            None => seen[2] += 1,
                            Some(b) => {
                                seen[3] += 1;
                                assert!(
                                    b.is_finite() && b > tolerance,
                                    "{name} d={d} t={t}: a lost bound of {b} is not a bound \
                                     the loop could have refused on"
                                );
                            }
                        }
                    }
                    Err(OffsetFitError::RefinementStalled { rounds, .. }) => {
                        seen[4] += 1;
                        assert!(
                            (rounds as usize) <= OFFSET_FIT_BUDGET,
                            "{name} d={d} t={t}: rounds past the budget"
                        );
                    }
                    Err(_) => {}
                }
            }
        }
    }
    eprintln!(
        "faces over the corpus: budget={} cap={} not_finite(none)={} not_finite(lost)={} \
         stalled={}",
        seen[0], seen[1], seen[2], seen[3], seen[4]
    );
    assert!(
        seen[1] > 0 && seen[2] > 0,
        "the corpus reached neither the cap face nor the not-finite face; the row proves nothing"
    );
}

/// Each message names the lever the face's doc claims, and no other.
#[test]
fn each_faces_message_names_its_lever() {
    let b = OffsetFitError::BudgetExhausted {
        budget: OFFSET_FIT_BUDGET,
        grid: (9, 9),
        achieved: 1e-3,
        tolerance: 1e-9,
    }
    .to_string();
    assert!(b.contains("OFFSET_FIT_BUDGET"), "{b}");
    assert!(!b.contains("OFFSET_FIT_SAMPLE_CAP"), "{b}");
    let c = OffsetFitError::SampleCapReached {
        cap: OFFSET_FIT_SAMPLE_CAP,
        rounds: 5,
        grid: (41, 40),
        achieved: 1e-3,
        tolerance: 1e-9,
    }
    .to_string();
    assert!(c.contains("OFFSET_FIT_SAMPLE_CAP"), "{c}");
    assert!(
        c.contains(&format!("5 of {OFFSET_FIT_BUDGET} rounds")),
        "{c}"
    );
    let n = OffsetFitError::BoundNotFinite {
        rounds: 4,
        grid: (25, 17),
        d: 1e-7,
        tolerance: 1e-3,
        last_finite: None,
    }
    .to_string();
    assert!(
        n.contains("without any round producing a finite sup bound"),
        "{n}"
    );
    assert!(
        n.contains("neither the round budget nor the sample cap"),
        "{n}"
    );
    // The never-finite message names no constant a caller could raise.
    assert!(!n.contains("OFFSET_FIT_"), "{n}");
    let l = OffsetFitError::BoundNotFinite {
        rounds: 4,
        grid: (25, 17),
        d: 1e-7,
        tolerance: 1e-3,
        last_finite: Some(3.2e-4),
    }
    .to_string();
    assert!(l.contains("the schedule is the lever"), "{l}");
    assert!(
        l.contains("0.00032 m"),
        "the lost bound is not in the message: {l}"
    );
    assert!(!l.contains("OFFSET_FIT_"), "{l}");
    assert_ne!(n, l, "the two not-finite cases must read differently");
}
