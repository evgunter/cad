//! Review probes for PR #5871 (an instance carries its part's
//! unproven coincidences). Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::atrest_carried_coincidence::{host, labelled, resting_part, reunited_part, unproven};
use crate::blocks::{block, block_on};
use crate::fixture::resolver::{PartStore, with_resolver};
use crate::fixture::{insert, insert_pinned};

use editor_core::{
    CancelToken, CheckEvidence, CheckFinding, ChecksConfig, DocRef, DocumentId, EvalOptions, Frame,
    Node, Operand, PartResolver, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, Subject,
    content_pin,
};
use geom_core::Tol;

fn carried(findings: &[CheckFinding]) -> Vec<&editor_core::CarriedCoincidence> {
    findings
        .iter()
        .map(|f| match &f.evidence {
            CheckEvidence::CarriedCoincidence { finding } => &**finding,
            other => panic!("a part's row, carried: {other:?}"),
        })
        .collect()
}

fn own_of(part: &ProfileDoc, opts: &EvalOptions) -> Vec<CheckFinding> {
    unproven(&crate::fixture::assemble_world_copies(part.clone()), opts)
}

/// A leaf instanced by two mids, a top instancing both mids: the leaf's
/// rows once, by the first mid's route.
#[test]
fn probe_a_leaf_under_two_mids_is_reported_once() {
    let leaf = resting_part("probe-two-mids-leaf");
    let own = own_of(&leaf, &EvalOptions::default());
    assert_eq!(own.len(), 5);
    let mut store = PartStore::new();
    let leaf_ref = store.insert(
        crate::fixture::assemble_world_copies(leaf.clone()),
        Tol::witness(),
    );
    let opts = with_resolver(store.clone());
    let mid = |label: &str| {
        let m = ProfileDoc::empty_derived(label, Tol::witness());
        insert_pinned(m, crate::fixture::mated_instance(leaf_ref), &opts)
    };
    let (mid1, inner1) = mid("probe-two-mids-a");
    let (mid2, _inner2) = mid("probe-two-mids-b");
    let r1 = store.insert(mid1, Tol::witness());
    let r2 = store.insert(mid2, Tol::witness());
    let opts = with_resolver(store);
    let top = ProfileDoc::empty_derived("probe-two-mids-top", Tol::witness());
    let (top, i1) = insert(top, crate::fixture::mated_instance(r1));
    let top = crate::fixture::pin(top, i1, &Frame::translation([0.0; 3]), &opts);
    let (top, i2) = insert(top, crate::fixture::mated_instance(r2));
    let top = crate::fixture::pin(top, i2, &Frame::translation([9.0, 0.0, 0.0]), &opts);
    let found = unproven(&top, &opts);
    let rows = carried(&found);
    assert_eq!(
        rows.iter().map(|r| &r.finding).collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
        "the leaf's five, once"
    );
    for row in rows {
        assert_eq!(row.route.through, i1);
        assert_eq!(
            row.route
                .via
                .iter()
                .map(editor_core::SpokenNode::id)
                .collect::<Vec<_>>(),
            vec![inner1]
        );
    }
}

/// An instance a host boolean consumes (unioned with a far block) still
/// reports the part's rows, once.
#[test]
fn probe_an_instance_under_a_boolean_still_reports() {
    let part = resting_part("probe-boolean-part");
    let own = own_of(&part, &EvalOptions::default());
    let mut store = PartStore::new();
    let part_ref = store.insert(
        crate::fixture::assemble_world_copies(part.clone()),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty_derived("probe-boolean-host", Tol::witness());
    let (doc, instance) = insert_pinned(doc, crate::fixture::mated_instance(part_ref), &opts);
    let (doc, far) = block(doc, (10.0, 11.0), (0.0, 1.0), 1.0);
    let (doc, far) = crate::fixture::place(doc, far);
    let (doc, fused) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                editor_core::BodyRead::plain(Operand::Node(instance)),
                far.into(),
            ]),
        },
    );
    let run = crate::fixture::run(&doc, &opts);
    assert!(
        run.value(fused).is_some(),
        "the premise: the union evaluates: {:?}",
        run.nodes.get(&fused)
    );
    let found = unproven(&doc, &opts);
    let rows = carried(&found);
    assert_eq!(
        rows.iter().map(|r| &r.finding).collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
    );
}

/// A part with one unproven union and one proven one: the host carries
/// exactly the part's own findings.
#[test]
fn probe_a_partly_proven_part_carries_its_own_verdict() {
    let doc = ProfileDoc::empty_derived("probe-partly", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, plate) = block_on(doc, base, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), plate.into()]),
        },
    );
    let (doc, _) = crate::fixture::place(doc, union);
    let (doc, the_box) = block(doc, (5.0, 6.0), (0.0, 1.0), 1.0);
    let tool = crate::fixture::plane_on(&doc, the_box, [5.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: the_box.into(),
            tool,
        },
    );
    let half = |h: editor_core::SplitHalf| doc.output(split, h.port()).unwrap();
    let members = vec![
        half(editor_core::SplitHalf::Above).into(),
        half(editor_core::SplitHalf::Below).into(),
    ];
    let (doc, reunited) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(members),
        },
    );
    let (part, _) = crate::fixture::place(doc, reunited);
    let own = own_of(&part, &EvalOptions::default());
    // Sanity: the reunited half alone proves everything.
    assert!(own_of(&reunited_part(), &EvalOptions::default()).is_empty());
    assert!(!own.is_empty(), "the premise: something unproven");
    let (h, _, opts) = host("probe-partly-host", PartStore::new(), &part, &[[0.0; 3]]);
    let found = unproven(&h, &opts);
    assert_eq!(
        carried(&found)
            .iter()
            .map(|r| &r.finding)
            .collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>()
    );
}

/// A resolver holding several versions of one document, by pin, that
/// counts its resolves.
#[derive(Debug, Default)]
struct Versions {
    docs: BTreeMap<(DocumentId, editor_core::ContentPin), ProfileDoc>,
    count: AtomicUsize,
}

impl Versions {
    fn insert(&mut self, doc: ProfileDoc) -> DocRef {
        let pin = content_pin(&doc, Tol::witness()).unwrap();
        let id = doc.id();
        self.docs.insert((id, pin), doc);
        DocRef { id, pin }
    }
}

impl PartResolver for Versions {
    fn resolve(&self, r: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.count.fetch_add(1, Ordering::SeqCst);
        self.docs
            .get(&(r.id, r.pin))
            .cloned()
            .ok_or_else(|| ResolveFailure {
                fault: ResolveFault::Unresolved,
                message: "none".into(),
            })
    }
}

fn resting_labelled(id: &str, base_label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, plate) = block_on(doc, base, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let doc = labelled(doc, base, base_label);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), plate.into()]),
        },
    );
    crate::fixture::assemble_world_copies(crate::fixture::place(doc, union).0)
}

/// Two instances of one document at two pins: two part rows, each
/// judged over, and said from, its own version.
#[test]
fn probe_two_pins_are_two_rows_each_its_own_document() {
    let v1 = resting_labelled("probe-pins-part", "base-one");
    let v2 = resting_labelled("probe-pins-part", "base-two");
    let mut versions = Versions::default();
    let r1 = versions.insert(v1);
    let r2 = versions.insert(v2);
    assert_ne!(r1.pin, r2.pin);
    let opts = EvalOptions {
        resolver: Some(Arc::new(versions)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty_derived("probe-pins-host", Tol::witness());
    let (doc, i1) = insert(doc, crate::fixture::mated_instance(r1));
    let doc = crate::fixture::pin(doc, i1, &Frame::translation([0.0; 3]), &opts);
    let (doc, i2) = insert(doc, crate::fixture::mated_instance(r2));
    let doc = crate::fixture::pin(doc, i2, &Frame::translation([9.0, 0.0, 0.0]), &opts);
    let found = unproven(&doc, &opts);
    let rows = carried(&found);
    assert_eq!(rows.len(), 10, "five per pin: {found:?}");
    for (f, row) in found.iter().zip(&rows) {
        assert_eq!(
            content_pin(&*row.doc, Tol::witness()).unwrap(),
            row.pin,
            "the document said from is the pinned one"
        );
        let said = editor_core::spoken_by(f, &doc);
        let (want, not) = if row.route.through == i1 {
            assert_eq!(row.pin, r1.pin);
            ("base-one", "base-two")
        } else {
            assert_eq!((row.route.through, row.pin), (i2, r2.pin));
            ("base-two", "base-one")
        };
        assert!(
            said.contains(want) && !said.contains(not),
            "{want}: {said}"
        );
    }
}

fn job_of(
    doc: &ProfileDoc,
    opts: &EvalOptions,
) -> (editor_core::ChecksReport, editor_core::DoorJob) {
    let ev = crate::fixture::run(doc, opts);
    let product = editor_core::product_recorded(doc, &ev, Tol::witness()).unwrap();
    let (report, job) = editor_core::run_checks_at_rest_on(
        doc,
        &ev,
        Subject::Product(&product),
        &ChecksConfig::default(),
        Tol::witness(),
    )
    .unwrap();
    (report, job.expect("a door job"))
}

/// The door job is cancelable on a host whose only rows are carried.
#[test]
fn probe_the_door_job_cancels() {
    let part = resting_part("probe-cancel-part");
    let (doc, _, opts) = host("probe-cancel-host", PartStore::new(), &part, &[[0.0; 3]]);
    let doc = crate::fixture::assemble_world_copies(doc);
    let (mut report, job) = job_of(&doc, &opts);
    assert_eq!(report.pending, vec![editor_core::CheckId::UnprovenCoincidence]);
    let canceled = CancelToken::new();
    canceled.cancel();
    assert!(job.run(&doc, &canceled).is_err(), "a canceled job answers Canceled");
    report.settle(job.run(&doc, &CancelToken::new()).unwrap());
    assert!(report.pending.is_empty());
    assert_eq!(
        report
            .findings
            .iter()
            .filter(|f| matches!(f.evidence, CheckEvidence::CarriedCoincidence { .. }))
            .count(),
        5
    );
}

/// A part instanced twice is replayed once: P's rows rest a plate on an
/// instance of Q, so each replay of P resolves Q; the host's job resolves
/// Q as often as P's own job does.
#[test]
fn probe_the_door_job_replays_once_per_part() {
    let q = {
        let d = ProfileDoc::empty_derived("probe-replay-q", Tol::witness());
        let (d, b) = block(d, (0.0, 2.0), (0.0, 2.0), 1.0);
        crate::fixture::assemble_world_copies(crate::fixture::place(d, b).0)
    };
    let mut versions = Versions::default();
    let q_ref = versions.insert(q.clone());
    let mut build = Versions::default();
    build.insert(q);
    let bopts = EvalOptions {
        resolver: Some(Arc::new(build)),
        ..EvalOptions::default()
    };
    let p = ProfileDoc::empty_derived("probe-replay-p", Tol::witness());
    let (p, inst) = insert(p, crate::fixture::mated_instance(q_ref));
    let p = crate::fixture::pin(p, inst, &Frame::translation([0.0; 3]), &bopts);
    let (p, plate) = block(p, (0.5, 1.5), (0.5, 1.5), 0.5);
    let (p, plate) = crate::fixture::in_world(p, plate, &Frame::translation([0.0, 0.0, 1.0]));
    let (p, union) = insert(
        p,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                editor_core::BodyRead::plain(Operand::Node(inst)),
                plate.into(),
            ]),
        },
    );
    let p = crate::fixture::assemble_world_copies(p);
    let p_ref = versions.insert(p.clone());
    let versions = Arc::new(versions);
    let opts = EvalOptions {
        resolver: Some(Arc::clone(&versions) as Arc<dyn PartResolver>),
        ..EvalOptions::default()
    };
    let ev = crate::fixture::run(&p, &opts);
    assert!(ev.value(union).is_some(), "the premise: P's union evaluates: {:?}", ev.nodes.get(&union));
    let (_, job) = job_of(&p, &opts);
    let before = versions.count.load(Ordering::SeqCst);
    let own = job.run(&p, &CancelToken::new()).unwrap();
    let own_resolves = versions.count.load(Ordering::SeqCst) - before;
    let doc = ProfileDoc::empty_derived("probe-replay-host", Tol::witness());
    let (doc, i1) = insert(doc, crate::fixture::mated_instance(p_ref));
    let doc = crate::fixture::pin(doc, i1, &Frame::translation([0.0; 3]), &opts);
    let (doc, i2) = insert(doc, crate::fixture::mated_instance(p_ref));
    let doc = crate::fixture::pin(doc, i2, &Frame::translation([0.0, 40.0, 0.0]), &opts);
    let doc = crate::fixture::assemble_world_copies(doc);
    let (mut report, job) = job_of(&doc, &opts);
    let before = versions.count.load(Ordering::SeqCst);
    report.settle(job.run(&doc, &CancelToken::new()).unwrap());
    let host_resolves = versions.count.load(Ordering::SeqCst) - before;
    let carried_n = report
        .findings
        .iter()
        .filter(|f| matches!(f.evidence, CheckEvidence::CarriedCoincidence { .. }))
        .count();
    eprintln!("own resolves {own_resolves}, host resolves {host_resolves}, carried {carried_n}, own {}", own_len(&own));
    assert!(own_resolves > 0, "the premise: P's replay resolves Q");
    assert_eq!(host_resolves, own_resolves, "one replay of P, not one per instance");
    let _: RecipeNodeId = i2;
}

fn own_len(f: &editor_core::DoorFindings) -> String {
    format!("{f:?}").matches("check: UnprovenCoincidence").count().to_string()
}
