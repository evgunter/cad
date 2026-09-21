//! R2's own e2e body for SYM-11: a TRIANGLE (not the unit's stadium or
//! washer) on a sketch plane at `(d, d, d)` with `d = 3.7e7` — a
//! magnitude between the two the tables name — extruded, and driven at
//! `Sym<f64>` and `Sym<Interval>`. Run at whatever `CAD_TOLERANCE_EPS`
//! the process carries, including ones no table row names.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::OrthoFrame;
use geom_core::sym::with_session_rules;
use geom_core::{
    Decide, ParamSymbol, Point2, Point3, Real, Sym, SymBudget, SymCounts, SymRules, Tol,
};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::Extrusion;

fn budget() -> SymBudget {
    SymBudget { max_terms: 4096, max_degree: 128 }
}

fn triangle_extrude<T: Decide>(d: T) -> Result<usize, String> {
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

fn drive<T>(label: &str, d: f64, lift: fn(f64) -> Sym<T>)
where
    T: Real + Decide + Send + 'static,
    Sym<T>: geom_brep::PcurveFittedLane,
{
    let eps = Tol::witness().eps();
    let out = std::thread::spawn(move || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            with_session_rules(budget(), SymRules::shipped(), || triangle_extrude(lift(d)))
        }))
    })
    .join()
    .expect("the probe thread joins");
    match out {
        Ok((r, c)) => {
            let _: SymCounts = c;
            println!(
                "  [{label}] eps={eps:e} d={d:e}: {r:?} | theorems_disputed {} numeric {} \
                 symbolic_zero {} registered {}",
                c.theorems_disputed, c.numeric, c.symbolic_zero, c.registered
            );
        }
        Err(_) => println!("  [{label}] eps={eps:e} d={d:e}: PANICKED (assertion fired)"),
    }
}

#[test]
fn r2_e2e_triangle_at_3e7() {
    let d = 3.7e7;
    println!("  [bare f64] {:?}", triangle_extrude::<f64>(d));
    drive::<f64>("Sym<f64>", d, |d| Sym::param(ParamSymbol::of("d"), d));
    #[cfg(feature = "interval")]
    drive::<geom_core::Interval>("Sym<Interval>", d, |d| {
        let e = Tol::witness().eps();
        Sym::param(
            ParamSymbol::of("d"),
            geom_core::Interval::from_bounds(d - e / 64.0, d + e / 64.0),
        )
    });
}
