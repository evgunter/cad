//! **M10-9's measurement bench** — the registered-identity door, on the
//! five documents whose ceilings it is supposed to move: the tour's
//! two-hole plate, R2's filleted bracket, R1's eccentric annulus,
//! R2's rounded-corner pad and R2's link.
//!
//! **What a bound IS, here**: the SET of predicates over the band at
//! the refusing end of a bisection, not the one name a drive reports
//! when it stops. The two differ, and M10-9's first cut was wrong
//! because of it — see `m10_8_harness::over_band_set`, the one home of
//! that instrument.
//!
//! Every row here is an `#[ignore]`d evidence probe that prints and
//! asserts nothing a gate could read ([[test-suite-cost]]); the
//! positive pins the measurement justifies live in
//! `m10_9_pins_interval.rs`, and the M10-8 harness
//! (`m10_8_harness::ceiling`) is the shared probe. Run them:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   m10_9_evidence_interval:: --ignored --nocapture
//! ```
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{certifies_whole, nominal_box};

/// A document as a function of the SCALE of its real study — the shape
/// every row here probes a ceiling with.
type Study<'a> = &'a dyn Fn(f64) -> ProfileDoc;

/// A named study, owned, so a list of them can be built in one place.
type NamedStudy = (&'static str, Box<dyn Fn(f64) -> ProfileDoc>);

/// A named study with the ceiling last measured for it.
type StudyAtCeiling<'a> = (&'static str, f64, Study<'a>);

/// The two rule sets this unit is a differential between: the shipped
/// tier with the registered-identity door open, and M10-8's exactly
/// (`SymRules::shipped_without_the_door`).
fn door_rows() -> [(&'static str, SymRules); 2] {
    [
        ("door OFF (M10-8)", SymRules::shipped_without_the_door()),
        ("door ON  (M10-9)", SymRules::shipped()),
    ]
}

/// The four documents, each as a function of the SCALE of its real
/// study, so a ceiling is a multiple of the study a user would ask for.
fn documents(tol: Tol) -> Vec<NamedStudy> {
    vec![
        (
            "two_hole_plate",
            Box::new(move |s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0)
                as Box<dyn Fn(f64) -> ProfileDoc>,
        ),
        (
            "r2_filleted_bracket",
            Box::new(move |s: f64| crate::m10_7_r2_probes_interval::bracket(s, tol).0),
        ),
        (
            "r1_annulus",
            Box::new(move |s: f64| crate::m10_8_r1_probes_interval::annulus(s, tol).0),
        ),
        (
            "r2_rounded_pad",
            Box::new(move |s: f64| crate::m10_8_r2_probes_interval::pad(s, tol).0),
        ),
        // R2's own arc document, adopted so the corrected diagnosis is
        // read on the same five documents both reviews measured.
        (
            "r2_link",
            Box::new(move |s: f64| crate::m10_9_r2_probes_interval::link(s, tol).0),
        ),
    ]
}

/// **The ceilings, door OFF and door ON, and THE OVER-BAND SET AT
/// CEILING + DELTA** — the bracket at both ends, and every predicate the
/// band could not classify at the tightest refusing scale the bisection
/// found, with its enclosure.
///
/// The last column is a SET and it is read at `hi`, the refusing end of
/// a 16-step log bisection, for the reason `m10_8_harness::over_band_set`
/// states: the first refusal a drive reports at a scale well PAST the
/// ceiling is an artefact of evaluation order, not the bound. Twelve
/// steps and a doubled scale — M10-9's first cut — were enough to be
/// wrong about which predicate bounds four of these five documents.
///
/// Run it once per ε row (`CAD_TOLERANCE_EPS=1e-6`, `1e-12`): the
/// tolerance is a `OnceLock`, so one process is one row. `CAD_M10_9_DOCS`
/// names a comma-separated subset (the bracket and the pad cost minutes
/// each under the shape report).
#[test]
#[ignore = "evidence-only: the ceilings, and the over-band set at ceiling + delta"]
fn m10_9_ceilings_with_and_without_the_door() {
    let tol = Tol::witness();
    let eps = tol.eps();
    println!("== eps = {eps:e}");
    let only = std::env::var("CAD_M10_9_DOCS")
        .ok()
        .filter(|s| !s.trim().is_empty());
    for (name, at) in documents(tol) {
        if only
            .as_deref()
            .is_some_and(|l| !l.split(',').any(|n| n.trim() == name))
        {
            continue;
        }
        for (label, rules) in door_rows() {
            let (lo, hi, per) =
                crate::m10_8_harness::ceiling(&*at, rules, tol, 1.0e-1 * eps, 1.0e6 * eps, 16);
            println!(
                "   {name:<20} {label}: certifies x{lo:e}, refuses x{hi:e} ({per:.2}s/probe) \
                 [= {:.3e}·eps .. {:.3e}·eps]",
                lo / eps,
                hi / eps
            );
            if !(lo.is_finite() && hi.is_finite()) {
                continue;
            }
            // AT CEILING + DELTA, and nowhere else: the set there is
            // the bound.
            let doc = at(hi);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let (shapes, _, counts) = replay(&doc, &ParamBox::of(&analyzed), rules, tol);
            println!("      at ceiling+delta ({:.4e}·eps): {counts:?}", hi / eps);
            println!(
                "{}",
                crate::m10_8_harness::render_over_band(&crate::m10_8_harness::over_band_set(
                    &shapes
                ))
            );
        }
    }
}

/// **The per-predicate split at each document's NOMINAL**, door OFF
/// against door ON: theorem / sign-gated / REGISTERED / numeric. Every
/// decide site decides at a point, so this is the complete table, and
/// it is where "the count is honest" is read — a `registered` column
/// that grows while `symbolic_zero` does not shrink.
#[test]
#[ignore = "evidence-only: prints the per-predicate discharge split at the nominal"]
fn m10_9_per_predicate_split_at_the_nominal() {
    let tol = Tol::witness();
    for (name, at) in documents(tol) {
        let doc = at(1.0);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let box_ = nominal_box(&analyzed);
        let mut table: BTreeMap<&'static str, [[u64; 4]; 2]> = BTreeMap::new();
        let mut totals = [[0u64; 4]; 2];
        for (col, (_, rules)) in door_rows().into_iter().enumerate() {
            let (shapes, _, counts) = replay(&doc, &box_, rules, tol);
            println!("== {name} {:?}", counts);
            for s in &shapes {
                let row = table.entry(s.predicate).or_default();
                let k = match s.outcome {
                    ShapeOutcome::Theorem => 0,
                    ShapeOutcome::SignGated => 1,
                    ShapeOutcome::Registered => 2,
                    _ => 3,
                };
                row[col][k] += 1;
                totals[col][k] += 1;
            }
        }
        for (pred, cols) in &table {
            println!(
                "   {pred:<34} off: {:?}  on: {:?}   (theorem/gated/registered/numeric)",
                cols[0], cols[1]
            );
        }
        println!(
            "   {:<34} off: {:?}  on: {:?}",
            "TOTAL", totals[0], totals[1]
        );
    }
}

/// **Claim 7 — what the door COSTS per leaf**, against M10-8's numbers
/// (plate 0.54 s, bracket 2.75 s): one whole-box replay of each
/// document at a scale it certifies with the door OFF, timed both ways.
#[test]
#[ignore = "evidence-only: prints the leaf cost with and without the door"]
fn m10_9_leaf_cost_with_and_without_the_door() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let scales = [
        ("two_hole_plate", 1.0e2 * eps),
        ("r2_filleted_bracket", 1.0e1 * eps),
        ("r1_annulus", 1.0e1 * eps),
        ("r2_rounded_pad", 1.0e-7),
    ];
    for (name, at) in documents(tol) {
        let scale = scales
            .iter()
            .find(|(n, _)| *n == name)
            .map_or(1.0e-7, |(_, s)| *s);
        let doc = at(scale);
        for (label, rules) in door_rows() {
            let t = std::time::Instant::now();
            let ok = certifies_whole(&doc, rules, tol);
            println!(
                "   {name:<20} x{scale:e} {label}: certifies_whole={ok} in {:.2}s",
                t.elapsed().as_secs_f64()
            );
        }
    }
}

/// **The fillet's obstacle, rendered — BOTH forms.** R2's pad authors
/// its four corners as `ProgramStep::Fillet(corner_r)`, so the tangency
/// `carrier_line_circle` decides is one the constructor declares; §2 of
/// the spec asks that, if the node-identity condition of §1 fails
/// there, the exact obstacle be filed WITH BOTH RENDERED FORMS. This
/// prints them.
///
/// The first is the consumer's: the residual `carrier_line_circle`
/// could not decide, read off the pad's own shape report
/// (`profile::seg::line_circle_joint`, `seg.rs:326-343`). The second is
/// the registrant's: what `fillet_arc_carrier` (`path.rs:2356`) holds
/// at the moment it guarantees the tangency, transcribed from those two
/// sites verbatim onto one corner's numbers, so the two forms can be
/// compared as forms and their NODE IDS as ids.
#[test]
#[ignore = "evidence-only: prints the fillet tangency residual's two rendered forms"]
fn m10_9_the_fillet_tangency_residual_rendered() {
    let tol = Tol::witness();
    let at = |s: f64| crate::m10_8_r2_probes_interval::pad(s, tol).0;
    for scale in [1.0e-7_f64, 1.0] {
        let doc = at(scale);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for (label, box_) in [
            ("nominal", nominal_box(&analyzed)),
            ("whole box", ParamBox::of(&analyzed)),
        ] {
            let (shapes, _, counts) = replay(&doc, &box_, SymRules::shipped(), tol);
            println!("== pad x{scale:e} {label}: {counts:?}");
            let mut seen = 0;
            for s in shapes
                .iter()
                .filter(|s| s.predicate == "carrier_line_circle")
            {
                seen += 1;
                if seen <= 2 {
                    println!(
                        "   CONSUMER [{:?}] {}",
                        s.outcome,
                        s.form.as_deref().unwrap_or("(decided; no form rendered)")
                    );
                }
            }
            println!("   ({seen} carrier_line_circle decisions)");
        }
    }
    the_two_fillet_forms();
}

/// The two sites' expressions, transcribed and rendered, on one
/// corner's numbers — the second half of the obstacle above.
///
/// **The consumer** (`profile::seg::line_circle_joint`, `seg.rs:335`)
/// decides `radius − |h|` with `h = line.unit.perp_dot(center −
/// line.a)`, where `line.unit` is the EMITTED leg's `chord / len`
/// (`seg.rs:118-130`) and `line.a` its start vertex.
///
/// **The registrant** (`profile::path::fillet_arc_carrier`,
/// `path.rs:2356`) holds `t2`, the arrival direction `u2` and the
/// radius, and builds `center = t2 + n̂·(σ·r)` with `n̂ = (−u2.y,
/// u2.x)`. The best `|h|` it can state is
/// `|u2.perp_dot(center − t2)| = |σ·r·(u2.x² + u2.y²)|`.
fn the_two_fillet_forms() {
    use geom_core::sym::report::{name_param, render_of};
    use geom_core::sym::with_session_rules;
    use geom_core::{Real, Sym, SymBudget, SymId};

    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    for n in ["r", "ax", "ay", "bx", "by"] {
        name_param(n);
    }
    let (_, counts) = with_session_rules(budget, SymRules::shipped(), || {
        let p = |n: &str, v: f64| Sym::param(geom_core::ParamSymbol::of(n), v);
        let (r, ax, ay, bx, by) = (
            p("r", 2.0e-3),
            p("ax", 0.0),
            p("ay", 0.0),
            p("bx", 4.0e-3),
            p("by", 3.0e-3),
        );
        // The leg, as the emitted segment builds it.
        let (dx, dy) = (bx - ax, by - ay);
        let len = (dx * dx + dy * dy).sqrt();
        let (ux, uy) = (dx / len, dy / len);
        // The registrant's own arrival direction: the SAME direction,
        // reached through the path algebra rather than through the
        // emitted chord. Here it is spelled the same way, which is the
        // most generous reading available to the door.
        let (u2x, u2y) = (ux, uy);
        // `fillet_arc_carrier`: center = t2 + n̂·(σ·r), t2 on the leg.
        let t = Sym::from_f64(0.75);
        let (t2x, t2y) = (ax + dx * t, ay + dy * t);
        let sgn = <Sym<f64> as Real>::one().copysign(<Sym<f64> as Real>::one());
        let (nx, ny) = (-u2y, u2x);
        let (cx, cy) = (t2x + nx * (sgn * r), t2y + ny * (sgn * r));
        // The CONSUMER's h: the emitted leg's unit against `center − a`.
        let h_consumer = ux * (cy - ay) - uy * (cx - ax);
        // The REGISTRANT's h: its own u2 against `center − t2`.
        let h_registrant = u2x * (cy - t2y) - u2y * (cx - t2x);
        let show = |what: &str, id: SymId| {
            println!(
                "   {what} node {:032x}\n      {}",
                id.bits(),
                render_of(id).unwrap_or_else(|| "(no session)".into())
            );
        };
        println!("== the two fillet forms, on one corner");
        show("CONSUMER |h|", h_consumer.abs().node());
        show("REGISTRANT |h|", h_registrant.abs().node());
        show("radius     r", r.node());
        println!(
            "   same node? {}",
            h_consumer.abs().node().bits() == h_registrant.abs().node().bits()
        );
        // And what the door answers when the registrant states it.
        println!(
            "   register(|h_registrant|, r) -> {:?}",
            h_registrant.abs().register_equal(r)
        );
        println!(
            "   register(|h_consumer|,   r) -> {:?}",
            h_consumer.abs().register_equal(r)
        );
    });
    println!("   counts {counts:?}");
}

/// **§4 — the ring width, measured not assumed.** The alternative to
/// the door was a wider coefficient ring: the plate's rim residual needs
/// ~640 bits and up, and the shipped bound is 256
/// (`geom_core::sym::COEFF_BITS`,
/// `work/m10/plate-rim-residual-needs-the-wide-coefficient-ring`).
///
/// `COEFF_BITS` is a compile-time constant, so this row is run three
/// times against three edited values (256, 1024, 4096) and the numbers
/// are transcribed into the PR body and the census; nothing here reads
/// the bound. What it prints, per document, is the cost of ONE leaf and
/// whether the whole-certifying ceiling moved off the 256-bit
/// measurement — half of it must certify and twice it must refuse, so
/// the answer is a bracket and not a point.
#[test]
#[ignore = "evidence-only: the COEFF_BITS ring table, one run per edited bound"]
fn m10_9_ring_table() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let rows: [StudyAtCeiling<'_>; 2] = [
        ("two_hole_plate", 7.787e2 * eps, &|s: f64| {
            crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0
        }),
        ("r2_filleted_bracket", 3.865e2 * eps, &|s: f64| {
            crate::m10_7_r2_probes_interval::bracket(s, tol).0
        }),
    ];
    for (name, ceiling, at) in rows {
        for (label, scale) in [("0.5x ceiling", 0.5), ("2x ceiling", 2.0)] {
            let doc = at(scale * ceiling);
            let t = std::time::Instant::now();
            let ok = certifies_whole(&doc, SymRules::shipped(), tol);
            println!(
                "   {name:<20} {label:<12} certifies_whole={ok} in {:.2}s (one leaf)",
                t.elapsed().as_secs_f64()
            );
        }
    }
}
