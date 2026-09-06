//! **R1's independent probes of M10-9's registered-identity door**, at
//! the scalar. Derived from the door's own contract rather than from the
//! unit's rows: what the WITNESS actually checks in the lane the driver
//! replays in, what a chain of registrations claims that nobody
//! witnessed, and whether `Curve3::circle_at`'s delegation (D13) moved a
//! bit.
//!
//! Evidence-only rows print; the rest assert what they measured.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Sign};
use geom_core::real::Real;
use geom_core::sym::{SymRegistration, with_session_rules};
use geom_core::{Decide, ParamSymbol, Sym, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn pi(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param(ParamSymbol::of(name), Interval::from_bounds(lo, hi))
}

fn sign_of(m: Sym<Interval>) -> Result<Sign, ()> {
    m.sign_within(band()).map_err(|_| ())
}

/// **THE WITNESS AT `Interval` IS "THE ENCLOSURES MEET", AND THAT IS NOT
/// AN IDENTITY TEST.** The lane the driver replays every leaf in is
/// `Sym<Interval>`, and there the door's witness asks only whether the
/// two certified enclosures INTERSECT over the box. A claim that is true
/// at ONE parameter point of the box and false everywhere else meets
/// that test, is `Recorded`, and the residual then decides `Zero` — for
/// every parameter value in the box, which is what a symbolic `Zero`
/// means.
///
/// `x² = x` is such a claim: true at `x = 1`, false at every other `x`.
/// Over `[0.9, 1.1]` the enclosures `[0.81, 1.21]` and `[0.9, 1.1]`
/// meet.
#[test]
fn r1_a_coincidence_at_one_point_of_the_box_registers_and_decides_zero() {
    let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
        let x = pi("x", 0.9, 1.1);
        let sq = x * x;
        let reg = sq.register_equal(x);
        let s = sign_of(sq - x);
        (reg, s)
    });
    assert_eq!(
        out.0,
        SymRegistration::Recorded,
        "the Interval witness accepts a claim that holds at one point of the box"
    );
    assert_eq!(
        out.1,
        Ok(Sign::Zero),
        "and the tier then answers Zero for the WHOLE box: {counts:?}"
    );
    assert_eq!(counts.registered, 1, "counted as a registered zero");
    // What the claim is actually worth at an interior point.
    let at = |v: f64| v * v - v;
    println!(
        "   x²−x at x=0.9 is {:e}, at x=1.1 is {:e}; the door said Zero over [0.9, 1.1]",
        at(0.9),
        at(1.1)
    );
}

/// The same shape with a GEOMETRIC error rather than a coincidence: a
/// rim whose declared radius is 0.1% wrong. At `f64` that is refused
/// (`WITNESS_REL = 1e-9`); at `Interval` over a box wide enough for the
/// two enclosures to overlap it is recorded, and the residual decides
/// `Zero`.
#[test]
fn r1_a_geometric_lie_the_f64_witness_refuses_is_recorded_at_interval() {
    let f64_says = <f64 as Real>::register_equal(1.0, 1.001);
    assert_eq!(f64_says, SymRegistration::Contradicted, "at a point: caught");
    let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
        // ‖v‖ over a box, against a radius parameter 0.1% too large.
        let (vx, vy) = (pi("vx", 2.97, 3.03), pi("vy", 3.96, 4.04));
        let n = (vx * vx + vy * vy).sqrt();
        let r = pi("r", 5.0 * 1.001 * 0.99, 5.0 * 1.001 * 1.01).abs();
        let reg = n.register_equal(r);
        let one = <Sym<Interval> as Real>::one();
        let s = sign_of(vx * (r / n - one));
        (reg, s)
    });
    println!("   interval witness on a 0.1%-wrong radius: {:?}", out.0);
    assert_eq!(
        out.0,
        SymRegistration::Recorded,
        "over a box the enclosures still meet, so the same lie is recorded"
    );
    assert_eq!(out.1, Ok(Sign::Zero), "and it discharges: {counts:?}");
}

/// **A CHAIN OF REGISTRATIONS CLAIMS A PAIR NOBODY WITNESSED.**
/// `Session::alias` resolves transitively and `register_equal` records
/// on the LEFT node's alias ROOT, so registering `a ≡ b` and then
/// `a ≡ c` records `b ≡ c` — a pair whose values the door never
/// compared. Sound if both registrants told the truth; the point is that
/// the witness is not evaluated for the claim actually stored.
#[test]
fn r1_the_registry_stores_a_pair_the_witness_never_compared() {
    let (rows, _) = with_session_rules(budget(), SymRules::shipped(), || {
        let a = pi("a", 0.9, 1.1);
        let b = a * a;
        let c = a * a * a;
        // Both of these are witnessed against `a`, never against each
        // other: over this box all three enclosures meet.
        let first = b.register_equal(a);
        let second = c.register_equal(a);
        // `b − c` is now the zero form, on a claim (`a² ≡ a³`) no
        // registrant made and no witness saw.
        let s = sign_of(b - c);
        (first, second, s)
    });
    println!("   {rows:?}");
    assert_eq!(rows.2, Ok(Sign::Zero), "a² ≡ a³ decided by transitivity");
}

/// **TWO REGISTRATIONS PUT THE TWO CHANNELS IN CONTRADICTION, AND THE
/// RUN NOW SAYS SO.** `b = a − 0.2` and `c = a + 0.2` each MEET `a`'s
/// enclosure at an endpoint, so both registrations are witnessed, and
/// the registry then makes `b − c` the zero form while its enclosure is
/// `[-0.6, -0.2]` — definite, and nowhere near zero.
///
/// R1 found this state SILENT: M10-9's first cut exempted the door's
/// arm from the `Decide` impl's debug assertion and counted nothing, so
/// the one state that assertion exists to catch was the one state it no
/// longer covered. The fix moved the check to decide time and out of
/// `debug_assert!`: the numeric channel still wins the answer (no false
/// `Zero` is ever returned), and the contradiction is COUNTED, so the
/// drive's receipt reports that a stated identity is false over this
/// box (`SymCounts::registrations_contradicted`, and
/// `ParamBoxVerdict`'s serialize/render lines).
///
/// This row is R1's, re-cut as the positive pin of the fix it forced.
#[test]
fn r1_two_registrations_make_the_two_channels_contradict_and_it_is_counted() {
    let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
        let a = pi("a", 0.9, 1.1);
        let lo = a - <Sym<Interval> as Real>::from_f64(0.2);
        let hi = a + <Sym<Interval> as Real>::from_f64(0.2);
        let r1 = lo.register_equal(a);
        let r2 = hi.register_equal(a);
        (r1, r2, sign_of(lo - hi))
    });
    println!("   {out:?} {counts:?}");
    assert_eq!(
        (out.0, out.1),
        (SymRegistration::Recorded, SymRegistration::Recorded),
        "touching enclosures satisfy the witness"
    );
    assert_eq!(
        out.2,
        Ok(Sign::Negative),
        "the numeric channel answers, so no false Zero is returned"
    );
    assert_eq!(
        counts.registered, 0,
        "the decision is not a registered discharge — the numeric answer stands"
    );
    assert_eq!(
        counts.registrations_contradicted, 1,
        "and the contradiction is COUNTED rather than silent: {counts:?}"
    );
}

/// **A refused registration IS a trace in the counts.** R1 found the
/// typed refusal dropped at every shipping registrant —
/// `Real::register_equal` was not `#[must_use]`, both registrants
/// ignored its answer, and nothing counted a refusal, so a constructor
/// registering a lie in a real document produced no column, no flag and
/// no receipt line. The fix made the method `#[must_use]`, made both
/// registrants handle the typed answer (loud in debug), and gave the
/// session a column. This row is R1's, re-cut as the positive pin.
#[test]
fn r1_a_refused_registration_is_counted_in_the_receipt() {
    let (_, counts) = with_session_rules(budget(), SymRules::shipped(), || {
        let x = pi("x", 1.0, 1.0);
        let y = pi("y", 5.0, 5.0);
        assert_eq!(
            x.register_equal(y),
            SymRegistration::Contradicted,
            "the enclosures are disjoint"
        );
    });
    println!("   after a refused registration: {counts:?}");
    assert_eq!(counts.registered, 0, "nothing was recorded");
    assert_eq!(
        counts.registrations_refused, 1,
        "and the refusal is counted: {counts:?}"
    );
}
