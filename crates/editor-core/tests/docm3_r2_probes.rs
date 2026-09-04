//! **Review lane R2 probes** for DOCM-3 (`Node::Union`).
//!
//! These are the reviewer's rows, not the unit's: a union whose
//! members actually INTERSECT (the unit's own rows union disjoint
//! bodies only), and the paths nothing else executes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanValue, CancelToken, EntityKind, EvalOptions, Evaluation, Node, NodeResult, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, ValuePayload, all_edges, all_faces, evaluate,
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

/// A unit box on z = `z0` at (x0, y0), extruded 1.
fn boxlet(doc: ProfileDoc, x0: f64, y0: f64, z0: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (x0, y0),
            (x0 + 1.0, y0),
            (x0 + 1.0, y0 + 1.0),
            (x0, y0 + 1.0),
        ]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    )
}

/// Three boxes that OVERLAP pairwise-adjacently, unioned in the given
/// member order.
fn overlapping(order: [usize; 3]) -> (ProfileDoc, [RecipeNodeId; 3], RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("docm3_r2", Tol::witness());
    let (doc, a) = boxlet(doc, 0.0, 0.0, 0.0);
    let (doc, b) = boxlet(doc, 0.5, 0.5, 0.25);
    let (doc, c) = boxlet(doc, 1.0, 1.0, 0.5);
    let boxes = [a, b, c];
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: order.map(|i| boxes[i]).to_vec(),
        },
    );
    (doc, boxes, u)
}

fn head(name: &StableName) -> &'static str {
    match name.path.first() {
        Some(RoleSeg::FromMember { .. }) => "FromMember",
        Some(RoleSeg::Seam { .. }) => "Seam",
        Some(RoleSeg::Merged(_)) => "Merged",
        Some(RoleSeg::OutputBody) => "OutputBody",
        Some(RoleSeg::FromA(_)) => "FromA",
        Some(RoleSeg::FromB(_)) => "FromB",
        Some(_) => "other",
        None => "empty",
    }
}

/// **The census of what an INTERSECTING union actually names.** The
/// unit's own rows union disjoint boxes, so `collapse`'s `Seam`,
/// `Merged` and `Fragment` arms are never reached by them.
#[test]
fn r2_intersecting_union_name_census() {
    let (doc, boxes, u) = overlapping([0, 1, 2]);
    let ev = run(&doc);
    let v = match ev.nodes.get(&u) {
        Some(NodeResult::Ok(v)) => v,
        other => panic!("the union did not evaluate: {other:?}"),
    };
    let mut census: std::collections::BTreeMap<(&str, &str), usize> = Default::default();
    for (name, _) in v.name_table.iter() {
        let kind = match name.kind {
            EntityKind::Body => "Body",
            EntityKind::Face => "Face",
            EntityKind::Edge => "Edge",
            EntityKind::Vertex => "Vertex",
        };
        *census.entry((kind, head(name))).or_default() += 1;
        assert_eq!(name.node, u, "every row is minted by the union: {name:?}");
    }
    eprintln!("members {boxes:?} union {u:?}");
    for ((kind, h), n) in &census {
        eprintln!("  {kind:>7} {h:<12} {n}");
    }
    for (name, _) in v.name_table.iter() {
        let s = format!("{name:?}");
        assert!(
            !s.contains("FromA") && !s.contains("FromB"),
            "a fold row survived into the union's table: {name:?}"
        );
    }
    assert!(
        census.keys().any(|(_, h)| *h == "Seam"),
        "an intersecting union mints seam rows; census {census:?}"
    );
}

/// **A2 generalized to members that intersect.** The union's whole
/// name table, as a SET, must not move with the member order.
#[test]
fn r2_intersecting_names_are_position_free() {
    let (doc, boxes, u) = overlapping([0, 1, 2]);
    let (rd, rboxes, ru) = overlapping([2, 1, 0]);
    assert_eq!(boxes, rboxes);
    let (ev, rev) = (run(&doc), run(&rd));
    let names = |ev: &Evaluation<f64>, u: RecipeNodeId| -> Vec<String> {
        match ev.nodes.get(&u) {
            Some(NodeResult::Ok(v)) => {
                let mut s: Vec<String> =
                    v.name_table.iter().map(|(n, _)| format!("{n:?}")).collect();
                s.sort();
                s
            }
            other => panic!("the union did not evaluate: {other:?}"),
        }
    };
    let (f, r) = (names(&ev, u), names(&rev, ru));
    let only_f: Vec<&String> = f.iter().filter(|n| !r.contains(n)).collect();
    let only_r: Vec<&String> = r.iter().filter(|n| !f.contains(n)).collect();
    eprintln!(
        "rows {} / {}; forward-only {}, reverse-only {}",
        f.len(),
        r.len(),
        only_f.len(),
        only_r.len()
    );
    for n in only_f.iter().take(4) {
        eprintln!("  F {n}");
    }
    for n in only_r.iter().take(4) {
        eprintln!("  R {n}");
    }
    assert_eq!(f, r, "the union's name set moved with the member order");
}

/// **The middle member of an intersecting union is droppable.**
#[test]
fn r2_dropping_an_intersecting_member_leaves_the_others_alone() {
    let (doc, boxes, u) = overlapping([0, 1, 2]);
    let ev = run(&doc);
    let before: Vec<StableName> = all_faces(&ev, u)
        .into_iter()
        .filter(|n| {
            matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member != boxes[1])
        })
        .collect();
    let after_doc = editor_core::apply(
        &doc,
        &editor_core::DocEdit::SetMembers {
            node: u,
            members: vec![boxes[0], boxes[2]],
        },
        Tol::witness(),
    )
    .expect("SetMembers accepted")
    .doc;
    let ev2 = run(&after_doc);
    let after: Vec<StableName> = all_faces(&ev2, u);
    let lost: Vec<&StableName> = before.iter().filter(|n| !after.contains(n)).collect();
    eprintln!("kept-member faces before {}, lost {}", before.len(), lost.len());
    for n in lost.iter().take(6) {
        eprintln!("  lost {n:?}");
    }
    assert!(lost.is_empty(), "a surviving member's names changed");
}

/// **Every union name resolves back to the entity it names.**
#[test]
fn r2_intersecting_union_names_denote() {
    let (doc, _, u) = overlapping([0, 1, 2]);
    let ev = run(&doc);
    for name in all_faces(&ev, u).into_iter().chain(all_edges(&ev, u)) {
        editor_core::denotation(&ev, u, &name)
            .unwrap_or_else(|e| panic!("a union name does not denote: {name:?} -> {e:?}"));
    }
}

/// **The empty-member path.** A member that evaluates empty refuses
/// typed rather than being absorbed.
#[test]
fn r2_an_empty_member_refuses() {
    let doc = ProfileDoc::empty_derived("docm3_r2_empty", Tol::witness());
    let (doc, a) = boxlet(doc, 0.0, 0.0, 0.0);
    let (doc, b) = boxlet(doc, 10.0, 10.0, 0.0);
    let (doc, empty) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Intersect,
            a,
            b,
            declare: None,
        },
    );
    let (doc, c) = boxlet(doc, 20.0, 20.0, 0.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![a, empty, c],
        },
    );
    let ev = run(&doc);
    match ev.nodes.get(&empty) {
        Some(NodeResult::Ok(v)) => assert!(
            matches!(&v.payload, ValuePayload::Boolean(BooleanValue::Empty)),
            "the fixture is an empty member: {:?}",
            v.payload
        ),
        other => panic!("the intersection did not evaluate: {other:?}"),
    }
    match ev.nodes.get(&u) {
        Some(NodeResult::Failed(e)) => eprintln!("union refused: {e:?}"),
        other => panic!("an empty member must refuse typed, got {other:?}"),
    }
}

/// **A `Merged` row under a union.** Two members that are placements
/// of ONE prototype, flush against each other, so the shared faces are
/// STRUCTURALLY coincident (F7/N3) and could merge without a
/// declaration — the arm `collapse` carries for `RoleSeg::Merged`.
#[test]
fn r2_a_union_of_two_flush_placements_of_one_prototype() {
    let doc = ProfileDoc::empty_derived("docm3_r2_flush", Tol::witness());
    let (doc, proto) = boxlet(doc, 0.0, 0.0, 0.0);
    let (doc, l) = insert(
        doc,
        Node::Transform {
            input: proto,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
            rotation_angle: fixture::ang(0.0),
        },
    );
    let (doc, r) = insert(
        doc,
        Node::Transform {
            input: proto,
            translation: [len(1.0), len(0.0), len(0.0)],
            rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
            rotation_angle: fixture::ang(0.0),
        },
    );
    let (doc, u) = insert(doc, Node::Union { members: vec![l, r] });
    let ev = run(&doc);
    match ev.nodes.get(&u) {
        Some(NodeResult::Ok(v)) => {
            let mut census: std::collections::BTreeMap<&str, usize> = Default::default();
            for (name, _) in v.name_table.iter() {
                *census.entry(head(name)).or_default() += 1;
            }
            eprintln!("flush union census {census:?}");
        }
        Some(NodeResult::Failed(e)) => eprintln!("flush union refused: {:?}", e.kind),
        other => panic!("no result: {other:?}"),
    }
}

/// **A `Fragment` row under a union.** A member that crosses another
/// member's face divides it, so the surviving pieces carry the
/// boolean emitter's `Fragment` discriminators — the tail arm of
/// `collapse`.
#[test]
fn r2_a_member_that_divides_anothers_face() {
    let doc = ProfileDoc::empty_derived("docm3_r2_frag", Tol::witness());
    let (doc, pslab) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, slab) = insert(
        doc,
        Node::Extrude {
            profile: pslab,
            distance: len(1.0),
        },
    );
    let (doc, ppost) = on_frame(
        doc,
        [0.0, 0.0, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, -1.0), (2.0, -1.0), (2.0, 2.0), (1.0, 2.0)]],
    );
    let (doc, post) = insert(
        doc,
        Node::Extrude {
            profile: ppost,
            distance: len(1.5),
        },
    );
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![slab, post],
        },
    );
    let ev = run(&doc);
    match ev.nodes.get(&u) {
        Some(NodeResult::Ok(v)) => {
            let mut frags = 0;
            for (name, _) in v.name_table.iter() {
                let s = format!("{name:?}");
                if s.contains("Fragment") {
                    frags += 1;
                    if frags <= 3 {
                        eprintln!("  frag {name:?}");
                    }
                }
                assert!(
                    !s.contains("FromA") && !s.contains("FromB"),
                    "a fold row survived: {name:?}"
                );
            }
            eprintln!("fragment rows: {frags}");
        }
        Some(NodeResult::Failed(e)) => eprintln!("divided union refused: {:?}", e.kind),
        other => panic!("no result: {other:?}"),
    }
}

/// **Every pip, not three.** For each of the die's 21 pips, exactly
/// two of the rim blend's 42 frozen arcs name that pip — the
/// `rims − 2` arithmetic checked against the geometry's own names.
#[test]
fn r2_every_pip_owns_exactly_two_rim_arcs() {
    let die = crate::corpus::die_composed_tour::document();
    let doc = die.doc;
    let union = doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Union { .. })))
        .expect("one union");
    let Some(Node::Union { members }) = doc.node(union) else {
        panic!("the union is a union")
    };
    let blends: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|id| matches!(doc.node(*id), Some(Node::Fillet { .. })))
        .collect();
    let Some(Node::Fillet { selection, .. }) = doc.node(blends[1]) else {
        panic!("the rim blend is a fillet")
    };
    assert_eq!(selection.len(), 42);
    let mut total = 0;
    for (k, m) in members.iter().enumerate() {
        let owned = selection
            .iter()
            .filter(|n| editor_core::derivation_nodes(n).contains(m))
            .count();
        assert_eq!(owned, 2, "pip {k} ({m:?}) owns {owned} rim arcs, not two");
        total += owned;
    }
    assert_eq!(total, 42, "every selected arc belongs to exactly one pip");
}

/// **`refactor::split` across a union.** A recipe carrying a union and
/// a blend whose frozen selection names union entities is cut into a
/// part; those names carry `FromMember`, whose member id is a LOCAL
/// node reference and must cross the remap.
#[test]
fn r2_a_union_survives_being_split_into_a_part() {
    let (doc, _, u) = overlapping([0, 1, 2]);
    let ev = run(&doc);
    let selection: Vec<StableName> = all_edges(&ev, u)
        .into_iter()
        .filter(|n| matches!(n.path.first(), Some(RoleSeg::Seam { .. })))
        .take(2)
        .collect();
    assert!(!selection.is_empty(), "a seam edge to blend");
    let (doc, blend) = insert(doc, Node::fillet(u, len(0.05), selection.clone()));
    let cut: std::collections::BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let out = editor_core::refactor::split(
        &doc,
        &cut,
        editor_core::DocumentId::derive("docm3-r2-part"),
        Tol::witness(),
    )
    .expect("the whole recipe is a closed cut");
    let part = out.part;
    let mapped_blend = *out.node_map.get(&blend).expect("the blend crossed");
    let mapped_union = *out.node_map.get(&u).expect("the union crossed");
    let Some(Node::Fillet { selection: sel, .. }) = part.node(mapped_blend) else {
        panic!("the blend is in the part")
    };
    for n in sel {
        for id in editor_core::derivation_nodes(n) {
            assert!(
                part.node(id).is_some(),
                "a remapped selection name points outside the part: {n:?} -> {id:?}"
            );
        }
    }
    let pev = evaluate::<f64>(
        &part,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match pev.nodes.get(&mapped_union) {
        Some(NodeResult::Ok(_)) => {}
        other => panic!("the part's union did not evaluate: {other:?}"),
    }
    // The blend's own geometry is not the point (a seam chain need not
    // be G1); that its FROZEN SELECTION still resolves is.
    if let Some(NodeResult::Failed(e)) = pev.nodes.get(&mapped_blend) {
        let kind = format!("{:?}", e.kind);
        assert!(
            !kind.contains("SelectionResolve") && !kind.contains("Resolve"),
            "the part's blend could not resolve its remapped selection: {kind}"
        );
        eprintln!("part blend refused geometrically (not by name): {kind}");
    }
}

/// A three-section loft, for the list-input edit's OTHER node kind.
fn loft_doc() -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty_derived("docm3_r2_loft", Tol::witness());
    let mut profiles = Vec::new();
    for (z, s) in [(0.0, 1.0), (1.0, 1.6), (2.0, 1.0), (3.0, 1.2)] {
        let (d, id) = on_frame(
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
        );
        doc = d;
        profiles.push(id);
    }
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: profiles[..3].to_vec(),
            v_degree: editor_core::Expr::count(2),
        },
    );
    (doc, loft, profiles)
}

/// **C4 on `Loft`.** `SetMembers` reaches the other list node, and its
/// four refusals answer there too.
#[test]
fn r2_set_members_over_a_loft() {
    let tol = Tol::witness();
    let (doc, loft, profiles) = loft_doc();
    let grown = doc
        .apply(
            &editor_core::DocEdit::SetMembers {
                node: loft,
                members: profiles.clone(),
            },
            tol,
        )
        .expect("a loft takes the list edit")
        .doc;
    assert_eq!(
        grown.node(loft).and_then(Node::list_input),
        Some(&profiles[..]),
        "the loft's sections are the new list"
    );
    let ev = run(&grown);
    match ev.nodes.get(&loft) {
        Some(NodeResult::Ok(_)) => {}
        other => panic!("the re-sectioned loft did not evaluate: {other:?}"),
    }
    for (what, members) in [
        ("duplicate", vec![profiles[0], profiles[0], profiles[1]]),
        ("short", vec![profiles[0]]),
        ("dangling", vec![profiles[0], RecipeNodeId(9999)]),
    ] {
        let err = doc
            .apply(
                &editor_core::DocEdit::SetMembers {
                    node: loft,
                    members,
                },
                tol,
            )
            .expect_err("the loft's list edit refuses");
        eprintln!("loft {what}: {err:?}");
    }
    // The wire round trip, on the OTHER list node: the SNAPSHOT half,
    // which the unit's own replay row does not exercise (it saves an
    // EMPTY base document plus an edit log).
    let edits: Vec<editor_core::DocEdit<editor_core::ProfileProgram>> = vec![];
    let text = editor_core::persist::save(&grown, &edits, tol).expect("the document saves");
    let loaded = editor_core::persist::load(&text, tol).expect("the document loads");
    assert!(
        loaded.doc.bit_eq(&grown),
        "a loft whose sections were set does not round trip"
    );
}

/// **A one-section `Loft` at the INSERT door.** DM5 is about pairwise
/// distinctness; the list floor is DM4's, and `input_fault` applies it
/// to `Loft` as well as to `Union` — so this insert, which the door
/// took before this unit, is refused now.
#[test]
fn r2_a_one_section_loft_at_the_insert_door() {
    let (doc, _, profiles) = loft_doc();
    let outcome = doc.apply(
        &editor_core::DocEdit::InsertNode {
            node: Node::Loft {
                profiles: vec![profiles[0]],
                v_degree: editor_core::Expr::count(1),
            },
        },
        Tol::witness(),
    );
    match outcome {
        Ok(_) => eprintln!("a one-section loft still inserts"),
        Err(e) => eprintln!("a one-section loft now refuses at the edit door: {e:?}"),
    }
}

/// **A3 with the CURVES and POINTS in it.** The unit's A3 rows compare
/// face/edge/vertex COUNTS and a sorted multiset of surface `Debug`
/// strings; nothing compares the curve geometry or the points. This
/// row adds both, over the die's 21 pips, so the "bit-identical"
/// claim is measured on more than the surfaces.
#[test]
fn r2_the_die_fold_and_chain_agree_on_curves_and_points() {
    let die = crate::corpus::die_composed_tour::document();
    let doc = die.doc;
    let union = doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Union { .. })))
        .expect("one union");
    let Some(Node::Union { members }) = doc.node(union) else {
        panic!("the union is a union")
    };
    let members = members.clone();
    let (doc, chain) = members.iter().skip(1).fold(
        (doc, members[0]),
        |(doc, acc): (ProfileDoc, RecipeNodeId), pip| {
            insert(
                doc,
                Node::Boolean {
                    op: editor_core::BooleanOp::Union,
                    a: acc,
                    b: *pip,
                    declare: None,
                },
            )
        },
    );
    let ev = run(&doc);
    let body = |id: RecipeNodeId| match &ev.value(id).expect("evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    };
    let (folded, chained) = (body(union), body(chain));
    let sorted = |mut v: Vec<String>| {
        v.sort();
        v
    };
    assert_eq!(
        sorted(folded.curves().map(|(_, c)| format!("{c:?}")).collect()),
        sorted(chained.curves().map(|(_, c)| format!("{c:?}")).collect()),
        "the fold's curves are not the chain's"
    );
    assert_eq!(
        sorted(folded.points().map(|(_, p)| format!("{p:?}")).collect()),
        sorted(chained.points().map(|(_, p)| format!("{p:?}")).collect()),
        "the fold's points are not the chain's"
    );
}

/// **What a refusal raised at a LATER fold step names.** Two members
/// fuse; the third meets them flush, so the step refuses and
/// `refusal_menu` reads the ACCUMULATED table — which is the pair
/// emitter's, minted under the union's id with `FromA`/`FromB` heads.
/// The name in the payload is therefore in the fold's internal space
/// and is in no table any consumer can look up.
#[test]
fn r2_a_refusal_at_a_later_fold_step_names_a_fold_row() {
    let doc = ProfileDoc::empty_derived("docm3_r2_menu", Tol::witness());
    let (doc, a) = boxlet(doc, 0.0, 0.0, 0.0);
    let (doc, b) = boxlet(doc, 2.0, 0.0, 0.0);
    let (doc, c) = boxlet(doc, 1.0, 0.0, 0.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![a, b, c],
        },
    );
    let ev = run(&doc);
    let Some(NodeResult::Failed(e)) = ev.nodes.get(&u) else {
        panic!("the flush third member must refuse: {:?}", ev.nodes.get(&u));
    };
    let text = format!("{:?}", e.kind);
    eprintln!("later-step refusal: {text}");
    assert!(
        text.contains("FromA") || text.contains("FromB"),
        "expected a fold-space name in the refusal payload, got {text}"
    );
}

/// **A union of a body and a PLACEMENT of that body names fine.** The
/// viewer's `combine_ops` fixture says the opposite in prose (the
/// `union` candidate's comment: "a union of a body and a placement of
/// that same body has two members whose tables give one entity one
/// name, and refuses at naming"). Under the amendment it does not: the
/// member EDGE tells them apart, which is what `FromMember` is for.
#[test]
fn r2_a_body_and_a_placement_of_it_are_two_members() {
    let doc = ProfileDoc::empty_derived("docm3_r2_placement", Tol::witness());
    let (doc, base) = boxlet(doc, 0.0, 0.0, 0.0);
    let (doc, moved) = insert(
        doc,
        Node::Transform {
            input: base,
            translation: [len(3.0), len(0.0), len(0.0)],
            rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
            rotation_angle: fixture::ang(0.0),
        },
    );
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![base, moved],
        },
    );
    let ev = run(&doc);
    match ev.nodes.get(&u) {
        Some(NodeResult::Ok(_)) => {}
        other => panic!("a body plus a placement of it did not name: {other:?}"),
    }
    let faces = all_faces(&ev, u);
    assert_eq!(faces.len(), 12, "twelve distinct faces, six per member");
    for m in [base, moved] {
        let n = faces
            .iter()
            .filter(|f| {
                matches!(f.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == m)
            })
            .count();
        assert_eq!(n, 6, "member {m:?} contributes six faces");
    }
}
