//! **The continuation's answer, and its refusal, do not move with the
//! thread count.**
//!
//! `SignCertificate::refine_to_target` resumes the faces its gate left
//! open and owes the certificate's own refusal rule: the face it names
//! is the FIRST refusing face in arena order, resumed or already
//! outstanding, and not merely a refusing one. `sign_walk_plus_v`
//! pins that rule and the piece-evaluation identity
//! `gate + refine == one` at whatever width the runner happens to give
//! it; this suite reads both at an EXPLICIT one and four threads,
//! because a continuation that decides its open faces on workers can
//! only keep the rule by ordering what it decided, and a width-blind
//! row cannot tell an ordered fold from a lucky schedule.
//!
//! Three properties, one walk of the roster per width (nextest is
//! process-per-test and every row wants the same three calls on the
//! same body):
//!
//! * **refusal parity** — the continuation and `mass_properties` agree
//!   about whether the body answers, and when they refuse they refuse
//!   with the SAME typed refusal, which names the same face;
//! * **piece-evaluation identity** — the gate's `props_quad_*` verdicts
//!   plus the continuation's are the measurement's on a body that
//!   answers, and on one that refuses the gate's alone are at least the
//!   measurement's, because the gate read every face where the
//!   measurement stopped at its first refusing one. **Nothing is
//!   claimed about the continuation's count on a refusing body**, and
//!   that is deliberate: a face's refusal is set by whichever window
//!   ran, so a body whose sign settles EARLY can leave faces open and
//!   then run out of schedule in the RESUMED window. This roster
//!   cannot reach one — every member's gate either exhausts the
//!   schedule (and refuses there) or settles with rounds to spare, and
//!   the arc-loft family was scanned at six scales between 1e12·ε and
//!   3e9·ε without landing between them — but the tour does, which is
//!   why `demos/tour`'s walk carries a `Measured::Bracket` arm for it;
//! * **bit identity across widths** — the number, or the refusal, is
//!   the same at one thread and at four.
//!
//! **The roster is scaled against the run's own ε** where that matters,
//! so no row goes vacuous at a point of the matrix: the coarse arc loft
//! is the body whose schedule runs OUT (the gate passes where the
//! number refuses), the fine one the body whose schedule runs ON (the
//! gate and the continuation split the rounds), `loft_prism` the
//! described-spline body, and `quintic_prism` the composite-round body
//! that is in BOTH regimes across the ε matrix — it answers at the two
//! loose rows and refuses at the tight one, and it is the roster's only
//! member with more than one face left open.
//!
//! **No thin strip**, though it is the obvious refusing fixture and the
//! sibling suites use it: a strip whose sign is undecided is refused by
//! TIER 3 itself (`sign_walk_plus_v`'s
//! `an_undecided_sign_with_the_schedule_run_out_refuses`), so there is
//! no certificate to continue from and nothing this suite can say about
//! it. What this suite needs is a body tier 3 ADMITS whose reporting
//! reading still refuses, which is the coarse arc loft.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, on_pool, quad_verdicts, quintic_prism, stacked};
use geom_core::sym::{SymBudget, SymCounts, with_session};
use geom_core::{Sym, Tol};
use sweep::loft_body;
use sweep::test_support::loft_prism;
use topo::Body;

fn arc_loft(s: f64) -> Body<f64> {
    loft_body::<f64>(
        &[arc_section(s), arc_section(s), arc_section(s)],
        &stacked(&[0.0, 1.0, 2.0], s),
        2,
        Tol::witness(),
    )
    .expect("the arc loft lofts")
    .body
}

fn roster() -> Vec<(String, Body<f64>)> {
    let eps = Tol::witness().get().eps;
    let mut out: Vec<(String, Body<f64>)> = [1.0e11, 1.0e9]
        .iter()
        .map(|k| (format!("arc_loft_{k:e}eps"), arc_loft(k * eps)))
        .collect();
    // Polyline sections, so `loft_prism`'s walls are described splines
    // on the quadrature lane — the lane whose continuation this row is
    // about, and the one the arc lofts above do not reach.
    out.push(("loft_prism".to_string(), loft_prism(Tol::witness())));
    out.push(("quintic_prism".to_string(), quintic_prism()));
    out
}

/// One body's reading at one width: the continuation's answer (bits, or
/// the typed refusal — which is where the refusing FACE is named) beside
/// the measurement's, and the three verdict counts.
fn reading(body: &Body<f64>) -> (String, String, usize, usize, usize) {
    let tol = Tol::witness();
    let mut gated = None;
    let gate = quad_verdicts(|| gated = Some(topo::validate_geometric_certificate(body, tol)));
    let gated = gated
        .expect("the closure ran")
        .unwrap_or_else(|errors| panic!("tier 3 must admit this roster: {errors:?}"));
    let mut continued = None;
    let refine = quad_verdicts(|| continued = Some(gated.refine_to_target()));
    let mut measured = None;
    let one = quad_verdicts(|| measured = Some(topo::mass_properties(body, tol)));
    (
        fold(&continued.expect("the closure ran")),
        fold(&measured.expect("the closure ran")),
        gate,
        refine,
        one,
    )
}

fn fold(r: &Result<topo::MassProperties<f64>, topo::MassPropsError>) -> String {
    match r {
        Ok(m) => format!(
            "v={:016x} a={:016x} vpad={:016x} apad={:016x}",
            m.volume.to_bits(),
            m.surface_area.to_bits(),
            m.volume_pad.to_bits(),
            m.area_pad.to_bits(),
        ),
        Err(e) => format!("REFUSED {e:?}"),
    }
}

#[test]
fn the_continuation_is_the_measurement_at_one_and_four_threads() {
    let mut strictly_early = 0usize;
    let mut refusing = 0usize;
    for (label, body) in roster() {
        let mut widths = Vec::new();
        for threads in [1usize, 4] {
            let (continued, measured, gate, refine, one) = on_pool(threads, || reading(&body));
            // PARITY: the same face, the same class, at this width.
            assert_eq!(
                continued, measured,
                "PARITY {label} @{threads}t: the continuation and the measurement are the \
                 same quadrature at two levels, so they answer the same thing — and where \
                 they refuse, they name the same face with the same refusal"
            );
            assert!(
                gate > 0,
                "{label} @{threads}t: a body that pays no certified quadrature is a body \
                 this row did not test"
            );
            // PIECES: no round run twice and none skipped.
            if measured.starts_with("REFUSED") {
                // Nothing is claimed about `refine` here, and that is
                // the correction this row carries: a face's refusal is
                // set by whichever window ran, so a body can reach its
                // budget refusal in the RESUMED window as easily as in
                // the gate's, and `refine == 0` was true of this
                // roster rather than of the door. What holds is that
                // the gate read every face where the measurement
                // stopped at its first refusing one.
                assert!(
                    gate >= one,
                    "PIECES {label} @{threads}t: the gate reads every face and the \
                     measurement stops at its first refusing one, so the gate's {gate} \
                     cannot be under the measurement's {one}"
                );
                refusing += 1;
            } else {
                assert_eq!(
                    gate + refine,
                    one,
                    "PIECES {label} @{threads}t: the gate's {gate} verdicts plus the \
                     continuation's {refine} must be the measurement's {one}"
                );
                if gate < one {
                    strictly_early += 1;
                }
            }
            eprintln!(
                "CONTINUATION {label} @{threads}t gate={gate} refine={refine} one={one} \
                 {}",
                if measured.starts_with("REFUSED") {
                    "REFUSED"
                } else {
                    "OK"
                }
            );
            widths.push((threads, continued, gate, refine, one));
        }
        let (_, ref first, gate, refine, one) = widths[0];
        for (threads, read, g, r, o) in &widths[1..] {
            assert_eq!(
                (read, *g, *r, *o),
                (first, gate, refine, one),
                "WIDTH {label}: the reading and the rounds at {threads} threads are not the \
                 ones at 1 thread"
            );
        }
    }
    assert!(
        strictly_early > 0,
        "no roster body left the gate strictly early, so no row exercised a continuation \
         that actually resumes a face"
    );
    assert!(
        refusing > 0,
        "no roster body refused, so no row exercised the refusal parity this suite is for"
    );
}

// ---------------------------------------------------------------- //
// The session arm: what a decision writes that NO splice can undo.   //
// ---------------------------------------------------------------- //

/// A symbolic session's budget for these rows. Small: the point is that
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
/// this suite reads is a value the walk hands back — so a parallel
/// resumption could compose it. These two cannot: `count_decision`
/// mutates the INSTALLED session in place and `sym::report::record`
/// pushes onto a thread-local, and a session is installed on the
/// CALLER only. A continuation that resumed its open faces on workers
/// would decide them with no session installed, so both numbers would
/// SHRINK at four threads while the answer itself changed — which is
/// what these rows compare.
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
