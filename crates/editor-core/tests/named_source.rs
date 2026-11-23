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
    let text = editor_core::save(&doc, Tol::witness()).expect("saves");
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
            members: editor_core::Bodies::spelled(vec![
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

// ---- review rows (PR 6856's dual review), pinning the fixed behaviour ----

fn redefine(doc: &ProfileDoc, var: VarId, decl: VarDecl) -> Result<ProfileDoc, EditError> {
    doc.apply(
        &DocEdit::DefineVar {
            var: var.into(),
            def: decl,
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .map(|applied| applied.doc)
}

/// `a = B`, `b = a`: the document a source cycle is closed in.
fn chained() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("named-source-chained", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "a", VarDecl::source(Operand::Node(b)));
    let doc = declared(
        doc,
        "b",
        VarDecl::source(Operand::Name(VarName::from_static("a"))),
    );
    (doc, b)
}

/// **A cycle through sources alone is a definition cycle** (VR3), refused
/// at the door — `a = b` over `b = a`, and `a = a` — never a recursion
/// without end.
///
/// *Breaks if* the definition graph leaves sources out.
#[test]
fn a_source_cycle_is_a_definition_cycle() {
    let (doc, _) = chained();
    let a = doc.var_named("a").expect("a");
    for read in ["b", "a"] {
        let refused = redefine(
            &doc,
            a,
            VarDecl::source(Operand::Name(VarName::from_static(read))),
        );
        assert!(
            matches!(refused, Err(EditError::DefinitionCycle { .. })),
            "a = {read}: {:?}",
            refused.map(|_| ())
        );
    }
}

/// **The load refuses a source cycle**: a file holding `a = b`, `b = a`.
#[test]
fn the_load_refuses_a_source_cycle() {
    let (doc, b) = chained();
    let bb = doc.var_named("b").expect("b");
    let text = editor_core::save(&doc, Tol::witness()).expect("saves");
    let spell = |var: VarId| serde_json::to_string(&var).expect("an id spells");
    let needle = format!("\"Source\": {}", spell(out(&doc, b)));
    assert_eq!(text.matches(&needle).count(), 1, "a's definition, once");
    let edited = text.replace(&needle, &format!("\"Source\": {}", spell(bb)));
    let refused = editor_core::load(&edited, Tol::witness());
    assert!(
        matches!(
            refused,
            Err(PersistError::Snapshot(
                SnapshotError::DefinitionCycle { .. }
            ))
        ),
        "{:?}",
        refused.map(|_| ())
    );
}

/// **A cycle through a source of a source refuses across nodes**:
/// `bracket = B`, `spare = bracket`, `C = place(spare)`, `F = C ∪ D`, then
/// `bracket = F`.
#[test]
fn a_cycle_through_a_source_of_a_source_is_refused() {
    let doc = ProfileDoc::empty_derived("named-source-cycle-chain", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "bracket", VarDecl::source(Operand::Node(b)));
    let doc = declared(
        doc,
        "spare",
        VarDecl::source(Operand::Name(VarName::from_static("bracket"))),
    );
    let (doc, c) = insert(doc, placed(Operand::Name(VarName::from_static("spare"))));
    let (doc, d) = insert(doc, placed(Operand::Node(d)));
    let (doc, f) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::spelled(vec![
                Operand::Node(c).into(),
                Operand::Node(d).into(),
            ]),
        },
    );
    let bracket = doc.var_named("bracket").expect("bracket");
    let refused = redefine(&doc, bracket, VarDecl::source(Operand::Node(f)));
    assert!(
        matches!(refused, Err(EditError::WouldCycle { .. })),
        "{:?}",
        refused.map(|_| ())
    );
}

/// **A placement passed a body mints a source by one rule** (VR9): the
/// body's node, its output by id, and a named output alike; a source
/// passed is shared.
///
/// *Breaks if* `Node::place` and the door disagree on a variable read.
#[test]
fn a_body_by_any_spelling_mints_a_source() {
    let doc = ProfileDoc::empty_derived("named-source-spellings", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let body = out(&doc, b);
    let (doc, _) = step(
        doc,
        DocEdit::RenameVar {
            var: body.into(),
            name: Some(VarName::from_static("plate")),
        },
    );
    for read in [
        Operand::Node(b),
        Operand::Var(body),
        Operand::Name(VarName::from_static("plate")),
        Operand::output(b, 0),
    ] {
        let doc = insert_placed(&doc, placed(read.clone()))
            .unwrap_or_else(|error| panic!("{read:?}: {error:?}"));
        let copy = *doc.ids().last().expect("the placement");
        let source = doc.copied_read(copy).expect("a source");
        assert_eq!(
            doc.source(source).map(|read| read.read),
            Some(body),
            "{read:?}: a source minted, reading the body"
        );
    }
}

/// **A redefinition is held to every seat reading the source**: `m =
/// place(S[0])` reads member 0 of a source of the family `S`; redefining
/// `S` onto one body would leave `m` reading a member of no family, which
/// the door refuses as the load would, and the takeover door with it.
#[test]
fn a_family_source_cannot_be_redefined_onto_one_body() {
    let doc = ProfileDoc::empty_derived("named-source-member", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let (doc, m) = crate::fixture::member(doc, row, 0);
    let s = doc.copied_read(m).expect("a source");
    assert!(
        doc.source(s).is_some_and(|body| !body.is_indexed()),
        "the source reads the family whole"
    );
    let refused = redefine(&doc, s, VarDecl::source(Operand::Node(d)));
    assert!(
        matches!(
            refused,
            Err(EditError::SlotVarKind {
                slot: SlotId::Operand(OperandSlot::Source),
                ..
            })
        ),
        "{:?}",
        refused.map(|_| ())
    );
    let refused = editor_core::take_over(
        &doc,
        &[s],
        &Operand::Node(d),
        &editor_core::EvalOptions::default(),
        &editor_core::CancelToken::new(),
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(EditError::SlotVarKind { .. })),
        "the takeover: {:?}",
        refused.map(|_| ())
    );
}

/// **A body source redefined onto a family is held to its readers**:
/// `bracket = B`, `c = place(bracket)`, a shell of `c`; `bracket = row`
/// would hand the placement a family its binder yields, and a placement's
/// source takes one body (D10 `[repetition]`), so the door refuses by kind
/// and the document stays one the load takes.
#[test]
fn a_body_source_redefined_onto_a_family_is_refused_at_its_placement() {
    let doc = ProfileDoc::empty_derived("named-source-family-onto", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "bracket", VarDecl::source(Operand::Node(b)));
    let (doc, c) = insert(doc, placed(Operand::Name(VarName::from_static("bracket"))));
    let top = StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let (doc, _) = insert(
        doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(Operand::Node(c), vec![top]),
        },
    );
    let bracket = doc.var_named("bracket").expect("bracket");
    let refused = redefine(&doc, bracket, VarDecl::source(Operand::Node(row)))
        .map(|_| ())
        .expect_err("a family at a placement's source is refused");
    assert!(
        matches!(
            refused,
            editor_core::EditError::SlotVarKind {
                found: editor_core::VarKind::Bodies,
                ..
            }
        ),
        "refused by kind at the placement: {refused:?}"
    );
    let text = editor_core::save(&doc, Tol::witness()).expect("saves");
    editor_core::load(&text, Tol::witness()).expect("the document as it stands loads");
}

/// **The reader count, reading (a)** (VR2): a lone placement whose mate
/// `on` side is a plane of a face selected in its own anonymous source has
/// one reader, so naming that `on` side's pose is admitted; another node
/// selecting in the same source is a second reader.
#[test]
fn a_placements_own_on_sides_are_its_reading_and_no_other_selection_is() {
    use crate::fixture::{bundle, flip, floor, fname, mate, plane, squared, stood, write_bundle};
    let doc = ProfileDoc::empty_derived("named-source-qualifier", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, ground, _) = floor(doc, Vec::new());
    let (doc, copy) = insert(doc, stood(Operand::Node(b)));
    let source = doc.copied_read(copy).expect("a source");
    let bottom = fname(b, RoleSeg::Cap(CapEnd::Start));
    let m = mate(
        plane(Operand::select(Operand::Var(source), vec![bottom])),
        flip(ground),
    );
    let placed_by = squared(&doc, m, &[]);
    let (doc, _) = write_bundle(doc, copy, placed_by.bundle());
    assert_eq!(doc.var_name(source), None, "the source stays anonymous");
    let on = bundle(&doc, copy)
        .iter()
        .find_map(|c| c.reads().into_iter().next().map(|(_, &read)| read))
        .expect("the mate's on side");
    let renamed = doc.apply(
        &DocEdit::RenameVar {
            var: on.into(),
            name: Some(VarName::from_static("ax")),
        },
        Tol::witness(),
    );
    assert!(renamed.is_ok(), "{:?}", renamed.map(|_| ()));
    let top = StableName {
        kind: EntityKind::Face,
        node: b,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let refused = insert_placed(
        &doc,
        Node::Shell {
            thickness: len(0.1),
            open: Operand::select(Operand::Var(source), vec![top]),
        },
    );
    assert!(
        matches!(refused, Err(EditError::SharedVarNeedsName { ref var }) if var.id() == source),
        "{:?}",
        refused.map(|_| ())
    );
}

/// **The load refuses what the door refuses**: a second placement reading
/// the first one's anonymous source, and a source redefined onto what its
/// own copy feeds.
#[test]
fn the_load_refuses_a_shared_anonymous_source_and_a_cycle_through_nodes() {
    let doc = ProfileDoc::empty_derived("named-source-load-both", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, c1) = insert(doc, placed(Operand::Node(b)));
    let (doc, c2) = insert(doc, placed(Operand::Node(b)));
    let (doc, dd) = insert(doc, placed(Operand::Node(d)));
    let (doc, f) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::spelled(vec![
                Operand::Node(c1).into(),
                Operand::Node(dd).into(),
            ]),
        },
    );
    let [s1, s2] = [c1, c2].map(|c| doc.copied_read(c).expect("a source"));
    let text = editor_core::save(&doc, Tol::witness()).expect("saves");
    let spell = |var: VarId| serde_json::to_string(&var).expect("an id spells");
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
    let def = format!("\"Source\": {}", spell(out(&doc, b)));
    assert_eq!(text.matches(&def).count(), 2, "both sources read B");
    let cycle = text.replacen(&def, &format!("\"Source\": {}", spell(out(&doc, f))), 1);
    let refused = editor_core::load(&cycle, Tol::witness());
    assert!(
        matches!(
            refused,
            Err(PersistError::Snapshot(SnapshotError::ReadCycle { .. }))
        ),
        "cycle: {:?}",
        refused.map(|_| ())
    );
}

/// **A source written in place at a family's own indices is stored as
/// written** (`mint_source`, REFERENCES DM3): `place(source(row[k]))`, `k`
/// the index the row's binder mints, reads one member at each value of
/// `k`; the plain read would be the family whole.
#[test]
fn a_source_at_the_familys_own_indices_is_stored_as_written() {
    let doc = ProfileDoc::empty_derived("named-source-own-index", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, k, row) = crate::fixture::row(doc, b, ("k", 2), editor_core::TargetAxis::X, 2.0);
    let k = editor_core::Formula::var(k, editor_core::Dimension::Count);
    let (doc, copy) = insert(
        doc,
        placed(Operand::source(editor_core::BodyRead::indexed(
            Operand::Node(row),
            vec![k],
        ))),
    );
    let source = doc.copied_read(copy).expect("a source");
    let at = doc
        .source(source)
        .map(|body| body.at.clone())
        .expect("the source reads the row");
    assert_eq!(
        doc.source(source).map(|body| body.read),
        Some(out(&doc, row)),
        "the source reads the row"
    );
    assert_eq!(at.len(), 1, "at one index, as written: {at:?}");
}

/// **The takeover door refuses what is no source**, in its own words.
#[test]
fn the_takeover_refuses_what_is_no_source() {
    let doc = ProfileDoc::empty_derived("named-source-not-a-source", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let refused = editor_core::take_over(
        &doc,
        &[out(&doc, b)],
        &Operand::Node(b),
        &editor_core::EvalOptions::default(),
        &editor_core::CancelToken::new(),
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(EditError::TakeOverNotASource { ref var }) if var.id() == out(&doc, b)),
        "{:?}",
        refused.map(|_| ())
    );
}

/// **The product slot refuses one member of a family** (`Member`): a member
/// is a body (D10 `[repetition]`), and a placement reads `Source(row[1])`,
/// but the product's gather does not read one member. Written in place,
/// declared and read by name, or reached by redefining a source the slot
/// already reads (held to the load's rule), the doors refuse it alike, so
/// no door leaves a slot the gather's own member arm answers.
#[test]
fn the_product_slot_refuses_one_member_of_a_family() {
    let doc = ProfileDoc::empty_derived("named-source-member-product", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 3), editor_core::TargetAxis::X, 2.0);
    let member =
        || editor_core::BodyRead::indexed(Operand::Node(row), vec![editor_core::Formula::count(1)]);
    let set = |d: &ProfileDoc, read: Operand| {
        d.apply(
            &DocEdit::SetProduct {
                product: Some(read),
            },
            Tol::witness(),
        )
        .map(|applied| applied.doc)
    };
    let is_member = |got: &Result<ProfileDoc, EditError>| {
        matches!(
            got,
            Err(EditError::ProductRead {
                fault: editor_core::ProductReadFault::Member,
                ..
            })
        )
    };
    assert!(
        insert_placed(&doc, placed(Operand::Source(Box::new(member())))).is_ok(),
        "a placement reads one member"
    );
    let got = set(&doc, Operand::Source(Box::new(member())));
    assert!(is_member(&got), "in place: {:?}", got.map(|_| ()));
    let named = declared(doc.clone(), "s", VarDecl::source(member()));
    let got = set(&named, Operand::Name(VarName::from_static("s")));
    assert!(is_member(&got), "by name: {:?}", got.map(|_| ()));
    let (doc, copy) = insert(doc, placed(Operand::Node(b)));
    let whole = declared(doc, "t", VarDecl::source(Operand::Node(copy)));
    let t = whole.var_named("t").expect("t");
    let whole = set(&whole, Operand::Var(t)).expect("a body in the world");
    // A source's redefinition holds each reader to the load's rule.
    let got = redefine(&whole, t, VarDecl::source(member()));
    let held = match &got {
        Err(EditError::SourceReaderRefused { reader, .. }) => matches!(
            **reader,
            PersistError::Snapshot(SnapshotError::ProductRead {
                fault: editor_core::ProductReadFault::Member,
                ..
            })
        ),
        _ => false,
    };
    assert!(held, "redefined under the slot: {:?}", got.map(|_| ()));
}

/// **What reads a body is the inverse of what each reader reads**: two
/// copies of `B`, one through `spare = bracket`, `bracket = B`, and a
/// third placed into `B`'s own space. `Doc::read_by(B)` lists the two
/// copies at the source seat and the third at the space seat, each
/// reading `B` in `Doc::operand_reads`; `Doc::placements_of(B)` is the
/// two copies, and `Doc::placements_of_var` finds the one through
/// `spare` from either source it passes through.
///
/// *Breaks if* a space seat stops reaching its definer, a read down a
/// chain of sources stops reaching the body, or the two directions
/// disagree.
#[test]
fn read_by_inverts_operand_reads_at_source_and_space_seats() {
    let doc = ProfileDoc::empty_derived("read-by-inverts", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let doc = declared(doc, "bracket", VarDecl::source(Operand::Node(b)));
    let doc = declared(
        doc,
        "spare",
        VarDecl::source(Operand::Name(VarName::from_static("bracket"))),
    );
    let (doc, first) = insert(doc, placed(Operand::Node(b)));
    let (doc, through) = insert(doc, placed(Operand::Name(VarName::from_static("spare"))));
    let own = doc.space_of(out(&doc, b)).expect("B is in its own space");
    let (doc, standing) = insert(
        doc,
        Node::place(
            editor_core::BodyRead::plain(Operand::Node(first)),
            Operand::Var(own),
            editor_core::stand().bundle(),
        ),
    );
    assert_eq!(
        doc.read_by(b),
        [
            (first, OperandSlot::Source),
            (through, OperandSlot::Source),
            (standing, OperandSlot::Space),
        ],
        "both copies, and the placement standing in B's space"
    );
    for (reader, seat) in doc.read_by(b) {
        assert!(
            doc.operand_reads(reader)
                .iter()
                .any(|read| read.seat == seat && read.reaches.contains(&b)),
            "{reader:?} reads B at {seat:?} in its own operand reads"
        );
    }
    assert_eq!(doc.placements_of(b), [first, through], "the copies of B");
    assert_eq!(doc.placements_of(first), [standing], "a copy of the copy");
    for name in ["spare", "bracket"] {
        let var = doc.var_named(name).expect("declared");
        assert_eq!(
            doc.placements_of_var(var),
            [through],
            "the copy reading spare reads {name}"
        );
    }
    assert_eq!(
        doc.placements_of_var(out(&doc, b)),
        [first, through],
        "B's body, directly and down both sources"
    );
}

/// **A copy of a member of a named family source is a copy of the body
/// the family reads, by both placement queries**: `xs = Row`, `m =
/// xs[1]`, `Place(m)`. The copy reads Row's member, so it is among Row's
/// copies whether asked by the node or by its body.
///
/// *Breaks if* the variable walk stops at the family source `xs` and
/// misses the body below it, or the two queries part.
#[test]
fn a_copy_of_a_named_familys_member_copies_the_body_the_family_reads() {
    let doc = ProfileDoc::empty_derived("member-of-a-named-family", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = crate::fixture::row(doc, b, ("k", 3), editor_core::TargetAxis::X, 2.0);
    let doc = declared(doc, "xs", VarDecl::source(Operand::Node(row)));
    let doc = declared(
        doc,
        "m",
        VarDecl::source(editor_core::BodyRead::indexed(
            Operand::Name(VarName::from_static("xs")),
            vec![editor_core::Formula::count(1)],
        )),
    );
    let (doc, copy) = insert(doc, placed(Operand::Name(VarName::from_static("m"))));
    assert_eq!(doc.placements_of(row), [copy], "a copy of Row's member");
    assert_eq!(
        doc.placements_of_var(out(&doc, row)),
        doc.placements_of(row),
        "asked by Row's body, the same answer"
    );
    let xs = doc.var_named("xs").expect("declared");
    assert_eq!(doc.placements_of_var(xs), [copy], "and a copy of xs");
}
