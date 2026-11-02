//! Review probes for PR 6393: an empty profile downstream.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, insert, len};
use editor_core::{
    Bodies, DocEdit, EvalOptions, Formula, Node, ProfileDoc, ProfileProgram, RecipeNodeId,
};
use geom_core::Tol;

fn empty() -> Node<ProfileProgram<Formula>, Formula> {
    Node::Profile(ProfileProgram {
        loops: Vec::new(),
        ids: Vec::new(),
        on: None,
    })
}

fn try_insert(
    doc: ProfileDoc,
    node: editor_core::AuthoredNode,
    what: &str,
    log: &mut Vec<String>,
) -> (ProfileDoc, Option<RecipeNodeId>) {
    match doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Ok(a) => {
            let id = a.record.minted;
            log.push(format!("{what}: inserted {id:?}"));
            (a.doc, id)
        }
        Err(e) => {
            log.push(format!("{what}: edit door refused: {e}"));
            (doc, None)
        }
    }
}

#[test]
fn review_6393_downstream_of_an_empty_profile() {
    let mut log = Vec::new();
    let doc = ProfileDoc::empty_derived("review-6393", Tol::witness());
    let (doc, p0) = insert(doc, empty());
    let (doc, e0) = insert(
        doc,
        Node::Extrude {
            profile: p0.into(),
            distance: len(1.0),
        },
    );
    let (doc, r0) = insert(
        doc,
        fixture::revolve_about(p0, (0.0, 0.0), fixture::along_y(), ang(0.5)),
    );
    let (doc, sq) = insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 1.0)])),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: sq.into(),
            distance: len(1.0),
        },
    );
    let place_into = |doc: ProfileDoc, target: RecipeNodeId, body: RecipeNodeId| {
        let sp = fixture::space(&doc, target);
        let placed = editor_core::Placed::of(&doc, Vec::new(), Vec::new());
        insert(
            doc,
            Node::place(
                editor_core::BodyRead::plain(editor_core::Operand::output(body, 0)),
                editor_core::Operand::Var(sp),
                placed.bundle(),
            ),
        )
    };
    let (doc, pe0) = place_into(doc, block, e0);
    let (doc, pb) = place_into(doc, e0, block);
    let (doc, u) = try_insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![block.into(), pe0.into()]),
        },
        "union",
        &mut log,
    );
    let (doc, s) = try_insert(
        doc,
        Node::Subtract {
            from: block.into(),
            tool: pe0.into(),
        },
        "subtract",
        &mut log,
    );
    let (doc, s2) = try_insert(
        doc,
        Node::Subtract {
            from: e0.into(),
            tool: pb.into(),
        },
        "subtract-from-empty",
        &mut log,
    );
    let sh = None;
    let plane = fixture::plane(editor_core::Operand::select(
        block,
        vec![fixture::fname(
            block,
            editor_core::RoleSeg::Cap(editor_core::CapEnd::Start),
        )],
    ));
    let (doc, m) = try_insert(
        doc,
        Node::Mirror {
            body: pe0.into(),
            plane: plane.clone().into(),
        },
        "mirror",
        &mut log,
    );
    let (doc, sp) = try_insert(
        doc,
        Node::Split {
            target: pe0.into(),
            tool: plane.into(),
        },
        "split",
        &mut log,
    );
    let (doc, _, row) = fixture::linear(doc, e0, [1.0, 0.0, 0.0], 3.0, 3);
    let axis = editor_core::pose::RoleRead::plain(editor_core::Operand::output(r0, 1));
    let (doc, circ) = {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            fixture::circular(doc.clone(), r0, (axis, editor_core::TargetAxis::Y), 1.0, 3)
        }));
        match r {
            Ok((d, _, c)) => (d, Some(c)),
            Err(_) => {
                log.push(
                    "circular about the empty revolve's axis: fixture panicked (edit door?)".into(),
                );
                (doc, None)
            }
        }
    };
    let loft = |first: RecipeNodeId, then: RecipeNodeId| Node::Loft {
        first: first.into(),
        then: vec![editor_core::LoftSection {
            id: None,
            profile: then.into(),
            station: vec![editor_core::StationMove::Slide {
                along: editor_core::Axis3::Z,
                by: len(1.0),
            }],
        }],
        v_degree: Formula::count(1),
        first_id: None,
    };
    let (doc, l1) = try_insert(doc, loft(sq, p0), "loft sq->empty", &mut log);
    let (doc, l2) = try_insert(doc, loft(p0, sq), "loft empty->sq", &mut log);
    let (doc, l3) = try_insert(doc, loft(p0, p0), "loft empty->empty", &mut log);

    let ev = fixture::run(&doc, &EvalOptions::default());
    let mut report = |what: &str, id: Option<RecipeNodeId>| {
        if let Some(id) = id {
            log.push(format!(
                "{what}: value? {} error: {:?}",
                ev.value(id).is_some(),
                ev.node_error(id).map(|e| e.kind.to_string())
            ));
        }
    };
    report("p0", Some(p0));
    report("placed e0", Some(pe0));
    report("block placed into e0", Some(pb));
    report("e0", Some(e0));
    report("r0", Some(r0));
    report("union", u);
    report("subtract", s);
    report("subtract-from-empty", s2);
    report("shell", sh);
    report("mirror", m);
    report("split", sp);
    report("linear row", Some(row));
    report("circular about r0 axis", circ);
    report("loft sq->empty", l1);
    report("loft empty->sq", l2);
    report("loft empty->empty", l3);
    // Save/load the whole thing.
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
    let loaded = editor_core::load(&text, Tol::witness()).expect("loads");
    assert!(loaded.doc.bit_eq(&doc));
    // Interval lane.
    let iv = editor_core::evaluate::<geom_core::Interval>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    log.push(format!(
        "interval e0: {:?}",
        iv.node_error(e0).map(|e| e.kind.to_string())
    ));
    log.push(format!("interval p0 has value: {}", iv.value(p0).is_some()));
    for l in &log {
        eprintln!("PROBE {l}");
    }
    assert!(ev.node_error(p0).is_none(), "p0 is ok");
}
