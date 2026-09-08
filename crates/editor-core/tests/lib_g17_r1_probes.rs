//! Reviewer probes (R1) for LIB-G17. Not part of the unit.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use corpus::{body_of, cup, eval, failures, vessel};
use editor_core::{
    DocEdit, EntityKind, Node, NodeErrorKind, NodeResult, ProfileDoc, RecipeNodeId, RoleSeg,
    SlotId, StableName, apply, evaluate,
};
use geom_core::Tol;

fn blank_of(doc: &ProfileDoc) -> RecipeNodeId {
    doc.order()
        .iter()
        .copied()
        .find(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
        .expect("blank")
}

/// CLAIM: a `Rebind` onto an already-designated face shrinks the list
/// keeping the EARLIER position. Unexercised by the unit's suite.
#[test]
fn probe_rebind_onto_a_designated_face_shrinks_keeping_the_earlier() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let top = cup::top(blank);
    let bottom = cup::bottom(blank);
    let (doc, shell) = fixture::insert(
        d.doc.clone(),
        Node::shell(
            blank,
            fixture::len(cup::T),
            vec![top.clone(), bottom.clone()],
        ),
    );
    // Rebind the SECOND (bottom) onto the FIRST (top).
    let out = apply(
        &doc,
        &DocEdit::Rebind {
            from: bottom.clone(),
            to: top.clone(),
        },
        Tol::witness(),
    )
    .expect("rebind applies");
    let Some(Node::Shell { open, .. }) = out.doc.node(shell) else {
        panic!("shell gone")
    };
    assert_eq!(open, &vec![top.clone()], "shrank to the earlier position");

    // And the other direction: rebind the FIRST onto the SECOND.
    let out2 = apply(
        &doc,
        &DocEdit::Rebind {
            from: top.clone(),
            to: bottom.clone(),
        },
        Tol::witness(),
    )
    .expect("rebind applies");
    let Some(Node::Shell { open, .. }) = out2.doc.node(shell) else {
        panic!("shell gone")
    };
    // Both entries become `bottom`; first occurrence kept => position 0.
    assert_eq!(open, &vec![bottom.clone()], "collapsed at position 0");
}

/// DISPATCHER: is the rim identity really ONLY the first name's?
/// Swap the order and compare everything else observable.
#[test]
fn probe_order_swap_changes_only_the_rim_name() {
    let a = vessel::document();
    let b = vessel::document_with_mouth(|pot| {
        [
            vessel::band_pi(pot, vessel::SEG_MOUTH),
            vessel::band(pot, vessel::SEG_MOUTH),
        ]
    });
    let (sa, sb) = (a.result.unwrap(), b.result.unwrap());
    let (ea, eb) = (eval::<f64>(&a.doc), eval::<f64>(&b.doc));
    assert!(failures(&ea).is_empty());
    assert!(failures(&eb).is_empty());
    let (ba, bb) = (body_of(&ea, sa), body_of(&eb, sb));
    let ma = topo::mass_properties(ba, Tol::witness()).unwrap();
    let mb = topo::mass_properties(bb, Tol::witness()).unwrap();
    assert_eq!(ma.volume, mb.volume, "volume must not move");
    assert_eq!(ma.surface_area, mb.surface_area, "area must not move");
    assert_eq!(ba.faces().count(), bb.faces().count());
    assert_eq!(ba.edges().count(), bb.edges().count());
    assert_eq!(ba.vertices().count(), bb.vertices().count());
    // Every OTHER name: the two tables must agree except on the rim.
    let names_a: std::collections::BTreeSet<String> = ea
        .value(sa)
        .unwrap()
        .name_table
        .iter()
        .map(|(n, _)| format!("{n:?}"))
        .collect();
    let names_b: std::collections::BTreeSet<String> = eb
        .value(sb)
        .unwrap()
        .name_table
        .iter()
        .map(|(n, _)| format!("{n:?}"))
        .collect();
    let only_a: Vec<_> = names_a.difference(&names_b).collect();
    let only_b: Vec<_> = names_b.difference(&names_a).collect();
    println!("ONLY IN A ({}): {only_a:#?}", only_a.len());
    println!("ONLY IN B ({}): {only_b:#?}", only_b.len());
    assert_eq!(only_a.len(), 1, "exactly one name differs");
    assert_eq!(only_b.len(), 1, "exactly one name differs");
}

/// DISPATCHER: the fold takes `Bounds::lo()` at non-f64 lanes. What
/// number does a user read at the Interval lane?
#[cfg(feature = "interval")]
#[test]
fn probe_interval_lane_refusal_numbers() {
    // A wall thicker than half the box: the facing offsets collide.
    for t in [0.6_f64, 0.5, 0.45] {
        let d = cup::document();
        let blank = blank_of(&d.doc);
        let (doc, n) = fixture::insert(
            d.doc.clone(),
            Node::shell(blank, fixture::len(t), vec![cup::top(blank)]),
        );
        let f = refusal_text::<f64>(&doc, n);
        let i = refusal_text::<geom_core::Interval>(&doc, n);
        println!("t={t}\n  f64      = {f:?}\n  interval = {i:?}");
    }
}

#[cfg(feature = "interval")]
fn refusal_text<T: editor_core::eval::EvalScalar>(doc: &ProfileDoc, node: RecipeNodeId) -> String {
    let mut ev = evaluate::<T>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.remove(&node) {
        Some(NodeResult::Failed(e)) => format!("{:?} :: {}", tag(&e.kind), e.kind),
        other => format!("NOT REFUSED: {other:?}"),
    }
}

#[cfg(feature = "interval")]
fn tag(k: &NodeErrorKind) -> &'static str {
    match k {
        NodeErrorKind::Shell(_) => "Shell",
        NodeErrorKind::ShellLaneUnsupported { .. } => "LaneUnsupported",
        _ => "other",
    }
}

/// ATTACK the rebuild row: bump the wall to a thickness that changes
/// the topology class (a very thick wall).
#[test]
fn probe_a_thick_wall_bump() {
    for t in [0.4_f64, 0.5, 0.6] {
        let d = cup::document();
        let shell = d.result.unwrap();
        let bumped = apply(
            &d.doc,
            &DocEdit::SetParam {
                node: shell,
                slot: SlotId::ShellThickness,
                expr: fixture::len(t),
            },
            Tol::witness(),
        )
        .expect("edit applies")
        .doc;
        let ev = eval::<f64>(&bumped);
        match ev.nodes.get(&shell) {
            Some(NodeResult::Ok(_)) => {
                let b = body_of(&ev, shell);
                let m = topo::mass_properties(b, Tol::witness()).unwrap();
                let want = cup::closed_forms(cup::L, cup::H, t);
                println!(
                    "t={t}: OK faces={} shells={} V={} want={} (match={})",
                    b.faces().count(),
                    b.shells().count(),
                    m.volume,
                    want.volume,
                    m.volume == want.volume
                );
            }
            Some(NodeResult::Failed(e)) => println!("t={t}: REFUSED {}", e.kind),
            other => println!("t={t}: {other:?}"),
        }
    }
}

/// ATTACK the rebuild row: change WHICH face is designated.
#[test]
fn probe_designating_a_side_wall_instead_of_the_top() {
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let (doc, n) = fixture::insert(
        d.doc.clone(),
        Node::shell(
            blank,
            fixture::len(cup::T),
            vec![fixture::fname(blank, fixture::wall(0))],
        ),
    );
    let ev = eval::<f64>(&doc);
    match ev.nodes.get(&n) {
        Some(NodeResult::Ok(_)) => {
            let b = body_of(&ev, n);
            let table = &ev.value(n).unwrap().name_table;
            let rims: Vec<_> = table
                .iter()
                .filter(|(nm, _)| matches!(nm.path.first(), Some(RoleSeg::Rim(_))))
                .map(|(nm, _)| format!("{nm:?}"))
                .collect();
            println!("side-wall open: faces={} rims={rims:#?}", b.faces().count());
            assert_eq!(rims.len(), 1);
        }
        other => panic!("side wall designation refused: {other:?}"),
    }
}

/// COVARIANCE: move the target's upstream so the target's own names
/// move, and see whether the shell's names move WITH them.
#[test]
fn probe_covariance_when_the_targets_names_move() {
    // Rebind inside the cup: rename nothing, but re-point the shell's
    // designation from the End cap to the Start cap and check the rim
    // name follows.
    let d = cup::document();
    let blank = blank_of(&d.doc);
    let shell = d.result.unwrap();
    let out = apply(
        &d.doc,
        &DocEdit::Rebind {
            from: cup::top(blank),
            to: cup::bottom(blank),
        },
        Tol::witness(),
    )
    .expect("rebind")
    .doc;
    let ev = eval::<f64>(&out);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let table = &ev.value(shell).unwrap().name_table;
    let rim = StableName {
        kind: EntityKind::Face,
        node: shell,
        path: vec![RoleSeg::Rim(Box::new(cup::bottom(blank)))],
    };
    let stale = StableName {
        kind: EntityKind::Face,
        node: shell,
        path: vec![RoleSeg::Rim(Box::new(cup::top(blank)))],
    };
    assert!(table.lookup(&rim).is_some(), "rim followed the rebind");
    assert!(table.lookup(&stale).is_none(), "old rim name is gone");
}

/// The thickness slot is NOT fed by `feed_shell` (`flow_bearing`
/// answers false for `ShellThickness`, its flow row being empty), so
/// the key must move on the SLOT VALUE alone — otherwise a wall edit
/// would serve the old body out of the memo.
#[test]
fn probe_a_thickness_only_edit_moves_the_content_key_and_the_memo() {
    let d = cup::document();
    let shell = d.result.unwrap();
    let bumped = apply(
        &d.doc,
        &DocEdit::SetParam {
            node: shell,
            slot: SlotId::ShellThickness,
            expr: fixture::len(cup::T_BUMPED),
        },
        Tol::witness(),
    )
    .expect("edit applies")
    .doc;
    let a = eval::<f64>(&d.doc);
    let b = eval::<f64>(&bumped);
    assert_ne!(
        a.value(shell).unwrap().content_key,
        b.value(shell).unwrap().content_key,
        "a thickness-only edit must move the shell's key"
    );
    // And through the memo: the prior must not be served.
    let with_prior = evaluate::<f64>(
        &bumped,
        Some(&a),
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    let body = body_of(&with_prior, shell);
    let m = topo::mass_properties(body, Tol::witness()).unwrap();
    assert_eq!(
        m.volume,
        cup::closed_forms(cup::L, cup::H, cup::T_BUMPED).volume,
        "the memo served the pre-edit hollow"
    );
}
