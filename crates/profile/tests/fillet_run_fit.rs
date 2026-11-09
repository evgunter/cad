//! **A fillet's run is decided once, on the band validation reads it
//! on.** `fillet_leg_fit` reads the run a fillet leaves on each leg —
//! the leg's length less the setback — against the run's linear band,
//! the band validation's `vertex_separation` reads a segment's length
//! on. So a run validation would call degenerate is a Zero fit and no
//! segment is emitted for it, a run in the band between escalates at
//! the fit, and only a definitely positive run is built.
//!
//! Each row would go red if the fit were read on a narrower band than
//! validation's: the sub-band rows would build a run and validation
//! would refuse it `DegenerateSegment`, or escalate `vertex_separation`
//! after the build instead of at the fit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{
    ArcSide, ClosedLoop, ConstructedProfile, Open, PathError, SketchPlane, Start, Sweep,
    ValidatedProfile,
};

fn eps() -> f64 {
    crate::common::tol().eps()
}

fn validated(c: &ClosedLoop<f64>, what: &str) -> ValidatedProfile<f64> {
    ConstructedProfile::new(SketchPlane::xy(), vec![c.loop_.clone()])
        .validate(crate::common::tol())
        .unwrap_or_else(|e| panic!("{what}: the loop validates, got {e:?}"))
}

/// The predicate an in-band escalation names, or a panic naming what
/// came instead.
fn escalated_on(r: Result<ClosedLoop<f64>, PathError<f64>>, what: &str) -> &'static str {
    match r {
        Err(PathError::Escalated { source, .. }) => source
            .predicate
            .unwrap_or_else(|| panic!("{what}: the escalation names its predicate")),
        other => panic!("{what}: expected an in-band escalation, got {other:?}"),
    }
}

#[derive(Clone, Copy, Debug)]
enum Before {
    Nothing,
    Tangent,
}

/// `line(len)` east from the origin, then `before`, then `fillet(r)`,
/// arriving up the line x = `x_arr` to (`x_arr`, 2), closed.
fn straight(
    before: Before,
    len: f64,
    r: f64,
    x_arr: f64,
) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    let leg = Open.at(Point2::new(0.0, 0.0)).angle(0.0, t)?.line(len, t)?;
    match before {
        Before::Nothing => leg.fillet(r, t)?,
        Before::Tangent => leg.tangent().fillet(r, t)?,
    }
    .toward(0.0, 1.0, t)?
    .to(Point2::new(x_arr, 2.0), t)?
    .line_to(Start, t)
}

/// **A run below the band is no run.** `line(2).fillet(0.5)` arriving at
/// x = 2.5 + d leaves a run of d: at and below a tenth of ε it is a Zero
/// fit, the arc springs off the leg's end (2, 0), and the loop holds
/// four vertices and validates — bare and after `.tangent()` alike.
#[test]
fn a_run_below_the_band_is_a_zero_fit() {
    for before in [Before::Nothing, Before::Tangent] {
        for d in [0.0, -1e-6, 1e-6, 1e-4, 1e-3, 1e-2, 0.1].map(|k| k * eps()) {
            let what = format!("{before:?} d = {d:e}");
            let c = straight(before, 2.0, 0.5, 2.5 + d).unwrap_or_else(|e| panic!("{what}: {e}"));
            let end = c.loop_.as_loop().vertices()[1];
            assert!(
                end.x == 2.0 && end.y == 0.0,
                "{what}: the leg keeps its end, got {end:?}"
            );
            assert_eq!(c.loop_.as_loop().vertices().len(), 4, "{what}: no run");
            assert_eq!(
                c.structure.fillets[0].fit_in,
                geom_core::Sign::Zero,
                "{what}"
            );
            validated(&c, &what);
        }
    }
}

/// **A run in the band escalates at the fit**, before anything is
/// built: at d = 2ε and 5ε the run is neither a segment validation
/// would keep nor one it would drop. A run well past the band (100ε)
/// is a segment of its own, and the loop holds five vertices.
#[test]
fn a_run_in_the_band_escalates_at_the_fit_and_one_past_it_is_built() {
    for before in [Before::Nothing, Before::Tangent] {
        for d in [2.0, 5.0].map(|k| k * eps()) {
            assert_eq!(
                escalated_on(
                    straight(before, 2.0, 0.5, 2.5 + d),
                    &format!("{before:?} d = {d:e}")
                ),
                "fillet_leg_fit"
            );
        }
        let what = format!("{before:?} d = 100ε");
        let c = straight(before, 2.0, 0.5, 2.5 + 100.0 * eps()).expect("builds");
        assert_eq!(c.loop_.as_loop().vertices().len(), 5, "{what}: the run");
        validated(&c, &what);
    }
}

/// **A decimal-authored corner**: `line(1.0).fillet(0.1)` arriving at
/// x = 1.1, where the run is the few ulps 1.1 − 0.1 − 1.0 rounds to.
#[test]
fn a_decimal_corner_rounds_to_no_run() {
    for before in [Before::Nothing, Before::Tangent] {
        let what = format!("{before:?}");
        let c = straight(before, 1.0, 0.1, 1.1).unwrap_or_else(|e| panic!("{what}: {e}"));
        assert_eq!(c.loop_.as_loop().vertices().len(), 4, "{what}: no run");
        validated(&c, &what);
    }
}

/// **The rounded-corner grid**: a leg from (r, 0) of length w − 2r ends
/// where the corner's trim falls, so every run is a rounding residue.
/// Every one of the 360 cases builds with no run (five vertices) and
/// validates.
#[test]
fn every_rounded_corner_of_the_grid_builds_without_a_run_and_validates() {
    let t = Tol::witness();
    let mut cases = 0;
    for wi in 1..=40 {
        for ri in 1..=9 {
            let w = f64::from(wi) * 0.1 + 0.3;
            let r = f64::from(ri) * 0.01 * f64::from(wi) / 4.0;
            if 2.0 * r >= w {
                continue;
            }
            cases += 1;
            let what = format!("w = {w}, r = {r}");
            let c = Open
                .at(Point2::new(r, 0.0))
                .angle(0.0, t)
                .and_then(|x| x.line(w - 2.0 * r, t))
                .and_then(|x| x.fillet(r, t))
                .and_then(|x| x.toward(0.0, 1.0, t))
                .and_then(|x| x.to(Point2::new(w, 1.0), t))
                .and_then(|x| x.line_to(Point2::new(0.0, 1.0), t))
                .and_then(|x| x.line_to(Start, t))
                .unwrap_or_else(|e| panic!("{what}: {e}"));
            assert_eq!(c.loop_.as_loop().vertices().len(), 5, "{what}: no run");
            validated(&c, &what);
        }
    }
    assert_eq!(cases, 360, "the grid's cases");
}

/// Straight up x = 2 to the tip (2, 0), a tangent arc of radius 2 about
/// the origin through `a` (none at `a = 0`), then `arc_fillet(Radius {
/// r: 2 })` — the same circle — with a fillet of radius `rf` whose
/// straight arrival touches it at the tip when `rf = 1`.
fn arc_extension_at_the_tip(a: f64, rf: f64) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    let (c, s) = (a.cos(), a.sin());
    let rot = |x: f64, y: f64| Point2::new(c * x - s * y, s * x + c * y);
    let (anchor, d) = (rot(-1.0, 1.0), rot(-1.0, 0.0));
    let leg = Open
        .at(Point2::new(2.0, -1.0))
        .toward(0.0, 1.0, t)?
        .line(1.0, t)?;
    let tip = if a == 0.0 {
        leg
    } else {
        leg.tangent().arc_to(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: a,
            },
            t,
        )?
    };
    tip.arc_fillet(
        profile::Radius {
            r: 2.0,
            side: ArcSide::Left,
        },
        rf,
        t,
    )?
    .toward(d.x, d.y, t)?
    .at(anchor, t)?
    .line(1.0, t)?
    .line_to(Start, t)
}

/// **An arc extension's run is decided the same way.** Where the
/// fillet touches the extended circle at the tip itself, the run on the
/// circle has no length: no arc is emitted for it and the fillet arc
/// springs off the tip (one vertex fewer than the leg, the arc leg, the
/// fillet arc, the arrival and the close would hold). At `rf` an ε off
/// the touch the run is in the band, and the fit escalates.
#[test]
fn an_arc_extension_run_below_the_band_is_a_zero_fit() {
    for a in [0.0, 0.3, 0.5] {
        // The straight leg, the arc leg unless `a = 0`, the fillet arc,
        // the arrival and the close: vertices = segments.
        let want = if a == 0.0 { 4 } else { 5 };
        for rf in [1.0, 1.0 - 1e-3 * eps(), 1.0 + 1e-3 * eps()] {
            let what = format!("a = {a}, rf = 1 {:+e}", rf - 1.0);
            let c = arc_extension_at_the_tip(a, rf).unwrap_or_else(|e| panic!("{what}: {e}"));
            assert_eq!(c.loop_.as_loop().vertices().len(), want, "{what}: no run");
            let vp = validated(&c, &what);
            assert!(vp.loops()[0].decided_joints().is_empty(), "{what}");
        }
        assert_eq!(
            escalated_on(
                arc_extension_at_the_tip(a, 1.0 - 2.0 * eps()),
                &format!("a = {a}, rf = 1 − 2ε")
            ),
            "fillet_leg_fit"
        );
    }
}

/// **The outgoing side is the same decision.** The arrival's far end
/// a tenth of ε past the fillet's tangent point (2.5, 0.5) leaves a run
/// out validation would call degenerate: it is a Zero fit, the arc ends
/// the side, and the loop validates with four vertices; at 2ε the fit
/// escalates.
#[test]
fn a_run_out_below_the_band_is_a_zero_fit() {
    let t = Tol::witness();
    let side = |d: f64| {
        Open.at(Point2::new(0.0, 0.0))
            .angle(0.0, t)
            .and_then(|x| x.line(2.0, t))
            .and_then(|x| x.fillet(0.5, t))
            .and_then(|x| x.toward(0.0, 1.0, t))
            .and_then(|x| x.to(Point2::new(2.5, 0.5 + d), t))
            .and_then(|x| x.line_to(Point2::new(0.0, 2.0), t))
            .and_then(|x| x.line_to(Start, t))
    };
    let c = side(0.1 * eps()).expect("builds");
    assert_eq!(c.structure.fillets[0].fit_out, geom_core::Sign::Zero);
    assert_eq!(c.loop_.as_loop().vertices().len(), 4, "no run out");
    validated(&c, "a run out a tenth of ε long");
    assert_eq!(
        escalated_on(side(2.0 * eps()), "a run out 2ε long"),
        "fillet_leg_fit"
    );
}
