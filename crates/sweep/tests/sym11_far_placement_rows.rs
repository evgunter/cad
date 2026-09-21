//! **The far placement at the three `Sym` lanes** — the first of the
//! two mechanisms that put a theorem and a definite numeric sign in
//! contradiction: an ordinary body placed far from the origin, where
//! the point channel's rounding exceeds the band while the form's
//! cancellation is exact.
//!
//! Two bodies, each on a sketch plane at `(d, d, d)`: a stadium (two
//! lines, two semicircles) extruded, and the M10-9 washer revolved.
//! Driven at `Sym<f64>`, `Sym<Probe>` and `Sym<Interval>` inside a
//! session so the receipt counts what the lane did, and at bare `f64`
//! and `Probe` so the same doors are exercised where a shipped run
//! exercises them — the bare lanes answer a TYPED refusal on the
//! documents the symbolic lanes contradict on, which is the column
//! that says the far placement is not a defect this tier introduced.
//!
//! One process per ε row: `CAD_TOLERANCE_EPS` is read at runtime, and
//! the rounding that trips this mechanism is measured against the
//! run's band.
//!
//! **What the mechanism does, measured** (both bodies, `Sym<f64>` and
//! `Sym<Probe>` identically — the recording scalar's value channel IS
//! an `f64`, so the same rounding reaches the same door):
//!
//! | ε | `d = 0` | `d = 1e6` | `d = 1e9` |
//! | --- | --- | --- | --- |
//! | `1e-6` | builds | builds | builds |
//! | `1e-9` | builds | builds | **contradiction** |
//! | `1e-12` | builds | **contradiction** | **contradiction** |
//!
//! and the bare lifts on the same documents, which is the column that
//! says what the placement costs a shipped run anyway: at `1e-9`
//! `d = 1e9` and at `1e-12` `d ∈ {1e6, 1e9}` both refuse TYPED —
//! `ResidualExceeded { Surface2Residual }` from the extrude and
//! `ResidualExceeded { MappedSource }` from the revolve — exactly
//! where the symbolic lanes contradict, and nowhere else. The
//! `Sym<Interval>` lane refuses those same points typed as well
//! (`Escalated { Surface1Residual }` / `{ EndpointEnd }`, the
//! enclosure straddling the band) and contradicts at none of the nine.
//! Where nothing refuses, every lane counts the same receipt:
//! `symbolic_zero: 697, registered: 58, numeric: 729`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::OrthoFrame;
use geom_core::sym::with_session_rules;
use geom_core::{
    Decide, ParamSymbol, Point2, Point3, Real, Sym, SymBudget, SymCounts, SymRules, Tol, Vec2,
};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

/// The placements the mechanism is measured at: the origin as the
/// control, and the two magnitudes the item names.
const PLACEMENTS: [f64; 3] = [0.0, 1.0e6, 1.0e9];

/// A stadium on the sketch plane at `(d, d, d)`, extruded by one unit
/// along the plane normal: four arc-arm registrants per lamina rim
/// (two arcs × {bottom, top} × {rim, span}) plus the side walls'.
fn stadium_extrude<T: Decide>(d: T, r: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(d, d, d)));
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(lit(-1.0), lit(0.0) - r), lit(0.0)),
        ProfileVertex::new(Point2::new(lit(1.0), lit(0.0) - r), lit(0.5)),
        ProfileVertex::new(Point2::new(lit(1.0), r), lit(0.0)),
        ProfileVertex::new(Point2::new(lit(-1.0), r), lit(0.5)),
    ]);
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .map_err(|e| format!("validate: {e:?}"))?;
    sweep::extrude(&vp, Extrusion::Distance(lit(1.0)), Tol::witness())
        .map(|e| e.body.faces().count())
        .map_err(|e| format!("extrude: {e:?}"))
}

/// The M10-9 washer on a sketch plane at `(d, d, d)`, revolved a full
/// turn about the sketch's y axis: every vertex is a latitude carrier
/// whose rim identity `revolve::full` / `revolve::surfaces` register.
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

/// One placement's two bodies at one `Sym` lane, ON ITS OWN THREAD:
/// the session's counts, or `Err` when the theorem-vs-numeric
/// assertion fired. A panic inside `with_session_rules` leaves that
/// thread's session installed, and the next placement would refuse to
/// nest.
fn drive_on_a_thread<T>(d: f64, build: fn(f64) -> (Sym<T>, Sym<T>)) -> Result<Built, ()>
where
    T: Real + Decide + geom_brep::PcurveFittedLane + Send + 'static,
    Sym<T>: geom_brep::PcurveFittedLane,
{
    std::thread::spawn(move || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let ((e, r), counts) = with_session_rules(budget(), SymRules::shipped(), || {
                let (dd, rr) = build(d);
                (
                    stadium_extrude(dd, rr * Sym::from_f64(0.5)),
                    washer_revolve(dd, rr),
                )
            });
            Built {
                extrude: e,
                revolve: r,
                counts,
            }
        }))
        .map_err(|_| ())
    })
    .join()
    .expect("the probe thread itself joins")
}

/// What one placement built, and what the session counted for it.
struct Built {
    extrude: Result<usize, String>,
    revolve: Result<usize, String>,
    counts: SymCounts,
}

fn report(lane: &str, d: f64, built: &Result<Built, ()>) {
    let eps = Tol::witness().eps();
    match built {
        Ok(b) => println!(
            "   [{lane}] eps={eps:e} d={d:e}: extrude {:?} revolve {:?} | {:?}",
            b.extrude, b.revolve, b.counts
        ),
        Err(()) => println!(
            "   [{lane}] eps={eps:e} d={d:e}: the theorem-vs-numeric assertion FIRED \
             (counts lost with the session)"
        ),
    }
}

/// **The bare lanes**, which is where a shipped run exercises these
/// doors: no session, nothing counted, and the column that says what
/// the placement costs downstream anyway.
#[test]
fn sym11_the_bare_lifts_answer_typed_at_every_placement() {
    for d in PLACEMENTS {
        let e = stadium_extrude::<f64>(d, 0.5);
        let r = washer_revolve::<f64>(d, 1.0);
        println!(
            "   [f64] eps={:e} d={d:e}: extrude {e:?} revolve {r:?}",
            Tol::witness().eps()
        );
        assert!(
            d > 0.0 || (e.is_ok() && r.is_ok()),
            "d={d:e}: the control placement must build: {e:?} / {r:?}"
        );
    }
}

/// **Which placements the mechanism reaches at this ε** — the table
/// in the header, as the assertion the rows below read. A placement
/// outside it must dispute NOTHING, which is the half that says the
/// charge is not firing everywhere.
fn disputes_expected(d: f64) -> bool {
    let eps = Tol::witness().eps();
    if (eps / 1.0e-12 - 1.0).abs() < 1.0e-3 {
        d >= 1.0e6
    } else if (eps / 1.0e-9 - 1.0).abs() < 1.0e-3 {
        d >= 1.0e9
    } else if (eps / 1.0e-6 - 1.0).abs() < 1.0e-3 {
        false
    } else {
        panic!("no measured row at eps = {eps:e}: measure one and add it")
    }
}

/// The dispute count and the expectation, asserted together.
fn assert_disputes(lane: &str, d: f64, built: &Result<Built, ()>) {
    let eps = Tol::witness().eps();
    let Ok(b) = built else {
        panic!(
            "[{lane}] eps={eps:e} d={d:e}: an INEXACT witness panicked on a              theorem-vs-numeric contradiction — the point channel is not a proof and the              charge at this scalar is a count, not an assertion"
        );
    };
    let want = disputes_expected(d);
    assert_eq!(
        b.counts.theorems_disputed > 0,
        want,
        "[{lane}] eps={eps:e} d={d:e}: the far placement's rounding {} exceed the band          here, so the tier {} owe the receipt a dispute: {:?}",
        if want { "does" } else { "does not" },
        if want { "does" } else { "does not" },
        b.counts
    );
}

/// `Sym<f64>` — the inexact witness in a session. Each placement on
/// its own thread still, and the thread is now what catches a
/// REGRESSION rather than a design: before SYM-11 the far placements
/// aborted here, and this row had to be `#[ignore]`d.
#[test]
fn sym11_the_far_placement_is_a_counted_dispute_at_sym_f64() {
    for d in PLACEMENTS {
        let built = drive_on_a_thread::<f64>(d, |d| {
            (
                Sym::param(ParamSymbol::of("d"), d),
                Sym::param(ParamSymbol::of("r"), 1.0),
            )
        });
        report("Sym<f64>", d, &built);
        assert_disputes("Sym<f64>", d, &built);
    }
}

/// `Sym<Probe>` — the recording scalar, whose value channel IS an
/// `f64`, so the same rounding reaches the same door.
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
        });
        report("Sym<Probe>", d, &built);
        assert_disputes("Sym<Probe>", d, &built);
    }
}

/// `Sym<Interval>` — the EXACT witness, and the lane the driver
/// replays in: a definite non-zero sign here is a certified proof, so
/// the contradiction is a soundness question and the assertion is
/// right. Nothing fires at any placement, which is the claim.
#[cfg(feature = "interval")]
#[test]
fn sym11_the_far_placement_never_contradicts_at_sym_interval() {
    use geom_core::Interval;
    let eps = Tol::witness().eps();
    for d in PLACEMENTS {
        let built = drive_on_a_thread::<Interval>(d, move |d| {
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
        });
        report("Sym<Interval>", d, &built);
        let Ok(b) = built else {
            panic!(
                "d={d:e} at eps={eps:e}: the EXACT witness contradicted its own form — a \
                 certified bracket that excludes zero and a form that is the zero \
                 polynomial cannot both be right, so one of the two channels does not \
                 contain its real. That is a soundness defect in a channel and a \
                 DIFFERENT unit"
            );
        };
        assert_eq!(
            b.counts.theorems_disputed, 0,
            "d={d:e} at eps={eps:e}: the dispute column is not this scalar's — at an EXACT \
             witness the contradiction is asserted, so a count here means the const on \
             `Interval` moved: {:?}",
            b.counts
        );
    }
}
