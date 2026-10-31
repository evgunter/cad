//! MEASUREMENT SCRATCH (carve-embed phase 1; delete before landing).
//!
//! The tour's lofts and sweeps asked `topo::clearance`'s injectivity
//! question (the embedding certificate), at `f64` and, where the scene
//! is generic, on the body built at `Interval`. Asserts nothing; run with
//! `cargo test --release --test all carve_embed_scratch -- --ignored --nocapture`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Instant;

use demo_tour::{klein, lily, projectbox, skinned};
use pncad::geom_core::k_stats::decide;
use pncad::geom_core::{Band, Bounds, CertifiedBounds, Decide, Margin, Sign, Tol};
use topo::clearance::{
    BoundReport, BoundVerdict, Budgets, Candidate, ClearanceBound, DEFAULT_MAX_CELL_DEPTH,
    EngineRefusal, Side, Sweep, Which,
};
use topo::{Body, FaceKey};

fn budget() -> usize {
    std::env::var("CARVE_EMBED_BUDGET")
        .ok()
        .and_then(|b| b.parse().ok())
        .unwrap_or(65_536)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn verify<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    body: &Body<T>,
) -> impl FnMut(Candidate<'_, '_, T>) -> Result<(FaceKey, FaceKey, f64), String> + '_ {
    move |candidate| {
        let (fa, fb) = candidate.faces();
        let surf = |f: FaceKey| {
            body.get_surface(body.get_face(f).unwrap().surface)
                .unwrap()
                .clone()
        };
        let (sa, sb) = (surf(fa), surf(fb));
        let (la, lb) = candidate.on_face_stations();
        let at = T::from_f64;
        let mut best: Option<T> = None;
        for &(au, av) in &la {
            let pa = sa.eval(at(au), at(av));
            for &(bu, bv) in &lb {
                let d = (pa - sb.eval(at(bu), at(bv))).norm();
                if best.is_none_or(|b| d.hi() < b.hi()) {
                    best = Some(d);
                }
            }
        }
        let d = best.ok_or("no station")?;
        let bound = ClearanceBound::StrictlyPositive;
        match decide(bound.predicate(), Margin::of(d - at(bound.c())), band()) {
            Ok(Sign::Negative) => Ok((fa, fb, d.hi())),
            Ok(Sign::Zero) if !bound.zero_discharges() => Ok((fa, fb, d.hi())),
            _ => Err(format!("closest station pair {} apart", d.hi())),
        }
    }
}

fn certify<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(body: &Body<T>) -> String {
    let faces = body.faces().map(|(k, _)| k).collect();
    let side = Side { body, faces };
    let windows = match side.windows(Which::First) {
        Ok(w) => w,
        Err(e) => return format!("WINDOWS REFUSED {}: {}", e.name(), e.payload()),
    };
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: budget(),
        },
    };
    let t = Instant::now();
    let r: BoundReport<_> = sweep.run(&windows, &windows, true, verify(body));
    let c = t.elapsed().as_secs_f64();
    let v = match &r.verdict {
        BoundVerdict::Holds => "Holds".to_owned(),
        BoundVerdict::Violated((a, b, d)) => format!("Violated {a:?} x {b:?} d<={d:.2e}"),
        BoundVerdict::Refused(e @ EngineRefusal::Uncertified { .. }) => {
            format!("Refused uncertified {}", e.payload())
        }
        BoundVerdict::Refused(e) => format!("Refused {}: {}", e.name(), e.payload()),
    };
    format!(
        "{v} | cert {c:.3}s | faces {} | {:?}",
        body.faces().count(),
        r.receipt
    )
}

#[test]
#[ignore = "measurement scratch: prints, asserts nothing"]
fn carve_embed_scratch_tour() {
    let tol = Tol::witness();
    let t = Instant::now();
    let stops = skinned::stops(tol);
    println!(
        "[embed-tour] skinned::stops built in {:.3}s",
        t.elapsed().as_secs_f64()
    );
    for stop in &stops {
        for b in &stop.bodies {
            println!("[embed-tour] skinned/{} f64: {}", b.name, certify(&b.body));
        }
    }
    let t = Instant::now();
    let leaf_a = lily::leaf_a::<f64>(lily::leaf_a_plan(), tol);
    println!(
        "[embed-tour] lily leaf_a alone built in {:.3}s",
        t.elapsed().as_secs_f64()
    );
    drop(leaf_a);
    let t = Instant::now();
    let p64 = lily::plant::<f64>(tol);
    println!(
        "[embed-tour] lily::plant<f64> built in {:.3}s",
        t.elapsed().as_secs_f64()
    );
    let wanted = |n: &str| n.starts_with("lily_leaf") || n.starts_with("lily_sepal");
    for p in p64.iter().filter(|p| wanted(p.name)) {
        println!("[embed-tour] {} f64: {}", p.name, certify(&p.body));
    }
    let t = Instant::now();
    let b64 = klein::bottle::<f64>(tol);
    println!(
        "[embed-tour] klein bottle<f64> built in {:.3}s",
        t.elapsed().as_secs_f64()
    );
    println!("[embed-tour] klein loop f64: {}", certify(&*b64.top));
    let t = Instant::now();
    let spring =
        projectbox::spring::<f64>(&[projectbox::square_wire(tol)], projectbox::PER_TURN, tol)
            .unwrap();
    println!(
        "[embed-tour] projectbox spring (6 turns, 193 stations) built in {:.3}s",
        t.elapsed().as_secs_f64()
    );
    println!("[embed-tour] projectbox spring f64: {}", certify(&spring));
}

/// Every interior knot of a NURBS face that sits within 64 ulps of a
/// dyadic split point `m/2^d` (d <= 40) WITHOUT being on it: the
/// bisection lands there, and the cell on the far side restricts to a
/// span a few ulps wide. `axis(depth, ulps)` per such knot.
fn sliver_knots(knots: &[f64]) -> Vec<String> {
    let mut out = Vec::new();
    for &k in knots {
        if k <= 0.0 || k >= 1.0 {
            continue;
        }
        for d in 1..=40 {
            let scale = f64::from(1u32 << d.min(31)) * f64::from(1u32 << (d - d.min(31)));
            let dy = (k * scale).round() / scale;
            #[allow(clippy::cast_possible_wrap)]
            let ulps = k.to_bits() as i64 - dy.to_bits() as i64;
            if ulps == 0 {
                break;
            }
            if ulps.abs() <= 64 {
                out.push(format!("{k:.6}@2^-{d}({ulps:+})"));
                break;
            }
        }
    }
    out
}

/// Each candidate face pair of `body` asked alone, with each NURBS
/// face's sliver knots.
fn pairs(name: &str, body: &Body<f64>) {
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for &f in &faces {
        if let pncad::geom::Surface::Nurbs(net) =
            body.get_surface(body.get_face(f).unwrap().surface).unwrap()
        {
            println!(
                "[pair] {name} {f:?} sliver v knots {:?} u knots {:?} (v {} knots, u {})",
                sliver_knots(net.knot_values_v().values()),
                sliver_knots(net.knot_values_u().values()),
                net.knot_values_v().values().len(),
                net.knot_values_u().values().len(),
            );
        }
    }
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: budget(),
        },
    };
    for (i, &a) in faces.iter().enumerate() {
        for &b in &faces[i..] {
            let w = |f| {
                Side {
                    body,
                    faces: vec![f],
                }
                .windows(Which::First)
            };
            let (Ok(wa), Ok(wb)) = (w(a), w(b)) else {
                println!("[pair] {name} {a:?} x {b:?}: WINDOW REFUSED");
                continue;
            };
            let t = Instant::now();
            let r = sweep.run(&wa, &wb, true, verify(body));
            let v = match &r.verdict {
                BoundVerdict::Holds => "Holds".to_owned(),
                BoundVerdict::Violated(_) => "Violated".to_owned(),
                BoundVerdict::Refused(e) => format!("Refused {}: {}", e.name(), e.payload()),
            };
            println!(
                "[pair] {name} {a:?} x {b:?}: {v} | {:.3}s | deepest {} | {:?}",
                t.elapsed().as_secs_f64(),
                r.widths.deepest,
                r.receipt
            );
        }
    }
}

#[test]
#[ignore = "measurement scratch: prints, asserts nothing"]
fn carve_embed_scratch_tour_pairs() {
    let tol = Tol::witness();
    for stop in &skinned::stops(tol) {
        for b in &stop.bodies {
            if b.name == "s_duct" || b.name == "twisted_loft" {
                pairs(&b.name, &b.body);
            }
        }
    }
    for p in lily::plant::<f64>(tol).iter() {
        if p.name == "lily_leaf_a" || p.name == "lily_leaf_b" {
            pairs(p.name, &p.body);
        }
    }
    let b = klein::bottle::<f64>(tol);
    pairs("klein_loop", &b.top);
}
