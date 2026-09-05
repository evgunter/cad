//! **R2's end-to-end probes of M10-7** (ERROR-DESIGN E12 + E3's extent
//! lever) — a distributed parametric document of R2's own, authored
//! through the public doors, driven with the tier ON and OFF.
//!
//! The fixture is deliberately NEITHER of the unit's two: it is a
//! filleted L-bracket (so an ARC stands between the profile and its
//! carriers, which is the family deviation D4 names) whose extrusion
//! distance is a DIVISION of a parameter, with two bores whose walls the
//! study measures. Everything the unit measured on the slab and the
//! plate is re-derived here on geometry the unit never built.
//!
//! Most rows are `#[ignore]`d evidence probes ([[test-suite-cost]]: a
//! row that only prints cannot gate). The rows that ASSERT are named so
//! and carry the claim they falsify.
//!
//! ITS PROBE-GATED CODE IS NOT EXECUTED BY CI — the disposition this reviewer's suite asks for. Its
//! `probe`-gated rows are evidence: an end-to-end study printed for a
//! reader, a ceiling re-derived, a per-predicate breakdown. None of
//! them has a threshold to cross, so none of them can gate; what they
//! produce is quoted in the unit's deviations with this file named. The
//! rows that ASSERT are NOT probe-gated and run on every merge.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, NamePat, Node, ParamName, ProfileDoc, ProfileProgram,
    ProgramStep, ProgramTarget, RecipeNodeId, Selector, SitedRef, SurfaceKindSet, UnitSym,
    select_where,
};
use geom_core::Tol;

use crate::fixture::Recorder;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn plen(n: &str) -> Expr {
    Expr::param(ParamName::new(n), Dimension::Length)
}

/// The nominal arm of the bracket, in metres.
const ARM: f64 = 3.0e-3;
/// The nominal fillet radius.
const FILLET: f64 = 0.5e-3;
/// The nominal bore radius.
const BORE: f64 = 0.25e-3;
/// The nominal centres of the two bores, along x.
const BORE_A_X: f64 = 0.6e-3;
const BORE_B_X: f64 = 2.2e-3;

/// **R2's own document**: a filleted L-bracket with two bores, extruded
/// through a DIVISION of a parameter, measuring the web between the two
/// bore walls and asserting a floor on it.
///
/// `scale` multiplies every tolerance together, so `1.0` is the study a
/// user would actually ask for and a smaller number is a narrower one.
pub(crate) fn bracket(scale: f64, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
    // The arm is a uniform tolerance; the fillet radius is a uniform
    // tolerance (the ARC's own parameter); each bore is normal.
    declare(
        &mut r,
        "arm",
        ARM,
        Distribution::Uniform {
            lo: -2.0e-5 * scale,
            hi: 2.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "fillet_r",
        FILLET,
        Distribution::Uniform {
            lo: -1.0e-5 * scale,
            hi: 1.0e-5 * scale,
        },
    );
    for n in ["bore_a", "bore_b"] {
        declare(
            r_mut(&mut r),
            n,
            BORE,
            Distribution::Normal {
                sigma: 5.0e-6 * scale,
            },
        );
    }

    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));

    // The L, with a PARAMETRIC fillet at its inner corner — the arc.
    let pt = |x: Expr, y: Expr| ProgramTarget::Point([x, y]);
    let bracket_loop = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::LineTo(pt(plen("arm"), len(0.0))),
        ProgramStep::LineTo(pt(plen("arm"), len(1.0e-3))),
        ProgramStep::Toward {
            dx: scl(-1.0),
            dy: scl(0.0),
        },
        ProgramStep::Fillet(plen("fillet_r")),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(1.0),
        },
        ProgramStep::FarEndTo([len(1.0e-3), len(3.0e-3)]),
        ProgramStep::LineTo(pt(len(0.0), len(3.0e-3))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![bracket_loop],
    }));
    // THE DIVISION: the plate is a quarter of the arm thick.
    let thickness = Expr::div(plen("arm"), scl(4.0)).expect("Length / Scalar");
    let _body = r.insert(Node::Extrude {
        profile,
        distance: thickness.clone(),
    });

    let bore = |r: &mut Recorder, x: f64, radius: &str| {
        let profile = r.insert(Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Circle {
                centre: [len(x), len(0.5e-3)],
                radius: plen(radius),
            }],
        }));
        r.insert(Node::Extrude {
            profile,
            distance: thickness.clone(),
        })
    };
    let bore_a = bore(&mut r, BORE_A_X, "bore_a");
    let bore_b = bore(&mut r, BORE_B_X, "bore_b");

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
            assert!(!faces.is_empty(), "a bore extrude has a cylindrical wall");
            SitedRef::new(node, faces.remove(0))
        };
        vec![wall(bore_a), wall(bore_b)]
    };

    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(
            MeasureExpr::value(plen("bore_a")),
            MeasureExpr::value(plen("bore_b")),
        )
        .expect("Length + Length"),
    )
    .expect("Length - Length");
    let measure = r.insert(Node::measure(web, refs).expect("both indices in range"));
    let nominal_web = BORE_B_X - BORE_A_X - 2.0 * BORE;
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(nominal_web - 5.0e-5),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

fn r_mut(r: &mut Recorder) -> &mut Recorder {
    r
}

fn drive_at(doc: &ProfileDoc, dials: SymbolicDials, tol: Tol) -> Option<String> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    drive(
        doc,
        &analyzed,
        &DriveConfig {
            symbolic: dials,
            max_leaves: 512,
            ..DriveConfig::default()
        },
        tol,
    )
    .ok()
    // `serialize()` is the goldening form — the text M10-6's own
    // differential is taken over — not the human `render()`.
    .map(|v| v.serialize())
}

/// **THE E2E EXERCISE**: R2's own document, driven both ways, with the
/// receipts, the stackup and the assertion verdicts printed.
///
/// EVIDENCE-ONLY.
#[test]
#[ignore = "evidence-only: prints R2's own end-to-end tolerance study"]
fn r2_end_to_end_bracket_study() {
    let tol = Tol::witness();
    for scale in [1.0_f64, 1.0e-2, 1.0e-4] {
        let (doc, measure, assertion) = bracket(scale, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for (name, dials) in [
            ("TIER ON ", SymbolicDials::default()),
            ("TIER OFF", SymbolicDials::off()),
        ] {
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    symbolic: dials,
                    max_leaves: 512,
                    ..DriveConfig::default()
                },
                tol,
            );
            match v {
                Err(e) => println!("== scale {scale:e} {name}: REFUSED {e}"),
                Ok(v) => {
                    println!(
                        "== scale {scale:e} {name}: receipt={:?} decisions={:?}",
                        v.receipt(),
                        v.decisions()
                    );
                    println!("{}", v.render(&analyzed));
                    let stack = editor_core::stackup::stackup(
                        &doc, measure, &analyzed, &v, None, false, tol,
                    );
                    println!("   stackup: {stack:?}");
                    let a = editor_core::drive::assertion_at(&doc, assertion, v.root(), dials, tol);
                    println!("   assertion at the root box: {a:?}");
                }
            }
        }
    }
}

/// **The ceiling on R2's own arc-bearing document**, tier on against
/// tier off — deviation D4's claim, re-derived on geometry the unit
/// never built.
///
/// EVIDENCE-ONLY.
#[test]
#[ignore = "evidence-only: prints R2's re-derived ceiling"]
fn r2_the_ceiling_on_an_arc_bearing_bracket() {
    let tol = Tol::witness();
    let certifies_whole = |scale: f64, dials: SymbolicDials| {
        let (doc, _, _) = bracket(scale, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_depth: 0,
                max_leaves: 1,
                symbolic: dials,
                ..DriveConfig::default()
            },
            tol,
        )
        .is_ok_and(|v| v.receipt().certified == 1)
    };
    let ceiling = |dials: SymbolicDials| -> f64 {
        let (mut lo, mut hi) = (1.0e-14, 1.0e3);
        if !certifies_whole(lo, dials) {
            return f64::NAN;
        }
        if certifies_whole(hi, dials) {
            return f64::INFINITY;
        }
        for _ in 0..40 {
            let mid = (0.5 * (lo.ln() + hi.ln())).exp();
            if certifies_whole(mid, dials) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    };
    let off = ceiling(SymbolicDials::off());
    let on = ceiling(SymbolicDials::default());
    println!("   R2 bracket, TIER OFF: widest whole-certifying box x{off:e} of the study");
    println!("   R2 bracket, TIER ON : widest whole-certifying box x{on:e} of the study");
    println!("   factor {:e}", on / off);
}

/// **The slab's ceiling, re-derived** — the PR's headline `0.488`, taken
/// with a bisection of R2's own and to more places than the unit's row
/// pins, plus the first refusal beyond it.
///
/// EVIDENCE-ONLY.
#[test]
#[ignore = "evidence-only: re-derives the slab's ceiling and its first refusal"]
fn r2_re_derives_the_slab_ceiling() {
    use crate::m10_3_driver_interval::slab;
    let tol = Tol::witness();
    let whole = |half: f64, dials: SymbolicDials| {
        let doc = slab(1.0, half);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_depth: 0,
                max_leaves: 1,
                symbolic: dials,
                ..DriveConfig::default()
            },
            tol,
        )
        .is_ok_and(|v| v.receipt().certified == 1)
    };
    for (name, dials) in [
        ("TIER ON ", SymbolicDials::default()),
        ("TIER OFF", SymbolicDials::off()),
    ] {
        let (mut lo, mut hi) = (1.0e-18, 1.0);
        assert!(whole(lo, dials), "{name}: the floor must certify");
        assert!(!whole(hi, dials), "{name}: the ceiling must not");
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if whole(mid, dials) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        println!("   slab {name}: widest whole-certifying HALF-WIDTH = {lo:.6e} .. {hi:.6e}");
    }
    // What refuses first beyond the ceiling, named.
    let doc = slab(1.0, 0.6);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_depth: 0,
            max_leaves: 1,
            ..DriveConfig::default()
        },
        tol,
    );
    println!("   slab at half-width 0.6, one leaf: {v:?}");
}

/// **D9, independently**: the same drive repeated, and the same drive
/// under the rayon schedule, produce byte-identical serializations,
/// content keys and receipts.
///
/// This row ASSERTS.
#[test]
fn r2_the_drive_is_bit_identical_across_repeats_and_the_rayon_schedule() {
    let tol = Tol::witness();
    let (doc, _, _) = bracket(1.0e-3, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |parallel: bool| {
        let v = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                parallel,
                max_leaves: 256,
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the bracket's nominal builds");
        (
            v.serialize(),
            format!("{:?}", v.content_key()),
            v.decisions(),
        )
    };
    let a = run(false);
    let b = run(false);
    let c = run(true);
    assert_eq!(a.0, b.0, "two sequential drives serialized differently");
    assert_eq!(a.1, b.1, "two sequential drives keyed differently");
    assert_eq!(a.2, b.2, "two sequential drives counted differently");
    assert_eq!(a.0, c.0, "the rayon schedule serialized differently");
    assert_eq!(a.1, c.1, "the rayon schedule keyed differently");
    assert_eq!(
        a.2, c.2,
        "the rayon schedule counted differently — a per-leaf session leaked"
    );
}

/// **Claim 1, on R2's own document**: the tier off serializes with NO
/// symbolic line at all, and the tier on serializes with one.
///
/// This row ASSERTS. It is the half of the tier-off differential that
/// does not need the merge base in hand; the byte differential against
/// the merge base is run out-of-tree and reported in the review.
#[test]
fn r2_the_tier_off_serialization_carries_no_symbolic_line() {
    let tol = Tol::witness();
    let (doc, _, _) = bracket(1.0e-3, tol);
    let off = drive_at(&doc, SymbolicDials::off(), tol).expect("tier off drives");
    let on = drive_at(&doc, SymbolicDials::default(), tol).expect("tier on drives");
    assert!(
        !off.contains("decisions symbolic_zero"),
        "the tier-off serialization carried an E12 line:\n{off}"
    );
    assert!(
        on.contains("decisions symbolic_zero"),
        "the tier-on serialization carried no E12 line:\n{on}"
    );
}

/// **A zero-term budget is claim 1 again, from inside the scalar.**
///
/// This row ASSERTS.
#[test]
fn r2_a_zero_term_budget_reproduces_the_tier_off_verdict() {
    let tol = Tol::witness();
    let (doc, _, _) = bracket(1.0e-3, tol);
    let off = drive_at(&doc, SymbolicDials::off(), tol).expect("tier off drives");
    let zero_budget = drive_at(
        &doc,
        SymbolicDials {
            enabled: true,
            max_terms: 0,
            max_degree: 0,
            ..SymbolicDials::default()
        },
        tol,
    )
    .expect("a zero budget drives");
    // The verdicts must agree everywhere the E12 receipt is not the
    // subject; the receipt line itself is the one legitimate difference,
    // since a zero-budget run still COUNTS its (numeric) decisions.
    // The E12 receipt line is the ONE legitimate difference: a
    // zero-budget run still counts its (numeric) decisions, where a
    // tier-off run installs no session and counts nothing.
    let strip = |s: &str| {
        s.lines()
            .filter(|l| !l.starts_with("decisions symbolic_zero"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        strip(&off),
        strip(&zero_budget),
        "a zero-term budget did not reproduce the tier-off verdict"
    );
}

/// **The measure's own answer, both ways** — what a real tolerance study
/// gets today on R2's document.
///
/// EVIDENCE-ONLY.
#[test]
#[ignore = "evidence-only: prints the study's stackup and assertion"]
fn r2_what_a_real_study_gets_today() {
    let tol = Tol::witness();
    let (doc, measure, assertion) = bracket(1.0, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    for (name, dials) in [
        ("TIER ON ", SymbolicDials::default()),
        ("TIER OFF", SymbolicDials::off()),
    ] {
        match drive(
            &doc,
            &analyzed,
            &DriveConfig {
                symbolic: dials,
                max_leaves: 512,
                ..DriveConfig::default()
            },
            tol,
        ) {
            Err(e) => println!("== {name}: the drive refused: {e}"),
            Ok(v) => {
                println!("== {name}: {:?} / {:?}", v.receipt(), v.decisions());
                for l in v.refused().iter().take(4) {
                    println!("   refused: {:?}", l.reason);
                }
                let s =
                    editor_core::stackup::stackup(&doc, measure, &analyzed, &v, None, false, tol);
                println!("   stackup: {s:?}");
                println!(
                    "   assertion: {:?}",
                    editor_core::drive::assertion_at(&doc, assertion, v.root(), dials, tol)
                );
            }
        }
    }
    // And the degenerate box, so the nominal answer is visible beside it.
    let nominal = ParamBox::from_axes(
        ParamBox::of(&analyzed)
            .axes()
            .keys()
            .map(|n| (n.clone(), BoxAxis::Fixed))
            .collect(),
    );
    let _ = Arc::new(nominal);
}

// ------------------------------------------- claim 4: the census rows

/// A rectangle with a REDUNDANT COLLINEAR VERTEX on its bottom edge:
/// two consecutive wall segments that really are collinear, so their
/// side planes really are cosurface.
#[cfg(feature = "probe")]
fn collinear_walls() -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("w"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 4.0e-3,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -1.0e-5,
                hi: 1.0e-5,
            }),
        },
    });
    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    // The middle vertex of the bottom edge splits ONE straight edge in
    // two: segments 0 and 1 are collinear by construction, whatever `w`
    // does, so `side_planes_cosurface` is a genuine IDENTITY here and
    // not a coincidence at the nominal.
    let w = || Expr::param(ParamName::new("w"), Dimension::Length);
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            // The DECLARED straight continuation: `LineTo` at a zero-turn
            // junction is refused by the profile door (`JunctionTangent`
            // — an undeclared zero-turn joint), so the split vertex is
            // authored the way the vocabulary spells it.
            ProgramStep::LineTo(ProgramTarget::Point([
                Expr::div(w(), scl(2.0)).expect("Length / Scalar"),
                len(0.0),
            ])),
            ProgramStep::ContinueTo(ProgramTarget::Point([w(), len(0.0)])),
            ProgramStep::LineTo(ProgramTarget::Point([w(), len(2.0e-3)])),
            ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(2.0e-3)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
    }));
    r.insert(Node::Extrude {
        profile,
        distance: len(1.0e-3),
    });
    r.doc
}

/// **Claim 4's `side_planes_cosurface` argument, tested.** The PR argues
/// its 0/8 is "genuinely not cosurface on a rectangle" rather than a
/// miss. On a profile whose consecutive walls REALLY ARE cosurface, the
/// tier should discharge it symbolically.
///
/// EVIDENCE-ONLY (it prints the split for every predicate).
#[cfg(feature = "probe")]
#[test]
#[ignore = "evidence-only: prints the split on a collinear-walled profile"]
fn r2_collinear_walls_should_discharge_side_planes_cosurface() {
    use std::collections::BTreeMap;
    let tol = Tol::witness();
    for (name, doc) in [
        ("collinear-walled prism", collinear_walls()),
        ("R2 filleted bracket", bracket(1.0, tol).0),
    ] {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let nominal = ParamBox::from_axes(
            ParamBox::of(&analyzed)
                .axes()
                .keys()
                .map(|n| (n.clone(), BoxAxis::Fixed))
                .collect(),
        );
        let opts = editor_core::EvalOptions {
            param_box: Some(Arc::new(nominal)),
            profile_lift: editor_core::ProfileLift::Guided,
            ..editor_core::EvalOptions::default()
        };
        geom_core::k_stats::start_recording();
        let _ = geom_core::sym::with_session(
            geom_core::SymBudget {
                max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
                max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
            },
            || {
                let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Probe>> =
                    editor_core::evaluate(&doc, None, &editor_core::CancelToken::new(), &opts, tol);
                ev
            },
        );
        let mut rows: BTreeMap<&'static str, (u64, u64)> = BTreeMap::new();
        for s in geom_core::k_stats::take_samples() {
            let e = rows.entry(s.predicate).or_default();
            match s.outcome {
                geom_core::k_stats::SampleOutcome::SymbolicZero => e.0 += 1,
                _ => e.1 += 1,
            }
        }
        println!("== {name}");
        for (p, (sym, num)) in &rows {
            println!("   {p:<40} symbolic={sym:<6} numeric={num}");
        }
    }
}

// ------------------------------- claim 1: the merge-base differential

/// **The tier-off differential, against the MERGE BASE's own bytes.**
///
/// M10-6's accounting golden is the sharpest serialized artefact this
/// unit moved, and neither fixture it goldens (`m10_3_driver_interval`'s
/// `slab` and `sliver_axis`) changed in this PR. So a drive of those two
/// fixtures at `SymbolicDials::off()` must reproduce the merge base's
/// committed golden BYTE FOR BYTE — the tier-off differential claim 1
/// makes, taken against a file this PR did not write rather than against
/// a row it did.
///
/// The three files under `tests/golden_r2/` are
/// `git show d935a96ad23:crates/editor-core/tests/golden/…`, copied
/// verbatim. They are R2's evidence and expire with this review.
#[test]
fn r2_the_tier_off_accounting_is_the_merge_bases_bytes() {
    let eps = format!("{:e}", Tol::witness().eps());
    let base = match eps.as_str() {
        "1e-6" => include_str!("golden_r2/base_m10_6_accounting_1e-6.txt"),
        "1e-9" => include_str!("golden_r2/base_m10_6_accounting_1e-9.txt"),
        "1e-12" => include_str!("golden_r2/base_m10_6_accounting_1e-12.txt"),
        other => panic!("r2 differential has no merge-base golden for eps={other}"),
    };
    let text = |dials: SymbolicDials| {
        let mut s = String::new();
        for (label, doc) in [
            (
                "planted_flip",
                crate::m10_3_driver_interval::slab(
                    20.0 * Tol::witness().eps(),
                    40.0 * Tol::witness().eps(),
                ),
            ),
            (
                "terminal_sliver",
                crate::m10_3_driver_interval::sliver_axis(),
            ),
        ] {
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let verdict = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 4096,
                    symbolic: dials,
                    ..DriveConfig::default()
                },
                Tol::witness(),
            )
            .expect("the nominal builds");
            s.push_str(&format!("== {label}\n"));
            s.push_str(
                &editor_core::report::MassBudget::of(verdict.accounting(), &analyzed).serialize(),
            );
        }
        s
    };
    assert_eq!(
        text(SymbolicDials::off()),
        base,
        "the tier OFF did not reproduce the merge base's accounting bytes at eps={eps}"
    );
    // And the same measurement the other way: with the tier ON the bytes
    // MOVE, which is what the re-blessed golden records.
    assert_ne!(
        text(SymbolicDials::default()),
        base,
        "the tier ON reproduced the merge base's bytes — then the re-bless \
         of tests/golden/m10_6_accounting_{eps}.txt records nothing"
    );
}
