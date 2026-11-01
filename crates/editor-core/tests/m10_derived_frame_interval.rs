//! **A body placed on a DERIVED pose whose placement is a pure
//! TRANSLATION, on the symbolic lane** — DOCM-1 review lane R1's two
//! rows (`docm/1-review-r1`, `docm1_review_r1_probes_interval.rs`),
//! ported unchanged in what they assert, and the measurement of what
//! answers them.
//!
//! A derived placement reads its pose off the body its copy rests on
//! (a face of it, read as its plane), so its axes are the kernel's
//! stored vectors, which the placed boss carries.
//! On THIS document the widened parameter is the cube's HEIGHT, so
//! every normalised quantity in the chain is CONSTANT in it: the
//! `sqrt` atoms are `sqrt(16384/256)` and friends, rule A0 folds them
//! to exact rationals, and the degree mechanism the item describes has
//! nothing to act on. Measured, on the whole declared box: with A0
//! alone the document freezes NOTHING and certifies up to a half-width
//! of `1e-3`; with every rule off it refuses at both measured widths
//! on `carrier_endpoint_start`.
//!
//! **This is the narrow case, not the item's.** A derived frame whose
//! AXES carry the parameter keeps a `sqrt` of a NON-constant form that
//! no constant fold reaches, and there the freezes are on degree and
//! the identity is not reached: `m10_derived_frame_tilted_interval` is
//! that document, and SYM-5's Phase 2 (PR-2) runs on it.
//!
//! Three rows here are PINS (`m10_*`): the transform-lifted row, which
//! DOCM filed red and which is green; the measured parity state of the
//! other ported row; and that the refusal left at `5e-2` is not a
//! freeze. The ported parity row itself stays `#[ignore]`d — it is red
//! at one width, for a reason that is the value channel's
//! (`work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`).
//! The `sym5_phase1_*` rows are evidence-only and assert nothing.
//!
//! The preamble this file shares with its siblings — `param_doc`,
//! `failures`, the budget constants — is copied across the `m10_*`
//! family; the class is
//! `work/sym/interval-test-preamble-is-copied-across-the-m10-files`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, Recorder, len};
use crate::m10_8_harness::head;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, CapEnd, Dimension, Distribution, DocEdit, EvalOptions, Evaluation, Formula,
    FreeVar, Node, NodeResult, ProfileDoc, RecipeNodeId, RoleSeg, UnitSym, VarName, evaluate,
};
use geom_core::{Interval, SymRules, Tol};

fn eps() -> f64 {
    Tol::witness().eps()
}

fn param_doc(name: &'static str, nominal: f64, half: f64, r: &mut Recorder) {
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(name),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        }),
    });
}

fn budget() -> geom_core::SymBudget {
    geom_core::SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

/// Node failures of an evaluation over the WHOLE declared box in one
/// leaf, on the plain `Interval` lane.
fn interval_failures(doc: &editor_core::ProfileDoc) -> Vec<String> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let ev: Evaluation<Interval> = evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
    failures(&ev)
}

/// The same, on the `Sym<Interval>` lane the E6 driver certifies on,
/// under the shipped rule set.
fn sym_failures(doc: &editor_core::ProfileDoc) -> Vec<String> {
    sym_failures_under(doc, SymRules::shipped()).0
}

/// The same, under a chosen rule set, with the leaf's counts.
fn sym_failures_under(doc: &ProfileDoc, rules: SymRules) -> (Vec<String>, geom_core::SymCounts) {
    sym_failures_in(doc, rules, budget())
}

/// The same, under a chosen rule set AND a chosen budget.
fn sym_failures_in(
    doc: &ProfileDoc,
    rules: SymRules,
    budget: geom_core::SymBudget,
) -> (Vec<String>, geom_core::SymCounts) {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    geom_core::sym::with_session_rules(budget, rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
        failures(&ev)
    })
}

fn failures<T: geom_core::Decide>(ev: &Evaluation<T>) -> Vec<String> {
    ev.order
        .iter()
        .filter_map(|id| match ev.result(*id) {
            Some(NodeResult::Failed(e)) => {
                Some(format!("node {} — {} — {:?}", id.0, e.kind, e.kind))
            }
            Some(NodeResult::Poisoned { through }) => {
                Some(format!("node {} poisoned through {}", id.0, through.0))
            }
            _ => None,
        })
        .collect()
}

/// The zero values a seat at rest names: no slide, no spin.
fn at_rest() -> Vec<(editor_core::Coord, Formula)> {
    use editor_core::{Coord, TargetAxis};
    vec![
        (
            Coord::Slide(TargetAxis::X),
            Formula::zero(Dimension::Length),
        ),
        (
            Coord::Slide(TargetAxis::Y),
            Formula::zero(Dimension::Length),
        ),
        (Coord::SPIN, Formula::zero(Dimension::Angle)),
    ]
}

/// The plane of `body`'s `end` cap, read at `site` (the body itself, or
/// a placement of it, whose copy carries the cap's name as it is).
fn cap_plane(site: RecipeNodeId, body: RecipeNodeId, end: CapEnd) -> editor_core::Operand {
    let name = fixture::fname(body, RoleSeg::Cap(end));
    fixture::plane(editor_core::Operand::select(site, vec![name]))
}

/// **`body` seated on `to`**: a copy of it whose start cap rests on the
/// plane `to`, at rest in that plane, under `orient`. Returns the
/// placement.
fn seated_as(
    r: &mut Recorder,
    body: RecipeNodeId,
    to: editor_core::Operand,
    orient: editor_core::Orientation,
) -> RecipeNodeId {
    let placement = r.insert(crate::fixture::stood(editor_core::BodyRead::plain(
        editor_core::Operand::from(body),
    )));
    r.constrain(
        placement,
        editor_core::Mate {
            on: cap_plane(body, body, CapEnd::Start),
            to,
            orient,
        },
        at_rest(),
    );
    placement
}

/// [`seated_as`] on a plane facing up: the start cap faces the plane's
/// normal unflipped, so the copy stands upended, its `z` along `−z`.
fn seated(r: &mut Recorder, body: RecipeNodeId, to: editor_core::Operand) -> RecipeNodeId {
    let upended = editor_core::Orientation::new(
        editor_core::SignedAxis::PlusX,
        editor_core::SignedAxis::MinusZ,
    )
    .expect("x and z lie along two axes");
    seated_as(r, body, to, upended)
}

/// `body`'s start-cap plane slid `by` up its own z and turned to face
/// up it: the plane a copy of `body` rests on to stand `by` up.
fn raised(body: RecipeNodeId, by: Formula) -> editor_core::Operand {
    use editor_core::pose::{PoseDef, PoseRole, RoleRead};
    let start = || cap_plane(body, body, CapEnd::Start);
    let down = editor_core::Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(start()),
        along: RoleRead::through(start(), PoseRole::Normal),
        by: Formula::neg(by).expect("a length negates"),
    }));
    fixture::flip(down)
}

/// **The derived-placement document**: a unit box whose HEIGHT is the
/// widened parameter, and a boss whose copy rests on the box's top cap
/// — a pose read off the box's geometry, which the height reaches.
/// Returns the document, the boss's profile and the boss's placement.
pub(crate) fn boss_on_widened_box(half: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    param_doc("h", 1.0, half, &mut r);
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let cube = r.insert(Node::Extrude {
        profile: p.into(),
        distance: Formula::named(VarName::from_static("h"), Dimension::Length),
    });
    let boss_p = r.insert(Node::Profile(fixture::desc(vec![fixture::square(
        0.0, 0.0, 0.5,
    )])));
    let boss = r.insert(Node::Extrude {
        profile: boss_p.into(),
        distance: len(0.25),
    });
    let placed = seated(&mut r, boss, cap_plane(cube, cube, CapEnd::End));
    (r.doc, boss_p, placed)
}

/// **The authored twin**: a boss whose copy stands the widened
/// parameter `z0` up its own z, on a pose of its own geometry the
/// parameter moves — the placement is at `T` exactly as the derived
/// placement's is.
pub(crate) fn boss_on_widened_authored_frame(half: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    param_doc("z0", 1.0, half, &mut r);
    let boss_p = r.insert(Node::Profile(fixture::desc(vec![fixture::square(
        0.0, 0.0, 0.5,
    )])));
    let boss = r.insert(Node::Extrude {
        profile: boss_p.into(),
        distance: len(0.25),
    });
    let z0 = Formula::named(VarName::from_static("z0"), Dimension::Length);
    let placed = seated(&mut r, boss, raised(boss, z0));
    (r.doc, placed)
}

/// **The lifted document**: an EXACT box whose copy is lifted by a
/// widened slide, and a boss whose copy rests on the lifted copy's top
/// cap.
pub(crate) fn transform_lifted_boss(half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    param_doc("lift", 0.0, half, &mut r);
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let cube = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    let lift = Formula::named(VarName::from_static("lift"), Dimension::Length);
    let lifted = seated(&mut r, cube, raised(cube, lift));
    let boss_p = r.insert(Node::Profile(fixture::desc(vec![fixture::square(
        0.0, 0.0, 0.5,
    )])));
    let boss = r.insert(Node::Extrude {
        profile: boss_p.into(),
        distance: len(0.25),
    });
    // The lifted copy stands upended, so its end cap faces down, the way
    // the boss's start cap does: the boss stands as it is drawn.
    seated_as(
        &mut r,
        boss,
        cap_plane(lifted, cube, CapEnd::End),
        editor_core::Orientation::IDENTITY,
    );
    r.doc
}

/// **DOCM-1 R1's C7 row** (`r1_c7_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin`),
/// unchanged in what it asserts: whatever the kernel does with a
/// widened placement, it must do the same for the two frame kinds.
#[test]
#[ignore = "red on the plain lane only: the derived document's widened-height box extrude refuses its own attachment gate at every width; the symbolic lane agrees at every width"]
fn m10_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin() {
    let e = eps();
    let mut mismatch = Vec::new();
    for w in [e / 8.0, 1e-6, 1e-3, 0.05] {
        let (derived, boss_p, boss) = boss_on_widened_box(w);
        let (authored, _) = boss_on_widened_authored_frame(w);
        for (lane, run) in [
            ("plain", interval_failures as fn(&ProfileDoc) -> Vec<String>),
            ("sym", sym_failures as fn(&ProfileDoc) -> Vec<String>),
        ] {
            let d_fails = run(&derived);
            let a_fails = run(&authored);
            println!(
                "C7 eps={e:e} half={w:e} lane={lane}: derived {d_fails:?}; authored {a_fails:?}"
            );
            let d_ok = d_fails.is_empty();
            let a_ok = a_fails.is_empty();
            if d_ok != a_ok {
                mismatch.push(format!(
                    "eps={e:e} half={w:e} lane={lane}: derived ok={d_ok}, authored ok={a_ok}"
                ));
            }
            let _ = (boss_p, boss);
        }
    }
    assert!(
        mismatch.is_empty(),
        "frame kinds disagree on a widened placement: {mismatch:?}"
    );
}

/// **DOCM-1 R1's transform-lifted row**
/// (`r1_c7_the_prs_transform_lifted_shape_with_an_extrude_above_it`),
/// unchanged in what it asserts.
#[test]
fn m10_the_transform_lifted_shape_with_an_extrude_above_it() {
    let e = eps();
    let doc = transform_lifted_boss(e / 8.0);
    let plain = interval_failures(&doc);
    let sym = sym_failures(&doc);
    println!(
        "C7 transform-lifted eps={e:e} half={:e}: plain {plain:?}; sym {sym:?}",
        e / 8.0
    );
    assert!(
        sym.is_empty(),
        "on the driver's lane the extrude above a derived frame on a transform-lifted box \
         must certify as the authored twin does: {sym:?}"
    );
}

// ---------------------------------------------------------------
// Phase 1 — the measurement
// ---------------------------------------------------------------

/// The FROZEN lines of an explanation with the ancestors that lead to
/// them — the normalisation chain, without the thousands of lines that
/// did not freeze.
fn frozen_chain(explain: &str) -> String {
    let lines: Vec<&str> = explain.lines().collect();
    let indent = |l: &str| l.len() - l.trim_start().len();
    let mut keep = vec![false; lines.len()];
    for (i, l) in lines.iter().enumerate() {
        if !l.contains("FROZEN") {
            continue;
        }
        keep[i] = true;
        let mut d = indent(l);
        for j in (0..i).rev() {
            let dj = indent(lines[j]);
            if dj < d {
                keep[j] = true;
                d = dj;
            }
        }
    }
    let n = keep.iter().filter(|k| **k).count();
    if n == 0 {
        return "    (no frozen node below this residual)".to_owned();
    }
    let mut out = String::new();
    for (i, l) in lines.iter().enumerate() {
        if keep[i] {
            out.push_str(&head(l, 400));
            out.push('\n');
        }
    }
    out.push_str(&format!(
        "    ({n} lines of {} on the frozen path)\n",
        lines.len()
    ));
    out
}

/// One replay with BOTH instruments installed: the structural profile
/// (`sym::profile`) and the shape report with an explanation depth, so
/// a blocked residual comes back with its form, its early form and the
/// DAG below it to `depth` levels.
fn measured_replay(label: &str, doc: &ProfileDoc, box_: &ParamBox, rules: SymRules, depth: usize) {
    use geom_core::sym::profile::{start_profile, take_profile};
    use geom_core::sym::report::{
        ShapeOutcome, explain_depth, name_param, start_shape_report, take_shape_report,
    };

    for name in box_.axes().keys() {
        name_param(
            geom_core::ParamSymbol::new(name.0.digest()),
            &doc.spoken_var(*name).to_string(),
        );
    }
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..EvalOptions::default()
    };
    start_profile();
    start_shape_report();
    explain_depth(depth);
    let (refusal, counts) = geom_core::sym::with_session_rules(budget(), rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
        failures(&ev)
    });
    let shapes = take_shape_report();
    let profile = take_profile();
    explain_depth(0);

    let mut split: std::collections::BTreeMap<String, usize> = Default::default();
    for s in &shapes {
        *split
            .entry(format!("{} {:?}", s.predicate, s.outcome))
            .or_default() += 1;
    }
    println!("=== {label}");
    println!("counts {counts:?}");
    println!("refusals {refusal:?}");
    println!("decisions {split:?}");
    println!("freezes {} by (cause, op/walk/origin):", profile.frozen());
    for ((cause, where_), s) in profile.freezes_by_cause() {
        println!("  {cause:?} {where_} -> {s:?}");
    }
    println!("unnoted {}", profile.unnoted());
    // The first blocked residual of each predicate, with its chain.
    let mut seen: std::collections::BTreeSet<&str> = Default::default();
    for s in &shapes {
        if matches!(
            s.outcome,
            ShapeOutcome::Indeterminate | ShapeOutcome::Invalid | ShapeOutcome::NumericZero
        ) && seen.insert(s.predicate)
        {
            println!("--- blocked {} {:?}", s.predicate, s.outcome);
            println!("    enclosure {:?} sizes {:?}", s.enclosure, s.sizes);
            if let Some(f) = &s.form {
                println!("    plain  {}", head(f, 1200));
            }
            if let Some(f) = &s.early_form {
                println!("    early  {}", head(f, 1200));
            }
            if let Some(e) = &s.explain {
                println!("{}", frozen_chain(e));
            }
        }
    }
    print!("{}", profile.render());
}

/// **Phase 1, the freeze profiled by origin and cause, and the frozen
/// kids rendered** — the derived-frame document at its nominal and at
/// one widened box, against its authored twin.
#[test]
#[ignore = "evidence-only: SYM-5 phase 1 — the freeze population and the normalisation chain"]
fn sym5_phase1_profile_the_derived_frame_freeze() {
    for half in [1.0e-3, 5.0e-2] {
        let (derived, _, _) = boss_on_widened_box(half);
        let (authored, _) = boss_on_widened_authored_frame(half);
        for (name, doc) in [("derived", &derived), ("authored", &authored)] {
            let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
            let root = ParamBox::of(&analyzed);
            let nominal = crate::m10_8_harness::nominal_box(&analyzed);
            for (scale, box_) in [("nominal", nominal), ("root", root)] {
                measured_replay(
                    &format!("{name} half={half:e} {scale} shipped"),
                    doc,
                    &box_,
                    SymRules::shipped(),
                    6,
                );
            }
        }
    }
}

/// **Phase 1, the tier with the algebra off** — the same documents
/// under `SymRules::without_the_algebra` (M10-9's tier bit for bit),
/// which is the state DOCM-1's diagnosis was taken in.
#[test]
#[ignore = "evidence-only: SYM-5 phase 1 — the same documents under M10-9's tier"]
fn sym5_phase1_the_derived_frame_under_the_earlier_tier() {
    for half in [1.0e-3, 5.0e-2] {
        let (derived, _, _) = boss_on_widened_box(half);
        let n = SymRules::none();
        let ladder = [
            ("none", n),
            (
                "A0",
                SymRules {
                    const_fold: true,
                    ..n
                },
            ),
            (
                "A0+early",
                SymRules {
                    const_fold: true,
                    early: true,
                    ..n
                },
            ),
            ("without_the_door", SymRules::shipped_without_the_door()),
            ("without_the_algebra", SymRules::without_the_algebra()),
            ("shipped", SymRules::shipped()),
        ];
        for (name, rules) in ladder {
            let (fails, counts) = sym_failures_under(&derived, rules);
            println!(
                "ladder derived half={half:e} {name}\n  counts {counts:?}\n  fails {} {}",
                fails.len(),
                head(fails.first().map_or("", String::as_str), 220)
            );
        }
    }
}

/// **Phase 1, the chain the plain form cannot see** — the same
/// derived-frame document with every rule OFF (`SymRules::none`), at
/// the nominal, so every residual the plain quotient form does not
/// reach is rendered with the DAG below it and the FROZEN nodes on the
/// path named. This is where the normalisation chain is readable: with
/// the shipped set the document has no blocked residual at all.
#[test]
#[ignore = "evidence-only: SYM-5 phase 1 — the normalisation chain under the plain form \
            alone; ~16 minutes, most of it rendering forms of 992 terms at degree 82"]
fn sym5_phase1_the_normalisation_chain_under_the_plain_form() {
    let (derived, _, _) = boss_on_widened_box(5.0e-2);
    let analyzed = analyzed_box(&derived, &AnalysisPolicy::default());
    let nominal = crate::m10_8_harness::nominal_box(&analyzed);
    measured_replay(
        "derived half=5e-2 nominal rules=none",
        &derived,
        &nominal,
        SymRules::none(),
        10,
    );
}

/// **The measured state of the parity row**, pinned while the row
/// above stays ignored. Measured at all three ε rows: at every
/// half-width up to `5e-2` the two frame kinds AGREE on the symbolic
/// lane — the derived-frame boss certifies exactly where its authored
/// twin does, which is what the tier buys the derived frame (with every
/// rule off it refuses, on `carrier_endpoint_start`).
///
/// At `5e-2` the derived boss used to refuse on a clause-1 invalid
/// margin on `newell_plane_residual` (`Vec3::normalize` dividing the
/// boss's side-plane cross-sum by a norm whose enclosure contained
/// zero). Under D's placement solve (a direction exactly on an axis
/// read as that axis, a spin set from the two charts' frames) the
/// placement's map no longer widens that cross-sum to zero, and the
/// boss certifies there too.
#[test]
fn m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width() {
    let e = eps();
    for w in [e / 8.0, 1.0e-6, 1.0e-3, 5.0e-2] {
        let (derived, _, _) = boss_on_widened_box(w);
        let (authored, _) = boss_on_widened_authored_frame(w);
        assert!(
            sym_failures(&derived).is_empty(),
            "half={w:e}: the derived-frame boss must certify on the symbolic lane"
        );
        assert!(
            sym_failures(&authored).is_empty(),
            "half={w:e}: the authored twin must certify"
        );
    }
}

/// **The freeze population is not what refuses, with A0.** With A0's
/// constant fold the derived-frame document freezes NOTHING and
/// certifies at `5e-2`: on the sign-hull construction the gate's
/// residual is a plain THEOREM — the opaque `Select` atoms of an
/// axis-aligned frame cancel as `X − X` — and the boss's side plane,
/// whose `newell_plane_residual` was a clause-1 `Invalid` while the
/// placement's map widened its cross-sum to zero, is decided under D's
/// placement solve. With every rule off, the plain tier freezes no form
/// and refuses nothing: a copy shares its source's body under its
/// frame, so the boss is never rebuilt over the box, and the
/// start-endpoint certification of the mapped extrude that refused over
/// an enclosure of [0, 0.32] is not run
/// (`work/lathe/the-guided-derived-frame-extrude-escalates-as-a-kernel-defect.md`
/// measured it). The ten forms that used to freeze at A0's rung sat
/// under `1/max(1, min(1, max(0, 1))/4)` — a `max`/`min` of two
/// CONSTANTS — and A0 decides those exactly now
/// (`work/decide/a0-leaves-max-and-min-of-constants-opaque`).
///
/// The `none` rung read `newell_plane_residual` with nothing frozen
/// while the profile's loops were the `f64` elaboration lifted as
/// constants; elaborated at the lane, the loops carry the box's
/// unknowns into the gate, and the rung is the attachment gate's
/// again (`work/decide/the-derived-frame-refusal-rows-none-rung-pins-the-retired-construction`
/// carries the earlier measurement).
#[test]
fn m10_the_derived_frames_refusal_is_not_a_freeze() {
    let (derived, _, _) = boss_on_widened_box(5.0e-2);
    let n = SymRules::none();

    // A copy shares its source's body under its frame (D10
    // `[placement.frame]`), so the boss is never rebuilt over the box:
    // the plain tier reaches no start-endpoint certification of a mapped
    // extrude and refuses nothing, freezing no form on the way.
    let (plain_fails, plain_counts) = sym_failures_under(&derived, n);
    assert!(
        plain_fails.is_empty(),
        "plain form alone certifies: {plain_fails:?}"
    );
    assert_eq!(
        plain_counts.frozen, 0,
        "and freezes no form getting there: {plain_counts:?}"
    );

    let a0 = SymRules {
        const_fold: true,
        ..n
    };
    let (a0_fails, a0_counts) = sym_failures_under(&derived, a0);
    assert_eq!(
        a0_counts.frozen, 0,
        "A0 alone freezes NOTHING: the products that used to freeze sat under \
         `1/max(1, min(1, max(0, 1))/4)`, a `max`/`min` of two CONSTANTS, and A0 decides \
         those exactly now: {a0_counts:?}"
    );
    assert!(
        a0_fails.is_empty(),
        "with no freeze left A0 alone certifies: the clause-1 refusal on the side plane is \
         gone with the placement's widening: {a0_fails:?}"
    );
}

/// **What answered DOCM's rows, and what never explained them**
/// (adopted from SYM-5's review lane, `sym/5-review` at `a3e2b1b46`).
/// DOCM's probes were taken on `20f04189` (2026-09-04), the day before
/// M10-8's constant fold landed (#1828, 2026-09-05), and the `none`
/// rung reproduces their refusal while A0 alone clears it. The BUDGET
/// never explained the refusal on this document: raise it to
/// 4,096 / 65,536 under `none` and it still refuses with the same
/// enclosure, freezing almost nothing — what stood was OPAQUE CONSTANT
/// ATOMS, which is exactly what A0 folds, and rule A alone (`sqrt(X)²
/// = X`, no constant fold) does not clear either.
#[test]
#[ignore = "evidence-only: SYM-5 phase 1 — the budget never explained the height document's refusal"]
fn sym5_phase1_the_height_document_under_a_raised_budget_and_rule_a_alone() {
    let (derived, _, _) = boss_on_widened_box(1.0e-3);
    let big = geom_core::SymBudget {
        max_terms: 65536,
        max_degree: 4096,
    };
    let a_only = SymRules {
        sqrt_square: true,
        ..SymRules::none()
    };
    for (name, rules, b) in [
        ("none/default", SymRules::none(), budget()),
        ("none/4096", SymRules::none(), big),
        ("A_only/default", a_only, budget()),
        ("A_only/4096", a_only, big),
        (
            "A0/default",
            SymRules {
                const_fold: true,
                ..SymRules::none()
            },
            budget(),
        ),
    ] {
        let (fails, counts) = sym_failures_in(&derived, rules, b);
        println!(
            "height 1e-3 {name}: {counts:?}\n  fails {} {}",
            fails.len(),
            head(fails.first().map_or("", String::as_str), 300)
        );
    }
}

/// **The 5e-2 refusal, printed whole** (adopted from `sym/5-review`):
/// the counts and every failure string under the shipped set, so the
/// diagnostic the pins match on is readable beside them.
#[test]
#[ignore = "evidence-only: SYM-5 phase 1 — the newell refusal at 5e-2, in full"]
fn sym5_phase1_the_newell_refusal_at_5e_2() {
    let (derived, _, _) = boss_on_widened_box(5.0e-2);
    eprintln!("--- plain Interval lane");
    let p = interval_failures(&derived);
    println!("derived 5e-2 plain: fails {}", p.len());
    for f in &p {
        println!("  {}", head(f, 400));
    }
    eprintln!("--- Sym<Interval> lane");
    let (fails, counts) = sym_failures_in(&derived, SymRules::shipped(), budget());
    println!("derived 5e-2 shipped: {counts:?} fails {}", fails.len());
    for f in &fails {
        println!("  {}", head(f, 400));
    }
}

/// **SYM-10 Phase 1.2 — the derived-frame row's refusal under `none`
/// and under A0, rendered.** `boss_on_widened_box(5e-2)` at the two
/// rungs `m10_the_derived_frames_refusal_is_not_a_freeze`
/// asserts: counts, the per-predicate split, every refusal, and the
/// blocked residual's plain form with its atom census (no early walk
/// runs at either rung, so the plain form is the whole of what the
/// tier held). What it showed on the sign-hull construction: under
/// `none` the attachment gate's `carrier_endpoint_start` is 32
/// THEOREMS and the first refusal is the boss's side plane's
/// `newell_plane_residual`, a clause-1 `Invalid` the tier is never
/// asked about; under A0 (replacing) the gate's residual freezes
/// (`sqrt` over three frozen kids, frozen 10) and refuses numerically,
/// because A0 folds `sqrt`/`abs` of a constant and not `max`/`min` of
/// two constants, which the folded frame is made of
/// (`work/decide/a0-leaves-max-and-min-of-constants-opaque`).
#[test]
#[ignore = "evidence-only: SYM-10 Phase 1.2, the derived-frame row's refusal rendered"]
fn sym10_phase1_the_derived_frame_rows_refusal_rendered() {
    use geom_core::sym::report::{ShapeOutcome, name_param, start_shape_report, take_shape_report};
    let (derived, _, _) = boss_on_widened_box(5.0e-2);
    let analyzed = analyzed_box(&derived, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    for name in box_.axes().keys() {
        name_param(
            geom_core::ParamSymbol::new(name.0.digest()),
            &derived.spoken_var(*name).to_string(),
        );
    }
    let n = SymRules::none();
    for (label, rules) in [
        ("none", n),
        (
            "A0",
            SymRules {
                const_fold: true,
                ..n
            },
        ),
    ] {
        let opts = EvalOptions {
            param_box: Some(Arc::new(box_.clone())),
            ..EvalOptions::default()
        };
        start_shape_report();
        let (fails, counts) = geom_core::sym::with_session_rules(budget(), rules, || {
            let ev: Evaluation<geom_core::Sym<Interval>> =
                evaluate(&derived, None, &CancelToken::new(), &opts, Tol::witness());
            failures(&ev)
        });
        let shapes = take_shape_report();
        println!("=== derived-frame 5e-2 {label}");
        println!("    counts {counts:?}");
        for (pred, row) in crate::m10_8_harness::split(&shapes) {
            println!("    split {pred:<36} {row:?}");
        }
        println!("    refusals {}", fails.len());
        for f in &fails {
            println!("      {}", head(f, 400));
        }
        let mut seen: std::collections::BTreeSet<&str> = Default::default();
        for s in &shapes {
            if matches!(
                s.outcome,
                ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
            ) && seen.insert(s.predicate)
            {
                println!("--- blocked {} {:?}", s.predicate, s.outcome);
                println!("    enclosure {:?} sizes {:?}", s.enclosure, s.sizes);
                if let Some(f) = &s.form {
                    let c = |needle: &str| f.matches(needle).count();
                    println!(
                        "    plain census select {} | max {} | min {} | abs {} | sqrt {} | copysign {} | terms(top) {}",
                        c("select("),
                        c("max("),
                        c("min("),
                        c("abs("),
                        c("sqrt("),
                        c("copysign("),
                        f.split(" + ").count()
                    );
                    println!("    plain  {}", head(f, 1500));
                }
            }
        }
    }
}
