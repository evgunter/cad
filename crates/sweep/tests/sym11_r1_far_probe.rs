//! SYM-11 review probe (r1): bodies the unit did not measure, at
//! placements the table does not name, each body in its OWN session so
//! the dispute count is per body.
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

/// A quarter-round bracket: a right triangle whose hypotenuse is a
/// bulged arc, extruded by 2.
fn bracket_extrude<T: Decide>(d: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(d, d, d)));
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(lit(0.0), lit(0.0)), lit(0.0)),
        ProfileVertex::new(Point2::new(lit(2.0), lit(0.0)), lit(0.0)),
        ProfileVertex::new(Point2::new(lit(2.0), lit(1.5)), lit(0.3)),
        ProfileVertex::new(Point2::new(lit(0.0), lit(1.5)), lit(0.0)),
    ]);
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .map_err(|e| format!("validate: {e:?}"))?;
    sweep::extrude(&vp, Extrusion::Distance(lit(2.0)), Tol::witness())
        .map(|e| e.body.faces().count())
        .map_err(|e| format!("extrude: {e:?}"))
}

/// A rectangle revolved a quarter turn about the sketch y axis.
fn wedge_revolve<T: Decide + geom_brep::PcurveFittedLane>(d: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(d, d, d)));
    let lp = ProfileLoop::polygon([
        Point2::new(lit(1.0), lit(0.0)),
        Point2::new(lit(3.0), lit(0.0)),
        Point2::new(lit(3.0), lit(0.7)),
        Point2::new(lit(1.0), lit(0.7)),
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
        Revolution::Partial(lit(core::f64::consts::FRAC_PI_2)),
        Tol::witness(),
    )
    .map(|r| r.body.faces().count())
    .map_err(|e| format!("revolve: {e:?}"))
}

fn one_session<T: Real + Decide>(
    d: Sym<T>,
    body: impl FnOnce(Sym<T>) -> Result<usize, String>,
) -> (Result<usize, String>, SymCounts) {
    with_session_rules(budget(), SymRules::shipped(), || body(d))
}

const PLACEMENTS: [f64; 3] = [3.0e7, 2.0e8, 1.0e9];

#[test]
fn r1_own_bodies_at_sym_f64_per_body_sessions() {
    for d0 in PLACEMENTS {
        let mk = || Sym::<f64>::param(ParamSymbol::of("d"), d0);
        let (e, ce) = one_session(mk(), |d| bracket_extrude(d));
        let (r, cr) = one_session(mk(), |d| wedge_revolve(d));
        println!(
            "   [Sym<f64>] eps={:e} d={d0:e}\n     bracket extrude {e:?}\n       {ce:?}\n     wedge revolve {r:?}\n       {cr:?}",
            Tol::witness().eps()
        );
    }
}

#[cfg(feature = "interval")]
#[test]
fn r1_own_bodies_at_sym_interval_per_body_sessions() {
    use geom_core::Interval;
    let eps = Tol::witness().eps();
    for d0 in PLACEMENTS {
        let mk = || {
            Sym::<Interval>::param(
                ParamSymbol::of("d"),
                Interval::from_bounds(d0 - eps / 64.0, d0 + eps / 64.0),
            )
        };
        let (e, ce) = one_session(mk(), |d| bracket_extrude(d));
        let (r, cr) = one_session(mk(), |d| wedge_revolve(d));
        println!(
            "   [Sym<Interval>] eps={eps:e} d={d0:e}\n     bracket extrude {e:?}\n       {ce:?}\n     wedge revolve {r:?}\n       {cr:?}"
        );
        assert_eq!(ce.theorems_disputed, 0, "{ce:?}");
        assert_eq!(cr.theorems_disputed, 0, "{cr:?}");
    }
}
