//! Review probes, INTENT stage 5 B, reviewer 1 (PR 4560).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{fname, insert, len, on_frame, place, square, wall};
use editor_core::{
    DocEdit, ExtrudeSide, MeasurePrimitive, Node, ProfileDoc, RecipeNodeId, SitedRef,
};
use geom_core::Tol;

fn block(
    doc: ProfileDoc,
    (cx, cy, z): (f64, f64, f64),
    h: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, z],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, cy, h)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
            side: ExtrudeSide::Along,
        },
    )
}

/// **Claim 6: a selection of a world copy minted at a measure's seat,
/// then read BY ID at a construction seat.** The edit door's world-copy
/// rule lives in `read_fault` keyed on the read variable's defining
/// node being a `PlaceInWorld`; a selection variable is defined by its
/// `Select`, so a construction reading the shared selection is asked
/// nothing. Red (the probe's claim) when the shell is admitted.
#[test]
fn a_construction_reading_a_measures_copy_selection_by_id_refuses() {
    let doc = ProfileDoc::empty_derived("review-s5b-r1-shared-select", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
    let (doc, b) = block(doc, (0.75, 0.0, 0.25), 0.3, 0.5);
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, b);
    let outer = fname(a, wall(&doc, a, 1)).in_copy(p);
    let inner = fname(b, wall(&doc, b, 3)).in_copy(q);
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![SitedRef::new(p, outer), SitedRef::new(q, inner)],
    );
    let Some(Node::Measure {
        primitive: MeasurePrimitive::Gap {
            outer: selection, ..
        },
    }) = doc.node(measure)
    else {
        panic!("a gap")
    };
    let selection = *selection;
    let doc = crate::fixture::step(
        doc,
        DocEdit::RenameVar {
            var: selection.into(),
            name: Some(editor_core::VarName::new("wall").unwrap()),
        },
    )
    .0;
    let shell = Node::Datum(editor_core::Datum::FaceFrame {
        face: editor_core::Operand::Name(editor_core::VarName::new("wall").unwrap()),
        spin: crate::fixture::ang(0.0),
    });
    match doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(shell),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(editor_core::EditError::ReadsWorldCopy { .. }) => {}
        Err(other) => panic!("refused otherwise: {other}"),
        Ok(applied) => {
            let frame = applied.record.minted.unwrap();
            let doc = applied.doc;
            let save = editor_core::persist::save(&doc, &[], Tol::witness());
            let run = crate::corpus::eval::<f64>(&doc);
            panic!(
                "the edit door admitted a face frame reading a world copy's selection by name; \
                 it evaluates to {:?}; save: {:?}",
                run.value(frame).map(|v| format!("{:?}", v.payload)),
                save.err()
            );
        }
    }
}

/// **The same asymmetry by deletion.** A measure's selection of a world
/// copy, named, outlives its measure: the edit door admits the delete,
/// and the selection is then read by no seat, which the save/load door's
/// `selection_reader` reads as a construction. Red (the probe's claim)
/// when the edit door leaves a document its own save refuses.
#[test]
fn deleting_the_measure_leaves_a_document_that_saves() {
    let doc = ProfileDoc::empty_derived("review-s5b-r1-orphan-select", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
    let (doc, b) = block(doc, (0.75, 0.0, 0.25), 0.3, 0.5);
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, b);
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![
            SitedRef::new(p, fname(a, wall(&doc, a, 1)).in_copy(p)),
            SitedRef::new(q, fname(b, wall(&doc, b, 3)).in_copy(q)),
        ],
    );
    let Some(Node::Measure {
        primitive: MeasurePrimitive::Gap {
            outer: selection, ..
        },
    }) = doc.node(measure)
    else {
        panic!("a gap")
    };
    let selection = *selection;
    let doc = crate::fixture::step(
        doc,
        DocEdit::RenameVar {
            var: selection.into(),
            name: Some(editor_core::VarName::new("wall").unwrap()),
        },
    )
    .0;
    editor_core::persist::save(&doc, &[], Tol::witness()).expect("with its measure it saves");
    let doc = match doc.apply(
        &DocEdit::DeleteNode { id: measure },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Ok(applied) => applied.doc,
        Err(refused) => {
            eprintln!("the delete refuses ({refused}): consistent");
            return;
        }
    };
    if let Err(e) = editor_core::persist::save(&doc, &[], Tol::witness()) {
        panic!("the edit door admitted a delete whose document no longer saves: {e:?}");
    }
}
