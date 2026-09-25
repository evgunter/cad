//! **The far placement at the three `Sym` lanes** — the first of the
//! two mechanisms that put a theorem and a definite numeric sign in
//! contradiction: an ordinary body placed far from the origin, where
//! the point channel's rounding exceeds the band while the form's
//! cancellation is exact.
//!
//! Three bodies, each on a sketch plane at `(d, d, d)`: a stadium (two
//! lines, two semicircles) extruded, the M10-9 washer —
//! `sweep::revolve_washer`'s parametric annulus — revolved, and a
//! triangle extruded. The triangle and the placement `d = 3.7e7` are
//! ADOPTED FROM R2's review probe, which drove a body the table did not
//! name at a magnitude between two that it did; they are what found the
//! `(1e-9, 3.7e7)` cell below, where the bare lift refuses all three
//! bodies and the symbolic lanes build two of them with nothing
//! disputed — the tier discharging identities the point channel could
//! not, which no cell of the original table showed.
//!
//! Driven at `Sym<f64>`, `Sym<Probe>` and `Sym<Interval>` inside a
//! session so the receipt counts what the lane did, and at bare `f64`
//! so the same doors are exercised where a shipped run exercises them.
//!
//! **The table is [`TABLE`] and lives there once.** Every claim this
//! file makes about which (ε, `d`) point refuses what is read out of
//! it, so there is no second copy in prose to go stale: what the rows
//! assert is exactly what it says, per lane, per body, completely — a
//! `built` in a cell is as much an assertion as a check name is.
//!
//! One process per ε row: `CAD_TOLERANCE_EPS` is read at runtime, and
//! the rounding that trips this mechanism is measured against the
//! run's band.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::OrthoFrame;
use geom_core::sym::with_session_rules;
use geom_core::{
    Decide, ParamSymbol, Point2, Point3, Real, Sym, SymBudget, SymCounts, SymRules, Tol, Vec2,
};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, Revolution, RevolveAxis};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

/// A stadium — two lines and two semicircles — on the sketch plane at
/// `(d, d, d)`, extruded by one unit along the plane normal: four
/// arc-arm registrants per lamina rim (two arcs × {bottom, top} ×
/// {rim, span}) plus the side walls'.
fn stadium_extrude<T: Decide>(d: T, r: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(d, d, d)));
    let lp = bulge_loop(vec![
        (Point2::new(lit(-1.0), lit(0.0) - r), lit(0.0)),
        (Point2::new(lit(1.0), lit(0.0) - r), lit(0.5)),
        (Point2::new(lit(1.0), r), lit(0.0)),
        (Point2::new(lit(-1.0), r), lit(0.5)),
    ]);
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .map_err(|e| format!("validate: {e:?}"))?;
    sweep::extrude(&vp, Extrusion::Distance(lit(1.0)), Tol::witness())
        .map(|e| e.body.faces().count())
        .map_err(|e| format!("extrude: {e:?}"))
}

/// **The M10-9 washer** — `sweep::revolve_washer`'s parametric
/// annulus, inner radius `r` — on a sketch plane at `(d, d, d)`,
/// revolved a full turn about the sketch's y axis: every vertex is a
/// latitude carrier whose rim identity `revolve::full` /
/// `revolve::surfaces` register.
fn washer_revolve<T: Decide + geom_brep::PcurveFittedLane>(d: T, r: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(d, d, d)));
    let lp = ProfileLoop::polygon([
        Point2::new(r, lit(0.0)),
        Point2::new(lit(2.0), lit(0.0)),
        Point2::new(lit(2.0), lit(1.0)),
        Point2::new(r, lit(1.0)),
    ]);
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .map_err(|e| format!("validate: {e:?}"))?;
    sweep::revolve(
        &vp,
        RevolveAxis {
            origin: Point2::new(lit(0.0), lit(0.0)),
            dir: Vec2::new(lit(0.0), lit(1.0)),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .map(|r| r.body.faces().count())
    .map_err(|e| format!("revolve: {e:?}"))
}

/// **R2's triangle**, adopted from that review's own e2e probe: three
/// straight edges and no arc, so it carries no arc registrant at all —
/// which is why its column moves independently of the other two.
fn triangle_extrude<T: Decide>(d: T, _r: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(d, d, d)));
    let lp = ProfileLoop::polygon([
        Point2::new(lit(0.0), lit(0.0)),
        Point2::new(lit(1.0), lit(0.0)),
        Point2::new(lit(0.3), lit(0.9)),
    ]);
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .map_err(|e| format!("validate: {e:?}"))?;
    sweep::extrude(&vp, Extrusion::Distance(lit(1.0)), Tol::witness())
        .map(|e| e.body.faces().count())
        .map_err(|e| format!("extrude: {e:?}"))
}

/// The three bodies, in the order [`TABLE`]'s cells list them.
const BODIES: [&str; 3] = ["stadium", "washer", "triangle"];

/// What one lane did with the three bodies at one (ε, `d`) point:
/// `built`, or the certification CHECK that refused, exactly as
/// [`outcome`] names it.
struct Lane {
    bodies: [&'static str; 3],
    /// `SymCounts::theorems_disputed` for the session that built all
    /// three. Not read for the bare lift, which has no session.
    disputes: u64,
}

/// One (ε, `d`) point at all four lanes.
struct Cell {
    bare: Lane,
    inexact: Lane,
    exact: Lane,
}

const fn lane(bodies: [&'static str; 3], disputes: u64) -> Lane {
    Lane { bodies, disputes }
}

/// The ε row this run is on, as the index into [`TABLE`]; any other ε
/// has no measured row and fails loud rather than reading a
/// neighbour's (the idiom is `m10_10_pins_interval`'s `eps_row`).
fn eps_row(eps: f64) -> usize {
    [1.0e-6, 1.0e-9, 1.0e-12]
        .iter()
        .position(|&e| (eps / e - 1.0).abs() < 1.0e-3)
        .unwrap_or_else(|| panic!("no measured row at eps = {eps:e}: measure one and add it"))
}

/// The placements, in the order [`TABLE`]'s rows list them: the origin
/// as the control, R2's `3.7e7` between the two the item names, and
/// those two.
const PLACEMENTS: [f64; 4] = [0.0, 1.0e6, 3.7e7, 1.0e9];

/// **THE MEASURED TABLE**, `[eps_row][placement]`, and the only copy of
/// it in this file.
///
/// Reading it across a row says what the mechanism costs as ε tightens;
/// reading it down a column says what it costs as the body moves out.
/// Three things in it are worth naming, because each is an assertion
/// some earlier cut of this file left to prose:
///
/// - **The dispute set is exactly four cells** — `(1e-9, 1e9)` and all
///   three non-origin placements at `1e-12` — and in every one of them
///   the count is `2`: the stadium's and the washer's. The triangle
///   refuses at those points too and disputes nothing.
/// - **At `(1e-9, 3.7e7)` the symbolic lanes BEAT the bare lift**: the
///   bare `f64` run refuses all three bodies, and `Sym<f64>` builds the
///   stadium and the triangle with zero disputes. The tier discharged
///   identities the point channel could not certify — which is the
///   whole point of the tier, and is why "the symbolic lane refuses
///   wherever the bare lane does" would have been a false summary.
/// - **The certified lane is not a superset or a subset of either.**
///   It refuses the washer at `(1e-6, 1e9)` and `(1e-9, 1e6)` where
///   every point lane builds it (the enclosure straddles the band where
///   the point does not), and it builds the triangle everywhere,
///   including the cells where both point lanes refuse it.
#[rustfmt::skip]
const TABLE: [[Cell; 4]; 3] = [
    // ε = 1e-6: nothing refuses anywhere, except the certified lane's
    // washer at the furthest placement.
    [
        Cell { bare: lane(["built", "built", "built"], 0), inexact: lane(["built", "built", "built"], 0), exact: lane(["built", "built", "built"], 0) },
        Cell { bare: lane(["built", "built", "built"], 0), inexact: lane(["built", "built", "built"], 0), exact: lane(["built", "built", "built"], 0) },
        Cell { bare: lane(["built", "built", "built"], 0), inexact: lane(["built", "built", "built"], 0), exact: lane(["built", "built", "built"], 0) },
        Cell { bare: lane(["built", "built", "built"], 0), inexact: lane(["built", "built", "built"], 0), exact: lane(["built", "MappedSource", "built"], 0) },
    ],
    // ε = 1e-9 (the shipped default): the mechanism reaches the point
    // lanes at 1e9, and at 3.7e7 the tier rescues two bodies the bare
    // lift refuses.
    [
        Cell { bare: lane(["built", "built", "built"], 0), inexact: lane(["built", "built", "built"], 0), exact: lane(["built", "built", "built"], 0) },
        Cell { bare: lane(["built", "built", "built"], 0), inexact: lane(["built", "built", "built"], 0), exact: lane(["built", "MappedSource", "built"], 0) },
        Cell { bare: lane(["Surface2Residual", "MappedSource", "MappedSource"], 0), inexact: lane(["built", "MappedSource", "built"], 0), exact: lane(["Surface1Residual", "EndpointEnd", "built"], 0) },
        Cell { bare: lane(["Surface2Residual", "MappedSource", "MappedSource"], 0), inexact: lane(["Surface2Residual", "MappedSource", "MappedSource"], 2), exact: lane(["Surface1Residual", "EndpointEnd", "built"], 0) },
    ],
    // ε = 1e-12: every placement off the origin disputes.
    [
        Cell { bare: lane(["built", "built", "built"], 0), inexact: lane(["built", "built", "built"], 0), exact: lane(["built", "built", "built"], 0) },
        Cell { bare: lane(["Surface2Residual", "MappedSource", "MappedSource"], 0), inexact: lane(["Surface2Residual", "MappedSource", "MappedSource"], 2), exact: lane(["Surface1Residual", "EndpointEnd", "built"], 0) },
        Cell { bare: lane(["Surface2Residual", "MappedSource", "MappedSource"], 0), inexact: lane(["Surface2Residual", "MappedSource", "MappedSource"], 2), exact: lane(["Surface1Residual", "EndpointEnd", "built"], 0) },
        Cell { bare: lane(["Surface2Residual", "MappedSource", "MappedSource"], 0), inexact: lane(["Surface2Residual", "MappedSource", "MappedSource"], 2), exact: lane(["Surface1Residual", "EndpointEnd", "built"], 0) },
    ],
];

fn cell(d: f64) -> &'static Cell {
    let placement = PLACEMENTS
        .iter()
        .position(|&p| p == d)
        .expect("every placement this file drives has a row in TABLE");
    &TABLE[eps_row(Tol::witness().eps())][placement]
}

/// `built`, or the certification CHECK a refusal named — the one thing
/// about an outcome this file pins, because the sample index and the
/// enclosure's digits in the rest of the message are a document's
/// arithmetic and not its verdict.
fn outcome(r: &Result<usize, String>) -> String {
    match r {
        Ok(_) => "built".to_owned(),
        Err(e) => e
            .split("check: ")
            .nth(1)
            .and_then(|t| t.split(|c: char| !c.is_alphanumeric()).next())
            .unwrap_or("refused")
            .to_owned(),
    }
}

fn build_all<T>(d: Sym<T>, r: Sym<T>) -> [Result<usize, String>; 3]
where
    T: Real + Decide,
    Sym<T>: geom_brep::PcurveFittedLane,
{
    [
        stadium_extrude(d, r * Sym::from_f64(0.5)),
        washer_revolve(d, r),
        triangle_extrude(d, r),
    ]
}

struct Built {
    outcomes: [Result<usize, String>; 3],
    counts: SymCounts,
}

/// One placement's three bodies at one `Sym` lane, ON ITS OWN THREAD.
/// A panic inside `with_session_rules` leaves that thread's session
/// installed and the next placement would refuse to nest, so the
/// thread is what makes the next case runnable — and since SYM-11 it
/// is what catches a REGRESSION rather than a design
/// ([`test_utils::own_thread`], which keeps the panic's own message).
fn drive_on_a_thread<T>(d: f64, lift: fn(f64) -> (Sym<T>, Sym<T>)) -> Result<Built, String>
where
    T: Real + Decide + geom_brep::PcurveFittedLane + Send + 'static,
    Sym<T>: geom_brep::PcurveFittedLane,
{
    test_utils::own_thread::caught(move || {
        let (outcomes, counts) = with_session_rules(budget(), SymRules::shipped(), || {
            let (dd, rr) = lift(d);
            build_all(dd, rr)
        });
        Built { outcomes, counts }
    })
}

/// Asserts one lane's three outcomes and its dispute count against the
/// cell, printing the measured row either way.
fn assert_lane(name: &str, d: f64, want: &Lane, got: &[Result<usize, String>; 3], disputes: u64) {
    let eps = Tol::witness().eps();
    for ((body, expected), r) in BODIES.iter().zip(want.bodies).zip(got) {
        let measured = outcome(r);
        println!("   [{name}] eps={eps:e} d={d:e} {body}: {measured}");
        assert_eq!(
            measured, expected,
            "[{name}] eps={eps:e} d={d:e}: the {body} answered `{measured}` where TABLE says \
             `{expected}`. A cell that moved is this document's verdict moving, which is a \
             finding and not a number to refresh: {r:?}"
        );
    }
    assert_eq!(
        disputes, want.disputes,
        "[{name}] eps={eps:e} d={d:e}: {disputes} theorem(s) disputed where TABLE says {}. \
         The dispute count is the point channel's rounding exceeding the band on a form the \
         tier proved zero, so a count that moved means the rounding, the band or the fold \
         did",
        want.disputes
    );
}

/// **The bare lifts**, which is where a shipped run exercises these
/// doors: no session, nothing counted, and the column that says what
/// the placement costs downstream anyway. Its refusals are TYPED at
/// exactly the points [`TABLE`] names and it builds everywhere else.
#[test]
fn sym11_the_bare_lifts_answer_typed_at_every_placement() {
    for d in PLACEMENTS {
        let outs = [
            stadium_extrude::<f64>(d, 0.5),
            washer_revolve::<f64>(d, 1.0),
            triangle_extrude::<f64>(d, 1.0),
        ];
        assert_lane("f64", d, &cell(d).bare, &outs, 0);
    }
}

/// `Sym<f64>` — the inexact witness in a session. Where [`TABLE`]
/// expects no dispute the documents BUILD (or refuse for a reason of
/// their own), and where it expects one the numeric answer is what
/// comes back: before SYM-11 the four disputing cells aborted here and
/// this row had to be `#[ignore]`d.
#[test]
fn sym11_the_far_placement_is_a_counted_dispute_at_sym_f64() {
    for d in PLACEMENTS {
        let built = drive_on_a_thread::<f64>(d, |d| {
            (
                Sym::param(ParamSymbol::of("d"), d),
                Sym::param(ParamSymbol::of("r"), 1.0),
            )
        })
        .unwrap_or_else(|m| {
            panic!(
                "[Sym<f64>] d={d:e}: an INEXACT witness panicked on a theorem-vs-numeric \
                 contradiction — the point channel is not a proof and the charge at this \
                 scalar is a count, not an assertion: {m}"
            )
        });
        assert_lane(
            "Sym<f64>",
            d,
            &cell(d).inexact,
            &built.outcomes,
            built.counts.theorems_disputed,
        );
    }
}

/// `Sym<Probe>` — the recording scalar, whose value channel IS an
/// `f64`, so the same rounding reaches the same door and the table's
/// inexact column is its column too.
#[cfg(feature = "probe")]
#[test]
fn sym11_the_far_placement_is_a_counted_dispute_at_sym_probe() {
    use geom_core::Probe;
    for d in PLACEMENTS {
        let built = drive_on_a_thread::<Probe>(d, |d| {
            (
                Sym::param(ParamSymbol::of("d"), <Probe as Real>::from_f64(d)),
                Sym::param(ParamSymbol::of("r"), <Probe as Real>::from_f64(1.0)),
            )
        })
        .unwrap_or_else(|m| {
            panic!(
                "[Sym<Probe>] d={d:e}: an INEXACT witness panicked on a theorem-vs-numeric \
                 contradiction: {m}"
            )
        });
        assert_lane(
            "Sym<Probe>",
            d,
            &cell(d).inexact,
            &built.outcomes,
            built.counts.theorems_disputed,
        );
    }
}

/// `Sym<Interval>` — the EXACT witness, and the lane the driver
/// replays in: a definite non-zero sign here is a certified proof, so
/// the contradiction is a soundness question and the assertion is
/// right. What this row PINS is the charge's arm at this scalar — the
/// count is zero in every cell because `Interval::WITNESS` is `Exact`,
/// which routes a contradiction to the `debug_assert!` and never to
/// the column, so a count here is that const having moved.
#[test]
fn sym11_the_far_placement_never_contradicts_at_sym_interval() {
    use geom_core::Interval;
    for d in PLACEMENTS {
        let built = drive_on_a_thread::<Interval>(d, |d| {
            let eps = Tol::witness().eps();
            (
                Sym::param(
                    ParamSymbol::of("d"),
                    Interval::from_bounds(d - eps / 64.0, d + eps / 64.0),
                ),
                Sym::param(
                    ParamSymbol::of("r"),
                    Interval::from_bounds(1.0 - eps / 64.0, 1.0 + eps / 64.0),
                ),
            )
        })
        .unwrap_or_else(|m| {
            panic!(
                "d={d:e}: the EXACT witness contradicted its own form — a certified bracket \
                 that excludes zero and a form that is the zero polynomial cannot both be \
                 right, so one of the two channels does not contain its real. That is a \
                 soundness defect in a channel and a DIFFERENT unit: {m}"
            )
        });
        assert_lane(
            "Sym<Interval>",
            d,
            &cell(d).exact,
            &built.outcomes,
            built.counts.theorems_disputed,
        );
    }
}
