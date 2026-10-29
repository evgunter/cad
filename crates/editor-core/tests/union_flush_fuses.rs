//! **Flush unions fuse, and name what they fuse** — two placements of
//! one prototype, a member resting under a merged wall, the merged rows
//! a flush union mints, the held flush corner, and what a kiss record
//! cites.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::corpus::body_of;
use crate::fixture::{self, flush_pairs, fname, insert, member_face, step, table};
use editor_core::{
    BooleanValue, DocEdit, EntityKind, Entry, Evaluation, Node, ProfileDoc, RecipeNodeId, RoleSeg,
    StableName, ValuePayload, VarId,
};
use geom_core::Tol;

/// A copy of `input` placed against its own geometry, slid along +x.
fn placed(doc: ProfileDoc, input: RecipeNodeId, dx: f64) -> (ProfileDoc, RecipeNodeId) {
    fixture::slid(doc, input, input, [dx, 0.0, 0.0])
}

/// **Two flush placements of ONE prototype fuse into one body.**
///
/// The margins decide the flush walls one carrier, so the fold glues
/// them.
#[test]
fn a_union_of_two_flush_placements_of_one_prototype_fuses() {
    let doc = ProfileDoc::empty_derived("docm7_placements", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![m1.into(), m2.into()]),
        },
    );
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the union refused: {:?}",
        failure(&ev, union)
    );
    let volume = topo::test_support::pinned_volume(
        "the fused body has mass",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&(body_of(&ev, union))),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume;
    assert_eq!(volume, 1.5, "the fused volume is the two blocks' union");
}

/// **A flush union mints `Merged` rows for the faces it fuses**, and
/// renames nothing else.
///
/// The four flush face pairs retire into four merged faces whose
/// constituents are the member-keyed names of the two coincident
/// faces. Every other face keeps its one-wrapper `From` name, built
/// here by hand rather than read back from the table, so the row
/// states the name instead of echoing it.
#[test]
fn a_flush_union_mints_merged_rows_and_renames_nothing_else() {
    let doc = ProfileDoc::empty_derived("docm7_merged", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let (doc, union) = fixture::union_over(doc, &[m1, m2]);
    let ev = run(&doc);
    let t = table(&ev, union);
    // Four merged faces: the two y-walls and the two caps.
    let merged: Vec<&StableName> = t
        .iter()
        .map(|(n, _)| n)
        .filter(|n| {
            n.kind == EntityKind::Face && matches!(n.path.first(), Some(RoleSeg::Merged(_)))
        })
        .collect();
    assert_eq!(
        merged.len(),
        4,
        "the four flush contacts merged: {merged:?}"
    );
    // And each merged row's constituents are the two coincident faces'
    // names, as a sorted set — so a selector spelled against the merged
    // face is exactly this name, and it resolves.
    for (a, b) in flush_pairs(&doc, (m1, proto), (m2, proto)) {
        // The row is the union's, so the constituents are the two
        // sited names in the union's own member space, each the
        // prototype's face under its member's copy.
        let mut set = vec![
            member_face(union, a.at, a.name),
            member_face(union, b.at, b.name),
        ];
        set.sort();
        let name = StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::Merged(set)],
        };
        assert!(
            matches!(t.lookup(&name), Some(Entry::Unique(_))),
            "the merged face {name} does not resolve"
        );
    }
    // The four flush rims along x are each joined into one edge across
    // both members, named for the set of the two members' rims.
    let joined: Vec<Vec<VarId>> = t
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Edge)
        .filter_map(|(n, _)| match n.path.as_slice() {
            [RoleSeg::Merged(set)] => Some(
                set.iter()
                    .map(|c| match c.path.as_slice() {
                        [RoleSeg::From { read, of }] if of.kind == EntityKind::Edge => *read,
                        _ => panic!("a joined edge's constituent is a member edge: {c}"),
                    })
                    .collect(),
            ),
            _ => None,
        })
        .collect();
    let mut both = vec![fixture::out(&doc, m1), fixture::out(&doc, m2)];
    both.sort();
    assert_eq!(joined, vec![both; 4], "four joined rims: {joined:?}");
    // The x-extreme walls are untouched: one `From` wrapper over
    // the prototype's own name under its member's copy.
    for (member, seg) in [(m1, 3u32), (m2, 1u32)] {
        let name = member_face(
            union,
            fixture::out(&doc, member),
            fname(
                proto,
                RoleSeg::Lateral(crate::fixture::piece(&doc, union, 0, seg as usize).into()),
            ),
        );
        assert!(
            matches!(t.lookup(&name), Some(Entry::Unique(_))),
            "the untouched wall {name} lost its member-keyed name"
        );
    }
}

/// **A member resting flush under a wall the fold merges fuses into
/// one body.**
///
/// Two flush placements of one prototype fuse their y=0 walls into one
/// `Merged` row; a third block rests flush under that merged wall.
/// Contact is judged pairwise before the fold, so the third block's
/// rest on each placement reaches the step that joins it.
#[test]
fn a_member_flush_under_a_merged_wall_fuses() {
    let doc = ProfileDoc::empty_derived("rv_r2_merged_refusal", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    // Flush under the merged y=0 wall (x 0..1.5 once m1 and m2 fuse).
    let (doc, m3) = block_on(doc, proto, (0.0, 1.5), (-1.0, 0.0), 0.0, 1.0);
    let (doc, union) = fixture::union_over(doc, &[m1, m2, m3]);
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the union refused: {:?}",
        failure(&ev, union)
    );
    let volume = topo::test_support::pinned_volume(
        "the fused body has mass",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&(body_of(&ev, union))),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume;
    assert_eq!(volume, 3.0, "the placements' 1.5 and m3's 1.5");
}

/// The union's entities on the corner two L-placed blocks share: the
/// vertical edge at x = 2, y = 0 and its two vertices, each lying
/// within an entity of both blocks.
fn shared_corner(ev: &Evaluation<f64>, union: RecipeNodeId) -> Vec<StableName> {
    let body = body_of(ev, union);
    let on_corner = |p: [f64; 3]| (p[0] - 2.0).abs() < 1e-9 && p[1].abs() < 1e-9;
    let mut out: Vec<StableName> = table(ev, union)
        .iter()
        .filter_map(|(name, entry)| {
            let Entry::Unique(at) = entry else {
                return None;
            };
            let held = match at.key {
                editor_core::EntityKey::Edge(k) => {
                    let edge = body.get_edge(k)?;
                    let p = body.half_edge_start_point(edge.he_plus)?.to_array();
                    let q = body.half_edge_start_point(edge.he_minus)?.to_array();
                    on_corner(p) && on_corner(q)
                }
                editor_core::EntityKey::Vertex(k) => {
                    let vertex = body.get_vertex(k)?;
                    on_corner(body.get_point(vertex.point)?.to_array())
                }
                _ => false,
            };
            held.then(|| name.clone())
        })
        .collect();
    out.sort();
    out
}

/// **A flush stretch several members hold is named for all of them**
/// (`names/README.md`, the flush rule; N0: no name reads mint order).
/// Two blocks placed in an L, `a` = [0, 2] × [0, 1] and `b` =
/// [1, 2] × [0, 2], share the vertical edge at x = 2, y = 0 and its two
/// end vertices, and the union names each of the three `Merged` of both
/// blocks' entities. A salt node inserted first moves every digest, `b`
/// is authored before or after `a`, and the member list is given both
/// ways round, so the answer is neither the digests' luck, nor which
/// block was minted first, nor the list's order. The union is then
/// re-pointed from `b` to a re-drawing of it (identical geometry, a
/// later id): the corner is named for `a` and the re-drawing, whose
/// read is a new key (DM4: re-pointing a member is a recipe edit), and
/// every entity `a` carries whole keeps its name.
#[test]
fn a_held_flush_stretch_is_named_for_every_member_that_holds_it() {
    let tol = Tol::witness();
    for salt in 0..6u32 {
        for (b_first, reversed) in [(false, false), (false, true), (true, false), (true, true)] {
            let listed = |x: RecipeNodeId, y: RecipeNodeId| {
                if reversed { vec![y, x] } else { vec![x, y] }
            };
            let case = format!("salt {salt}, b authored first {b_first}, list reversed {reversed}");
            let doc = ProfileDoc::empty_derived("redraw-flush", tol);
            let (doc, _) = insert(
                doc,
                Node::Profile(fixture::desc(vec![fixture::square(
                    f64::from(salt),
                    0.0,
                    0.5,
                )])),
            );
            // Whichever block is authored first is the root the other
            // two copies are placed against.
            let (doc, a, b, redrawn) = if b_first {
                let (doc, b) = block(doc, (1.0, 2.0), (0.0, 2.0), 1.0);
                let (doc, a) = block_on(doc, b, (0.0, 2.0), (0.0, 1.0), 0.0, 1.0);
                let (doc, redrawn) = block_on(doc, b, (1.0, 2.0), (0.0, 2.0), 0.0, 1.0);
                (doc, a, b, redrawn)
            } else {
                let (doc, a) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
                let (doc, b) = block_on(doc, a, (1.0, 2.0), (0.0, 2.0), 0.0, 1.0);
                let (doc, redrawn) = block_on(doc, a, (1.0, 2.0), (0.0, 2.0), 0.0, 1.0);
                (doc, a, b, redrawn)
            };
            let (doc, union) = fixture::union_over(doc, &listed(a, b));
            let ev = run(&doc);
            assert!(
                failure(&ev, union).is_none(),
                "{case}: {:?}",
                failure(&ev, union)
            );
            let held = shared_corner(&ev, union);
            assert_eq!(
                held.len(),
                3,
                "{case}: the corner edge and its two ends: {held:?}"
            );
            let both = |held: &[StableName], reads: [VarId; 2]| {
                held.iter().all(|name| {
                    let [RoleSeg::Merged(set)] = name.path.as_slice() else {
                        return false;
                    };
                    let cited: Vec<VarId> = set
                        .iter()
                        .filter_map(|c| match c.path.first() {
                            Some(RoleSeg::From { read, .. }) => Some(*read),
                            _ => None,
                        })
                        .collect();
                    cited.len() == set.len() && reads.iter().all(|r| cited.contains(r))
                })
            };
            let (a_read, b_read) = (fixture::out(&doc, a), fixture::out(&doc, b));
            assert!(
                both(&held, [a_read, b_read]),
                "{case}: the corner is named for both blocks: {held:?}"
            );

            let a_own = |ev: &Evaluation<f64>| -> Vec<StableName> {
                table(ev, union)
                    .iter()
                    .filter(|(name, _)| {
                        matches!(name.path.as_slice(), [RoleSeg::From { read, .. }] if *read == a_read)
                    })
                    .map(|(name, _)| name.clone())
                    .collect()
            };
            let a_before = a_own(&ev);
            assert!(
                !a_before.is_empty(),
                "{case}: `a` carries entities of its own whole"
            );
            let (doc, _) = step(
                doc,
                DocEdit::SetMembers {
                    node: union,
                    members: editor_core::Bodies::Spelled(
                        listed(a, redrawn).into_iter().map(Into::into).collect(),
                    ),
                },
            );
            let ev = run(&doc);
            assert!(
                failure(&ev, union).is_none(),
                "{case}: {:?}",
                failure(&ev, union)
            );
            assert_eq!(
                a_own(&ev),
                a_before,
                "{case}: re-pointing `b` leaves every entity `a` carries whole as it was"
            );
            let redrawn_read = fixture::out(&doc, redrawn);
            let now = shared_corner(&ev, union);
            assert!(
                now.len() == 3 && both(&now, [a_read, redrawn_read]),
                "{case}: the re-drawn corner is named for `a` and the re-drawing: {now:?}"
            );
        }
    }
}

/// The one vertex-pair record of `id`'s boolean value, and the row of
/// the node's own coincidences it cites.
fn kiss_record(ev: &Evaluation<f64>, id: RecipeNodeId) -> editor_core::NamedCoincidence {
    let value = ev.value(id).expect("the node evaluated");
    let ValuePayload::Boolean(BooleanValue::Body { .. }) = &value.payload else {
        panic!("a boolean body: {:?}", value.payload);
    };
    let contacts = &value.carry.contacts;
    let [record] = contacts.rows() else {
        panic!("one vertex pair: {contacts:?}");
    };
    let cited: Vec<topo::Backing> = record.cites.iter().collect();
    let [topo::Backing::Decided(k)] = cited[..] else {
        panic!("one decision of the node's own: {cited:?}");
    };
    value
        .coincidences
        .get(k as usize)
        .unwrap_or_else(|| panic!("row {k} of {:?}", value.coincidences))
        .clone()
}

/// **A corner kiss's record cites the vertex identity its node
/// decided, and the node names its inputs by their reads**: two blocks
/// meeting at one corner, joined as a pair (a two-member union) and as
/// the last step of a union with a third block apart. Each node's one
/// record cites a `VertexFusion` row of its own, naming the two members'
/// corners; the pair's records cite its inputs as its two member reads.
/// Red if a node publishes records citing rows it does not hold (a
/// union's last fold step's rows unpublished).
#[test]
fn a_kiss_record_cites_its_vertex_fusion_row_at_the_pair_and_the_union() {
    let doc = ProfileDoc::empty_derived("docm7_kiss_cites", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (1.0, 2.0), 1.0, 1.0);
    let (doc, far) = block_on(doc, a, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, pair) = fixture::union_over(doc, &[a, b]);
    let (doc, union) = fixture::union_over(doc, &[far, a, b]);
    let ev = run(&doc);
    for (what, id) in [("pair", pair), ("union", union)] {
        let row = kiss_record(&ev, id);
        assert_eq!(row.site, topo::DecisionSite::VertexFusion, "{what}");
        assert!(
            row.cells
                .iter()
                .all(|c| matches!(c, editor_core::NamedCell::Entity { .. })),
            "{what}: {row:?}"
        );
    }
    let pair_value = ev.value(pair).expect("the pair evaluated");
    let pair_row = kiss_record(&ev, pair);
    let (read_a, read_b) = (fixture::out(&doc, a), fixture::out(&doc, b));
    let mut inputs: Vec<VarId> = pair_row
        .cells
        .iter()
        .filter_map(|c| match c {
            editor_core::NamedCell::Entity { input, .. } => Some(*input),
            _ => None,
        })
        .collect();
    inputs.sort();
    let mut want = vec![read_a, read_b];
    want.sort();
    assert_eq!(inputs, want, "the row names one corner of each member");
    assert_eq!(
        pair_value.carry.cited_inputs.as_ref(),
        [
            editor_core::CitedInput::Read(read_a),
            editor_core::CitedInput::Read(read_b)
        ],
        "the pair cites its inputs by its reads"
    );
}

/// **A contact against what the fold merges glues in every member
/// order.** `a` and `c` are flush, so the fold merges their tops into
/// one `Merged({a.capEnd, c.capEnd})` row; `d` rests on both. The
/// margins decide each rest one carrier, so every order builds.
#[test]
fn a_contact_against_a_merged_cap_glues_in_every_order() {
    let doc = ProfileDoc::empty_derived("r1_merged_refusal", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block_on(doc, a, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    for order in [[a, c, d], [d, c, a], [c, d, a]] {
        let (doc, union) = fixture::union_over(doc.clone(), &order);
        let ev = run(&doc);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: {:?}",
            failure(&ev, union)
        );
        let v = topo::test_support::pinned_volume(
            "mass",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(&(body_of(&ev, union))),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume;
        assert!((v - (1.5 + 1.1 * 0.6 * 0.5)).abs() < 1e-9, "{order:?}: {v}");
    }
}

/// **A stepped pyramid of three slabs builds**: two resting contacts,
/// each glued by its margins.
#[test]
fn a_stack_of_resting_slabs_builds() {
    let doc = ProfileDoc::empty_derived("declared-pairs-converge", Tol::witness());
    let (doc, low) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, mid) = block_on(doc, low, (0.5, 2.5), (0.5, 2.5), 1.0, 1.0);
    let (doc, top) = block_on(doc, low, (1.0, 2.0), (1.0, 2.0), 2.0, 1.0);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![low.into(), mid.into(), top.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let volume = topo::test_support::pinned_volume(
        "the stack has mass",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&(body_of(&ev, union))),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume;
    assert_eq!(volume, 9.0 + 4.0 + 1.0);
}
