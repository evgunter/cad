//! **A refusal's recourse is a claim, and these rows follow it.**
//!
//! A recourse the door cannot honour is the defect a missing one only
//! looks like, so each row here takes the way through a refusal names
//! and shows the door accepting it: a count redeclared continuous, a
//! count slot moved onto another count, a forward reference rebound
//! back. And a door that forwards an edit refusal the user never
//! authored states its own recourse rather than the edit door's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::block;
use crate::fixture;
use editor_core::{
    BooleanOp, Dimension, Distribution, DocEdit, DocParam, DocumentId, EditError, Expr, Node,
    ParamName, PatternKind, ProfileDoc, ProfileProgram, RecipeNodeId, SitedRef, SlotId, SplitError,
    UnitSym, UpstreamCause, apply,
};
use fixture::{fname, insert, len, run, scl, step, wall};
use geom_core::{Sign, Tol};
use std::collections::BTreeSet;

fn p(name: &'static str) -> ParamName {
    ParamName::from_static(name)
}

fn edit(doc: &ProfileDoc, e: DocEdit<ProfileProgram>) -> Result<ProfileDoc, EditError> {
    apply(doc, &e, Tol::witness(), &editor_core::RefusingReach).map(|a| a.doc)
}

fn length(mm: f64) -> DocParam {
    DocParam::written_length(quantity::WrittenLength::in_unit(mm, quantity::MM))
}

fn mm() -> UnitSym {
    UnitSym::from_def(&quantity::MM.def())
}

/// **A count's unit and distribution: the redeclaration the sentence
/// names gets through.** Both refusals name redeclaring the parameter
/// continuous; the create-or-replace door accepts it, and the unit and
/// the distribution the count refused then land.
#[test]
fn a_count_refuses_a_unit_and_a_distribution_and_names_the_redeclaration_that_gets_through() {
    let doc = ProfileDoc::empty(DocumentId::derive("recourse-count"), Tol::witness());
    let doc = edit(
        &doc,
        DocEdit::SetDocParam {
            name: p("n"),
            value: DocParam::Count { value: 4 },
        },
    )
    .unwrap();
    let band = Distribution::Band { lo: -0.1, hi: 0.1 };
    let refusals = [
        edit(
            &doc,
            DocEdit::SetDocParamUnit {
                name: p("n"),
                unit: mm(),
            },
        )
        .expect_err("a count has no unit"),
        edit(
            &doc,
            DocEdit::SetDocParamDistribution {
                name: p("n"),
                distribution: Some(band),
            },
        )
        .expect_err("a count has no distribution"),
    ];
    for refusal in &refusals {
        let text = refusal.to_string();
        assert!(
            text.contains("Recourse: redeclare it as a continuous parameter")
                && !text.contains("There is no way through"),
            "the count arm names the redeclaration: {text}"
        );
    }
    let redeclared = edit(
        &doc,
        DocEdit::SetDocParam {
            name: p("n"),
            value: length(4.0),
        },
    )
    .expect("the create-or-replace door redeclares an unreferenced count continuous");
    let with_unit = edit(
        &redeclared,
        DocEdit::SetDocParamUnit {
            name: p("n"),
            unit: mm(),
        },
    )
    .expect("the redeclared parameter takes a unit");
    edit(
        &with_unit,
        DocEdit::SetDocParamDistribution {
            name: p("n"),
            distribution: Some(band),
        },
    )
    .expect("the redeclared parameter takes a distribution");
}

/// **A count a slot reads as a count: the redeclaration refuses with
/// the slot's recourse, and that recourse gets through too.** Moving
/// the slot onto another count parameter frees the first to be
/// redeclared.
#[test]
fn a_count_a_slot_reads_refuses_the_redeclaration_with_a_recourse_that_gets_through() {
    let doc = ProfileDoc::empty_derived("recourse_count_slot", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: p("n"),
            value: DocParam::Count { value: 3 },
        },
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body,
            count: Expr::param(p("n"), Dimension::Count),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let refused = edit(
        &doc,
        DocEdit::SetDocParam {
            name: p("n"),
            value: length(4.0),
        },
    )
    .expect_err("a count slot reads n as a count");
    assert!(
        matches!(refused, EditError::SlotDocParamDimension { .. }),
        "the slot's arm refuses the redeclaration: {refused:?}"
    );
    let text = refused.to_string();
    assert!(
        text.contains("Recourse: reference a parameter declared count, or declare this one count"),
        "the slot's recourse: {text}"
    );
    // Its first half, followed: another count, and the slot moved onto it.
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: p("m"),
            value: DocParam::Count { value: 3 },
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pattern,
            slot: SlotId::Count,
            expr: Expr::param(p("m"), Dimension::Count),
        },
    );
    edit(
        &doc,
        DocEdit::SetDocParam {
            name: p("n"),
            value: length(4.0),
        },
    )
    .expect("with no slot reading it, the count redeclares continuous");
}

/// A document holding a Declare whose `b` side was rebound onto a node
/// inserted after it, and that node's wall name before the rebind.
fn forward_declare() -> (ProfileDoc, editor_core::StableName, editor_core::StableName) {
    let doc = ProfileDoc::empty_derived("recourse_split_forward", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (wa, wb) = (wall(&doc, a, 0), wall(&doc, b, 0));
    let (doc, _declare) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(a, wa)),
            SitedRef::new(b, fname(b, wb.clone())),
        )]),
    );
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let wc = wall(&doc, c, 0);
    let (from, to) = (fname(b, wb), fname(c, wc));
    let (doc, _) = step(
        doc,
        DocEdit::Rebind {
            from: from.clone(),
            to: to.clone(),
        },
    );
    (doc, to, from)
}

/// **A split that cannot rebuild a forward reference says what to
/// rebind, not what to edit.** The part is rebuilt in document order,
/// so a Declare rebound onto a later node names a node the part does
/// not hold yet. The user never wrote that insert: the sentence states
/// the split's recourse, and the recourse, followed, splits.
#[test]
fn a_split_that_cannot_rebuild_a_forward_reference_names_the_rebind_that_gets_through() {
    let (doc, forward, back) = forward_declare();
    let cut: BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let split = |doc: &ProfileDoc| {
        editor_core::split(
            doc,
            &cut,
            DocumentId::derive("recourse-split-forward-part"),
            Tol::witness(),
            None,
        )
    };
    let err = split(&doc).expect_err("the part cannot be rebuilt in order");
    assert!(
        matches!(
            &err,
            SplitError::PartEdit { error } if matches!(**error, EditError::DeclareNamesMissingNode { .. })
        ),
        "{err:?}"
    );
    let text = err.to_string();
    assert!(
        text.contains(
            "Recourse: rebind that reference to an entity of a node that comes before the node \
             carrying it, since the split rebuilds the part in document order"
        ) && !text.contains("name an entity of a node the document holds")
            && text.matches("Recourse:").count() == 1,
        "the split's own recourse, once, and not the insert door's: {text}"
    );
    let (back_doc, _) = step(
        doc,
        DocEdit::Rebind {
            from: forward,
            to: back,
        },
    );
    split(&back_doc).expect("rebound onto an earlier node, the cut splits");
}

/// The predicates a plain subtract logs that no owner has words for,
/// each group with its reason. A predicate in neither this list nor an
/// owner's table reds [`every_predicate_a_subtract_logs_has_words_or_a_reason`].
const WORDLESS: &[(&str, &[&str])] = &[
    (
        // Deliberately: the invariant lane (`k_stats::decide_invariant`)
        // is the kernel checking its own result, not a decision about
        // the model, so there is no model question to put in words.
        "the invariant lane",
        &[
            "volume_backstop",
            "volume_backstop_operand",
            "volume_backstop_violation",
        ],
    ),
    (
        // Filed: work/edit/flip-reports-name-no-decision-for-most-predicates.md.
        "no words yet",
        &[
            "bool_chord_side",
            "bool_contact_edge",
            "bool_contact_edge_length",
            "bool_germ_line",
            "bool_join_chord",
            "bool_join_facing",
            "bool_join_nearest",
            "bool_pierce_sector_side_curved",
            "bool_point_in_solid_advance",
            "bool_point_in_solid_denom",
            "bool_point_in_solid_infinity",
            "bool_point_in_solid_order",
            "bool_sector_coplanar",
            "bool_sector_within",
            "bool_vertex_face_side",
            "carrier_endpoint_end",
            "carrier_endpoint_start",
            "carrier_matches_mapped_source",
            "carrier_on_surface_1",
            "carrier_on_surface_2",
            "dihedral_arm",
            "dihedral_wedge",
            "enters_material",
            "enters_material_arm",
            "extrusion_normal_component",
            "interval_span_forward",
            "interval_span_winding",
            "newell_plane_residual",
            "side_planes_cosurface",
            "witness_at_mid_parameter",
            "witness_on_surface_1",
            "witness_on_surface_2",
        ],
    ),
];

/// **Every predicate a plain subtract logs has words in its flip
/// report, or a stated reason not to.** The flip report reads one
/// lookup over every owner's words; a predicate that has words and
/// renders as the unnamed decision fell through it, and a listed
/// predicate that an owner has since given words to is a stale entry.
#[test]
fn every_predicate_a_subtract_logs_has_words_or_a_reason() {
    let doc = ProfileDoc::empty_derived("recourse_flip_census", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, m) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, _cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: m,
            declare: None,
        },
    );
    let ev = run(&doc, &editor_core::EvalOptions::default());
    let logged: BTreeSet<&'static str> = doc
        .order()
        .iter()
        .filter_map(|id| ev.value(*id))
        .flat_map(|v| v.verdicts.iter().map(|verdict| verdict.predicate))
        .collect();
    let unnamed = format!("the margin of {}", geom_core::UNNAMED_DECISION);
    let mut problems = Vec::new();
    for predicate in &logged {
        let text = UpstreamCause::PredicateFlip {
            predicate,
            at: RecipeNodeId(1),
            from: Sign::Negative,
            to: Sign::Positive,
        }
        .to_string();
        let listed = WORDLESS.iter().find(|(_, names)| names.contains(predicate));
        match (text.starts_with(&unnamed), listed) {
            (true, None) => problems.push(format!(
                "{predicate} renders as the unnamed decision and is not listed: {text}"
            )),
            (false, Some((why, _))) => problems.push(format!(
                "{predicate} is listed ({why}) but renders words: {text}"
            )),
            (false, None) | (true, Some(_)) => {}
        }
        if text.contains(predicate) {
            problems.push(format!("{predicate} renders its routing name: {text}"));
        }
        // The owners outside this crate, asked directly: words there
        // that the report does not show fell through the lookup.
        for words in [
            profile::decision_subject(predicate),
            topo::decision_words(predicate),
        ]
        .into_iter()
        .flatten()
        {
            if !text.contains(words) {
                problems.push(format!(
                    "{predicate} has words ({words}) the report drops: {text}"
                ));
            }
        }
    }
    for (why, names) in WORDLESS {
        for name in *names {
            if !logged.contains(name) {
                problems.push(format!(
                    "{name} ({why}) is listed but the subtract logs no such predicate"
                ));
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
