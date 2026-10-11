//! Reviewer R1 probes for PR #4674 (FORK-DM4 unit 2). Each row states
//! what it expects of a correct tree; a red row is a finding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len, on_frame, scl};
use editor_core::{
    Axis3, Datum, DocEdit, EntityKind, Entry, ExtrudeSide, Node, ProfileDoc, RecipeNodeId,
    Resolution, ResolveError, RoleSeg, RunCtx, SlotId, SplitHalf, StableName, resolve,
    resolve_with_prior,
};
use geom_core::Tol;

fn step_split(at: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("r1_dm4_probes", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (0.0, 1.0),
        ]],
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(at), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: ext.into(),
            tool: tool.into(),
        },
    );
    (doc, ext, tool, split)
}

/// P1: a vanished name's tombstone patch is "the output body and entity
/// key within that node's value" (`MeshPatchKey`). For a name the prior
/// run held in the LOWER half, that is body 1 of the split's value.
#[test]
fn p1_tombstone_of_a_lower_half_name_points_at_the_lower_body() {
    // Prior: plane at x = 3, the riser (x = 2) is Below. New: plane at
    // x = 1, the riser is Above, so the lower read loses it.
    let (doc, ext, tool, split) = step_split(3.0);
    let riser = fixture::fname(ext, fixture::wall(&doc, ext, 3));
    let below = doc.output(split, SplitHalf::Below.port()).unwrap();
    let before = fixture::run(&doc, &Default::default());
    let prior = RunCtx {
        doc: &doc,
        eval: &before,
    };
    let Resolution::Resolved(was) = resolve(prior, below, &riser) else {
        panic!("premise: the riser is the lower half's in the prior run");
    };
    assert_eq!(was.entity.body, 1, "premise: body 1 of the split's value");
    let (moved, _) = fixture::step(
        doc.clone(),
        DocEdit::SetParam {
            node: tool,
            slot: SlotId::Origin(Axis3::X),
            value: len(1.0).into(),
            fresh: Vec::new(),
        },
    );
    let after = fixture::run(&moved, &Default::default());
    let new = RunCtx {
        doc: &moved,
        eval: &after,
    };
    match resolve_with_prior(new, prior, below, &riser) {
        Resolution::Failed(f) => match f.error {
            ResolveError::Vanished { last_good, .. } => {
                let ghost = last_good.expect("a last-good tombstone");
                println!("P1 tombstone patch = {:?}; prior resolved at {:?}", ghost.patch, was);
                assert_eq!(
                    ghost.patch.entity, was.entity,
                    "the tombstone's patch is the entity the prior run resolved in the lower read"
                );
            }
            other => panic!("vanished, not {other:?}"),
        },
        other => panic!("the lower read lost it: {other:?}"),
    }
}

/// P1b: a split-minted name both halves hold (a cut wall's fragment),
/// referenced through the LOWER read, vanishes when the plane leaves
/// the body. Its tombstone should be the lower half's fragment (body 1
/// of the split's value in the prior run), not the upper half's.
#[test]
fn p1b_tombstone_of_a_shared_fragment_read_below_is_the_lower_ones() {
    let (doc, _ext, tool, split) = step_split(1.0);
    let above = doc.output(split, 0).unwrap();
    let below = doc.output(split, 1).unwrap();
    let before = fixture::run(&doc, &Default::default());
    let (ta, tb) = (before.table(above).unwrap(), before.table(below).unwrap());
    let frag = ta
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| {
            n.kind == EntityKind::Face
                && matches!(n.path.last(), Some(RoleSeg::SplitFragment { .. }))
                && tb.lookup(n).is_some()
        })
        .expect("a shared fragment");
    let prior = RunCtx {
        doc: &doc,
        eval: &before,
    };
    let Resolution::Resolved(was) = resolve(prior, below, &frag) else {
        panic!("premise");
    };
    assert_eq!(was.entity.body, 1);
    let (moved, _) = fixture::step(
        doc.clone(),
        DocEdit::SetParam {
            node: tool,
            slot: SlotId::Origin(Axis3::X),
            value: len(5.0).into(),
            fresh: Vec::new(),
        },
    );
    let after = fixture::run(&moved, &Default::default());
    let new = RunCtx {
        doc: &moved,
        eval: &after,
    };
    match resolve_with_prior(new, prior, below, &frag) {
        Resolution::Failed(f) => match f.error {
            ResolveError::Vanished { last_good, .. } => {
                let ghost = last_good.expect("a last-good tombstone");
                println!("P1b tombstone patch = {:?}; prior resolved at {:?}", ghost.patch, was);
                assert_eq!(
                    (ghost.patch.node, ghost.patch.entity),
                    (was.node, was.entity),
                    "the ghost is the lower half's fragment"
                );
            }
            other => panic!("vanished, not {other:?}"),
        },
        other => panic!("the lower read lost it: {other:?}"),
    }
}

/// P2: re-attachment offers are (read, name) pairs. A name both halves
/// carry is held by two reads; an offer naming one read only leaves the
/// other half unrepairable from the offers.
#[test]
fn p2_rebind_offers_name_every_read_holding_a_shared_split_name() {
    let (doc, ext, _, split) = step_split(1.0);
    let ev = fixture::run(&doc, &Default::default());
    let above = doc.output(split, 0).unwrap();
    let below = doc.output(split, 1).unwrap();
    let (ta, tb) = (ev.table(above).unwrap(), ev.table(below).unwrap());
    // A split fragment of an extrude face, held by both halves.
    let (frag, parent) = ta
        .iter()
        .filter_map(|(n, _)| match n.path.last() {
            Some(RoleSeg::SplitFragment { parent })
                if n.kind == EntityKind::Face && tb.lookup(n).is_some() =>
            {
                Some((n.clone(), (**parent).clone()))
            }
            _ => None,
        })
        .next()
        .expect("a wall fragment both halves hold");
    println!("P2 shared fragment: {frag:?}");
    let ext_read = fixture::out(&doc, ext);
    let (painted, _) = fixture::step(
        doc.clone(),
        DocEdit::SetAppearance {
            body: ext_read,
            name: parent.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    let ev = fixture::run(&painted, &Default::default());
    let offers = editor_core::appearance_rebind_suggestions(painted.appearance(), &ev);
    let mine = offers
        .iter()
        .find(|(k, _)| k.name == parent)
        .map(|(_, v)| v.clone())
        .expect("the key has an offer row");
    let reads: Vec<_> = mine.iter().filter(|o| o.name == frag).map(|o| o.at).collect();
    println!("P2 reads offered for the shared fragment: {reads:?} (above {above:?}, below {below:?})");
    assert!(
        reads.contains(&above) && reads.contains(&below),
        "both halves hold the fragment, so both reads are offered: {reads:?}"
    );
}

/// P3: `AppearanceBodyNotARead` — "a read that is not a body output".
/// A datum plane's output is an output, but holds no body.
#[test]
fn p3_set_appearance_refuses_a_read_that_holds_no_body() {
    let (doc, ext, tool, _) = step_split(1.0);
    let face = fixture::fname(ext, fixture::wall(&doc, ext, 3));
    let plane_read = doc.output(tool, 0).expect("the plane's output");
    let refused = editor_core::apply(
        &doc,
        &DocEdit::SetAppearance {
            body: plane_read,
            name: face,
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("P3 outcome is_err = {}", refused.is_err());
    assert!(
        matches!(
            refused,
            Err(editor_core::EditError::AppearanceBodyNotARead { .. })
        ),
        "a datum plane's read holds no body"
    );
}

/// P4: one `At::Node` rule. A revolve defines a body and an axis; the
/// name doors read it alone at its one named output, and the Python
/// `resolve` door reads it through `Doc::read_of_node`.
#[test]
fn p4_a_revolve_named_alone_reads_the_same_in_every_door() {
    let doc = ProfileDoc::empty_derived("r1_dm4_rev", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![fixture::square(2.0, 0.0, 0.5)],
    );
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis_origin: [len(0.0), len(0.0)],
            axis_direction: [scl(0.0), scl(1.0)],
            angle: fixture::ang(std::f64::consts::TAU),
        },
    );
    let ev = fixture::run(&doc, &Default::default());
    let kernel = ev.at_var(revolve);
    let doc_side = doc.read_of_node(revolve);
    println!("P4 at_var = {kernel:?}, read_of_node = {doc_side:?}");
    assert_eq!(kernel, doc_side, "one rule for a node named alone");
}

/// P6: resolving in a read the document no longer defines.
#[test]
fn p6_resolve_in_a_gone_read() {
    let (doc, ext, _, split) = step_split(1.0);
    let face = fixture::fname(ext, fixture::wall(&doc, ext, 3));
    let below = doc.output(split, 1).unwrap();
    let (gone, _) = fixture::step(doc.clone(), DocEdit::DeleteNode { id: split });
    let ev = fixture::run(&gone, &Default::default());
    let r = resolve(
        RunCtx {
            doc: &gone,
            eval: &ev,
        },
        below,
        &face,
    );
    println!("P6 resolve in a deleted split's read: {r:?}");
}

/// P7: the ambiguous appearance enrichment reads the entry's own read.
#[test]
fn p7_section_face_tables_are_per_half() {
    let (doc, _, _, split) = step_split(1.0);
    let ev = fixture::run(&doc, &Default::default());
    let section = StableName {
        kind: EntityKind::Face,
        node: split,
        path: vec![RoleSeg::SectionFace { section: 0 }],
    };
    for port in [0u8, 1] {
        let read = doc.output(split, port).unwrap();
        assert!(matches!(
            ev.table(read).unwrap().lookup(&section),
            Some(Entry::Unique(_))
        ));
    }
}

/// P8: the two new paths no PR row reaches — a delete of the operation
/// defining a key's body read strands it `Took::Body`, and a read whose
/// operation is gone refuses `AppearanceBodyNotARead`.
#[test]
fn p8_took_body_and_body_not_a_read_fire() {
    let (doc, ext, _, split) = step_split(1.0);
    let riser = fixture::fname(ext, fixture::wall(&doc, ext, 3));
    let above = doc.output(split, 0).unwrap();
    let (painted, _) = fixture::step(
        doc.clone(),
        DocEdit::SetAppearance {
            body: above,
            name: riser.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    let applied = editor_core::apply(
        &painted,
        &DocEdit::DeleteNode { id: split },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a delete strands, it does not refuse");
    let said: Vec<String> = applied
        .maintenance
        .iter()
        .map(|m| format!("{m:?}"))
        .collect();
    println!("P8 maintenance: {said:?}");
    assert!(said.iter().any(|m| m.contains("StrandedAppearance") && m.contains("Body")));
    let refused = editor_core::apply(
        &applied.doc,
        &DocEdit::SetAppearance {
            body: above,
            name: riser,
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    assert!(matches!(
        refused,
        Err(editor_core::EditError::AppearanceBodyNotARead { .. })
    ));
}
