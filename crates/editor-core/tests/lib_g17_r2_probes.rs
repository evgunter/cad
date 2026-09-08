//! R2 review probes for LIB-G17 (`Node::Shell`). Not part of the unit;
//! each row attacks one claim of PR 2150 and records what happened.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::{self, body_of, cup, eval, failures, vessel};
use crate::fixture;

use editor_core::{
    CancelToken, DocEdit, EntityKind, Entry, EvalOptions, LoopProgram, Node, NodeErrorKind,
    NodeResult, PersistError, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SlotId,
    StableName, apply, evaluate, load, save,
};
use geom_core::Tol;
use topo::ShellError;

fn shelled(shell: RecipeNodeId, kind: EntityKind, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node: shell,
        path: vec![seg],
    }
}

fn blank_of(doc: &ProfileDoc) -> RecipeNodeId {
    doc.order()
        .iter()
        .copied()
        .find(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
        .expect("the cup's blank")
}

fn refusal(doc: &ProfileDoc, node: RecipeNodeId) -> NodeErrorKind {
    let mut ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.remove(&node) {
        Some(NodeResult::Failed(e)) => e.kind,
        other => panic!("expected a refusal at {node:?}, got {other:?}"),
    }
}

/// Names in a table, sorted, with the `Rim` rows stripped.
fn names_minus_rim(t: &editor_core::NameTable) -> Vec<StableName> {
    let mut v: Vec<StableName> = t
        .iter()
        .filter(|(n, _)| !matches!(n.path.first(), Some(RoleSeg::Rim(_))))
        .map(|(n, _)| n.clone())
        .collect();
    v.sort();
    v
}

/// P1 — dispatcher's question: is the rim's identity the ONLY thing the
/// order of `open` changes? Mass bits, counts, every other name.
#[test]
fn p1_order_swap_changes_only_the_rim_name() {
    let a = vessel::document();
    let b = vessel::document_with_mouth(|pot| {
        [
            vessel::band_pi(pot, vessel::SEG_MOUTH),
            vessel::band(pot, vessel::SEG_MOUTH),
        ]
    });
    let (ea, eb) = (eval::<f64>(&a.doc), eval::<f64>(&b.doc));
    assert!(failures(&ea).is_empty() && failures(&eb).is_empty());
    let (sa, sb) = (a.result.unwrap(), b.result.unwrap());
    let (ba, bb) = (body_of(&ea, sa), body_of(&eb, sb));
    let ma = topo::mass_properties(ba, Tol::witness()).unwrap();
    let mb = topo::mass_properties(bb, Tol::witness()).unwrap();
    eprintln!(
        "P1 volumes {:?} vs {:?} (bit-equal: {}), areas {:?} vs {:?} (bit-equal: {})",
        ma.volume,
        mb.volume,
        ma.volume.to_bits() == mb.volume.to_bits(),
        ma.surface_area,
        mb.surface_area,
        ma.surface_area.to_bits() == mb.surface_area.to_bits()
    );
    assert_eq!(ba.faces().count(), bb.faces().count());
    assert_eq!(ba.edges().count(), bb.edges().count());
    assert_eq!(ba.vertices().count(), bb.vertices().count());
    let ta = &ea.value(sa).unwrap().name_table;
    let tb = &eb.value(sb).unwrap().name_table;
    assert_eq!(
        names_minus_rim(ta),
        names_minus_rim(tb),
        "every non-rim name must be the same under either order"
    );
    assert_eq!(ta.iter().count(), tb.iter().count());
    // The volumes should agree exactly; record if they do not.
    assert_eq!(ma.volume.to_bits(), mb.volume.to_bits(), "volume bits moved with the order");
}

/// P1b — two designated faces on DISTINCT charts: the order carries no
/// meaning (each rim is named for its own chart's only face), yet the
/// key still discriminates. Measured, not judged here.
#[test]
fn p1b_order_on_distinct_charts_moves_the_key_but_nothing_else() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let (da, sa) = fixture::insert(
        d.doc.clone(),
        Node::shell(blank, fixture::len(cup::T), vec![cup::top(blank), cup::bottom(blank)]),
    );
    let (db, sb) = fixture::insert(
        d.doc.clone(),
        Node::shell(blank, fixture::len(cup::T), vec![cup::bottom(blank), cup::top(blank)]),
    );
    let (ea, eb) = (eval::<f64>(&da), eval::<f64>(&db));
    assert!(failures(&ea).is_empty(), "{:?}", failures(&ea));
    assert!(failures(&eb).is_empty(), "{:?}", failures(&eb));
    let ta = &ea.value(sa).unwrap().name_table;
    let tb = &eb.value(sb).unwrap().name_table;
    let mut na: Vec<_> = ta.iter().map(|(n, _)| n.clone()).collect();
    let mut nb: Vec<_> = tb.iter().map(|(n, _)| n.clone()).collect();
    na.sort();
    nb.sort();
    assert_eq!(na, nb, "two charts, two rims, the same names either way");
    eprintln!(
        "P1b keys differ: {}",
        ea.value(sa).unwrap().content_key != eb.value(sb).unwrap().content_key
    );
    let body = body_of(&ea, sa);
    eprintln!(
        "P1b tube-cup: faces {}, shells {}, V {:?}",
        body.faces().count(),
        body.shells().count(),
        topo::mass_properties(body, Tol::witness()).unwrap().volume
    );
}

/// P2 — the variant is public: a raw `Node::Shell { open: [x, x] }`
/// inserted through `DocEdit::InsertNode` bypasses `Node::shell`. Does
/// the edit door accept it, what does evaluation say, and does the file
/// it saves then refuse to load?
#[test]
fn p2_raw_variant_with_a_repeat_bypasses_the_construction_door() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let raw = Node::Shell {
        target: blank,
        thickness: fixture::len(cup::T),
        open: vec![cup::top(blank), cup::top(blank)],
    };
    let out = apply(&d.doc, &DocEdit::InsertNode { node: raw }, Tol::witness());
    let (doc, id) = match out {
        Ok(o) => (o.doc.clone(), o.record.minted.expect("minted")),
        Err(e) => panic!("P2: the edit door REFUSED the raw repeat: {e:?}"),
    };
    let e = refusal(&doc, id);
    eprintln!("P2 evaluation of the raw repeat: {e}");
    assert!(
        matches!(&e, NodeErrorKind::Shell(inner) if matches!(**inner, ShellError::OpenFaceRepeated { .. })),
        "{e:?}"
    );
    // Measured: `save` itself refuses (validate_snapshot runs on save),
    // so the in-memory document the edit door accepted cannot be
    // persisted at all.
    match save(&doc, &[], Tol::witness()) {
        Err(PersistError::Snapshot(editor_core::SnapshotError::ShellOpenRepeated { .. })) => {
            eprintln!("P2: the edit door accepted a document that SAVE refuses");
        }
        Ok(text) => match load(&text, Tol::witness()) {
            Err(PersistError::Snapshot(editor_core::SnapshotError::ShellOpenRepeated { .. })) => {
                eprintln!("P2: saved, and the load door refuses");
            }
            other => panic!("P2: expected the load refusal, got {other:?}"),
        },
        Err(other) => panic!("P2: unexpected save refusal {other:?}"),
    }
}

/// P3 — `Rebind` onto an already-designated face: the list shrinks and
/// the surviving entry keeps the EARLIER position (three entries, so
/// the position is observable).
#[test]
fn p3_rebind_keeps_the_earlier_position() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let a = fixture::fname(blank, fixture::wall(0));
    let b = fixture::fname(blank, fixture::wall(1));
    let c = fixture::fname(blank, fixture::wall(2));
    let (doc, id) = fixture::insert(
        d.doc.clone(),
        Node::shell(blank, fixture::len(cup::T), vec![a.clone(), b.clone(), c.clone()]),
    );
    let open_of = |doc: &ProfileDoc| match doc.node(id) {
        Some(Node::Shell { open, .. }) => open.clone(),
        other => panic!("{other:?}"),
    };
    // c → a: a is earlier, so [a, b].
    let r = apply(
        &doc,
        &DocEdit::Rebind {
            from: c.clone(),
            to: a.clone(),
        },
        Tol::witness(),
    )
    .expect("rebind")
    .doc;
    assert_eq!(open_of(&r), vec![a.clone(), b.clone()]);
    // a → c: the rebound entry sits at a's OLD position, so [c, b] —
    // c now carries the rim, though it was named last.
    let r = apply(
        &doc,
        &DocEdit::Rebind {
            from: a.clone(),
            to: c.clone(),
        },
        Tol::witness(),
    )
    .expect("rebind")
    .doc;
    assert_eq!(open_of(&r), vec![c.clone(), b.clone()]);
}

/// P4 — a bump to a wall that changes the topology class (t ≥ L/2): the
/// refusal is typed, carries the numbers, and what do they read?
#[test]
fn p4_thick_wall_bump_refuses_typed_with_numbers() {
    let d = cup::document();
    let shell = d.result.unwrap();
    for t in [0.5, 0.625] {
        let bumped = apply(
            &d.doc,
            &DocEdit::SetParam {
                node: shell,
                slot: SlotId::ShellThickness,
                expr: fixture::len(t),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
        let e = refusal(&bumped, shell);
        eprintln!("P4 t={t}: {e}");
        match &e {
            NodeErrorKind::Shell(inner) => match **inner {
                ShellError::WallClearance { gap, needed, .. } => {
                    eprintln!("P4 t={t}: gap={gap:?} needed={needed:?}");
                }
                ref other => eprintln!("P4 t={t}: other kernel refusal {other:?}"),
            },
            other => panic!("P4: not the shell's refusal: {other:?}"),
        }
    }
}

/// P5 — the Interval lane: the fold takes `lo()`. Compare the folded
/// document refusal against the kernel's own Display at the lane.
#[cfg(feature = "interval")]
#[test]
fn p5_the_fold_at_interval_reports_lo_and_the_text_moves() {
    use geom_core::{Interval, Real};
    let d = cup::document();
    let shell = d.result.unwrap();
    let blank = blank_of(&d.doc);
    for t in [0.625, -0.125] {
        let bumped = apply(
            &d.doc,
            &DocEdit::SetParam {
                node: shell,
                slot: SlotId::ShellThickness,
                expr: fixture::len(t),
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
        let ev = eval::<Interval>(&bumped);
        let folded = match ev.nodes.get(&shell) {
            Some(NodeResult::Failed(e)) => e.kind.to_string(),
            other => panic!("{other:?}"),
        };
        // The kernel's own text at the lane.
        let body = body_of(&ev, blank);
        let top = match ev.value(blank).unwrap().name_table.lookup(&cup::top(blank)) {
            Some(Entry::Unique(r)) => match r.key {
                editor_core::EntityKey::Face(k) => k,
                _ => panic!(),
            },
            other => panic!("{other:?}"),
        };
        let kernel = topo::shell_open(body, Interval::from_f64(t), &[top], Tol::witness())
            .err()
            .map(|e| e.to_string())
            .unwrap();
        eprintln!("P5 t={t}\n  folded: {folded}\n  kernel: {kernel}");
        let f64_text = refusal(&bumped, shell).to_string();
        eprintln!("  f64:    {f64_text}");
    }
}

/// P6 — change WHICH face is designated through `Rebind` (top → bottom)
/// and re-evaluate: `Rim(bottom)` resolves, `Rim(top)` is gone, the
/// closed forms are the same cup upside down.
#[test]
fn p6_rebinding_the_designation_moves_the_rim() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let shell = d.result.unwrap();
    let doc = apply(
        &d.doc,
        &DocEdit::Rebind {
            from: cup::top(blank),
            to: cup::bottom(blank),
        },
        Tol::witness(),
    )
    .unwrap()
    .doc;
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let t = &ev.value(shell).unwrap().name_table;
    let rim_bottom = shelled(
        shell,
        EntityKind::Face,
        RoleSeg::Rim(Box::new(cup::bottom(blank))),
    );
    let rim_top = shelled(
        shell,
        EntityKind::Face,
        RoleSeg::Rim(Box::new(cup::top(blank))),
    );
    let inner_top = shelled(
        shell,
        EntityKind::Face,
        RoleSeg::Inner(Box::new(cup::top(blank))),
    );
    assert!(matches!(t.lookup(&rim_bottom), Some(Entry::Unique(_))));
    assert!(t.lookup(&rim_top).is_none());
    assert!(matches!(t.lookup(&inner_top), Some(Entry::Unique(_))));
    let m = topo::mass_properties(body_of(&ev, shell), Tol::witness()).unwrap();
    assert_eq!(m.volume, cup::closed_forms(cup::L, cup::H, cup::T).volume);
}

/// P7 — a designated face WITH A HOLE: the `HoleRim` role has no row
/// anywhere in the unit. Is it reachable through the door at all?
#[test]
fn p7_a_holed_designated_face_mints_a_hole_rim() {
    let mut r = corpus::Recorder::new();
    let outer = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).unwrap();
    let hole =
        LoopProgram::polygon([(0.375, 0.375), (0.625, 0.375), (0.625, 0.625), (0.375, 0.625)])
            .unwrap();
    let plane = r.insert(fixture::xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![outer, hole],
    }));
    let blank = r.insert(Node::Extrude {
        profile,
        distance: fixture::len(1.0),
    });
    let shell = r.insert(Node::shell(
        blank,
        fixture::len(0.0625),
        vec![cup::top(blank)],
    ));
    let ev = eval::<f64>(&r.doc);
    let bad = failures(&ev);
    if !bad.is_empty() {
        eprintln!("P7: refused:\n{}", bad.join("\n"));
        return;
    }
    let t = &ev.value(shell).unwrap().name_table;
    let hole_rims: Vec<_> = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::HoleRim { .. })))
        .map(|(n, _)| n.clone())
        .collect();
    let body = body_of(&ev, shell);
    eprintln!(
        "P7: faces {}, hole rims {:?}, valid {:?} closed {:?}",
        body.faces().count(),
        hole_rims,
        topo::validate(body),
        topo::validate_closed(body)
    );
    assert_eq!(hole_rims.len(), 1, "one hole, one hole rim");
}
