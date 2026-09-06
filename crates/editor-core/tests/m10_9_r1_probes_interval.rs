//! **R1's independent probes of M10-9 at document scale** — the
//! registered-identity door driven on documents the unit did not
//! measure, and the walk of refusals past the plate's ceiling.
//!
//! EVERY ROW IS EVIDENCE-ONLY (`#[ignore]`d, prints; the D13 bit row is
//! the one exception and it asserts) — [[test-suite-cost]]. Run:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   m10_9_r1_probes_interval:: --ignored --nocapture
//! ```
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::time::Instant;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, NamePat, Node, ParamName, ProfileDoc, ProfileProgram,
    RecipeNodeId, Selector, SitedRef, SurfaceKindSet, UnitSym, select_where,
};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::fixture::Recorder;
use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{ceiling, certifies_whole, dials};

/// The two rule sets this unit is a differential between.
fn rows() -> [(&'static str, SymRules); 2] {
    [
        ("door OFF", SymRules::shipped_without_the_door()),
        ("door ON ", SymRules::shipped()),
    ]
}

/// Just the predicate name out of a refusal message.
fn pred(msg: &str) -> String {
    msg.split_once("predicate '")
        .and_then(|(_, r)| r.split_once('\'').map(|(p, rest)| {
            let encl = rest
                .split_once("enclosure ")
                .and_then(|(_, e)| e.split_once(')').map(|(v, _)| v.to_owned()))
                .unwrap_or_default();
            format!("{p} {encl}")
        }))
        .unwrap_or_else(|| msg.chars().take(90).collect())
}

/// **D13 — `Curve3::circle_at` is the old `eval` arm, bit for bit.**
/// The delegation is only sound for the value channel if the
/// association did not change; this asserts it at `f64` and at
/// `Interval`.
#[test]
fn r1_circle_at_and_eval_agree_bit_for_bit() {
    use geom_core::interval::Interval;
    use geom::Curve3;
    use geom_core::{Point3, Vec3};
    let cases = [
        (0.3_f64, 0.7, -1.1, 0.0, 0.0, 1.0, 2.75, 0.6, 0.8, 0.0, 1.234),
        (-3.5, 0.25, 7.0, 0.0, 1.0, 0.0, 0.125, 0.0, 0.0, 1.0, -0.77),
        (1e3, -1e-3, 5.0, 0.0, 0.0, -1.0, 1e-4, 1.0, 0.0, 0.0, 3.14159),
    ];
    for (cx, cy, cz, ax, ay, az, r, ux, uy, uz, t) in cases {
        let c = Point3::new(cx, cy, cz);
        let a = Vec3::new(ax, ay, az);
        let u = Vec3::new(ux, uy, uz);
        let door = Curve3::circle_at(c, a, r, u, t);
        let via = Curve3::Circle {
            center: c,
            axis: a,
            radius: r,
            u_ref: u,
        }
        .eval(t);
        assert_eq!(
            (door.x.to_bits(), door.y.to_bits(), door.z.to_bits()),
            (via.x.to_bits(), via.y.to_bits(), via.z.to_bits()),
            "f64 at t={t}"
        );
        let iv = |v: f64| Interval::from_bounds(v, v);
        let ci = Point3::new(iv(cx), iv(cy), iv(cz));
        let ai = Vec3::new(iv(ax), iv(ay), iv(az));
        let ui = Vec3::new(iv(ux), iv(uy), iv(uz));
        let door_i = Curve3::circle_at(ci, ai, iv(r), ui, iv(t));
        let via_i = Curve3::Circle {
            center: ci,
            axis: ai,
            radius: iv(r),
            u_ref: ui,
        }
        .eval(iv(t));
        assert_eq!(
            format!("{:?}{:?}{:?}", door_i.x, door_i.y, door_i.z),
            format!("{:?}{:?}{:?}", via_i.x, via_i.y, via_i.z),
            "Interval at t={t}"
        );
    }
}

/// **THE REFUSAL WALK PAST THE PLATE'S CEILING** (claim 3): at a ladder
/// of scales above the measured ceiling, what refuses FIRST with the
/// door open and shut, and every predicate that stayed indeterminate in
/// the same replay. The question this answers is whether the tail past
/// the door is more identity-shaped residuals or a real margin.
#[test]
#[ignore = "evidence-only: the plate's refusal walk above its ceiling"]
fn r1_the_refusal_walk_past_the_plates_ceiling() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for mult in [1.0_f64, 2.0, 4.0, 16.0, 64.0, 256.0, 1024.0] {
        let s = 7.787e2 * eps * mult;
        let doc = crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0;
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let box_ = ParamBox::of(&analyzed);
        for (label, r) in rows() {
            let (shapes, refusal, counts) = replay(&doc, &box_, r, tol);
            let mut blocked: BTreeMap<&'static str, usize> = BTreeMap::new();
            for sh in shapes.iter().filter(|sh| {
                matches!(
                    sh.outcome,
                    ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
                )
            }) {
                *blocked.entry(sh.predicate).or_default() += 1;
            }
            println!(
                "   x{mult:<7} {label}: first {:<70} blocked {blocked:?} reg={} num={}",
                refusal.as_deref().map(pred).unwrap_or_else(|| "-".into()),
                counts.registered,
                counts.numeric
            );
        }
    }
}

/// **The plate's ceiling, bisected here** (claim 3), door on and off, so
/// the PR's `[7.787e2, 7.817e2]·ε` is a number this review took rather
/// than one it read.
#[test]
#[ignore = "evidence-only: R1's own bisection of the plate ceiling"]
fn r1_the_plate_ceiling_bisected_both_ways() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let at = |s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0;
    for (label, r) in rows() {
        let (lo, hi, per) = ceiling(&at, r, tol, 1.0e2 * eps, 1.0e4 * eps, 12);
        println!(
            "   plate {label}: [{:.4e}, {:.4e}]·eps ({per:.2}s/probe)",
            lo / eps,
            hi / eps
        );
    }
}

// ---------------------------------------------------------------- e2e

/// **R1's OWN arc-bearing document**: a disc with a five-arc split bore
/// (`LoopProgram::CircleSplit`, `n = 5`) — five swept arc carriers per
/// wall instead of one seamless circle, which is the registrant's own
/// site five times over, and no other M10 fixture authors it.
///
/// Study: the bore radius Normal (σ), the bore's offset and the outer
/// radius Uniform (±), all scaled together, so a ceiling is a multiple
/// of a study a user would ask for.
pub(crate) fn split_bore_disc(scale: f64, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("finite length");
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite scalar");
    let plen = |n: &str| Expr::param(ParamName::new(n), Dimension::Length);
    let mut r = Recorder::new();
    let declare = |r: &mut Recorder, n: &str, value: f64, distribution: Distribution| {
        r.push(DocEdit::SetDocParam {
            name: ParamName::new(n),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(distribution),
            },
        });
    };
    declare(
        &mut r,
        "outer_r",
        6.0e-3,
        Distribution::Uniform {
            lo: -5.0e-5 * scale,
            hi: 5.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "offset",
        1.5e-3,
        Distribution::Uniform {
            lo: -5.0e-5 * scale,
            hi: 5.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "bore_r",
        2.0e-3,
        Distribution::Normal {
            sigma: 1.0e-5 * scale,
        },
    );
    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let thickness = Expr::div(plen("outer_r"), scl(4.0)).expect("Length / Scalar");
    let disc_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [len(0.0), len(0.0)],
            radius: plen("outer_r"),
        }],
    }));
    let disc = r.insert(Node::Extrude {
        profile: disc_profile,
        distance: thickness.clone(),
    });
    let bore_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::CircleSplit {
            centre: [plen("offset"), len(0.0)],
            radius: plen("bore_r"),
            n: 5,
            phase: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
        }],
    }));
    let bore = r.insert(Node::Extrude {
        profile: bore_profile,
        distance: thickness,
    });
    let refs = {
        let ev: editor_core::Evaluation<f64> = editor_core::evaluate(
            &r.doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            tol,
        );
        let env = r.doc.param_env::<f64>();
        let wall = |node: RecipeNodeId| {
            let mut faces = select_where(
                &ev,
                node,
                &Selector::of(NamePat::of_kind(EntityKind::Face)),
                &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                    geom_brep::SurfaceKind::Cylinder,
                ))],
                &env,
                tol,
            )
            .expect("the surface-kind atom is exact");
            faces.sort();
            assert!(!faces.is_empty(), "an arc wall is cylindrical: {node:?}");
            SitedRef::new(node, faces.remove(0))
        };
        vec![wall(disc), wall(bore)]
    };
    let web = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    let measure = r.insert(Node::measure(web, refs).expect("both indices in range"));
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(2.0e-3),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

/// **THE E2E EXERCISE**: R1's split-bore disc driven with the door open
/// and shut at three scales — receipt, decisions, the human render's
/// door line, the stackup and the assertion, plus the first refusal at
/// the real box and its ceiling.
#[test]
#[ignore = "evidence-only: R1's own end-to-end arc study, door on and off"]
fn r1_split_bore_disc_end_to_end() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for scale in [1.0_f64, 1.0e2 * eps, 1.0e-4] {
        let (doc, measure, assertion) = split_bore_disc(scale, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for (label, r) in rows() {
            let t = Instant::now();
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    symbolic: dials(r),
                    max_leaves: 32,
                    ..DriveConfig::default()
                },
                tol,
            );
            let dt = t.elapsed().as_secs_f64();
            match v {
                Err(e) => println!("== scale {scale:e} {label} {dt:>6.1}s: REFUSED {e}"),
                Ok(v) => {
                    println!(
                        "== scale {scale:e} {label} {dt:>6.1}s: receipt={:?} decisions={:?}",
                        v.receipt(),
                        v.decisions()
                    );
                    for line in v.render(&analyzed).lines().filter(|l| {
                        l.contains("registered") || l.contains("symbolic") || l.contains("certif")
                    }) {
                        println!("      render| {line}");
                    }
                    let stack = editor_core::stackup::stackup(
                        &doc, measure, &analyzed, &v, None, false, tol,
                    );
                    println!("      stackup {stack:?}");
                    let a = editor_core::drive::assertion_at(
                        &doc,
                        assertion,
                        v.root(),
                        dials(r),
                        tol,
                    );
                    println!("      assertion {a:?}");
                }
            }
        }
        let (_, refusal, counts) = replay(
            &doc,
            &ParamBox::of(&analyzed),
            SymRules::shipped(),
            tol,
        );
        println!(
            "   whole-box replay (door ON): {counts:?}; first refusal {}",
            refusal.as_deref().map(pred).unwrap_or_else(|| "-".into())
        );
    }
    // And the ceiling, both ways.
    let at = |s: f64| split_bore_disc(s, tol).0;
    for (label, r) in rows() {
        let (lo, hi, per) = ceiling(&at, r, tol, 1.0e0 * eps, 1.0e5 * eps, 10);
        println!(
            "   split_bore_disc {label}: [{:.4e}, {:.4e}]·eps ({per:.2}s/probe)",
            lo / eps,
            hi / eps
        );
    }
}

/// **The value-channel differential on a document the unit never
/// measured** (claim 1 and 6): the same drive, door on and off, with
/// every line but the receipt's decision line compared.
#[test]
#[ignore = "evidence-only: R1's differential on its own document"]
fn r1_the_door_moves_no_bit_on_r1s_document() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for scale in [1.0e2 * eps, 1.0e-4] {
        let (doc, _, _) = split_bore_disc(scale, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let run = |r: SymRules| {
            drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 8,
                    symbolic: dials(r),
                    ..DriveConfig::default()
                },
                tol,
            )
            .map(|v| v.serialize())
        };
        let (open, shut) = (run(SymRules::shipped()), run(SymRules::shipped_without_the_door()));
        match (open, shut) {
            (Ok(o), Ok(s)) => {
                let strip = |t: &str| {
                    t.lines()
                        .filter(|l| !l.starts_with("decisions "))
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                println!(
                    "   scale {scale:e}: identical off the receipt line = {}; receipt moved = {}",
                    strip(&o) == strip(&s),
                    o != s
                );
                for l in o.lines().filter(|l| l.starts_with("decisions ")) {
                    println!("      open| {l}");
                }
                for l in s.lines().filter(|l| l.starts_with("decisions ")) {
                    println!("      shut| {l}");
                }
            }
            (o, s) => println!("   scale {scale:e}: open={o:?} shut={s:?}"),
        }
    }
}

/// **Claim 5, re-taken**: does the pad's ceiling move when the
/// CONSUMER's node is the one registered? The door accepts the
/// consumer's node (the finding says so), but no constructor holds it;
/// this row measures what the pad's ceiling is either way, so the
/// "`line_span` is behind it" claim is a number here too.
#[test]
#[ignore = "evidence-only: the pad's ceiling and what bounds it"]
fn r1_the_pads_bound_is_line_span_either_way() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for mult in [1.0_f64, 2.0, 8.0] {
        let doc = crate::m10_8_r2_probes_interval::pad(2.083e3 * eps * mult, tol).0;
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for (label, r) in rows() {
            let (shapes, refusal, counts) =
                replay(&doc, &ParamBox::of(&analyzed), r, tol);
            let mut blocked: BTreeMap<&'static str, usize> = BTreeMap::new();
            for sh in shapes.iter().filter(|sh| {
                matches!(
                    sh.outcome,
                    ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
                )
            }) {
                *blocked.entry(sh.predicate).or_default() += 1;
            }
            println!(
                "   pad x{mult} {label}: first {:<70} blocked {blocked:?} reg={}",
                refusal.as_deref().map(pred).unwrap_or_else(|| "-".into()),
                counts.registered
            );
        }
        println!(
            "      certifies_whole open={} shut={}",
            certifies_whole(&doc, SymRules::shipped(), tol),
            certifies_whole(&doc, SymRules::shipped_without_the_door(), tol)
        );
    }
}
