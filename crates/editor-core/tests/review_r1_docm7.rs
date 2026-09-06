//! DOCM-7 review lane R1 — probes against the frozen head `723c0f9f`.
//! Each row states which claim it exercises; rows marked `MEASURE` print
//! what they find and assert only what the review needs pinned.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, CapEnd, DocEdit, EditError, EntityKind, EvalOptions,
    Evaluation, NameTable, Node, NodeErrorKind, NodeResult, ProfileDoc, ProfileVertexRef,
    RecipeNodeId, RoleSeg, StableName, ValuePayload, evaluate,
};
use fixture::{Recorder, ang, fname, insert, len, on_frame, scl, step, wall};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
}

fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<&NodeErrorKind> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => Some(&e.kind),
        _ => None,
    }
}

fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

fn contacts_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::ContactRecords {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { contacts, .. }) => (**contacts).clone(),
        other => panic!("expected a boolean body, got {other:?}"),
    }
}

fn volume(b: &topo::Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness()).expect("mass").volume
}

fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v
}
fn surfaces(b: &topo::Body<f64>) -> Vec<String> {
    sorted(b.surfaces().map(|(_, s)| format!("{s:?}")).collect())
}
fn curves(b: &topo::Body<f64>) -> Vec<String> {
    sorted(b.curves().map(|(_, c)| format!("{c:?}")).collect())
}
fn points(b: &topo::Body<f64>) -> Vec<String> {
    sorted(b.points().map(|(_, p)| format!("{p:?}")).collect())
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// An L-shaped prism: (0,0)-(2,0)-(2,1)-(1,1)-(1,2)-(0,2), shifted by `x0`.
fn lshape(doc: ProfileDoc, x0: f64) -> (ProfileDoc, RecipeNodeId) {
    let pts = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![pts.iter().map(|(x, y)| (x + x0, *y)).collect()],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    )
}

fn placed(doc: ProfileDoc, input: RecipeNodeId, dx: f64) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            translation: [len(dx), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    )
}

fn member_name(union: RecipeNodeId, member: RecipeNodeId, of: StableName) -> StableName {
    StableName {
        kind: of.kind,
        node: union,
        path: vec![RoleSeg::FromMember {
            member,
            of: Box::new(of),
        }],
    }
}

fn cap(end: CapEnd) -> RoleSeg {
    RoleSeg::Cap(end)
}

/// The L-shape's flush pairs under an x offset: walls 0, 2, 4 and both caps.
fn l_flush_segs() -> Vec<RoleSeg> {
    vec![wall(0), wall(2), wall(4), cap(CapEnd::Start), cap(CapEnd::End)]
}

fn box_flush_segs() -> Vec<RoleSeg> {
    vec![wall(0), wall(2), cap(CapEnd::Start), cap(CapEnd::End)]
}

fn pairs_in_member_space(
    union: RecipeNodeId,
    (m1, e1): (RecipeNodeId, RecipeNodeId),
    (m2, e2): (RecipeNodeId, RecipeNodeId),
    segs: Vec<RoleSeg>,
) -> Vec<(StableName, StableName)> {
    segs.into_iter()
        .map(|seg| {
            (
                member_name(union, m1, fname(e1, seg.clone())),
                member_name(union, m2, fname(e2, seg)),
            )
        })
        .collect()
}

/// The unit's two-pass authoring, verbatim in shape.
fn declared_union(
    doc: ProfileDoc,
    members: &[RecipeNodeId],
    pairs: impl Fn(RecipeNodeId) -> Vec<(StableName, StableName)>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, first) = insert(
        doc,
        Node::Union {
            members: members.to_vec(),
            declare: None,
        },
    );
    let (doc, decl) = insert(doc, Node::declare_rest(pairs(first)));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: members.to_vec(),
            declare: Some(decl),
        },
    );
    let mut doc = doc;
    // One Rebind per DISTINCT name: a name two pairs share is rewritten
    // at both sites by its first Rebind, and a second would find none.
    let mut moves: std::collections::BTreeMap<StableName, StableName> = Default::default();
    for ((fa, fb), (ta, tb)) in pairs(first).into_iter().zip(pairs(union)) {
        moves.insert(fa, ta);
        moves.insert(fb, tb);
    }
    for (from, to) in moves {
        doc = step(doc, DocEdit::Rebind { from, to }).0;
    }
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: first });
    (doc, union, decl)
}

/// Three blocks in a row, each flush with the next (a–c, c–d), declared
/// with MEMBER-SPACE names only. MEASURE: which list orders fuse.
#[test]
fn r1_c2_member_space_declarations_across_three_chained_members_by_order() {
    let doc = ProfileDoc::empty_derived("r1_c2_chain", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let pairs = |u: RecipeNodeId| {
        let mut v = pairs_in_member_space(u, (a, a), (c, c), box_flush_segs());
        v.extend(pairs_in_member_space(u, (c, c), (d, d), box_flush_segs()));
        v
    };
    let mut outcomes = Vec::new();
    for (label, order) in [
        ("[a, d, c]", vec![a, d, c]),
        ("[a, c, d]", vec![a, c, d]),
        ("[c, d, a]", vec![c, d, a]),
        ("[d, a, c]", vec![d, a, c]),
    ] {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs);
        let ev = run(&docx);
        let outcome = match failure(&ev, union) {
            None => format!(
                "Ok volume={} merged={} merged_rows={:?}",
                volume(&body_of(&ev, union)),
                merged_count(&ev, union),
                table(&ev, union)
                    .iter()
                    .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
                    .map(|(n, _)| format!("{n:?}").len())
                    .collect::<Vec<_>>()
            ),
            Some(e) => {
                let s = format!("{e:?}");
                format!("REFUSED {}", &s[..s.len().min(160)])
            }
        };
        eprintln!("MEASURE r1_c2_chain: order {label} -> {outcome}");
        outcomes.push((label, failure(&ev, union).is_none()));
    }
    eprintln!("MEASURE r1_c2_chain: fuses by order = {outcomes:?}");
    assert!(outcomes.iter().any(|(_, ok)| *ok), "no order fused at all");
}

fn member_of(n: &StableName) -> Option<RecipeNodeId> {
    match n.path.first() {
        Some(RoleSeg::FromMember { member, .. }) => Some(*member),
        _ => None,
    }
}

fn merged_count(ev: &Evaluation<f64>, id: RecipeNodeId) -> usize {
    table(ev, id)
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .count()
}

// ---------------------------------------------------------------------
// C1 — a prototype and a placement the implementer did not choose.
// ---------------------------------------------------------------------

/// L-shaped prototype, two placements 0.5 apart in x: five flush pairs.
#[test]
fn r1_c1_l_shaped_placements_fuse_when_declared_and_refuse_without() {
    let doc = ProfileDoc::empty_derived("r1_c1_l", Tol::witness());
    let (doc, proto) = lshape(doc, 0.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let (bare, plain) = insert(
        doc.clone(),
        Node::Union {
            members: vec![m1, m2],
            declare: None,
        },
    );
    let ev = run(&bare);
    let Some(NodeErrorKind::UndeclaredContact { finding, .. }) = failure(&ev, plain) else {
        panic!("expected UndeclaredContact, got {:?}", failure(&ev, plain));
    };
    let named = [member_of(&finding.pair.0), member_of(&finding.pair.1)];
    assert!(named.contains(&Some(m1)) && named.contains(&Some(m2)), "{named:?}");

    let (doc, union, _) = declared_union(doc, &[m1, m2], |u| {
        pairs_in_member_space(u, (m1, proto), (m2, proto), l_flush_segs())
    });
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let v = volume(&body_of(&ev, union));
    assert!((v - 4.0).abs() < 1e-9, "fused L volume {v}");
    let merged: Vec<String> = table(&ev, union)
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .map(|(n, _)| format!("{n}"))
        .collect();
    eprintln!("MEASURE r1_c1: {} of 5 declared pairs minted Merged rows:\n  {}", merged.len(), merged.join("\n  "));
    assert!(!merged.is_empty());
}

/// The `bit_eq` half on two DISTINCT L prisms, where the pair spelling exists.
/// MEASURE: how far the two bodies are from bit-identical.
#[test]
fn r1_c1_distinct_l_prisms_union_vs_pair_boolean() {
    let doc = ProfileDoc::empty_derived("r1_c1_l_pair", Tol::witness());
    let (doc, a) = lshape(doc, 0.0);
    let (doc, b) = lshape(doc, 0.5);
    let op_pairs: Vec<(StableName, StableName)> = l_flush_segs()
        .into_iter()
        .map(|s| (fname(a, s.clone()), fname(b, s)))
        .collect();
    let (doc, decl) = insert(doc, Node::declare_rest(op_pairs));
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let (doc, union, _) = declared_union(doc, &[a, b], |u| {
        pairs_in_member_space(u, (a, a), (b, b), l_flush_segs())
    });
    let ev = run(&doc);
    assert!(failure(&ev, pair).is_none(), "{:?}", failure(&ev, pair));
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let (f, p) = (body_of(&ev, union), body_of(&ev, pair));
    assert_eq!(f.faces().count(), p.faces().count());
    assert_eq!(f.edges().count(), p.edges().count());
    assert_eq!(f.vertices().count(), p.vertices().count());
    assert_eq!(surfaces(&f), surfaces(&p));
    assert_eq!(curves(&f), curves(&p));
    assert_eq!(points(&f), points(&p));
    let whole_eq = format!("{f:?}") == format!("{p:?}");
    let (fd, pd) = (format!("{f:?}"), format!("{p:?}"));
    let first_diff = fd.bytes().zip(pd.bytes()).position(|(x, y)| x != y);
    eprintln!(
        "MEASURE r1_c1: whole-body Debug equal = {whole_eq}; first diff at {first_diff:?}; \
         context: {:?}",
        first_diff.map(|i| (&fd[i.saturating_sub(60)..(i + 60).min(fd.len())], &pd[i.saturating_sub(60)..(i + 60).min(pd.len())]))
    );
    assert!((volume(&f) - 4.0).abs() < 1e-9);
}

// ---------------------------------------------------------------------
// C6 — whose asymmetry is it?
// ---------------------------------------------------------------------

/// MEASURE: union[a,b] vs pair(a,b), union[b,a] vs pair(b,a), pair(a,b) vs pair(b,a).
#[test]
fn r1_c6_the_reorder_asymmetry_is_the_pair_verbs() {
    let doc = ProfileDoc::empty_derived("r1_c6", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl) = fixture::declare_x_offset_flush(doc, a, b);
    let (doc, pab) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let (doc, pba) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: b,
            b: a,
            declare: Some(decl),
        },
    );
    let (doc, uab, _) = declared_union(doc, &[a, b], |u| {
        pairs_in_member_space(u, (a, a), (b, b), box_flush_segs())
    });
    let (doc, uba, _) = declared_union(doc, &[b, a], |u| {
        pairs_in_member_space(u, (a, a), (b, b), box_flush_segs())
    });
    let ev = run(&doc);
    for id in [pab, pba, uab, uba] {
        assert!(failure(&ev, id).is_none(), "{id:?}: {:?}", failure(&ev, id));
    }
    let (bpab, bpba, buab, buba) = (
        body_of(&ev, pab),
        body_of(&ev, pba),
        body_of(&ev, uab),
        body_of(&ev, uba),
    );
    let same = |x: &topo::Body<f64>, y: &topo::Body<f64>| {
        surfaces(x) == surfaces(y) && curves(x) == curves(y) && points(x) == points(y)
    };
    eprintln!(
        "MEASURE r1_c6: union[a,b]==pair(a,b): {}; union[b,a]==pair(b,a): {}; \
         pair(a,b)==pair(b,a): {}; union[a,b]==union[b,a]: {}",
        same(&buab, &bpab),
        same(&buba, &bpba),
        same(&bpab, &bpba),
        same(&buab, &buba)
    );
    let diff_surfaces = |x: &topo::Body<f64>, y: &topo::Body<f64>| {
        let (sx, sy) = (surfaces(x), surfaces(y));
        let only_x: Vec<_> = sx.iter().filter(|s| !sy.contains(s)).cloned().collect();
        let only_y: Vec<_> = sy.iter().filter(|s| !sx.contains(s)).cloned().collect();
        (only_x, only_y)
    };
    eprintln!(
        "MEASURE r1_c6 pair(a,b) vs pair(b,a) surfaces: {:?}",
        diff_surfaces(&bpab, &bpba)
    );
    // Names: which member's rims carry Fragment rows under each order.
    let frag_members = |id: RecipeNodeId| {
        let mut v: Vec<String> = table(&ev, id)
            .iter()
            .filter(|(n, _)| n.path.iter().any(|s| matches!(s, RoleSeg::Fragment(_))))
            .map(|(n, _)| format!("{:?}", member_of(n)))
            .collect();
        v.sort();
        v.dedup();
        v
    };
    eprintln!(
        "MEASURE r1_c6 fragment rows under union[a,b] belong to {:?}; under union[b,a] to {:?} (a={a:?}, b={b:?})",
        frag_members(uab),
        frag_members(uba)
    );
    assert!(same(&buab, &bpab), "union[a,b] is not pair(a,b)");
    assert!(same(&buba, &bpba), "union[b,a] is not pair(b,a)");
}

// ---------------------------------------------------------------------
// C2 — routing: the two-fold-rows arm, the four-member document,
// SetMembers re-deriving the routing.
// ---------------------------------------------------------------------

/// Deviation 8's arm is reachable: two Merged rows paired together
/// route to bucket 1 and resolve BOTH in operand A there.
#[test]
fn r1_c2_two_fold_rows_route_to_the_first_step_that_has_both() {
    let doc = ProfileDoc::empty_derived("r1_c2_two_rows", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let merged = |u: RecipeNodeId, seg: RoleSeg| {
        let mut set = vec![
            member_name(u, m1, fname(proto, seg.clone())),
            member_name(u, m2, fname(proto, seg)),
        ];
        set.sort();
        StableName {
            kind: EntityKind::Face,
            node: u,
            path: vec![RoleSeg::Merged(set)],
        }
    };
    let pairs = |u: RecipeNodeId| {
        let mut v = pairs_in_member_space(u, (m1, proto), (m2, proto), box_flush_segs());
        v.push((merged(u, wall(0)), merged(u, wall(2))));
        v
    };
    // Three members: a step is left after the merge; both rows resolve
    // in the accumulation and the vocabulary refuses a same-operand
    // face pair — which is the proof the step held both rows.
    let (doc3, union3, _) = declared_union(doc.clone(), &[m1, m2, far], pairs);
    let ev = run(&doc3);
    match failure(&ev, union3) {
        Some(NodeErrorKind::DeclareUnsupportedPair { kinds, cross_operand }) => {
            assert_eq!(*kinds, (EntityKind::Face, EntityKind::Face));
            assert!(!cross_operand, "both rows were found in ONE operand");
        }
        other => panic!("expected DeclareUnsupportedPair (same operand), got {other:?}"),
    }
    // Two members: no step is left, refused as unroutable.
    let (doc2, union2, _) = declared_union(doc, &[m1, m2], pairs);
    let ev = run(&doc2);
    assert!(
        matches!(failure(&ev, union2), Some(NodeErrorKind::UnionDeclareStep { .. })),
        "{:?}",
        failure(&ev, union2)
    );
}

/// Four members [a, far, c, d]: a↔c flush at bucket 1 mints Merged rows;
/// d meets those rows at bucket 2. Then the lower-bound question: a
/// fragment row of `a` minted at bucket 1 (mentions no later member)
/// paired with c routes to bucket 1, which does not hold it yet.
#[test]
fn r1_c2_four_member_document_routes_accumulation_rows() {
    let doc = ProfileDoc::empty_derived("r1_c2_four", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let merged_ac = |u: RecipeNodeId, seg: RoleSeg| {
        let mut set = vec![
            member_name(u, a, fname(a, seg.clone())),
            member_name(u, c, fname(c, seg)),
        ];
        set.sort();
        StableName {
            kind: EntityKind::Face,
            node: u,
            path: vec![RoleSeg::Merged(set)],
        }
    };
    let base_pairs = |u: RecipeNodeId| {
        let mut v = pairs_in_member_space(u, (a, a), (c, c), box_flush_segs());
        for seg in box_flush_segs() {
            v.push((merged_ac(u, seg.clone()), member_name(u, d, fname(d, seg))));
        }
        v
    };
    let (doc4, union4, _) = declared_union(doc.clone(), &[a, far, c, d], base_pairs);
    let ev = run(&doc4);
    assert!(failure(&ev, union4).is_none(), "{:?}", failure(&ev, union4));
    let v = volume(&body_of(&ev, union4));
    assert!((v - 3.0).abs() < 1e-9, "volume {v}");
    let rows: Vec<String> = table(&ev, union4)
        .iter()
        .filter(|(n, _)| !matches!(n.path.as_slice(), [RoleSeg::FromMember { .. }]))
        .map(|(n, _)| format!("{n:?}"))
        .collect();
    eprintln!("MEASURE r1_c2_four: {} non-member rows:\n  {}", rows.len(), rows.join("\n  "));

    // A Merged row of (a, c) paired with c itself: c joined at the step
    // that minted the row — unroutable, typed.
    let (docx, unionx, _) = declared_union(doc.clone(), &[a, far, c, d], |u| {
        let mut v = base_pairs(u);
        v.push((merged_ac(u, wall(0)), member_name(u, c, fname(c, wall(2)))));
        v
    });
    let ev = run(&docx);
    assert!(
        matches!(failure(&ev, unionx), Some(NodeErrorKind::UnionDeclareStep { .. })),
        "{:?}",
        failure(&ev, unionx)
    );

    // The lower bound: a fragment row of `a` that bucket 1 minted. Read
    // it off the three-member fold [a, far, c], whose final table is
    // exactly bucket 2's accumulation view in the four-member fold.
    let (doc3, union3, _) = declared_union(doc.clone(), &[a, far, c], |u| {
        pairs_in_member_space(u, (a, a), (c, c), box_flush_segs())
    });
    let ev3 = run(&doc3);
    assert!(failure(&ev3, union3).is_none(), "{:?}", failure(&ev3, union3));
    let frag_of_a: Vec<StableName> = table(&ev3, union3)
        .iter()
        .filter(|(n, _)| {
            member_of(n) == Some(a)
                && n.kind == EntityKind::Edge
                && n.path.iter().any(|s| matches!(s, RoleSeg::Fragment(_)))
        })
        .map(|(n, _)| n.clone())
        .collect();
    eprintln!("MEASURE r1_c2_four: fragment rows of a after a∪c: {} first={:?}", frag_of_a.len(), frag_of_a.first());
    let Some(frag) = frag_of_a.first().cloned() else {
        eprintln!("MEASURE r1_c2_four: no fragment row of a — lower-bound probe not reachable this way");
        return;
    };
    // Re-spell the row in the four-member union's space (same path, the
    // union's id) — the closure gets the id.
    let respell = |u: RecipeNodeId| StableName {
        kind: frag.kind,
        node: u,
        path: frag.path.clone(),
    };
    // Paired with d: routed to bucket 2, which holds it; the vocabulary
    // then refuses an Edge–Face pair — proof the row was FOUND there.
    let (docd, uniond, _) = declared_union(doc.clone(), &[a, far, c, d], |u| {
        let mut v = base_pairs(u);
        v.push((respell(u), member_name(u, d, fname(d, wall(0)))));
        v
    });
    let ev = run(&docd);
    eprintln!("MEASURE r1_c2_four: (frag of a @bucket1, d.face) -> {:?}", failure(&ev, uniond));
    assert!(
        matches!(failure(&ev, uniond), Some(NodeErrorKind::DeclareUnsupportedPair { cross_operand: true, .. })),
        "{:?}",
        failure(&ev, uniond)
    );
    // Paired with c: routed to bucket 1 (arrival = max(0, 1)), which has
    // not minted it yet — refused as a VANISHED name, not as unroutable.
    let (docc, unionc, _) = declared_union(doc, &[a, far, c, d], |u| {
        let mut v = base_pairs(u);
        v.push((respell(u), member_name(u, c, fname(c, wall(0)))));
        v
    });
    let ev = run(&docc);
    eprintln!("MEASURE r1_c2_four: (frag of a @bucket1, c.face) -> {:?}", failure(&ev, unionc));
    assert!(
        matches!(failure(&ev, unionc), Some(NodeErrorKind::DeclareResolve { .. })),
        "{:?}",
        failure(&ev, unionc)
    );
}

/// The routing is re-derived after `SetMembers`: a reorder and an
/// insertion both keep the same declaration fusing.
#[test]
fn r1_c2_set_members_re_derives_the_routing() {
    let doc = ProfileDoc::empty_derived("r1_c2_setm", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, b], |u| {
        pairs_in_member_space(u, (a, a), (b, b), box_flush_segs())
    });
    let (re, _) = step(
        doc.clone(),
        DocEdit::SetMembers {
            node: union,
            members: vec![b, a],
        },
    );
    let ev = run(&re);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(merged_count(&ev, union), 4);
    let (grown, _) = step(
        doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a, far, b],
        },
    );
    let ev = run(&grown);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(merged_count(&ev, union), 4);
    assert!((volume(&body_of(&ev, union)) - 2.5).abs() < 1e-9);
}

// ---------------------------------------------------------------------
// C2/C4 — a same-member pair is a carried record at its member's step.
// ---------------------------------------------------------------------

/// A V–F pair inside ONE member: fed as that member's carried record at
/// its own step, and visible in the result's contacts when that step is
/// the last. MEASURE: whether a member-0 carried record survives to a
/// later step's result.
#[test]
fn r1_c4_same_member_pair_is_a_carried_record_at_its_step() {
    let doc = ProfileDoc::empty_derived("r1_c4_carried", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, far2) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let vname = StableName {
        kind: EntityKind::Vertex,
        node: a,
        path: vec![RoleSeg::CapVertex(
            CapEnd::End,
            ProfileVertexRef {
                loop_index: 0,
                vertex: 0,
            },
        )],
    };
    let fname_a = fname(a, cap(CapEnd::Start));
    // The pair boolean's reading of the same carried claim, for reference.
    let (doc, pdecl) = insert(doc, Node::declare_rest(vec![(vname.clone(), fname_a.clone())]));
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: far,
            b: a,
            declare: Some(pdecl),
        },
    );
    // Member a as operand B of the last step.
    let (doc, u_last, _) = declared_union(doc, &[far, a], |u| {
        vec![(member_name(u, a, vname.clone()), member_name(u, a, fname_a.clone()))]
    });
    // Member a as operand A of step 1 in a two-step fold.
    let (doc, u_first, _) = declared_union(doc, &[a, far, far2], |u| {
        vec![(member_name(u, a, vname.clone()), member_name(u, a, fname_a.clone()))]
    });
    let ev = run(&doc);
    for id in [pair, u_last, u_first] {
        assert!(failure(&ev, id).is_none(), "{id:?}: {:?}", failure(&ev, id));
    }
    let (cp, cl, cf) = (contacts_of(&ev, pair), contacts_of(&ev, u_last), contacts_of(&ev, u_first));
    eprintln!(
        "MEASURE r1_c4: pair b_on_a={} a_on_b={}; union[far,a] b_on_a={} a_on_b={}; union[a,far,far2] b_on_a={} a_on_b={}",
        cp.b_on_a.len(), cp.a_on_b.len(), cl.b_on_a.len(), cl.a_on_b.len(), cf.b_on_a.len(), cf.a_on_b.len()
    );
    assert_eq!(cp.b_on_a.len(), 1, "the pair boolean carries the claim");
    assert_eq!(cl.b_on_a.len(), 1, "the union fed it as operand B's carried record at the last step");
}

// ---------------------------------------------------------------------
// The two-pass authoring: the edit log replays to the direct document.
// ---------------------------------------------------------------------

#[test]
fn r1_edit_log_replay_of_the_two_pass_construction_is_bit_identical() {
    let tol = Tol::witness();
    let mut rec = Recorder::new();
    let pa = rec.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let a = rec.insert(Node::Extrude {
        profile: pa,
        distance: len(1.0),
    });
    let pb = rec.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.5, 0.0), (1.5, 0.0), (1.5, 1.0), (0.5, 1.0)]],
    );
    let b = rec.insert(Node::Extrude {
        profile: pb,
        distance: len(1.0),
    });
    let pairs = |u: RecipeNodeId| pairs_in_member_space(u, (a, a), (b, b), box_flush_segs());
    let first = rec.insert(Node::Union {
        members: vec![a, b],
        declare: None,
    });
    let decl = rec.insert(Node::declare_rest(pairs(first)));
    let union = rec.insert(Node::Union {
        members: vec![a, b],
        declare: Some(decl),
    });
    for ((fa, fb), (ta, tb)) in pairs(first).into_iter().zip(pairs(union)) {
        for (from, to) in [(fa, ta), (fb, tb)] {
            rec.push(DocEdit::Rebind { from, to });
        }
    }
    rec.push(DocEdit::DeleteNode { id: first });
    let direct = rec.doc.clone();
    // Log-only save: the snapshot is empty, everything replays.
    let empty = ProfileDoc::empty_derived("mod", tol);
    let text = editor_core::persist::save(&empty, &rec.edits, tol).expect("saves");
    let loaded = editor_core::persist::load(&text, tol).expect("loads");
    assert!(loaded.doc.bit_eq(&direct), "the log replay is not the direct document");
    // Snapshot-only save of the direct document.
    let text2 = editor_core::persist::save(&direct, &[], tol).expect("saves");
    let loaded2 = editor_core::persist::load(&text2, tol).expect("loads");
    assert!(loaded2.doc.bit_eq(&direct));
    // And the three evaluate to one union: same content key, same table.
    let (e0, e1, e2) = (run(&direct), run(&loaded.doc), run(&loaded2.doc));
    for e in [&e0, &e1, &e2] {
        assert!(failure(e, union).is_none(), "{:?}", failure(e, union));
    }
    let key = |e: &Evaluation<f64>| e.value(union).unwrap().content_key;
    assert_eq!(key(&e0), key(&e1));
    assert_eq!(key(&e0), key(&e2));
    assert_eq!(table(&e0, union), table(&e1, union));
    assert_eq!(table(&e0, union), table(&e2, union));
    // What the declaration ended up naming: the union, not the deleted first.
    let Some(Node::Declare { pairs: p }) = direct.node(decl) else {
        panic!("declare")
    };
    assert!(p.iter().all(|((x, y), _)| x.node == union && y.node == union));
}

// ---------------------------------------------------------------------
// The doors around a declared union.
// ---------------------------------------------------------------------

/// MEASURE: deleting the union (leaving the Declare that names it), and
/// deleting the Declare (which the union consumes).
#[test]
fn r1_delete_doors_around_a_declared_union() {
    let doc = ProfileDoc::empty_derived("r1_delete", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], |u| {
        pairs_in_member_space(u, (a, a), (b, b), box_flush_segs())
    });
    let del_union = doc.apply(&DocEdit::DeleteNode { id: union }, Tol::witness());
    let del_decl = doc.apply(&DocEdit::DeleteNode { id: decl }, Tol::witness());
    eprintln!(
        "MEASURE r1_delete: DeleteNode(union) -> {:?}; DeleteNode(decl) -> {:?}",
        del_union.as_ref().map(|_| "Ok").map_err(|e| format!("{e}")),
        del_decl.as_ref().map(|_| "Ok").map_err(|e| format!("{e}"))
    );
    assert!(matches!(del_decl, Err(EditError::DeleteWouldDangle { .. })));
    // After the union is gone the Declare stays, naming a dead node; the
    // document still saves and loads.
    if let Ok(applied) = del_union {
        let text = editor_core::persist::save(&applied.doc, &[], Tol::witness());
        eprintln!("MEASURE r1_delete: save after deleting the union -> {}", text.is_ok());
        let ev = run(&applied.doc);
        eprintln!("MEASURE r1_delete: dangling Declare evaluates to {:?}", ev.nodes.get(&decl).map(|r| matches!(r, NodeResult::Failed(_))));
    }
}

