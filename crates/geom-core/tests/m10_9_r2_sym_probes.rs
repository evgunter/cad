//! **R2's independent probes of M10-9's registered-identity door** —
//! derived from the claims, not from the unit's rows.
//!
//! Three attacks on claim 1 (soundness of the door), each at the lane
//! scalar the driver replays at (`Sym<Interval>`):
//!
//! - a registration that is TRUE AT THE NOMINAL and false elsewhere in
//!   the box (`x² ≡ x` at `x = 1`) — the witness at `Interval` asks only
//!   that the two enclosures MEET, which every such coincidence does;
//! - a registration that is false by a GEOMETRIC amount but whose two
//!   enclosures still meet because the box is wide — the wider the box,
//!   the more lies the `Interval` witness lets through, and the driver
//!   registers exactly over the leaf's box;
//! - the shipped rim identity at the shape the constructor builds, with
//!   a bulge at |b| = 1 (a semicircle), |b| > 1 (a major arc) and b < 0.
//!
//! Every row is a deterministic fixture ([[test-suite-cost]]). NOT
//! proposed for merge: the branch carries a probe instrument in
//! `geom_core::sym::report` (the enclosure column).

#![cfg(feature = "interval")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use geom_core::interval::Interval;
use geom_core::predicate::{Band, Sign};
use geom_core::real::Real;
use geom_core::sym::{SymRegistration, with_session, with_session_rules};
use geom_core::{Decide, ParamSymbol, Sym, SymBudget, SymCounts, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A parameter over `[lo, hi]`.
fn p(name: &str, lo: f64, hi: f64) -> Sym<Interval> {
    Sym::param(ParamSymbol::of(name), Interval::from_bounds(lo, hi))
}

fn lit(x: f64) -> Sym<Interval> {
    <Sym<Interval> as Real>::from_f64(x)
}

fn sign_of(m: Sym<Interval>) -> Result<Sign, String> {
    m.sign_within(band()).map_err(|e| format!("{e:?}"))
}

fn shipped<R>(f: impl FnOnce() -> R) -> (R, SymCounts) {
    with_session_rules(budget(), SymRules::shipped(), f)
}

// ------------------------------------------------------ claim 1

/// **A coincidence at the nominal is REGISTERED and then DISCHARGED over
/// the whole box.** `x² ≡ x` holds at `x = 1` and nowhere else; over
/// `[0.9, 1.1]` the two enclosures `[0.81, 1.21]` and `[0.9, 1.1]`
/// meet, so the witness answers `Witnessed`, the door records, and the
/// consumer residual `x² − x` — whose true range over the box is
/// `[−0.09, 0.11]` — decides `Zero` as a registered identity.
///
/// The band is `zero = 1e-9`; the residual is 1e8 band-widths wide.
/// This is the shape claim 1 asked about ("values that meet at the
/// nominal and diverge with the parameter"): the door does NOT catch
/// it, and the unit's soundness rests entirely on the registrant's
/// proof. Recorded here as a fact about the mechanism, not a bug in
/// the shipped registrants (whose identities hold at every parameter
/// value — the rows below).
#[test]
fn r2_a_coincidence_at_the_nominal_is_registered_and_discharged_over_the_box() {
    let ((reg, decided), counts) = shipped(|| {
        let x = p("x", 0.9, 1.1);
        let sq = x * x;
        // `x.register_equal(x*x)` is refused Cyclic (the unit pins that);
        // the OTHER direction is accepted, and it is the direction a
        // constructor naturally writes (derived quantity ≡ held one).
        let reg = sq.register_equal(x);
        let decided = sign_of(sq - x);
        (reg, decided)
    });
    assert_eq!(
        reg,
        SymRegistration::Recorded,
        "the witness lets the coincidence through"
    );
    assert_eq!(
        decided,
        Ok(Sign::Zero),
        "and `x² − x` decides Zero over a box where it ranges over [-0.09, 0.11]"
    );
    assert_eq!(counts.registered, 1, "{counts:?}");
}

/// **The `Interval` witness weakens as the box widens.** A registration
/// false by 0.5 on a unit-scale value (`x ≡ x + 0.5`) is refused over a
/// narrow box and RECORDED over a wide one, because "the enclosures
/// meet" is the whole check. Then a consumer residual the numeric
/// channel cannot decide (`(x − (x + ½))·y`, `y` straddling zero)
/// decides `Zero` through the door although it is `−y/2`, and the
/// `f64` witness at a nominal `y = 0` would evaluate it to exactly zero
/// — neither witness sees the lie.
#[test]
fn r2_the_interval_witness_lets_a_geometric_lie_through_over_a_wide_box() {
    let narrow = shipped(|| {
        let x = p("x", 1.0, 1.01);
        (x + lit(0.5)).register_equal(x)
    })
    .0;
    assert_eq!(narrow, SymRegistration::Contradicted, "narrow box: refused");
    let ((wide, decided), counts) = shipped(|| {
        let x = p("x", 0.0, 1.0);
        let y = p("y", -1.0, 1.0);
        let reg = (x + lit(0.5)).register_equal(x);
        let decided = sign_of((x - (x + lit(0.5))) * y);
        (reg, decided)
    });
    assert_eq!(
        wide,
        SymRegistration::Recorded,
        "wide box: the same lie is RECORDED"
    );
    assert_eq!(
        decided,
        Ok(Sign::Zero),
        "and a residual equal to −y/2 decides Zero through the door: {counts:?}"
    );
}

/// **The numeric shield does not reach a dependency-widened residual.**
/// The same lie, and the consumer residual is `x − (x + ½)` ITSELF —
/// `−½` at every parameter value. The numeric channel cannot prove it
/// non-zero: interval arithmetic sees `[0, 1] − [0.5, 1.5] = [−1.5,
/// 0.5]`, which straddles, so the door is consulted and answers `Zero`.
/// The PR's shield ("no registration can turn a margin the enclosure
/// proved non-zero into a Zero") holds exactly where the enclosure
/// proves something — and over a box wide enough for a lie's two
/// enclosures to meet, the residual between them straddles for the
/// same reason. The two weaknesses coincide by construction.
#[test]
fn r2_the_numeric_shield_does_not_reach_a_dependency_widened_lie() {
    let (decided, counts) = shipped(|| {
        let x = p("x", 0.0, 1.0);
        assert_eq!((x + lit(0.5)).register_equal(x), SymRegistration::Recorded);
        sign_of(x - (x + lit(0.5)))
    });
    assert_eq!(
        decided,
        Ok(Sign::Zero),
        "a residual equal to −½ everywhere decides Zero: {counts:?}"
    );
    assert_eq!(counts.registered, 1, "{counts:?}");
}

// ------------------------------------------- claim 9: the theorems

/// What [`sagitta`] answers: `(‖a − c‖, radius, a − c, c)`.
type Sagitta = (
    Sym<Interval>,
    Sym<Interval>,
    [Sym<Interval>; 2],
    [Sym<Interval>; 2],
);

/// The sagitta construction as `profile::seg` spells it, at the lane
/// scalar over a box: chord `a → b`, bulge `bulge`.
fn sagitta(
    ax: Sym<Interval>,
    ay: Sym<Interval>,
    bx: Sym<Interval>,
    by: Sym<Interval>,
    bulge: Sym<Interval>,
) -> Sagitta {
    let half = lit(0.5);
    let (dx, dy) = (bx - ax, by - ay);
    let len = (dx * dx + dy * dy).sqrt();
    let (ux, uy) = (dx / len, dy / len);
    let (nx, ny) = (lit(0.0) - uy, ux);
    let (mx, my) = (ax + dx * half, ay + dy * half);
    let b2 = bulge.powi(2);
    let four_b = lit(4.0) * bulge;
    let apothem = len * (lit(1.0) - b2) / four_b;
    let signed_radius = len * (lit(1.0) + b2) / four_b;
    let (cx, cy) = (mx + nx * apothem, my + ny * apothem);
    let radius = signed_radius.abs();
    let (vx, vy) = (ax - cx, ay - cy);
    let norm = (vx * vx + vy * vy).sqrt();
    (norm, radius, [vx, vy], [cx, cy])
}

/// **The rim identity holds at |b| = 1 and |b| > 1 and b < 0**, over a
/// box, and discharges the rim residual `v·(r/‖v‖ − 1)`; the witness
/// records each. (A bulge STRADDLING zero is not a case: the arc is
/// classified `segment_straightness` over the box first and refuses
/// there before any carrier is built.)
#[test]
fn r2_the_rim_identity_holds_at_the_semicircle_the_major_arc_and_a_clockwise_turn() {
    for (label, blo, bhi) in [
        ("semicircle", 0.9995, 1.0005),
        ("major arc", 2.4995, 2.5005),
        ("clockwise minor", -0.3005, -0.2995),
    ] {
        let ((reg, rows), counts) = shipped(|| {
            // Boxes at the driver's regime: ~1e-7 on 4e-3 geometry and
            // 1e-3 on the bulge. (Wider, the norm's enclosure loses its
            // certification and the witness answers `Unwitnessed`, which
            // records nothing — the safe direction, and a fact worth
            // knowing: the door is inert on a leaf too wide to certify.)
            let (ax, ay) = (p("ax", 0.0, 1e-7), p("ay", 0.0, 1e-7));
            let (bx, by) = (
                p("bx", 4.0e-3, 4.0e-3 + 1e-7),
                p("by", 3.0e-3, 3.0e-3 + 1e-7),
            );
            let b = p("b", blo, bhi);
            let (norm, radius, [vx, vy], _) = sagitta(ax, ay, bx, by, b);
            let reg = norm.register_equal(radius);
            let scale = radius / norm - lit(1.0);
            (reg, [sign_of(vx * scale), sign_of(vy * scale)])
        });
        assert_eq!(reg, SymRegistration::Recorded, "{label}");
        assert_eq!(
            rows,
            [Ok(Sign::Zero), Ok(Sign::Zero)],
            "{label}: {counts:?}"
        );
        assert_eq!(counts.registered, 2, "{label}: {counts:?}");
    }
}

/// **Evidence** — how loose is the `Interval` witness on a real
/// registration? The rim identity over boxes of the plate's ceiling
/// scale and wider: the two enclosures' widths, so a reader can see how
/// far apart two values could be and still "meet".
#[test]
#[ignore = "evidence-only: prints the witness enclosures at the plate's ceiling scale"]
fn r2_evidence_how_loose_is_the_interval_witness_at_the_ceiling_scale() {
    for w in [4.0e-11_f64, 4.0e-9, 4.0e-7] {
        let _ = with_session(budget(), || {
            let (ax, ay) = (p("ax", 0.0, w), p("ay", 0.0, w));
            let (bx, by) = (p("bx", 4.0e-3, 4.0e-3 + w), p("by", 3.0e-3, 3.0e-3 + w));
            let b = p("b", 0.4, 0.4 + w);
            let (norm, radius, _, _) = sagitta(ax, ay, bx, by, b);
            let bo = |v: Sym<Interval>| {
                (
                    geom_core::Bounds::lo(v.value),
                    geom_core::Bounds::hi(v.value),
                )
            };
            let (nl, nh) = bo(norm);
            let (rl, rh) = bo(radius);
            println!(
                "   box width {w:e}: norm in [{nl:.17e}, {nh:.17e}] (width {:e}); r in [{rl:.17e}, {rh:.17e}] (width {:e})",
                nh - nl,
                rh - rl
            );
        });
    }
}
