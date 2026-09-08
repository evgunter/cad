//! **The R2 review's rows for the shell door**, adopted into the suite:
//! each attacks one claim of the door and asserts what it found.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::{self, body_of, cup, eval, failures, vessel};
use crate::fixture;

use editor_core::{
    CancelToken, DocEdit, EntityKind, Entry, EvalOptions, LoopProgram, Node, NodeErrorKind,
    NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SlotId, StableName, apply,
    evaluate,
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
    let b = vessel::document_with_open(|pot| {
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
    assert_eq!(
        ma.volume.to_bits(),
        mb.volume.to_bits(),
        "volume bits moved with the order"
    );
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
        Node::shell(
            blank,
            fixture::len(cup::T),
            vec![cup::top(blank), cup::bottom(blank)],
        ),
    );
    let (db, sb) = fixture::insert(
        d.doc.clone(),
        Node::shell(
            blank,
            fixture::len(cup::T),
            vec![cup::bottom(blank), cup::top(blank)],
        ),
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
    // The key discriminates on order even across distinct charts: a
    // harmless over-discrimination (two memo entries for one body),
    // stated at `feed_shell` and pinned here as the fact it is.
    assert_ne!(
        ea.value(sa).unwrap().content_key,
        eb.value(sb).unwrap().content_key,
        "the key reads the order even where the order carries no rim"
    );
    // Two opposite faces opened: a square tube, one shell, ten faces
    // (four outer walls, two rims, four cavity walls), and the exact
    // volume `L²H − (L−2t)²H`.
    let body = body_of(&ea, sa);
    assert_eq!(body.faces().count(), 10);
    assert_eq!(body.shells().count(), 1);
    let inner = cup::L - 2.0 * cup::T;
    assert_eq!(
        topo::mass_properties(body, Tol::witness()).unwrap().volume,
        cup::L * cup::L * cup::H - inner * inner * cup::H
    );
}

/// P2 — the variant is public: a raw `Node::Shell { open: [x, x] }`
/// inserted through `DocEdit::InsertNode` bypasses `Node::shell`. The
/// insert door refuses it TYPED, through the same `Node::input_fault`
/// the load door asks (`lib_g17_shell_node::a_repeated_open_entry_is_refused_at_load`
/// is that half), so no document holding a repeat can exist to save.
#[test]
fn p2_raw_variant_with_a_repeat_is_refused_at_the_insert_door() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let raw = Node::Shell {
        target: blank,
        thickness: fixture::len(cup::T),
        open: vec![cup::top(blank), cup::bottom(blank), cup::top(blank)],
    };
    match apply(&d.doc, &DocEdit::InsertNode { node: raw }, Tol::witness()) {
        Err(editor_core::EditError::RepeatedDesignation {
            first: 0, again: 2, ..
        }) => {}
        other => panic!("P2: the edit door must refuse the raw repeat typed, got {other:?}"),
    }
    // And the construction door, handed the same list, keeps the first
    // occurrence — the repair the refusal's text names.
    let Node::Shell { open, .. }: Node<ProfileProgram> = Node::shell(
        blank,
        fixture::len(cup::T),
        vec![cup::top(blank), cup::bottom(blank), cup::top(blank)],
    ) else {
        panic!("the door builds a shell")
    };
    assert_eq!(open, vec![cup::top(blank), cup::bottom(blank)]);
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
        Node::shell(
            blank,
            fixture::len(cup::T),
            vec![a.clone(), b.clone(), c.clone()],
        ),
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
        match &e {
            NodeErrorKind::Shell(inner) => match **inner {
                // The two facing walls are `L` apart and the two
                // offsets need `2t`: at `t = L/2` the gate refuses
                // because the margin is not certifiably positive, not
                // because it is negative.
                ShellError::WallClearance { gap, needed, .. } => {
                    assert_eq!(gap, cup::L, "the facing walls are the blank's side apart");
                    assert_eq!(needed, 2.0 * t, "two offsets need twice the wall");
                }
                ref other => panic!("P4 t={t}: expected the clearance gate, got {other:?}"),
            },
            other => panic!("P4: not the shell's refusal: {other:?}"),
        }
    }
}

/// P5 — **the Interval lane's witness, through the document**: the
/// wall is a parameter widened into a genuine bracket (`lo() ≠ hi()`)
/// by a parameter box, and the folded refusal reports the end each
/// field declares — the needed clearance at its SUPREMUM, the refused
/// thickness at its INFIMUM. A fold reading one end everywhere reds
/// on one of the two.
#[cfg(feature = "interval")]
#[test]
fn p5_the_interval_witness_reports_the_declared_end_of_a_widened_parameter() {
    use editor_core::analysis::{BoxAxis, ParamBox};
    use editor_core::{Dimension, DocParam, Expr, ParamName, UnitSym};
    use geom_core::Interval;
    use std::collections::BTreeMap;
    use std::sync::Arc;

    let width = 1.0 / 64.0;
    let shelled_at = |nominal: f64| -> (ProfileDoc, RecipeNodeId) {
        let d = cup::document();
        let blank = blank_of(&d.doc);
        let doc = apply(
            &d.doc,
            &DocEdit::SetDocParam {
                name: ParamName::new("t"),
                value: DocParam::Continuous {
                    dim: Dimension::Length,
                    value: nominal,
                    display_unit: UnitSym::canonical_for(Dimension::Length),
                    distribution: None,
                },
            },
            Tol::witness(),
        )
        .expect("the parameter declares")
        .doc;
        fixture::insert(
            doc,
            Node::shell(
                blank,
                Expr::param(ParamName::new("t"), Dimension::Length),
                vec![cup::top(blank)],
            ),
        )
    };
    let widened = || EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(BTreeMap::from([(
            ParamName::new("t"),
            BoxAxis::Varying {
                lo: -width,
                hi: width,
            },
        )])))),
        ..EvalOptions::default()
    };
    let refused = |doc: &ProfileDoc, node: RecipeNodeId| -> ShellError<f64> {
        let mut ev =
            evaluate::<Interval>(doc, None, &CancelToken::new(), &widened(), Tol::witness());
        match ev.nodes.remove(&node) {
            Some(NodeResult::Failed(e)) => match e.kind {
                NodeErrorKind::Shell(inner) => *inner,
                other => panic!("expected the shell's refusal, got {other:?}"),
            },
            other => panic!("expected a refusal, got {other:?}"),
        }
    };

    // A wall too thick for the box: the clearance the offsets NEED is
    // `2t`, a bracket `[2(t−w), 2(t+w)]`, reported at its supremum.
    let (doc, node) = shelled_at(0.625);
    match refused(&doc, node) {
        ShellError::WallClearance { gap, needed, .. } => {
            assert_eq!(gap, cup::L, "the facing walls' gap is exact");
            assert_eq!(
                needed,
                2.0 * (0.625 + width),
                "the needed wall reports its supremum"
            );
        }
        other => panic!("expected the clearance gate, got {other:?}"),
    }
    // A wall below zero: the refused thickness is a bracket
    // `[t−w, t+w]`, reported at its infimum.
    let (doc, node) = shelled_at(-0.125);
    match refused(&doc, node) {
        ShellError::Thickness { thickness } => {
            assert_eq!(
                thickness,
                -0.125 - width,
                "the thickness reports its infimum"
            );
        }
        other => panic!("expected the thickness gate, got {other:?}"),
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
    let hole = LoopProgram::polygon([
        (0.375, 0.375),
        (0.625, 0.375),
        (0.625, 0.625),
        (0.375, 0.625),
    ])
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
    assert!(
        bad.is_empty(),
        "P7: the holed slab must hollow:\n{}",
        bad.join("\n")
    );
    let t = &ev.value(shell).unwrap().name_table;
    let hole_rims: Vec<_> = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::HoleRim { .. })))
        .map(|(n, _)| n.clone())
        .collect();
    let body = body_of(&ev, shell);
    assert_eq!(topo::validate(body), Ok(()));
    assert_eq!(topo::validate_closed(body), Ok(()));
    // Outer: bottom, four walls, four hole walls (9); the rim annulus
    // and the hole's rim annulus (2); cavity: floor, four walls, four
    // hole walls (9).
    assert_eq!(body.faces().count(), 20, "9 outer + 2 rims + 9 cavity");
    assert_eq!(hole_rims.len(), 1, "one hole, one hole rim");
    assert_eq!(
        hole_rims[0],
        shelled(
            shell,
            EntityKind::Face,
            RoleSeg::HoleRim {
                of: Box::new(cup::top(blank)),
                hole: 0,
            },
        ),
        "the hole rim is named for the designated face and its hole's pairing index"
    );
}
