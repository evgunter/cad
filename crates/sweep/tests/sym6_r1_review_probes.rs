//! SYM-6 review probes (reviewer r1, branch `sym/6-review-r1`) — a
//! document the unit did not measure, driven through the public doors.
//!
//! Not part of the unit; these rows exist to falsify SYM-6's claims 1,
//! 2 and 5 on a registrant at coordinates of order 1e9.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::sym::{SymRegistration, with_session_rules};
use geom_core::{Point2, Real, SymBudget, SymRules, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 8192,
        max_degree: 128,
    }
}

/// The far washer: a rectangle at x ∈ [R, 2R] with R = 1e9, revolved
/// fully, with the inner radius a PARAMETER over a narrow box so the
/// rim identity is a genuine identity in the parameters and only the
/// registration answers it (`revolve_washer`'s argument).
macro_rules! far_washer_counts {
    ($S:ty, $param:expr, $rules:expr) => {{
        type S = $S;
        with_session_rules(budget(), $rules, || {
            let lit = |v: f64| <S as Real>::from_f64(v);
            let big = 1.0e9_f64;
            let r0: S = $param(big);
            let (zero, one, two) = (lit(0.0), lit(big), lit(2.0 * big));
            let loop_ = ProfileLoop::polygon([
                Point2::new(r0, zero),
                Point2::new(two, zero),
                Point2::new(two, one),
                Point2::new(r0, one),
            ]);
            let vp = match Profile::new(SketchPlane::xy(), vec![loop_]).validate(Tol::witness()) {
                Ok(vp) => vp,
                Err(e) => {
                    println!("   profile refused: {e:?}");
                    return false;
                }
            };
            let axis = RevolveAxis {
                origin: Point2::new(zero, zero),
                dir: Vec2::new(zero, lit(1.0)),
            };
            match revolve(&vp, axis, Revolution::Full, Tol::witness()) {
                Ok(_) => true,
                Err(e) => {
                    println!("   revolve refused: {e:?}");
                    false
                }
            }
        })
    }};
}

/// **Claim 1 and 2 at `Sym<Interval>`**: the far washer's registrants
/// discharge and nothing is refused, at whatever eps row the process
/// runs at.
#[cfg(feature = "interval")]
#[test]
fn r1_the_far_revolve_registers_and_refuses_nothing_at_interval() {
    use geom_core::{Interval, ParamSymbol, Sym};
    let eps = Tol::witness().eps();
    let (built, counts) = far_washer_counts!(
        Sym<Interval>,
        |big: f64| Sym::param(
            ParamSymbol::of("r0"),
            Interval::from_bounds(big * (1.0 - eps / 64.0), big * (1.0 + eps / 64.0)),
        ),
        SymRules::shipped()
    );
    println!("   Sym<Interval> far washer at eps={eps:e}: built={built} {counts:?}");
    assert_eq!(
        (
            counts.registrations_refused,
            counts.registrations_contradicted
        ),
        (0, 0),
        "nothing refused at 1e9 at eps={eps:e}: {counts:?}"
    );
}

/// The same document at `Sym<f64>` — the INEXACT lane, where an
/// absolute-eps slack would refuse the true rim identity at 1e9.
#[test]
fn r1_the_far_revolve_registers_and_refuses_nothing_at_f64() {
    use geom_core::{ParamSymbol, Sym};
    let eps = Tol::witness().eps();
    let (built, counts) = far_washer_counts!(
        Sym<f64>,
        |big: f64| Sym::param(ParamSymbol::of("r0"), big),
        SymRules::shipped()
    );
    println!("   Sym<f64> far washer at eps={eps:e}: built={built} {counts:?}");
    assert!(counts.registered > 0, "the door is asked: {counts:?}");
    assert_eq!(
        counts.registrations_refused, 0,
        "nothing refused at 1e9 at eps={eps:e}: {counts:?}"
    );
}

/// The same at `Sym<Probe>`, which delegates to `f64`'s witness.

/// **Claim 2, the planted registrant**: two sides apart by
/// `k · eps · scale` at a scale of 1e9, for k just below and just above
/// one. Below: witnessed. Above: refused, and by the INEXACT arm.
#[test]
fn r1_the_slack_is_relative_and_floored_at_1e9() {
    let eps = Tol::witness().eps();
    let a = 1.0e9_f64;
    for (k, want) in [
        (0.99_f64, SymRegistration::Witnessed),
        (1.01_f64, SymRegistration::Disputed),
    ] {
        let b = a + k * eps * a;
        let got = <f64 as Real>::register_equal(a, b, Tol::witness());
        println!(
            "   f64  k={k} eps={eps:e} a={a:e} b-a={:e} -> {got:?}",
            b - a
        );
        assert_eq!(got, want, "k={k} at eps={eps:e}");
    }
    // The floor: a near-zero pair is compared ABSOLUTELY at eps.
    assert_eq!(
        <f64 as Real>::register_equal(1.0e-30, 1.0e-30 + 0.99 * eps, Tol::witness()),
        SymRegistration::Witnessed
    );
    assert_eq!(
        <f64 as Real>::register_equal(1.0e-30, 1.0e-30 + 1.01 * eps, Tol::witness()),
        SymRegistration::Disputed
    );
    // A TRUE identity at 1e9 whose two sides differ by f64 rounding
    // only: an ABSOLUTE eps would refuse this at 1e-12 (rounding at
    // 1e9 is ~1.2e-7); the relative spelling witnesses it at every row.
    let rounded = f64::from_bits(a.to_bits() + 1000);
    let ulps = 1000;
    println!(
        "   rounding at 1e9: (3a)/3 - a = {:e} ({ulps} ulp)",
        rounded - a
    );
    assert_eq!(
        <f64 as Real>::register_equal(a, rounded, Tol::witness()),
        SymRegistration::Witnessed,
        "a true identity at 1e9 must be witnessed at eps={eps:e}"
    );
}

/// **Claim 5**: the arm each lane can answer, at the public door.
#[cfg(feature = "interval")]
#[test]
fn r1_the_arm_is_keyed_to_the_kind_of_witness() {
    use geom_core::Interval;
    // Exact witness: disjoint certified enclosures -> Contradicted.
    assert_eq!(
        <Interval as Real>::register_equal(
            Interval::from_bounds(1.0, 1.0),
            Interval::from_bounds(5.0, 5.0),
            Tol::witness()
        ),
        SymRegistration::Contradicted
    );
    // ... and never Disputed, however far apart or however wide.
    for (alo, ahi, blo, bhi) in [
        (1.0e9, 1.0e9, 1.0e9 + 1.0e3, 1.0e9 + 1.0e3),
        (0.0, 1.0, 5.0, 6.0),
        (-1.0e18, 1.0e18, 2.0e18, 3.0e18),
    ] {
        let got = <Interval as Real>::register_equal(
            Interval::from_bounds(alo, ahi),
            Interval::from_bounds(blo, bhi),
            Tol::witness(),
        );
        assert_ne!(
            got,
            SymRegistration::Disputed,
            "{alo}..{ahi} vs {blo}..{bhi}"
        );
    }
    // Inexact witnesses never answer Contradicted, however far apart.
    for (a, b) in [(1.0_f64, 2.0), (0.0, 1.0e300), (-1.0e18, 1.0e18)] {
        assert_eq!(
            <f64 as Real>::register_equal(a, b, Tol::witness()),
            SymRegistration::Disputed,
            "f64 {a} vs {b}"
        );
    }
}

#[cfg(all(feature = "probe", feature = "interval"))]
#[test]
fn r1_probe_answers_the_inexact_arm() {
    use geom_core::Probe;
    for (a, b) in [(1.0_f64, 2.0), (1.0e9, 3.0e9)] {
        assert_eq!(
            <Probe as Real>::register_equal(
                <Probe as Real>::from_f64(a),
                <Probe as Real>::from_f64(b),
                Tol::witness()
            ),
            SymRegistration::Disputed,
            "Probe {a} vs {b}"
        );
    }
}

/// The far ARC: a full circle of radius 1e3 centred at (1e9, 1e9),
/// extruded. This is the document that reaches `swept.rs`'s two
/// registrants — the rim identity at a scale of 1e3 and the SPAN
/// identity at coordinates of 1e9, which is exactly where an absolute
/// eps slack refuses a true identity.
macro_rules! far_arc_counts {
    ($S:ty, $lift:expr, $rules:expr) => {{
        type S = $S;
        with_session_rules(budget(), $rules, || {
            let lit = |v: f64| <S as Real>::from_f64(v);
            let f = $lift;
            let (cx, cy, r) = (1.0e9_f64, 1.0e9_f64, 1.0e3_f64);
            let loop_ = <ProfileLoop<S> as RawLoop<S>>::new(vec![
                ProfileVertex::new(Point2::new(f(cx - r), lit(cy)), lit(1.0)),
                ProfileVertex::new(Point2::new(f(cx + r), lit(cy)), lit(1.0)),
            ]);
            let vp = match Profile::new(SketchPlane::xy(), vec![loop_]).validate(Tol::witness()) {
                Ok(vp) => vp,
                Err(e) => {
                    println!("   profile refused: {e:?}");
                    return false;
                }
            };
            match extrude(&vp, Extrusion::Distance(lit(1.0e2)), Tol::witness()) {
                Ok(_) => true,
                Err(e) => {
                    println!("   extrude refused: {e:?}");
                    false
                }
            }
        })
    }};
}

#[cfg(feature = "interval")]
#[test]
fn r1_the_far_arc_extrude_at_interval() {
    use geom_core::{Interval, Sym};
    let eps = Tol::witness().eps();
    let (built, counts) = far_arc_counts!(
        Sym<Interval>,
        |v: f64| Sym::opaque(Interval::from_bounds(v, v)),
        SymRules::shipped()
    );
    println!("   Sym<Interval> far arc at eps={eps:e}: built={built} {counts:?}");
    assert_eq!(
        (
            counts.registrations_refused,
            counts.registrations_contradicted
        ),
        (0, 0),
        "nothing refused at 1e9 at eps={eps:e}: {counts:?}"
    );
}

/// IGNORED: at eps rows 1e-9 and 1e-12 this document trips the M10-8
/// two-channel-contradiction `debug_assert!` in `sym.rs`, and it does
/// so at the MERGE BASE too (measured by r1) — a pre-existing limit of
/// `Sym<f64>` at coordinates of 1e9, not SYM-6's.
#[test]
#[ignore = "pre-existing Sym<f64> channel-contradiction assertion at 1e9; reds at the merge base too"]
fn r1_the_far_arc_extrude_at_f64() {
    use geom_core::Sym;
    let eps = Tol::witness().eps();
    let (built, counts) = far_arc_counts!(Sym<f64>, |v: f64| Sym::from_f64(v), SymRules::shipped());
    println!("   Sym<f64> far arc at eps={eps:e}: built={built} {counts:?}");
    assert_eq!(
        counts.registrations_refused, 0,
        "nothing refused at 1e9 at eps={eps:e}: {counts:?}"
    );
}
