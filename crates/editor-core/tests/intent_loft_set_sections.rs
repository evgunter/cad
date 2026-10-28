//! **A loft's sections are written whole, each with its station**
//! (`DocEdit::SetLoftSections`): adding, dropping and reordering
//! sections is that one edit, and `SetMembers` handed a loft keeps its
//! count and names it as the recourse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len};
use editor_core::{
    Axis3, DocEdit, EditError, EvalOptions, Formula, LoftFault, LoftSection, Node, NodeResult,
    Operand, ProfileDoc, RecipeNodeId, SlotId, StationMove, ValuePayload,
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
        .body()
        .vertices()
        .map(|(v, _)| {
            topo::readback::vertex_point(body.body(), v)
                .expect("a vertex point")
                .z
        })
        .collect();
    zs.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let volume = topo::test_support::pinned_volume(
        "the loft",
        &topo::test_support::finished("the loft", topo::Body::clone(body.body()), Tol::witness()),
        Tol::witness(),
    )
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
/// refuses it. Broken by a door that writes the read without asking its
/// kind.
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
}

/// **A station move of the wrong dimension refuses** at the slot it
/// fills: a spin by a length. Broken by a door that lowers the station
/// without asking D6's slot rule.
#[test]
fn set_loft_sections_refuses_a_station_of_the_wrong_dimension() {
    let (doc, [a, b, _], loft) = two_section_loft();
    let spun_by_a_length = LoftSection {
        profile: b.into(),
        station: vec![
            StationMove::Slide {
                along: Axis3::Z,
                by: len(1.0),
            },
            StationMove::Spin {
                about: Axis3::Z,
                by: len(1.0),
            },
        ],
    };
    let err = set(&doc, loft, a, vec![spun_by_a_length]).expect_err("a spin is an angle");
    assert!(
        matches!(
            err,
            EditError::SlotDimensionMismatch {
                slot: SlotId::StationSpin {
                    section: 1,
                    step: 1
                },
                ..
            }
        ),
        "{err:?}"
    );
}

/// The sections after a loft's first, as an edit writes them.
type Sections = Vec<LoftSection<Operand, Formula>>;

/// The loft `then` and `degree` make over `a`, as `InsertNode` writes
/// it.
fn lofted(
    a: RecipeNodeId,
    then: Vec<LoftSection<Operand, Formula>>,
    degree: i64,
) -> editor_core::AuthoredNode {
    Node::Loft {
        first: a.into(),
        then,
        v_degree: Formula::count(degree),
    }
}

/// `err` is `LoftShape` of `loft` with `fault`.
fn assert_shape(err: &EditError, fault: &LoftFault, what: &str) {
    assert!(
        matches!(err, EditError::LoftShape { fault: f, .. } if f == fault),
        "{what}: {err:?}"
    );
}

/// **A shape no evaluation can loft refuses at both doors alike**: one
/// section, a section with an empty station or one whose moves carry
/// the first section's plane onto itself under the current values (it
/// stands in that plane), and a degree the sections cannot carry, each
/// through `InsertNode` and through `SetLoftSections`. Broken by a
/// shape check that one door asks and the other does not, or that
/// either leaves to evaluation.
#[test]
fn a_loft_shape_no_evaluation_lofts_refuses_at_both_doors() {
    let (doc, [a, b, c], loft) = two_section_loft();
    let empty = || LoftSection {
        profile: b.into(),
        station: Vec::new(),
    };
    let in_plane = |station| LoftSection {
        profile: b.into(),
        station,
    };
    let slide = |along, by| StationMove::Slide { along, by: len(by) };
    let rows: [(Sections, i64, LoftFault, &str); 6] = [
        (
            Vec::new(),
            1,
            LoftFault::TooFewSections { have: 1 },
            "one section",
        ),
        (
            vec![up(b, 1.0), empty()],
            1,
            LoftFault::EmptyStation { section: 2 },
            "an empty station",
        ),
        (
            vec![up(b, 0.0)],
            1,
            LoftFault::StationInFirstPlane { section: 1 },
            "a slide by nothing",
        ),
        (
            vec![
                up(c, 2.0),
                in_plane(vec![
                    slide(Axis3::X, 1.0),
                    StationMove::Spin {
                        about: Axis3::Z,
                        by: fixture::ang(0.5),
                    },
                ]),
            ],
            1,
            LoftFault::StationInFirstPlane { section: 2 },
            "a slide and a spin within the plane",
        ),
        (
            vec![in_plane(vec![slide(Axis3::Z, 1.0), slide(Axis3::Z, -1.0)])],
            1,
            LoftFault::StationInFirstPlane { section: 1 },
            "a slide up and back",
        ),
        (
            vec![up(c, 2.0)],
            2,
            LoftFault::DegreeOutOfRange {
                degree: 2,
                sections: 2,
            },
            "degree 2 through two sections",
        ),
    ];
    for (then, degree, fault, what) in rows {
        let inserted = fixture::insert_refused(&doc, lofted(a, then.clone(), degree));
        assert_shape(&inserted, &fault, &format!("insert, {what}"));
        // The degree is the loft's own, kept by the edit: a degree-2
        // loft through three sections dropped to two.
        let (base, target) = if degree == 1 {
            (doc.clone(), loft)
        } else {
            insert(doc.clone(), lofted(a, vec![up(b, 1.0), up(c, 2.0)], degree))
        };
        let rewritten = set(&base, target, a, then).expect_err(what);
        assert_shape(&rewritten, &fault, &format!("SetLoftSections, {what}"));
    }
    let err = fixture::insert_refused(&doc, lofted(a, Vec::new(), 1));
    assert!(
        err.to_string().contains("through 1 section"),
        "the sentence: {err}"
    );
    let err = fixture::insert_refused(&doc, lofted(a, vec![up(b, 0.0)], 1));
    let said = err.to_string();
    assert!(
        said.contains("onto itself under the current values")
            && said.contains("slide it along the first section's z"),
        "the sentence and its recourse: {said}"
    );
}

/// **A degree written past the sections refuses at the slot door
/// too**, so no edit leaves a loft in a shape the insert door refuses.
/// Broken by a slot door that writes the degree without asking the
/// loft's shape.
#[test]
fn a_degree_written_past_the_sections_refuses() {
    let (doc, _, loft) = two_section_loft();
    let err = doc
        .apply(
            &DocEdit::SetStructuralParam {
                node: loft,
                slot: SlotId::VDegree,
                expr: Formula::count(2),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("two sections admit degree 1 alone");
    assert_shape(
        &err,
        &LoftFault::DegreeOutOfRange {
            degree: 2,
            sections: 2,
        },
        "SetStructuralParam",
    );
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

/// The loft's evaluation, which is the skin's refusal of a degenerate
/// stacking.
fn assert_degenerate_stacking(doc: &ProfileDoc, loft: RecipeNodeId) {
    let ev = editor_core::evaluate::<f64>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(
        matches!(
            ev.result(loft),
            Some(NodeResult::Failed(editor_core::NodeError {
                kind: editor_core::NodeErrorKind::Loft(sweep::LoftError::DegenerateStacking {
                    slab: 0
                }),
                ..
            }))
        ),
        "{:?}",
        ev.result(loft)
    );
}

/// **A station rounding leaves next to the first plane is the skin's to
/// refuse**: two half turns about x compose, in `f64`, to a plane a
/// rounding off the first, which the door's exact check admits and the
/// evaluation refuses typed. Broken by a door that judges coincidence
/// within a tolerance of its own, or a skin that lofts the sliver.
#[test]
fn a_station_rounded_next_to_the_first_plane_is_refused_by_the_skin() {
    let (doc, [a, b, _], _) = two_section_loft();
    let half_turn = || StationMove::Spin {
        about: Axis3::X,
        by: fixture::ang(std::f64::consts::PI),
    };
    let (doc, loft) = insert(
        doc,
        lofted(
            a,
            vec![LoftSection {
                profile: b.into(),
                station: vec![half_turn(), half_turn()],
            }],
            1,
        ),
    );
    assert_degenerate_stacking(&doc, loft);
}

/// **A value edit that brings a station's slide to 0 is admitted, and
/// evaluation refuses the loft**: the door reads a station's values
/// when it writes the loft, and `SetVarValue` writes no loft. The next
/// edit that does write it (here its own degree, unchanged) refuses it
/// at the door. Broken by a value edit that re-asks the loft's shape
/// (this row then pins the wrong door), or by a skin that lofts the
/// coincident section.
#[test]
fn a_value_edit_to_a_zero_slide_is_refused_at_evaluation() {
    let (doc, _, loft) = two_section_loft();
    let Some(Node::Loft { then, .. }) = doc.node(loft) else {
        panic!("{loft} is a loft")
    };
    let [LoftSection { station, .. }] = &then[..] else {
        panic!("one section after the first: {then:?}")
    };
    let [StationMove::Slide { by, .. }] = &station[..] else {
        panic!("one slide: {station:?}")
    };
    let doc = doc
        .apply(
            &DocEdit::SetVarValue {
                var: (*by).into(),
                value: editor_core::FreeValue::Continuous(0.0),
            },
            Tol::witness(),
        )
        .expect("a value edit does not re-ask the loft's shape")
        .doc;
    assert_degenerate_stacking(&doc, loft);
    let err = doc
        .apply(
            &DocEdit::SetStructuralParam {
                node: loft,
                slot: SlotId::VDegree,
                expr: Formula::count(1),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("a slot edit of the loft asks its shape");
    assert_shape(
        &err,
        &LoftFault::StationInFirstPlane { section: 1 },
        "SetStructuralParam after the value edit",
    );
}
