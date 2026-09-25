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

use crate::common::{arc_section, quad_verdicts, stacked, strip_section};
use geom_core::Tol;
use sweep::loft_body;
use topo::Body;

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
    // TWO scales, and the cost is why there are not three. Each member
    // is a certified quadrature over a rational-walled body and this
    // suite pays it three times over (gate, continuation, measurement);
    // a third scale at `1e7·eps` was measured and dropped, because its
    // split (8 + 4 = 12) says the same thing `1e9·eps`'s (8 + 7 = 15)
    // does. What the two that remain say and one alone could not: the
    // coarse one is the body whose schedule runs OUT, so the gate
    // passes where the number refuses, and the fine one is the body
    // whose schedule runs ON, so the gate and the continuation split
    // the rounds between them.
    [1.0e11, 1.0e9]
        .iter()
        .map(|k| (format!("arc loft @ {k:e}·eps"), arc_loft(k * eps)))
        .collect()
}

/// The thin strip lofted TWO stations high at scale `s`, its rectangle
/// `delta` thick — the body whose sign can be UNDECIDED.
///
/// Two stations at v-degree 1, not three at degree 2: the disposition
/// this row pins is identical either way (measured at all three ε) and
/// the cheaper body is half the build, which is where this row's cost
/// actually is.
fn strip_loft(s: f64, delta: f64, reversed: bool) -> Option<Body<f64>> {
    loft_body::<f64>(
        &[
            strip_section(s, delta, reversed),
            strip_section(s, delta, reversed),
        ],
        &stacked(&[0.0, 1.0], s),
        1,
        Tol::witness(),
    )
    .ok()
    .map(|l| l.body)
}

/// **UNDECIDED IS NOT A PASS** — the hole an early exit opens, and the
/// reason this unit's loop returns a VERDICT rather than a stopping
/// condition.
///
/// Check 7 refuses only on a definite disagreement, so an undecided
/// enclosure at the reporting target is a pass — the body's volume WAS
/// measured and simply sits inside the band, which is what `main`
/// does too. An undecided enclosure with the SCHEDULE RUN OUT is a
/// different thing: nothing was measured, the enclosure straddles
/// zero, and passing it admits an inside-out body on the strength of
/// a quadrature that never finished. That is the false ACCEPTANCE
/// mirroring the false refusal this unit removed.
///
/// **The invariant, in one line:** a body tier 3 ADMITS while a
/// target-level reading of it is REFUSED must have had its sign
/// decided — `volume_lo > 0`. Nothing else is asserted about which
/// bodies land where, because that is the fixture's property and not
/// the kernel's.
///
/// **The fixture** is `common::strip_section`'s thin curved strip:
/// two large rational walls whose fluxes nearly cancel, so the volume
/// is `≈ 2·s·delta` per unit height while the enclosure width is the
/// walls' own. At `s = 1e12·ε` the schedule is exhausted after round 0
/// at every ε — the family's disposition is a function of `delta/s`
/// alone, which is why two thicknesses pin the same two arms at every
/// ε row rather than going vacuous at one of them. Both traversal
/// senses are built: the reversed strip is the inside-out twin, and
/// an undecided sign cannot tell it from the upright one, which is
/// the whole reason an undecided sign may not pass.
///
/// A body that settles at a LATER round and passes is the roster row
/// below, whose `strictly_early` requirement is exactly that.
#[test]
fn an_undecided_sign_with_the_schedule_run_out_refuses() {
    let tol = Tol::witness();
    let s = 1.0e12 * tol.get().eps;
    let (mut admitted, mut refused) = (0usize, 0usize);
    for rel in [1e-1, 1e-3] {
        for reversed in [false, true] {
            let label = format!("strip rel={rel:e} reversed={reversed}");
            let body = strip_loft(s, rel * s, reversed)
                .unwrap_or_else(|| panic!("{label}: the strip lofts at every ε row"));
            let gated = topo::validate_geometric_certificate(&body, tol);
            let plain = topo::validate_geometric(&body, tol);
            assert_eq!(
                gated.is_ok(),
                plain.is_ok(),
                "{label}: the two tier-3 doors are one function"
            );
            let Ok(cert) = gated else {
                let errors = plain.expect_err("the doors agree");
                assert!(
                    errors
                        .iter()
                        .any(|e| matches!(e, topo::ValidationError::VolumeUncomputable { .. })),
                    "{label}: an undecided sign with the schedule run out is CHECK 7's \
                     refusal, the same one the reporting door makes: {errors:?}"
                );
                refused += 1;
                continue;
            };
            let e = cert.enclosure();
            if let Some(outstanding) = cert.target_refusal() {
                assert!(
                    e.volume_lo > 0.0,
                    "{label}: tier 3 admitted a body whose number is refused \
                     ({outstanding:?}) and whose volume enclosure [{}, {}] does not \
                     exclude zero — an undecided sign is not a pass",
                    e.volume_lo,
                    e.volume_hi
                );
                admitted += 1;
            }
        }
    }
    assert_eq!(
        (admitted, refused),
        (2, 2),
        "STRIP: the family must straddle the decision — two thicknesses admitted on a \
         decided sign with the number still refused (the false refusal this unit \
         removed), two refused on an undecided one (the false acceptance it must not \
         open), and each in both traversal senses"
    );
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

        // REUSE: no round run twice, none skipped — on EVERY roster
        // body, in the form that body can carry.
        eprintln!("ROUNDS {label}: gate {gate}, continuation {refine}, measurement {one}");
        assert!(
            gate > 0,
            "REUSE {label}: this roster is about the certified quadrature, so a body that \
             pays none of it is a body this row did not test"
        );
        if measured.is_ok() {
            assert_eq!(
                gate + refine,
                one,
                "REUSE {label}: the gate's {gate} verdicts plus the continuation's {refine} \
                 must be the measurement's {one} — a continuation that re-ran a round the \
                 gate already paid for would exceed it, one that skipped a round would \
                 fall short"
            );
            assert!(
                gate <= one,
                "REUSE {label}: on a body the measurement door ANSWERS the gate must never \
                 pay MORE rounds than it — {gate} against {one}"
            );
            if gate < one {
                strictly_early += 1;
            }
        } else {
            // THE REFUSING BODY, which is the body this unit exists
            // for and which the identity above cannot be stated on
            // unchanged: the measurement stops at the first face whose
            // schedule ran out, while the gate must read every face to
            // have a sum at all. What IS claimable, and is claimed:
            // the continuation adds nothing at all — every face the
            // gate read is already at the round it will die at — so
            // the pair costs exactly the gate, and the gate costs at
            // least the measurement because it read a superset of its
            // faces.
            assert_eq!(
                refine, 0,
                "REUSE {label}: the measurement refuses, so every face is already at the \
                 round its schedule ends on and the continuation has nothing to run — \
                 {refine} verdicts says it re-entered a lane"
            );
            assert!(
                gate >= one,
                "REUSE {label}: the gate reads every face and the measurement stops at its \
                 first refusing one, so the gate's {gate} cannot be under the \
                 measurement's {one}"
            );
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
