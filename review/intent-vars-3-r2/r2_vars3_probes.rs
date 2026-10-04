//! Reviewer r2 probes for INTENT-VARS-1 PR 3 (not for merge).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::fixture::resolver::PartStore;
use crate::fixture::{desc, insert, len, on_frame, square};
use editor_core::{
    Dimension, DocEdit, DocumentId, EditError, Expr, ExtrudeSide, FreeVar, Maintenance, Node,
    ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, VarDef, VarId, VarName, apply, inline, load,
    save, split,
};
use geom_core::Tol;

fn n(name: &'static str) -> VarName {
    VarName::from_static(name)
}
fn named(name: &'static str) -> Expr {
    Expr::named(n(name), Dimension::Length)
}
fn try_step(
    doc: &ProfileDoc,
    edit: DocEdit<ProfileProgram>,
) -> Result<editor_core::Applied<ProfileProgram>, EditError> {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
}
fn step(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> editor_core::Applied<ProfileProgram> {
    try_step(doc, edit).expect("the edit applies")
}
fn declare_edit(name: &'static str, v: f64) -> DocEdit<ProfileProgram> {
    DocEdit::DeclareVar {
        name: n(name),
        def: VarDef::Free(FreeVar::continuous(Dimension::Length, v)),
    }
}
fn declare(doc: &ProfileDoc, name: &'static str, v: f64) -> ProfileDoc {
    step(doc, declare_edit(name, v)).doc
}
fn id(doc: &ProfileDoc, name: &str) -> VarId {
    doc.var_named(name).expect("declared")
}
fn slot(doc: &ProfileDoc, node: RecipeNodeId, s: SlotId) -> Expr {
    doc.node(node).and_then(|x| x.expr(s)).cloned().unwrap()
}
fn block(doc: ProfileDoc, cx: f64, depth: Expr) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let frame = doc.order()[doc.order().len() - 2];
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: depth,
            side: ExtrudeSide::Along,
        },
    );
    (doc, [frame, profile, extrude])
}

/// C7: inline re-points carried readers at the HOST's ids — both when
/// the name merges (bit_eq) and when the host declares it fresh.
#[test]
fn p1_inline_remaps_readers_to_host_ids() {
    let part = ProfileDoc::empty(DocumentId::derive("r2-part"), Tol::witness());
    let part = declare(&part, "d", 1.0);
    let part = declare(&part, "e", 0.5);
    let (part, [_, _, pe]) = block(part, 0.0, named("d"));
    let (part, [_, _, pe2]) = block(part, 4.0, named("e"));
    let _ = (pe, pe2);
    let mut store = PartStore::default();
    let doc_ref = store.insert(part.clone(), Tol::witness());
    let store: Arc<dyn editor_core::PartResolver> = Arc::new(store);

    let host = ProfileDoc::empty(DocumentId::derive("r2-host"), Tol::witness());
    let host = declare(&host, "pad", 9.0);
    let host = declare(&host, "d", 1.0); // merges with the part's d
    let (host, instance) = insert(host, Node::instantiate_part(doc_ref));
    let host_d = id(&host, "d");
    let out = inline(&host, instance, &store, Tol::witness()).expect("inline");
    let doc = out.doc;
    let host_e = id(&doc, "e");
    assert_ne!(host_e, id(&part, "e"), "the host mints its own e");
    assert_eq!(id(&doc, "d"), host_d, "merged by name");
    let mut seen = BTreeSet::new();
    for &nid in doc.order() {
        if let Some(Node::Extrude { distance, .. }) = doc.node(nid) {
            let mut reads = Vec::new();
            distance.var_reads(&mut reads);
            for (v, _) in reads {
                seen.insert(v);
            }
        }
    }
    assert_eq!(seen, BTreeSet::from([host_d, host_e]), "readers read host ids");
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(load(&text, Tol::witness()).expect("loads").doc.bit_eq(&doc));
}

/// C4/C5: an authored log that renames, re-declares the old name and
/// reads it again replays to the same ids, and saves/loads with that log.
#[test]
fn p2_authored_log_with_rename_and_redeclare_replays() {
    let id0 = DocumentId::derive("r2-log");
    let mut doc = ProfileDoc::empty(id0, Tol::witness());
    let mut log: Vec<DocEdit<ProfileProgram>> = Vec::new();
    let mut push = |doc: &mut ProfileDoc, e: DocEdit<ProfileProgram>| {
        *doc = step(doc, e.clone()).doc;
        log.push(e);
    };
    push(&mut doc, declare_edit("w", 1.0));
    let (next, profile) = on_frame(
        doc.clone(),
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let frame = next.order()[0];
    push(
        &mut doc,
        DocEdit::InsertNode {
            node: Box::new(next.node(frame).unwrap().clone()),
        },
    );
    push(
        &mut doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Profile(desc(frame, vec![square(0.0, 0.0, 0.5)]))),
        },
    );
    assert_eq!(doc.order()[1], profile);
    push(
        &mut doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile,
                distance: named("w"),
                side: ExtrudeSide::Along,
            }),
        },
    );
    let e1 = *doc.order().last().unwrap();
    let first = id(&doc, "w");
    push(
        &mut doc,
        DocEdit::RenameVar {
            var: n("w").into(),
            name: Some(n("v")),
        },
    );
    push(&mut doc, declare_edit("w", 2.0));
    let second = id(&doc, "w");
    push(
        &mut doc,
        DocEdit::SetParam {
            node: e1,
            slot: SlotId::Distance,
            expr: Expr::add(named("w"), Expr::named(n("v"), Dimension::Length)).unwrap(),
        },
    );
    let mut reads = Vec::new();
    slot(&doc, e1, SlotId::Distance).var_reads(&mut reads);
    assert_eq!(
        reads.iter().map(|r| r.0).collect::<Vec<_>>(),
        vec![second, first]
    );
    let replayed = ProfileDoc::replay(id0, &log, Tol::witness()).expect("replays");
    assert!(replayed.bit_eq(&doc));
    // The snapshot after the first insert-by-name, with the rest of the
    // authored log (rename, re-declare, re-read by name) after it.
    let at4 = ProfileDoc::replay(id0, &log[..4], Tol::witness()).unwrap();
    let text = save(&at4, &log[4..], Tol::witness()).expect("saves with its authored log");
    let loaded = load(&text, Tol::witness()).expect("loads");
    assert!(loaded.doc.bit_eq(&doc) || loaded.doc.bit_eq(&at4), "neither");
    eprintln!("loaded == final: {}", loaded.doc.bit_eq(&doc));
}

/// C5: an anonymous variable read twice survives losing one reader and
/// is removed (reported) with the second; a DeleteNode detaches too.
#[test]
fn p3_anonymous_var_two_readers() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-anon"), Tol::witness());
    let doc = declare(&doc, "w", 1.0);
    let (doc, [_, _, a]) = block(doc, 0.0, named("w"));
    let (doc, [_, _, b]) = block(doc, 4.0, named("w"));
    let w = id(&doc, "w");
    let doc = step(
        &doc,
        DocEdit::RenameVar {
            var: w.into(),
            name: None,
        },
    )
    .doc;
    let one = step(
        &doc,
        DocEdit::SetParam {
            node: a,
            slot: SlotId::Distance,
            expr: len(1.0),
        },
    );
    assert!(one.maintenance.is_empty(), "{:?}", one.maintenance);
    assert!(one.doc.var(w).is_some());
    let two = step(&one.doc, DocEdit::DeleteNode { id: b });
    assert!(
        two.maintenance
            .iter()
            .any(|m| matches!(m, Maintenance::AnonymousVarRemoved { var } if var.id() == w)),
        "{:?}",
        two.maintenance
    );
    assert!(two.doc.var(w).is_none() && two.doc.var_order().is_empty());
    // A declare after: var_order is just the new one.
    let three = declare(&two.doc, "w", 1.0);
    assert_eq!(three.var_order(), &[id(&three, "w")]);
    assert_ne!(id(&three, "w"), w);
}

/// C4: a name read at the wrong kind AND in a wrong-dimension slot —
/// established order (dimension first). And an id-authored reader of a
/// dead variable refuses SlotUnresolvedVar.
#[test]
fn p4_door_refusals_for_dead_and_unminted_ids() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-dead"), Tol::witness());
    let doc = declare(&doc, "w", 1.0);
    let (doc, [_, _, e]) = block(doc, 0.0, len(1.0));
    let w = id(&doc, "w");
    let gone = step(&doc, DocEdit::DeleteVar { var: w.into() }).doc;
    match try_step(
        &gone,
        DocEdit::SetParam {
            node: e,
            slot: SlotId::Distance,
            expr: Expr::var(w, Dimension::Length),
        },
    ) {
        Err(EditError::SlotUnresolvedVar { var, .. }) => assert_eq!(var.id(), w),
        other => panic!("{other:?}"),
    }
    match try_step(
        &gone,
        DocEdit::SetParam {
            node: e,
            slot: SlotId::Distance,
            expr: Expr::var(VarId(12345), Dimension::Length),
        },
    ) {
        Err(EditError::SlotUnresolvedVar { var, .. }) => assert_eq!(var.id(), VarId(12345)),
        other => panic!("{other:?}"),
    }
}

/// C7: split of a document whose cut reads a DELETED variable.
#[test]
fn p5_split_with_unresolved_reader_in_the_cut() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-split-dead"), Tol::witness());
    let doc = declare(&doc, "h", 1.5);
    let (doc, cut) = block(doc, 0.0, named("h"));
    let (doc, _) = block(doc, 10.0, len(1.0));
    let doc = step(&doc, DocEdit::DeleteVar { var: n("h").into() }).doc;
    let r = split(
        &doc,
        &BTreeSet::from(cut),
        DocumentId::derive("r2-part2"),
        Tol::witness(),
        None,
    );
    eprintln!("split of an unresolved reader: {:?}", r.as_ref().map(|_| ()).map_err(|e| e.to_string()));
}

/// C2: an AnalyzedBox taken before a rename is accepted after it (its
/// equality ignores the spoken names), and the MC refusal it feeds
/// speaks the OLD name.
#[test]
fn p6_analyzed_box_speaks_a_stale_name_after_a_rename() {
    use editor_core::analysis::{AnalysisPolicy, analyzed_box};
    let doc = ProfileDoc::empty(DocumentId::derive("r2-mc"), Tol::witness());
    let doc = declare(&doc, "w", 1.0);
    let (doc, _) = block(doc, 0.0, named("w"));
    let doc = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: n("w").into(),
            distribution: Some(editor_core::Distribution::Band { lo: -0.01, hi: 0.01 }),
        },
    )
    .doc;
    let policy = AnalysisPolicy::default();
    let bx = analyzed_box(&doc, &policy);
    let renamed = step(
        &doc,
        DocEdit::RenameVar {
            var: n("w").into(),
            name: Some(n("v")),
        },
    )
    .doc;
    assert_eq!(bx, analyzed_box(&renamed, &policy), "the box is 'the same'");
    let r = editor_core::mc::monte_carlo(
        &renamed,
        &bx,
        &editor_core::mc::McConfig::default(),
        Tol::witness(),
    );
    let text = format!("{}", r.as_ref().err().expect("a band refuses"));
    eprintln!("R2 MC refusal on the renamed doc: {text}");
    assert!(text.contains(" v") || !text.contains(" w"), "stale name: {text}");
}
