//! **A rectangle whose width is a parameter box validates at
//! `Sym<Interval>`**, drawn with the pen (heading 0, `line(w)`, quarter
//! turns, closed onto its start) and as four corners.
//!
//! Each adjacent pair's shared vertex is on the other segment's carrier
//! and an end of its span by construction (`profile::seg`'s
//! `line_line`). Read instead, its side and its span margin are
//! recomputed from the chord it ends, and the span margin `L − t` reads
//! about six times the box's half-width either side of zero.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom_core::sym::with_session_rules;
use geom_core::{Decide, Interval, ParamSymbol, Point2, Real, Sym, SymBudget, SymRules, Tol};
use profile::{
    ConstructedLoop, ConstructedProfile, EscalationSite, Gate, Open, ProfileError, SketchPlane,
    Start, ValidatedProfile,
};

type Lane = Sym<Interval>;

fn tol() -> Tol {
    Tol::witness()
}

fn in_session<R>(f: impl FnOnce() -> R) -> R {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    with_session_rules(budget, SymRules::shipped(), f).0
}

fn param(c: f64, h: f64) -> Lane {
    Sym::param(
        ParamSymbol::new(test_utils::symbol_id("w")),
        Interval::from_bounds(c - h, c + h),
    )
}

fn validate<T: Decide>(lp: ConstructedLoop<T>) -> Result<ValidatedProfile<T>, ProfileError> {
    ConstructedProfile::new(SketchPlane::xy(), vec![lp]).validate(tol())
}

fn pen<T: Decide>(w: T, quarter: T) -> ConstructedLoop<T> {
    let (zero, one) = (T::from_f64(0.0), T::from_f64(1.0));
    Open.at(Point2::new(zero, zero))
        .angle(zero, tol())
        .unwrap()
        .line(w, tol())
        .unwrap()
        .turn(quarter, tol())
        .unwrap()
        .line(one, tol())
        .unwrap()
        .turn(quarter, tol())
        .unwrap()
        .line(w, tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap()
        .loop_
}

fn corners<T: Decide>(w: T) -> ConstructedLoop<T> {
    let (zero, one) = (T::from_f64(0.0), T::from_f64(1.0));
    Open.at(Point2::new(zero, zero))
        .line_to(Point2::new(w, zero), tol())
        .unwrap()
        .line_to(Point2::new(w, one), tol())
        .unwrap()
        .line_to(Point2::new(zero, one), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap()
        .loop_
}

const HALVES: [f64; 5] = [0.0, 1e-9, 1e-6, 1e-3, 0.05];

#[test]
fn a_rectangle_with_a_boxed_width_validates_at_sym_interval() {
    for h in HALVES {
        in_session(|| {
            let quarter = Lane::pi() / Lane::from_f64(2.0);
            for (form, lp) in [
                ("pen", pen(param(1.0, h), quarter)),
                ("corners", corners(param(1.0, h))),
            ] {
                if let Err(e) = validate(lp) {
                    panic!("{form} over ±{h:e}: {e:?}");
                }
            }
        });
    }
}

/// The plain `Interval` lane reads the same pairs with no symbolic tier
/// behind its readings: the shared vertices are still not read.
#[test]
fn a_cornered_rectangle_with_a_boxed_width_validates_at_interval() {
    for h in HALVES {
        let w = Interval::from_bounds(1.0 - h, 1.0 + h);
        if let Err(e) = validate(corners(w)) {
            panic!("corners over ±{h:e}: {e:?}");
        }
    }
}

/// **Today's truth, filed as
/// `profile/a-pen-loop-turned-by-a-rounded-quarter-cannot-order-its-start-over-an-interval`**:
/// turned by the rounded `π/2`, the pen's fourth vertex has
/// `x = w·(1 + cos² − sin²) + cos` of that angle, a few `10⁻¹⁷` over the
/// reals, which reads `±2h` about zero (and straddles it at a point box
/// too), so the canonical order's exact `x` comparison against the start
/// is undecided. The pair pass is not reached.
#[test]
fn a_pen_rectangle_turned_by_a_rounded_quarter_cannot_order_its_start() {
    for h in HALVES {
        in_session(|| {
            let lp = pen(param(1.0, h), Lane::from_f64(FRAC_PI_2));
            match validate(lp) {
                Err(ProfileError::Escalated {
                    site: EscalationSite::Loop { loop_index: 0 },
                    gate: Gate::CanonicalOrderX,
                    ..
                }) => {}
                other => panic!("over ±{h:e}: {:?}", other.map(|_| "validated")),
            }
        });
    }
}

/// **A square whose one corner a box widens validates at `Interval`**:
/// the corner is the shared vertex of two adjacent pairs, and every
/// other pair reads it far from its carrier.
#[test]
fn a_square_with_a_boxed_corner_validates_at_interval() {
    for h in [1e-6, 0.01, 0.1] {
        let (zero, one) = (Interval::from_f64(0.0), Interval::from_f64(1.0));
        let corner = Point2::new(
            Interval::from_bounds(1.0 - h, 1.0 + h),
            Interval::from_bounds(1.0 - h, 1.0 + h),
        );
        let lp = Open
            .at(Point2::new(zero, zero))
            .line_to(Point2::new(one, zero), tol())
            .unwrap()
            .line_to(corner, tol())
            .unwrap()
            .line_to(Point2::new(zero, one), tol())
            .unwrap()
            .line_to(Start, tol())
            .unwrap()
            .loop_;
        if let Err(e) = validate(lp) {
            panic!("the square over ±{h:e}: {e:?}");
        }
    }
}
