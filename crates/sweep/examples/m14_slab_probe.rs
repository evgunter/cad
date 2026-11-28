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

type Boxed = Sym<Interval>;

fn boxed(name: u64, mid: f64, w: f64) -> Boxed {
    Sym::param_over(
        ParamSymbol::new(name),
        Interval::from_bounds(mid - w, mid + w),
        mid - w,
        mid + w,
    )
}

/// Population 1 (the slab) and 4 (wide slabs): census, then the whole battery.
fn m16_slabs(tol: Tol, band: Band) {
    use geom_core::sym::m16;
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let records = topo::ContactRecords::default();
    for (pop, w) in [
        ("pop1", 0.05),
        ("pop1", 0.1),
        ("pop1", 0.25),
        ("pop1", 0.45),
        ("pop4w", 0.44),
        ("pop4w", 0.488),
        ("pop4w", 0.49),
        ("pop4w", 0.6),
        ("pop4w", 0.9),
    ] {
        for pass in ["census", "finish"] {
            let what = format!("{pop}|slab|{w}|{pass}");
            m16::context(&what);
            let (out, counts) = geom_core::sym::with_session(budget, || {
                let body = match slab::<Boxed>(boxed(0x4d31_3600, 1.0, w), tol) {
                    Ok(body) => body,
                    Err(e) => return Err(e),
                };
                Ok(if pass == "census" {
                    topo::census_traces(
                        &body,
                        &records,
                        band,
                        tol,
                        None,
                        topo::CensusStrategy::Realized,
                    )
                    .0
                } else {
                    topo::validate_pseudomanifold_structural(&body, &records, tol)
                        .err()
                        .unwrap_or_default()
                })
            });
            match out {
                Ok(out) => {
                    let kinds: Vec<String> = out
                        .iter()
                        .map(|e| format!("{e:?}").chars().take(40).collect())
                        .collect();
                    m16::note(&format!(
                        "{what}\tfindings={}\t{counts:?}\t{kinds:?}",
                        out.len()
                    ));
                }
                Err(e) => m16::note(&format!("{what}\tdoor refuses\t{}", &e[..e.len().min(300)])),
            }
        }
    }
}

/// Population 2: a brick cut by a tilted slab through the Boolean door,
/// the cutter's height over a box.
fn m16_verbs(tol: Tol, band: Band) {
    use geom_core::Real as _;
    use geom_core::sym::m16;
    use topo::FinishedBody as _;
    let c = Boxed::from_f64;
    let records = topo::ContactRecords::default();
    for w in [0.0, 0.05, 0.1, 0.25, 0.45] {
        for budget in [
            SymBudget {
                max_terms: 64,
                max_degree: 8,
            },
            SymBudget {
                max_terms: 4096,
                max_degree: 128,
            },
        ] {
            let what = format!("pop2|cut|{w}|b{}", budget.max_terms);
            m16::context(&format!("{what}|build"));
            let (out, counts) =
                geom_core::sym::with_session(budget, || -> Result<String, String> {
                    let stock = topo::test_support::finished(
                        "the stock",
                        sweep::test_support::brick::<Boxed>(
                            (0.0, 1.0),
                            (0.0, 1.0),
                            (0.0, 3.0),
                            tol,
                        ),
                        tol,
                    );
                    let cutter = topo::test_support::finished(
                        "the cutter",
                        sweep::test_support::brick::<Boxed>(
                            (-2.0, 3.0),
                            (-2.0, 3.0),
                            (0.0, 4.0),
                            tol,
                        ),
                        tol,
                    );
                    let axis = geom_core::UnitVec3::new(
                        geom_core::Vec3::new(c(0.6), c(0.8), c(0.0)),
                        "the tilt",
                        band,
                    )
                    .map_err(|e| format!("axis: {e:?}"))?;
                    let tilt = geom_core::RigidMotion::rotation(axis, c(0.25))
                        .map_err(|e| format!("rotation: {e:?}"))?;
                    let lift = geom_core::RigidMotion::translation(geom_core::Vec3::new(
                        c(0.0),
                        c(0.0),
                        boxed(0x4d31_3601, 1.5, w),
                    ))
                    .map_err(|e| format!("translation: {e:?}"))?;
                    let cutter = cutter
                        .moved(&tilt, tol)
                        .map_err(|e| format!("tilt refuses: {e}"))?
                        .moved(&lift, tol)
                        .map_err(|e| format!("lift refuses: {e}"))?;
                    let cut = topo::boolean_op_with(
                        topo::BooleanOp::Subtract,
                        &stock,
                        &cutter,
                        &topo::BooleanInputs::default(),
                        topo::SweepStrategy::Realized,
                        tol,
                    )
                    .map_err(|e| {
                        let t = format!("{e:?}");
                        format!("boolean refuses: {}", &t[..t.len().min(600)])
                    })?;
                    let Some(result) = cut.body() else {
                        return Err("boolean empty".into());
                    };
                    let body = result.body.clone();
                    m16::context(&format!("{what}|census"));
                    println!("M15 CASE {what}");
                    let census = topo::census_traces(
                        &body,
                        &records,
                        band,
                        tol,
                        None,
                        topo::CensusStrategy::Realized,
                    )
                    .0;
                    println!("M15 END {what}");
                    m16::context(&format!("{what}|finish"));
                    let finish = topo::validate_pseudomanifold_structural(&body, &records, tol)
                        .err()
                        .unwrap_or_default();
                    let kinds = |v: &[topo::ValidationError]| -> Vec<String> {
                        v.iter()
                            .map(|e| format!("{e:?}").chars().take(160).collect())
                            .collect()
                    };
                    Ok(format!(
                        "faces={} census={} {:?} finish={} {:?}",
                        body.faces().count(),
                        census.len(),
                        kinds(&census),
                        finish.len(),
                        kinds(&finish)
                    ))
                });
            m16::note(&format!("{what}\t{out:?}\t{counts:?}"));
            println!("M16 {what}: {out:?}");
        }
    }
}

fn main() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    match std::env::var("BOXFINISH_M16_RUN").as_deref() {
        Ok("slabs") => return m16_slabs(tol, band),
        Ok("verbs") => return m16_verbs(tol, band),
        _ => {}
    }
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
