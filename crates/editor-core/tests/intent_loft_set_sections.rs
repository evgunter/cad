//! **A loft's sections are written whole, each with its station**
//! (`DocEdit::SetLoftSections`): adding, dropping and reordering
//! sections is that one edit, and `SetMembers` handed a loft keeps its
//! count and names it as the recourse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len};
use editor_core::{
    Axis3, DocEdit, EditError, EvalOptions, Formula, LoftSection, Node, NodeResult, Operand,
    ProfileDoc, RecipeNodeId, StationMove, ValuePayload,
};
use geom_core::Tol;

/// The body `node` evaluates to, as the `z` of its end caps and its
/// volume. A degree-1 loft is ruled between consecutive sections, so a
/// middle section's station shows in the volume, not in a vertex.
fn measured(doc: &ProfileDoc, node: RecipeNodeId) -> ([f64; 2], f64) {
    let ev = editor_core::evaluate::<f64>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let Some(NodeResult::Ok(value)) = ev.result(node) else {
        panic!("{node} evaluates: {:?}", ev.result(node));
    };
    let ValuePayload::Body(body) = &value.payload else {
        panic!("{node} is a body: {:?}", value.payload);
    };
    let mut zs: Vec<f64> = body
        .vertices()
        .map(|(v, _)| {
            topo::readback::vertex_point(body, v)
                .expect("a vertex point")
                .z
        })
        .collect();
    zs.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let volume = topo::mass_properties(
        &topo::test_support::finished("the loft", topo::Body::clone(body), Tol::witness()),
        Tol::witness(),
    )
    .expect("mass properties")
    .volume;
    ([zs[0], zs[zs.len() - 1]], volume)
}

/// The volume of a degree-1 loft through square sections, each
/// `(half-side, z)`: a frustum per pair, `d·4(h₀² + h₀h₁ + h₁²)/3`.
fn frusta(sections: &[(f64, f64)]) -> f64 {
    sections
        .windows(2)
        .map(|w| {
            let ((h0, z0), (h1, z1)) = (w[0], w[1]);
            (z1 - z0) * 4.0 * (h0 * h0 + h0 * h1 + h1 * h1) / 3.0
        })
        .sum()
}

/// `got` is `(caps, frusta(want))`.
fn assert_loft(got: ([f64; 2], f64), want: &[(f64, f64)], what: &str) {
    let (caps, volume) = got;
    let expected = frusta(want);
    assert_eq!(
        caps,
        [want[0].1, want[want.len() - 1].1],
        "{what}: the end caps"
    );
    assert!(
        (volume - expected).abs() <= 1e-9 * expected,
        "{what}: volume {volume}, through {want:?} it is {expected}"
    );
}

/// A section of `profile` slid up the first section's `z` by `z`.
fn up(profile: RecipeNodeId, z: f64) -> LoftSection<Operand, Formula> {
    LoftSection {
        profile: profile.into(),
        station: vec![StationMove::Slide {
            along: Axis3::Z,
            by: len(z),
        }],
    }
}

/// Three squares, of half-sides 0.5, 0.4 and 0.3, and a loft from the
/// first to the second at `z = 1`.
fn two_section_loft() -> (ProfileDoc, [RecipeNodeId; 3], RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let square = |h| Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, h)]));
    let (doc, a) = insert(doc, square(0.5));
    let (doc, b) = insert(doc, square(0.4));
    let (doc, c) = insert(doc, square(0.3));
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            first: a.into(),
            then: vec![up(b, 1.0)],
            v_degree: Formula::count(1),
        },
    );
    (doc, [a, b, c], loft)
}

fn set(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    first: RecipeNodeId,
    then: Vec<LoftSection<Operand, Formula>>,
) -> Result<ProfileDoc, EditError> {
    doc.apply(
        &DocEdit::SetLoftSections {
            node,
            first: first.into(),
            then,
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .map(|applied| applied.doc)
}

/// **Adding, dropping and reordering sections**: each is the one edit
/// with the full list, and the loft evaluates through the sections at
/// the stations written. Broken by a door that keeps the old list, or
/// drops or defaults a written station (the section planes would not
/// be the ones written).
#[test]
fn set_loft_sections_adds_drops_and_reorders() {
    let (doc, [a, b, c], loft) = two_section_loft();
    assert_loft(
        measured(&doc, loft),
        &[(0.5, 0.0), (0.4, 1.0)],
        "as inserted",
    );

    let added = set(&doc, loft, a, vec![up(b, 1.0), up(c, 2.5)]).expect("a section added");
    let Some(Node::Loft { then, .. }) = added.node(loft) else {
        panic!("{loft} is a loft");
    };
    assert_eq!(then.len(), 2, "the loft holds the added section");
    assert_eq!(
        added.upstream(loft),
        vec![a, b, c],
        "the added section's profile is read"
    );
    assert_loft(
        measured(&added, loft),
        &[(0.5, 0.0), (0.4, 1.0), (0.3, 2.5)],
        "the added section stands at its written station",
    );

    let dropped = set(&added, loft, a, vec![up(c, 2.5)]).expect("a section dropped");
    assert_eq!(
        dropped.upstream(loft),
        vec![a, c],
        "the dropped read is gone"
    );
    assert_loft(
        measured(&dropped, loft),
        &[(0.5, 0.0), (0.3, 2.5)],
        "the kept section keeps the station written for it",
    );

    // The first section moves to the far end: c stands at the loft's
    // own plane, a and b above it.
    let reordered =
        set(&added, loft, c, vec![up(b, 1.0), up(a, 2.5)]).expect("the sections reordered");
    let Some(Node::Loft { first, then, .. }) = reordered.node(loft) else {
        panic!("{loft} is a loft");
    };
    let order: Vec<_> = core::iter::once(*first)
        .chain(then.iter().map(|s| s.profile))
        .map(|var| reordered.read_operation(var))
        .collect();
    assert_eq!(
        order,
        [Some(c), Some(b), Some(a)],
        "the sections in the order written"
    );
    assert_loft(
        measured(&reordered, loft),
        &[(0.3, 0.0), (0.4, 1.0), (0.5, 2.5)],
        "reordered",
    );
}

/// **A node that is not a loft refuses**, naming the edit's own
/// recourse. Broken by a door that writes sections onto any node with
/// a list input, or that answers `SetMembersOnNonList`.
#[test]
fn set_loft_sections_refuses_a_non_loft() {
    let (doc, [a, b, _], _) = two_section_loft();
    let err = set(&doc, b, a, vec![up(b, 1.0)]).expect_err("a profile is not a loft");
    assert!(
        matches!(&err, EditError::SetLoftSectionsOnNonLoft { node } if node.id() == b),
        "{err:?}"
    );
    assert!(
        err.to_string().contains("is not a loft"),
        "the sentence: {err}"
    );
}

/// **A section read that is not a profile refuses** as the insert door
/// refuses it, and the document is untouched. Broken by a door that
/// writes the read without asking its kind.
#[test]
fn set_loft_sections_refuses_a_non_profile_section() {
    let (doc, [a, b, _], loft) = two_section_loft();
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: b.into(),
            distance: len(1.0),
        },
    );
    let err = set(&doc, loft, a, vec![up(block, 1.0)]).expect_err("a body is not a profile");
    assert!(matches!(err, EditError::SlotVarKind { .. }), "{err:?}");
    assert_eq!(doc.upstream(loft), vec![a, b], "the loft as it was");
}

/// **`SetMembers` handed a loft a different count refuses**, and its
/// sentence names `SetLoftSections` as the way to add a section.
/// Broken by a count check that is dropped (an added section would
/// have no station) or a recourse that names no edit that can do it.
#[test]
fn set_members_on_a_loft_names_set_loft_sections() {
    let (doc, [a, b, c], loft) = two_section_loft();
    let err = doc
        .apply(
            &DocEdit::SetMembers {
                node: loft,
                members: editor_core::Bodies::Spelled(vec![a.into(), b.into(), c.into()]),
            },
            Tol::witness(),
        )
        .expect_err("a third section has no station");
    assert!(
        matches!(
            err,
            EditError::LoftSectionCount {
                sections: 2,
                given: 3,
                ..
            }
        ),
        "{err:?}"
    );
    assert!(
        err.to_string().contains("SetLoftSections"),
        "the recourse names the edit: {err}"
    );
}
