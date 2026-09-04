//! DOCM-3 review lane R1 — executed probes against the unit's claims.
//! Not part of the unit; committed on `docm/3-review-r1` only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, DocEdit, DocumentId, EditError, EvalOptions, Evaluation,
    Node, ProfileDoc, RecipeNodeId, RoleSeg, StableName, ValuePayload, all_edges, all_faces,
    all_vertices, evaluate, split,
};
use fixture::{insert, len, on_frame};
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

/// A box on a frame at height `z0`, footprint `[x0,x1]×[y0,y1]`, height `h`.
fn boxed(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    h: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(h),
        },
    )
}

/// Three boxes that actually intersect: A∩B ≠ ∅, B∩C ≠ ∅, A∩C = ∅, no
/// coplanar faces anywhere.
fn intersecting_boxes() -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty_derived("docm3_r1", Tol::witness());
    let (doc, a) = boxed(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = boxed(doc, (0.5, 1.5), (0.2, 0.8), 0.2, 0.5);
    let (doc, c) = boxed(doc, (1.2, 2.2), (0.35, 0.65), 0.35, 0.2);
    (doc, [a, b, c])
}

fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

fn all_names(ev: &Evaluation<f64>, id: RecipeNodeId) -> BTreeSet<StableName> {
    let mut s: BTreeSet<StableName> = BTreeSet::new();
    s.extend(all_faces(ev, id));
    s.extend(all_edges(ev, id));
    s.extend(all_vertices(ev, id));
    s
}

/// Deep check: no `FromA`/`FromB` anywhere in a name (recursively).
fn has_descent(name: &StableName) -> bool {
    name.path.iter().any(|seg| match seg {
        RoleSeg::FromA(_) | RoleSeg::FromB(_) => true,
        RoleSeg::FromMember { of, .. } => has_descent(of),
        RoleSeg::Seam { a, b } => has_descent(a) || has_descent(b),
        RoleSeg::Merged(cs) => cs.iter().any(has_descent),
        RoleSeg::Fragment(editor_core::Qualifier::SideOf(v)) => {
            v.iter().any(|(n, _)| has_descent(n))
        }
        _ => false,
    })
}

fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<String> {
    match ev.nodes.get(&id) {
        Some(editor_core::NodeResult::Failed(e)) => Some(format!("{:?}", e.kind)),
        _ => None,
    }
}

// ---------------------------------------------------------------------
// C1 with members that INTERSECT: seams, fragments — the whole table.
// ---------------------------------------------------------------------

/// Every order of three intersecting boxes yields the SAME name table
/// (faces, edges AND vertices), and no name carries a fold descent.
#[test]
fn r1_intersecting_members_whole_table_is_position_free() {
    let orders: [[usize; 3]; 4] = [[0, 1, 2], [2, 1, 0], [1, 0, 2], [0, 2, 1]];
    let mut tables: Vec<BTreeSet<StableName>> = Vec::new();
    let mut counts = Vec::new();
    for order in orders {
        let (doc, bx) = intersecting_boxes();
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: order.map(|i| bx[i]).to_vec(),
            },
        );
        let ev = run(&doc);
        assert!(
            failure(&ev, u).is_none(),
            "order {order:?}: {:?}",
            failure(&ev, u)
        );
        let names = all_names(&ev, u);
        for n in &names {
            assert!(
                !has_descent(n),
                "order {order:?}: fold descent survived: {n:?}"
            );
            assert_eq!(n.node, u);
        }
        // Bidirectional agreement between the table and the body.
        let body = body_of(&ev, u);
        let ents = body.faces().count() + body.edges().count() + body.vertices().count();
        let table = &ev.value(u).unwrap().name_table;
        for n in &names {
            assert!(table.lookup(n).is_some(), "{n:?} not in table");
        }
        counts.push((ents, names.len(), body.faces().count()));
        tables.push(names);
    }
    println!("r1 counts (entities, names, faces) per order: {counts:?}");
    for (i, t) in tables.iter().enumerate() {
        assert_eq!(
            t,
            &tables[0],
            "order {:?} names differ from order {:?}: only-in-this {:?} / only-in-first {:?}",
            orders[i],
            orders[0],
            t.difference(&tables[0]).collect::<Vec<_>>(),
            tables[0].difference(t).collect::<Vec<_>>()
        );
    }
}

// ---------------------------------------------------------------------
// C2 with intersecting members: fold vs chain, descriptions AND sources.
// ---------------------------------------------------------------------

#[test]
fn r1_fold_equals_chain_with_intersecting_members() {
    let (doc, bx) = intersecting_boxes();
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: bx.to_vec(),
        },
    );
    let (doc, ab) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: bx[0],
            b: bx[1],
            declare: None,
        },
    );
    let (doc, abc) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: ab,
            b: bx[2],
            declare: None,
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, u).is_none(), "{:?}", failure(&ev, u));
    assert!(failure(&ev, abc).is_none(), "{:?}", failure(&ev, abc));
    let (f, c) = (body_of(&ev, u), body_of(&ev, abc));
    assert_eq!(f.faces().count(), c.faces().count());
    assert_eq!(f.edges().count(), c.edges().count());
    assert_eq!(f.vertices().count(), c.vertices().count());
    let surfs = |b: &topo::Body<f64>| {
        let mut v: Vec<String> = b.surfaces().map(|(_, s)| format!("{s:?}")).collect();
        v.sort();
        v
    };
    let curves = |b: &topo::Body<f64>| {
        let mut v: Vec<String> = b.curves().map(|(_, s)| format!("{s:?}")).collect();
        v.sort();
        v
    };
    let points = |b: &topo::Body<f64>| {
        let mut v: Vec<String> = b.points().map(|(_, s)| format!("{s:?}")).collect();
        v.sort();
        v
    };
    assert_eq!(surfs(&f), surfs(&c), "surfaces differ");
    assert_eq!(curves(&f), curves(&c), "curves differ");
    assert_eq!(points(&f), points(&c), "points differ");
    // Sources: every surface of the fold carries a source (nothing left
    // unstamped), and the multiset of surface-source SHAPES agrees with
    // the chain's up to the minting node id.
    let srcs = |b: &topo::Body<f64>| {
        let mut v: Vec<String> = b
            .surfaces()
            .map(|(k, _)| format!("{:?}", b.surface_source(k).map(|s| format!("{s:?}"))))
            .collect();
        v.sort();
        v
    };
    let fs = srcs(&f);
    assert!(
        fs.iter().all(|s| s != "None"),
        "an unstamped surface survived the fold: {fs:?}"
    );
    let curve_unstamped = f
        .curves()
        .filter(|(k, _)| f.curve_source(*k).is_none())
        .count();
    let point_unstamped = f
        .points()
        .filter(|(k, _)| f.point_source(*k).is_none())
        .count();
    assert_eq!(
        (curve_unstamped, point_unstamped),
        (0, 0),
        "unstamped curves/points after the fold"
    );
    println!("r1 fold surface sources: {fs:?}");
    println!("r1 chain surface sources: {:?}", srcs(&c));
}

// ---------------------------------------------------------------------
// C2: empty members and the empty-intermediate path.
// ---------------------------------------------------------------------

#[test]
fn r1_empty_member_refuses_empty_operand_at_every_position() {
    let doc = ProfileDoc::empty_derived("docm3_r1", Tol::witness());
    let (doc, a) = boxed(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = boxed(doc, (5.0, 6.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, other) = boxed(doc, (10.0, 11.0), (0.0, 1.0), 0.0, 1.0);
    // An empty body: the intersection of two disjoint boxes.
    let (doc, empty) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Intersect,
            a,
            b: far,
            declare: None,
        },
    );
    let (doc, u_first) = insert(
        doc,
        Node::Union {
            members: vec![empty, a, other],
        },
    );
    let (doc, u_mid) = insert(
        doc,
        Node::Union {
            members: vec![a, empty, other],
        },
    );
    let (doc, u_last) = insert(
        doc,
        Node::Union {
            members: vec![a, other, empty],
        },
    );
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b: empty,
            declare: None,
        },
    );
    let ev = run(&doc);
    assert!(matches!(
        &ev.value(empty).unwrap().payload,
        ValuePayload::Boolean(BooleanValue::Empty)
    ));
    for (what, id) in [
        ("first", u_first),
        ("middle", u_mid),
        ("last", u_last),
        ("pair", pair),
    ] {
        let f = failure(&ev, id).unwrap_or_else(|| panic!("{what}: an empty member must refuse"));
        assert!(
            f.contains("EmptyOperand") && f.contains(&format!("{}", empty.0)),
            "{what}: {f}"
        );
    }
}

// ---------------------------------------------------------------------
// The refusal menu from a LATER fold step: what names does it carry?
// ---------------------------------------------------------------------

/// Undeclared coincidence at step 2 of a three-member fold: the
/// refusal's names should be names the union would mint — not
/// uncollapsed fold rows.
#[test]
fn r1_refusal_from_a_later_fold_step_names_union_space_names() {
    let (doc, bx) = intersecting_boxes();
    // D shares A's x = 1 plane over a patch that B does not cover.
    let (doc, d) = boxed(doc, (1.0, 2.0), (0.0, 0.15), 0.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![bx[0], bx[1], d],
        },
    );
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: bx[0],
            b: d,
            declare: None,
        },
    );
    let ev = run(&doc);
    let pf = failure(&ev, pair).expect("the pair refuses the undeclared contact");
    println!("r1 pair refusal: {pf}");
    let uf = failure(&ev, u).expect("the fold refuses the undeclared contact at step 2");
    println!("r1 fold refusal: {uf}");
    assert!(uf.contains("UndeclaredContact"), "{uf}");
    assert!(
        !uf.contains("FromA(") && !uf.contains("FromB("),
        "the fold's refusal names an uncollapsed fold row: {uf}"
    );
}

// ---------------------------------------------------------------------
// A1 for pips the implementer did not choose.
// ---------------------------------------------------------------------

#[test]
fn r1_removing_other_pips_leaves_both_die_fillets_resolving() {
    let tol = Tol::witness();
    let die = crate::corpus::die_composed_tour::document();
    let doc = die.doc;
    let union = doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Union { .. })))
        .unwrap();
    let Some(Node::Union { members }) = doc.node(union) else {
        panic!()
    };
    let members = members.clone();
    let blends: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|id| matches!(doc.node(*id), Some(Node::Fillet { .. })))
        .collect();
    let before = run(&doc);
    let (rim_target, rim_radius, rims) = match doc.node(blends[1]) {
        Some(Node::Fillet {
            target,
            radius,
            selection,
        }) => (*target, radius.clone(), selection.clone()),
        other => panic!("{other:?}"),
    };
    // Geometry check of the "two rim arcs per pip" arithmetic: count
    // the rim names whose derivation touches each member.
    for m in &members {
        let n = rims
            .iter()
            .filter(|r| editor_core::derivation_nodes(r).contains(m))
            .count();
        assert_eq!(n, 2, "member {m:?} owns {n} rim arcs");
    }
    for k in [1usize, 7, 13, 19] {
        let kept: Vec<RecipeNodeId> = members
            .iter()
            .copied()
            .filter(|m| *m != members[k])
            .collect();
        let edited = doc
            .apply(
                &DocEdit::SetMembers {
                    node: union,
                    members: kept,
                },
                tol,
            )
            .unwrap()
            .doc;
        let edited = edited
            .apply(&DocEdit::DeleteNode { id: members[k] }, tol)
            .unwrap()
            .doc;
        // FIRST: as the spec's A1 literally reads — SetMembers +
        // DeleteNode, re-evaluate with prior, no re-authoring.
        let after = evaluate::<f64>(
            &edited,
            Some(&before),
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        println!(
            "r1 pip {k}: box-edge fillet {:?}; rim fillet (frozen 42) {:?}",
            failure(&after, blends[0]),
            failure(&after, blends[1]).map(|s| s.chars().take(160).collect::<String>())
        );
        assert!(
            failure(&after, blends[0]).is_none(),
            "pip {k}: box-edge fillet broke"
        );
        // THEN the implementer's re-authoring.
        let doomed: Vec<StableName> = rims
            .iter()
            .filter(|n| editor_core::derivation_nodes(n).contains(&members[k]))
            .cloned()
            .collect();
        assert_eq!(doomed.len(), 2);
        let kept_rims: Vec<StableName> = rims
            .iter()
            .filter(|n| !doomed.contains(n))
            .cloned()
            .collect();
        let edited = edited
            .apply(&DocEdit::DeleteNode { id: blends[1] }, tol)
            .unwrap()
            .doc;
        let (edited, rim) = insert(
            edited,
            Node::fillet(rim_target, rim_radius.clone(), kept_rims),
        );
        let after2 = evaluate::<f64>(
            &edited,
            Some(&after),
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        assert!(failure(&after2, blends[0]).is_none(), "pip {k}: box fillet");
        assert!(
            failure(&after2, rim).is_none(),
            "pip {k}: rim fillet {:?}",
            failure(&after2, rim)
        );
        assert!(after2.value(rim).is_some());
    }
}

// ---------------------------------------------------------------------
// C4: SetMembers on a Loft, and the insert door's new refusal on Loft.
// ---------------------------------------------------------------------

fn section(doc: ProfileDoc, z: f64, s: f64) -> (ProfileDoc, RecipeNodeId) {
    on_frame(
        doc,
        [0.0, 0.0, z],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (2.0 * s, 0.0),
            (2.0 * s, 1.0 * s),
            (0.0, 1.0 * s),
        ]],
    )
}

#[test]
fn r1_set_members_over_a_loft() {
    let tol = Tol::witness();
    let mut doc = ProfileDoc::empty_derived("docm3_r1", tol);
    let mut profiles = Vec::new();
    for (z, s) in [(0.0, 1.0), (1.0, 1.6), (2.0, 1.0)] {
        let (d, id) = section(doc, z, s);
        doc = d;
        profiles.push(id);
    }
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: profiles.clone(),
            v_degree: editor_core::Expr::count(1),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, loft).is_none(), "{:?}", failure(&ev, loft));
    // Drop the middle section: applies, evaluates (a reversal applies
    // too but the kernel refuses `Loft(ReversedStacking)` at eval).
    let rev = doc
        .apply(
            &DocEdit::SetMembers {
                node: loft,
                members: vec![profiles[0], profiles[2]],
            },
            tol,
        )
        .expect("dropping a loft section applies")
        .doc;
    let ev = run(&rev);
    assert!(failure(&ev, loft).is_none(), "{:?}", failure(&ev, loft));
    // Duplicate, short, dangling, cycle — the same four refusals.
    let dup = doc.apply(
        &DocEdit::SetMembers {
            node: loft,
            members: vec![profiles[0], profiles[1], profiles[0]],
        },
        tol,
    );
    assert!(
        matches!(dup, Err(EditError::DuplicateInput { .. })),
        "{dup:?}"
    );
    let short = doc.apply(
        &DocEdit::SetMembers {
            node: loft,
            members: vec![profiles[0]],
        },
        tol,
    );
    assert!(
        matches!(short, Err(EditError::TooFewMembers { found: 1, .. })),
        "{short:?}"
    );
    let cyc = doc.apply(
        &DocEdit::SetMembers {
            node: loft,
            members: vec![profiles[0], loft],
        },
        tol,
    );
    assert!(matches!(cyc, Err(EditError::WouldCycle { .. })), "{cyc:?}");
    // The INSERT door now refuses a one-section loft over a live profile
    // (was: accepted at insert, refused at evaluation). Record which.
    let one = doc.apply(
        &DocEdit::InsertNode {
            node: Node::Loft {
                profiles: vec![profiles[0]],
                v_degree: editor_core::Expr::count(1),
            },
        },
        tol,
    );
    println!("r1 one-section loft at insert: {:?}", one.as_ref().err());
    assert!(matches!(
        one,
        Err(EditError::TooFewMembers { found: 1, .. })
    ));
    // Wire round trip of the loft edit.
    let edits: Vec<DocEdit<editor_core::ProfileProgram>> = {
        let mut v: Vec<_> = doc
            .order()
            .iter()
            .map(|id| DocEdit::InsertNode {
                node: doc.node(*id).unwrap().clone(),
            })
            .collect();
        v.push(DocEdit::SetMembers {
            node: loft,
            members: vec![profiles[2], profiles[0]],
        });
        v
    };
    let empty = ProfileDoc::empty_derived("docm3_r1", tol);
    let text = editor_core::persist::save(&empty, &edits, tol).unwrap();
    let loaded = editor_core::persist::load(&text, tol).unwrap();
    let mut replayed = empty.clone();
    for e in &edits {
        replayed = replayed.apply(e, tol).unwrap().doc;
    }
    assert!(loaded.doc.bit_eq(&replayed));
}

// ---------------------------------------------------------------------
// member_edge's consumers: refactor::split across a union.
// ---------------------------------------------------------------------

#[test]
fn r1_split_remaps_the_member_edge_and_refuses_a_cut_without_the_member() {
    let tol = Tol::witness();
    // Disjoint boxes: a whole box edge blends without a corner policy.
    let doc = ProfileDoc::empty_derived("docm3_r1", tol);
    let (doc, a) = boxed(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = boxed(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let bx = [a, b, a];
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![bx[0], bx[1]],
        },
    );
    let ev = run(&doc);
    // A DECLARE node naming two faces of the union: payload names, no
    // edges — the name-carrying node the re-map must reach.
    let faces: Vec<StableName> = all_faces(&ev, u)
        .into_iter()
        .filter(|n| matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == bx[1]))
        .collect();
    assert_eq!(faces.len(), 6);
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(faces[0].clone(), faces[1].clone())]),
    );
    // Whole-document cut: every node moves; the member edge must be
    // re-mapped to the part document's id for B.
    let cut: BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let out =
        split(&doc, &cut, DocumentId::derive("docm3_r1_part"), tol).expect("the split is legal");
    let b_new = out.node_map.get(&bx[1]).copied().expect("B is mapped");
    let decl_new = out.node_map.get(&decl).copied().unwrap();
    let Some(Node::Declare { pairs }) = out.part.node(decl_new) else {
        panic!()
    };
    assert_eq!(pairs.len(), 1);
    for name in [&pairs[0].0.0, &pairs[0].0.1] {
        match name.path.first() {
            Some(RoleSeg::FromMember { member, .. }) => {
                assert_eq!(
                    *member, b_new,
                    "the member edge was not re-mapped: {name:?}"
                );
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(name.node, out.node_map[&u]);
    }
    let ev_part = run(&out.part);
    assert!(
        failure(&ev_part, decl_new).is_none(),
        "{:?}",
        failure(&ev_part, decl_new)
    );
    // A cut holding only the declare node: its names reach the union
    // and the member outside the cut, so the split refuses on the NAME.
    let lone: BTreeSet<RecipeNodeId> = BTreeSet::from([decl]);
    let err = split(&doc, &lone, DocumentId::derive("docm3_r1_part2"), tol)
        .expect_err("names reach outside");
    println!("r1 lone-declare cut: {err:?}");
    assert!(format!("{err:?}").contains("FromMember"), "{err:?}");
}

// ---------------------------------------------------------------------
// The load door: a union whose FromMember names a node the document
// does not hold.
// ---------------------------------------------------------------------

#[test]
fn r1_snapshot_with_a_member_edge_to_a_missing_node_refuses() {
    let tol = Tol::witness();
    let (doc, bx) = intersecting_boxes();
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![bx[0], bx[1]],
        },
    );
    let ev = run(&doc);
    let pick = all_edges(&ev, u)
        .into_iter()
        .find(|n| matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == bx[1]))
        .unwrap();
    let (doc, _fillet) = insert(doc, Node::fillet(u, len(0.02), vec![pick.clone()]));
    let text = editor_core::persist::save(&doc, &[], tol).unwrap();
    // Point the member edge at an id past the mint counter.
    let needle = format!("\"member\": {}", bx[1].0);
    assert!(
        text.contains(&needle),
        "the member edge is on the wire as {needle}"
    );
    let tampered = text.replacen(&needle, "\"member\": 9999", 1);
    let r = editor_core::persist::load(&tampered, tol);
    println!("r1 tampered member edge: {:?}", r.as_ref().err());
    assert!(r.is_err(), "a member edge past the mint counter loaded");
}

// ---------------------------------------------------------------------
// roots::on_set_members — the product-root set after a bare SetMembers
// (no DeleteNode afterwards), checked through the load validator.
// ---------------------------------------------------------------------

#[test]
fn r1_set_members_alone_keeps_the_root_set_equal_to_the_sink_set() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("docm3_r1", tol);
    let (doc, a) = boxed(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = boxed(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = boxed(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![a, b],
        },
    );
    // c is a sink (a root), u is a root; a and b are consumed.
    let set = |r: &[RecipeNodeId]| r.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        set(doc.roots()),
        BTreeSet::from([c, u]),
        "{:?}",
        doc.roots()
    );
    // Swap b out for c: c stops being a sink, b becomes one.
    let doc = doc
        .apply(
            &DocEdit::SetMembers {
                node: u,
                members: vec![a, c],
            },
            tol,
        )
        .expect("applies")
        .doc;
    println!("r1 roots after SetMembers: {:?}", doc.roots());
    let text = editor_core::persist::save(&doc, &[], tol).expect("saves");
    let loaded = editor_core::persist::load(&text, tol).expect("the root set validates at load");
    assert!(loaded.doc.bit_eq(&doc));
    assert!(
        doc.roots().contains(&b),
        "the dropped member is a sink and must be a root"
    );
    assert!(
        !doc.roots().contains(&c),
        "the added member has a consumer and must not be a root"
    );
    assert!(doc.roots().contains(&u));
}
