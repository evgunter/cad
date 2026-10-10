//! Review probe (PR 4518): the newly built far washers are correct bodies.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::OrthoFrame;
use geom_core::sym::with_session_rules;
use geom_core::{Bounds, Decide, Interval, ParamSymbol, Point2, Point3, Real, Sym, SymBudget, SymRules, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis};

fn washer<T: Decide + topo::AtRestPolicy>(d: T, r: T) -> Result<topo::Body<T>, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(d, d, d)));
    let lp = ProfileLoop::polygon([
        Point2::new(r, lit(0.0)), Point2::new(lit(2.0), lit(0.0)),
        Point2::new(lit(2.0), lit(1.0)), Point2::new(r, lit(1.0)),
    ]);
    let vp = Profile::new(plane, vec![lp]).validate(Tol::witness()).map_err(|e| format!("{e:?}"))?;
    sweep::revolve(&vp, RevolveAxis { origin: Point2::new(lit(0.0), lit(0.0)), dir: Vec2::new(lit(0.0), lit(1.0)) }, Revolution::Full, Tol::witness())
        .map(|r| r.body).map_err(|e| format!("{e:?}"))
}

fn report<T: Real>(lane: &str, d: f64, b: &Result<topo::Body<T>, String>, mid: impl Fn(T) -> f64) {
    match b {
        Err(e) => println!("REVIEW {lane} d={d:e}: refused {}", &e[..e.len().min(120)]),
        Ok(body) => {
            let closed = topo::validate::validate_closed(body).is_ok();
            let mut worst: f64 = 0.0;
            let mut n = 0;
            for (k, _) in body.vertices() {
                let p = topo::readback::vertex_point(body, k).unwrap();
                let (x, y, z) = (mid(p.x) - d, mid(p.y) - d, mid(p.z) - d);
                let rad = (x * x + z * z).sqrt();
                let er = (rad - 1.0).abs().min((rad - 2.0).abs());
                let ey = y.abs().min((y - 1.0).abs());
                worst = worst.max(er).max(ey);
                n += 1;
            }
            println!("REVIEW {lane} d={d:e}: built, faces {}, vertices {n}, closed {closed}, worst vertex off its analytic circle {worst:e} (ulp(d) {:e})",
                body.faces().count(), d.next_up() - d);
        }
    }
}

#[test]
fn review_4518_far_washers() {
    let eps = Tol::witness().eps();
    for d in [0.0, 1.0e6, 3.7e7, 1.0e9] {
        report("f64", d, &washer::<f64>(d, 1.0), |v| v);
        let (b, c) = with_session_rules(SymBudget { max_terms: 4096, max_degree: 128 }, SymRules::shipped(), || {
            washer(Sym::param(ParamSymbol::new(test_utils::symbol_id("d")), d), Sym::param(ParamSymbol::new(test_utils::symbol_id("r")), 1.0))
        });
        println!("REVIEW Sym<f64> d={d:e} washer alone: disputes {}", c.theorems_disputed);
        report("Sym<f64>", d, &b, |v: Sym<f64>| v.value);
        let (b, _) = with_session_rules(SymBudget { max_terms: 4096, max_degree: 128 }, SymRules::shipped(), || {
            washer(
                Sym::param(ParamSymbol::new(test_utils::symbol_id("d")), Interval::from_bounds(d - eps / 64.0, d + eps / 64.0)),
                Sym::param(ParamSymbol::new(test_utils::symbol_id("r")), Interval::from_bounds(1.0 - eps / 64.0, 1.0 + eps / 64.0)),
            )
        });
        report("Sym<Interval>", d, &b, |v: Sym<Interval>| 0.5 * (v.value.lo() + v.value.hi()));
    }
}

fn triangle<T: Decide + topo::AtRestPolicy>(d: T) -> Result<usize, String> {
    let lit = |v: f64| T::from_f64(v);
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(d, d, d)));
    let lp = ProfileLoop::polygon([Point2::new(lit(0.0), lit(0.0)), Point2::new(lit(1.0), lit(0.0)), Point2::new(lit(0.3), lit(0.9))]);
    let vp = Profile::new(plane, vec![lp]).validate(Tol::witness()).map_err(|e| format!("{e:?}"))?;
    sweep::extrude(&vp, sweep::Extrusion::Distance { depth: lit(1.0), side: sweep::ExtrudeSide::Along }, Tol::witness())
        .map(|e| e.body.faces().count()).map_err(|e| format!("{e:?}"))
}

#[test]
fn review_4518_triangle_alone() {
    for d in [1.0e6, 1.0e9] {
        let (b, c) = with_session_rules(SymBudget { max_terms: 4096, max_degree: 128 }, SymRules::shipped(), || {
            triangle(Sym::param(ParamSymbol::new(test_utils::symbol_id("d")), d))
        });
        println!("REVIEW triangle alone d={d:e}: disputes {} built {}", c.theorems_disputed, b.is_ok());
    }
}
