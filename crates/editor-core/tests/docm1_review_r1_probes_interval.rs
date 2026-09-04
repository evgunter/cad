//! DOCM-1 review lane R1 — the Interval-lane probes on `20f04189`.
//!
//! Lane emphasis (ii): the PR says an interval extrude's certification
//! "refuses a height bracket of ANY width at every ε row", which is why
//! its widening row lifts an exact box by a transform. The driver
//! suite (`m10_3_driver_interval.rs`) certifies a ±0.05 slab WHOLE —
//! through `Sym<Interval>`. These rows measure the fact on both lanes,
//! plain `Interval` and `Sym<Interval>`, at the row's ε, and then ask
//! the question the PR's fixture stopped short of: does an EXTRUDE of
//! a profile on a widened DERIVED frame certify, and does the
//! authored-frame twin under `Guided`?

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, Recorder, ang, len, scl};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::SymbolicDials;
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, Node, NodeResult, ParamName, ProfileDoc, ProfileLift, RecipeNodeId,
    RoleSeg, UnitSym, evaluate,
};
use geom_core::{Interval, Tol};

fn eps() -> f64 {
    Tol::witness().eps()
}

fn param_doc(name: &str, nominal: f64, half: f64, r: &mut Recorder) {
    r.push(DocEdit::SetDocParam {
        name: ParamName::new(name),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        },
    });
}

/// Node failures of an evaluation over the WHOLE declared box in one
/// leaf, on the plain `Interval` lane.
fn interval_failures(doc: &ProfileDoc, lift: ProfileLift) -> Vec<String> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        profile_lift: lift,
        ..EvalOptions::default()
    };
    let ev: Evaluation<Interval> =
        evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
    failures(&ev)
}

/// The same, on the `Sym<Interval>` lane the E6 driver certifies on.
fn sym_failures(doc: &ProfileDoc, lift: ProfileLift) -> Vec<String> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        profile_lift: lift,
        ..EvalOptions::default()
    };
    let dials = SymbolicDials::default();
    let budget = geom_core::SymBudget {
        max_terms: dials.max_terms,
        max_degree: dials.max_degree,
    };
    let (out, _) = geom_core::sym::with_session(budget, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
        failures(&ev)
    });
    out
}

fn failures<T: geom_core::Decide>(ev: &Evaluation<T>) -> Vec<String> {
    ev.order
        .iter()
        .filter_map(|id| match ev.result(*id) {
            Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
            Some(NodeResult::Poisoned { through }) => {
                Some(format!("node {} poisoned through {}", id.0, through.0))
            }
            _ => None,
        })
        .collect()
}

/// A unit box whose HEIGHT is the widened parameter — the PR's
/// "kernel fact" fixture.
fn slab(half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    param_doc("depth", 1.0, half, &mut r);
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    r.insert(Node::Extrude {
        profile: p,
        distance: Expr::param(ParamName::new("depth"), Dimension::Length),
    });
    r.doc
}

/// **(ii) The kernel fact, measured.** Plain `Interval` and
/// `Sym<Interval>`, several widths, this ε row. Printed in full; the
/// assertion is the PR's own sentence: on the lane the driver
/// certifies on, a width of ε/8 must NOT refuse.
#[test]
fn r1_ii_a_widened_extrude_height_measured_on_both_lanes() {
    let e = eps();
    let widths = [e / 32.0, e / 8.0, 1e-6, 1e-3, 0.05];
    let mut sym_ok_at_eps8 = false;
    for w in widths {
        let doc = slab(w);
        let plain = interval_failures(&doc, ProfileLift::Pinned);
        let sym = sym_failures(&doc, ProfileLift::Pinned);
        println!(
            "R1 (ii) eps={e:e} half={w:e}: plain Interval -> {} failure(s) {plain:?}; Sym<Interval> -> {} failure(s) {sym:?}",
            plain.len(),
            sym.len()
        );
        if w == e / 8.0 {
            sym_ok_at_eps8 = sym.is_empty();
        }
    }
    assert!(
        sym_ok_at_eps8,
        "a height bracket of ±ε/8 refuses on the Sym<Interval> lane too — the PR's 'any width' \
         sentence would then be true on the lane that matters"
    );
}

/// The PR's ORIGINAL fixture shape: the box's height is widened, a
/// derived frame reads the box's top, a profile on it, and — the node
/// the PR's row dropped — an EXTRUDE of that profile.
fn boss_on_widened_box(half: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    param_doc("h", 1.0, half, &mut r);
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: Expr::param(ParamName::new("h"), Dimension::Length),
    });
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: cube,
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::Top)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    (r.doc, boss_p, boss)
}

/// The authored twin: a frame whose ORIGIN z is the widened parameter,
/// a profile on it, an extrude — under `Guided`, the placement is at
/// `T` exactly as a derived frame's is under every lift.
fn boss_on_widened_authored_frame(half: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    param_doc("z0", 1.0, half, &mut r);
    let frame = r.insert(Node::Datum(Datum::Frame {
        origin: [
            len(0.0),
            len(0.0),
            Expr::param(ParamName::new("z0"), Dimension::Length),
        ],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    (r.doc, boss)
}

/// **C7, the row the PR stopped short of.** A widened placement into
/// an extrude: derived frame (every lift) vs authored frame (Guided),
/// on both lanes, at this ε. The assertion is parity: whatever the
/// kernel does with a widened placement, it must do the same for the
/// two frame kinds — a derived frame that certifies WORSE than an
/// authored one under Guided is DM1c's own defect.
#[test]
fn r1_c7_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin() {
    let e = eps();
    let mut mismatch = Vec::new();
    for w in [e / 8.0, 1e-6, 1e-3, 0.05] {
        let (derived, boss_p, boss) = boss_on_widened_box(w);
        let (authored, _) = boss_on_widened_authored_frame(w);
        for (lane, run) in [
            ("plain", interval_failures as fn(&ProfileDoc, ProfileLift) -> Vec<String>),
            ("sym", sym_failures as fn(&ProfileDoc, ProfileLift) -> Vec<String>),
        ] {
            let d_pinned = run(&derived, ProfileLift::Pinned);
            let d_guided = run(&derived, ProfileLift::Guided);
            let a_guided = run(&authored, ProfileLift::Guided);
            println!(
                "R1 C7 eps={e:e} half={w:e} lane={lane}: derived/Pinned {d_pinned:?}; derived/Guided {d_guided:?}; authored/Guided {a_guided:?}"
            );
            let d_ok = d_pinned.is_empty();
            let a_ok = a_guided.is_empty();
            if d_ok != a_ok {
                mismatch.push(format!(
                    "eps={e:e} half={w:e} lane={lane}: derived ok={d_ok}, authored/Guided ok={a_ok}"
                ));
            }
            let _ = (boss_p, boss);
        }
    }
    assert!(mismatch.is_empty(), "frame kinds disagree on a widened placement: {mismatch:?}");
}

/// **C7 — is `DerivedFrameSection` decided by type?** `Sym<Interval>`
/// with an EXACT document (the box carries no parameter) still refuses
/// the section; `Interval` likewise — whatever the numbers.
#[test]
fn r1_c7_the_section_refusal_is_by_type_on_an_exact_document() {
    let (doc, loft) = crate::docm1_face_frame::lofted_on_face_frame();
    for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
        let plain = interval_failures(&doc, lift);
        let sym = sym_failures(&doc, lift);
        println!("R1 C7 section eps={:e} {lift:?}: plain {plain:?}; sym {sym:?}", eps());
        for f in [&plain, &sym] {
            assert!(
                f.iter().any(|s| s.starts_with(&format!("node {} — ", loft.0))
                    && s.contains("derived frame")),
                "{lift:?}: the loft refuses DerivedFrameSection: {f:?}"
            );
            assert_eq!(f.len(), 1, "{lift:?}: only the loft refuses: {f:?}");
        }
    }
}

/// **The PR's own shape, one node further.** An EXACT box lifted by a
/// widened rigid transform (the PR's `boxed_on_param`), a derived
/// frame on the lifted body's top cap, a profile, and the EXTRUDE the
/// PR's row leaves out. Printed on both lanes; the assertion is the
/// PR's own sentence read on the driver's lane: on `Sym<Interval>` an
/// extrude of a profile whose placement carries width certifies for
/// an AUTHORED frame (the row above), so if it refuses here the
/// refusal is a fact about the DERIVED placement, not "the extrude's
/// certification".
#[test]
fn r1_c7_the_prs_transform_lifted_shape_with_an_extrude_above_it() {
    let e = eps();
    let mut r = Recorder::new();
    param_doc("lift", 0.0, e / 8.0, &mut r);
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let lifted = r.insert(Node::Transform {
        input: cube,
        translation: [
            len(0.0),
            len(0.0),
            Expr::param(ParamName::new("lift"), Dimension::Length),
        ],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    });
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: lifted,
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::Top)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    let plain = interval_failures(&r.doc, ProfileLift::Pinned);
    let sym = sym_failures(&r.doc, ProfileLift::Pinned);
    println!("R1 C7 transform-lifted eps={e:e} half={:e}: plain {plain:?}; sym {sym:?}", e / 8.0);
    let _ = (boss_p, boss);
    assert!(
        sym.is_empty(),
        "on the driver's lane the extrude above a derived frame on a transform-lifted box \
         must certify as the authored twin does: {sym:?}"
    );
}
