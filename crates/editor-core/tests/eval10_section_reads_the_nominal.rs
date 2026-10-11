//! **A loft's section is its profile's value, as an extrude's profile
//! is** — read off the profile node under the evaluation's lift, never
//! re-resolved.
//!
//! At `f64` the lane environment and the nominal differ only under a
//! degenerate box at a non-zero offset (`f64::axis` accepts `lo == hi`
//! by bits and binds `nominal + c`), so that is the shape: a loft whose
//! lower section's radius is the parameter `p`, evaluated under
//! `p ∈ nominal + [c, c]`. Under the pinned lift the profile's value is
//! the `f64` elaboration at the nominal, and the boxed loft is the
//! unboxed document's, bit for bit. Under the guided lift the profile
//! is elaborated over the lane's environment, and the boxed loft is
//! the document re-authored at `nominal + c`: the section reads what
//! the lane reads (it was pinned to the nominal under both lifts while
//! a section stayed `f64`). The two differ, which is what makes each
//! equality a row rather than a tautology.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;
use crate::fixture::digest::digest;

use editor_core::analysis::{BoxAxis, ParamBox};
use editor_core::{
    Dimension, DocEdit, EvalOptions, Formula, FreeVar, LoopProgram, Node, ProfileDoc,
    ProfileProgram, RecipeNodeId, VarName,
};
use geom_core::Tol;

fn p() -> VarName {
    VarName::from_static("p")
}

/// The probe's one variable in `doc`.
fn var(doc: &ProfileDoc) -> editor_core::VarId {
    doc.var_named("p").expect("the probe declares p")
}

/// `p` a Length parameter at `nominal`; a circle of radius `p` on the
/// xy frame; a circle of radius one on a frame one unit up; the loft
/// between them. Returns the document and the loft's id.
fn loft_doc(nominal: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("eval10_section_reads_the_nominal", Tol::witness());
    let doc = doc
        .apply(
            &DocEdit::DeclareVar {
                name: p(),
                def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, nominal)),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("the parameter declares")
        .doc;
    let (doc, lower_frame) = fixture::insert(doc, fixture::xy_frame());
    let (doc, lower) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: lower_frame.into(),
            loops: vec![LoopProgram::Circle {
                centre: [fixture::len(0.0), fixture::len(0.0)],
                radius: Formula::named(p(), Dimension::Length),
            }],
            ids: Vec::new(),
        }),
    );
    let (doc, upper_frame) = fixture::insert(
        doc,
        fixture::frame([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, upper) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: upper_frame.into(),
            loops: vec![LoopProgram::Circle {
                centre: [fixture::len(0.0), fixture::len(0.0)],
                radius: fixture::len(1.0),
            }],
            ids: Vec::new(),
        }),
    );
    fixture::insert(
        doc,
        Node::Loft {
            profiles: vec![lower.into(), upper.into()],
            v_degree: Formula::count(1),
        },
    )
}

/// The one-axis degenerate box `p ∈ nominal + [offset, offset]`.
fn boxed_at(doc: &ProfileDoc, offset: f64) -> EvalOptions {
    let mut axes = BTreeMap::new();
    axes.insert(
        var(doc),
        BoxAxis::Varying {
            lo: offset,
            hi: offset,
        },
    );
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(axes))),
        ..EvalOptions::default()
    }
}

/// Under the pinned lift the boxed loft is the unboxed document's;
/// under the guided lift it is the offset document's.
#[test]
fn a_section_under_a_degenerate_offset_box_reads_its_profile_value() {
    const NOMINAL: f64 = 0.5;
    const OFFSET: f64 = 0.25;
    let (doc, loft) = loft_doc(NOMINAL);
    let guided = |opts: EvalOptions| EvalOptions {
        profile_lift: editor_core::ProfileLift::Guided,
        ..opts
    };
    let unboxed = fixture::run(&doc, &EvalOptions::default());
    let boxed = fixture::run(&doc, &boxed_at(&doc, OFFSET));
    let boxed_guided = fixture::run(&doc, &guided(boxed_at(&doc, OFFSET)));
    let (shifted_doc, shifted_loft) = loft_doc(NOMINAL + OFFSET);
    let shifted = fixture::run(&shifted_doc, &EvalOptions::default());
    assert_eq!(shifted_loft, loft, "the same insertion order");
    for (name, ev) in [
        ("unboxed", &unboxed),
        ("boxed", &boxed),
        ("boxed, guided", &boxed_guided),
        ("shifted", &shifted),
    ] {
        assert!(
            ev.node_error(loft).is_none(),
            "{name}: the loft builds: {:?}",
            ev.node_error(loft)
        );
    }
    assert_ne!(
        digest(&shifted),
        digest(&unboxed),
        "the offset moves the section, so the equalities below are not vacuous"
    );
    assert_eq!(
        digest(&boxed),
        digest(&unboxed),
        "the pinned lift's profile is the nominal's, and so is the loft's section"
    );
    assert_eq!(
        loft_digest(&boxed_guided, loft),
        loft_digest(&shifted, loft),
        "the guided lift's profile is the lane's, and so is the loft's section"
    );
}

/// The bits of every vertex of `loft`'s body.
fn loft_digest(ev: &editor_core::Evaluation<f64>, loft: RecipeNodeId) -> Vec<u64> {
    let Some(editor_core::ValuePayload::Body(body)) = ev.value(loft).map(|v| &v.payload) else {
        panic!("the loft is a body");
    };
    body.points()
        .flat_map(|(_, p)| p.to_array().map(f64::to_bits))
        .collect()
}

/// **A seed on a section's radius reaches the loft's walls.** At
/// `Dual64` with the seed on `p` (the lower section's radius) and the
/// guided lift, the lower rim's vertex moves with `p`: the section is
/// its profile's value at the lane scalar, carried through the skin
/// (it was a dual constant while a section stayed `f64`).
#[test]
fn a_seed_on_a_section_radius_moves_the_lofted_wall() {
    use geom_core::Dual64;
    let (doc, loft) = loft_doc(0.5);
    let ev = editor_core::evaluate::<Dual64>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions {
            profile_lift: editor_core::ProfileLift::Guided,
            seed: Some(var(&doc)),
            ..EvalOptions::default()
        },
        Tol::witness(),
    );
    assert!(ev.node_error(loft).is_none(), "{:?}", ev.node_error(loft));
    let Some(editor_core::ValuePayload::Body(body)) = ev.value(loft).map(|v| &v.payload) else {
        panic!("the loft is a body");
    };
    let moved: Vec<f64> = body
        .points()
        .filter(|(_, p)| p.z.value == 0.0)
        .map(|(_, p)| p.x.deriv.hypot(p.y.deriv))
        .collect();
    assert!(!moved.is_empty(), "the lower rim has a vertex");
    assert!(
        moved.iter().all(|d| (d - 1.0).abs() <= 1e-12),
        "the lower rim's vertex moves one for one with the radius: {moved:?}"
    );
}

/// **A box over a section's radius is refused, never the nominal over
/// the box.** At `Interval` under the guided lift (the drive's), a `p`
/// widened by ε/16 still validates as a profile, but its value is wider
/// than a point, and the loft refuses `SectionNotAPoint` naming the
/// section. While a section was resolved at the nominal, this loft
/// built its nominal wall and the lane certified it over the whole box.
#[test]
fn a_box_over_a_section_radius_refuses_rather_than_lofting_the_nominal() {
    use geom_core::Interval;
    let (doc, loft) = loft_doc(0.5);
    let mut axes = BTreeMap::new();
    axes.insert(
        var(&doc),
        BoxAxis::Varying {
            lo: -Tol::witness().get().eps / 16.0,
            hi: Tol::witness().get().eps / 16.0,
        },
    );
    let ev = editor_core::evaluate::<Interval>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions {
            profile_lift: editor_core::ProfileLift::Guided,
            param_box: Some(Arc::new(ParamBox::from_axes(axes))),
            ..EvalOptions::default()
        },
        Tol::witness(),
    );
    match ev.node_error(loft).map(|e| &e.kind) {
        Some(editor_core::NodeErrorKind::Loft(sweep::LoftError::SectionNotAPoint {
            section: 0,
        })) => {}
        other => panic!("the boxed loft must refuse its wide section, got {other:?}"),
    }
}

/// **A loft on a face frame evaluates at `Dual64` and at `Sym<f64>`**
/// (the `Interval` row is `docm1_face_frame_interval`'s), under the
/// guided lift: the section on the derived frame is its profile's
/// value there, placed on the frame's landed value.
#[test]
fn a_loft_on_a_face_frame_evaluates_at_dual_and_sym() {
    use editor_core::ProfileLift;
    use geom_core::{Dual64, Sym, SymBudget};
    let (doc, loft) = crate::docm1_face_frame::lofted_on_face_frame();
    let opts = EvalOptions {
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    let dual = editor_core::evaluate::<Dual64>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    assert!(
        dual.node_error(loft).is_none(),
        "Dual: {:?}",
        dual.node_error(loft)
    );
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (sym_ok, _) = geom_core::sym::with_session(budget, || {
        let ev = editor_core::evaluate::<Sym<f64>>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &opts,
            Tol::witness(),
        );
        ev.node_error(loft).map(|e| e.kind.to_string())
    });
    assert_eq!(sym_ok, None, "Sym");
}
