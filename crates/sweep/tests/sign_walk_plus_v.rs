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
//!   the gate's bracket CONTAINS the measurement's value, and the
//!   continued number is never wider than the measurement's;
//! * **the false refusal is gone** — a valid solid whose schedule
//!   cannot reach `1024·ε` passes tier 3 and still refuses a caller
//!   who asks for its volume;
//! * **no round is run twice** — the gate's rounds plus the
//!   continuation's are at least the measurement's, and are exactly the
//!   measurement's, with the measurement's bits, wherever the gate took
//!   no face past the round the number stops it at; on at least one
//!   member the gate strictly stops early.
//!
//! **The roster is scaled, not sampled.** Every body here is built at
//! a multiple of the run's own ε where that matters, so each row
//! asserts the same thing at every ε the matrix gates rather than
//! going quietly vacuous at one of them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, quad_rounds, stacked, strip_section};
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

/// **A deciding predicate run out at the wall is THE refusal, and it
/// carries the enclosure** (`[tolerance.measure]`; PR 5741's review,
/// n6). The thin strip of the row below, at `delta/s = 1e-10`: every
/// wall runs to its work wall with the enclosure still straddling the
/// strip's own volume, so `V ≤ V_true` cannot be decided. The door
/// answers [`topo::MassPropsError::Undecided`] — not `Ok` with a stop
/// and no verdict, which a caller reading only the verdict would take
/// for a decision — and its bracket holds the closed form
/// `V = 2·s·delta·s`. Through the raw door: tier 3 refuses this body
/// (check 7's own undecided sign), which is the row below.
#[test]
fn a_relation_run_out_at_the_wall_is_the_refusal_carrying_the_bracket() {
    let tol = Tol::witness();
    let s = 1.0e14 * tol.get().eps;
    let delta = 1e-10 * s;
    let body = strip_loft(s, delta, false).expect("the strip lofts at every ε row");
    let truth = 2.0 * s * delta * s;
    let asked = topo::Settle::AtMost {
        bound: truth,
        name: "sign_walk_plus_v_undecided",
    };
    // Raw door: tier 3 refuses the strip (VolumeSignUnresolved), and the measurement's own refusal is the subject.
    match topo::test_support::measure_unvalidated(&body, tol, asked) {
        Err(topo::MassPropsError::Undecided {
            volume: (lo, hi),
            stop,
        }) => {
            assert!(
                matches!(stop, topo::Stop::Wall | topo::Stop::Floor),
                "the walk stops at a face's own stop: {stop:?}"
            );
            assert!(
                lo <= truth && truth <= hi,
                "the refusal's bracket [{lo:e}, {hi:e}] holds V = {truth:e}"
            );
        }
        other => panic!("V ≤ V_true is undecided at the wall, so it is the refusal: {other:?}"),
    }
}

/// **UNDECIDED IS NOT A PASS** — the hole an early exit opens, and the
/// reason this unit's loop returns a VERDICT rather than a stopping
/// condition.
///
/// Check 7 refuses on a definite disagreement, and the sign walk
/// refines every face its lane can refine while the enclosure straddles
/// zero, the number's resolution no stopping point for it. An
/// undecided enclosure with the SCHEDULE RUN OUT is then what is left:
/// nothing was measured, the enclosure straddles zero, and passing it
/// admits an inside-out body on the strength of a quadrature that never
/// finished. That is the false ACCEPTANCE
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
/// walls' own. At `s = 1e14·ε` no wall reaches the reporting target
/// at any ε (each stops at the work wall after 11 rounds, 2.489e-5 m
/// against 1.024e-6 m at ε = 1e-9), so the sign walk runs the schedule
/// out wherever the sign stays open: at `delta/s = 1e-1` it settles and
/// the number is a bracket, and at `1e-10` it does not settle and check 7
/// refuses `VolumeSignUnresolved` (measured at 1e-9; `1e-8` settles, so
/// the refusing arm sits two decades past it). The scale was `1e12·ε`
/// and the thin arm `1e-8` while every lane stopped at its round cap
/// and refused a face at round 0 on a predicted last round: the walls
/// now refine to the work wall, which certifies the `1e12·ε` strip's
/// number and settles its `1e-8` sign.
/// The family's disposition is a function of `delta/s` alone, which is
/// why two thicknesses pin the same two arms at every ε row rather than
/// going vacuous at one of them. Both traversal senses are built; the
/// profile canonicalizes traversal (`profile`'s "winding is invisible"),
/// so both are the one positive body and each must land as the other
/// does.
///
/// A body that settles at a LATER round and passes is the roster row
/// below, whose `strictly_early` requirement is exactly that.
#[test]
fn an_undecided_sign_with_the_schedule_run_out_refuses() {
    let tol = Tol::witness();
    let s = 1.0e14 * tol.get().eps;
    let (mut admitted, mut refused) = (0usize, 0usize);
    for rel in [1e-1, 1e-10] {
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
                // The refusal carries the enclosure the walk stopped on,
                // and it holds the strip's closed form: the two bulges
                // cancel, so V = 2·s·delta·s (the loft is `s` tall). The
                // profile canonicalizes traversal, so both senses build
                // this one body.
                let truth = 2.0 * s * (rel * s) * s;
                let bracket = errors.iter().find_map(|e| match e {
                    topo::ValidationError::VolumeSignUnresolved {
                        limit: topo::SignLimit::Quadrature,
                        volume,
                        ..
                    } => *volume,
                    _ => None,
                });
                let (lo, hi) = bracket.unwrap_or_else(|| {
                    panic!(
                        "{label}: a sign still open at the wall is CHECK 7's quadrature \
                         refusal: {errors:?}"
                    )
                });
                assert!(
                    lo <= truth && truth <= hi,
                    "{label}: the refusal's bracket [{lo:e}, {hi:e}] holds V = {truth:e}"
                );
                refused += 1;
                continue;
            };
            let e = cert.enclosure();
            let measured = cert
                .measure(topo::test_support::pinned_resolution(tol))
                .unwrap_or_else(|refusal| panic!("{label}: the certificate measures: {refusal}"));
            assert_ne!(
                measured.stop,
                topo::Stop::Met,
                "{label}: at s = 1e14·ε no wall reaches the resolution, yet the number \
                 measured: {measured:?}"
            );
            assert!(
                e.volume_lo > 0.0,
                "{label}: tier 3 admitted a body whose number is open and whose volume \
                 enclosure [{}, {}] does not exclude zero — an undecided sign is not a pass",
                e.volume_lo,
                e.volume_hi
            );
            admitted += 1;
        }
    }
    assert_eq!(
        (admitted, refused),
        (2, 2),
        "STRIP: the family must straddle the decision — two thicknesses admitted on a \
         decided sign with the number still open, two refused on an undecided one (the \
         false acceptance it must not open), and each in both traversal senses"
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
        let gate = quad_rounds(|| gated = Some(topo::validate_geometric_certificate(&body, tol)));
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
        let refine = quad_rounds(|| {
            continued = Some(gated.measure(topo::test_support::pinned_resolution(tol)));
        });
        let continued = continued
            .expect("the closure ran")
            .unwrap_or_else(|e| panic!("{label}: the continuation measures: {e}"));
        // Finished outside the count: the count is the one measurement's.
        let finished =
            topo::test_support::finished("the measured body", topo::Body::clone(&body), tol);
        let mut measured = None;
        let one = quad_rounds(|| {
            measured = Some(topo::mass_properties(
                &finished,
                tol,
                topo::test_support::pinned_resolution(tol),
            ))
        });
        let measured = measured
            .expect("the closure ran")
            .unwrap_or_else(|e| panic!("{label}: the measurement measures: {e}"));

        // REUSE: none skipped — on EVERY roster body, in the form that
        // body can carry.
        eprintln!("ROUNDS {label}: gate {gate}, continuation {refine}, measurement {one}");
        assert!(
            gate > 0,
            "REUSE {label}: this roster is about the certified quadrature, so a body that \
             pays none of it is a body this row did not test"
        );
        if measured.stop == topo::Stop::Met {
            // The sign walk refines every face its lane can refine while
            // the sign straddles, past the number's resolution included,
            // so the pair can run MORE rounds than the measurement; never
            // fewer, because the continuation resumes every face still
            // short of the resolution from the round after its last.
            assert!(
                gate + refine >= one,
                "REUSE {label}: the gate's {gate} rounds plus the continuation's {refine} \
                 cannot fall short of the measurement's {one} — a continuation that \
                 skipped a round would"
            );
            if gate < one {
                strictly_early += 1;
            }
        } else {
            // THE BODY SHORT OF THE RESOLUTION, which is the body this
            // unit exists for: every face runs to its own stop, so the
            // pair still runs no fewer rounds than the measurement.
            assert!(
                gate + refine >= one,
                "REUSE {label}: the gate's {gate} rounds plus the continuation's {refine} \
                 cannot fall short of the measurement's {one}"
            );
        }

        // AGREEMENT: the two doors are the same quadrature at two
        // levels, so where both measure, the continuation is never wider
        // than the measurement and is the measurement wherever it ran the
        // measurement's rounds; and where the measurement stops short,
        // so does the continuation.
        let c = &continued.enclosure;
        let m = &measured;
        match (continued.stop, measured.stop) {
            (topo::Stop::Met, topo::Stop::Met) => {
                assert!(
                    c.volume_pad <= m.enclosure.volume_pad,
                    "AGREEMENT {label}: the continuation keeps every face's narrowest \
                     enclosure, so its pad {:e} is never over the measurement's {:e}",
                    c.volume_pad,
                    m.enclosure.volume_pad
                );
                if gate + refine == one {
                    assert_eq!(
                        (
                            c.volume.to_bits(),
                            c.surface_area.to_bits(),
                            c.volume_pad.to_bits(),
                            c.area_pad.to_bits()
                        ),
                        (
                            m.enclosure.volume.to_bits(),
                            m.enclosure.surface_area.to_bits(),
                            m.enclosure.volume_pad.to_bits(),
                            m.enclosure.area_pad.to_bits()
                        ),
                        "AGREEMENT {label}: the gate took no face past the round the number \
                         stops it at, so the continuation ran the measurement's rounds and \
                         must BE the measurement, bit for bit"
                    );
                }
                // CONTAINMENT: the gate's bracket, taken at whatever
                // round its sign became certain, contains the value
                // the reporting door eventually computed.
                assert!(
                    enclosure.volume_lo <= m.enclosure.volume
                        && m.enclosure.volume <= enclosure.volume_hi,
                    "AGREEMENT {label}: the sign-level bracket [{}, {}] must contain the \
                     reported volume {}",
                    enclosure.volume_lo,
                    enclosure.volume_hi,
                    m.enclosure.volume
                );
                assert!(
                    m.enclosure.volume > 0.0,
                    "AGREEMENT {label}: the gate certified a positive sign, so the number \
                     must be positive too, got {}",
                    m.enclosure.volume
                );
            }
            (topo::Stop::Floor | topo::Stop::Wall, topo::Stop::Floor | topo::Stop::Wall) => {
                assert!(
                    c.volume_pad <= m.enclosure.volume_pad,
                    "AGREEMENT {label}: the continuation's bracket is never wider than the \
                     measurement's"
                );
            }
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
        if continued.stop != topo::Stop::Met {
            eprintln!("FALSE-REFUSAL {label}: tier 3 passed, the number is a bracket");
        }
    }
    assert!(
        strictly_early > 0,
        "REUSE: on at least one roster body the gate must stop STRICTLY before the \
         reporting target — otherwise the sign level is a name and not a level"
    );
}

/// **THE NARROWER BRACKET HOLDS THE CLOSED FORM** — where the sign walk
/// refined a face PAST the resolution, [`topo::SignCertificate::measure`]
/// keeps that face's narrower enclosure (a face run's one transition
/// keeps the narrower of two), while the reporting door stops every face at the
/// first round meeting the resolution. The two doors then differ in the
/// bits of exactly the faces the sign walk took further: here the
/// strip's two walls, whose fluxes nearly cancel, so deciding the sign
/// needs a width far under the volume `2·s²·delta` while the number
/// only needs the resolution. The row pins that the narrower bracket is
/// a true one: it is strictly narrower, and it still holds the closed
/// form `V = 2·s·δ·h` (`h = s`, two stations `s` apart).
///
/// The body is one of the PR 5628 review's (R1's
/// `review_5628_continuation_vs_measurement`, `k = 1e6`, `δ/s = 1e-4`),
/// where the continuation stopped being `mass_properties` bit for bit.
#[test]
fn the_sign_walks_narrower_bracket_holds_the_closed_form() {
    let tol = Tol::witness();
    let s = 1.0e6 * tol.get().eps;
    let delta = 1.0e-4 * s;
    let want = 2.0 * s * s * delta;
    let body = strip_loft(s, delta, false).expect("the strip lofts at every ε row");
    let cert = topo::validate_geometric_certificate(&body, tol)
        .unwrap_or_else(|errors| panic!("tier 3 admits the strip: {errors:?}"));
    let reported = topo::mass_properties(
        &topo::test_support::finished("the strip", body.clone(), tol),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap_or_else(|e| panic!("the reporting door measures the strip: {e}"));
    let measured = cert
        .measure(topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("the certificate measures the strip: {e}"));
    assert!(
        measured.enclosure.volume_pad < reported.enclosure.volume_pad,
        "the sign walk refined the walls past the resolution, so the certificate's pad \
         {:e} must be under the reporting door's {:e}",
        measured.enclosure.volume_pad,
        reported.enclosure.volume_pad
    );
    for (door, p) in [("certificate", &measured), ("reporting door", &reported)] {
        assert!(
            (p.enclosure.volume - want).abs() <= p.enclosure.volume_pad,
            "the {door}'s bracket {:e} ± {:e} must hold the closed form {want:e}",
            p.enclosure.volume,
            p.enclosure.volume_pad
        );
    }
}

/// The thin strip of [`strip_loft`], translated `dx·s` along `+x`: a
/// solid that shares a body with one at the origin without touching it.
fn strip_loft_at(s: f64, delta: f64, dx: f64) -> Body<f64> {
    loft_body::<f64>(
        &[
            strip_section(s, delta, false),
            strip_section(s, delta, false),
        ],
        &[0.0, 1.0]
            .map(|h| profile::SketchPlane::xy_at(geom_core::Point3::new(dx * s, 0.0, h * s))),
        1,
        Tol::witness(),
    )
    .expect("the translated strip lofts")
    .body
}

/// **THE SIGN WALK TAKES EVERY OPEN SOLID PAST THE NUMBER'S
/// RESOLUTION, IN ONE WALK.** The strip at `s = 1e6·ε`, `δ/s = 1e-4`
/// (the PR 5628 review's, R1; [`the_sign_walks_narrower_bracket_holds_the_closed_form`]'s
/// body) holds a volume `2·s²·δ` under its walls' width at `1024·ε`:
/// its sign is decided only by refining the walls past the round the
/// number stops them at, so a walk that stops refining a face once it
/// meets the number's resolution refuses it.
///
/// Two copies grafted as two solids are one check-7 walk over the solids
/// ([`topo::validate_geometric_certificate`]). Each solid is refined, and
/// its role read, only while its own sign is open, so the pair records
/// exactly the two singles' role reads under `positive_volume_exact`
/// (an escalation each round its sign is open, a verdict when it is
/// decided): a
/// walk that left one solid waiting while another refined would read the
/// waiting one again at every round it waited, and one that refined a
/// decided solid would read it again too.
#[test]
fn the_sign_walk_takes_every_open_solid_past_the_numbers_resolution() {
    let tol = Tol::witness();
    let s = 1.0e6 * tol.get().eps;
    let delta = 1.0e-4 * s;
    let reads = |label: &str, body: &Body<f64>| {
        let bracket = geom_core::k_stats::Bracket::open();
        let gated = topo::validate_geometric_certificate(body, tol);
        let recorded = bracket.finish();
        if let Err(errors) = gated {
            panic!(
                "{label}: the strip's sign is decided past the number's resolution, so tier 3 \
                 admits it: {errors:?}"
            );
        }
        // A round that leaves the sign open records an escalation, the
        // round that decides it a verdict: both are reads.
        let name = "positive_volume_exact";
        recorded
            .verdicts
            .iter()
            .filter(|v| v.predicate == name)
            .count()
            + recorded
                .escalations
                .iter()
                .filter(|e| e.predicate() == name)
                .count()
    };
    let (near, far) = (strip_loft_at(s, delta, 0.0), strip_loft_at(s, delta, 4.0));
    let alone = reads("the strip", &near) + reads("its twin", &far);
    let mut pair = Body::new();
    for solid in [&near, &far] {
        topo::graft_disjoint(&mut pair, solid).expect("a disjoint graft");
    }
    let together = reads("the pair", &pair);
    assert!(
        alone > 2,
        "each strip's sign is read past round 0: {alone} reads"
    );
    assert_eq!(
        together, alone,
        "one walk over two solids reads each solid's role at the rounds it reads alone"
    );
}

/// **THE BACKSTOP STARTS FROM ROUND 0.** An intersection planted with
/// the arc loft at `1e9·ε` as its result, inside one brick taken as both
/// operands: every bound the backstop asks (the result at most each
/// operand, and at least zero) holds on the result's round-0 enclosure,
/// so the backstop reads each of the result's quadrature faces once and
/// no further. The number door runs the same faces to the resolution,
/// rounds past round 0 that a backstop reading from the reporting walk
/// would pay.
#[test]
fn the_backstop_reads_from_round_zero() {
    let tol = Tol::witness();
    let s = 1.0e9 * tol.get().eps;
    let result = arc_loft(s);
    let brick: Body<f64> = sweep::test_support::brick(
        (-10.0 * s, 10.0 * s),
        (-10.0 * s, 10.0 * s),
        (-10.0 * s, 10.0 * s),
        tol,
    );
    let number = quad_rounds(|| {
        topo::mass_properties(
            &topo::test_support::finished("the loft", result.clone(), tol),
            tol,
            topo::test_support::pinned_resolution(tol),
        )
        .expect("the loft measures");
    });
    // Every quadrature face is a curved one: the loft's planar caps are
    // closed forms.
    let curved = result
        .faces()
        .filter(|(_, face)| {
            !matches!(
                result.get_surface(face.surface),
                Some(geom::Surface::Plane { .. })
            )
        })
        .count();
    let backstop = quad_rounds(|| {
        topo::test_support::volume_backstop(
            topo::BooleanOp::Intersect,
            &brick,
            &brick,
            &result,
            tol,
        )
        .expect("every bound holds");
    });
    assert!(
        backstop > 0,
        "the loft has a quadrature wall, so its round 0 runs one"
    );
    assert!(
        backstop <= curved,
        "the backstop reads each quadrature face once: {backstop} rounds over {curved} curved \
         faces"
    );
    assert!(
        backstop < number,
        "the backstop reads round 0 and the number door refines past it: {backstop} rounds \
         against the number's {number}"
    );
}

/// **A DECIDING PREDICATE WALKS A QUADRATURE FACE PAST ROUND 0** — the
/// measurement door (`[tolerance.measure]`, through
/// [`topo::test_support::measure_to`]) asked a relation of the two-section
/// arc loft at `1e7·ε`, whose rational wall is a quadrature face. The
/// posture is the scale's alone, so it holds at every ε.
///
/// - A bound ten times the volume SETTLES at round 0: its enclosure is
///   the reference round-0 width.
/// - A bound `1e-5` of the volume above it straddles at round 0, so the
///   walk REFINES, and then settles `≤` as holding on a narrower
///   enclosure.
/// - The volume itself is the threshold: the walk refines while the
///   enclosure straddles the band and STOPS IN IT (`Stop::Band`) on a
///   narrower enclosure, where no narrower one decides it.
///
/// Each ask is recorded under the relation's name, so the walks that
/// refine record more of it than the one that settles at round 0.
#[test]
fn a_relation_refines_a_quadrature_face_and_settles_or_stops_in_the_band() {
    use topo::{Settle, SettleVerdict, Stop};
    let tol = Tol::witness();
    let s = 1.0e7 * tol.get().eps;
    let body = loft_body::<f64>(
        &[arc_section(s), arc_section(s)],
        &stacked(&[0.0, 1.0], s),
        1,
        tol,
    )
    .expect("the arc loft lofts")
    .body;
    let v = topo::test_support::pinned_volume(
        "the arc loft measures",
        &topo::test_support::finished("the loft", body.clone(), tol),
        tol,
    )
    .volume;
    let name = "sign_walk_plus_v_relation";
    let ask = |bound: f64| {
        let bracket = geom_core::k_stats::Bracket::open();
        let measured = topo::test_support::measure_to(&body, Settle::AtMost { bound, name }, tol)
            .unwrap_or_else(|refusal| panic!("the door measures the loft: {refusal}"));
        let recorded = bracket.finish();
        let asked = recorded
            .verdicts
            .iter()
            .filter(|d| d.predicate == name)
            .count()
            + recorded
                .escalations
                .iter()
                .filter(|e| e.predicate() == name)
                .count();
        let (stop, e) = measured;
        (stop, e.volume_hi - e.volume_lo, asked)
    };
    let (stop, round_zero, asked_once) = ask(10.0 * v);
    assert_eq!(
        stop,
        Stop::Settled(SettleVerdict::Holds(true)),
        "a bound ten times over"
    );
    let (stop, near, asked_near) = ask(v * (1.0 + 1.0e-5));
    assert_eq!(
        stop,
        Stop::Settled(SettleVerdict::Holds(true)),
        "a bound just above the volume settles once refined"
    );
    assert!(
        near < round_zero,
        "it refined past round 0: width {near:e} against round 0's {round_zero:e}"
    );
    let (stop, at, asked_at) = ask(v);
    assert_eq!(
        stop,
        Stop::Band,
        "the volume itself is decided by no enclosure"
    );
    assert!(
        at < round_zero,
        "it refined past round 0 before stopping in the band: width {at:e} against \
         round 0's {round_zero:e}"
    );
    assert!(
        asked_near > asked_once && asked_at > asked_once,
        "every round asked is recorded: {asked_once} at round 0, {asked_near} and {asked_at} \
         where the walk refined"
    );
}

/// **A SHELL'S ROLE READ OFF ITS VOLUME WALKS PAST ROUND 0.** A brick
/// holding [`the_sign_walks_narrower_bracket_holds_the_closed_form`]'s
/// strip as a void: the void's walls are splines, which the ray reader
/// has no arm for, so its role is read off the sign of its volume
/// (`side_at_infinity`'s fallback), and that sign straddles zero at
/// round 0 exactly as the upright strip's does. The shell classification
/// reads it `Void` only by refining its walls past round 0, so a
/// classification that read round 0 alone would refuse it.
#[test]
fn a_void_read_off_its_volume_is_refined_past_round_zero() {
    let tol = Tol::witness();
    let s = 1.0e6 * tol.get().eps;
    let mut body: Body<f64> = sweep::test_support::brick(
        (-10.0 * s, 10.0 * s),
        (-10.0 * s, 10.0 * s),
        (-10.0 * s, 10.0 * s),
        tol,
    );
    let solid = body.solids().next().expect("one solid").0;
    let cavity = strip_loft(s, 1.0e-4 * s, false).expect("the strip lofts");
    let evidence = topo::boolean::VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    topo::boolean::VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    };
    topo::boolean::insert_void(&mut body, solid, cavity, &evidence)
        .expect("the strip is strictly inside the brick");
    let finished = topo::test_support::finished("the hollowed brick", body, tol);
    let roles = topo::classify_shells(&finished, tol)
        .unwrap_or_else(|e| panic!("the shell classification reads both shells: {e:?}"));
    assert_eq!(roles.len(), 2, "the brick and its void");
    let voids = roles
        .iter()
        .filter(|r| r.role == topo::ShellRole::Void)
        .count();
    assert_eq!(voids, 1, "the strip's shell is the void: {roles:?}");
}
