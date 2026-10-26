//! **A document's parts are entered before its mate solve and node
//! map** (`eval::parts`' module docs): the top evaluation evaluates
//! each distinct part once, in one serial pass in document order, so
//! which instance "owns" a part's evaluation is no longer a question a
//! schedule or a lock answers.
//!
//! The rows here read the `probe` sample sequence, which is where an
//! owner shows: the part's samples lead every node's under the serial
//! walk, with and without a mate whose solve reads the part (whose
//! recording units belong to placements), and the parallel node map
//! records the serial walk's sequence over a three-level assembly. The
//! count rows beside them (`part_evaluations` under the parallel map, a
//! pose edit, an unchanged re-run and a faulted part) live in
//! `asm2a_instantiate.rs`, beside the memo rows they extend.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    CancelToken, CapEnd, DocRef, DocumentId, EvalOptions, Evaluation, Frame, Node, NodeResult,
    ProfileDoc, RecipeNodeId, evaluate,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame, on_pool, square};
use geom_core::Tol;
use geom_core::k_stats::{self, MarginSample, Probe};

type Sample = (&'static str, u64, u64, u64, String);

/// One sample, with every float read by its bits.
fn bits(s: &MarginSample) -> Sample {
    (
        s.predicate,
        s.margin.to_bits(),
        s.band_zero.to_bits(),
        s.band_escalate.to_bits(),
        format!("{:?}", s.outcome),
    )
}

/// A block: a square of half-side `half` about `(cx, cy)` on its own
/// xy, extruded `height` up its own z.
fn block(
    doc: ProfileDoc,
    (cx, cy): (f64, f64),
    half: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, cy, half)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// The part — one block, placed in its world — stored: the options
/// that resolve it, its reference, and the node its caps are named on.
fn stored_part(label: &str) -> (EvalOptions, DocRef, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut store = PartStore::new();
    let (doc_ref, body) = store.insert_part(block(doc, (0.0, 0.0), 0.5, 1.0), Tol::witness());
    (with_resolver(store), doc_ref, body)
}

/// How an assembly places its instances after the first, which is
/// pinned to the world at the identity.
#[derive(Clone, Copy)]
enum Rest {
    /// Each pinned at the identity too.
    Pinned,
    /// The second placed by a rest of its bottom cap on the first's
    /// top cap, the freedoms it leaves written 0, so the solve reads
    /// the part (its faces' poses).
    Mated,
}

/// An assembly of `n` instances of `part`, placed as `rest` says, after
/// — when `lead` — a block of its own whose samples differ from the
/// part's. The block's nodes come first in the serial order (Kahn,
/// least id first), so a walk that evaluated the part at an instance
/// would record the block's samples ahead of the part's.
fn assembly(
    label: &str,
    lead: bool,
    (_, part, body): &(EvalOptions, DocRef, RecipeNodeId),
    n: usize,
    rest: Rest,
) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    if lead {
        doc = block(doc, (3.0, 0.0), 0.3, 0.7).0;
    }
    let mut ids = Vec::new();
    for _ in 0..n {
        let (next, id) = insert(doc, fixture::mated_instance(*part));
        doc = next;
        ids.push(id);
    }
    if let Rest::Mated = rest {
        let mate = fixture::rest(
            fixture::head_at(ids[1], in_part(ids[1], *body, CapEnd::Start)),
            fixture::head_at(ids[0], in_part(ids[0], *body, CapEnd::End)),
        );
        doc = fixture::constrain(doc, mate).0;
    }
    doc
}

/// `doc` evaluated at `Probe`, and every sample the caller's sink
/// received.
fn recorded(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    parallel: bool,
) -> (Evaluation<Probe>, Vec<Sample>) {
    let opts = EvalOptions {
        parallel,
        ..opts.clone()
    };
    k_stats::start_recording();
    let ev = evaluate::<Probe>(doc, None, &CancelToken::new(), &opts, Tol::witness());
    let samples = k_stats::take_samples().iter().map(bits).collect();
    for (&id, result) in &ev.nodes {
        assert!(
            matches!(result, NodeResult::Ok(_)),
            "{}: node {} must evaluate for its samples to mean anything",
            doc.id(),
            id.0
        );
    }
    (ev, samples)
}

/// **Where the leading block's samples sit** in `full`, the run of a
/// document that is `rest` with the block put first: every `k` at which
/// `rest`'s sequence, with the block's spliced in at `k`, is `full`.
/// `0` is the block ahead of everything the rest records.
fn splices(full: &[Sample], block: &[Sample], rest: &[Sample]) -> Vec<usize> {
    (0..=rest.len())
        .filter(|&k| {
            full.len() == block.len() + rest.len()
                && full[..k] == rest[..k]
                && full[k..k + block.len()] == *block
                && full[k + block.len()..] == rest[k..]
        })
        .collect()
}

/// The part's samples lead the leading block's in `label`'s assembly
/// placed as `rest`: the block's samples splice into the block-less
/// assembly's sequence at one place, and not at its head. Returns the
/// serial sequence of the assembly with the block, and its part count.
fn the_part_leads(label: &str, n: usize, rest: Rest) -> (Vec<Sample>, usize) {
    let part = stored_part(&format!("{label}-part"));
    let opts = &part.0;
    let full = assembly(&format!("{label}-full"), true, &part, n, rest);
    let bare = assembly(&format!("{label}-bare"), false, &part, n, rest);
    let lead = block(
        ProfileDoc::empty(DocumentId::derive(&format!("{label}-lead")), Tol::witness()),
        (3.0, 0.0),
        0.3,
        0.7,
    )
    .0;
    let (full_ev, full_samples) = recorded(&full, opts, false);
    let (_, bare_samples) = recorded(&bare, opts, false);
    let (_, lead_samples) = recorded(&lead, opts, false);
    assert!(
        !lead_samples.is_empty(),
        "{label}: the leading block recorded no sample, so this row compares nothing"
    );
    let at = splices(&full_samples, &lead_samples, &bare_samples);
    assert_eq!(
        at.len(),
        1,
        "{label}: the leading block's samples splice into the block-less run at {at:?}, \
         not at exactly one place"
    );
    assert_ne!(
        at[0], 0,
        "{label}: the leading block's samples come first, so the part's were recorded after \
         a node's rather than ahead of every node's"
    );
    (full_samples, full_ev.part_evaluations)
}

/// **One part instanced three times is evaluated once, ahead of every
/// node.** Serially its samples lead the leading block's.
#[test]
fn a_part_instanced_three_times_is_entered_once_ahead_of_every_node() {
    let (_, count) = the_part_leads("parts-entered-pinned", 3, Rest::Pinned);
    assert_eq!(count, 1, "three instances, one part, one evaluation");
}

/// A block part, stored, and its reference.
fn stored_block(store: &mut PartStore, label: &str, half: f64, height: f64) -> DocRef {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    store
        .insert_part(block(doc, (0.0, 0.0), half, height), Tol::witness())
        .0
}

/// **The parallel node map records the serial walk's sample sequence
/// over a three-level assembly**, at one, three and four threads: a
/// leading block, then six instances — a mid-level part that instances
/// one leaf twice, that leaf, a second leaf, and the three again. Every
/// instance sits in level 0, so a part evaluated by whichever instance
/// a worker reached first would splice its samples at that instance;
/// entered before the map, the parts' samples are the serial walk's at
/// every width.
#[test]
fn a_nested_assembly_records_the_serial_walks_sequence_under_the_parallel_map() {
    let label = "parts-entered-nested";
    let mut store = PartStore::new();
    let leaf = stored_block(&mut store, &format!("{label}-leaf"), 0.5, 1.0);
    let other = stored_block(&mut store, &format!("{label}-other"), 0.4, 0.6);
    let mid = ProfileDoc::empty(DocumentId::derive(&format!("{label}-mid")), Tol::witness());
    let (mid, _) = insert(mid, crate::fixture::mated_instance(leaf));
    let (mid, apart) = insert(mid, fixture::mated_instance(leaf));
    let mid = fixture::pin(
        mid,
        apart,
        &Frame::translation([9.0, 0.0, 0.0]),
        &with_resolver(store.clone()),
    );
    let mid = crate::fixture::assemble_world_copies(mid);
    let mid = store.insert(mid, Tol::witness());
    let opts = with_resolver(store);
    let mut top = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    top = block(top, (3.0, 0.0), 0.3, 0.7).0;
    for part in [mid, leaf, other, mid, leaf, other] {
        top = insert(top, crate::fixture::mated_instance(part)).0;
    }
    let (serial_ev, serial) = recorded(&top, &opts, false);
    assert_eq!(
        serial_ev.part_evaluations, 4,
        "the mid part and the leaf inside it, the leaf, the other leaf"
    );
    for threads in [1, 3, 4] {
        let (ev, parallel) = on_pool(threads, || recorded(&top, &opts, true));
        assert_eq!(
            ev.part_evaluations, serial_ev.part_evaluations,
            "{threads} thread(s): the parallel map evaluated another set of parts"
        );
        assert!(
            parallel == serial,
            "{threads} thread(s): the parallel schedule recorded {} samples against the \
             serial walk's {}, first difference at {:?}",
            parallel.len(),
            serial.len(),
            parallel.iter().zip(&serial).position(|(a, b)| a != b)
        );
    }
}

/// **A mated part's samples land ahead of every node, not in a
/// placement's recording unit.** The solve reads the part (the mated
/// faces' poses) and keeps what it records per
/// placement, spliced at that placement's node; the part was entered
/// before the solve ran, so none of its samples are there.
#[test]
fn a_mated_parts_samples_lead_every_node() {
    let (_, count) = the_part_leads("parts-entered-mated", 2, Rest::Mated);
    assert_eq!(
        count, 1,
        "two instances and a mate, one part, one evaluation"
    );
}
