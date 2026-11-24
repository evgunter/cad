//! Probe (M14): the extrude door's unit slab of depth `1 ± w` — each
//! aimed ray's reading, the reader's, and tier 3′'s census by itself.
//! Prints only. `BOXFINISH_M14_FIX=1` fixes each aimed ray.
//!
//! `cargo run -p sweep --features test-support --example m14_slab_probe`
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Decide, Interval, ParamSymbol, Point2, Sym, SymBudget, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, LeafBody};

fn slab<T: Decide + topo::AtRestPolicy>(h: T, tol: Tol) -> Result<Body<T>, String> {
    let c = T::from_f64;
    let verts = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
        .iter()
        .map(|&(x, y)| (Point2::new(c(x), c(y)), c(0.0)))
        .collect();
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![bulge_loop(verts)])
        .validate(tol)
        .map_err(|e| format!("{e:?}"))?;
    let extrusion = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, extrusion, tol)
        .map(|built| built.body.into_body())
        .map_err(|e| format!("{e:?}"))
}

fn rows<T: Decide + topo::AtRestPolicy + Bounds>(what: &str, body: &Body<T>, band: Band, tol: Tol) {
    for (f, _) in body.faces() {
        let label: String = body
            .face(f)
            .map(|v| format!("{:?}", v.surface()))
            .unwrap_or_default()
            .chars()
            .take(420)
            .collect();
        println!("M14 {what} key {f:?} = {label}");
    }
    for r in topo::test_support::each_aim(body, band) {
        let cut: String = r.chars().take(700).collect();
        println!("M14 {what} aim | {cut}");
    }
    for r in topo::test_support::aimed_rays(body, band) {
        let cut: String = r.chars().take(300).collect();
        println!("M14 {what} reader | {cut}");
    }
    let (errors, _) = topo::census_traces(
        body,
        &topo::ContactRecords::default(),
        band,
        tol,
        None,
        topo::CensusStrategy::Realized,
    );
    println!("M14 {what} census | {} findings", errors.len());
    for e in errors {
        let cut: String = format!("{e:?}").chars().take(500).collect();
        println!("M14 {what} census-finding | {cut}");
    }
}

fn main() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    for w in [0.05, 0.1, 0.25, 0.45] {
        for f in [-1.0, -0.5, 0.0, 0.5, 1.0] {
            let member = slab::<f64>(1.0 + f * w, tol).expect("a member builds");
            println!(
                "M14 slab w={w} member {} | {:?}",
                1.0 + f * w,
                topo::test_support::aimed_rays(&member, band)
            );
        }
        geom_core::sym::with_session(budget, || {
            let top = Sym::param_over(
                ParamSymbol::new(0x4d31_3400),
                Interval::from_bounds(1.0 - w, 1.0 + w),
                1.0 - w,
                1.0 + w,
            );
            match slab::<Sym<Interval>>(top, tol) {
                Ok(body) => rows(&format!("slab w={w} sym"), &body, band, tol),
                Err(e) => println!("M14 slab w={w} sym door refuses | {e}"),
            }
        });
        match slab::<Interval>(Interval::from_bounds(1.0 - w, 1.0 + w), tol) {
            Ok(body) => rows(&format!("slab w={w} iv"), &body, band, tol),
            Err(e) => println!("M14 slab w={w} iv door refuses | {e}"),
        }
    }
}
