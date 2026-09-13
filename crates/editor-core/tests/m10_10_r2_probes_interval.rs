//! **R2's independent probes of M10-10 on documents** (PR #2100, frozen
//! head `e904d9691`): the plate's real study driven whole with every
//! REFUSED leaf replayed and read as its over-band set (and a sample of
//! them refined one level further), and an arc-bearing document of
//! R2's own — a D-tab with an authored-bulge arc and a hole — driven
//! end to end with the algebra on and off.
//!
//! Every row is `#[ignore]`d EVIDENCE: it prints, and asserts only
//! what must hold for the printed numbers to mean anything.
//!
//! ```sh
//! cargo test --release -p editor-core --features interval --test all -- \
//!   m10_10_r2_probes_interval:: --ignored --nocapture --test-threads 1
//! ```
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::analysis::{
    AnalysisPolicy, AnalyzedBox, BoxAxis, ParamBox, analyzed_box, box_mass,
};
use editor_core::drive::{DriveConfig, RefusalReason, drive};
use editor_core::stackup::stackup;
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, NamePat, Node, ParamName, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, Selector, SitedRef, SurfaceKindSet,
    UnitSym, select_where,
};
use geom_core::{SymRules, Tol};

use crate::fixture::Recorder;
use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{OverBand, ceiling, certifies_whole, over_band_set, render_over_band};

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// The mass of a leaf's box under the study's own distributions — the
/// product over the varying axes, independently of the driver's
/// accounting.
fn leaf_mass(analyzed: &AnalyzedBox, box_: &ParamBox) -> f64 {
    let mut m = 1.0;
    for (name, axis) in box_.axes() {
        let Some(p) = analyzed.get(name) else {
            continue;
        };
        let Some(dist) = &p.distribution else {
            continue;
        };
        if let BoxAxis::Varying { lo, hi } = axis {
            m *= box_mass(name, dist, (*lo, *hi)).unwrap_or(f64::NAN);
        }
    }
    m
}

/// The signature of an over-band set: its predicate names, in order.
fn signature(set: &[OverBand]) -> String {
    if set.is_empty() {
        "{} (certifies)".to_owned()
    } else {
        format!(
            "{{{}}}",
            set.iter()
                .map(|e| e.predicate)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// The 2ⁿ children of a box, every varying axis bisected.
fn children(box_: &ParamBox) -> Vec<ParamBox> {
    let mut out = vec![BTreeMap::new()];
    for (name, axis) in box_.axes() {
        let halves: Vec<BoxAxis> = match *axis {
            BoxAxis::Fixed => vec![BoxAxis::Fixed],
            BoxAxis::Varying { lo, hi } => {
                let mid = 0.5 * (lo + hi);
                vec![
                    BoxAxis::Varying { lo, hi: mid },
                    BoxAxis::Varying { lo: mid, hi },
                ]
            }
        };
        out = out
            .into_iter()
            .flat_map(|acc: BTreeMap<ParamName, BoxAxis>| {
                halves.iter().map(move |h| {
                    let mut a = acc.clone();
                    a.insert(name.clone(), *h);
                    a
                })
            })
            .collect();
    }
    out.into_iter().map(ParamBox::from_axes).collect()
}

/// **The plate's real study driven whole, and every refused leaf read
/// as its over-band set** — `CAD_R2_LEAVES` leaves (default 1024). For
/// each refused leaf: its reason class, the over-band set of a
/// whole-leaf replay under the shipped set, and the widest
/// `assert_bound` enclosure; a tally of the set signatures over all
/// refused leaves. Then `CAD_R2_REFINE` refused leaves (default 16,
/// spread over the list) are bisected one level and each child
/// replayed: certifies / bounded by `assert_bound` alone / bounded by
/// something else. The certified mass is re-summed here from the
/// study's own distributions, beside the driver's accounting.
#[test]
#[ignore = "evidence-only: drives the plate's real study and reads every refused leaf's over-band set"]
fn r2_evidence_every_refused_leaf_of_the_plates_real_study_read_as_its_set() {
    let tol = Tol::witness();
    let max_leaves = env_usize("CAD_R2_LEAVES", 1024);
    let refine = env_usize("CAD_R2_REFINE", 16);
    let (doc, measure, _) = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let t = std::time::Instant::now();
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves,
            ..DriveConfig::default()
        },
        tol,
    )
    .expect("the nominal builds");
    println!(
        "== plate real study at {max_leaves} leaves: {:?} in {:.1}s",
        verdict.receipt(),
        t.elapsed().as_secs_f64()
    );
    let acc = verdict.accounting();
    let certified_mass: f64 = verdict
        .certified()
        .iter()
        .map(|l| leaf_mass(&analyzed, &l.box_))
        .sum();
    let refused_mass: f64 = verdict
        .refused()
        .iter()
        .map(|l| leaf_mass(&analyzed, &l.box_))
        .sum();
    println!(
        "   mass under the study's distributions: certified {certified_mass:.5}, refused \
         {refused_mass:.5}, sum {:.5}; driver's accounting: certified {:?}, unanalyzed {:?}",
        certified_mass + refused_mass,
        acc.certified,
        acc.unanalyzed
    );
    match stackup(&doc, measure, &analyzed, &verdict, None, true, tol) {
        Ok(r) => println!(
            "   stackup: worst case [{:.6e}, {:.6e}] over {} leaves; nominal {:?}",
            r.worst_case.lo, r.worst_case.hi, r.worst_case.leaves, r.nominal
        ),
        Err(e) => println!("   stackup refused: {e}"),
    }
    // Every refused leaf, replayed whole and read as its set.
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();
    let mut signatures: BTreeMap<String, usize> = BTreeMap::new();
    let mut assert_lo = f64::INFINITY;
    let mut assert_hi = f64::NEG_INFINITY;
    let mut assert_narrowest = f64::INFINITY;
    let t = std::time::Instant::now();
    for leaf in verdict.refused() {
        let class = match &leaf.reason {
            RefusalReason::Budget(_) => "Budget",
            RefusalReason::SliverTerminal { .. } => "SliverTerminal",
            RefusalReason::FlipCrossing { .. } => "FlipCrossing",
            _ => "other",
        };
        *classes.entry(class.to_owned()).or_default() += 1;
        let (shapes, _, _) = replay(&doc, &leaf.box_, SymRules::shipped(), tol);
        let set = over_band_set(&shapes);
        *signatures.entry(signature(&set)).or_default() += 1;
        if let Some(e) = set.iter().find(|e| e.predicate == "assert_bound") {
            assert_lo = assert_lo.min(e.enclosure.0);
            assert_hi = assert_hi.max(e.enclosure.1);
            assert_narrowest = assert_narrowest.min(e.enclosure.1 - e.enclosure.0);
        }
    }
    println!(
        "   refused leaves by class: {classes:?} ({} replays in {:.1}s)",
        verdict.refused().len(),
        t.elapsed().as_secs_f64()
    );
    println!("   over-band SET signatures over the refused leaves: {signatures:?}");
    println!(
        "   assert_bound over the refused leaves: enclosures within [{assert_lo:.3e}, \
         {assert_hi:.3e}], narrowest {assert_narrowest:.3e}"
    );
    assert!(
        classes.keys().all(|k| k == "Budget"),
        "the PR says every refusal is Budget: {classes:?}"
    );
    // A sample of refused leaves refined one level further.
    let n = verdict.refused().len();
    let mut child_tally: BTreeMap<String, usize> = BTreeMap::new();
    let mut shown = 0;
    for i in (0..n).step_by((n / refine).max(1)).take(refine) {
        let leaf = &verdict.refused()[i];
        let (shapes, _, _) = replay(&doc, &leaf.box_, SymRules::shipped(), tol);
        let parent = over_band_set(&shapes);
        for child in children(&leaf.box_) {
            let (shapes, _, _) = replay(&doc, &child, SymRules::shipped(), tol);
            let set = over_band_set(&shapes);
            *child_tally.entry(signature(&set)).or_default() += 1;
            if shown < 12 && set.iter().any(|e| e.predicate != "assert_bound") {
                shown += 1;
                println!("   -- child of refused leaf #{i} bounded by something else:");
                println!("{}", render_over_band(&set));
            }
        }
        if i == 0 {
            println!(
                "   refused leaf #0 parent set:\n{}",
                render_over_band(&parent)
            );
        }
    }
    println!("   children (one bisection level, {refine} refused leaves sampled): {child_tally:?}");
}

/// **R2's own arc-bearing document: a D-tab.** A rectangle whose right
/// side is an arc authored by BULGE (`ArcTo(Bulge)`), a round hole
/// inside it, and the measure `distance(hole wall, arc wall)` with an
/// assertion on it. Two variants: the bulge a LITERAL (0.4) with the
/// hole's centre and radius varying, and the bulge a PARAMETER (the
/// honest limit rule D states: `atan|b|` against `atan b`).
///
/// Geometry (metres): chord `(4e-3, ∓2e-3)`, bulge `b` → sagitta
/// `2e-3·b`, radius `(1 + b²)/(2b)·2e-3`, centre `x = 4e-3 + 2e-3·b −
/// R`; hole at `(hole_x, 0)` radius `hole_r`. The face-to-face gap
/// along the axis is `4e-3 + 2e-3·b − hole_x − hole_r` — linear —
/// 1.8 mm at the nominal.
pub(crate) fn d_tab(
    scale: f64,
    bulge_is_a_parameter: bool,
    tol: Tol,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("finite length");
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite scalar");
    let mut r = Recorder::new();
    let declare = |r: &mut Recorder, n: &str, dim: Dimension, value: f64, d: Distribution| {
        r.push(DocEdit::SetDocParam {
            name: ParamName::new(n),
            value: DocParam::Continuous {
                dim,
                value,
                display_unit: UnitSym::canonical_for(dim),
                distribution: Some(d),
            },
        });
    };
    declare(
        &mut r,
        "hole_x",
        Dimension::Length,
        2.5e-3,
        Distribution::Uniform {
            lo: -5.0e-5 * scale,
            hi: 5.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "hole_r",
        Dimension::Length,
        0.5e-3,
        Distribution::Normal {
            sigma: 1.0e-5 * scale,
        },
    );
    let bulge = if bulge_is_a_parameter {
        declare(
            &mut r,
            "bulge",
            Dimension::Scalar,
            0.4,
            Distribution::Uniform {
                lo: -0.05 * scale,
                hi: 0.05 * scale,
            },
        );
        Expr::param(ParamName::new("bulge"), Dimension::Scalar)
    } else {
        scl(0.4)
    };
    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let outline = LoopProgram::Chain(vec![
        ProgramStep::At([len(-4.0e-3), len(-2.0e-3)]),
        ProgramStep::LineTo(ProgramTarget::Point([len(4.0e-3), len(-2.0e-3)])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point([len(4.0e-3), len(2.0e-3)]),
            b: bulge,
        }),
        ProgramStep::LineTo(ProgramTarget::Point([len(-4.0e-3), len(2.0e-3)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![outline],
    }));
    let thickness = len(1.0e-3);
    let tab = r.insert(Node::Extrude {
        profile,
        distance: thickness.clone(),
    });
    let hole_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [
                Expr::param(ParamName::new("hole_x"), Dimension::Length),
                len(0.0),
            ],
            radius: Expr::param(ParamName::new("hole_r"), Dimension::Length),
        }],
    }));
    let hole = r.insert(Node::Extrude {
        profile: hole_profile,
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
            // A seamed cylinder comes as two faces (the plate's fixture
            // takes the first too); the arc wall is one.
            assert!(!faces.is_empty(), "a cylindrical wall on node {}", node.0);
            SitedRef::new(node, faces.remove(0))
        };
        vec![wall(hole), wall(tab)]
    };
    let gap = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    let measure = r.insert(Node::measure(gap, refs).expect("both indices in range"));
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(1.78e-3),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

/// **The D-tab end to end**, both variants: the real study driven with
/// the algebra on and off (`CAD_R2_LEAVES`, default 64), the receipts
/// and the certified worst case; the whole-certifying bracket under
/// both rule sets (a 14-step log bisection between `0.1·ε` and `10×`
/// the study) and the over-band set at ceiling + δ with enclosures.
#[test]
#[ignore = "evidence-only: R2's D-tab driven end to end with the algebra on and off"]
fn r2_evidence_the_d_tab_end_to_end() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let max_leaves = env_usize("CAD_R2_LEAVES", 64);
    for bulge_param in [false, true] {
        let name = if bulge_param {
            "d_tab (bulge a PARAMETER)"
        } else {
            "d_tab (bulge a literal)"
        };
        println!("==== {name}, eps = {eps:e}");
        let at = |s: f64| d_tab(s, bulge_param, tol).0;
        let (doc, measure, _) = d_tab(1.0, bulge_param, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for (label, rules) in [
            ("algebra OFF (M10-9)", SymRules::without_the_algebra()),
            ("algebra ON  (shipped)", SymRules::shipped()),
        ] {
            let t = std::time::Instant::now();
            let verdict = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves,
                    symbolic: crate::m10_8_harness::dials(rules),
                    ..DriveConfig::default()
                },
                tol,
            )
            .expect("the nominal builds");
            let certified_mass: f64 = verdict
                .certified()
                .iter()
                .map(|l| leaf_mass(&analyzed, &l.box_))
                .sum();
            let mut classes: BTreeMap<String, usize> = BTreeMap::new();
            for leaf in verdict.refused() {
                let k = format!("{:?}", leaf.reason);
                let k = k.split(['{', '(']).next().unwrap_or(&k).trim().to_owned();
                *classes.entry(k).or_default() += 1;
            }
            println!(
                "   real study, {label}: {:?} in {:.1}s; certified mass {certified_mass:.4}; \
                 refusals {classes:?}",
                verdict.receipt(),
                t.elapsed().as_secs_f64()
            );
            if let Some(l) = verdict.certified().first() {
                println!("      first certified leaf's receipt: {:?}", l.decisions);
            }
            match stackup(&doc, measure, &analyzed, &verdict, None, false, tol) {
                Ok(r) => println!(
                    "      stackup: worst case [{:.6e}, {:.6e}] over {} leaves; nominal {:?}",
                    r.worst_case.lo, r.worst_case.hi, r.worst_case.leaves, r.nominal
                ),
                Err(e) => println!("      stackup refused: {e}"),
            }
        }
        for (label, rules) in [
            ("algebra OFF (M10-9)", SymRules::without_the_algebra()),
            ("algebra ON  (shipped)", SymRules::shipped()),
        ] {
            let (lo, hi, per) = ceiling(&at, rules, tol, 1.0e-1 * eps, 1.0e1, 14);
            println!(
                "   {label}: certifies x{lo:.4e}, refuses x{hi:.4e} of the real study \
                 [= {:.4e}·eps .. {:.4e}·eps] ({per:.2}s/probe)",
                lo / eps,
                hi / eps
            );
            if !(lo.is_finite() && hi.is_finite()) {
                continue;
            }
            let doc = at(hi);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let (shapes, refusal, counts) = replay(&doc, &ParamBox::of(&analyzed), rules, tol);
            println!("      at ceiling + delta: {counts:?}; the drive stops at {refusal:?}");
            println!("{}", render_over_band(&over_band_set(&shapes)));
            let certifying = certifies_whole(&at(lo), rules, tol);
            assert!(
                certifying,
                "{name} {label}: the bracket's lower end certifies"
            );
        }
    }
}

/// EVIDENCE — **every M10 fixture's drive under
/// `SymRules::without_the_algebra()`, serialized in full** (the
/// receipt line included), for a byte diff against the MERGE BASE's
/// `SymRules::shipped()` — the bit-identity claim (claim 5 / D-off)
/// checked against the base binary rather than against re-pinned rows.
#[test]
#[ignore = "evidence-only: serializes every M10 fixture with the algebra off, for a diff against the base"]
fn r2_evidence_serialize_every_m10_fixture_with_the_algebra_off() {
    let tol = Tol::witness();
    for (name, doc) in crate::m10_9_r2_probes_interval::fixtures(tol) {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let out = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 8,
                symbolic: crate::m10_8_harness::dials(SymRules::without_the_algebra()),
                ..DriveConfig::default()
            },
            tol,
        )
        .map(|v| v.serialize())
        .map_err(|e| format!("{e:?}"));
        println!("##### {name}");
        match out {
            Ok(text) => println!("{text}"),
            Err(e) => println!("ERR {e}"),
        }
    }
}
