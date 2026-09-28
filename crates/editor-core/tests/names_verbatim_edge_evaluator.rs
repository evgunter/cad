//! **The recipe walks' pass-through set agrees with what evaluation
//! passes through.**
//!
//! `names::verbatim_edge` is the one statement of N1's pass-through set
//! that the product's two-roots check and the mate member walk follow.
//! What actually passes names through is the evaluator's decision
//! (`eval::wire`), and the compiler holds nothing between the two: a
//! node whose evaluation hands its input's names on unchanged compiles
//! filed as re-minting, and both walks stop at it silently.
//!
//! The tie is observable on every evaluated value. A node publishes a
//! name it did NOT mint — a row whose head `node` is not the node's own
//! id — exactly when it carries names through verbatim, because every
//! other op wraps what it carries in a segment of its own and heads the
//! result itself. So over a corpus that evaluates every node kind that
//! can evaluate:
//!
//! ```text
//! (some row of node id's table has name.node != id)
//!     ⇔ carries_names_verbatim(node)
//! ```
//!
//! **Why "some row" and why the head.** A split publishes both kinds of
//! row — the intact entities under their original minters and the cut
//! faces under its own id — so the left side is an existential, not a
//! universal. An op that re-mints by WRAPPING a foreign name inside a
//! role argument (a blend's `FromTarget`, an instance's `InPart`, a
//! boolean's operand segments) heads the wrapped row itself, so only
//! the head is compared: a walk into role arguments would read those
//! ops as pass-throughs.
//!
//! **Coverage is asserted, not assumed.** The roster below is welded to
//! [`Node`] by `test_utils::f6_variants!`, so a variant added to the
//! vocabulary stops this file compiling until it is named here, and the
//! census then reds until some corpus document evaluates one — or until
//! it is listed in [`NEVER_EVALUATES`], whose own entries are held in
//! the other direction: an exempt kind that does evaluate reds too.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus;
use crate::fixture;

use editor_core::test_support::carries_names_verbatim;
use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocRef, DocumentId, EvalOptions, MateFrame,
    MatePrimitive, Node, ProfileDoc, ProfileProgram,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame};
use geom_core::Tol;

type ProfileNode = Node<ProfileProgram>;

test_utils::f6_variants! {
    /// **Every node kind**, welded to `Node` by the match the macro
    /// writes: a variant added to the vocabulary leaves it
    /// non-exhaustive until it is named here, and the census below then
    /// reds until the corpus evaluates one.
    const NODE_KIND: ProfileNode = [
        Datum,
        Profile,
        Extrude,
        Revolve,
        Tube,
        HollowTube,
        Loft,
        Sweep,
        Fillet,
        Chamfer,
        Shell,
        Split,
        Boolean,
        Union,
        Transform,
        Pattern,
        Part,
        PlacedUnion,
        Declare,
        InstantiatePart,
        Mate,
        Measure,
        Assertion,
    ];
}

/// The node kinds no document can evaluate to a value, each with the
/// reason — the kinds the census does not require.
///
/// - `Sweep`: a sweep's path operand is a validated profile, whose loop
///   is closed and so has two or more segments, and every multi-segment
///   path refuses at `wire_sweep`'s one frontier arm
///   (`review_m5_pr10_sweep_node`).
const NEVER_EVALUATES: [&str; 1] = ["Sweep"];

/// The unit cube `[0,1]³` as a whole part document, its body at
/// `fixture::resolver::PART_BODY` so `in_part` names its caps.
fn block(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    )
    .0
}

fn mate_frame(origin: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// **Two instanced blocks, one seated on the other** — the assembly
/// kinds the Band 4 corpus does not author (`InstantiatePart`, `Mate`).
/// An instance's rows wrap its part's names in `InPart` under the
/// instance's own head, which is the shape the head comparison exists
/// to read correctly.
fn stacked_blocks() -> (ProfileDoc, EvalOptions) {
    let mut store = PartStore::new();
    let base: DocRef = store.insert(block("vedge-base"), Tol::witness());
    let top: DocRef = store.insert(block("vedge-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("vedge-stack"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base));
    let (doc, top) = insert(doc, Node::instantiate_part(top));
    let (doc, _mate) = insert(
        doc,
        Node::Mate {
            a: fixture::head(in_part(base, CapEnd::End)),
            b: fixture::head(in_part(top, CapEnd::Start)),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: mate_frame([0.0, 0.0, 1.0]),
                b: mate_frame([0.0, 0.0, 0.0]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Opposed,
                clocking: None,
            },
        },
    );
    (doc, with_resolver(store))
}

/// The documents the guard evaluates: the Band 4 registry, the two
/// hollowing documents that sit beside it (the registry's `Shell`
/// frontier), and [`stacked_blocks`].
fn samples() -> Vec<(&'static str, ProfileDoc, EvalOptions)> {
    let mut out: Vec<(&'static str, ProfileDoc, EvalOptions)> = corpus::documents()
        .into_iter()
        .chain([corpus::cup::document(), corpus::vessel::document()])
        .map(|d| (d.name, d.doc, EvalOptions::default()))
        .collect();
    let (doc, opts) = stacked_blocks();
    out.push(("stacked_blocks", doc, opts));
    out
}

/// INVARIANT (N1, held against the producer): a node publishes a name
/// whose head is another node exactly when `verbatim_edge` classifies
/// it as a name-carrying edge — and every node kind that can evaluate
/// is sampled.
///
/// Red when a node that evaluates to its input's names is filed under
/// `None` (it publishes a foreign head and is not classified), or when
/// a re-minting node is classified (it publishes only its own heads).
#[test]
fn a_node_publishes_foreign_names_iff_verbatim_edge_classifies_it() {
    let mut disagreements = Vec::new();
    let mut sampled: BTreeSet<String> = BTreeSet::new();
    for (name, doc, opts) in samples() {
        let ev = fixture::run(&doc, &opts);
        let failures = corpus::failures(&ev);
        assert!(
            failures.is_empty(),
            "{name}: every node must evaluate, or the guard samples less than it claims: \
             {failures:#?}"
        );
        for (&id, result) in &ev.nodes {
            let Some(value) = result.value() else {
                continue;
            };
            let node = doc.node(id).expect("an evaluated node is in its document");
            let kind = test_utils::f6::variant_identifier(node);
            let foreign = value
                .name_table
                .iter()
                .map(|(n, _)| n)
                .find(|n| n.node != id);
            let classified = carries_names_verbatim(node);
            if foreign.is_some() != classified {
                disagreements.push(format!(
                    "{name}: {kind} at {id:?} is {} by `verbatim_edge`, and its table \
                     ({} rows) {}",
                    if classified {
                        "a name-carrying edge"
                    } else {
                        "re-minting"
                    },
                    value.name_table.len(),
                    match foreign {
                        Some(n) => format!("publishes {n}"),
                        None => "heads every row itself".to_string(),
                    },
                ));
            }
            sampled.insert(kind);
        }
    }
    assert!(
        disagreements.is_empty(),
        "`names::verbatim_edge` and the evaluator disagree on which nodes pass names \
         through:\n  {}",
        disagreements.join("\n  ")
    );

    let required: Vec<&str> = NODE_KIND
        .identifiers()
        .iter()
        .copied()
        .filter(|k| !NEVER_EVALUATES.contains(k))
        .collect();
    let sampled: Vec<&str> = sampled.iter().map(String::as_str).collect();
    if let Some(report) = test_utils::census::set_difference(
        &required,
        &sampled,
        "the node kinds this guard must sample and the kinds the corpus evaluated disagree",
        "evaluated here and exempted as never evaluating — drop it from `NEVER_EVALUATES`",
        "no document evaluates one — add one to `samples`, or exempt it in \
         `NEVER_EVALUATES` with the reason it cannot evaluate",
    ) {
        panic!("{report}");
    }
}
