//! ASM-2B acceptance — multi-solid referenced products.
//!
//! ASM-2A shipped instantiation with one deliberate restriction: a
//! referenced document whose product held N ≠ 1 solids refused. This
//! suite is the lifted door: a part is its product, however many solids
//! that product holds, and the instantiate path takes all N through
//! `transform_rigid` + the keyed graft as ONE unit.
//!
//! The kernel half (stub resolver, as ASM-2A's D-3 promises); the
//! workspace/two-process half lives in `pncad`'s own suite.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, DocEdit, DocRef, DocumentId, EntityKind, EvalOptions, Evaluation, Frame, Node,
    PartResolver, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, StableName,
    content_pin, evaluate, load, product, save,
};
use fixture::{desc, insert, len, on_frame, square, step};
use geom_core::Tol;

// ---- The stub store (ASM-2A's, verbatim in behaviour) ----

/// A resolver over an in-memory map that verifies the pin exactly as
/// the document layer's does — the stub has no FILES, not no gate.
#[derive(Debug, Default)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        let found = content_pin(doc, Tol::witness()).expect("the pin computes");
        if found != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

fn with_resolver(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness())
}

/// A one-solid part: a unit square extruded 1 tall, centered at `cx`.
fn part(label: &str, cx: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

/// An assembly instantiating `refs` in order, each a root, the i-th
/// displaced `spacing * i` along +x so the solids stay disjoint.
fn assembly(label: &str, refs: &[DocRef], spacing: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for (i, r) in refs.iter().enumerate() {
        let (next, id) = insert(doc, Node::instantiate_part(*r));
        doc = next;
        if i > 0 {
            #[allow(clippy::cast_precision_loss)]
            let dx = spacing * i as f64;
            let (next, _) = step(
                doc,
                DocEdit::SetPlacement {
                    node: id,
                    frame: Frame::translation([dx, 0.0, 0.0]),
                },
            );
            doc = next;
        }
        ids.push(id);
    }
    (doc, ids)
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass properties")
        .volume
}

/// Every x of a body's vertices in ascending total order — the
/// rigid-motion probe: a translation along +x preserves that order, so
/// comparing the two sorted lists pairs each vertex with its own moved
/// self, and every difference must be the translation exactly.
fn xs(body: &topo::Body<f64>) -> Vec<f64> {
    let mut v: Vec<f64> = body
        .vertices()
        .filter_map(|(_, e)| body.get_point(e.point))
        .map(|p| p.x)
        .collect();
    v.sort_by(f64::total_cmp);
    v
}

fn names_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<StableName> {
    ev.value(node)
        .expect("the node evaluated")
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .collect()
}

// ---- Row 2: the doubly-nested sub-assembly ----

/// The 2B fixture family: part P (one solid), sub-assembly B (two
/// instances of P), assembly A (two instances of B). A's product holds
/// FOUR solids, and every name in it is doubly `InPart`-wrapped.
fn nested_fixture() -> (StubStore, ProfileDoc, ProfileDoc, DocRef, Vec<RecipeNodeId>) {
    let mut store = StubStore::default();
    let p = store.insert(part("asm2b-p", 0.0), Tol::witness());
    let (b_doc, _) = assembly("asm2b-b", &[p, p], 3.0);
    let b = store.insert(b_doc.clone(), Tol::witness());
    let (a_doc, a_ids) = assembly("asm2b-a", &[b, b], 100.0);
    (store, a_doc, b_doc, b, a_ids)
}

/// Row 2 (kernel half) — a sub-assembly instantiates: A's product is
/// four solids, its volume bit-exactly 4× the part's, and two
/// evaluations of the same recipe agree bit for bit (D9).
#[test]
fn row2_a_sub_assembly_instantiates_into_four_solids() {
    let (store, a_doc, _, _, a_ids) = nested_fixture();
    let opts = with_resolver(store);

    let ev = run(&a_doc, &opts);
    let body = product(&a_doc, &ev, Tol::witness()).expect("the product gathers");
    assert_eq!(body.solids().count(), 4, "two sub-assemblies of two parts");

    let p_doc = part("asm2b-p", 0.0);
    let p_ev = run(&p_doc, &EvalOptions::default());
    let p_body = product(&p_doc, &p_ev, Tol::witness()).expect("the part's product");
    assert_eq!(
        volume(&body).to_bits(),
        (4.0 * volume(&p_body)).to_bits(),
        "four copies, bit-exactly"
    );

    // The instance value itself is the multi-solid body — the lift's
    // whole content: one rigid map over N solids, not N maps.
    let solids_of = |node: RecipeNodeId| match ev.value(node).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => b.solids().count(),
        other => panic!("an instance's value is a body, got {other:?}"),
    };
    assert_eq!(solids_of(a_ids[0]), 2, "each instance carries both solids");
    assert_eq!(solids_of(a_ids[1]), 2);

    // The sub-assembly evaluates ONCE for both instances, and the part
    // once inside it: the memo crosses the second seam too.
    assert_eq!(ev.part_evaluations, 2, "one B evaluation, one P inside it");

    let ev2 = run(&a_doc, &opts);
    let body2 = product(&a_doc, &ev2, Tol::witness()).expect("the product gathers again");
    assert_eq!(
        volume(&body).to_bits(),
        volume(&body2).to_bits(),
        "two evaluations agree bit for bit (D9)"
    );
    assert_eq!(
        xs(&body).iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
        xs(&body2).iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
        "and the positions with them"
    );
}

// ---- Row 3: names across all N solids, doubly wrapped ----

/// Row 3 — every name of a doubly-nested instance is
/// `InPart(InPart(part-local))` minted at the OUTER instantiate node;
/// the four copies' name sets are pairwise disjoint; and each name
/// resolves to its own copy (the cross-wiring probe).
#[test]
fn row3_doubly_wrapped_names_are_distinct_and_resolve_to_their_own_copy() {
    let (store, a_doc, _, _, a_ids) = nested_fixture();
    let opts = with_resolver(store);
    let ev = run(&a_doc, &opts);

    let a = names_of(&ev, a_ids[0]);
    let b = names_of(&ev, a_ids[1]);
    assert!(!a.is_empty(), "an instance has names at all");
    assert_eq!(a.len(), b.len(), "the two instances name the same entities");
    for name in &a {
        assert!(!b.contains(name), "no name is shared between instances");
    }

    // Shape: the body name is the instance's own output; every other
    // name wraps a name that itself wraps a part-local one — the
    // sub-assembly's instantiate node is the inner minter.
    let mut inner_nodes = std::collections::BTreeSet::new();
    for (names, node) in [(&a, a_ids[0]), (&b, a_ids[1])] {
        for name in names {
            assert_eq!(name.node, node, "names are minted at the outer instance");
            match &name.path[..] {
                [RoleSeg::InPart { of }] => {
                    assert_eq!(of.kind, name.kind, "the wrapper preserves the kind");
                    match &of.path[..] {
                        [RoleSeg::InPart { of: inner }] => {
                            assert_eq!(inner.kind, name.kind);
                            inner_nodes.insert(of.node);
                        }
                        other => panic!("expected a doubly wrapped name, got {other:?}"),
                    }
                }
                [RoleSeg::OutputBody] => assert_eq!(name.kind, EntityKind::Body),
                other => panic!("unexpected instance role path {other:?}"),
            }
        }
    }
    assert_eq!(
        inner_nodes.len(),
        2,
        "the two solids come from the sub-assembly's two instantiate nodes"
    );

    // Cross-wiring probe: one part-local vertex name, wrapped under
    // each of the four (outer, inner) pairs, lands in FOUR distinct
    // positions — one per copy.
    let vertex = a
        .iter()
        .find(|n| n.kind == EntityKind::Vertex)
        .expect("a vertex name");
    let RoleSeg::InPart { of: inner } = &vertex.path[0] else {
        panic!("wrapped");
    };
    let mut seen = std::collections::BTreeSet::new();
    for outer in &a_ids {
        for inner_node in &inner_nodes {
            let name = StableName {
                kind: vertex.kind,
                node: *outer,
                path: vec![RoleSeg::InPart {
                    of: Box::new(StableName {
                        kind: inner.kind,
                        node: *inner_node,
                        path: inner.path.clone(),
                    }),
                }],
            };
            let p = editor_core::vertex_position(&ev, *outer, &name).expect("resolves");
            assert!(seen.insert(p.x.to_bits()), "each copy has its own position");
            // A name minted under one instance is not the other's.
            let other = a_ids.iter().find(|n| **n != *outer).expect("the other");
            assert!(
                editor_core::vertex_position(&ev, *other, &name).is_err(),
                "identity is per instance, not per part"
            );
        }
    }
    assert_eq!(seen.len(), 4, "four copies, four distinct positions");
}

/// Row 3 (persistence half) — a doubly-wrapped name survives the wire
/// verbatim as an appearance key.
#[test]
fn row3_doubly_wrapped_names_round_trip_persistence() {
    let (store, a_doc, _, _, a_ids) = nested_fixture();
    let opts = with_resolver(store);
    let ev = run(&a_doc, &opts);
    let name = names_of(&ev, a_ids[0])
        .into_iter()
        .find(|n| n.kind == EntityKind::Face)
        .expect("a face name");

    let (doc, _) = step(
        a_doc,
        DocEdit::SetAppearance {
            name: name.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8 {
                r: 9,
                g: 8,
                b: 7,
                a: 255,
            }),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads").doc;
    assert!(
        loaded.appearance().keys().any(|k| *k == name),
        "a doubly-wrapped name survives the wire verbatim"
    );
    assert!(loaded.bit_eq(&doc), "the whole document round-trips");
}

// ---- Row 4: placements over a multi-solid instance ----

/// Row 4 — `SetPlacement` on a multi-solid instance moves ALL of its
/// solids rigidly (every vertex x by exactly the translation, volume
/// unchanged bit for bit), and the document's content pin moves with
/// the edit.
#[test]
fn row4_a_placement_moves_every_solid_of_a_multi_solid_instance() {
    let mut store = StubStore::default();
    let p = store.insert(part("asm2b-r4-p", 0.0), Tol::witness());
    let (b_doc, _) = assembly("asm2b-r4-b", &[p, p], 3.0);
    let b = store.insert(b_doc, Tol::witness());
    let (doc, ids) = assembly("asm2b-r4-a", &[b], 0.0);
    let opts = with_resolver(store);

    let before = run(&doc, &opts);
    let body0 = product(&doc, &before, Tol::witness()).expect("gathers");
    let pin0 = content_pin(&doc, Tol::witness()).expect("pins");

    let (moved, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([7.0, 0.0, 0.0]),
        },
    );
    assert_ne!(
        content_pin(&moved, Tol::witness()).expect("pins"),
        pin0,
        "the pin moves"
    );

    let after = run(&moved, &opts);
    let body1 = product(&moved, &after, Tol::witness()).expect("gathers");
    assert_eq!(body1.solids().count(), 2, "both solids came along");
    assert_eq!(
        volume(&body0).to_bits(),
        volume(&body1).to_bits(),
        "a rigid motion changes no volume"
    );
    let (x0, x1) = (xs(&body0), xs(&body1));
    assert_eq!(x0.len(), x1.len(), "same vertices, moved");
    for (a, b) in x0.iter().zip(x1.iter()) {
        assert_eq!((b - a).to_bits(), 7.0_f64.to_bits(), "moved by exactly 7");
    }
}

// ---- Row 5: the single-solid path is bit-identical ----

/// FNV-1a 64 over an evaluation's name tables in node order, arena
/// keys included — the replay-identity digest (M4 PR 3's shape).
fn digest(ev: &Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for byte in s.bytes() {
            h ^= u64::from(byte);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(&format!("#{id:?}"));
        if let Some(v) = ev.value(*id) {
            for (n, e) in v.name_table.iter() {
                feed(&format!("{n:?}={e:?};"));
            }
        }
    }
    h
}

/// The pinned single-solid assembly digests, CAPTURED ON MAIN BEFORE
/// this unit's lift and unchanged by it. The invariant: lifting the
/// multi-solid refusal is a widening — the single-solid path takes the
/// same code, allocates the same arena keys, and yields the same
/// product bits (D9, D-3's bit-identity row). A drift here is a
/// regression in the shipped path, never a re-pin.
///
/// Both quantities are ε-INVARIANT, which is what makes them pinnable
/// under CI's eps sweep. The saved BYTES are not: an assembly's text
/// embeds its references' content pins, a part's pin covers that
/// document's own recorded ε, and CI's eps rows sweep the ambient ε by
/// design — so the bytes legitimately move per run. Persistence is
/// pinned byte-wise where it is ε-free (`pncad`'s `plate_param`
/// fixture) and observed here as a round-trip.
const SINGLE_SOLID_NAMES_DIGEST: u64 = 18_302_139_915_801_724_049;
const SINGLE_SOLID_VOLUME_BITS: u64 = 4_611_686_018_427_387_904; // 2.0

#[test]
fn row5_the_single_solid_path_is_bit_identical_across_the_lift() {
    let mut store = StubStore::default();
    let p = store.insert(part("asm2b-r5-p", 0.0), Tol::witness());
    let (doc, _) = assembly("asm2b-r5-a", &[p, p], 5.0);
    let opts = with_resolver(store);

    let ev = run(&doc, &opts);
    let body = product(&doc, &ev, Tol::witness()).expect("gathers");
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads").doc;
    assert!(loaded.bit_eq(&doc), "the assembly round-trips");

    assert_eq!(
        (digest(&ev), volume(&body).to_bits(), body.solids().count()),
        (SINGLE_SOLID_NAMES_DIGEST, SINGLE_SOLID_VOLUME_BITS, 2),
        "the single-solid path is bit-identical across the lift"
    );
}

// ---- The at-rest gate over instances: what it does and does not see ----

/// A PRE-EXISTING gap, probed here so the lift is not blamed for it:
/// two instances placed so their solids INTERPENETRATE gather into a
/// product without complaint — `topo::validate_geometric` does not do
/// solid-solid interference detection (tier 3 is a per-edge/per-face
/// LOCAL battery; detection is planned as tier-3′ census growth, issue
/// #382), and the gather's module docs state exactly that boundary.
///
/// The row is written both ways on purpose: the SINGLE-solid case is
/// ASM-2A behaviour, untouched by this unit, and the MULTI-solid case
/// is the same code reaching the same decision — the lift widened what
/// may be grafted, not what counts as valid. Should the gate ever learn
/// interference, both assertions flip together, here.
#[test]
fn the_at_rest_gate_does_not_see_instance_interference_single_or_multi() {
    let mut store = StubStore::default();
    let p = store.insert(part("asm2b-ov-p", 0.0), Tol::witness());
    // ASM-2A's shape: two copies of a one-solid part, 0.25 apart.
    let (single, _) = assembly("asm2b-ov-single", &[p, p], 0.25);
    // 2B's shape: two copies of a two-solid sub-assembly, placed so a
    // solid of each lands inside a solid of the other.
    let (b_doc, _) = assembly("asm2b-ov-b", &[p, p], 3.0);
    let b = store.insert(b_doc, Tol::witness());
    let (multi, _) = assembly("asm2b-ov-multi", &[b, b], 3.25);
    let opts = with_resolver(store);

    let single_ev = run(&single, &opts);
    let multi_ev = run(&multi, &opts);
    assert_eq!(
        product(&single, &single_ev, Tol::witness())
            .map(|b| b.solids().count())
            .ok(),
        Some(2),
        "ASM-2A's own shape: interference is not caught at rest"
    );
    assert_eq!(
        product(&multi, &multi_ev, Tol::witness())
            .map(|b| b.solids().count())
            .ok(),
        Some(4),
        "and a multi-solid instance reaches exactly the same decision"
    );
}
