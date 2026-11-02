//! **A loft's section is its profile's value, as an extrude's profile
//! is** — read off the profile node, never re-resolved.
//!
//! At `f64` the lane environment and the nominal differ only under a
//! degenerate box at a non-zero offset (`f64::axis` accepts `lo == hi`
//! by bits and binds `nominal + c`), so that is the shape: a loft whose
//! lower section's radius is the parameter `p`, evaluated under
//! `p ∈ nominal + [c, c]`. The profile is elaborated over the lane's
//! environment, and the boxed loft is the document re-authored at
//! `nominal + c`: the section reads what the lane reads. That differs
//! from the unboxed document, which is what makes the equality a row
//! rather than a tautology.

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
    let circle = |radius| LoopProgram::Circle {
        centre: [fixture::len(0.0), fixture::len(0.0)],
        radius,
    };
    loft_doc_of(
        nominal,
        circle(Formula::named(p(), Dimension::Length)),
        circle(fixture::len(1.0)),
    )
}

/// [`loft_doc`] with rectangles for circles: the lower section
/// `[0, p] × [0, 1]`, the upper the unit square. Every value is a line
/// endpoint, so at `Interval` with no box the sections are points.
fn square_loft_doc(nominal: f64) -> (ProfileDoc, RecipeNodeId) {
    let rect = |w: Formula| {
        let at = |x: Formula, y: f64| {
            editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([
                x,
                fixture::len(y),
            ]))
        };
        LoopProgram::Chain(vec![
            editor_core::ProgramStep::At([fixture::len(0.0), fixture::len(0.0)]),
            at(w.clone(), 0.0),
            at(w, 1.0),
            at(fixture::len(0.0), 1.0),
            editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Start),
        ])
    };
    loft_doc_of(
        nominal,
        rect(Formula::named(p(), Dimension::Length)),
        rect(fixture::len(1.0)),
    )
}

/// `p` at `nominal`; `lower` on the xy frame, `upper` on a frame one
/// unit up, and the loft between them.
fn loft_doc_of(
    nominal: f64,
    lower: LoopProgram<Formula>,
    upper: LoopProgram<Formula>,
) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("eval10_section_reads_the_nominal", Tol::witness());
    let doc = doc
        .apply(
            &DocEdit::DeclareVar {
                name: p(),
                def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, nominal)),
            },
            Tol::witness(),
        )
        .expect("the parameter declares")
        .doc;
    let (doc, lower) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![lower],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, upper) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![upper],
            ids: Vec::new(),
            on: None,
        }),
    );
    fixture::insert(
        doc,
        Node::Loft {
            first: lower.into(),
            then: vec![editor_core::LoftSection {
                profile: upper.into(),
                station: vec![editor_core::StationMove::Slide {
                    along: editor_core::Axis3::Z,
                    by: fixture::len(1.0),
                }],
            }],
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

/// The boxed loft is the offset document's, not the unboxed one's.
#[test]
fn a_section_under_a_degenerate_offset_box_reads_its_profile_value() {
    const NOMINAL: f64 = 0.5;
    const OFFSET: f64 = 0.25;
    let (doc, loft) = loft_doc(NOMINAL);
    let unboxed = fixture::run(&doc, &EvalOptions::default());
    let boxed = fixture::run(&doc, &boxed_at(&doc, OFFSET));
    let (shifted_doc, shifted_loft) = loft_doc(NOMINAL + OFFSET);
    let shifted = fixture::run(&shifted_doc, &EvalOptions::default());
    assert_eq!(shifted_loft, loft, "the same insertion order");
    for (name, ev) in [
        ("unboxed", &unboxed),
        ("boxed", &boxed),
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
        "the offset moves the section, so the equality below is not vacuous"
    );
    assert_eq!(
        loft_digest(&boxed, loft),
        loft_digest(&shifted, loft),
        "the profile is the lane's, and so is the loft's section"
    );
}

/// The bits of every vertex of `loft`'s body.
fn loft_digest(ev: &editor_core::Evaluation<f64>, loft: RecipeNodeId) -> Vec<u64> {
    let Some(editor_core::ValuePayload::Body(body)) = ev.value(loft).map(|v| &v.payload) else {
        panic!("the loft is a body");
    };
    body.body()
        .points()
        .flat_map(|(_, p)| p.to_array().map(f64::to_bits))
        .collect()
}

/// **A seed on a section's radius reaches the loft's walls.** At
/// `Dual64` with the seed on `p` (the lower section's radius), the
/// lower rim's vertex moves with `p`: the section is
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
        .body()
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

/// **A box over a section's width lofts the family it spans, never the
/// nominal over the box.** At `Interval`, with `p` widened by ε/256,
/// the lower section's value is
/// wider than a point and the loft skins it in `T`. (At ε/16 a wall's
/// pcurve residual encloses about 13 times the box's width and
/// escalates at the band, which the drive bisects.) Every vertex of the
/// `f64` loft at each end of the box and at its middle (the degenerate
/// box at that offset, [`boxed_at`]) lies in the
/// enclosing body's vertex of the same index. Were the nominal lofted over the box, the
/// ends would fall outside it.
#[test]
fn a_box_over_a_section_width_lofts_the_family_it_spans() {
    use geom_core::{Bounds, Interval};
    let (doc, loft) = square_loft_doc(0.5);
    let w = Tol::witness().get().eps / 256.0;
    let mut axes = BTreeMap::new();
    axes.insert(var(&doc), BoxAxis::Varying { lo: -w, hi: w });
    let ev = editor_core::evaluate::<Interval>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions {
            param_box: Some(Arc::new(ParamBox::from_axes(axes))),
            ..EvalOptions::default()
        },
        Tol::witness(),
    );
    assert!(
        ev.node_error(loft).is_none(),
        "the boxed loft evaluates: {:?}",
        ev.node_error(loft)
    );
    let brackets: Vec<[(f64, f64); 3]> = crate::corpus::body_of(&ev, loft)
        .points()
        .map(|(_, p)| [p.x, p.y, p.z].map(|c| (c.lo(), c.hi())))
        .collect();
    for offset in [-w, 0.0, w] {
        let member = editor_core::evaluate::<f64>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &boxed_at(&doc, offset),
            Tol::witness(),
        );
        let vertices: Vec<[f64; 3]> = crate::corpus::body_of(&member, loft)
            .points()
            .map(|(_, p)| [p.x, p.y, p.z])
            .collect();
        assert_eq!(
            vertices.len(),
            brackets.len(),
            "offset {offset}: the vertex counts agree"
        );
        for (i, (b, v)) in brackets.iter().zip(&vertices).enumerate() {
            assert!(
                b.iter().zip(v).all(|((lo, hi), x)| lo <= x && x <= hi),
                "offset {offset}: the member's vertex {i} at {v:?} lies outside its enclosure {b:?}"
            );
        }
    }
}
