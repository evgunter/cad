//! Review probes for PR 6856 (lane `placing-dual-6856-r2`): rows that try
//! to falsify the named-source claims. Each row asserts what the PR or
//! VARIABLES VR4/VR9 promise; a red row is a finding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{insert, len, out, step};

use editor_core::{
    CapEnd, DocEdit, EditError, EntityKind, Node, Operand, PersistError, ProfileDoc, RoleSeg,
    SnapshotError, StableName, VarDecl, VarId, VarName,
};
use geom_core::Tol;

fn placed(read: Operand) -> editor_core::AuthoredNode {
    Node::place(
        editor_core::BodyRead::plain(read),
        Operand::Var(VarId::WORLD),
        editor_core::stand().bundle(),
    )
}

fn raw_insert(doc: &ProfileDoc, node: editor_core::AuthoredNode) -> Result<ProfileDoc, EditError> {
    doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .map(|applied| applied.doc)
}

fn top(node: editor_core::RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

/// VR9: "A placement passed a body mints an anonymous source for that
/// call". A body held by a *named output* variable is a body; the
/// façade's `Node::place` passes a `Name`/`Var` read through unwrapped,
/// where `Doc::place_read` wraps it. Asserts the VR9 promise.
#[test]
fn probe_facade_place_of_a_named_output_mints_a_source() {
    let doc = ProfileDoc::empty_derived("r2-named-output", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let body = out(&doc, b);
    let (doc, _) = step(
        doc,
        DocEdit::RenameVar {
            var: body.into(),
            name: Some(VarName::from_static("plate")),
        },
    );
    let by_name = raw_insert(&doc, placed(Operand::Name(VarName::from_static("plate"))));
    let by_var = raw_insert(&doc, placed(Operand::Var(body)));
    // The document-holding spelling wraps both.
    let wrapped = raw_insert(
        &doc,
        placed(doc.place_read(Operand::Name(VarName::from_static("plate")))),
    );
    assert!(wrapped.is_ok(), "place_read wraps: {:?}", wrapped.err());
    assert!(
        by_name.is_ok() && by_var.is_ok(),
        "façade place(plate) by name: {:?}; by var: {:?}",
        by_name.err(),
        by_var.err()
    );
}

/// The reader-count qualifier admits a placement's own selections of
/// its anonymous source, and no more: a non-placement node selecting in
/// that source is a second reader (VR2), refused `SharedVarNeedsName`.
#[test]
fn probe_another_node_selecting_in_an_anonymous_source_needs_the_name() {
    let doc = ProfileDoc::empty_derived("r2-select-other", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, copy) = insert(doc, placed(Operand::Node(b)));
    let source = doc.copied_read(copy).expect("a source");
    assert_eq!(doc.var_name(source), None);
    let refused = raw_insert(
        &doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(Operand::Var(source), vec![top(b)]),
        },
    );
    assert!(
        matches!(refused, Err(EditError::SharedVarNeedsName { ref var }) if var.id() == source),
        "{:?}",
        refused.map(|_| ())
    );
}

/// A cycle through a source of a source: `bracket = B`, `spare =
/// bracket`, `C = place(spare)`, `F = C ∪ D`; `bracket = F` refuses.
#[test]
fn probe_a_cycle_through_a_source_of_a_source_is_refused() {
    let doc = ProfileDoc::empty_derived("r2-cycle-chain", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("bracket"),
            def: VarDecl::source(Operand::Node(b)),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("spare"),
            def: VarDecl::source(Operand::Name(VarName::from_static("bracket"))),
        },
    );
    let (doc, c) = insert(doc, placed(Operand::Name(VarName::from_static("spare"))));
    let (doc, d) = insert(doc, placed(Operand::Node(d)));
    let (doc, f) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                Operand::Node(c).into(),
                Operand::Node(d).into(),
            ]),
        },
    );
    let refused = doc.apply(
        &DocEdit::DefineVar {
            var: VarName::from_static("bracket").into(),
            def: VarDecl::source(Operand::Node(f)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(EditError::WouldCycle { .. })),
        "{:?}",
        refused.map(|_| ())
    );
}

/// The load refuses what the door refuses: a file whose second
/// placement reads the first one's anonymous source (two readers of an
/// unnamed variable), and a file whose source is redefined onto what its
/// own copy feeds (a cycle through nodes).
#[test]
fn probe_the_load_refuses_a_shared_anonymous_source_and_a_source_cycle() {
    let doc = ProfileDoc::empty_derived("r2-load", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, c1) = insert(doc, placed(Operand::Node(b)));
    let (doc, c2) = insert(doc, placed(Operand::Node(b)));
    let (doc, dd) = insert(doc, placed(Operand::Node(d)));
    let (doc, f) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                Operand::Node(c1).into(),
                Operand::Node(dd).into(),
            ]),
        },
    );
    let [s1, s2] = [c1, c2].map(|c| doc.copied_read(c).expect("a source"));
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
    let spell = |var: VarId| serde_json::to_string(&var).expect("an id spells");
    // Shared anonymous source.
    let needle = format!("\"source\": {}", spell(s2));
    assert!(text.contains(&needle));
    let shared = text.replace(&needle, &format!("\"source\": {}", spell(s1)));
    let refused = editor_core::load(&shared, Tol::witness());
    assert!(
        matches!(
            refused,
            Err(PersistError::Snapshot(
                SnapshotError::SharedVarNeedsName { .. } | SnapshotError::AnonymousVarUnread { .. }
            ))
        ),
        "shared: {:?}",
        refused.map(|_| ())
    );
    // A cycle through nodes: s1 = F.
    let def = format!("\"Source\": {}", spell(out(&doc, b)));
    assert_eq!(text.matches(&def).count(), 2, "both sources read B: {text}");
    let cyc = text.replacen(&def, &format!("\"Source\": {}", spell(out(&doc, f))), 1);
    let refused = editor_core::load(&cyc, Tol::witness());
    assert!(
        matches!(
            refused,
            Err(PersistError::Snapshot(SnapshotError::ReadCycle { .. }))
        ),
        "cycle: {:?}",
        refused.map(|_| ())
    );
}

/// A member placement keeps its index on the placement and reads a
/// source of the family (`m = place(S[0])`, `S = row`). Redefining `S`
/// as a read of a plain body leaves `m` reading member 0 of a body. The
/// door should refuse that redefinition (or the load the file it
/// writes); asserts one of them does, and that the result evaluates
/// otherwise.
#[test]
fn probe_redefining_a_family_source_onto_a_body_is_refused() {
    let doc = ProfileDoc::empty_derived("r2-member-source", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let (doc, m) = crate::fixture::member(doc, row, 0);
    let s = doc.copied_read(m).expect("a source");
    assert!(
        doc.source(s).is_some_and(|body| !body.is_indexed()),
        "the source reads the family whole: {:?}",
        doc.source(s)
    );
    let redefined = doc.apply(
        &DocEdit::DefineVar {
            var: s.into(),
            def: VarDecl::source(Operand::Node(d)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    let Ok(applied) = redefined else {
        return; // the door refuses: fine
    };
    let saved = editor_core::save(&applied.doc, &[], Tol::witness());
    let loaded = saved
        .as_ref()
        .map(|text| editor_core::load(text, Tol::witness()).map(|_| ()));
    let ev = crate::blocks::run(&applied.doc);
    panic!(
        "the door admitted S = D under a member read S[0]; save/load: {:?}; evaluation of m: {:?}",
        loaded,
        crate::blocks::failure(&ev, m)
    );
}

/// The converse: `bracket = B`, `c = place(bracket)`, a shell of `c`;
/// redefining `bracket` as a read of a family makes `c` a family under a
/// reader that takes one body. The door should refuse, or what it
/// writes should save.
#[test]
fn probe_redefining_a_body_source_onto_a_family_is_refused_or_saves() {
    let doc = ProfileDoc::empty_derived("r2-family-onto", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("bracket"),
            def: VarDecl::source(Operand::Node(b)),
        },
    );
    let (doc, c) = insert(doc, placed(Operand::Name(VarName::from_static("bracket"))));
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(Operand::Node(c), vec![top(b)]),
        },
    );
    assert!(crate::blocks::failure(&crate::blocks::run(&doc), shell).is_none());
    let redefined = doc.apply(
        &DocEdit::DefineVar {
            var: VarName::from_static("bracket").into(),
            def: VarDecl::source(Operand::Node(row)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    let Ok(applied) = redefined else {
        return;
    };
    let saved = editor_core::save(&applied.doc, &[], Tol::witness());
    let loaded = saved
        .as_ref()
        .map(|text| editor_core::load(text, Tol::witness()).map(|_| ()));
    assert!(
        matches!(loaded, Ok(Ok(()))),
        "the door admitted bracket = a family; save/load: {loaded:?}"
    );
}
