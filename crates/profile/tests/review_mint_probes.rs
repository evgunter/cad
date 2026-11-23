//! Reviewer probes for the direction mint's structural question
//! (PR 7488's review). Each row prints what it finds; the asserts state
//! what the probe observed on the frozen head.


use geom_core::sym::with_session_rules;
use geom_core::{Real, Bounds, Decide, Interval, ParamSymbol, Point2, Sym, SymBudget, SymRules, Tol, Vec2};
use profile::{ConstructedLoop, ConstructedProfile, Open, SegmentDirections, SketchPlane, Start};

fn tol() -> Tol {
    Tol::witness()
}

fn budget() -> SymBudget {
    SymBudget { max_terms: 4096, max_degree: 128 }
}

fn eager<R>(f: impl FnOnce() -> R) -> R {
    with_session_rules(budget(), SymRules::shipped(), f).0
}

fn deferring<R>(f: impl FnOnce() -> R) -> R {
    geom_core::sym::deferred::with_deferred_session(
        budget(),
        SymRules::shipped(),
        geom_core::SymRetry::none(),
        f,
    )
    .0
}

fn pf(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
}

fn quad<T: Decide>(x1: T, x2: T, y: T) -> Result<ConstructedLoop<T>, profile::PathError<T>> {
    let z = T::from_f64(0.0);
    Ok(Open
        .at(Point2::new(z, z))
        .line_to(Point2::new(x1, z), tol())?
        .line_to(Point2::new(x2, y), tol())?
        .line_to(Point2::new(z, y), tol())?
        .line_to(Start, tol())?
        .loop_)
}

fn side<T: Decide + Bounds>(x1: T, x2: T) -> [(f64, f64); 2] {
    let lp = quad(x1, x2, T::from_f64(1.0)).expect("the quad builds");
    let vp = ConstructedProfile::new(SketchPlane::xy(), vec![lp]).validate(tol()).expect("validates");
    match vp.loops()[0].segments()[1].directions {
        SegmentDirections::Line(d) => {
            let d: Vec2<T> = d;
            [(d.x.lo(), d.x.hi()), (d.y.lo(), d.y.hi())]
        }
        SegmentDirections::Arc { .. } => panic!("a line"),
    }
}

const AXIS_UP: [(f64, f64); 2] = [(0.0, 0.0), (1.0, 1.0)];

/// **P1 — the branch depends on whether the session saw the nodes
/// minted**: the same document, `(x, 0) -> ((x + w) - w, 1)`, with the
/// identity's operands computed before the session is installed and the
/// hop computed inside it. The question cannot see the outer nodes, so
/// the side takes the quotient (one ulp off the axis at (0.1, 0.3))
/// where the document built wholly inside the session takes the axis.
#[test]
fn p1_nodes_minted_before_the_session_take_the_other_branch() {
    for (x, w) in [(1.0, 1.0), (0.1, 0.3)] {
        let inside = eager(|| {
            let (x, w) = (pf("x", x), pf("w", w));
            side(x, (x + w) - w)
        });
        let (xo, wo) = (pf("x", x), pf("w", w));
        let x2 = (xo + wo) - wo;
        let outside = eager(|| side(xo, x2));
        let outside_d = deferring(|| side(xo, x2));
        println!("P1 ({x},{w}): inside {inside:?} outside {outside:?} deferring {outside_d:?}");
        assert_eq!(inside, AXIS_UP);
    }
}

/// **P2 — order**: the identity asked first, and asked after a large
/// non-identity sharing its subexpression has been asked in the same
/// session (freezing forms in the question's persisted state).
#[test]
fn p2_an_earlier_ask_does_not_move_a_later_answer() {
    use geom_core::Expression;
    let asks = |first_big: bool| {
        eager(|| {
            let (x, y, w) = (pf("x", 0.3), pf("y", 0.7), pf("w", 0.2));
            let mut big = x + y + w;
            for _ in 0..6 {
                big = big * (x + y + Sym::one());
            }
            let proved = |v: Sym<f64>| v.expression().and_then(Expression::proves_zero).is_some();
            let id = ((big + w) - w) - big;
            let id2 = (big * w + big) - big * (w + Sym::one());
            if first_big {
                let _ = proved(big - x);
                let _ = proved(big * big - y);
            }
            [proved(id), proved(id2), proved(big - big * Sym::one())]
        })
    };
    let (cold, warm) = (asks(false), asks(true));
    println!("P2 cold {cold:?} warm {warm:?}");
    assert_eq!(cold, warm, "the order moved an answer");
}

/// **P3 — both components proved zero**: `(W, 0) -> ((W + 1) - 1, 0)`
/// in the middle of a path; what does the verb answer?
#[test]
fn p3_a_hop_proved_zero_in_both_components_is_refused_by_name() {
    for at_box in [false, true] {
        let r = eager(|| {
            let w = if at_box {
                Sym::param(ParamSymbol::new(test_utils::symbol_id("W")), Interval::from_bounds(0.99, 1.01))
            } else {
                Sym::param(ParamSymbol::new(test_utils::symbol_id("W")), Interval::from_bounds(1.0, 1.0))
            };
            let z = Sym::<Interval>::from_f64(0.0);
            let one = Sym::<Interval>::one();
            let lp = Open
                .at(Point2::new(z, z))
                .line_to(Point2::new(w, z), tol())
                .and_then(|p| p.line_to(Point2::new((w + one) - one, z), tol()))
                .map(|_| ());
            format!("{lp:?}")
        });
        println!("P3 box={at_box}: {r}");
        assert!(r.starts_with("Err"), "a zero hop is refused: {r}");
    }
}

/// **P4 — a proved zero whose other component reads Zero without
/// proof**: `(a, 0) -> (b, 0)` with `a`, `b` different parameters equal
/// at the valuation.
#[test]
fn p4_a_coincident_other_component_is_refused_by_name() {
    let r = eager(|| {
        let (a, b) = (pf("a", 1.0), pf("b", 1.0));
        let z = Sym::<f64>::from_f64(0.0);
        let lp = Open
            .at(Point2::new(z, z))
            .line_to(Point2::new(a, z), tol())
            .and_then(|p| p.line_to(Point2::new(b, z), tol()))
            .map(|_| ());
        format!("{lp:?}")
    });
    println!("P4: {r}");
    assert!(r.starts_with("Err"), "{r}");
}
