//! R2 review probes for PR #4674 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, desc, insert, len, scl, xy_frame};
use editor_core::{
    EntityKind, Entry, ExtrudeSide, Node, ProfileDoc, RecipeNodeId, RoleSeg, SplitHalf, StableName,
};
use geom_core::Tol;

pub fn box_split() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("r2_dm4_probes", Tol::witness());
    let (doc, plane) = insert(doc, xy_frame());
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            plane,
            vec![vec![(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]],
        )),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, tool) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: ext.into(),
            tool: tool.into(),
        },
    );
    (doc, ext, split)
}

pub fn dump(ev: &editor_core::Evaluation<f64>, var: editor_core::VarId, what: &str) {
    let t = ev.table(var).expect("a table");
    for (name, entry) in t.iter() {
        let tied = matches!(entry, Entry::Tied(_));
        println!("{what}: {:?} {:?} tied={tied}", name.kind, name.path);
    }
}

pub fn section(split: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: split,
        path: vec![RoleSeg::SectionFace { section: 0 }],
    }
}

/// The halves moved apart, then unioned: both carry the section face
/// name and the four wall fragments under one name each.
#[test]
fn r2_union_of_halves_moved_apart() {
    let (doc, _ext, split) = box_split();
    let above = doc.output(split, SplitHalf::Above.port()).unwrap();
    let below = doc.output(split, SplitHalf::Below.port()).unwrap();
    let (doc, up) = insert(
        doc,
        Node::transform(
            above,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(5.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let up_read = doc.outputs(up)[0];
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![up_read.into(), below.into()]),
            declare: Vec::new(),
        },
    );
    let ev = fixture::run(&doc, &Default::default());
    println!("union result: {:?}", ev.nodes.get(&u));
    let uv = doc.outputs(u)[0];
    if ev.table(uv).is_some() {
        dump(&ev, uv, "union");
    }
    println!(
        "section in union: {:?}",
        ev.table(uv).and_then(|t| t.lookup(&section(split)).cloned())
    );
}

/// A world placement of the LOWER half: its table is the lower half's.
#[test]
fn r2_place_lower_half() {
    let (doc, _ext, split) = box_split();
    let below = doc.output(split, SplitHalf::Below.port()).unwrap();
    let (doc, placed) = fixture::step(doc, editor_core::DocEdit::place(below, None));
    let placed = placed.unwrap();
    let ev = fixture::run(&doc, &Default::default());
    println!("placement: ok={:?}", ev.value(placed).is_some());
    let pv = doc.outputs(placed)[0];
    let t = ev.table(pv).expect("the placement's table");
    let lower = ev.table(below).unwrap();
    println!("placement rows {} lower rows {}", t.iter().count(), lower.iter().count());
    assert_eq!(t.iter().count(), lower.iter().count());
}

/// A key sited on a transform's output whose transform is deleted: the
/// loss is `NodeGone`; what do `resolve` at that read and the loss's
/// enrichment answer?
#[test]
fn r2_resolve_at_a_deleted_read() {
    let doc = ProfileDoc::empty_derived("r2_deleted_read", Tol::witness());
    let (doc, plane) = insert(doc, xy_frame());
    let (doc, pa) = insert(
        doc,
        Node::Profile(desc(
            plane,
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, a) = insert(
        doc,
        Node::Extrude {
            profile: pa.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, t) = insert(
        doc,
        fixture::xform(a, [0.0, 0.0, 2.0], [0.0, 0.0, 1.0], 0.0),
    );
    let t_read = doc.outputs(t)[0];
    let face = fixture::fname(a, RoleSeg::Cap(editor_core::CapEnd::End));
    let (doc, _) = fixture::step(
        doc,
        editor_core::DocEdit::SetAppearance {
            body: t_read,
            name: face.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    let before = fixture::run(&doc, &Default::default());
    let deleted = editor_core::apply(
        &doc,
        &editor_core::DocEdit::DeleteNode { id: t },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("delete applies");
    println!("delete maintenance: {:?}", deleted.maintenance);
    assert!(
        deleted.maintenance.iter().any(|m| matches!(
            m,
            editor_core::Maintenance::StrandedAppearance {
                took: editor_core::Took::Body,
                ..
            }
        )),
        "Took::Body strand"
    );
    let gone = deleted.doc;
    let ev = fixture::run(&gone, &Default::default());
    println!("losses: {:?}", ev.appearance.losses);
    let ctx = editor_core::RunCtx {
        doc: &gone,
        eval: &ev,
    };
    let r = editor_core::resolve(ctx, t_read, &face);
    println!("resolve at deleted read: {r:?}");
    let prior = editor_core::RunCtx {
        doc: &doc,
        eval: &before,
    };
    let r2 = editor_core::resolve_with_prior(ctx, prior, t_read, &face);
    println!("resolve_with_prior at deleted read: {r2:?}");
    for loss in &ev.appearance.losses {
        println!(
            "enriched: {:?}",
            editor_core::enrich_appearance_loss_with_prior(ctx, prior, loss)
        );
    }
    assert!(
        matches!(
            r,
            editor_core::Resolution::Failed(editor_core::ResolutionFailure {
                error: editor_core::ResolveError::NodeGone { .. },
                ..
            })
        ),
        "a read whose operation is deleted answers NodeGone"
    );
}

/// The flush finder across the two halves of one split: the section
/// face is one name on both sides.
#[test]
fn r2_flush_between_halves() {
    let (doc, _ext, split) = box_split();
    let above = doc.output(split, SplitHalf::Above.port()).unwrap();
    let below = doc.output(split, SplitHalf::Below.port()).unwrap();
    let ev = fixture::run(&doc, &Default::default());
    let found = editor_core::find_flush_candidates(&ev, above, below, Tol::witness());
    match &found {
        Ok(f) => {
            for x in f {
                println!("flush: {x:?}");
            }
        }
        Err(e) => println!("refused: {e:?}"),
    }
    let f = found.expect("the finder answers");
    assert_eq!(
        f.len(),
        5,
        "the section faces (rest) and four wall fragment continuations"
    );
}

fn tool_of(doc: &ProfileDoc, split: RecipeNodeId) -> RecipeNodeId {
    doc.upstream(split)
        .into_iter()
        .find(|&n| matches!(doc.node(n), Some(Node::Datum(_))))
        .unwrap()
}

/// The tombstone of a vanished section face resolved through the
/// LOWER half's read: which half's patch does it address?
#[test]
fn r2_tombstone_of_lower_half_section_face() {
    let (doc, _ext, split) = box_split();
    let below = doc.output(split, SplitHalf::Below.port()).unwrap();
    let tool = tool_of(&doc, split);
    let before = fixture::run(&doc, &Default::default());
    let prior = editor_core::RunCtx {
        doc: &doc,
        eval: &before,
    };
    let lower_face = match editor_core::resolve(prior, below, &section(split)) {
        editor_core::Resolution::Resolved(r) => r,
        other => panic!("{other:?}"),
    };
    println!("prior lower section face in value: {:?}", lower_face.entity);
    let (moved, _) = fixture::step(
        doc.clone(),
        editor_core::DocEdit::SetParam {
            node: tool,
            slot: editor_core::SlotId::Origin(editor_core::Axis3::Z),
            value: len(5.0).into(),
            fresh: Vec::new(),
        },
    );
    let after = fixture::run(&moved, &Default::default());
    let new = editor_core::RunCtx {
        doc: &moved,
        eval: &after,
    };
    let r = editor_core::resolve_with_prior(new, prior, below, &section(split));
    println!("lower site after move: {r:?}");
    let editor_core::Resolution::Failed(f) = &r else {
        panic!("not failed")
    };
    let editor_core::ResolveError::Vanished {
        last_good: Some(t), ..
    } = &f.error
    else {
        panic!("no tombstone")
    };
    assert_eq!(
        t.patch.entity, lower_face.entity,
        "the tombstone addresses the lower half's section face"
    );
}

/// An appearance key on a non-body read (a datum plane's output): does
/// SetAppearance refuse `AppearanceBodyNotARead`?
#[test]
fn r2_appearance_on_a_datum_read() {
    let (doc, ext, split) = box_split();
    let tool = tool_of(&doc, split);
    let tool_read = doc.outputs(tool)[0];
    let face = fixture::fname(ext, RoleSeg::Cap(editor_core::CapEnd::End));
    let r = editor_core::apply(
        &doc,
        &editor_core::DocEdit::SetAppearance {
            body: tool_read,
            name: face.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!(
        "datum read: {:?}",
        r.as_ref().map(|_| ()).map_err(|e| e.to_string())
    );
    assert!(
        r.is_err(),
        "a datum's output is not a body read, yet the paint is accepted"
    );
}

/// A key sited on a transform's output, its name minted upstream; the
/// transform's input re-pointed to another block takes the name out of
/// the read's reach. A selection there reports `Took::Reach`; does the
/// appearance key?
#[test]
fn r2_repoint_strands_appearance_key() {
    let doc = ProfileDoc::empty_derived("r2_repoint", Tol::witness());
    let (doc, plane) = insert(doc, xy_frame());
    let sq = |x: f64| vec![vec![(x, 0.0), (x + 1.0, 0.0), (x + 1.0, 1.0), (x, 1.0)]];
    let (doc, pa) = insert(doc, Node::Profile(desc(plane, sq(0.0))));
    let (doc, pb) = insert(doc, Node::Profile(desc(plane, sq(3.0))));
    let ext = |p: RecipeNodeId| Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    };
    let (doc, a) = insert(doc, ext(pa));
    let (doc, b) = insert(doc, ext(pb));
    let (doc, t) = insert(
        doc,
        fixture::xform(a, [0.0, 0.0, 2.0], [0.0, 0.0, 1.0], 0.0),
    );
    let t_read = doc.outputs(t)[0];
    let face = fixture::fname(a, RoleSeg::Cap(editor_core::CapEnd::End));
    let (doc, _) = fixture::step(
        doc,
        editor_core::DocEdit::SetAppearance {
            body: t_read,
            name: face.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    let ev = fixture::run(&doc, &Default::default());
    println!("losses before: {:?}", ev.appearance.losses);
    assert!(ev.appearance.is_lossless());
    let b_read = doc.outputs(b)[0];
    let applied = editor_core::apply(
        &doc,
        &editor_core::DocEdit::SetParam {
            node: t,
            slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Input),
            value: editor_core::Operand::from(b_read).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("re-point applies");
    println!("maintenance: {:?}", applied.maintenance);
    let ev2 = fixture::run(&applied.doc, &Default::default());
    println!("losses after: {:?}", ev2.appearance.losses);
    assert!(
        applied
            .maintenance
            .iter()
            .any(|m| matches!(m, editor_core::Maintenance::StrandedAppearance { .. })),
        "the re-point took the key's name out of its read's reach, unreported"
    );
}
