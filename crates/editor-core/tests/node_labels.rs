//! Node labels (DESIGN.md Band 1, "Node labels"): document data beside
//! the node — set and cleared by one edit, outside the mint and every
//! content key, dropped with the node, carried by split and inline,
//! saved, pinned, and said by the spoken node.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;
use std::sync::Arc;

use editor_core::{
    CancelToken, DocEdit, DocumentId, EditError, EvalOptions, InlineError, Label, LoggedEdit,
    Maintenance, Node, PersistError, ProfileDoc, RecipeNodeId, RootFault, SitedRef, SnapshotError,
    SplitError, content_pin, evaluate, inline, load, save, split,
};
use fixture::resolver::PartStore;
use fixture::{die, insert, len, on_frame, square, step};
use geom_core::Tol;
use test_utils::refusal::tag;

fn label(text: &str) -> Label {
    Label::new(text).expect("a valid label")
}

fn set_label(doc: ProfileDoc, node: RecipeNodeId, text: Option<&str>) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetLabel {
            node,
            label: text.map(label),
        },
    )
    .0
}

fn refusal(doc: &ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>) -> EditError {
    editor_core::apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
        .expect_err("the edit refuses")
}

/// A square block on its own sketch frame, `cx` along x: the frame,
/// the profile and the extrude, in the order inserted.
fn block(doc: ProfileDoc, cx: f64) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let before = doc.order().len();
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let frame = doc.order()[before];
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, [frame, profile, extrude])
}

/// A rename recomputes nothing and moves no id: the evaluation memo
/// reuses every node, the mint chain is untouched (so the next insert
/// draws the id it would have drawn unlabelled), and only the content
/// pin — which answers "which version" — moves.
#[test]
fn a_rename_recomputes_nothing_and_mints_nothing() {
    let d = die();
    let tol = Tol::witness();
    let opts = EvalOptions::default();
    let ev0 = evaluate::<f64>(&d.doc, None, &CancelToken::new(), &opts, tol);
    let labelled = set_label(d.doc.clone(), d.final_node, Some("die body"));
    let ev1 = evaluate::<f64>(&labelled, Some(&ev0), &CancelToken::new(), &opts, tol);
    assert_eq!(ev1.recomputed, 0, "a label edit recomputes no node");
    let renamed = set_label(labelled.clone(), d.final_node, Some("the die"));
    let ev2 = evaluate::<f64>(&renamed, Some(&ev1), &CancelToken::new(), &opts, tol);
    assert_eq!(ev2.recomputed, 0, "a rename recomputes no node");
    let diff = labelled.diff(&renamed);
    assert_eq!(
        (diff.nodes.len(), diff.labels),
        (0, vec![d.final_node]),
        "a rename changes no node, only its label"
    );

    assert_eq!(renamed.mint(), d.doc.mint(), "a label edit mints nothing");
    let frame = fixture::xy_frame();
    let (_, after_rename) = insert(renamed.clone(), frame.clone());
    let (_, unlabelled) = insert(d.doc.clone(), frame);
    assert_eq!(
        after_rename, unlabelled,
        "the next insert draws the same id with or without the label"
    );

    let pins = [&d.doc, &labelled, &renamed].map(|doc| content_pin(doc, tol).expect("pins"));
    assert_ne!(pins[0], pins[1], "a label is in the content pin");
    assert_ne!(pins[1], pins[2], "a rename moves the pin");
}

/// The edit's own doors: a dead node refuses, an edit that would leave
/// the label as it is refuses, a clear clears, a delete drops the
/// label, and two nodes may share one.
#[test]
fn set_label_refuses_a_dead_node_and_a_no_op_and_delete_drops_the_label() {
    let doc = ProfileDoc::empty_derived("node-labels-doors", Tol::witness());
    let (doc, [frame, profile, extrude]) = block(doc, 0.0);

    let doc = set_label(doc, extrude, Some("pin"));
    let doc = set_label(doc, profile, Some("pin"));
    assert_eq!(
        (doc.label(extrude), doc.label(profile), doc.label(frame)),
        (Some(&label("pin")), Some(&label("pin")), None),
        "two nodes may carry one label"
    );

    match refusal(
        &doc,
        DocEdit::SetLabel {
            node: extrude,
            label: Some(label("pin")),
        },
    ) {
        EditError::LabelUnchanged { node } => {
            assert_eq!(node, doc.spoken(extrude));
            assert_eq!(
                node.label(),
                Some(&label("pin")),
                "the label it already has"
            );
        }
        other => panic!("relabelling with the same text refuses LabelUnchanged, got {other:?}"),
    }
    match refusal(
        &doc,
        DocEdit::SetLabel {
            node: frame,
            label: None,
        },
    ) {
        EditError::LabelUnchanged { node } => {
            assert_eq!((node.id(), node.label()), (frame, None));
        }
        other => panic!("clearing no label refuses LabelUnchanged, got {other:?}"),
    }

    let cleared = set_label(doc.clone(), profile, None);
    assert_eq!(cleared.label(profile), None, "None clears");

    let (deleted, _) = step(doc, DocEdit::DeleteNode { id: extrude });
    assert_eq!(deleted.label(extrude), None, "the label dies with its node");
    assert_eq!(deleted.labels().len(), 1, "the other label stands");
    match refusal(
        &deleted,
        DocEdit::SetLabel {
            node: extrude,
            label: Some(label("back")),
        },
    ) {
        EditError::UnknownNode { id } => assert_eq!(id.id(), extrude),
        other => panic!("labelling a dead node refuses UnknownNode, got {other:?}"),
    }
}

/// The label is in the save file — as the snapshot's and as a logged
/// edit's — and an unlabelled document's file has no label section.
#[test]
fn a_label_survives_save_and_load_and_replays_from_the_log() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("node-labels-save", tol);
    let (doc, [_, _, extrude]) = block(doc, 0.0);
    let plain = save(&doc, &[], tol).expect("saves");
    assert!(
        !plain.contains("\"labels\""),
        "an unlabelled document writes no label section"
    );

    let labelled = set_label(doc.clone(), extrude, Some("base plate"));
    let text = save(&labelled, &[], tol).expect("saves");
    let loaded = load(&text, tol).expect("loads");
    assert_eq!(loaded.doc.label(extrude), Some(&label("base plate")));
    assert!(loaded.doc.bit_eq(&labelled), "the snapshot round-trips");

    let edits = [LoggedEdit {
        edit: DocEdit::SetLabel {
            node: extrude,
            label: Some(label("base plate")),
        },
        maintenance: Vec::new(),
    }];
    let logged = load(&save(&doc, &edits, tol).expect("saves"), tol).expect("loads");
    assert_eq!(
        logged.snapshot.label(extrude),
        None,
        "the snapshot is unlabelled"
    );
    assert!(logged.doc.bit_eq(&labelled), "the log's label edit replays");
}

/// The load door holds a file to the edit door's rules: a label's text
/// passes `Label::new`, and its key names a live node.
#[test]
fn the_load_door_refuses_a_blank_label_and_a_label_on_a_dead_node() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("node-labels-load", tol);
    let (doc, [_, _, live]) = block(doc, 0.0);
    let (doc, [_, _, gone]) = block(doc, 5.0);
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: gone });
    let doc = set_label(doc, live, Some("lid"));
    let text = save(&doc, &[], tol).expect("saves");
    let key = |id: RecipeNodeId| serde_json::to_string(&id.0.to_string()).expect("a string");
    let entry = format!("{}: \"lid\"", key(live));
    assert!(text.contains(&entry), "the file holds {entry}");

    let blank = text.replace(&entry, &format!("{}: \" \"", key(live)));
    match load(&blank, tol) {
        Err(PersistError::Unreadable { .. }) => {}
        other => panic!("a blank label refuses at the parse, got {other:?}"),
    }
    let dead = text.replace(&entry, &format!("{}: \"lid\"", key(gone)));
    match load(&dead, tol) {
        Err(PersistError::Snapshot(SnapshotError::LabelOnMissingNode { node })) => {
            assert_eq!(node, editor_core::SpokenNode::absent(gone));
        }
        other => panic!("a label on a dead node refuses LabelOnMissingNode, got {other:?}"),
    }
}

/// The load door speaks a node from the document it judges: a file
/// whose `order` runs backwards refuses with its nodes' kinds and
/// labels, read off the parsed document, and an id that document does
/// not hold reads as a node.
#[test]
fn the_load_door_speaks_the_nodes_of_the_file_it_refuses() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("node-labels-speak", tol);
    let (doc, [_, profile, extrude]) = block(doc, 0.0);
    let doc = set_label(doc, profile, Some("outline"));
    let doc = set_label(doc, extrude, Some("base \"plate\""));
    let text = save(&doc, &[], tol).expect("saves");
    let (header, body) = text.split_once('\n').expect("a header line, then the body");
    let mut v: serde_json::Value = serde_json::from_str(body).expect("the body is JSON");
    v["snapshot"]["order"]
        .as_array_mut()
        .expect("the file carries its order")
        .reverse();
    match load(&format!("{header}\n{v}\n"), tol) {
        Err(PersistError::Snapshot(SnapshotError::ForwardInput { node, input })) => {
            assert!(
                [profile, extrude].contains(&node.id()),
                "a labelled node is refused"
            );
            assert_eq!(node, doc.spoken(node.id()), "the refused node");
            assert_eq!(input, doc.spoken(input.id()), "its input");
            let said = if node.id() == extrude {
                format!("Extrude \"base \\\"plate\\\"\" ({})", tag(extrude.0))
            } else {
                format!("Profile \"outline\" ({})", tag(profile.0))
            };
            let sentence =
                PersistError::Snapshot(SnapshotError::ForwardInput { node, input }).to_string();
            assert!(
                sentence.contains(&format!("{said} takes input from")),
                "the sentence speaks the node with its label: {sentence}"
            );
        }
        other => panic!("a backwards order refuses ForwardInput, got {other:?}"),
    }
}

/// Split carries each cut node's label onto its copy in the part and
/// leaves its new instance unlabelled; inline carries each part node's
/// label onto the node it splices.
#[test]
fn split_and_inline_carry_labels_and_the_new_instance_has_none() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("node-labels-split", tol);
    let (doc, [_, _, kept]) = block(doc, 0.0);
    let (doc, [frame, profile, extrude]) = block(doc, 5.0);
    let doc = set_label(doc, kept, Some("base"));
    let doc = set_label(doc, profile, Some("pin sketch"));
    let doc = set_label(doc, extrude, Some("pin"));

    let cut = BTreeSet::from([frame, profile, extrude]);
    let out = split(&doc, &cut, DocumentId::derive("node-labels-pin"), tol, None)
        .expect("the split is legal");
    let in_part = |old: RecipeNodeId| out.part.label(out.node_map[&old]);
    assert_eq!(
        (in_part(frame), in_part(profile), in_part(extrude)),
        (None, Some(&label("pin sketch")), Some(&label("pin"))),
        "each cut node's copy carries its label"
    );
    assert_eq!(out.part.labels().len(), 2, "and the part holds no other");
    assert_eq!(
        out.remainder.label(out.instance),
        None,
        "the new instance is unlabelled"
    );
    assert_eq!(
        out.remainder.labels().keys().copied().collect::<Vec<_>>(),
        vec![kept],
        "the remainder keeps its own label and drops the cut's"
    );

    let mut store = PartStore::default();
    store.insert(out.part.clone(), tol);
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
    let back = inline(&out.remainder, out.instance, &resolver, tol).expect("the inline is legal");
    let spliced = |old: RecipeNodeId| back.doc.label(back.node_map[&out.node_map[&old]]);
    assert_eq!(
        (spliced(frame), spliced(profile), spliced(extrude)),
        (None, Some(&label("pin sketch")), Some(&label("pin"))),
        "each spliced node carries its part-side label"
    );
    assert_eq!(back.doc.label(kept), Some(&label("base")));
    assert_eq!(back.doc.labels().len(), 3, "and nothing else is labelled");
}

/// The spoken node reads the label off the document when it is built:
/// kind, quoted label and tag; kind and tag once cleared; `node` and
/// the tag once the node is gone.
#[test]
fn the_spoken_node_says_kind_label_and_tag_as_the_document_holds_them() {
    let doc = ProfileDoc::empty_derived("node-labels-spoken", Tol::witness());
    let (doc, [_, _, extrude]) = block(doc, 0.0);
    let labelled = set_label(doc.clone(), extrude, Some("base plate"));
    let t = tag(extrude.0);
    assert_eq!(
        labelled.spoken(extrude).to_string(),
        format!("Extrude \"base plate\" ({t})")
    );
    assert_eq!(labelled.spoken(extrude).label(), Some(&label("base plate")));
    assert_eq!(doc.spoken(extrude).to_string(), format!("Extrude {t}"));
    let quoted = set_label(doc.clone(), extrude, Some(r#"Bolt "M6" \ 2"#));
    assert_eq!(
        quoted.spoken(extrude).to_string(),
        format!(r#"Extrude "Bolt \"M6\" \\ 2" ({t})"#),
        "a quote and a backslash in the label are escaped"
    );
    let (gone, _) = step(labelled, DocEdit::DeleteNode { id: extrude });
    assert_eq!(gone.spoken(extrude).to_string(), format!("node {t}"));
}

/// **An edit refusal speaks each node it names at the raise**, from the
/// document the door holds: kind, label and tag for a held node, and
/// kind and tag for the node an insert is minting, which carries no
/// label. The typed field keeps the id a caller matches on.
#[test]
fn an_edit_refusal_names_each_node_as_the_document_holds_it() {
    let doc = ProfileDoc::empty_derived("node-labels-refusals", Tol::witness());
    let (doc, [_, profile, extrude]) = block(doc, 0.0);
    let doc = set_label(doc, profile, Some("sketch"));
    let doc = set_label(doc, extrude, Some("base plate"));
    let (p, e) = (tag(profile.0), tag(extrude.0));

    let dangle = refusal(&doc, DocEdit::DeleteNode { id: profile });
    let EditError::DeleteWouldDangle { id, referenced_by } = &dangle else {
        panic!("deleting a read node refuses DeleteWouldDangle, got {dangle:?}");
    };
    assert_eq!((id.id(), referenced_by.id()), (profile, extrude));
    assert!(
        dangle.to_string().starts_with(&format!(
            "Profile \"sketch\" ({p}) is still an input to Extrude \"base plate\" ({e})"
        )),
        "{dangle}"
    );

    let twice = refusal(
        &doc,
        DocEdit::InsertNode {
            node: Node::Union {
                members: vec![extrude, extrude],
                declare: None,
            },
        },
    );
    let EditError::DuplicateInput { node, input } = &twice else {
        panic!("a member named twice refuses DuplicateInput, got {twice:?}");
    };
    assert_eq!(
        (node.kind(), node.label(), input),
        (Some("Union"), None, &doc.spoken(extrude)),
        "the minted node by its kind alone, the input as the document holds it"
    );
    assert!(
        twice.to_string().contains(&format!(
            "Extrude \"base plate\" ({e}) is taken as an input twice"
        )),
        "{twice}"
    );
}

/// **A held node an edit rewrites is spoken with its label**: a
/// `SetMembers` that names one member twice refuses about the union as
/// the document holds it, label and all, not as the insert door's
/// kind-and-tag spelling of a node being minted.
#[test]
fn a_set_members_refusal_names_the_labelled_union_it_rewrites() {
    let doc = ProfileDoc::empty_derived("node-labels-set-members", Tol::witness());
    let (doc, [_, _, left]) = block(doc, 0.0);
    let (doc, [_, _, right]) = block(doc, 2.0);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![left, right],
            declare: None,
        },
    );
    let doc = set_label(doc, union, Some("pair"));
    let doc = set_label(doc, left, Some("left"));

    let twice = refusal(
        &doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![left, left],
        },
    );
    let EditError::DuplicateInput { node, input } = &twice else {
        panic!("a member named twice refuses DuplicateInput, got {twice:?}");
    };
    assert_eq!(
        (node, input),
        (&doc.spoken(union), &doc.spoken(left)),
        "both nodes as the document holds them"
    );
    assert_eq!(
        node.label(),
        Some(&label("pair")),
        "the rewritten union keeps its label"
    );
    assert!(
        twice.to_string().contains(&format!(
            "Extrude \"left\" ({}) is taken as an input twice",
            tag(left.0)
        )),
        "{twice}"
    );
}

/// **A delete's report speaks from the document the door was handed.**
/// The stranded name's minting node is the one the delete removed, so
/// only the document before the edit still holds its label: a row
/// spoken from the document the edit leaves would say `node <tag>`.
/// The surviving carrier and an orphaned declaration are spoken the
/// same way, label and all.
#[test]
fn a_strand_names_the_deleted_minting_node_with_the_label_it_had() {
    let doc = ProfileDoc::empty_derived("node-labels-strand", Tol::witness());
    let (doc, [_, _, kept]) = block(doc, 0.0);
    let (doc, [_, _, victim]) = block(doc, 4.0);
    let named = fixture::fname(victim, fixture::wall(&doc, victim, 0));
    let (doc, carrier) = insert(
        doc,
        Node::Datum(editor_core::Datum::FaceFrame {
            at: kept,
            face: named.clone(),
            spin: fixture::ang(0.0),
        }),
    );
    let doc = set_label(doc, victim, Some("base plate"));
    let doc = set_label(doc, carrier, Some("mount"));

    let applied = editor_core::apply(
        &doc,
        &DocEdit::DeleteNode { id: victim },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a name is not an edge, so the delete lands");
    let [Maintenance::Strand { node, name }] = applied.maintenance.as_slice() else {
        panic!("one strand, got {:?}", applied.maintenance);
    };
    assert_eq!((node.id(), name.name()), (carrier, &named));
    assert_eq!(
        name.minter().label(),
        Some(&label("base plate")),
        "the minting node is spoken from the document that still held it"
    );
    assert_eq!(
        applied.maintenance[0].to_string().split(';').next(),
        Some(
            format!(
                "Datum frame (on face) \"mount\" ({}) carries a face name minted by Extrude \
                 \"base plate\" ({})",
                tag(carrier.0),
                tag(victim.0)
            )
            .as_str()
        ),
    );
}

/// **A root refusal at the edit door speaks both roots with their
/// labels**, and its recourse names the one to drop the same way.
#[test]
fn a_root_refusal_speaks_the_labelled_roots_and_its_recourse_does_too() {
    let doc = ProfileDoc::empty_derived("node-labels-roots", Tol::witness());
    let (doc, [_, profile, extrude]) = block(doc, 0.0);
    let doc = set_label(doc, profile, Some("sketch"));
    let doc = set_label(doc, extrude, Some("base plate"));
    let refused = refusal(
        &doc,
        DocEdit::SetRoots {
            roots: vec![profile, extrude],
        },
    );
    let EditError::Roots(RootFault::Ancestor {
        ancestor,
        descendant,
    }) = &refused
    else {
        panic!("an ancestor pair refuses, got {refused:?}");
    };
    assert_eq!(
        (ancestor, descendant),
        (&doc.spoken(profile), &doc.spoken(extrude))
    );
    let (p, e) = (tag(profile.0), tag(extrude.0));
    let text = refused.to_string();
    assert!(
        text.starts_with(&format!(
            "product root Profile \"sketch\" ({p}) is an ancestor of product root Extrude \
             \"base plate\" ({e})"
        )),
        "{text}"
    );
    assert!(
        text.contains(&format!("drop Profile \"sketch\" ({p}) from the root list")),
        "{text}"
    );
}

/// **A name an edit refusal forwards speaks its minting node** as the
/// document holds it.
#[test]
fn a_forwarded_name_speaks_its_labelled_minting_node() {
    let doc = ProfileDoc::empty_derived("node-labels-name", Tol::witness());
    let (doc, [_, _, extrude]) = block(doc, 0.0);
    let doc = set_label(doc, extrude, Some("base plate"));
    let unreferenced = fixture::fname(extrude, fixture::wall(&doc, extrude, 0));
    let refused = refusal(
        &doc,
        DocEdit::Rebind {
            from: unreferenced.clone(),
            to: fixture::fname(extrude, fixture::wall(&doc, extrude, 1)),
        },
    );
    assert_eq!(
        refused,
        EditError::RebindNoReferences {
            name: doc.spoken_name(&unreferenced)
        }
    );
    assert!(
        refused.to_string().contains(&format!(
            "face name minted by Extrude \"base plate\" ({})",
            tag(extrude.0)
        )),
        "{refused}"
    );
}

/// **The load door speaks the node it refuses with its label.** The
/// validator judges a deserialized document whose labels have already
/// passed `Label::new` and the live-key rule, so a root refusal there
/// speaks from it like the edit door's does. The craft drops one of
/// two tips from the root list, stranding that tip's whole chain.
#[test]
fn a_load_root_refusal_speaks_the_labelled_node_from_the_file() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("node-labels-load-roots", tol);
    let (doc, [_, _, kept]) = block(doc, 0.0);
    let (doc, lost) = block(doc, 5.0);
    let doc = lost
        .iter()
        .fold(doc, |doc, &id| set_label(doc, id, Some("stranded")));
    let text = save(&doc, &[], tol).expect("the honest document saves");
    let honest = format!(
        "\"roots\": [\n      {},\n      {}\n    ]",
        kept.0, lost[2].0
    );
    assert!(
        text.contains(&honest),
        "the save's root list is the two tips"
    );
    let crafted = text.replace(&honest, &format!("\"roots\": [\n      {}\n    ]", kept.0));
    let refused = match load(&crafted, tol) {
        Err(PersistError::Snapshot(SnapshotError::Roots(fault))) => fault,
        other => panic!("a crafted uncovered document refuses, got {other:?}"),
    };
    let RootFault::Uncovered { node } = &refused else {
        panic!("the stranded chain is uncovered, got {refused:?}");
    };
    assert!(lost.contains(&node.id()), "{node}");
    assert_eq!(node.label(), Some(&label("stranded")), "{node}");
    assert!(
        refused
            .to_string()
            .contains(&format!("\"stranded\" ({})", tag(node.id().0))),
        "{refused}"
    );
}

/// **A severing split speaks both ends of the edge** from the document
/// being split, labels and all, and says which side each is on by its
/// role: neither end's id is printed in decimal.
#[test]
fn a_severing_split_speaks_both_ends_and_prints_no_decimal_id() {
    let doc = ProfileDoc::empty_derived("node-labels-severed", Tol::witness());
    let (doc, [frame, profile, extrude]) = block(doc, 0.0);
    let doc = set_label(doc, profile, Some("sketch"));
    let doc = set_label(doc, extrude, Some("base plate"));
    let refused = split(
        &doc,
        &BTreeSet::from([frame, profile]),
        DocumentId::derive("node-labels-severed-part"),
        Tol::witness(),
        None,
    )
    .expect_err("the kept extrude's input is cut");
    let SplitError::SeveredEdge {
        consumer, input, ..
    } = &refused
    else {
        panic!("the edge is severed, got {refused:?}");
    };
    assert_eq!(
        (consumer, input),
        (&doc.spoken(extrude), &doc.spoken(profile))
    );
    let text = refused.to_string();
    assert_eq!(
        text,
        format!(
            "split: the cut severs the edge from Extrude \"base plate\" ({}) to its input \
             Profile \"sketch\" ({}). The consumer is kept and the input is cut, but a cut \
             must be closed under inputs and consumers",
            tag(extrude.0),
            tag(profile.0)
        )
    );
    for id in [extrude, profile] {
        assert!(
            !text.contains(&id.0.to_string()),
            "{id:?} in decimal: {text}"
        );
    }
}

/// **A split's forward reference speaks its name from the document
/// being split.** The name is spelled in that document's ids, which
/// the part being rebuilt does not hold, and the recourse sends the
/// reader there; spoken from the part it would lose its label.
#[test]
fn a_split_forward_reference_speaks_from_the_document_being_split() {
    let (doc, late, c) = forward_reference("node-labels-forward");
    let cut: BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let refused = split(
        &doc,
        &cut,
        DocumentId::derive("node-labels-forward-part"),
        Tol::witness(),
        None,
    )
    .expect_err("the part cannot be rebuilt in document order");
    let SplitError::PartEdit { error } = &refused else {
        panic!("the replay refuses, got {refused:?}");
    };
    assert_eq!(
        **error,
        EditError::DeclareNamesMissingNode {
            name: doc.spoken_name(&late)
        }
    );
    assert!(
        refused.to_string().contains(&format!(
            "face name minted by Extrude \"late block\" ({})",
            tag(c.0)
        )),
        "{refused}"
    );
}

/// **An inline's forward reference speaks its name from the part**,
/// whose ids it is spelled in and where its recourse sends the reader:
/// the host the replay writes holds no node of it.
#[test]
fn an_inline_forward_reference_speaks_from_the_part() {
    let tol = Tol::witness();
    let (part_doc, late, c) = forward_reference("node-labels-inline-forward");
    let mut store = PartStore::default();
    let doc_ref = store.insert(part_doc.clone(), tol);
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
    let host = ProfileDoc::empty_derived("node-labels-inline-forward-host", tol);
    let (host, inst) = insert(host, Node::instantiate_part(doc_ref));
    let refused = inline(&host, inst, &resolver, tol).expect_err("the part splices in order");
    let InlineError::Edit { error } = &refused else {
        panic!("the replay refuses, got {refused:?}");
    };
    assert_eq!(
        **error,
        EditError::DeclareNamesMissingNode {
            name: part_doc.spoken_name(&late)
        }
    );
    assert!(
        refused.to_string().contains(&format!(
            "face name minted by Extrude \"late block\" ({})",
            tag(c.0)
        )),
        "{refused}"
    );
}

/// A document holding a Declare whose `b` side was rebound onto the
/// wall of a block inserted after it, labelled `late block`: the
/// rebound name and that block's extrude.
fn forward_reference(id: &str) -> (ProfileDoc, editor_core::StableName, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, [_, _, a]) = block(doc, 0.0);
    let (doc, [_, _, b]) = block(doc, 0.5);
    let (wa, wb) = (fixture::wall(&doc, a, 0), fixture::wall(&doc, b, 0));
    let early = fixture::fname(b, wb);
    let (doc, _) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(a, fixture::fname(a, wa)),
            SitedRef::new(b, early.clone()),
        )]),
    );
    let (doc, [_, _, c]) = block(doc, 0.5);
    let late = fixture::fname(c, fixture::wall(&doc, c, 0));
    let (doc, _) = step(
        doc,
        DocEdit::Rebind {
            from: early,
            to: late.clone(),
        },
    );
    (set_label(doc, c, Some("late block")), late, c)
}

/// **An inline refusal speaks each node from the document that holds
/// it**: the instance and its consumer from the host, the part's root
/// from the part, each with the label that document gives it.
#[test]
fn an_inline_refusal_speaks_host_nodes_from_the_host_and_part_nodes_from_the_part() {
    let tol = Tol::witness();
    let part_doc = ProfileDoc::empty_derived("node-labels-inline-part", tol);
    let (part_doc, [_, _, body]) = block(part_doc, 0.0);
    let part_doc = set_label(part_doc, body, Some("bracket"));
    let mut store = PartStore::default();
    let doc_ref = store.insert(part_doc, tol);
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
    let host = ProfileDoc::empty_derived("node-labels-inline-host", tol);
    let (host, inst) = insert(host, Node::instantiate_part(doc_ref));
    let host = set_label(host, inst, Some("left bracket"));

    let (placed, _) = step(
        host.clone(),
        DocEdit::SetPlacement {
            node: inst,
            frame: editor_core::Frame::translation([3.0, 0.0, 0.0]),
        },
    );
    let refused = inline(&placed, inst, &resolver, tol).expect_err("a placed plain part");
    let InlineError::UnplaceableFrame { root } = &refused else {
        panic!("the frame is not expressible, got {refused:?}");
    };
    assert_eq!(
        (root.id(), root.label()),
        (body, Some(&label("bracket"))),
        "the root is the part's, spoken from the part"
    );
    assert!(
        refused.to_string().contains(&format!(
            "the part's root Extrude \"bracket\" ({})",
            tag(body.0)
        )),
        "{refused}"
    );

    let (consumed, by) = insert(
        host,
        Node::transform(
            inst,
            editor_core::Step::Rigid {
                translation: [len(1.0), len(0.0), len(0.0)],
                axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
                angle: fixture::ang(0.0),
            },
        ),
    );
    let consumed = set_label(consumed, by, Some("offset"));
    let refused = inline(&consumed, inst, &resolver, tol).expect_err("a consumed instance");
    assert_eq!(
        refused.to_string(),
        format!(
            "inline: InstantiatePart \"left bracket\" ({}) is consumed by Transform \"offset\" \
             ({}) — the recipe cannot rewire a consumer onto a spliced product",
            tag(inst.0),
            tag(by.0)
        )
    );
}
