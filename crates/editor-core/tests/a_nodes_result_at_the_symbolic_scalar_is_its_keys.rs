//! **A node's result at the symbolic scalar is a function of its key**,
//! on a document whose replays run the retry ladder and on one whose
//! constructors register identities.
//!
//! The rows replay a document's leaves as the driver does, each in a
//! session of its own on one drive's table, and read every node's
//! result: its verdict rows, escalations and marks, and what it left in
//! the symbolic session (`NodeValue::sym`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{
    DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS, DriveConfig, ParamBoxVerdict, drive,
};
use editor_core::{CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, evaluate};
use geom_core::sym::{DriveMemo, with_session_memo_retry};
use geom_core::{Interval, Sym, SymBudget, SymCounts, SymRetry, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

/// A document whose replays run the retry ladder: R2's filleted
/// bracket, at ten ε.
fn retrying(tol: Tol) -> ProfileDoc {
    crate::m10_7_r2_probes_interval::bracket(1.0e1 * tol.eps(), tol).0
}

/// A document whose constructors register identities: the two-hole
/// plate, whose arcs state their rims.
fn registering(tol: Tol) -> ProfileDoc {
    crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0
}

/// The document's box and, for each axis that varies, its two halves:
/// a node that does not read an axis has one key in both.
fn boxes(doc: &ProfileDoc) -> Vec<ParamBox> {
    let root = ParamBox::of(&analyzed_box(doc, &AnalysisPolicy::default()));
    let halves = root.varying().flat_map(|(var, lo, hi)| {
        let mid = 0.5 * (lo + hi);
        [(lo, mid), (mid, hi)].map(|(lo, hi)| {
            let mut axes = root.axes().clone();
            axes.insert(var, BoxAxis::Varying { lo, hi });
            ParamBox::from_axes(axes)
        })
    });
    core::iter::once(root.clone()).chain(halves).collect()
}

/// One leaf replayed on the drive's table, as `drive::classify` does.
fn leaf(
    doc: &ProfileDoc,
    box_: &ParamBox,
    table: &Arc<DriveMemo>,
    tol: Tol,
) -> (Evaluation<Sym<Interval>>, SymCounts) {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..EvalOptions::default()
    };
    with_session_memo_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        table,
        || evaluate(doc, None, &CancelToken::new(), &opts, tol),
    )
}

/// What a row reads off a document's replays.
#[derive(Debug, Default)]
struct Read {
    /// Under each (node, content key, naming key), every distinct
    /// result text.
    results: BTreeMap<String, BTreeSet<String>>,
    /// Results read under a key already seen.
    repeats: usize,
    retried: u64,
    registrations: usize,
    /// Per leaf: the session's counts, and the sum of its nodes'
    /// records.
    receipts: Vec<(SymCounts, SymCounts)>,
}

fn read(doc: &ProfileDoc, tol: Tol) -> Read {
    let table = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
    let mut out = Read::default();
    for box_ in boxes(doc) {
        let (ev, session) = leaf(doc, &box_, &table, tol);
        let mut nodes = SymCounts::default();
        for id in &ev.order {
            let record = match ev.result(*id) {
                Some(NodeResult::Ok(v)) => {
                    let key = format!("{id:?} {:?} {:?}", v.content_key, v.naming_key);
                    let text = format!(
                        "{:?} | {:?} | {:?} | {:?}",
                        v.verdicts, v.escalations, v.marks, v.sym
                    );
                    let seen = out.results.entry(key).or_default();
                    out.repeats += usize::from(!seen.is_empty());
                    seen.insert(text);
                    &v.sym
                }
                Some(NodeResult::Failed(e)) => &e.sym,
                Some(NodeResult::Poisoned { .. }) | None => continue,
            };
            nodes.absorb(record.counts());
            out.retried += record.counts().retried;
            out.registrations += record.registrations();
        }
        out.receipts.push((
            SymCounts {
                frozen: 0,
                ..session
            },
            nodes,
        ));
    }
    out
}

fn assert_one_result_per_key(label: &str, read: &Read) {
    assert!(
        read.repeats > 0,
        "{label}: no node repeats a key across the leaves"
    );
    for (key, results) in &read.results {
        assert_eq!(
            results.len(),
            1,
            "{label}: node {key} has {} results under one key",
            results.len()
        );
    }
}

/// **One key, one result across leaves, where the retry ladder runs.**
/// Every node of the bracket that has one content key in two leaves
/// has one result in them: the same verdict rows, escalations and
/// marks, and the same symbolic record, its retried decisions among
/// them.
///
/// What it cannot see: every leaf runs its nodes in one order, so a
/// dependence of a node on an earlier node of the SAME leaf (a shared
/// retry guard) gives every leaf the same wrong record. That is the
/// tier's row, `a_nodes_answer_..::the_retry_guard_counts_a_nodes_own_forms`.
#[test]
fn one_key_one_result_across_leaves_in_a_drive_that_retries() {
    let tol = Tol::witness();
    let read = read(&retrying(tol), tol);
    println!(
        "retrying: {} keys, {} repeats, retried {}, registrations {}",
        read.results.len(),
        read.repeats,
        read.retried,
        read.registrations
    );
    assert!(read.retried > 0, "the ladder closed a decision");
    assert_one_result_per_key("retrying", &read);
}

/// **One key, one result across leaves, where identities are
/// registered**: the plate.
///
/// What it cannot see, for the reason above: a registry shared by the
/// nodes of one leaf. That is the tier's row,
/// `a_nodes_answer_..::a_registration_is_not_read_by_a_node_it_does_not_feed`.
#[test]
fn one_key_one_result_across_leaves_in_a_drive_that_registers() {
    let tol = Tol::witness();
    let read = read(&registering(tol), tol);
    println!(
        "registering: {} keys, {} repeats, retried {}, registrations {}",
        read.results.len(),
        read.repeats,
        read.retried,
        read.registrations
    );
    assert!(read.registrations > 0, "a constructor registered");
    assert_one_result_per_key("registering", &read);
}

/// **A leaf's receipt is the sum of its nodes' records** on two
/// documents with no family, no part and no product gather: nothing
/// there is decided at a symbolic scalar outside a node's scope. The
/// rows below read a family, a part and the product.
///
/// Mutant: a node's record that drops its counts leaves the sum short.
#[test]
fn a_plain_documents_receipt_is_the_sum_of_its_nodes_records() {
    let tol = Tol::witness();
    for (label, doc) in [
        ("retrying", retrying(tol)),
        ("registering", registering(tol)),
    ] {
        for (i, (session, nodes)) in read(&doc, tol).receipts.iter().enumerate() {
            assert!(session.decisions() > 0, "{label} leaf {i} decided");
            assert_eq!(session, nodes, "{label} leaf {i}");
        }
    }
}

fn driven(doc: &ProfileDoc, parallel: bool, plain_memo: bool, tol: Tol) -> ParamBoxVerdict {
    drive(
        doc,
        &analyzed_box(doc, &AnalysisPolicy::default()),
        &DriveConfig {
            max_leaves: 8,
            parallel,
            plain_memo,
            ..DriveConfig::default()
        },
        &editor_core::Stop::default(),
        tol,
    )
    .expect("the document drives")
}

/// **One thread against many, and the drive's table serving against
/// not**: the verdict, every leaf with its receipt, and the serialized
/// drive are the same, on the retrying document and the registering
/// one.
///
/// What it cannot see: a leaf handed another leaf's box-dependent
/// forms where the documents' leaves take the same arms. That is the
/// tier's row, `a_drives_table_..::a_leafs_early_forms_are_its_boxs`.
#[test]
fn two_drives_are_the_same_on_one_thread_or_many_and_with_the_table_or_without() {
    let tol = Tol::witness();
    // The bracket past the box it certifies whole, so the drive splits.
    let wide_bracket = crate::m10_7_r2_probes_interval::bracket(1.0e3 * tol.eps(), tol).0;
    for (label, doc) in [
        ("retrying", wide_bracket),
        ("registering", registering(tol)),
    ] {
        let reference = driven(&doc, false, true, tol);
        assert!(
            reference.certified().len() + reference.refused().len() > 1,
            "{label}: the drive has more than one leaf"
        );
        assert!(
            label != "retrying" || reference.decisions().retried > 0,
            "the ladder closed a decision in the retrying drive"
        );
        for (parallel, plain_memo) in [(true, true), (false, false), (true, false)] {
            let other = driven(&doc, parallel, plain_memo, tol);
            let what = format!("{label}: parallel {parallel}, the table serving {plain_memo}");
            assert_eq!(reference.serialize(), other.serialize(), "{what}");
            assert_eq!(reference.certified(), other.certified(), "{what}");
            assert_eq!(reference.refused(), other.refused(), "{what}");
        }
    }
}

// ----- every decision of a leaf is on a record a reader receives -----

/// The whole of a leaf's receipt as the evaluation carries it: its
/// nodes' records and its parts' runs'.
fn carried(ev: &Evaluation<Sym<Interval>>) -> (SymCounts, usize) {
    let mut sum = SymCounts::default();
    let mut registrations = 0;
    for id in &ev.order {
        let record = match ev.result(*id) {
            Some(NodeResult::Ok(v)) => &v.sym,
            Some(NodeResult::Failed(e)) => &e.sym,
            Some(NodeResult::Poisoned { .. }) | None => continue,
        };
        sum.absorb(record.counts());
        registrations += record.registrations();
    }
    for part in &ev.parts_sym {
        sum.absorb(part.counts());
    }
    (sum, registrations)
}

fn one_leaf(
    doc: &ProfileDoc,
    resolver: Option<Arc<dyn editor_core::PartResolver>>,
    prior: Option<&Evaluation<Sym<Interval>>>,
    table: &Arc<DriveMemo>,
    tol: Tol,
) -> (Evaluation<Sym<Interval>>, SymCounts) {
    let root = ParamBox::of(&analyzed_box(doc, &AnalysisPolicy::default()));
    let opts = EvalOptions {
        param_box: Some(Arc::new(root)),
        resolver,
        ..EvalOptions::default()
    };
    let (ev, session) = with_session_memo_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        table,
        || evaluate(doc, prior, &CancelToken::new(), &opts, tol),
    );
    (
        ev,
        SymCounts {
            frozen: 0,
            ..session
        },
    )
}

fn table() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(budget(), SymRules::shipped()))
}

/// **A family's value carries its members' records.** The plate with a
/// row of three copies of its first hole: the family's record holds
/// what its three members decided and registered, and the leaf's
/// receipt is the sum over its nodes.
///
/// Mutant: a family value built with an empty record leaves the
/// receipt short by the members' decisions.
#[test]
fn a_familys_value_carries_its_members_records() {
    let tol = Tol::witness();
    let p = crate::m10_7_plate::plate_with_holes(5.0e-5, 1.0e-5, tol);
    let (doc, _k, row) = crate::fixture::row(
        p.doc,
        p.holes[0],
        ("k", 3),
        editor_core::TargetAxis::Y,
        5.0e-3,
    );
    let (ev, session) = one_leaf(&doc, None, None, &table(), tol);
    let Some(NodeResult::Ok(family)) = ev.result(row) else {
        panic!("the row builds");
    };
    let members = family.members.as_ref().expect("the row is a family");
    let mut of_members = SymCounts::default();
    let mut identities = 0;
    for (_, member) in &members.members {
        of_members.absorb(member.sym.counts());
        identities += member.sym.registrations();
    }
    assert!(of_members.decisions() > 0, "the members decided");
    let whole = family.sym.counts();
    assert!(
        whole.symbolic >= of_members.symbolic && whole.numeric >= of_members.numeric,
        "the family's record holds its members' decisions: {whole:?} against {of_members:?}"
    );
    assert_eq!(
        family.sym.registrations(),
        identities,
        "and their identities"
    );
    assert_eq!(session, carried(&ev).0, "the receipt is the sum");
}

#[derive(Debug)]
struct OneDoc(ProfileDoc);

impl editor_core::PartResolver for OneDoc {
    fn resolve(
        &self,
        _doc_ref: &editor_core::DocRef,
        _tol: Tol,
    ) -> Result<ProfileDoc, editor_core::ResolveFailure> {
        Ok(self.0.clone())
    }
}

/// A cylinder: a circle of radius 1 extruded 1, placed in the world.
fn cylinder(label: &str, tol: Tol) -> ProfileDoc {
    use editor_core::{LoopProgram, Node, ProfileProgram};
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive(label), tol);
    let (doc, profile) = crate::fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [crate::fixture::len(0.0), crate::fixture::len(0.0)],
                radius: crate::fixture::len(1.0),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, body) = crate::fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: crate::fixture::len(1.0),
        },
    );
    crate::fixture::place(doc, body).0
}

/// The cylinder instanced once, in a document named `assembly`.
fn instancing(
    part: &ProfileDoc,
    assembly: &str,
    tol: Tol,
) -> (ProfileDoc, editor_core::RecipeNodeId) {
    let pin = editor_core::content_pin(part, tol).expect("the pin computes");
    let doc_ref = editor_core::DocRef { id: part.id(), pin };
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive(assembly), tol);
    let (doc, instance) = crate::fixture::insert(doc, crate::fixture::mated_instance(doc_ref));
    (crate::fixture::assemble_world_copies(doc), instance)
}

/// **A part's run is counted once, as the part's, and its identities
/// reach the node that instantiates it.** The cylinder as a part: the
/// leaf's receipt is its nodes' records and its parts', and the
/// instance node's record holds the identities the part's extrude
/// registered.
///
/// Mutant: a part's run in no scope of its own leaves its decisions at
/// the root, on no record; an instance node that does not gather the
/// row's record carries no identity.
#[test]
fn a_parts_run_is_the_parts_and_its_identities_reach_its_instance() {
    let tol = Tol::witness();
    let part = cylinder("session-state-part", tol);
    let (own, own_session) = one_leaf(&part, None, None, &table(), tol);
    let (own_sum, own_registrations) = carried(&own);
    assert_eq!(own_session, own_sum, "as its own document");
    assert!(own_registrations > 0, "the cylinder's extrude registers");

    let (doc, instance) = instancing(&part, "session-state-assembly", tol);
    let (ev, session) = one_leaf(&doc, Some(Arc::new(OneDoc(part))), None, &table(), tol);
    assert_eq!(ev.parts_sym.len(), 1, "one part run");
    assert!(ev.parts_sym[0].counts().decisions() > 0, "the part decided");
    let Some(NodeResult::Ok(placed)) = ev.result(instance) else {
        panic!("the instance builds");
    };
    assert_eq!(
        placed.sym.registrations(),
        ev.parts_sym[0].registrations(),
        "the instance node's record holds the part's identities"
    );
    assert_eq!(session, carried(&ev).0, "the receipt is nodes and parts");
}

/// **A part's run is a function of the part**: instanced from two
/// documents, in leaves of two drives, the part's record is the same,
/// whatever the instancing document is.
///
/// Mutant: a part's scope opened over the registry of the scope it was
/// entered from reads what the instancing document registered.
#[test]
fn a_parts_run_is_the_same_in_two_instancing_documents() {
    let tol = Tol::witness();
    let part = cylinder("session-state-part", tol);
    let record_in = |assembly: &str| {
        let (doc, _) = instancing(&part, assembly, tol);
        let (ev, _) = one_leaf(
            &doc,
            Some(Arc::new(OneDoc(part.clone()))),
            None,
            &table(),
            tol,
        );
        ev.parts_sym[0].counts()
    };
    assert_eq!(
        record_in("session-state-first"),
        record_in("session-state-second")
    );
}

/// **The product's gather is the product's.** The gather runs after
/// the nodes; what it decides is on the product, and the session's
/// counts are the nodes', the parts' and the product's.
///
/// Mutant: a gather in no scope of its own decides at the root, and
/// the product's record is empty.
#[test]
fn the_products_gather_is_on_the_product() {
    let tol = Tol::witness();
    let part = cylinder("session-state-part", tol);
    let (doc, _) = instancing(&part, "session-state-product", tol);
    let root = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
    let opts = EvalOptions {
        param_box: Some(Arc::new(root)),
        resolver: Some(Arc::new(OneDoc(part))),
        ..EvalOptions::default()
    };
    let ((ev, product), session) = with_session_memo_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        &table(),
        || {
            let ev: Evaluation<Sym<Interval>> =
                evaluate(&doc, None, &CancelToken::new(), &opts, tol);
            let product = editor_core::product::product_recorded(&doc, &ev, tol);
            (ev, product)
        },
    );
    let product = product.expect("the product gathers");
    assert!(product.sym.counts().decisions() > 0, "the gather decided");
    let mut sum = carried(&ev).0;
    sum.absorb(product.sym.counts());
    assert_eq!(
        SymCounts {
            frozen: 0,
            ..session
        },
        sum,
        "nodes, parts and the product"
    );
}

/// **A prior built under another table is not read.** The plate's root
/// leaf evaluated in a session on no drive's table, then handed as the
/// prior to a leaf on a drive's table: nothing is reused, and the leaf
/// decides what it decides with no prior.
///
/// Mutant: a prior read whatever table built it hands the leaf values
/// whose nodes its table does not hold; every node is reused and the
/// leaf decides nothing.
#[test]
fn a_prior_built_under_another_table_is_not_read() {
    let tol = Tol::witness();
    let doc = registering(tol);
    let (prior, alone) = one_leaf(&doc, None, None, &table(), tol);
    let (ev, session) = one_leaf(&doc, None, Some(&prior), &table(), tol);
    assert_eq!(ev.reused, 0, "nothing of the foreign prior is reused");
    assert_eq!(session, alone, "the leaf decides as it does with no prior");
    // Another leaf of the SAME table is another session, and its prior
    // is read.
    let shared = table();
    let (first, _) = one_leaf(&doc, None, None, &shared, tol);
    let (second, _) = one_leaf(&doc, None, Some(&first), &shared, tol);
    assert!(
        second.reused > 0,
        "a prior of the same drive's table is read"
    );
}

/// A family of three cylinders: a circle extruded `1 + k/2`, under the
/// index `k`. Each member is an extrude of its own, and registers.
fn family_of_cylinders(tol: Tol) -> (ProfileDoc, editor_core::RecipeNodeId) {
    use editor_core::{Formula, LoopProgram, Node, ProfileProgram};
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive("session-state-family"), tol);
    let (doc, k) = crate::fixture::index(doc, "k", 3);
    let (doc, profile) = crate::fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [crate::fixture::len(0.0), crate::fixture::len(0.0)],
                radius: crate::fixture::len(1.0),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let grown =
        Formula::mul(crate::fixture::scalar_of(k), crate::fixture::len(0.5)).expect("a length");
    let distance = Formula::add(crate::fixture::len(1.0), grown).expect("a length");
    crate::fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance,
        },
    )
}

/// **A family whose members register carries their identities.** Three
/// extruded cylinders under one index: each member's extrude registers,
/// and the family's record holds all of them, in member order.
///
/// Mutant: a family value built with an empty record carries none.
#[test]
fn a_family_whose_members_register_carries_their_identities() {
    let tol = Tol::witness();
    let (doc, family) = family_of_cylinders(tol);
    let (ev, session) = one_leaf(&doc, None, None, &table(), tol);
    let Some(NodeResult::Ok(value)) = ev.result(family) else {
        panic!("the family builds: {:?}", ev.result(family).map(|_| ()));
    };
    let members = value.members.as_ref().expect("the extrude is a family");
    let of_members: usize = members
        .members
        .iter()
        .map(|(_, member)| member.sym.registrations())
        .sum();
    assert!(of_members > 0, "the members registered");
    assert_eq!(
        value.sym.registrations(),
        of_members,
        "the family holds them all"
    );
    assert_eq!(session, carried(&ev).0, "the receipt is the sum");
}

/// **Two instances of one part in one document read one run.** The
/// part is built once in the replay, and both instance nodes carry the
/// identities of that one run, whatever each is mated to.
///
/// Mutant: a part run counted at each instance doubles the part's
/// decisions in the receipt.
#[test]
fn two_instances_of_one_part_read_one_run() {
    let tol = Tol::witness();
    let part = cylinder("session-state-part", tol);
    let pin = editor_core::content_pin(&part, tol).expect("the pin computes");
    let doc_ref = editor_core::DocRef { id: part.id(), pin };
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive("session-state-two"), tol);
    let (doc, first) = crate::fixture::insert(doc, crate::fixture::mated_instance(doc_ref));
    let (doc, second) = crate::fixture::insert(doc, crate::fixture::mated_instance(doc_ref));
    let doc = crate::fixture::assemble_world_copies(doc);
    let (ev, session) = one_leaf(&doc, Some(Arc::new(OneDoc(part))), None, &table(), tol);
    assert_eq!(ev.parts_sym.len(), 1, "one part run for two instances");
    let identities = |id| match ev.result(id) {
        Some(NodeResult::Ok(v)) => v.sym.registrations(),
        _ => panic!("the instance builds"),
    };
    assert!(
        identities(first) > 0,
        "the instance carries the part's identities"
    );
    assert_eq!(
        identities(first),
        identities(second),
        "both read the one run"
    );
    assert_eq!(session, carried(&ev).0, "the part is counted once");
}
