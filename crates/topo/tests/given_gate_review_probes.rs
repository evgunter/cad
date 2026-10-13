//! Review probes for PR 5002 (`flush/given-gate`): routes by which a
//! caller outside `topo` puts a pair the kernel did not decide for
//! these operands into `BooleanInputs::given`. Prints, does not pin.
#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::print_stdout
)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::{AtRestBody, Body, BooleanInputs, BooleanOp, BooleanResult, FaceKey, Surface};

fn tol() -> Tol {
    Tol::witness()
}

fn face_at_z(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == z
            )
        })
        .map(|(k, _)| k)
        .expect("a face on z")
}

fn run(
    op: BooleanOp,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    inputs: &BooleanInputs,
) -> Result<BooleanResult<f64>, topo::BooleanError> {
    topo::boolean_op_with(op, a, b, inputs, topo::SweepStrategy::Realized, tol())
}

fn report(what: &str, r: &Result<BooleanResult<f64>, topo::BooleanError>) {
    match r {
        Ok(BooleanResult::Body(out)) => {
            let v = topo::mass_properties(&out.body, tol()).map(|m| m.volume);
            println!(
                "PROBE {what}: BUILT volume={v:?} faces={} shells={} rows={} verdicts={}",
                out.body.faces().count(),
                out.body.shells().count(),
                out.coincidences.len(),
                out.verdicts.len()
            );
        }
        Ok(BooleanResult::Empty) => println!("PROBE {what}: EMPTY"),
        Err(e) => println!("PROBE {what}: REFUSED {e:?}"),
    }
}

#[test]
fn routes_into_given() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let b_rest = finished(
        "b",
        brick((0.25, 0.75), (0.25, 0.75), (1.0, 2.0), tol()),
        tol(),
    );
    let b_gap = finished(
        "bg",
        brick((0.25, 0.75), (0.25, 0.75), (1.5, 2.5), tol()),
        tol(),
    );
    // A side-by-side continuation (coplanar tops, overlapping): a SameOriented verdict.
    let b_side = finished(
        "bs",
        brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );

    let decided = match run(BooleanOp::Union, &a, &b_rest, &BooleanInputs::default()).unwrap() {
        BooleanResult::Body(out) => out,
        BooleanResult::Empty => panic!(),
    };
    let [rest] = decided.verdicts[..] else {
        panic!("{:?}", decided.verdicts)
    };
    println!("PROBE rest verdict: {rest:?}");
    println!(
        "PROBE decided A∪B_rest: volume={:?} faces={}",
        topo::mass_properties(&decided.body, tol()).map(|m| m.volume),
        decided.body.faces().count()
    );

    // Route A: same verdict, the caller picks a different A face (A's bottom).
    let a_bot = face_at_z(&a, 0.0);
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        let r = run(
            op,
            &a,
            &b_rest,
            &BooleanInputs {
                given: Some(vec![rest.given_at(a_bot)]),
                ..BooleanInputs::default()
            },
        );
        report(&format!("A: {op:?} given_at(A bottom) on A,B_rest"), &r);
    }

    // Route B: reuse the verdict on a different B (0.5 above A).
    println!(
        "PROBE B: verdict.b == B_gap bottom key? {}",
        rest.b() == face_at_z(&b_gap, 1.5)
    );
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        let r = run(
            op,
            &a,
            &b_gap,
            &BooleanInputs {
                given: Some(vec![rest.given_at(rest.a())]),
                ..BooleanInputs::default()
            },
        );
        report(&format!("B: {op:?} reused verdict on A,B_gap"), &r);
    }
    report(
        "B: ref Union decided A,B_gap",
        &run(BooleanOp::Union, &a, &b_gap, &BooleanInputs::default()),
    );

    // Route C: a Continuation verdict (from a side-by-side union) handed to the opposed touch.
    let side = match run(BooleanOp::Union, &a, &b_side, &BooleanInputs::default()) {
        Ok(BooleanResult::Body(out)) => out,
        other => {
            report("C: side union", &other);
            return;
        }
    };
    println!("PROBE C: side verdicts: {:?}", side.verdicts);
    if let Some(cont) = side
        .verdicts
        .iter()
        .find(|v| v.relation() == topo::Relation::SameOriented && v.a() == face_at_z(&a, 1.0))
    {
        let b_bot = face_at_z(&b_rest, 1.0);
        println!(
            "PROBE C: cont.b == B_rest bottom key? {}",
            cont.b() == b_bot
        );
        for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
            let r = run(
                op,
                &a,
                &b_rest,
                &BooleanInputs {
                    given: Some(vec![cont.given_at(cont.a())]),
                    ..BooleanInputs::default()
                },
            );
            report(
                &format!("C: {op:?} continuation verdict on opposed A,B_rest"),
                &r,
            );
        }
    }

    // Route D: Some(vec![]) on a real touch.
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        let r = run(
            op,
            &a,
            &b_rest,
            &BooleanInputs {
                given: Some(vec![]),
                ..BooleanInputs::default()
            },
        );
        report(&format!("D: {op:?} Some(vec![]) on A,B_rest"), &r);
    }
    // Route D': Some(vec![]) on disjoint bodies.
    report(
        "D': Union Some(vec![]) on A,B_gap",
        &run(
            BooleanOp::Union,
            &a,
            &b_gap,
            &BooleanInputs {
                given: Some(vec![]),
                ..BooleanInputs::default()
            },
        ),
    );
}

/// Every kernel-minted verdict, re-keyed onto every A face, handed to
/// every op over several B placements: does any build a body that
/// differs from the decided run?
#[test]
fn reused_verdict_sweep() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let bs = [
        ("rest", brick((0.25, 0.75), (0.25, 0.75), (1.0, 2.0), tol())),
        ("gap", brick((0.25, 0.75), (0.25, 0.75), (1.5, 2.5), tol())),
        (
            "pierce",
            brick((0.25, 0.75), (0.25, 0.75), (0.5, 1.5), tol()),
        ),
        ("side", brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol())),
        ("corner", brick((0.5, 1.5), (0.5, 1.5), (0.5, 1.5), tol())),
        (
            "flush-x",
            brick((1.0, 2.0), (0.25, 0.75), (0.25, 0.75), tol()),
        ),
    ];
    let bs: Vec<_> = bs
        .into_iter()
        .map(|(n, b)| (n, finished(n, b, tol())))
        .collect();
    let mut verdicts = Vec::new();
    for (n, b) in &bs {
        if let Ok(BooleanResult::Body(out)) =
            run(BooleanOp::Union, &a, b, &BooleanInputs::default())
        {
            for v in &out.verdicts {
                verdicts.push((*n, *v));
            }
        }
    }
    println!("PROBE sweep: {} verdicts", verdicts.len());
    let afaces: Vec<FaceKey> = a.faces().map(|(k, _)| k).collect();
    let summary = |r: &Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(BooleanResult::Body(out)) => Some((
            (topo::mass_properties(&out.body, tol())
                .map(|m| m.volume)
                .unwrap_or(f64::NAN)
                * 1e9)
                .round() as i64,
            out.body.faces().count(),
            out.body.shells().count(),
        )),
        Ok(BooleanResult::Empty) => Some((0, 0, 0)),
        Err(_) => None,
    };
    let (mut built, mut refused, mut wrong) = (0, 0, 0);
    for (bn, b) in &bs {
        for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
            let truth = summary(&run(op, &a, b, &BooleanInputs::default()));
            for (vn, v) in &verdicts {
                for &f in &afaces {
                    let r = run(
                        op,
                        &a,
                        b,
                        &BooleanInputs {
                            given: Some(vec![v.given_at(f)]),
                            ..BooleanInputs::default()
                        },
                    );
                    match summary(&r) {
                        None => refused += 1,
                        Some(s) => {
                            built += 1;
                            if Some(s) != truth {
                                wrong += 1;
                                println!(
                                    "PROBE WRONG {op:?} B={bn} verdict-from={vn} {v:?} at A face {f:?}: got {s:?} truth {truth:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    println!("PROBE sweep: built={built} refused={refused} wrong={wrong}");
}

/// The rest verdict reused on a B resting a small definite gap above A.
#[test]
fn reused_verdict_small_gap() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let rest = finished(
        "b",
        brick((0.25, 0.75), (0.25, 0.75), (1.0, 2.0), tol()),
        tol(),
    );
    let Ok(BooleanResult::Body(decided)) =
        run(BooleanOp::Union, &a, &rest, &BooleanInputs::default())
    else {
        panic!()
    };
    let [v] = decided.verdicts[..] else { panic!() };
    for g in [5e-9, 2e-8, 1e-7, 1e-6, 1e-4, 1e-2, -1e-6, -1e-2] {
        let b = finished(
            "bg",
            brick((0.25, 0.75), (0.25, 0.75), (1.0 + g, 2.0 + g), tol()),
            tol(),
        );
        assert_eq!(v.b(), face_at_z(&b, 1.0 + g));
        for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
            report(
                &format!("gap {g:e} {op:?} decided"),
                &run(op, &a, &b, &BooleanInputs::default()),
            );
            report(
                &format!("gap {g:e} {op:?} given"),
                &run(
                    op,
                    &a,
                    &b,
                    &BooleanInputs {
                        given: Some(vec![v.given_at(v.a())]),
                        ..BooleanInputs::default()
                    },
                ),
            );
        }
    }
}

/// A Continuation verdict re-keyed onto the opposed rest touch: the
/// main-side "wrong relation" measurement, through kernel-minted types.
#[test]
fn continuation_verdict_on_an_opposed_touch() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let side = finished(
        "bs",
        brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rest = finished(
        "b",
        brick((0.25, 0.75), (0.25, 0.75), (1.0, 2.0), tol()),
        tol(),
    );
    let Ok(BooleanResult::Body(s)) = run(BooleanOp::Union, &a, &side, &BooleanInputs::default())
    else {
        panic!()
    };
    let (a_top, b_bot) = (face_at_z(&a, 1.0), face_at_z(&rest, 1.0));
    let cont = s
        .verdicts
        .iter()
        .find(|v| v.b() == b_bot)
        .expect("a continuation verdict whose B key is B_rest's bottom");
    println!("PROBE cont {cont:?} re-keyed to A top {a_top:?}");
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        report(
            &format!("cont-on-rest {op:?} decided"),
            &run(op, &a, &rest, &BooleanInputs::default()),
        );
        report(
            &format!("cont-on-rest {op:?} given"),
            &run(
                op,
                &a,
                &rest,
                &BooleanInputs {
                    given: Some(vec![cont.given_at(a_top)]),
                    ..BooleanInputs::default()
                },
            ),
        );
    }
}

/// Review's proposed pin: the only public fn in `topo`'s source that
/// returns a `GivenPair` or a `GlueVerdict` (or `Self` inside their
/// impls) is `GlueVerdict::given_at`.
#[test]
fn the_one_public_mint_of_a_given_pair_is_given_at() {
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push(p);
            }
        }
    }
    let mut files = Vec::new();
    walk(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut files,
    );
    let gated = |s: &str| s.contains("GivenPair") || s.contains("GlueVerdict");
    let mut mints = Vec::new();
    for f in files {
        let src = std::fs::read_to_string(&f).unwrap();
        let mut in_gated_impl = false;
        for line in src.lines() {
            let t = line.trim_start();
            if t.starts_with("impl") {
                in_gated_impl = gated(t);
            }
            // A trait impl's `fn` (From, Default, ...) is public too.
            let trait_fn = in_gated_impl && t.starts_with("fn ");
            let Some(sig) = t.strip_prefix("pub fn ").or(trait_fn.then(|| &t[3..])) else {
                continue;
            };
            let ret = sig.split("->").nth(1).unwrap_or("");
            if gated(ret) || (in_gated_impl && ret.contains("Self")) {
                mints.push(sig.split('(').next().unwrap().to_owned());
            }
        }
    }
    assert_eq!(
        mints,
        ["given_at"],
        "a public mint of a given pair besides given_at"
    );
    // A derive that mints: Default or Deserialize on either type.
    for f in ["boolean/mod.rs"] {
        let src = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join(f),
        )
        .unwrap();
        let lines: Vec<&str> = src.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            if l.starts_with("pub struct GivenPair") || l.starts_with("pub struct GlueVerdict") {
                let derive = lines[..i]
                    .iter()
                    .rev()
                    .find(|d| d.starts_with("#[derive"))
                    .unwrap();
                assert!(
                    !derive.contains("Default") && !derive.contains("Deserialize"),
                    "{l}: {derive}"
                );
            }
        }
    }
}
