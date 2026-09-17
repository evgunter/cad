//! Review probes for EDIT-DECL (PR #2809), lane `decl-r1`. Each row
//! falsifies one claim of the PR body or of DM4 by execution; the
//! outcome each records is what the frozen head does, not what the
//! claim says.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run, table};
use crate::fixture::{ang, fname, insert, len, scl, step, wall};
use editor_core::{
    BooleanOp, CapEnd, DocEdit, EditError, Node, NodeErrorKind, ProfileDoc, RecipeNodeId,
    ResolveError, RoleSeg, SitedRef, find_flush_candidates,
};
use geom_core::Tol;

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

fn volume(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::mass_properties(body_of(ev, id), Tol::witness())
        .expect("mass")
        .volume
}

// ---------------------------------------------------------------------
// Claim 3 — what does a union's refusal name when the contact is
// against a MERGED row of the accumulation?
// ---------------------------------------------------------------------

/// `a` and `c` fuse at step 1 (declared), so the accumulation's top is
/// one `Merged({a.capEnd, c.capEnd})` row. `d` rests on that merged
/// row at step 2 and is undeclared. The PR says `union_refusal` hands
/// back a pair the caller can declare verbatim ("total by
/// construction"); this row records what it actually hands back.
#[test]
fn r1_a_union_refusal_against_a_merged_row() {
    let doc = ProfileDoc::empty_derived("r1_merged_refusal", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    let (doc, union, _) = declared_union(doc, &[a, c, d], flush_pairs((a, a), (c, c)));
    let ev = run(&doc);
    let got = failure(&ev, union);
    eprintln!("R1 merged-row refusal: {got:?}");
    assert!(
        matches!(got, Some(NodeErrorKind::UndeclaredContact { .. })),
        "a user's undeclared contact against a merged row is not reported as an \
         UndeclaredContact finding: {got:?}"
    );
}

/// The honest shape: declaring the contact through a CONSTITUENT of the
/// merged row, sited at its member, resolves through the look-through.
#[test]
fn r1_a_merged_row_contact_is_declared_through_a_constituent() {
    let doc = ProfileDoc::empty_derived("r1_merged_declared", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    let mut pairs = flush_pairs((a, a), (c, c));
    pairs.push((
        SitedRef::new(c, fname(c, RoleSeg::Cap(CapEnd::End))),
        SitedRef::new(d, fname(d, RoleSeg::Cap(CapEnd::Start))),
    ));
    let (doc, union, _) = declared_union(doc, &[a, c, d], pairs);
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let v = volume(&ev, union);
    assert!((v - (1.5 + 1.1 * 0.6 * 0.5)).abs() < 1e-9, "{v}");
}

// ---------------------------------------------------------------------
// Claim 2 — the site is the side, through a pass-through operand.
// ---------------------------------------------------------------------

/// Sited at the EXTRUDE (the minting node) when the boolean's operand
/// is a transform of it: `DeclareSiteNotAnOperand`. Sited at the
/// transform but naming a row it does not carry: `Vanished`.
#[test]
fn r1_pair_boolean_site_at_the_minting_node_and_absent_row() {
    let base = ProfileDoc::empty_derived("r1_site_mint", Tol::witness());
    let (base, a) = block(base, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (base, b0) = block(base, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (base, tr) = placed(base, b0, 0.5);
    let boolean = |doc: ProfileDoc, decl| {
        insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a,
                b: tr,
                declare: Some(decl),
            },
        )
    };
    // Sited at the minting node, which is not an operand.
    let (doc, decl) = insert(
        base.clone(),
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(a, wall(0))),
            SitedRef::new(b0, fname(b0, wall(0))),
        )]),
    );
    let (doc, u) = boolean(doc, decl);
    let ev = run(&doc);
    assert!(
        matches!(failure(&ev, u), Some(NodeErrorKind::DeclareSiteNotAnOperand { at }) if *at == b0),
        "{:?}",
        failure(&ev, u)
    );
    // Sited at the transform, naming a row the block does not have.
    let (doc, decl) = insert(
        base.clone(),
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(a, wall(0))),
            SitedRef::new(tr, fname(b0, wall(7))),
        )]),
    );
    let (doc, u) = boolean(doc, decl);
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, u),
            Some(NodeErrorKind::DeclareResolve { error }) if matches!(**error, ResolveError::Vanished { .. })
        ),
        "{:?}",
        failure(&ev, u)
    );
}

/// At the PAIR boolean, does rung 1 (`NodeGone`) outrank a site that
/// is not an operand? The union's door checks `live` before the site;
/// this row asks the pair boolean's door the same question.
#[test]
fn r1_pair_boolean_dead_name_versus_foreign_site() {
    let doc = ProfileDoc::empty_derived("r1_rung1_pair", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, x) = block(doc, (12.0, 13.0), (0.0, 1.0), 0.0, 1.0);
    // The name is `c`'s; the site is `x`, live but not an operand.
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(a, wall(0))),
            SitedRef::new(x, fname(c, wall(0))),
        )]),
    );
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: c });
    let ev = run(&doc);
    let got = failure(&ev, u);
    eprintln!("R1 pair-boolean dead name vs foreign site: {got:?}");
    assert!(
        matches!(got, Some(NodeErrorKind::DeclareResolve { error }) if matches!(**error, ResolveError::NodeGone { .. })),
        "rung 1 does not outrank the site question at the pair boolean: {got:?}"
    );
}

// ---------------------------------------------------------------------
// Claim 5 — deleting a site; rebinding a name away from its site.
// ---------------------------------------------------------------------

/// `SetMembers` dropping a site: the arm is `Vanished` (DM4: "as a
/// vanished name does").
#[test]
fn r1_set_members_dropping_a_site_refuses_vanished() {
    let doc = ProfileDoc::empty_derived("r1_dropped_arm", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, b, far], flush_pairs((a, a), (b, b)));
    let (doc, _) = step(
        doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a, far],
        },
    );
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::DeclareResolve { error }) if matches!(**error, ResolveError::Vanished { .. })
        ),
        "{:?}",
        failure(&ev, union)
    );
}

/// `DeleteNode` of a site (a transform member) cascades the union
/// away, so there is no consumer left to refuse: the `Declare`
/// survives with a dangling site AND a dangling name, and the next
/// evaluation refuses nothing. What DM7 reports at the delete is
/// recorded.
#[test]
fn r1_delete_node_of_a_site_cascades_and_nothing_refuses() {
    let doc = ProfileDoc::empty_derived("r1_delete_site", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let (doc, union, decl) = declared_union(doc, &[m1, m2], flush_pairs((m1, proto), (m2, proto)));
    let order = editor_core::cascade_delete_order(&doc, m2);
    assert_eq!(order, vec![union, m2], "{order:?}");
    let mut doc = doc;
    let mut maintenance = Vec::new();
    for id in order {
        let applied = doc
            .apply(&DocEdit::DeleteNode { id }, Tol::witness())
            .unwrap_or_else(|e| panic!("deleting {id:?}: {e:?}"));
        maintenance.extend(applied.maintenance);
        doc = applied.doc;
    }
    eprintln!("R1 delete-site maintenance: {maintenance:?}");
    assert!(doc.node(union).is_none(), "the union cascaded");
    assert!(doc.node(decl).is_some(), "the Declare survives");
    let ev = run(&doc);
    assert!(failure(&ev, decl).is_none(), "{:?}", failure(&ev, decl));
    // The maintenance names no strand: the names are `proto`'s, which
    // survives; the site is `m2`, which is not reported.
    let strands = maintenance
        .iter()
        .filter(|m| matches!(m, editor_core::Maintenance::Strand { .. }))
        .count();
    assert_eq!(strands, 0);
}

/// `Rebind` a pair's name onto a name minted elsewhere while the site
/// stays: the evaluation refuses `Vanished` (the name is not in the
/// site's table), and the site is untouched.
#[test]
fn r1_rebind_a_name_away_from_its_site_refuses_vanished() {
    let doc = ProfileDoc::empty_derived("r1_rebind", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
    let applied = doc
        .apply(
            &DocEdit::Rebind {
                from: fname(a, wall(0)),
                to: fname(far, wall(0)),
            },
            Tol::witness(),
        )
        .expect("the rebind applies");
    let doc = applied.doc;
    let Some(Node::Declare { pairs }) = doc.node(decl) else {
        panic!("the Declare survives")
    };
    let moved: Vec<&SitedRef> = pairs
        .iter()
        .flat_map(|((x, y), _)| [x, y])
        .filter(|r| r.name.node == far)
        .collect();
    assert_eq!(moved.len(), 1);
    assert_eq!(moved[0].at, a, "the site stays");
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::DeclareResolve { error }) if matches!(**error, ResolveError::Vanished { .. })
        ),
        "{:?}",
        failure(&ev, union)
    );
}

// ---------------------------------------------------------------------
// Claim 6 — the persisted form.
// ---------------------------------------------------------------------

/// The bare-name refusal: what error is it, and does it name the pair?
#[test]
fn r1_bare_name_load_refusal_shape() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("r1_bare", tol);
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, _union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
    let text = editor_core::persist::save(&doc, &[], tol).expect("saves");
    let split = text.find('{').expect("body");
    let (header, body) = text.split_at(split);
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("parses");
    let side = &mut wire["snapshot"]["nodes"][decl.0.to_string()]["Declare"]["pairs"][0][0][0];
    let bare = side["name"].clone();
    *side = bare;
    let err = editor_core::persist::load(&format!("{header}{wire}"), tol).expect_err("refuses");
    eprintln!("R1 bare-name load refusal Debug: {err:?}");
    eprintln!("R1 bare-name load refusal Display: {err}");
}

/// Every corpus document that declares: the `Declare` precedes its
/// consumer in `order()`, names and sites point backward, and
/// re-inserting the nodes in document order is accepted.
#[test]
fn r1_every_declaring_corpus_document_replays_in_document_order() {
    let mut declaring = 0;
    for d in crate::corpus::documents() {
        let doc = &d.doc;
        let positions = |id: RecipeNodeId| doc.order().iter().position(|n| *n == id);
        let mut has_declare = false;
        for id in doc.order() {
            if let Some(Node::Declare { pairs }) = doc.node(*id) {
                has_declare = true;
                for r in pairs.iter().flat_map(|((x, y), _)| [x, y]) {
                    assert!(positions(r.at) < positions(*id), "{}: site forward", d.name);
                    assert!(
                        positions(r.name.node) < positions(*id),
                        "{}: name forward",
                        d.name
                    );
                }
            }
        }
        if !has_declare {
            continue;
        }
        declaring += 1;
        // The edit log replays from empty, edit by edit, and every
        // `Declare` insert precedes the insert of its consumer.
        let mut replay = ProfileDoc::empty_derived("r1_replay", Tol::witness());
        let mut declares_seen = 0;
        for (i, edit) in d.edits.iter().enumerate() {
            if let DocEdit::InsertNode { node } = edit {
                match node {
                    Node::Declare { .. } => declares_seen += 1,
                    Node::Boolean {
                        declare: Some(_), ..
                    }
                    | Node::Union {
                        declare: Some(_), ..
                    } => assert!(declares_seen > 0, "{}: consumer at edit {i} first", d.name),
                    _ => {}
                }
            }
            replay = replay
                .apply(edit, Tol::witness())
                .unwrap_or_else(|e| panic!("{}: edit {i} refused: {e:?}", d.name))
                .doc;
        }
        assert_eq!(replay.order(), doc.order(), "{}: the replay is the document", d.name);
    }
    eprintln!("R1 declaring corpus documents: {declaring}");
    assert!(declaring >= 6, "{declaring}");
}

// ---------------------------------------------------------------------
// Claim 8 — my mutant: `payload_read_sites` yielding only the FIRST
// site of each pair. This row is the one that would catch it: the
// suite's insert-door row puts the dead site FIRST.
// ---------------------------------------------------------------------

#[test]
fn r1_the_insert_door_checks_the_second_site_too() {
    let doc = ProfileDoc::empty_derived("r1_second_site", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let future = RecipeNodeId(doc.order().last().expect("a node").0 + 1);
    let refused = doc.apply(
        &DocEdit::InsertNode {
            node: Node::declare_rest(vec![(
                SitedRef::new(a, fname(a, wall(0))),
                SitedRef::new(future, fname(b, wall(0))),
            )]),
        },
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(EditError::ReadSiteMissingNode { at }) if at == future),
        "{refused:?}"
    );
}

// ---------------------------------------------------------------------
// Claim 10 (kernel half) — the DM4 headline through the flush door:
// findings of two placements of one prototype, declared verbatim,
// consumed by a union in one pass.
// ---------------------------------------------------------------------

#[test]
fn r1_flush_findings_of_two_placements_declare_and_fuse_through_a_union() {
    let doc = ProfileDoc::empty_derived("r1_flush_union", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let ev = run(&doc);
    let findings = find_flush_candidates(&ev, m1, m2, Tol::witness()).expect("detects");
    assert_eq!(findings.len(), 4, "{findings:?}");
    for f in &findings {
        assert_eq!((f.pair.0.at, f.pair.1.at), (m1, m2));
        assert_eq!((f.pair.0.name.node, f.pair.1.name.node), (proto, proto));
    }
    let (applied, decl) = editor_core::declare_all(&doc, &findings, Tol::witness()).expect("declares");
    let (doc, union) = insert(
        applied.doc,
        Node::Union {
            members: vec![m1, m2],
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(volume(&ev, union), 1.5);
    // And the union's OWN refusal, declared verbatim, is the same set.
    let (bare, plain) = insert(
        doc.clone(),
        Node::Union {
            members: vec![m1, m2],
            declare: None,
        },
    );
    let ev = run(&bare);
    let Some(NodeErrorKind::UndeclaredContact { finding, .. }) = failure(&ev, plain) else {
        panic!("{:?}", failure(&ev, plain))
    };
    let (applied, decl2) =
        editor_core::declare_all(&bare, core::slice::from_ref(&**finding), Tol::witness())
            .expect("declares the refusal verbatim");
    let (doc2, union2) = insert(
        applied.doc,
        Node::Union {
            members: vec![m1, m2],
            declare: Some(decl2),
        },
    );
    let ev = run(&doc2);
    // One pair declared of four: the other three contacts still refuse
    // — recorded, whichever way it goes.
    eprintln!("R1 one refusal declared verbatim: {:?}", failure(&ev, union2));
    let _ = table(&ev, m1);
}
