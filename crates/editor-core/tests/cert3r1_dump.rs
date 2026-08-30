//! CERT-3 review lane R1 — the fence differential, reproduced.
//!
//! Replicates the m10-p fence's exact walk (`corpus_digest`), but
//! PRINTS every observable instead of digesting it: structure lines
//! (`RSTRUCT`) and coordinate lines (`RCOORD`, bits in hex). Run on
//! this tree and on a tree with only `affine.rs`/`mat.rs` reverted,
//! grep the prefixes, diff. Local-only; never pushed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{CancelToken, EvalOptions, NodeResult, ValuePayload, evaluate};
use geom_core::Tol;

fn walk<T, F, S>(lane: &str, coord: F, scalar: S)
where
    T: editor_core::EvalScalar,
    F: Fn(&str, &geom_core::Point3<T>),
    S: Fn(&str, T),
{
    fixture_walk::<T>(lane, &scalar);
    for doc in corpus::documents() {
        let ev = evaluate::<T>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        for (id, result) in ev.nodes.iter() {
            match result {
                NodeResult::Poisoned { through } => {
                    println!("RSTRUCT {lane} {} {} poisoned {}", doc.name, id.0, through.0);
                }
                NodeResult::Failed(_) => println!("RSTRUCT {lane} {} {} failed", doc.name, id.0),
                NodeResult::Ok(v) => {
                    println!(
                        "RSTRUCT {lane} {} {} ok {}",
                        doc.name,
                        id.0,
                        v.payload.kind_name()
                    );
                    if let ValuePayload::Body(b) = &v.payload {
                        for (i, (_, p)) in b.points().enumerate() {
                            coord(&format!("{lane} {} {} {i}", doc.name, id.0), p);
                        }
                    }
                }
            }
        }
    }
}

/// The fence's arc-carrier fixture, replicated verbatim from
/// `m10_p_fence.rs` with prints in place of digest feeds.
fn fixture_walk<T: profile::ArcCarrierScalar>(lane: &str, scalar: &impl Fn(&str, T)) {
    use geom_core::Point2;
    use profile::{ArcData, ArcSweep, Center, Open, Start, Step, Target};
    let p2 = |x: f64, y: f64| Point2::new(x, y);
    let tip = 0.75_f64.sqrt();
    let programs = [
        Open.arc_fillet_arc(
            Center {
                c: p2(-0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -tip),
            },
            0.35,
            Center {
                c: p2(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        ),
        Open.arc_fillet_arc(
            Center {
                c: p2(-1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -3.0_f64.sqrt()),
            },
            0.5,
            Center {
                c: p2(1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        ),
    ];
    let embed = |step: &Step<f64>| -> Step<T> {
        let pt = |p: Point2<f64>| Point2::new(T::from_f64(p.x), T::from_f64(p.y));
        let tgt = |t: Target<f64>| match t {
            Target::Start => Target::Start,
            Target::Point(p) => Target::Point(pt(p)),
        };
        let spec = |a: ArcData<f64>| match a {
            ArcData::Center { c, winding, target } => ArcData::Center {
                c: pt(c),
                winding,
                target: tgt(target),
            },
            _ => unreachable!("this fixture authors Center-mode arcs only"),
        };
        match *step {
            Step::ArcFilletArc {
                spec: a,
                radius,
                spec2,
            } => Step::ArcFilletArc {
                spec: spec(a),
                radius: T::from_f64(radius),
                spec2: spec(spec2),
            },
            _ => unreachable!("this fixture is one fused step"),
        }
    };
    for (i, built) in programs.into_iter().enumerate() {
        let closed = built.expect("the arc-carrier fixture constructs at f64");
        let steps: Vec<Step<T>> = closed.program.iter().map(embed).collect();
        match profile::replay(&steps, Tol::witness()) {
            Ok(lp) => {
                println!("RSTRUCT {lane} fixture{i} ok {}", lp.vertices().len());
                for (j, v) in lp.vertices().iter().enumerate() {
                    scalar(&format!("{lane} fixture{i} v{j} x"), v.pos().x);
                    scalar(&format!("{lane} fixture{i} v{j} y"), v.pos().y);
                    scalar(&format!("{lane} fixture{i} v{j} b"), v.bulge());
                }
            }
            Err(_) => println!("RSTRUCT {lane} fixture{i} refused"),
        }
    }
}

#[test]
fn r1_dump_f64() {
    walk::<f64, _, _>(
        "f64",
        |tag, p| {
            for (a, c) in [("x", p.x), ("y", p.y), ("z", p.z)] {
                println!("RCOORD {tag} {a} {:016x}", c.to_bits());
            }
        },
        |tag, v: f64| println!("RCOORD {tag} {:016x}", v.to_bits()),
    );
}

#[cfg(feature = "interval")]
#[test]
fn r1_dump_interval() {
    use geom_core::{Bounds, Interval};
    walk::<Interval, _, _>(
        "iv",
        |tag, p| {
            for (a, c) in [("x", p.x), ("y", p.y), ("z", p.z)] {
                println!(
                    "RCOORD {tag} {a} {:016x} {:016x}",
                    c.lo().to_bits(),
                    c.hi().to_bits()
                );
            }
        },
        |tag, v: Interval| {
            println!(
                "RCOORD {tag} {:016x} {:016x}",
                v.lo().to_bits(),
                v.hi().to_bits()
            );
        },
    );
}
