//! ASM-ROOTS acceptance — explicit product roots (spec rows 1–6, each
//! an executable falsifier).
//!
//! The unit under test is the ordered root list (A10): its invariants
//! (D-2), its automatic maintenance (D-3), the whole-document product
//! gather (D-4), and the v6 clean break (D-1).
//!
//! Undo note (row 1e): this layer has no undo STACK — `apply` is pure
//! and undo is keeping prior values (`edit.rs` module docs). "Undone
//! restores the prior list exactly" is therefore checked the way the
//! layer actually offers it: the pre-edit value's root list, held
//! across the edit, is unchanged and `bit_eq`-identical.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, Doc, DocEdit, EvalOptions, Evaluation, Node, PatternKind, PersistError,
    ProductError, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, RootFault, SnapshotError,
    content_pin, evaluate, load, save,
};
use fixture::{desc, insert, len, scl, square, step};
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

/// A unit square profile centered at `cx` on the z = 0 plane.
fn block(doc: ProfileDoc, cx: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(cx, 0.0, 0.5)],
        )),
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, profile, extrude)
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass properties")
        .volume
}

// ---- Row 1: automatic maintenance (D-3) ----

/// Row 1a — an insert consuming no current root APPENDS. Two blocks
/// authored in sequence leave both tips rooted, in insertion order.
#[test]
fn row1a_no_consumer_insert_appends() {
    // A lone profile is a sink, so it roots itself; its extrude then
    // consumes it (row 1b's rule) and takes the slot.
    let (doc, p0) = insert(
        ProfileDoc::empty_derived("asm-roots-1a", Tol::witness()),
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, 0.5)],
        )),
    );
    assert_eq!(doc.roots(), &[p0][..], "a lone profile roots itself");
    let (doc, e0) = insert(
        doc,
        Node::Extrude {
            profile: p0,
            distance: len(1.0),
        },
    );
    assert_eq!(
        doc.roots(),
        &[e0][..],
        "the extrude took the profile's slot"
    );
    assert!(
        !run(&doc).order.is_empty(),
        "the exemplar evaluates (anti-vacuity for the rows below)"
    );
    let (doc, _p1, e1) = block(doc, 5.0);
    assert_eq!(doc.roots(), &[e0, e1][..], "both tips, in insertion order");
}

/// Row 1b — an insert consuming current roots REPLACES them at the
/// EARLIEST consumed position (tip transfer). A boolean over the
/// second and third of three tips lands where the second was.
#[test]
fn row1b_consuming_insert_replaces_at_earliest_position() {
    let (doc, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-1b", Tol::witness()),
        0.0,
    );
    let (doc, _, b) = block(doc, 5.0);
    let (doc, _, c) = block(doc, 10.0);
    assert_eq!(doc.roots(), &[a, b, c][..]);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            a: b,
            b: c,
            op: editor_core::BooleanOp::Union,
            declare: None,
        },
    );
    assert_eq!(
        doc.roots(),
        &[a, u][..],
        "the union takes b's slot; c drops out"
    );
}

/// Row 1c — deleting a root re-roots the direct inputs its departure
/// turned into sinks, in DOCUMENT order, expanding at the deleted
/// root's position.
#[test]
fn row1c_root_delete_rereoots_orphans_in_document_order() {
    let (doc, _, keep) = block(
        ProfileDoc::empty_derived("asm-roots-1c", Tol::witness()),
        0.0,
    );
    let (doc, pa, a) = block(doc, 5.0);
    let (doc, pb, b) = block(doc, 10.0);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            a,
            b,
            op: editor_core::BooleanOp::Union,
            declare: None,
        },
    );
    assert_eq!(doc.roots(), &[keep, u][..]);
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: u });
    assert_eq!(
        doc.roots(),
        &[keep, a, b][..],
        "the union's two operands re-root at its position, in document order"
    );
    // And one more level down: deleting `a` re-roots its profile.
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: a });
    assert_eq!(doc.roots(), &[keep, pa, b][..]);
    assert!(doc.node(pb).is_some(), "pb is untouched");
}

/// Row 1d — `SetRoots` overrides the defaults: it is the total
/// designate/undesignate door, and it can REORDER without changing
/// the set.
#[test]
fn row1d_set_roots_overrides() {
    let (doc, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-1d", Tol::witness()),
        0.0,
    );
    let (doc, _, b) = block(doc, 5.0);
    let (doc, record) = step(doc.clone(), DocEdit::SetRoots { roots: vec![b, a] });
    assert_eq!(doc.roots(), &[b, a][..]);
    assert!(record.is_none(), "SetRoots mints no node");
}

/// Row 1e — undo restores the prior list exactly (module docs: undo
/// at this layer is keeping the prior VALUE, and `apply` is pure).
#[test]
fn row1e_undo_restores_the_prior_root_list() {
    let (before, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-1e", Tol::witness()),
        0.0,
    );
    let (before, _, b) = block(before, 5.0);
    let prior_roots = before.roots().to_vec();
    for edit in [
        DocEdit::SetRoots { roots: vec![b, a] },
        DocEdit::InsertNode {
            node: Node::Boolean {
                a,
                b,
                op: editor_core::BooleanOp::Union,
                declare: None,
            },
        },
        DocEdit::DeleteNode { id: b },
    ] {
        let after = before
            .apply(&edit, Tol::witness())
            .expect("the edit applies")
            .doc;
        assert_ne!(after.roots(), &prior_roots[..], "the edit moved the list");
        assert_eq!(
            before.roots(),
            &prior_roots[..],
            "the pre-edit value's list is untouched — that IS the undo"
        );
        assert!(before.bit_eq(&before.clone()), "the prior value is intact");
    }
}

// ---- Row 2: invariants (D-2) ----

/// Row 2a — ancestor-freedom refuses and NAMES BOTH nodes.
#[test]
fn row2a_ancestor_freedom_names_both() {
    let (doc, profile, extrude) = block(
        ProfileDoc::empty_derived("asm-roots-2a", Tol::witness()),
        0.0,
    );
    let err = doc
        .apply(
            &DocEdit::SetRoots {
                roots: vec![profile, extrude],
            },
            Tol::witness(),
        )
        .expect_err("an ancestor pair must refuse");
    assert_eq!(
        err,
        editor_core::EditError::Roots(RootFault::Ancestor {
            ancestor: profile,
            descendant: extrude,
        })
    );
    // The prose names both too (the bindings' message surface).
    let text = format!("{err}");
    assert!(
        text.contains(&format!("{}", profile.0)) && text.contains(&format!("{}", extrude.0)),
        "both nodes must be named: {text}"
    );
}

/// Row 2b — coverage refuses on a HAND-CRAFTED SAVE TEXT: the load
/// door is the backstop even when no edit could have produced the
/// state. The craft drops one of two tips from the root list, which
/// strands that tip's whole chain.
#[test]
fn row2b_coverage_refuses_on_a_crafted_save() {
    let (doc, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-2b", Tol::witness()),
        0.0,
    );
    let (doc, _, b) = block(doc, 5.0);
    let text = save(&doc, &[], Tol::witness()).expect("the honest document saves");
    let honest = format!("\"roots\": [\n      {},\n      {}\n    ]", a.0, b.0);
    assert!(
        text.contains(&honest),
        "the save's root list must be the two tips, in order"
    );
    let crafted = text.replace(&honest, &format!("\"roots\": [\n      {}\n    ]", a.0));
    match load(&crafted, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::Roots(RootFault::Uncovered { node }))) => {
            assert!(
                node != a,
                "the stranded chain is b's, not a's (got node {})",
                node.0
            );
        }
        other => panic!("a crafted uncovered document must refuse, got {other:?}"),
    }
}

/// Row 2c — a duplicate entry refuses (the list is a SET plus an
/// order, and a repeated root would gather one body twice).
#[test]
fn row2c_duplicate_entry_refuses() {
    let (doc, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-2c", Tol::witness()),
        0.0,
    );
    let err = doc
        .apply(&DocEdit::SetRoots { roots: vec![a, a] }, Tol::witness())
        .expect_err("a duplicate must refuse");
    assert_eq!(
        err,
        editor_core::EditError::Roots(RootFault::Duplicate { root: a })
    );
    // And a dead entry refuses too.
    let ghost = RecipeNodeId(9_999);
    assert_eq!(
        doc.apply(&DocEdit::SetRoots { roots: vec![ghost] }, Tol::witness())
            .expect_err("a dead entry must refuse"),
        editor_core::EditError::Roots(RootFault::NotLive { root: ghost })
    );
}

// ---- Row 3: the gather (D-4) ----

/// Row 3a — two disjoint extrudes gather into a 2-solid product whose
/// volume is the parts' sum.
#[test]
fn row3a_two_disjoint_extrudes_gather_additively() {
    let (doc, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-3a", Tol::witness()),
        0.0,
    );
    let (doc, _, b) = block(doc, 5.0);
    let ev = run(&doc);
    let product = editor_core::product(&doc, &ev, Tol::witness()).expect("the product gathers");
    assert_eq!(product.solids().count(), 2, "one solid per root");
    let part = |id| match &ev.value(id).expect("the tip evaluated").payload {
        editor_core::ValuePayload::Body(body) => volume(body),
        other => panic!("an extrude is a body, got {}", other.kind_name()),
    };
    let sum = part(a) + part(b);
    assert!(
        (volume(&product) - sum).abs() < 1e-9,
        "product volume {} vs the parts' sum {sum}",
        volume(&product)
    );
}

/// Row 3b — an `Instances` root materializes into N placed solids of
/// the ONE product body (C1's disposition), with the pattern's
/// provenance indices and `Instance(i)` names intact.
#[test]
fn row3b_pattern_root_gathers_n_solids_with_provenance() {
    let (doc, _, extrude) = block(
        ProfileDoc::empty_derived("asm-roots-3b", Tol::witness()),
        0.0,
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: extrude,
            count: editor_core::Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    assert_eq!(doc.roots(), &[pattern][..], "the pattern is the only tip");
    let ev = run(&doc);
    let product = editor_core::product(&doc, &ev, Tol::witness()).expect("the product gathers");
    assert_eq!(product.solids().count(), 3, "N placed solids, no boolean");

    // Provenance: instances 1..N carry `Placed { node: pattern, i }`
    // (instance 0 is the master verbatim, by the pattern's own rule),
    // and the graft transplants those records untouched.
    let placed: std::collections::BTreeSet<u32> = product
        .surfaces()
        .filter_map(|(k, _)| product.surface_source(k))
        .filter_map(|s| match &s.expr {
            topo::SourceExpr::Placed { node, instance, .. } if *node == pattern.0 => {
                Some(*instance)
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        placed,
        [1, 2].into_iter().collect(),
        "each placed instance's index survives into the product"
    );

    // Names: the pattern's `Instance(i)` wrapping is the evaluation's,
    // and the gather is table-neutral (it takes `&Evaluation` and
    // returns a body) — so every index is still addressable after it.
    let table = &ev.value(pattern).expect("the pattern evaluated").name_table;
    let wrapped: std::collections::BTreeSet<u32> = table
        .iter()
        .filter_map(|(name, _)| match name.path.first() {
            Some(RoleSeg::Instance { i, .. }) => Some(*i),
            _ => None,
        })
        .collect();
    assert_eq!(wrapped, [0, 1, 2].into_iter().collect());
}

/// Row 3c — a `Split` root contributes BOTH pieces (a mold document
/// wants both halves).
#[test]
fn row3c_split_root_gathers_both_pieces() {
    let (doc, _, extrude) = block(
        ProfileDoc::empty_derived("asm-roots-3c", Tol::witness()),
        0.0,
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: extrude,
            tool: plane,
        },
    );
    assert_eq!(doc.roots(), &[split][..], "the split consumed both inputs");
    let ev = run(&doc);
    let product = editor_core::product(&doc, &ev, Tol::witness()).expect("the product gathers");
    assert_eq!(product.solids().count(), 2, "both pieces are present");
    assert!(
        (volume(&product) - 1.0).abs() < 1e-9,
        "the two halves recover the whole block: {}",
        volume(&product)
    );
}

// ---- Row 4: no body-denoting root ----

/// Row 4 — a profile-only document has no body product, typed.
#[test]
fn row4_no_body_roots_refuses_typed() {
    let (doc, profile) = insert(
        ProfileDoc::empty_derived("asm-roots-4", Tol::witness()),
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, 0.5)],
        )),
    );
    assert_eq!(doc.roots(), &[profile][..]);
    let ev = run(&doc);
    assert!(
        ev.value(profile).is_some(),
        "the profile itself evaluated — the refusal is about DENOTATION, not health"
    );
    match editor_core::product(&doc, &ev, Tol::witness()) {
        Err(ProductError::NoBodyRoots) => {}
        other => panic!("a profile-only document must refuse NoBodyRoots, got {other:?}"),
    }
}

// ---- Row 5: the pin and D9 ordering ----

/// Row 5a — a root REORDER moves the content pin: order is semantic
/// because it is product-solid order (D-1's include-by-default).
#[test]
fn row5a_root_reorder_moves_the_pin() {
    let (doc, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-5a", Tol::witness()),
        0.0,
    );
    let (doc, _, b) = block(doc, 5.0);
    let (reordered, _) = step(doc.clone(), DocEdit::SetRoots { roots: vec![b, a] });
    assert_ne!(
        content_pin(&doc, Tol::witness()).unwrap(),
        content_pin(&reordered, Tol::witness()).unwrap(),
        "a reorder is a different version"
    );
    // And it is really only the order that changed.
    let mut lhs = doc.roots().to_vec();
    let mut rhs = reordered.roots().to_vec();
    lhs.sort_unstable();
    rhs.sort_unstable();
    assert_eq!(lhs, rhs, "same set, different order");
}

/// Row 5b — a root-neutral edit leaves the product's solid ORDER
/// stable across two evaluations (D9): the gather is a pure function
/// of (root list, evaluation).
#[test]
fn row5b_root_neutral_edits_keep_the_product_order_stable() {
    let (doc, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-5b", Tol::witness()),
        0.0,
    );
    let (doc, _, _b) = block(doc, 5.0);
    let roots_before = doc.roots().to_vec();
    let first = editor_core::product(&doc, &run(&doc), Tol::witness()).expect("gather 1");
    // A continuous edit on the first block's depth: the root list is
    // untouched, so the product's solid order must be too.
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: a,
            slot: editor_core::SlotId::Distance,
            expr: len(2.0),
        },
    );
    assert_eq!(doc.roots(), &roots_before[..], "the edit was root-neutral");
    let second = editor_core::product(&doc, &run(&doc), Tol::witness()).expect("gather 2");
    // Solid order IS gather order, read off the transplanted
    // provenance: solid k's faces were minted by root k's node.
    let order = |body: &topo::Body<f64>| -> Vec<Vec<u64>> {
        body.solids()
            .map(|(key, _)| minting_nodes(body, key))
            .collect()
    };
    assert_eq!(order(&first), vec![vec![a.0], vec![_b.0]]);
    assert_eq!(
        order(&second),
        order(&first),
        "a root-neutral edit leaves the product's solid order alone"
    );
    // …and a root REORDER is exactly what moves it.
    let (swapped, _) = step(doc, DocEdit::SetRoots { roots: vec![_b, a] });
    let third = editor_core::product(&swapped, &run(&swapped), Tol::witness()).expect("gather 3");
    assert_eq!(order(&third), vec![vec![_b.0], vec![a.0]]);
}

/// The recipe nodes that minted a solid's face carriers — the
/// provenance a disjoint graft transplants verbatim, and therefore the
/// honest read of "which root contributed this solid".
fn minting_nodes(body: &topo::Body<f64>, solid: topo::SolidKey) -> Vec<u64> {
    let mut out = std::collections::BTreeSet::new();
    if let Some(s) = body.get_solid(solid) {
        for &shell in &s.shells {
            let Some(sh) = body.get_shell(shell) else {
                continue;
            };
            for &face in &sh.faces {
                if let Some(f) = body.get_face(face)
                    && let Some(src) = body.surface_source(f.surface)
                {
                    out.insert(src.node);
                }
            }
        }
    }
    out.into_iter().collect()
}

// ---- Row 6: the persisted root list ----

/// Row 6b — a document's save text is byte-stable across two saves
/// (the fixtures' bless pipeline is a function, not a nondeterministic
/// dump), and it round-trips through load with the root list intact.
#[test]
fn row6b_saves_are_byte_stable_and_roots_round_trip() {
    let (doc, _, a) = block(
        ProfileDoc::empty_derived("asm-roots-6b", Tol::witness()),
        0.0,
    );
    let (doc, _, b) = block(doc, 5.0);
    let (doc, _) = step(doc, DocEdit::SetRoots { roots: vec![b, a] });
    let once = save(&doc, &[], Tol::witness()).expect("saves");
    let twice = save(&doc, &[], Tol::witness()).expect("saves again");
    assert_eq!(once, twice, "two blesses, one byte string");
    let loaded = load(&once, Tol::witness()).expect("the saved text loads");
    assert_eq!(
        loaded.doc.roots(),
        &[b, a][..],
        "the order survives the file"
    );
    assert!(loaded.doc.bit_eq(&doc));
}

/// Row 6c — replay re-derives the root list exactly: maintenance is a
/// pure function of (document, edit), so the edit log alone rebuilds
/// it (D9 determinism, and what makes the save's log honest).
#[test]
fn row6c_replay_rebuilds_the_root_list() {
    let mut log: Vec<DocEdit<ProfileProgram>> = Vec::new();
    let id = editor_core::DocumentId::derive("asm-roots-6c");
    let mut doc: Doc<ProfileProgram> = Doc::empty(id, Tol::witness());
    for cx in [0.0, 5.0] {
        for node in [
            Node::Profile(desc(
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                vec![square(cx, 0.0, 0.5)],
            )),
            Node::Extrude {
                profile: RecipeNodeId(doc.len() as u64),
                distance: len(1.0),
            },
        ] {
            let edit = DocEdit::InsertNode { node };
            doc = doc.apply(&edit, Tol::witness()).expect("insert").doc;
            log.push(edit);
        }
    }
    let swap = DocEdit::SetRoots {
        roots: doc.roots().iter().rev().copied().collect(),
    };
    doc = doc.apply(&swap, Tol::witness()).expect("set roots").doc;
    log.push(swap);
    let replayed = Doc::replay(id, &log, Tol::witness()).expect("the log replays");
    assert_eq!(replayed.roots(), doc.roots());
    assert!(replayed.bit_eq(&doc));
}
