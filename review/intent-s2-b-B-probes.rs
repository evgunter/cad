//! Reviewer B's probes for PR 4342 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, axis_in_plane, insert, len};
use crate::wire::up_to_ids;
use editor_core::{
    DocEdit, EditError, ExtrudeSide, MeasureExpr, Node, Operand, OperandSlot,
    ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, apply, load, save,
};
use geom_core::Tol;

fn set(node: RecipeNodeId, slot: OperandSlot, read: Operand) -> DocEdit<ProfileProgram> {
    DocEdit::SetParam {
        node,
        slot: SlotId::Operand(slot),
        value: read.into(),
        fresh: Vec::new(),
    }
}

fn try_apply(
    doc: &ProfileDoc,
    edit: &DocEdit<ProfileProgram>,
) -> Result<editor_core::Applied<ProfileProgram>, EditError> {
    apply(doc, edit, Tol::witness(), &editor_core::RefusingReach)
}

/// P1: an assertion's measure re-pointed through the slot door at a
/// measure of another dimension. The insert door refuses this
/// combination `AssertionDimension`; does the slot door?
#[test]
fn p1_assertion_repointed_across_dimensions() {
    let mut r = fixture::Recorder::new();
    let profile = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let _ = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let m_len = r.insert(Node::Measure {
        expr: MeasureExpr::value(len(1.0)),
        refs: Vec::new(),
    });
    let m_ang = r.insert(Node::Measure {
        expr: MeasureExpr::value(ang(0.5)),
        refs: Vec::new(),
    });
    let assertion = r.insert(Node::Assertion {
        measure: m_len.into(),
        bound: len(0.5),
        dir: editor_core::AssertionDir::AtLeast,
    });
    // Control: the insert door refuses the angle measure with a length bound.
    let insert_refusal = try_apply(
        &r.doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Assertion {
                measure: m_ang.into(),
                bound: len(0.5),
                dir: editor_core::AssertionDir::AtLeast,
            }),
            fresh: Vec::new(),
        },
    );
    println!("P1 insert door: {:?}", insert_refusal.as_ref().err());
    let edit = set(assertion, OperandSlot::Measure, m_ang.into());
    let out = try_apply(&r.doc, &edit);
    match &out {
        Err(e) => println!("P1 slot door refused: {e:?}"),
        Ok(applied) => {
            println!("P1 slot door ACCEPTED");
            let snap = save(&applied.doc, &[], Tol::witness());
            println!("P1 save(snapshot): {:?}", snap.as_ref().err());
            if let Ok(text) = snap {
                println!("P1 load: {:?}", load(&text, Tol::witness()).err());
            }
            let mut edits = r.edits.clone();
            edits.push(edit.clone());
            let logged = save(&ProfileDoc::empty_derived("mod", Tol::witness()), &edits, Tol::witness());
            println!("P1 save(with log): {:?}", logged.as_ref().err());
            if let Ok(text) = logged {
                match load(&text, Tol::witness()) {
                    Ok(l) => println!("P1 load(with log): OK, bit_eq={}", l.doc.bit_eq(&applied.doc)),
                    Err(e) => println!("P1 load(with log): {e:?}"),
                }
            }
            let ev = fixture::run(&applied.doc, &editor_core::EvalOptions::default());
            println!("P1 eval assertion: {:?}", ev.node_error(assertion));
        }
    }
    assert!(
        out.is_err(),
        "the slot door must refuse what the insert door refuses"
    );
}

/// P2: `Operand::Node(revolve)` at an Axis seat. `Operand::Node`'s doc
/// says "its one output, or the one output of the seat's kind".
#[test]
fn p2_node_sugar_at_an_axis_seat() {
    let (doc, plane, p) = fixture::on_frame_keeping(
        ProfileDoc::empty_derived("p2", Tol::witness()),
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(1.5, 0.5, 0.5)],
    );
    let (doc, axis) = insert(doc, axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, rev) = insert(
        doc,
        Node::Revolve {
            profile: p.into(),
            axis: axis.into(),
            angle: ang(1.0),
        },
    );
    let by_node = try_apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Revolve {
                profile: p.into(),
                axis: rev.into(),
                angle: ang(1.0),
            }),
            fresh: Vec::new(),
        },
    );
    let by_port = try_apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Revolve {
                profile: p.into(),
                axis: Operand::output(rev, 1),
                angle: ang(1.0),
            }),
            fresh: Vec::new(),
        },
    );
    println!("P2 Node(revolve) at axis seat: {:?}", by_node.as_ref().err());
    println!("P2 Output(revolve,1) at axis seat: {:?}", by_port.as_ref().err());
}

/// P3: DM5 over operations read: the two halves of one split in one
/// union by port refuse, by `Part` accept.
#[test]
fn p3_split_halves_two_spellings() {
    let doc = ProfileDoc::empty_derived("p3", Tol::witness());
    let (doc, _, profile) = fixture::on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: block.into(),
            tool: plane.into(),
        },
    );
    let union = |members: Vec<Operand>| DocEdit::InsertNode {
        node: Box::new(Node::Union {
            members,
            declare: Vec::new(),
        }),
        fresh: Vec::new(),
    };
    let by_port = try_apply(
        &doc,
        &union(vec![Operand::output(split, 0), Operand::output(split, 1)]),
    );
    println!("P3 union by port: {:?}", by_port.as_ref().err());
    let (doc, above) = insert(
        doc,
        Node::Part {
            of: split.into(),
            select: editor_core::PartSelect::SplitHalf(editor_core::SplitHalf::Above),
        },
    );
    let (doc, below) = insert(
        doc,
        Node::Part {
            of: split.into(),
            select: editor_core::PartSelect::SplitHalf(editor_core::SplitHalf::Below),
        },
    );
    let by_part = try_apply(&doc, &union(vec![above.into(), below.into()]));
    println!("P3 union by part: {:?}", by_part.as_ref().err().map(|e| e.to_string()));
    if let Ok(a) = &by_part {
        let ev = fixture::run(&a.doc, &editor_core::EvalOptions::default());
        let u = a.record.minted.unwrap();
        println!("P3 union-by-part eval: {:?}", ev.node_error(u));
    }

    // P5: the comparator is blind to a port. A boolean reading the
    // split's `above` and the same boolean reading `below`.
    let (doc, other) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    let boolean = |a: Operand| Node::Boolean {
        op: editor_core::BooleanOp::Union,
        a,
        b: other.into(),
        declare: Vec::new(),
    };
    let (d0, _) = insert(doc.clone(), boolean(Operand::output(split, 0)));
    let (d1, _) = insert(doc, boolean(Operand::output(split, 1)));
    let w = |d: &ProfileDoc| {
        let t = save(d, &[], Tol::witness()).expect("saves");
        up_to_ids::reads_as_inputs(&crate::wire::wire_body(&t))
    };
    let (w0, w1) = (w(&d0), w(&d1));
    println!(
        "P5 raw json equal: {}",
        crate::wire::wire_body(&save(&d0, &[], Tol::witness()).unwrap())
            == crate::wire::wire_body(&save(&d1, &[], Tol::witness()).unwrap())
    );
    let verdict = up_to_ids::same_up_to_ids(&w0, &w1);
    println!("P5 comparator on above vs below: {verdict:?}");
    assert!(
        verdict.is_err(),
        "a read re-pointed from one port to the other must be a mismatch"
    );
}

/// P4: a re-point recorded in the log replays bit-exact through save
/// and load (C2's replay half), including a re-point that strands.
#[test]
fn p4_repoint_replays_through_the_log() {
    let mut r = fixture::Recorder::new();
    let pa = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let pb = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(3.5, 0.5, 0.5)],
    );
    let a = r.insert(Node::Extrude {
        profile: pa.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let b = r.insert(Node::Extrude {
        profile: pb.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let ev = fixture::run(&r.doc, &editor_core::EvalOptions::default());
    let edges: Vec<_> = editor_core::all_edges(&ev, a).into_iter().take(1).collect();
    let fillet = r.insert(Node::fillet(a, len(0.1), edges));
    let before = r.doc.clone();
    {
        let text = save(&ProfileDoc::empty_derived("mod", Tol::witness()), &r.edits, Tol::witness()).expect("control saves");
        let loaded = load(&text, Tol::witness()).expect("control loads");
        println!("P4 control (no re-point) bit_eq: {}", loaded.doc.bit_eq(&r.doc));
    }
    let applied = try_apply(&r.doc, &set(fillet, OperandSlot::Target, b.into())).unwrap();
    println!("P4 maintenance: {:?}", applied.maintenance);
    r.push(set(fillet, OperandSlot::Target, b.into()));
    let text = save(&ProfileDoc::empty_derived("mod", Tol::witness()), &r.edits, Tol::witness()).expect("a re-point saves with its log");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    let resaved = save(&loaded.doc, &[], Tol::witness()).unwrap();
    let mine = save(&r.doc, &[], Tol::witness()).unwrap();
    std::fs::write("/tmp/claude-0/p4_loaded.txt", &resaved).unwrap();
    std::fs::write("/tmp/claude-0/p4_recorded.txt", &mine).unwrap();
    println!("P4 snapshot text equal: {}", resaved == mine);
    assert!(loaded.doc.bit_eq(&r.doc), "replay is bit-exact");
    // and back
    r.push(set(fillet, OperandSlot::Target, a.into()));
    println!("P4 round trip equals before: {}", r.doc.bit_eq(&before));
    let text = save(&ProfileDoc::empty_derived("mod", Tol::witness()), &r.edits, Tol::witness()).expect("saves");
    assert!(load(&text, Tol::witness()).unwrap().doc.bit_eq(&r.doc));
}

/// P6: each pre-B corpus file, loaded by this build: which refusal, and its text.
#[test]
fn p6_pre_b_files_on_this_build() {
    let base = std::path::PathBuf::from("/home/user/base-b");
    for file in [
        "crates/editor-core/tests/golden/golden.cad",
        "crates/editor-core/tests/corpus/die_tool.pncad",
        "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
        "crates/viewer/tests/gallery_ring.pncad",
        "crates/pncad/tests/plate_param.pncad",
    ] {
        let text = std::fs::read_to_string(base.join(file)).unwrap();
        match load(&text, Tol::witness()) {
            Ok(_) => println!("P6 {file}: LOADS"),
            Err(e) => {
                let shown = e.to_string();
                println!(
                    "P6 {file}: {} | regenerate-recourse-in-text={} | {}",
                    format!("{e:?}").chars().take(60).collect::<String>(),
                    shown.contains(editor_core::REGENERATE_RECOURSE),
                    shown.chars().take(400).collect::<String>()
                );
            }
        }
    }
}

/// P7: a placer whose input is deleted: the delete strands it; does the
/// stranded document save and load (its output's kind read off a dead input)?
#[test]
fn p7_stranded_transform_saves_and_loads() {
    let mut r = fixture::Recorder::new();
    let p = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let x = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let t = r.insert(fixture::xform(x, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    let t2 = r.insert(fixture::xform(t, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    let del = try_apply(&r.doc, &DocEdit::DeleteNode { id: x }).expect("delete accepted");
    println!("P7 maintenance kinds: {:?}", del.maintenance.iter().map(|m| format!("{m}")).collect::<Vec<_>>());
    println!("P7 signature(t) after delete: {:?}", del.doc.signature(t));
    let snap = save(&del.doc, &[], Tol::witness());
    println!("P7 save: {:?}", snap.as_ref().err().map(|e| e.to_string()));
    if let Ok(text) = snap {
        println!("P7 load: {:?}", load(&text, Tol::witness()).err().map(|e| e.to_string()));
    }
    let mut edits = r.edits.clone();
    edits.push(DocEdit::DeleteNode { id: x });
    let logged = save(&ProfileDoc::empty_derived("mod", Tol::witness()), &edits, Tol::witness());
    println!("P7 save(log): {:?}", logged.as_ref().err().map(|e| e.to_string()));
    // re-point the stranded transform at a fresh body: kind is fixed at minting
    let (doc2, y) = insert(del.doc.clone(), Node::Extrude {
        profile: p.into(),
        distance: len(2.0),
        side: ExtrudeSide::Along,
    });
    println!("P7 re-point stranded t at body: {:?}", try_apply(&doc2, &set(t, OperandSlot::Input, y.into())).err());
    let ev = fixture::run(&del.doc, &editor_core::EvalOptions::default());
    println!("P7 t={t:?} t2={t2:?} err(t2)={:?}", ev.node_error(t2));
}

/// P8: a hand-written file whose union reads its own transform's output.
#[test]
fn p8_read_cycle_at_load() {
    let mut r = fixture::Recorder::new();
    let p = r.profile([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![fixture::square(0.5, 0.5, 0.5)]);
    let q = r.profile([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![fixture::square(3.5, 0.5, 0.5)]);
    let a = r.insert(Node::Extrude { profile: p.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let b = r.insert(Node::Extrude { profile: q.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let u = r.insert(Node::Union { members: vec![a.into(), b.into()], declare: Vec::new() });
    let t = r.insert(fixture::xform(u, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    let text = save(&r.doc, &[], Tol::witness()).unwrap();
    let (out_b, out_t) = (r.doc.output(b, 0).unwrap().0.to_string(), r.doc.output(t, 0).unwrap().0.to_string());
    let (header, body) = text.split_once('\n').unwrap();
    let mut v: serde_json::Value = serde_json::from_str(body).unwrap();
    for node in v["snapshot"]["nodes"].as_object_mut().unwrap().values_mut() {
        if let Some(m) = node.get_mut("Union").and_then(|u| u.get_mut("members")) {
            for x in m.as_array_mut().unwrap() {
                if x.as_str() == Some(out_b.as_str()) {
                    *x = serde_json::json!(out_t);
                }
            }
        }
    }
    let corrupt = format!("{header}\n{}\n", serde_json::to_string_pretty(&v).unwrap());
    assert_ne!(corrupt, text);
    println!("P8 load: {:?}", load(&corrupt, Tol::witness()).err().map(|e| format!("{e:?}").chars().take(200).collect::<String>()));
}

fn points(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> Vec<[u64; 3]> {
    let body = match ev.value(id).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => b,
        Some(editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. })) => body,
        other => panic!("not a body: {other:?} / {:?}", ev.node_error(id)),
    };
    let mut p: Vec<[u64; 3]> = body.points().map(|(_, p)| p.to_array().map(f64::to_bits)).collect();
    p.sort_unstable();
    p
}

/// P9: the memo (D4) after re-pointing a read from a split's `above`
/// port to its `below` port: the content key feeds the split's key, not
/// the port.
#[test]
fn p9_memo_across_a_port_repoint() {
    let doc = ProfileDoc::empty_derived("p9", Tol::witness());
    let (doc, _, profile) = fixture::on_frame_keeping(
        doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let (doc, block) = insert(doc, Node::Extrude { profile: profile.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let (doc, plane) = insert(doc, Node::Datum(editor_core::Datum::Plane {
        origin: [len(0.0), len(0.0), len(0.5)],
        normal: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
    }));
    let (doc, split) = insert(doc, Node::Split { target: block.into(), tool: plane.into() });
    let (doc, _, far) = fixture::on_frame_keeping(
        doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![fixture::square(5.5, 0.5, 0.5)],
    );
    let (doc, other) = insert(doc, Node::Extrude { profile: far.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let (doc0, boolean) = insert(doc, Node::Boolean {
        op: editor_core::BooleanOp::Union,
        a: Operand::output(split, 0),
        b: other.into(),
        declare: Vec::new(),
    });
    let opts = editor_core::EvalOptions::default();
    let cancel = editor_core::CancelToken::new();
    let ev0 = editor_core::evaluate::<f64>(&doc0, None, &cancel, &opts, Tol::witness());
    let doc1 = try_apply(&doc0, &set(boolean, OperandSlot::A, Operand::output(split, 1))).unwrap().doc;
    let fresh = editor_core::evaluate::<f64>(&doc1, None, &cancel, &opts, Tol::witness());
    let memo = editor_core::evaluate::<f64>(&doc1, Some(&ev0), &cancel, &opts, Tol::witness());
    let k = |ev: &editor_core::Evaluation<f64>| ev.value(boolean).map(|v| v.content_key);
    println!("P9 key above={:?} below={:?}", k(&ev0), k(&fresh));
    let (p0, pf, pm) = (points(&ev0, boolean), points(&fresh, boolean), points(&memo, boolean));
    println!("P9 above-vs-below bodies differ: {}", p0 != pf);
    println!("P9 memo equals fresh: {} ; memo equals stale(above): {}", pm == pf, pm == p0);
    assert_eq!(pm, pf, "the memo must not return the other half's body");
}

/// P10: DM5 "over the operations read": a circular pattern of a
/// revolve's body about that revolve's own `axis` port (FORK-1b gave
/// the revolve the port so it can be read).
#[test]
fn p10_pattern_about_its_revolves_own_axis() {
    let (doc, plane, p) = fixture::on_frame_keeping(
        ProfileDoc::empty_derived("p10", Tol::witness()),
        [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![fixture::square(1.5, 0.5, 0.5)],
    );
    let (doc, axis) = insert(doc, axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, rev) = insert(doc, Node::Revolve { profile: p.into(), axis: axis.into(), angle: ang(0.5) });
    let pattern = |about: Operand| DocEdit::InsertNode {
        node: Box::new(Node::Pattern {
            input: rev.into(),
            count: editor_core::Formula::count(3),
            kind: editor_core::PatternKind::Circular { axis: about, step: ang(1.0) },
        }),
        fresh: Vec::new(),
    };
    let by_port = try_apply(&doc, &pattern(Operand::output(rev, 1)));
    let by_datum = try_apply(&doc, &pattern(axis.into()));
    println!("P10 about rev.axis port: {:?}", by_port.as_ref().err());
    println!("P10 about the datum the revolve reads: {:?}", by_datum.as_ref().err());
}
