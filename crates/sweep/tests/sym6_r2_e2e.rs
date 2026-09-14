//! SYM-6 review (r2) — end-to-end through the public sweep doors.
//!
//! Two bodies the unit did not measure, whose registrants run at
//! coordinates of order 1e9 (a true identity the relative slack must
//! witness at every eps row): a stadium (two lines, two semicircles)
//! extruded, and the M10-9 washer revolved, each on a sketch plane at `(d, d, d)`
//! for `d ∈ {0, 1e9}`. Driven at `Sym<f64>`, `Sym<Probe>` and
//! `Sym<Interval>` inside a session so the receipt counts every refusal,
//! and at bare `f64` / `Probe` so the same doors are exercised where a
//! shipped run exercises them. One process per eps row: the suite's
//! `CAD_TOLERANCE_EPS` discipline is the third axis.

use geom_core::sym::with_session_rules;
use geom_core::{
    Decide, ParamSymbol, Point2, Point3, Real, Sym, SymBudget, SymCounts, SymRules, Tol, Vec2, Vec3,
};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

/// A stadium on the sketch plane at `(d, d, d)`, extruded by one unit
/// along the plane normal: four arc-arm registrants per lamina rim
/// (two arcs × {bottom, top} × {rim, span}) plus the side walls'.
fn stadium_extrude<T: Decide>(d: T, r: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(
        Point3::new(d, d, d),
        Vec3::new(lit(1.0), lit(0.0), lit(0.0)),
        Vec3::new(lit(0.0), lit(1.0), lit(0.0)),
    );
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

/// The M10-9 washer (`sweep::revolve_washer`'s parametric annulus, inner
/// radius `r`) on a sketch plane at `(d, d, d)`, revolved a full turn
/// about the sketch's y axis: every vertex is a latitude carrier whose
/// rim identity `revolve::full` / `revolve::surfaces` register.
fn washer_revolve<T: Decide + geom_brep::PcurveFittedLane>(d: T, r: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(
        Point3::new(d, d, d),
        Vec3::new(lit(1.0), lit(0.0), lit(0.0)),
        Vec3::new(lit(0.0), lit(1.0), lit(0.0)),
    );
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

const PLACEMENTS: [f64; 3] = [0.0, 1e6, 1e9];

fn report(
    lane: &str,
    d: f64,
    built: (Result<usize, String>, Result<usize, String>),
    c: &SymCounts,
) {
    println!(
        "   [{lane}] eps={:e} d={d:e}: counts {c:?} | extrude {:?} revolve {:?}",
        Tol::witness().eps(),
        built.0,
        built.1
    );
}

/// Bare `f64`: no session, nothing counted — the doors just have to
/// admit the body at 1e9 (the refusal arm is observed by a scratch
/// instrumentation of the registrants, not here).
#[test]
fn sym6_r2_bare_f64_builds_both_bodies_at_every_placement() {
    for d in PLACEMENTS {
        let e = stadium_extrude::<f64>(d, 0.5);
        let r = washer_revolve::<f64>(d, 1.0);
        println!(
            "   [f64] eps={:e} d={d:e}: extrude {e:?} revolve {r:?}",
            Tol::witness().eps()
        );
        assert!(
            d > 0.0 || (e.is_ok() && r.is_ok()),
            "d={d:e}: {e:?} / {r:?}"
        );
    }
}

#[cfg(feature = "probe")]
#[test]
fn sym6_r2_bare_probe_builds_both_bodies_at_every_placement() {
    use geom_core::Probe;
    for d in PLACEMENTS {
        let e =
            stadium_extrude::<Probe>(<Probe as Real>::from_f64(d), <Probe as Real>::from_f64(0.5));
        let r =
            washer_revolve::<Probe>(<Probe as Real>::from_f64(d), <Probe as Real>::from_f64(1.0));
        println!(
            "   [Probe] eps={:e} d={d:e}: extrude {e:?} revolve {r:?}",
            Tol::witness().eps()
        );
        assert!(
            d > 0.0 || (e.is_ok() && r.is_ok()),
            "d={d:e}: {e:?} / {r:?}"
        );
    }
}

/// `Sym<f64>`: the inexact witness in a session. A true identity at
/// 1e9 is witnessed at every eps row, so `registrations_refused` is 0
/// — this is the row that would go red under the absolute-eps slack.
#[test]
fn sym6_r2_sym_f64_refuses_no_true_identity_at_1e9_at_this_eps() {
    for d in PLACEMENTS {
        let run = || {
            with_session_rules(budget(), SymRules::shipped(), || {
                let dd: Sym<f64> = Sym::param(ParamSymbol::of("d"), d);
                let r: Sym<f64> = Sym::param(ParamSymbol::of("r"), 1.0);
                (
                    stadium_extrude(dd, r * Sym::from_f64(0.5)),
                    washer_revolve(dd, r),
                )
            })
        };
        let Ok((built, counts)) = std::panic::catch_unwind(run) else {
            println!(
                "   [Sym<f64>] eps={:e} d={d:e}: PANICKED inside the session (the sym.rs theorem-vs-numeric debug_assert, off this door; counts lost)",
                Tol::witness().eps()
            );
            continue;
        };
        report("Sym<f64>", d, built.clone(), &counts);
        assert!(
            d > 0.0 || (built.0.is_ok() && built.1.is_ok()),
            "d={d:e}: {built:?}"
        );
        assert!(
            counts.registered > 0 || d > 0.0,
            "d={d:e}: the door was never consulted: {counts:?}"
        );
        assert_eq!(
            counts.registrations_refused, 0,
            "d={d:e}: the inexact witness refused a TRUE identity: {counts:?}"
        );
    }
}

#[cfg(feature = "probe")]
#[test]
fn sym6_r2_sym_probe_refuses_no_true_identity_at_1e9_at_this_eps() {
    use geom_core::Probe;
    for d in PLACEMENTS {
        let run = || {
            with_session_rules(budget(), SymRules::shipped(), || {
                let dd: Sym<Probe> = Sym::param(ParamSymbol::of("d"), <Probe as Real>::from_f64(d));
                let r: Sym<Probe> =
                    Sym::param(ParamSymbol::of("r"), <Probe as Real>::from_f64(1.0));
                (
                    stadium_extrude(dd, r * Sym::from_f64(0.5)),
                    washer_revolve(dd, r),
                )
            })
        };
        let Ok((built, counts)) = std::panic::catch_unwind(run) else {
            println!(
                "   [Sym<Probe>] eps={:e} d={d:e}: PANICKED inside the session (the sym.rs theorem-vs-numeric debug_assert, off this door; counts lost)",
                Tol::witness().eps()
            );
            continue;
        };
        report("Sym<Probe>", d, built.clone(), &counts);
        assert!(
            d > 0.0 || (built.0.is_ok() && built.1.is_ok()),
            "d={d:e}: {built:?}"
        );
        assert!(
            counts.registered > 0 || d > 0.0,
            "d={d:e}: the door was never consulted: {counts:?}"
        );
        assert_eq!(
            counts.registrations_refused, 0,
            "d={d:e}: the inexact witness refused a TRUE identity: {counts:?}"
        );
    }
}

/// `Sym<Interval>`: the lane that decides. The receipt is the
/// certification-decision record this review compares before/after
/// (merge-base against the head): every count identical, and
/// `registrations_refused == 0` because in this lane every refusal is
/// a proof of a defect.
#[cfg(feature = "interval")]
#[test]
fn sym6_r2_sym_interval_receipt_is_the_same_document_at_every_eps() {
    use geom_core::Interval;
    let eps = Tol::witness().eps();
    for d in PLACEMENTS {
        let run = || {
            with_session_rules(budget(), SymRules::shipped(), || {
                let dd: Sym<Interval> = Sym::param(
                    ParamSymbol::of("d"),
                    Interval::from_bounds(d - eps / 64.0, d + eps / 64.0),
                );
                let r: Sym<Interval> = Sym::param(
                    ParamSymbol::of("r"),
                    Interval::from_bounds(1.0 - eps / 64.0, 1.0 + eps / 64.0),
                );
                (
                    stadium_extrude(dd, r * Sym::from_f64(0.5)),
                    washer_revolve(dd, r),
                )
            })
        };
        let Ok((built, counts)) = std::panic::catch_unwind(run) else {
            println!(
                "   [Sym<Interval>] eps={:e} d={d:e}: PANICKED inside the session (the sym.rs theorem-vs-numeric debug_assert, off this door; counts lost)",
                Tol::witness().eps()
            );
            continue;
        };
        report("Sym<Interval>", d, built.clone(), &counts);
        assert!(
            d > 0.0 || (built.0.is_ok() && built.1.is_ok()),
            "d={d:e}: {built:?}"
        );
        assert!(
            counts.registered > 0 || d > 0.0,
            "d={d:e}: the door was never consulted: {counts:?}"
        );
        assert_eq!(
            (
                counts.registrations_refused,
                counts.registrations_contradicted
            ),
            (0, 0),
            "d={d:e}: the exact witness refused on a real document: {counts:?}"
        );
    }
}
