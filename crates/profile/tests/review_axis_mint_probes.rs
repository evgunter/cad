//! Reviewer probes of validation's axis mint (`profile::seg::axis_or`):
//! does one document take one branch in the mint at every valuation?
//! Each row prints what it measured before it asserts.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::print_stdout
)]

use geom_core::sym::deferred::with_deferred_session;
use geom_core::sym::revalue::{Decision, record};
use geom_core::sym::with_session_rules;
use geom_core::{
    Bounds, Decide, Interval, ParamSymbol, Point2, Real, Sym, SymBudget, SymRetry, SymRules, Tol,
    Vec2,
};
use profile::{ConstructedProfile, Open, SegmentDirections, SketchPlane, Start, ValidatedProfile};

fn tol() -> Tol {
    Tol::witness()
}

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

#[derive(Clone, Copy, Debug)]
enum Session {
    Eager,
    Deferring,
}

fn run<R>(kind: Session, f: impl FnOnce() -> R) -> R {
    match kind {
        Session::Eager => with_session_rules(budget(), SymRules::shipped(), f).0,
        Session::Deferring => {
            with_deferred_session(budget(), SymRules::shipped(), SymRetry::none(), f).0
        }
    }
}

fn director<T: Real>(d: SegmentDirections<T>) -> Vec2<T> {
    match d {
        SegmentDirections::Line(d) => d,
        SegmentDirections::Arc { .. } => panic!("a line"),
    }
}

/// The quadrilateral (0,0) → (x1,0) → (x2,1) → (0,1), its second side's
/// stored director as bounds.
fn hop<T: Decide + Bounds>(x1: T, x2: T) -> [(f64, f64); 2] {
    let (z, one) = (T::from_f64(0.0), T::from_f64(1.0));
    let lp = Open
        .at(Point2::new(z, z))
        .line_to(Point2::new(x1, z), tol())
        .unwrap()
        .line_to(Point2::new(x2, one), tol())
        .unwrap()
        .line_to(Point2::new(z, one), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    let vp: ValidatedProfile<T> = ConstructedProfile::new(SketchPlane::xy(), vec![lp.loop_])
        .validate(tol())
        .expect("validates");
    let d = director(vp.loops()[0].segments()[1].directions);
    [(d.x.lo(), d.x.hi()), (d.y.lo(), d.y.hi())]
}

fn pf(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
}

fn pi(name: &str, c: f64, h: f64) -> Sym<Interval> {
    Sym::param(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(c - h, c + h),
    )
}

fn signs(d: &[Decision]) -> Vec<(&'static str, Option<geom_core::Sign>, bool)> {
    d.iter().map(|d| (d.predicate, d.sign, d.proven)).collect()
}

/// **At `Sym<f64>`, the replay's lane, an identity whose `f64` value
/// rounds off zero at one valuation and to zero at another takes two
/// branches.** The hop from `(x, 0)` to `((x + w) − w, 1)`: its chord's
/// `x` is `((x + w) − w) − x`, zero as a theorem. Where `f64` computes
/// it as `0.0` the numeric channel reads Zero, the tier proves it and
/// the mint stores the axis; where it rounds to an ulp the numeric
/// channel reads a definite sign, the tier is not asked, and the mint
/// stores the quotient.
#[test]
fn review_an_identity_that_rounds_takes_one_branch_at_sym_f64() {
    let residual = |x: f64, w: f64| ((x + w) - w) - x;
    let (p0, p1) = ((1.0, 1.0), (0.1, 0.3));
    assert_eq!(residual(p0.0, p0.1), 0.0);
    assert_ne!(residual(p1.0, p1.1), 0.0, "pick a valuation that rounds");
    let mut split = Vec::new();
    for kind in [Session::Eager, Session::Deferring] {
        let build = |(x, w): (f64, f64)| {
            record(|| {
                run(kind, || {
                    let (x, w) = (pf("x", x), pf("w", w));
                    hop(x, (x + w) - w)
                })
            })
        };
        let ((d0, at0), (d1, at1)) = (build(p0), build(p1));
        let axis = |d: [(f64, f64); 2]| d[0] == (0.0, 0.0) && d[1] == (1.0, 1.0);
        println!(
            "{kind:?}: p0 director {d0:?} (axis {}), {} decisions",
            axis(d0),
            at0.len()
        );
        println!(
            "{kind:?}: p1 director {d1:?} (axis {}), {} decisions",
            axis(d1),
            at1.len()
        );
        if axis(d0) != axis(d1) || signs(&at0) != signs(&at1) {
            split.push(kind);
        }
    }
    assert!(
        split.is_empty(),
        "the mint took two branches at two valuations in {split:?}"
    );
}

/// **Inside a deferring session at `Sym<Interval>`, an identity over a
/// box of any width escalates and is never asked after.** The slab's
/// side `(W, 0) → (W, 1)`: `W − W` over a box straddles zero, which the
/// least band escalates; an eager session hands the escalation to the
/// tier, a deferring one returns it, and the mint reads no Zero to
/// discharge. Over a box of no width the same margin is `[0, 0]`, a
/// Zero the mint discharges.
#[test]
fn review_the_slabs_side_takes_one_branch_in_a_deferring_session() {
    let h = tol().eps() / 64.0;
    let build = |kind: Session, h: f64| {
        run(kind, || {
            let w = pi("W", 1.0, h);
            hop(w, w)
        })
    };
    let rows = [
        ("eager, a box", build(Session::Eager, h)),
        ("eager, a point", build(Session::Eager, 0.0)),
        ("deferring, a box", build(Session::Deferring, h)),
        ("deferring, a point", build(Session::Deferring, 0.0)),
    ];
    for (what, d) in rows {
        println!("{what}: {d:?}");
    }
    let axis = [(0.0, 0.0), (1.0, 1.0)];
    for (what, d) in rows {
        assert_eq!(d, axis, "{what}: the slab's side is the axis");
    }
}

/// **Does the decision read reach the mint?** At an exact witness the
/// early walk reads a `max`'s comparison off the box
/// (`SymRules::decision_read`), so `max(a, b) − b` could be the zero
/// form where the box certifies `a < b` and no identity where it
/// certifies `a > b`. Measured here in every spelling: printed, and
/// asserted only as far as the two valuations of one spelling agree.
#[test]
fn review_a_max_in_a_corner_takes_the_branch_its_box_certifies() {
    let axis = [(0.0, 0.0), (1.0, 1.0)];
    for h in [0.0, 1e-12] {
        for (what, a0) in [("a < b", 0.5), ("a > b", 1.5)] {
            let rows = run(Session::Eager, || {
                let (a, b) = (pi("a", a0, h), pi("b", 1.0, h));
                let m = Real::max(a, b);
                let n = Real::min(a, b);
                [
                    ("max, b", hop(m, b)),
                    ("b, max", hop(b, m)),
                    ("max, a", hop(m, a)),
                    ("min, a", hop(n, a)),
                    ("min, b", hop(n, b)),
                ]
            });
            for (spelling, d) in rows {
                println!("h {h:e}, {what}, {spelling}: axis {} {d:?}", d == axis);
            }
        }
    }
}
