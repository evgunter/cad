//! **The continuation's session receipt does not move with the thread
//! count.**
//!
//! `SignCertificate::refine_to_target` resumes the faces its gate left
//! open. `sign_walk_plus_v` pins its refusal rule and the
//! piece-evaluation identity `gate + refine == one` at whatever width
//! the runner gives it; this suite reads the one channel a width-blind
//! row cannot compose back — the installed symbolic session's receipt
//! and shape report — at an EXPLICIT one and four threads.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, on_pool, stacked};
use geom_core::sym::{SymBudget, SymCounts, with_session};
use geom_core::{Sym, Tol};
use sweep::loft_body;
use topo::Body;

// ---------------------------------------------------------------- //
// The session arm: what a decision writes that NO splice can undo.   //
// ---------------------------------------------------------------- //

/// A symbolic session's budget for this row. Small: the point is that
/// the receipt COUNTS the continuation's decisions, not what the tier
/// proves.
fn budget() -> SymBudget {
    SymBudget {
        max_terms: 32,
        max_degree: 4,
    }
}

fn sym_arc_loft(s: f64) -> Body<Sym<f64>> {
    loft_body::<Sym<f64>>(
        &[arc_section(s), arc_section(s), arc_section(s)],
        &stacked(&[0.0, 1.0, 2.0], s),
        2,
        Tol::witness(),
    )
    .expect("the arc loft lofts at Sym")
    .body
}

fn counts_line(refused: bool, c: &SymCounts, shapes: usize) -> String {
    format!(
        "{} decisions={} sz={} sg={} reg={} rref={} rcon={} num={} frozen={} shapes={}",
        if refused { "REFUSED" } else { "OK" },
        c.decisions(),
        c.symbolic_zero,
        c.sign_gated,
        c.registered,
        c.registrations_refused,
        c.registrations_contradicted,
        c.numeric,
        c.frozen,
        shapes,
    )
}

/// The gate and its continuation under one installed session, at an
/// explicit pool width: what the session's receipt collected and how
/// long the shape report got.
fn session_reading(body: &Body<Sym<f64>>, threads: usize) -> String {
    let tol = Tol::witness();
    on_pool(threads, || {
        geom_core::sym::report::start_shape_report();
        let (refused, counts) =
            with_session(budget(), || {
                match topo::validate_geometric_certificate(body, tol) {
                    Ok(cert) => cert.refine_to_target().is_err(),
                    Err(_) => true,
                }
            });
        let shapes = geom_core::sym::report::take_shape_report().len();
        counts_line(refused, &counts, shapes)
    })
}

/// **The continuation stays on the caller's thread while a session is
/// installed**, and this is the row that says so.
///
/// `refine_to_target` has a serial arm of its own, taken when
/// `decisions_are_thread_portable` is false, and every other channel
/// the continuation reports is a value the walk hands back — so a parallel
/// resumption could compose it. These two cannot: `count_decision`
/// mutates the INSTALLED session in place and `sym::report::record`
/// pushes onto a thread-local, and a session is installed on the
/// CALLER only. A continuation that resumed its open faces on workers
/// would decide them with no session installed, so both numbers would
/// SHRINK at four threads while the answer itself changed — which is
/// what this row compares.
///
/// Two bodies, because the arm has two exits: one whose continuation
/// answers, and one whose schedule runs out so it returns at a refusal
/// with faces behind it still unresumed.
#[test]
fn the_session_receipt_does_not_move_with_the_thread_count() {
    let eps = Tol::witness().get().eps;
    for (label, body) in [
        ("sym_arc_loft_1e9eps", sym_arc_loft(1.0e9 * eps)),
        ("sym_arc_loft_1e11eps", sym_arc_loft(1.0e11 * eps)),
    ] {
        let one = session_reading(&body, 1);
        assert!(
            !one.contains("decisions=0"),
            "{label}: the session recorded no decision at all, so the comparison below is \
             vacuous — this roster no longer reaches the symbolic tier"
        );
        assert_eq!(
            one,
            session_reading(&body, 4),
            "{label}: the session's receipt or its shape report moved with the pool width, \
             so some face was decided off the caller's thread — where no session is \
             installed, the decision itself is the plain numeric one, and neither the \
             receipt nor the report can be composed back"
        );
        eprintln!("SESSION {label} {one}");
    }
}
