#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use std::time::Instant;
use geom_core::{Band, Interval, ParamSymbol, Point2, Real, Sym, SymBudget, SymRules, Tol};
use sweep::test_support::prism;
use topo::clearance::{Budgets, DEFAULT_MAX_CELL_DEPTH, Side, embedded};

fn tol() -> Tol { Tol::witness() }

fn report<T: geom_core::Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(body: topo::Body<T>, label: &str) {
    let t0 = Instant::now();
    let fin = topo::finish(body.clone(), tol());
    let t3 = t0.elapsed();
    let faces = body.faces().map(|(k, _)| k).collect();
    let side = Side { body: &body, faces };
    let t0 = Instant::now();
    let r = embedded(&side, Band::linear(tol()).unwrap(), Budgets { max_cell_depth: DEFAULT_MAX_CELL_DEPTH, max_cell_pairs: 65_536 });
    println!("[scratch] {label}: tier3 {:?} in {t3:?}; cert {:?} in {:?}; {:?}", fin.as_ref().err(), r.verdict, t0.elapsed(), r.receipt);
    let c = topo::scratch_zero_counts();
    println!("[scratch] counts {:?}", c.iter().filter(|(k, _)| !k.starts_with("dist")).collect::<Vec<_>>());
}

#[test]
fn scratch_cone_family() {
    use sweep::{Revolution, RevolveAxis, revolve};
    use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
    use geom_core::Vec2;
    for w in [0.0, 1e-3, 1e-2] {
        let budget = SymBudget { max_terms: 4096, max_degree: 128 };
        let ((), d) = geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
            type S = Sym<Interval>;
            let h: S = Sym::param_over(ParamSymbol::new(test_utils::symbol_id("h")), Interval::from_bounds(1.0 - w, 1.0 + w), 1.0 - w, 1.0 + w);
            let lit = |v: f64| S::from_f64(v);
            let lp = ProfileLoop::polygon([Point2::new(lit(0.0), lit(0.0)), Point2::new(lit(1.0), lit(0.0)), Point2::new(lit(0.0), h)]);
            let vp = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()).expect("validates");
            let axis = RevolveAxis { origin: Point2::new(lit(0.0), lit(0.0)), dir: Vec2::new(lit(0.0), lit(1.0)) };
            let body = revolve(&vp, axis, Revolution::Full, tol()).expect("revolves").body.into_body();
            report(body, &format!("cone Sym<Interval> w={w}"));
        });
        println!("[scratch] sym session {d:?}");
    }
}

fn run<T: geom_core::Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(h: T, label: &str) {
    let v = |x: f64, y: f64| (Point2::new(x, y).map(T::from_f64), T::zero());
    let body = prism(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 1.0), v(0.0, 1.0)], h, tol());
    let t0 = Instant::now();
    let fin = topo::finish(body.clone(), tol());
    let t3 = t0.elapsed();
    let faces = body.faces().map(|(k, _)| k).collect();
    let side = Side { body: &body, faces };
    let t0 = Instant::now();
    let r = embedded(&side, Band::linear(tol()).unwrap(), Budgets { max_cell_depth: DEFAULT_MAX_CELL_DEPTH, max_cell_pairs: 65_536 });
    println!("[scratch] {label}: tier3 {} in {t3:?}; cert {:?} in {:?}; {:?}", if fin.is_ok() {"ok"} else {"refuses"}, r.verdict, t0.elapsed(), r.receipt);
    println!("[scratch] counts {:?}", topo::scratch_zero_counts());
}

#[test]
fn scratch_box_family() {
    for w in [1e-2, 3e-2, 0.1] {
        let _ = Interval::from_bounds;
        let budget = SymBudget { max_terms: 4096, max_degree: 128 };
        let ((), d) = geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
            let h: Sym<Interval> = Sym::param_over(ParamSymbol::new(test_utils::symbol_id("h")), Interval::from_bounds(1.0 - w, 1.0 + w), 1.0 - w, 1.0 + w);
            run(h, &format!("Sym<Interval> w={w}"));
        });
        println!("[scratch] sym session {d:?}");
    }
}
