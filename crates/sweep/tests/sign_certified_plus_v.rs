//! **Tier 3's +V check certifies a SIGN, not a precision.**
//!
//! Check 7 reads a volume enclosure and refuses only on a definite
//! disagreement, so what it needs of a certified quadrature is that
//! the enclosure exclude zero — not that it reach the REPORTING
//! target `1024·ε`, which is a compromise cut for a caller who wants
//! the number. This suite is that distinction, from the outside:
//!
//! * **the level is the type** — a `SignCertificate` has a bracket and
//!   no volume, and the number is a continuation away;
//! * **the two doors agree** on every body both accept: the same sign,
//!   and the gate's bracket CONTAINS the measurement's value;
//! * **the false refusal is gone** — a valid solid whose schedule
//!   cannot reach `1024·ε` passes tier 3 and still refuses a caller
//!   who asks for its volume;
//! * **no round is run twice and none is skipped** — the gate's
//!   quadrature verdicts plus the continuation's are the
//!   measurement's, over a roster, and on at least one member the gate
//!   strictly stops early.
//!
//! **The roster is scaled, not sampled.** Every body here is built at
//! a multiple of the run's own ε where that matters, so each row
//! asserts the same thing at every ε the matrix gates rather than
//! going quietly vacuous at one of them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, stacked};
use geom_core::Tol;
use geom_core::k_stats::Bracket;
use sweep::loft_body;
use topo::Body;

/// The number of quadrature-lane classifications recorded while `run`
/// executed — the rounds a call actually paid for, counted rather than
/// timed. NOT `common::`: `tcost_k3_certificate` keeps its own copy
/// because it is the door-count row and this is the level row, and the
/// two suites are read separately.
fn quad_verdicts(run: impl FnOnce()) -> usize {
    let bracket = Bracket::open();
    run();
    bracket
        .finish()
        .verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("props_quad"))
        .count()
}

/// A three-station arc loft at scale `s` — rational walls, so every
/// wall face is a certified quadrature and none of them is a closed
/// form.
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

/// The roster: bodies at scales spanning the schedule's reach, so that
/// some certify comfortably and some cannot reach the target at all.
///
/// **One lane, and the reason.** These are the patch engine's bodies.
/// The cylinder chart's Green form needs a CONIC TRIM carrier, which
/// no sweep verb mints — an extrusion of a bulged profile carries an
/// analytic cylinder wall with an iso boundary, which is a CLOSED
/// FORM and pays no quadrature at all (`m8_3_rational_volume` uses
/// exactly that as its independent oracle). The bodies that reach the
/// cylinder lane are the boolean cuts, and they reach it through the
/// same `validate_geometric` this suite drives; what this roster
/// cannot claim is that it exercised that lane itself.
fn roster() -> Vec<(String, Body<f64>)> {
    let eps = Tol::witness().get().eps;
    [1.0e11, 1.0e9, 1.0e7]
        .iter()
        .map(|k| (format!("arc loft @ {k:e}·eps"), arc_loft(k * eps)))
        .collect()
}

/// **THE LEVEL / AGREEMENT / REUSE** — one walk of the roster, because
/// every row wants the same two calls on the same body and nextest is
/// process-per-test. Each assertion names its property.
#[test]
fn the_gate_certifies_a_sign_and_the_continuation_certifies_the_number() {
    let tol = Tol::witness();
    let mut strictly_early = 0usize;
    for (label, body) in roster() {
        // The gate. Tier 3 must ADMIT every body here: they are all
        // valid solids, and the only thing a tight ε can take from
        // them is the number.
        let mut gated = None;
        let gate = quad_verdicts(|| gated = Some(topo::validate_geometric_certificate(&body, tol)));
        let gated = gated
            .expect("the closure ran")
            .unwrap_or_else(|errors| panic!("LEVEL {label}: tier 3 must admit it: {errors:?}"));
        // THE LEVEL: the certificate is a BRACKET. A definite sign is
        // the claim; there is no volume field to misread as one.
        let enclosure = gated.enclosure();
        assert!(
            enclosure.volume_lo <= enclosure.volume_hi,
            "LEVEL {label}: a bracket runs low to high, got [{}, {}]",
            enclosure.volume_lo,
            enclosure.volume_hi
        );
        assert!(
            enclosure.volume_lo > 0.0,
            "LEVEL {label}: tier 3 admitted it, so its volume enclosure excludes zero from \
             above: [{}, {}]",
            enclosure.volume_lo,
            enclosure.volume_hi
        );

        // The continuation, and the measurement door beside it.
        let mut continued = None;
        let refine = quad_verdicts(|| continued = Some(gated.refine_to_target()));
        let continued = continued.expect("the closure ran");
        let mut measured = None;
        let one = quad_verdicts(|| measured = Some(topo::mass_properties(&body, tol)));
        let measured = measured.expect("the closure ran");

        // REUSE: no round run twice, none skipped. The claim is about
        // a body the measurement door ANSWERS: where it refuses it
        // stops at the first face whose schedule ran out, while the
        // gate must read every face to have a sum at all, so the two
        // counts are over different face sets and comparing them
        // would be comparing two different walks.
        eprintln!("ROUNDS {label}: gate {gate}, continuation {refine}, measurement {one}");
        assert!(
            gate > 0,
            "REUSE {label}: this roster is about the certified quadrature, so a body that \
             pays none of it is a body this row did not test"
        );
        if measured.is_ok() {
            assert!(
                gate <= one,
                "REUSE {label}: the gate must never pay MORE rounds than the measurement — \
                 {gate} against {one}"
            );
            assert_eq!(
                gate + refine,
                one,
                "REUSE {label}: the gate's {gate} verdicts plus the continuation's {refine} \
                 must be the measurement's {one}"
            );
            if gate < one {
                strictly_early += 1;
            }
        }

        // AGREEMENT: the two doors are the same quadrature at two
        // levels, so the continuation IS the measurement — and where
        // the measurement refuses, so does the continuation, with the
        // same typed refusal.
        match (&continued, &measured) {
            (Ok(c), Ok(m)) => {
                assert_eq!(
                    (
                        c.volume.to_bits(),
                        c.surface_area.to_bits(),
                        c.volume_pad.to_bits(),
                        c.area_pad.to_bits()
                    ),
                    (
                        m.volume.to_bits(),
                        m.surface_area.to_bits(),
                        m.volume_pad.to_bits(),
                        m.area_pad.to_bits()
                    ),
                    "AGREEMENT {label}: the continuation must BE the measurement, bit for bit"
                );
                // CONTAINMENT: the gate's bracket, taken at whatever
                // round its sign became certain, contains the value
                // the reporting door eventually computed.
                assert!(
                    enclosure.volume_lo <= m.volume && m.volume <= enclosure.volume_hi,
                    "AGREEMENT {label}: the sign-level bracket [{}, {}] must contain the \
                     reported volume {}",
                    enclosure.volume_lo,
                    enclosure.volume_hi,
                    m.volume
                );
                assert!(
                    m.volume > 0.0,
                    "AGREEMENT {label}: the gate certified a positive sign, so the number \
                     must be positive too, got {}",
                    m.volume
                );
            }
            (Err(c), Err(m)) => assert_eq!(
                format!("{c:?}"),
                format!("{m:?}"),
                "AGREEMENT {label}: both doors refuse, and with the same refusal"
            ),
            (c, m) => panic!(
                "AGREEMENT {label}: the continuation and the measurement are the same \
                 quadrature, so they cannot disagree about whether it converged: \
                 continuation {c:?} vs measurement {m:?}"
            ),
        }

        // FALSE REFUSAL: a body the schedule cannot measure is still a
        // body tier 3 admits. The roster's coarsest scales are there
        // to produce this arm; the assertion above (`tier 3 must admit
        // it`) is what carries it, and this is the record of which
        // rows exercised it.
        if continued.is_err() {
            eprintln!("FALSE-REFUSAL {label}: tier 3 passed, the number refused");
        }
    }
    assert!(
        strictly_early > 0,
        "REUSE: on at least one roster body the gate must stop STRICTLY before the \
         reporting target — otherwise the sign level is a name and not a level"
    );
}
