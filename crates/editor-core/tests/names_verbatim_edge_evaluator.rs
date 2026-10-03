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
//! The tie is observable on every evaluated value, row by row: a row is
//! FOREIGN when its head `node` is not the publishing node's own id,
//! and OWN otherwise. Each edge the walks follow fixes the mix:
//!
//! | `verbatim_kind`  | the node's table                         |
//! |------------------|------------------------------------------|
//! | `Some(Whole)`    | at least one row, and every row foreign  |
//! | `Some(Selected)` | at least one row, and every row foreign  |
//! | `Some(Intact)`   | at least one foreign and one own row     |
//! | `None`           | no foreign row                           |
//!
//! A transform or a part mints nothing, so one own row there is a
//! partial re-mint the walks would carry a name across wrongly. A split
//! publishes its intact entities under their original minters and its
//! cut faces under its own id, so it carries both.
//!
//! **Why the head.** An op that re-mints by WRAPPING a foreign name
//! inside a role argument (a blend's `FromTarget`, an instance's
//! `InPart`, a boolean's operand segments) heads the wrapped row
//! itself, so only the head is compared: a walk into role arguments
//! would read those ops as pass-throughs.
//!
//! **What the coverage census holds, exactly.** The roster below is
//! welded to [`Node`] by `test_utils::f6_variants!`, so a variant added
//! to the vocabulary stops this file compiling until it is named here.
//! A kind then counts as SAMPLED only when some corpus node of that kind
//! published at least one row — a kind that evaluates to an empty table
//! satisfies the `None` row vacuously, so evaluating is not enough. Two
//! lists stand in for the rows that cannot exist:
//!
//! - [`ROW_FREE`]: kinds whose value carries no names. They must still
//!   evaluate somewhere, and every sample of one is asserted to publish
//!   ZERO rows — so one that starts publishing is checked, not trusted.
//! - `corpus::NEVER_EVALUATES`: kinds no document can evaluate at all
//!   (the frontier's one home, which `m4_pr8_corpus` reads too).
//!
//! Both are held in the other direction as well: a listed kind that
//! publishes rows, or evaluates, reds until its entry is removed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus;
use crate::fixture;

use editor_core::test_support::{VerbatimKind, verbatim_kind};
use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocumentId, EvalOptions, MateFrame, MatePrimitive,
    Node, ProfileDoc, ProfileProgram, RecipeNodeId, StableName,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame, step};
use geom_core::Tol;

type ProfileNode = Node<ProfileProgram>;

test_utils::f6_variants! {
    /// **Every node kind**, welded to `Node` by the match the macro
    /// writes: a variant added to the vocabulary leaves it
    /// non-exhaustive until it is named here, and the census below then
    /// reds until the corpus samples one.
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
        InstantiatePart,
        Gauge,
        Mate,
        Measure,
        Assertion,
    ];
}

/// **The node kinds whose value carries no names**: a datum, a profile,
/// a solved mate, a measurement and an assertion verdict are not
/// bodies, so their tables are empty and the equivalence holds of them
/// vacuously. Listed so that vacuity is asserted — every sample publishes zero rows — rather than counted as
/// coverage.
const ROW_FREE: [&str; 6] = ["Datum", "Profile", "Mate", "Gauge", "Measure", "Assertion"];

/// The unit cube `[0,1]³` as a whole part document, and its body.
fn block(label: &str) -> (ProfileDoc, RecipeNodeId) {
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
}

fn mate_frame(origin: [f64; 3]) -> MateFrame {
    MateFrame::authored(origin, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
}

/// **Two instanced blocks on a gauge, one seated on the other** — the
/// assembly kinds the Band 4 corpus does not author (`InstantiatePart`,
/// `Gauge`, `Mate`).
/// An instance's rows wrap its part's names in `InPart` under the
/// instance's own head, which is the shape the head comparison exists
/// to read correctly.
fn stacked_blocks() -> (ProfileDoc, EvalOptions) {
    let mut store = PartStore::new();
    let (base, base_body) = store.insert_part(block("vedge-base"), Tol::witness());
    let (top, top_body) = store.insert_part(block("vedge-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("vedge-stack"), Tol::witness());
    let (doc, gauge) = insert(doc, Node::gauge(None, editor_core::Placement::IDENTITY));
    let (doc, base) = insert(doc, Node::instantiate_part(base));
    let (doc, top) = insert(doc, Node::instantiate_part(top));
    let (doc, _) = step(
        doc,
        editor_core::DocEdit::SetGauge {
            node: base,
            gauge: Some(gauge),
        },
    );
    let (doc, _) = step(
        doc,
        editor_core::DocEdit::SetGauge {
            node: top,
            gauge: Some(gauge),
        },
    );
    let (doc, _mate) = insert(
        doc,
        Node::Mate {
            a: fixture::head(in_part(base, base_body, CapEnd::End)),
            b: fixture::head(in_part(top, top_body, CapEnd::Start)),
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

/// One evaluated node, as the two rows below read it.
struct Sample {
    /// The document it was evaluated in.
    doc: &'static str,
    /// Its id there.
    id: RecipeNodeId,
    /// Its `Node` variant.
    kind: String,
    /// The edge the recipe walks read it as.
    edge: Option<VerbatimKind>,
    /// Its table's row count.
    rows: usize,
    /// A row headed by another node, if any.
    foreign: Option<StableName>,
    /// A row headed by the node itself, if any.
    own: Option<StableName>,
}

/// Every node with a value across [`samples`], each document required
/// green first — a failed node would shrink the sample silently.
fn evaluated() -> Vec<Sample> {
    let mut out = Vec::new();
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
            let heads = || value.name_table.iter().map(|(n, _)| n);
            out.push(Sample {
                doc: name,
                id,
                kind: test_utils::f6::variant_identifier(node),
                edge: verbatim_kind(node),
                rows: value.name_table.len(),
                foreign: heads().find(|n| n.node != id).cloned(),
                own: heads().find(|n| n.node == id).cloned(),
            });
        }
    }
    out
}

/// What is wrong with `s` against the edge it is classified as, if
/// anything (the table in the module docs).
fn disagreement(s: &Sample) -> Option<String> {
    let (foreign, own) = (s.foreign.is_some(), s.own.is_some());
    let wanted = match s.edge {
        Some(VerbatimKind::Whole | VerbatimKind::Selected) if !foreign || own => {
            "every row headed by another node"
        }
        Some(VerbatimKind::Intact) if !foreign || !own => {
            "rows headed by another node AND rows headed by itself"
        }
        None if foreign => "every row headed by itself",
        None if ROW_FREE.contains(&s.kind.as_str()) && s.rows > 0 => {
            "no rows at all (it is listed in `ROW_FREE`)"
        }
        _ => return None,
    };
    let seen = |n: &Option<StableName>| n.as_ref().map_or("none".to_string(), |n| n.to_string());
    Some(format!(
        "{}: {} at {:?}, read as {:?} by `verbatim_edge`, should publish {wanted}; its \
         table has {} rows, a foreign one {}, an own one {}",
        s.doc,
        s.kind,
        s.id,
        s.edge,
        s.rows,
        seen(&s.foreign),
        seen(&s.own),
    ))
}

/// INVARIANT (N1, held against the producer): every evaluated node's
/// rows are headed the way the edge `verbatim_edge` reads it as
/// requires — foreign only for `Whole`/`Selected`, both for `Intact`,
/// own only for `None` — and a `ROW_FREE` kind publishes none.
///
/// Red when a node that evaluates to its input's names is filed under
/// `None`, when a re-minting node is classified as an edge, when a node
/// is filed under the wrong edge, and when a transform or a part
/// re-mints part of what it carries.
#[test]
fn every_node_publishes_the_heads_its_verbatim_edge_requires() {
    let disagreements: Vec<String> = evaluated().iter().filter_map(disagreement).collect();
    assert!(
        disagreements.is_empty(),
        "`names::verbatim_edge` and the evaluator disagree on which nodes pass names \
         through:\n  {}",
        disagreements.join("\n  ")
    );
}

/// INVARIANT: the row above samples every node kind — each kind
/// published rows somewhere in the corpus, or is `ROW_FREE` and
/// evaluated somewhere, or is on the evaluation frontier. Both lists
/// are exact: a `ROW_FREE` kind that publishes, or a frontier kind that
/// evaluates, reds until its entry goes.
#[test]
fn the_corpus_samples_every_node_kind_with_rows() {
    let all = evaluated();
    let with_rows: BTreeSet<&str> = all
        .iter()
        .filter(|s| s.rows > 0)
        .map(|s| s.kind.as_str())
        .collect();
    let evaluated: BTreeSet<&str> = all.iter().map(|s| s.kind.as_str()).collect();
    let row_free: BTreeSet<&str> = evaluated
        .iter()
        .copied()
        .filter(|k| ROW_FREE.contains(k))
        .collect();

    let mut reports = Vec::new();
    // Every kind the roster names is sampled or exempt, and nothing is
    // both: a sampled frontier kind lands in the witnessed set without
    // being declared.
    let declared: Vec<&str> = NODE_KIND
        .identifiers()
        .iter()
        .copied()
        .filter(|k| !corpus::NEVER_EVALUATES.contains(k))
        .collect();
    let sampled: Vec<&str> = with_rows
        .iter()
        .chain(&row_free)
        .chain(
            evaluated
                .iter()
                .filter(|k| corpus::NEVER_EVALUATES.contains(k)),
        )
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    reports.extend(test_utils::census::set_difference(
        &declared,
        &sampled,
        "the node kinds this guard must sample and the kinds the corpus sampled disagree",
        "evaluated here, yet listed in `corpus::NEVER_EVALUATES` — drop it there",
        "no document publishes a row from one — add one to `samples`, or list it in \
         `ROW_FREE` (it evaluates, to no names) or `corpus::NEVER_EVALUATES` (it cannot \
         evaluate), with the reason",
    ));
    // `ROW_FREE` is exact: each entry evaluated, and none published.
    let row_free_witnessed: Vec<&str> = row_free
        .iter()
        .copied()
        .filter(|k| !with_rows.contains(k))
        .collect();
    reports.extend(test_utils::census::set_difference(
        &ROW_FREE,
        &row_free_witnessed,
        "`ROW_FREE` and the row-free kinds the corpus evaluated disagree",
        "unreachable: witnessed only from `ROW_FREE` itself",
        "listed as row-free, and either no document evaluates one or one published rows \
         — evaluate one in `samples`, or drop the entry",
    ));
    assert!(reports.is_empty(), "{}", reports.join("\n"));
}
