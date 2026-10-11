//! **A shared variable is named at the doors** (VARIABLES-DESIGN VR2,
//! VR7, VR9; FORK-7): an unnamed variable has exactly one reader, a
//! slot or a definition, and one formula reading it twice is one
//! reader. Giving an unnamed variable a second reader, or clearing the
//! name of a shared one, refuses `SharedVarNeedsName` naming the
//! variable to name, at every door; the load walk refuses a file that
//! holds one. An output is exempt: it is named by its operation and
//! port.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{insert, len};
use editor_core::persist::SnapshotError;
use editor_core::{
    Axis3, Datum, Dimension, DocEdit, DocumentId, EditError, ExprPath, Formula, FreeVar,
    FreshEntry, Node, PersistError, ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, VarDecl,
    VarId, VarName, apply, load, save,
};
use geom_core::Tol;

fn n(name: &'static str) -> VarName {
    VarName::from_static(name)
}

fn try_step(
    doc: &ProfileDoc,
    edit: DocEdit<ProfileProgram>,
) -> Result<editor_core::Applied<ProfileProgram>, EditError> {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
}

fn step(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> ProfileDoc {
    try_step(doc, edit).expect("the edit applies").doc
}

/// The variable `edit` refuses to leave shared and unnamed.
fn needs_name(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> editor_core::SpokenVar {
    match try_step(doc, edit) {
        Err(EditError::SharedVarNeedsName { var }) => var,
        other => panic!("a second reader of an unnamed variable refuses, got {other:?}"),
    }
}

fn x() -> SlotId {
    SlotId::Origin(Axis3::X)
}

fn read(var: VarId) -> Formula {
    Formula::var(var, Dimension::Length)
}

/// A document holding a point at `(0.5, 0, 0)`, typed: its x reads an
/// unnamed free variable, its one reader. The point and that variable.
fn typed_point(seed: &str) -> (ProfileDoc, RecipeNodeId, VarId) {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let (doc, point) = insert(
        doc,
        Node::Datum(Datum::Point {
            position: [len(0.5), len(0.0), len(0.0)],
        }),
    );
    let v = doc.slot(point, x()).expect("the point reads its x");
    assert_eq!(doc.var_name(v), None, "the premise: v is unnamed");
    (doc, point, v)
}

fn point_reading(position: [Formula; 3]) -> DocEdit<ProfileProgram> {
    DocEdit::InsertNode {
        node: Box::new(Node::Datum(Datum::Point { position })),
        fresh: Vec::new(),
    }
}

/// **The insert door**: a node whose slot reads another node's unnamed
/// variable refuses, and so does one whose two slots read one; once the
/// variable is named, the same insert lands and both read it.
#[test]
fn the_insert_door_refuses_a_second_reader_until_it_is_named() {
    let (doc, _, v) = typed_point("fork7-insert");
    let second = point_reading([read(v), len(0.0), len(0.0)]);
    assert_eq!(needs_name(&doc, second.clone()).id(), v);
    let named = step(
        &doc,
        DocEdit::RenameVar {
            var: v.into(),
            name: Some(n("v")),
        },
    );
    let applied = try_step(&named, second).expect("a named variable is shared");
    let b = applied.record.minted.expect("the second point");
    assert_eq!(applied.doc.slot(b, x()), Some(v));
}

/// **The formula door**: a path edit writing a read of another node's
/// unnamed variable into a slot's definition refuses.
#[test]
fn the_formula_door_refuses_a_path_edit_that_shares() {
    let (doc, _, v) = typed_point("fork7-path");
    let twice = Formula::mul(len(0.25), Formula::ratio(2, 1).unwrap()).unwrap();
    let applied = try_step(&doc, point_reading([twice, len(0.0), len(0.0)])).unwrap();
    let b = applied.record.minted.expect("the second point");
    let at = DocEdit::SetExpression {
        path: ExprPath {
            node: b,
            slot: x(),
            path: vec![0],
        },
        expr: read(v),
    };
    assert_eq!(needs_name(&applied.doc, at).id(), v);
}

/// **The definition doors**: a declare, or a redefinition, whose
/// formula reads a slot's unnamed variable refuses. One formula reading
/// it twice is still one reader, so a definition that alone reads an
/// unnamed variable twice lands.
#[test]
fn a_definition_is_a_reader() {
    let (doc, _, v) = typed_point("fork7-define");
    let plus = || Formula::add(read(v), len(0.001)).unwrap();
    let declare = DocEdit::DeclareVar {
        name: n("k"),
        def: VarDecl::defined(plus()),
    };
    assert_eq!(needs_name(&doc, declare).id(), v);

    let doc = step(
        &doc,
        DocEdit::DeclareVar {
            name: n("k"),
            def: FreeVar::continuous(Dimension::Length, 1.0).into(),
        },
    );
    let redefine = DocEdit::DefineVar {
        var: n("k").into(),
        def: VarDecl::defined(plus()),
        fresh: Vec::new(),
    };
    assert_eq!(needs_name(&doc, redefine).id(), v);

    // An entry read twice by the one definition it serves: one reader.
    let entry = || Formula::fresh(0, Dimension::Length);
    let doc = step(
        &doc,
        DocEdit::DefineVar {
            var: n("k").into(),
            def: VarDecl::defined(Formula::add(entry(), entry()).unwrap()),
            fresh: vec![FreeVar::continuous(Dimension::Length, 0.5).into()],
        },
    );
    let k = doc.var_named("k").expect("declared");
    let reads = doc.var(k).and_then(|var| var.def().defined()).map(|def| {
        let mut reads = Vec::new();
        def.var_reads(&mut reads);
        reads
    });
    let reads = reads.expect("k is defined");
    assert_eq!(reads.len(), 2, "k reads its entry twice");
    assert_eq!(reads[0], reads[1]);
    assert_eq!(
        doc.var_name(reads[0].0),
        None,
        "and the entry stays unnamed"
    );
}

/// **Clearing the name of a shared variable** refuses, speaking it by
/// the name it holds; the name of a variable one reader reads clears.
#[test]
fn clearing_the_name_two_readers_share_refuses() {
    let (doc, _, v) = typed_point("fork7-clear");
    let doc = step(
        &doc,
        DocEdit::RenameVar {
            var: v.into(),
            name: Some(n("v")),
        },
    );
    let clear = DocEdit::RenameVar {
        var: n("v").into(),
        name: None,
    };
    let lone = step(&doc, clear.clone());
    assert_eq!(lone.var_name(v), None, "one reader: the name clears");

    let doc = step(&doc, point_reading([read(v), len(0.0), len(0.0)]));
    let var = needs_name(&doc, clear);
    assert_eq!((var.id(), var.name()), (v, Some(&n("v"))));
}

/// **The slot door's re-point leaves the old reader's variable with
/// it**: a slot moved off an unnamed variable onto a named one takes
/// the unnamed one with it (VR7: an unnamed variable goes with its
/// reader), and nothing else moves.
#[test]
fn an_unnamed_variable_goes_with_its_reader() {
    let (doc, point, v) = typed_point("fork7-goes");
    let doc = step(
        &doc,
        DocEdit::DeclareVar {
            name: n("w"),
            def: FreeVar::continuous(Dimension::Length, 0.5).into(),
        },
    );
    let w = doc.var_named("w").expect("declared");
    let applied = try_step(
        &doc,
        DocEdit::SetParam {
            node: point,
            slot: x(),
            value: read(w).into(),
            fresh: Vec::new(),
        },
    )
    .unwrap();
    assert!(applied.doc.var(v).is_none(), "v went with its reader");
    assert_eq!(applied.doc.slot(point, x()), Some(w));
}

/// **An output is exempt**: two operations reading one extrude's body
/// read one unnamed output, and both land.
#[test]
fn an_output_two_operations_read_needs_no_name() {
    let doc = ProfileDoc::empty(DocumentId::derive("fork7-output"), Tol::witness());
    let (doc, profile) = crate::fixture::on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![crate::fixture::square(0.0, 0.0, 0.5)],
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let edges = crate::fixture::prism_edges(&doc, cube, 4);
    let (doc, _) = insert(doc, Node::fillet(cube, len(0.125), edges.clone()));
    let (doc, _) = insert(doc, Node::chamfer(cube, len(0.125), edges));
    let body = doc.read_of_node(cube).expect("the extrude's body");
    assert_eq!(
        doc.var_name(body),
        None,
        "the premise: the output is unnamed"
    );
    assert_eq!(doc.reader_counts()[&body], 2);
    assert!(doc.shared_unnamed_vars().is_empty());
}

/// **A fresh entry carrying a name crosses the edit log**: a document
/// whose log mints one saves, and the load replays it to the same
/// document. Breaks if the entry's name does not ride the wire.
#[test]
fn a_named_entry_rides_the_edit_log() {
    let doc = ProfileDoc::empty(DocumentId::derive("fork7-log"), Tol::witness());
    let fresh = || Formula::fresh(0, Dimension::Length);
    let edit = DocEdit::InsertNode {
        node: Box::new(Node::Datum(Datum::Point {
            position: [fresh(), fresh(), len(0.0)],
        })),
        fresh: vec![FreshEntry::named(
            n("s"),
            FreeVar::continuous(Dimension::Length, 0.5),
        )],
    };
    let after = step(&doc, edit.clone());
    let text = save(&doc, &[edit], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads");
    assert!(loaded.doc.bit_eq(&after), "the replay is the document");
    assert!(loaded.doc.var_named("s").is_some());
}

/// **The load walk** refuses a file holding an unnamed variable two
/// definitions read, with the regenerate recourse. Breaks if the walk
/// asks only whether an unnamed variable is read.
#[test]
fn the_load_walk_refuses_a_shared_unnamed_variable() {
    let doc = ProfileDoc::empty(DocumentId::derive("fork7-load"), Tol::witness());
    let doc = step(
        &doc,
        DocEdit::DeclareVar {
            name: n("w"),
            def: FreeVar::continuous(Dimension::Length, 0.5).into(),
        },
    );
    let w = doc.var_named("w").expect("declared");
    let scaled = |by: u32| {
        VarDecl::defined(Formula::mul(read(w), Formula::ratio(by.into(), 1).unwrap()).unwrap())
    };
    let doc = step(
        &doc,
        DocEdit::DeclareVar {
            name: n("a"),
            def: scaled(2),
        },
    );
    let doc = step(
        &doc,
        DocEdit::DeclareVar {
            name: n("b"),
            def: scaled(3),
        },
    );
    let text = save(&doc, &[], Tol::witness()).unwrap();
    load(&text, Tol::witness()).expect("the undoctored save loads");
    let corrupt = crate::wire::doctored(&text, |wire| {
        let names = wire["snapshot"]["var_names"]
            .as_object_mut()
            .expect("the name table");
        assert!(names.remove(&w.0.to_string()).is_some(), "w had a name");
    });
    let err = load(&corrupt, Tol::witness()).expect_err("the doctored save refuses");
    let PersistError::Snapshot(SnapshotError::SharedVarNeedsName { var }) = &err else {
        panic!("not SharedVarNeedsName: {err:?}")
    };
    assert_eq!(var.id(), w);
    assert!(
        err.to_string().contains("regenerate the file"),
        "the regenerate recourse: {err}"
    );
}

/// **The text round trip keeps a share** (`Doc::unparse`): a slot reading
/// a named shared variable writes it by name, so the text set back reads
/// the same variable — where an unnamed one, written as its value, would
/// re-mint.
#[test]
fn a_shared_variable_survives_its_text() {
    let (doc, point, v) = typed_point("fork7-text");
    let doc = step(
        &doc,
        DocEdit::RenameVar {
            var: v.into(),
            name: Some(n("v")),
        },
    );
    let doc = step(&doc, point_reading([read(v), len(0.0), len(0.0)]));
    let written = doc.slot_expansion(point, x()).expect("x as written");
    let text = editor_core::unparse(&written, &|id| doc.var_name(id));
    assert_eq!(text, "v");
    let names = [(n("v"), Dimension::Length)].into_iter().collect();
    let parsed = editor_core::parse_formula(&text, &names).expect("the text parses");
    let doc = step(
        &doc,
        DocEdit::SetParam {
            node: point,
            slot: x(),
            value: parsed.into(),
            fresh: Vec::new(),
        },
    );
    assert_eq!(doc.slot(point, x()), Some(v), "the share holds");
}

/// **An assertion's value and its bound are two slots** (VR4), so
/// they are two readers (VR2): an assertion reading one unnamed variable
/// on both sides refuses, where a definition reading it twice is one
/// reader. Breaks if the count folds one node's slots into one reader.
#[test]
fn the_value_and_the_bound_of_one_assertion_are_two_readers() {
    let doc = ProfileDoc::empty(DocumentId::derive("fork7-assertion"), Tol::witness());
    let edit = |fresh: FreshEntry| DocEdit::InsertNode {
        node: Box::new(Node::Assertion {
            value: Formula::fresh(0, Dimension::Length),
            bound: Formula::fresh(0, Dimension::Length),
            relation: editor_core::AssertionRelation::AtMost,
        }),
        fresh: vec![fresh],
    };
    match try_step(
        &doc,
        edit(FreeVar::continuous(Dimension::Length, 0.5).into()),
    ) {
        Err(EditError::SharedVarNeedsName { var }) => assert_eq!(var.name(), None),
        other => panic!("two sides reading one unnamed variable refuse, got {other:?}"),
    }
    let named = edit(FreshEntry::named(
        n("m"),
        FreeVar::continuous(Dimension::Length, 0.5),
    ));
    try_step(&doc, named).expect("a named variable both sides read lands");
}

/// **A log's replay passes the same door**: a saved log whose rename is
/// cut out gives the second point an unnamed variable's second read, so
/// the load refuses `EditReplay` with `SharedVarNeedsName`, and the
/// sentence's recourse is the file's (regenerate it), not the edit
/// door's (name it). Breaks if replay skips the check, or forwards the
/// edit door's recourse to a load that made no edit.
#[test]
fn a_replayed_share_refuses_with_the_files_recourse() {
    let (doc, _, v) = typed_point("fork7-replay");
    let rename = DocEdit::RenameVar {
        var: v.into(),
        name: Some(n("v")),
    };
    let second = point_reading([read(v), len(0.0), len(0.0)]);
    let text = save(&doc, &[rename, second], Tol::witness()).expect("the log saves");
    load(&text, Tol::witness()).expect("the undoctored log loads");
    let corrupt = crate::wire::doctored(&text, |wire| {
        let edits = wire["edits"].as_array_mut().expect("the log");
        assert!(edits[0].get("RenameVar").is_some(), "aimed at the rename");
        edits.remove(0);
    });
    let err = load(&corrupt, Tol::witness()).expect_err("the doctored log refuses");
    let PersistError::EditReplay { index, error } = &err else {
        panic!("not EditReplay: {err:?}")
    };
    assert_eq!(*index, 0);
    assert!(
        matches!(**error, EditError::SharedVarNeedsName { ref var } if var.id() == v),
        "{error:?}"
    );
    let said = err.to_string();
    assert!(said.contains("regenerate the file"), "{said}");
    assert!(!said.contains("name it"), "{said}");
}
