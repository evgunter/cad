//! **A loft's section resolves at the nominal, not at the lane's
//! parameters** — the row for the `section_of` entry in
//! `wire::LaneEnv::nominal`'s census.
//!
//! At `f64` the lane environment and the nominal differ only under a
//! degenerate box at a non-zero offset (`f64::axis` accepts `lo == hi`
//! by bits and binds `nominal + c`), so that is the shape: a loft whose
//! lower section's radius is the parameter `p`, evaluated under
//! `p ∈ nominal + [c, c]`. The loft's body is the unboxed document's,
//! bit for bit — a section is `f64`-pinned structure (C6/D9), and the
//! environment that pins it is the document's own. A third evaluation,
//! the document re-authored at `nominal + c` and unboxed, is what the
//! boxed loft would equal had the section read the lane's environment;
//! it differs, which is what makes the first equality a row rather
//! than a tautology.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;
use crate::fixture::digest::digest;

use editor_core::analysis::{BoxAxis, ParamBox};
use editor_core::{
    Dimension, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node, ParamName, ProfileDoc,
    ProfileProgram, RecipeNodeId,
};
use geom_core::Tol;

fn p() -> ParamName {
    ParamName::new("p")
}

/// `p` a Length parameter at `nominal`; a circle of radius `p` on the
/// xy frame; a circle of radius one on a frame one unit up; the loft
/// between them. Returns the document and the loft's id.
fn loft_doc(nominal: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("eval10_section_reads_the_nominal", Tol::witness());
    let doc = doc
        .apply(
            &DocEdit::SetDocParam {
                name: p(),
                value: DocParam::continuous(Dimension::Length, nominal),
            },
            Tol::witness(),
        )
        .expect("the parameter declares")
        .doc;
    let (doc, lower_frame) = fixture::insert(doc, fixture::xy_frame());
    let (doc, lower) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: lower_frame,
            loops: vec![LoopProgram::Circle {
                centre: [fixture::len(0.0), fixture::len(0.0)],
                radius: Expr::param(p(), Dimension::Length),
            }],
        }),
    );
    let (doc, upper_frame) = fixture::insert(
        doc,
        fixture::frame([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, upper) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: upper_frame,
            loops: vec![LoopProgram::Circle {
                centre: [fixture::len(0.0), fixture::len(0.0)],
                radius: fixture::len(1.0),
            }],
        }),
    );
    fixture::insert(
        doc,
        Node::Loft {
            profiles: vec![lower, upper],
            v_degree: Expr::count(1),
        },
    )
}

/// The one-axis degenerate box `p ∈ nominal + [offset, offset]`.
fn boxed_at(offset: f64) -> EvalOptions {
    let mut axes = BTreeMap::new();
    axes.insert(
        p(),
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

/// The boxed loft is the unboxed document's, and not the offset
/// document's.
#[test]
fn a_section_under_a_degenerate_offset_box_resolves_at_the_nominal() {
    const NOMINAL: f64 = 0.5;
    const OFFSET: f64 = 0.25;
    let (doc, loft) = loft_doc(NOMINAL);
    let unboxed = fixture::run(&doc, &EvalOptions::default());
    let boxed = fixture::run(&doc, &boxed_at(OFFSET));
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
    assert_eq!(
        digest(&boxed),
        digest(&unboxed),
        "the boxed section is resolved at the nominal"
    );
    assert_ne!(
        digest(&shifted),
        digest(&unboxed),
        "the offset moves the section when it is the nominal, so the equality above is not vacuous"
    );
}
