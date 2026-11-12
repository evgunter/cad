//! **A placement reads its body through a source** (D10
//! `[placement.placement]`, VARIABLES VR4): a `Body` variable defined as
//! a read of a body (`bracket = B`), which the usual naming rule governs.
//! The door and the load refuse a placement reading a body directly; the
//! façade's `place(B, …)` mints an anonymous source per call; a source may
//! read another source; redefining one is a definition edit whose cycle
//! check spans the nodes its placements feed; and a delete hands a
//! source's definition back as it hands any reader.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture::{insert, len, out, step};

use editor_core::{
    CapEnd, DocEdit, EditError, EntityKind, Node, Operand, OperandSlot, PersistError, ProfileDoc,
    RecipeNodeId, RoleSeg, SlotId, SnapshotError, StableName, VarDecl, VarId, VarName,
};
use geom_core::Tol;

/// A placement of `read` into the world by the identity bundle, as
/// authored: the façade's spelling.
fn placed(read: Operand) -> editor_core::AuthoredNode {
    Node::place(
        editor_core::BodyRead::plain(read),
        Operand::Var(VarId::WORLD),
        editor_core::stand().bundle(),
    )
}

/// The placement of `read` into the world inserted through the door as
/// written, or the door's refusal.
fn insert_placed(
    doc: &ProfileDoc,
    node: editor_core::AuthoredNode,
) -> Result<ProfileDoc, EditError> {
    doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .map(|applied| applied.doc)
}

/// The body `copy` copies, down its source.
fn copied(doc: &ProfileDoc, copy: RecipeNodeId) -> VarId {
    doc.through_sources(doc.copied_read(copy).expect("a source"))
        .read
}

fn declared(doc: ProfileDoc, name: &'static str, decl: VarDecl) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static(name),
            def: decl,
        },
    )
    .0
}

/// **The façade's `place(B, …)` mints an anonymous source per call**, and
/// a placement of the source shares it only once it is named (VR2).
///
/// *Breaks if* a placement stores the body's output, two calls share a
/// source, or an anonymous source takes a second reader.
#[test]
fn place_mints_a_source_per_call_and_a_name_shares_it() {
    let doc = ProfileDoc::empty_derived("named-source-mint", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, first) = insert(doc, placed(Operand::Node(b)));
    let (doc, second) = insert(doc, placed(Operand::Node(b)));
    let [s1, s2] = [first, second].map(|copy| doc.copied_read(copy).expect("a source"));
    assert_ne!(s1, s2, "two calls, two sources");
    for source in [s1, s2] {
        assert_eq!(
            doc.source(source).map(|read| read.read),
            Some(out(&doc, b)),
            "each reads B"
        );
        assert_eq!(doc.var_name(source), None, "and is anonymous");
    }
    let shared = insert_placed(&doc, placed(Operand::Var(s1)));
    assert!(
        matches!(shared, Err(EditError::SharedVarNeedsName { ref var }) if var.id() == s1),
        "an anonymous source takes no second reader: {shared:?}"
    );
    let doc = step(
        doc,
        DocEdit::RenameVar {
            var: s1.into(),
            name: Some(VarName::from_static("bracket")),
        },
    )
    .0;
    let (doc, third) = insert(doc, placed(Operand::Name(VarName::from_static("bracket"))));
    assert_eq!(
        doc.copied_read(third),
        Some(s1),
        "a named source is shared by reading it"
    );
}

/// **The door refuses a placement reading its body directly**, at the
/// insert and at the slot door alike.
///
/// *Breaks if* a direct read is stored.
#[test]
fn the_door_refuses_a_direct_read() {
    let doc = ProfileDoc::empty_derived("named-source-direct", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let direct = Node::Place {
        source: editor_core::BodyRead::plain(Operand::Var(out(&doc, b))),
        target: Operand::Var(VarId::WORLD),
        constraints: editor_core::stand().bundle(),
    };
    let refused = insert_placed(&doc, direct);
    assert!(
        matches!(refused, Err(EditError::PlacementReadsNoSource { ref read, .. }) if read.id() == out(&doc, b)),
        "{refused:?}"
    );
    let (doc, copy) = insert(doc, placed(Operand::Node(b)));
    let refused = doc.apply(
        &DocEdit::SetParam {
            node: copy,
            slot: SlotId::Operand(OperandSlot::Source),
            value: Operand::Var(out(&doc, b)).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(EditError::PlacementReadsNoSource { .. })),
        "{refused:?}"
    );
}

/// **The load refuses a placement reading its body directly**: a file
/// whose placement names the body's output in its source slot.
///
/// *Breaks if* the load check admits what the door refuses.
#[test]
fn the_load_refuses_a_direct_read() {
    let doc = ProfileDoc::empty_derived("named-source-load", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, copy) = insert(doc, placed(Operand::Node(b)));
    let source = doc.copied_read(copy).expect("a source");
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
    let spell = |var: VarId| serde_json::to_string(&var).expect("an id spells");
    let needle = format!("\"source\": {}", spell(source));
    assert!(text.contains(&needle), "the file spells the source read");
    let edited = text.replace(&needle, &format!("\"source\": {}", spell(out(&doc, b))));
    let refused = editor_core::load(&edited, Tol::witness());
    assert!(
        matches!(
            refused,
            Err(PersistError::Snapshot(
                SnapshotError::PlacementReadsNoSource { .. }
            ))
        ),
        "{:?}",
        refused.map(|_| ())
    );
}

/// **A source may read another source, and follows it**: `spare =
/// bracket`, `bracket = B`; a copy of `spare` shows `B`, and once `bracket
/// = D` it shows `D`.
///
/// *Breaks if* a source of a source reads the first one's old body, or
/// refuses.
#[test]
fn a_source_of_a_source_follows_its_redefinition() {
    let doc = ProfileDoc::empty_derived("named-source-chain", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "bracket", VarDecl::source(Operand::Node(b)));
    let doc = declared(
        doc,
        "spare",
        VarDecl::source(Operand::Name(VarName::from_static("bracket"))),
    );
    let (doc, copy) = insert(doc, placed(Operand::Name(VarName::from_static("spare"))));
    assert_eq!(copied(&doc, copy), out(&doc, b), "spare shows B");
    let (doc, _) = step(
        doc,
        DocEdit::DefineVar {
            var: VarName::from_static("bracket").into(),
            def: VarDecl::source(Operand::Node(d)),
            fresh: Vec::new(),
        },
    );
    assert_eq!(
        copied(&doc, copy),
        out(&doc, d),
        "spare follows bracket = D"
    );
    let ev = crate::blocks::run(&doc);
    assert!(
        crate::blocks::failure(&ev, copy).is_none(),
        "the copy evaluates: {:?}",
        crate::blocks::failure(&ev, copy)
    );
}

/// **Redefining a source onto what its own copy feeds is a cycle** that
/// the definition door refuses across nodes: `bracket = F`, `F = C ∪ D`,
/// `C = place(bracket)`, `D` another body's copy beside it.
///
/// *Breaks if* the cycle check looks at definitions alone.
#[test]
fn a_redefinition_through_its_own_copy_is_refused() {
    let doc = ProfileDoc::empty_derived("named-source-cycle", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "bracket", VarDecl::source(Operand::Node(b)));
    let (doc, c) = insert(doc, placed(Operand::Name(VarName::from_static("bracket"))));
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

/// **A delete hands a source's definition back** (DM6: a definition is a
/// reader): `bracket = F`, `F` a shell of `B`, which continues `B`;
/// deleting `F` redefines `bracket = B`, and the copy shows `B` again.
///
/// *Breaks if* the delete leaves the source reading the deleted shell, or
/// takes the copy with it.
#[test]
fn a_delete_hands_a_sources_definition_back() {
    let doc = ProfileDoc::empty_derived("named-source-delete", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "bracket", VarDecl::source(Operand::Node(b)));
    let (doc, copy) = insert(doc, placed(Operand::Name(VarName::from_static("bracket"))));
    let top = StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let (doc, f) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(Operand::Node(b), vec![top]),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::DefineVar {
            var: VarName::from_static("bracket").into(),
            def: VarDecl::source(Operand::Node(f)),
            fresh: Vec::new(),
        },
    );
    assert_eq!(copied(&doc, copy), out(&doc, f), "the copy shows F");
    let bracket = doc.var_named("bracket").expect("bracket");
    let splice = editor_core::continuation::splice(&doc, f)
        .expect("the delete splices")
        .expect("F is live");
    assert!(
        splice.edits.iter().any(|edit| matches!(
            edit,
            DocEdit::DefineVar { var: editor_core::VarRef::Id(var), def: VarDecl::Source(_), .. }
                if *var == bracket
        )),
        "bracket is handed back: {:?}",
        splice.edits
    );
    assert_eq!(splice.deleted, [f], "the copy stays");
    let mut run = editor_core::Recording::start(&doc, Tol::witness());
    splice.apply(&mut run).expect("the splice applies");
    let doc = run.finish().expect("the action finishes").doc;
    assert_eq!(copied(&doc, copy), out(&doc, b), "bracket = B again");
}
