//! **The continuation's answer, and its refusal, do not move with the
//! thread count.**
//!
//! `SignCertificate::refine_to_target` resumes the faces its gate left
//! open and owes the REPORTING walk's rule: the face it names is the
//! FIRST refusing face in arena order, resumed or already outstanding,
//! and not merely a refusing one. `sign_certified_plus_v` pins that
//! rule and the piece-evaluation identity `gate + refine == one` at
//! whatever width the runner happens to give it; this suite reads both
//! at an EXPLICIT one and four threads, because a continuation that
//! decides its open faces on workers can only keep the rule by ordering
//! what it decided, and a width-blind row cannot tell an ordered fold
//! from a lucky schedule.
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
//!   answers, and the continuation runs no lane at all on one that does
//!   not;
//! * **bit identity across widths** — the number, or the refusal, is
//!   the same at one thread and at four.
//!
//! **The roster is scaled against the run's own ε** where that matters,
//! so no row goes vacuous at a point of the matrix: the coarse arc loft
//! is the body whose schedule runs OUT (the gate passes where the number
//! refuses), the fine one the body whose schedule runs ON (the gate and
//! the continuation split the rounds), `loft_prism` the described-spline
//! body, and the thin strip the one whose reading refuses outright.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, on_pool, quad, quad_verdicts, stacked};
use geom_core::Tol;
use sweep::loft_body;
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

/// `loft_prism`, rebuilt from the corpus document's own sections
/// (`editor-core/tests/corpus/loft_prism.rs`). Polyline sections, so
/// the walls are described splines on the quadrature lane.
fn loft_prism() -> Body<f64> {
    let sq = quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    let d = 0.375;
    let tr = quad([(-1.0 - d, -1.0), (1.0 + d, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    loft_body::<f64>(
        &[sq.clone(), tr, sq],
        &stacked(&[0.0, 1.0, 2.0], 1.0),
        2,
        Tol::witness(),
    )
    .expect("the prism lofts")
    .body
}

fn roster() -> Vec<(String, Body<f64>)> {
    let eps = Tol::witness().get().eps;
    let mut out: Vec<(String, Body<f64>)> = [1.0e11, 1.0e9]
        .iter()
        .map(|k| (format!("arc_loft_{k:e}eps"), arc_loft(k * eps)))
        .collect();
    out.push(("loft_prism".to_string(), loft_prism()));
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
    let fold = |r: &Result<topo::MassProperties<f64>, topo::MassPropsError>| match r {
        Ok(m) => format!(
            "v={:016x} a={:016x} vpad={:016x} apad={:016x}",
            m.volume.to_bits(),
            m.surface_area.to_bits(),
            m.volume_pad.to_bits(),
            m.area_pad.to_bits(),
        ),
        Err(e) => format!("REFUSED {e:?}"),
    };
    (
        fold(&continued.expect("the closure ran")),
        fold(&measured.expect("the closure ran")),
        gate,
        refine,
        one,
    )
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
                assert_eq!(
                    refine, 0,
                    "PIECES {label} @{threads}t: the measurement refuses, so every face is \
                     already at the round its schedule ends on and the continuation has \
                     nothing to run — {refine} verdicts says it re-entered a lane"
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
