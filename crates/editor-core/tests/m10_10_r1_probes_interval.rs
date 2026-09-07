//! **R1's independent probes of M10-10**, on a document the unit never
//! measured: a CIRCULAR-SEGMENT BOSS — one straight chord closed by one
//! authored-bulge MAJOR arc (`bulge = 2`, so `theta = 4*atan 2`) — with a
//! bore inside it, and the web between the arc wall and the bore wall
//! as the measure.
//!
//! It reaches the arc family by the CHAIN/BULGE route (`ArcTo { Bulge }`,
//! the `SketchSegment::eval` pushforward the mechanism is about) at a
//! bulge that is NOT the circle kernel's literal 1, which no measured
//! document authors: the plate, the annulus and R1's split-bore disc
//! all go through `LoopProgram::Circle`/`CircleSplit`. The question every row answers is the
//! one the unit's acceptance asks: **what does a REAL study get today**,
//! and is the ceiling still a multiple of ε.
//!
//! EVERY ROW IS EVIDENCE-ONLY (`#[ignore]`d, prints, asserts nothing a
//! gate could read — [[test-suite-cost]]). Run:
//!
//! ```sh
//! cargo test -p editor-core --features interval --release --test all -- \
//!   m10_10_r1_probes_interval:: --ignored --nocapture --test-threads 1
//! ```
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)]

use std::time::Instant;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, NamePat, Node, ParamName, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, Selector, SitedRef, SurfaceKindSet,
    UnitSym, select_where,
};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::fixture::Recorder;
use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{bound, dials, nominal_box, render_over_band};

/// The chord half-length's nominal, in metres.
const CHORD_HALF: f64 = 2.0e-3;
/// The bore's nominal radius.
const BORE_R: f64 = 0.3e-3;
/// The authored bulge of the segment's arc — a MAJOR arc, so both
/// junctions with the chord are corners rather than tangencies (the
/// kernel refuses an undeclared tangency, which is how the first cut
/// of this fixture died).
const BULGE: f64 = 2.0;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn plen(n: &str) -> Expr {
    Expr::param(ParamName::new(n), Dimension::Length)
}

/// **R1's circular-segment boss**, as a function of the SCALE of its
/// real study (`scale = 1.0` is the study a user would ask for:
/// `chord_half` uniform ±0.05 mm, `bore_r` normal at σ = 0.01 mm).
/// Returns the document, the web measure and its assertion.
///
/// The outer loop is `at(−c, 0) → line_to(c, 0) → arc_to(bulge 2 →
/// start)`: ONE authored-bulge major arc closing on one straight
/// chord, with `θ = 4·atan(2)` — a turn no other M10 fixture authors
/// (the plate, the annulus and R1's split-bore disc all go through
/// `LoopProgram::Circle`/`CircleSplit`; the pad's arcs are fillets and
/// the bracket's and the link's are its own). Rule D's multiple set on
/// this document is `q = i/2` at `atan(2)` rather than at `atan(1)`,
/// so the closed forms carry a bulge that is not the circle kernel's
/// literal 1 and A0 cannot fold `abs(2)` into the same shape.
///
/// A bore sits inside the segment, off the arc's own centre, so the
/// web is `R − d − bore_r` with `R` and the centre DERIVED from the
/// chord and the bulge through the sagitta closed forms — the two
/// spellings rule D is about.
pub(crate) fn segment_boss(scale: f64, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
        "chord_half",
        CHORD_HALF,
        Distribution::Uniform {
            lo: -5.0e-5 * scale,
            hi: 5.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "bore_r",
        BORE_R,
        Distribution::Normal {
            sigma: 1.0e-5 * scale,
        },
    );

    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));

    let neg_c = Expr::neg(plen("chord_half"));
    let seg_loop = LoopProgram::Chain(vec![
        ProgramStep::At([neg_c, len(0.0)]),
        ProgramStep::LineTo(ProgramTarget::Point([plen("chord_half"), len(0.0)])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Start,
            b: scl(BULGE),
        }),
    ]);
    let seg_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![seg_loop],
    }));
    let thickness = Expr::div(plen("chord_half"), scl(4.0)).expect("Length / Scalar");
    let seg = r.insert(Node::Extrude {
        profile: seg_profile,
        distance: thickness.clone(),
    });
    let bore_centre_y = Expr::mul(plen("chord_half"), scl(0.2)).expect("Length * Scalar");
    let bore_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [len(0.0), bore_centre_y],
            radius: plen("bore_r"),
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
            assert!(!faces.is_empty(), "{node:?} has a cylindrical wall");
            SitedRef::new(node, faces.remove(0))
        };
        vec![wall(seg), wall(bore)]
    };
    let web = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    let measure = r.insert(Node::measure(web, refs).expect("both indices in range"));
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(0.25e-3),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

/// **THE E2E EXERCISE — what a real study gets today on a document
/// nobody tuned for.** The segment boss's real study driven whole with the
/// algebra ON and OFF, with the receipt, the stackup, the assertion and
/// the cost; then the whole-certifying bracket under both rule sets,
/// the over-band SET at its refusing end (ceiling + δ, never a
/// multiple), and the same set at the certifying end for contrast.
#[test]
#[ignore = "evidence-only: R1's own end-to-end circular-segment study"]
fn r1_the_segment_bosss_real_study_end_to_end() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let at = |s: f64| segment_boss(s, tol).0;

    for (label, rules) in [
        ("algebra OFF", SymRules::without_the_algebra()),
        ("shipped    ", SymRules::shipped()),
    ] {
        let (doc, measure, assertion) = segment_boss(1.0, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let t = Instant::now();
        let v = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 256,
                symbolic: dials(rules),
                ..DriveConfig::default()
            },
            tol,
        );
        let dt = t.elapsed().as_secs_f64();
        match v {
            Err(e) => println!("== REAL STUDY {label} {dt:>6.1}s: REFUSED {e}"),
            Ok(v) => {
                println!(
                    "== REAL STUDY {label} {dt:>6.1}s: receipt={:?}\n   decisions={:?}",
                    v.receipt(),
                    v.decisions()
                );
                let stack =
                    editor_core::stackup::stackup(&doc, measure, &analyzed, &v, None, false, tol);
                match &stack {
                    Ok(rep) => println!(
                        "   stackup: worst case {:?} over {} leaves, nominal {:?}",
                        rep.worst_case, rep.worst_case.leaves, rep.nominal
                    ),
                    Err(e) => println!("   stackup: REFUSED {e}"),
                }
                let a = editor_core::drive::assertion_at(
                    &doc,
                    assertion,
                    v.root(),
                    dials(rules),
                    tol,
                );
                println!("   assertion at the root box: {a:?}");
                let (shapes, refusal, _) =
                    replay(&doc, &ParamBox::of(&analyzed), rules, tol);
                println!("   whole-study replay stops at {refusal:?}");
                println!(
                    "{}",
                    render_over_band(&crate::m10_8_harness::over_band_set(&shapes))
                );
            }
        }
    }

    // The whole-certifying bracket and the SET at its refusing end.
    for (label, rules) in [
        ("algebra OFF", SymRules::without_the_algebra()),
        ("shipped    ", SymRules::shipped()),
    ] {
        let t = Instant::now();
        let (lo, hi, set) = bound(&at, rules, tol, 1.0e-11, 1.0e2, 20);
        println!(
            "== CEILING {label}: certifies x{lo:.4e}, refuses x{hi:.4e} \
             [= {:.4e}·eps .. {:.4e}·eps] in {:.1}s",
            lo * 5.0e-5 / eps,
            hi * 5.0e-5 / eps,
            t.elapsed().as_secs_f64()
        );
        match set {
            Some(rows) => println!("{}", render_over_band(&rows)),
            None => println!("   (no finite refusing end)"),
        }
        // For contrast: the set just BELOW the ceiling, which must be
        // empty — a certifying replay has nothing over the band.
        if lo.is_finite() {
            let doc = at(lo);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let (shapes, refusal, counts) =
                replay(&doc, &ParamBox::of(&analyzed), rules, tol);
            let over = shapes
                .iter()
                .filter(|s| {
                    matches!(
                        s.outcome,
                        ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
                    )
                })
                .count();
            println!("   at the CERTIFYING end: {over} over the band, refusal {refusal:?}");
            println!("   counts {counts:?}");
        }
    }
}

/// **The per-predicate split at the segment boss's nominal**, algebra off and
/// on — which of the arc family's identity residuals rule D and the
/// door reach on a document authored through the chain vocabulary
/// rather than through `LoopProgram::Circle`.
#[test]
#[ignore = "evidence-only: R1's circular-segment boss, per predicate at the nominal"]
fn r1_the_segment_bosss_per_predicate_split_at_the_nominal() {
    let tol = Tol::witness();
    let (doc, _, _) = segment_boss(1.0, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let nominal = nominal_box(&analyzed);
    for (label, rules) in [
        ("algebra OFF", SymRules::without_the_algebra()),
        ("shipped    ", SymRules::shipped()),
    ] {
        let t = Instant::now();
        let (shapes, refusal, counts) = replay(&doc, &nominal, rules, tol);
        let mut by: std::collections::BTreeMap<&str, [usize; 4]> = Default::default();
        for s in &shapes {
            let e = by.entry(s.predicate).or_default();
            let slot = match s.outcome {
                ShapeOutcome::Theorem => 0,
                ShapeOutcome::SignGated => 1,
                ShapeOutcome::Registered => 2,
                _ => 3,
            };
            e[slot] += 1;
        }
        println!(
            "== NOMINAL {label} ({:.1}s): refusal {refusal:?}\n   {counts:?}",
            t.elapsed().as_secs_f64()
        );
        for (name, [th, g, reg, num]) in by {
            if th + g + reg + num > 0 && (num > 0 || th > 0 || reg > 0) {
                println!("   {name:<36} {th}/{g}/{reg}/{num}");
            }
        }
    }
}

// ---------------- claim 3: what the plate's Budget refusals really are

/// Halves `b`'s widest varying axis.
fn halve(b: &ParamBox) -> Option<(ParamBox, ParamBox)> {
    let (name, lo, hi) = b
        .varying()
        .max_by(|a, c| (a.2 - a.1).partial_cmp(&(c.2 - c.1)).unwrap())
        .map(|(n, lo, hi)| (n.clone(), lo, hi))?;
    let mid = 0.5 * (lo + hi);
    let mut left = b.axes().clone();
    let mut right = b.axes().clone();
    left.insert(name.clone(), editor_core::analysis::BoxAxis::Varying { lo, hi: mid });
    right.insert(name, editor_core::analysis::BoxAxis::Varying { lo: mid, hi });
    Some((ParamBox::from_axes(left), ParamBox::from_axes(right)))
}

/// **THE ATTACK ON THE ACCEPTANCE.** The unit's headline is that the
/// plate's real study returns 431 certified leaves and 593 refusals ALL
/// of class `Budget`, and that the refusals "sit along the surface
/// where the web crosses the floor … which is the real flip the study
/// contains". `Budget` is the class the driver prices a FRONTIER leaf
/// under when `max_leaves` runs out — it says nothing at all about what
/// that leaf could not decide. This row asks the leaves themselves.
///
/// For a sample of refused leaves it replays each one and prints the
/// OVER-BAND SET (the instrument the unit itself insists on), then
/// bisects the leaf `DEPTH` times along its widest axis and prints the
/// set at the deepest sub-box. A refusal that is the study's real flip
/// shows `assert_bound` and nothing else, at every depth. A refusal
/// that shows an IDENTITY residual — `carrier_matches_mapped_source`,
/// `pcurve_map_residual`, `carrier_on_surface_*` — or a
/// dependency-widened real margin is a bound the headline does not
/// name.
#[test]
#[ignore = "evidence-only: R1 opens the plate's Budget refusals"]
fn r1_what_the_plates_budget_refusals_are_bounded_by() {
    const DEPTH: usize = 4;
    let tol = Tol::witness();
    let rules = SymRules::shipped();
    let leaves: usize = std::env::var("CAD_R1_LEAVES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(256);
    let (doc, _, _) = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let t = Instant::now();
    let v = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: leaves,
            symbolic: dials(rules),
            ..DriveConfig::default()
        },
        tol,
    )
    .expect("the plate builds");
    println!(
        "== plate real study at {leaves} leaves in {:.1}s: {:?}",
        t.elapsed().as_secs_f64(),
        v.receipt()
    );
    let mut classes: std::collections::BTreeMap<String, usize> = Default::default();
    for l in v.refused() {
        *classes.entry(format!("{:?}", l.reason.class())).or_default() += 1;
    }
    println!("   refusals by class: {classes:?}");

    // A spread of refused leaves rather than the first few in order.
    let refused = v.refused();
    let step = (refused.len() / 6).max(1);
    for (i, leaf) in refused.iter().step_by(step).take(6).enumerate() {
        let spans: Vec<String> = leaf
            .box_
            .varying()
            .map(|(n, lo, hi)| format!("{n:?} [{lo:.3e},{hi:.3e}]"))
            .collect();
        println!("\n-- refused leaf {i}: {:?}\n   {}", leaf.reason, spans.join(" "));
        let (shapes, refusal, _) = replay(&doc, &leaf.box_, rules, tol);
        println!("   at the leaf: refusal {refusal:?}");
        println!("{}", render_over_band(&crate::m10_8_harness::over_band_set(&shapes)));
        // Refine: follow the half that still has something over the band.
        let mut b = leaf.box_.clone();
        for d in 1..=DEPTH {
            let Some((lo_b, hi_b)) = halve(&b) else { break };
            let mut next = None;
            for (which, cand) in [("lo", lo_b), ("hi", hi_b)] {
                let (shapes, _, _) = replay(&doc, &cand, rules, tol);
                let set = crate::m10_8_harness::over_band_set(&shapes);
                if !set.is_empty() {
                    println!("   depth {d} ({which}):");
                    println!("{}", render_over_band(&set));
                    if next.is_none() {
                        next = Some(cand);
                    }
                }
            }
            match next {
                Some(n) => b = n,
                None => {
                    println!("   depth {d}: both halves clean — the leaf was budget alone");
                    break;
                }
            }
        }
    }
}
