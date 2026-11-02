//! **An instance carries its part's unproven coincidences** (D10
//! `[assertions]`, `[placement.world]`; ASSEMBLY A5): a part's unproven
//! row is the part's finding, judged by the part's own door over the
//! part's document and quiet only by the part's assertions. The report
//! over a document instancing it carries it once per part row, by its
//! route, shown at the route's first instance.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on};
use crate::fixture::resolver::{PartStore, with_resolver};
use crate::fixture::{fname, insert, insert_pinned, len};

use editor_core::{
    AssertionRelation, CapEnd, CarriedCoincidence, CheckEvidence, CheckFinding, CheckId,
    ChecksConfig, EvalOptions, FindingSubject, Frame, MeasurePrimitive, Node, Operand, ProfileDoc,
    RecipeNodeId, RoleSeg, SitedRef,
};
use geom_core::Tol;

/// A document labelled `text` at `node`.
fn labelled(doc: ProfileDoc, node: RecipeNodeId, text: &str) -> ProfileDoc {
    crate::fixture::step(
        doc,
        editor_core::DocEdit::SetLabel {
            node,
            label: Some(editor_core::Label::new(text).expect("a valid label")),
        },
    )
    .0
}

/// **A plate resting on a block, unioned**, its two extrudes labelled
/// `base-block` and `rest-plate`: the union's `Rest` of the block's top
/// and the plate's bottom, two constructions, and the plate's four
/// corners on that top are unproven.
fn resting_part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, plate) = block_on(doc, base, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let plate_extrude = crate::emit_shared_rim_several::construction(&doc, plate);
    let doc = labelled(doc, base, "base-block");
    let doc = labelled(doc, plate_extrude, "rest-plate");
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), plate.into()]),
        },
    );
    crate::fixture::place(doc, union).0
}

/// **A box split in two and reunited**: every row of the union lies on
/// the split's one tool plane, and the door proves each.
fn reunited_part() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("carried-coincidence-reunite", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let tool = crate::fixture::plane_on(&doc, the_box, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: the_box.into(),
            tool,
        },
    );
    let half = |half: editor_core::SplitHalf| {
        doc.output(split, half.port())
            .expect("a split defines a port per half")
    };
    let members = vec![
        half(editor_core::SplitHalf::Above).into(),
        half(editor_core::SplitHalf::Below).into(),
    ];
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(members),
        },
    );
    crate::fixture::place(doc, union).0
}

/// **Two blocks resting by values, each its own world copy**: the
/// census's contact of the big block's top and the small one's bottom
/// is unproven. `(doc, big copy, its top, small copy, its bottom)`.
fn stacked_copies(label: &str) -> (ProfileDoc, SitedRef, SitedRef) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, big) = block(doc, (-0.5, 0.5), (-0.5, 0.5), 1.0);
    let (doc, small) = block(doc, (-0.3, 0.3), (-0.3, 0.3), 0.5);
    let (doc, p) = crate::fixture::place(doc, big);
    let (doc, q) = crate::fixture::in_world(doc, small, &Frame::translation([0.0, 0.0, 1.0]));
    let top = SitedRef::new(p, fname(big, RoleSeg::Cap(CapEnd::End)));
    let bottom = SitedRef::new(q, fname(small, RoleSeg::Cap(CapEnd::Start)));
    (crate::fixture::assemble_world_copies(doc), top, bottom)
}

/// A `Gap(outer, inner) = 0` in `doc`: the document after it, and the
/// assertion.
fn gap_zero<R: Clone + Into<Operand>>(
    doc: &ProfileDoc,
    outer: R,
    inner: R,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer, inner],
    );
    let value = crate::fixture::value_of(&doc, measure);
    insert(
        doc,
        Node::Assertion(editor_core::Claim::Value {
            value,
            bound: len(0.0),
            relation: AssertionRelation::Equal,
        }),
    )
}

/// A document instancing `part` once per `at`, each pinned to the world
/// there; the instances, and the options that resolve every document
/// `store` holds and `part`.
fn host(
    label: &str,
    mut store: PartStore,
    part: &ProfileDoc,
    at: &[[f64; 3]],
) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions) {
    let part = crate::fixture::assemble_world_copies(part.clone());
    let doc_ref = store.insert(part, Tol::witness());
    let opts = with_resolver(store);
    let mut doc = ProfileDoc::empty_derived(label, Tol::witness());
    let mut ids = Vec::new();
    for &along in at {
        let (next, id) = insert(doc, crate::fixture::mated_instance(doc_ref));
        doc = crate::fixture::pin(next, id, &Frame::translation(along), &opts);
        ids.push(id);
    }
    (doc, ids, opts)
}

/// The `unproven-coincidence` findings of the checks over `doc`.
fn unproven(doc: &ProfileDoc, opts: &EvalOptions) -> Vec<CheckFinding> {
    let run = crate::fixture::run(doc, opts);
    editor_core::run_checks(doc, &run, &ChecksConfig::default(), Tol::witness())
        .unwrap_or_else(|e| panic!("the checks run: {e}"))
        .findings
        .into_iter()
        .filter(|f| f.check == CheckId::UnprovenCoincidence)
        .collect()
}

/// Each finding's carried row, or a panic naming the one that is not.
fn carried(findings: &[CheckFinding]) -> Vec<&CarriedCoincidence> {
    findings
        .iter()
        .map(|f| match &f.evidence {
            CheckEvidence::CarriedCoincidence { finding } => &**finding,
            other => panic!("a part's row, carried: {other:?}"),
        })
        .collect()
}

/// **A part's unproven `Rest` is reported through its instance, as the
/// part's.** The part reports its union's five rows unproven; the host
/// instancing it reports each once, the part's own finding verbatim, by
/// the route through the instance and shown there, its cells said by
/// the part's labels. Instanced twice, it is still reported once.
///
/// *Breaks if* the instance drops the part's rows (the host's lint is
/// silent, and `Certified` reads that as proven), re-decides them, or
/// reports them once per instance.
#[test]
fn a_parts_unproven_rest_is_reported_through_its_instance() {
    let part = resting_part("carried-coincidence-rest");
    let part_in_world = crate::fixture::assemble_world_copies(part.clone());
    let own = unproven(&part_in_world, &EvalOptions::default());
    assert_eq!(
        own.len(),
        5,
        "the premise: the rest and its four corners: {own:?}"
    );
    for n in [1, 2] {
        let at: Vec<[f64; 3]> = (0..n).map(|i| [4.0 * f64::from(i), 0.0, 0.0]).collect();
        let (doc, ids, opts) = host("carried-coincidence-host", PartStore::new(), &part, &at);
        let found = unproven(&doc, &opts);
        let rows = carried(&found);
        assert_eq!(
            rows.iter().map(|r| &r.finding).collect::<Vec<_>>(),
            own.iter().collect::<Vec<_>>(),
            "{n} instance(s): the part's own findings, each once"
        );
        for (finding, row) in found.iter().zip(&rows) {
            assert_eq!(
                (row.route.through, row.route.of, row.route.via.len()),
                (ids[0], part.id(), 0),
                "by the first instance, of the part"
            );
            assert_eq!(finding.subject, FindingSubject::Node(ids[0]), "shown there");
            assert_eq!(finding.quiet, None, "a union's row is loud");
        }
        let said = editor_core::spoken_by(&found[0], &doc);
        assert!(
            said.contains("base-block")
                && said.contains("rest-plate")
                && said.contains("leaves a coincidence of its own unproven, at Union"),
            "the caps said by the part's labels, at the part's union: {said}"
        );
        assert!(
            said.ends_with(&format!(
                "the row is the part's, so act in document {}: {}",
                part.id(),
                editor_core::spoken_by(&own[0], &part_in_world)
                    .rsplit_once(" — ")
                    .expect("the part's own finding ends in its recourse")
                    .1
            )),
            "its recourse is the part's own, pointed at the part: {said}"
        );
        let debug = format!("{:?}", rows[0]);
        assert!(
            debug.contains(&format!("PinnedDoc({})", part.id())) && !debug.contains("mint"),
            "the carried row's Debug names the part's document, not its body: {debug}"
        );
    }
}

/// **A part's proven rows report nothing through its instance.**
///
/// *Breaks if* the host re-decides a row the part's door proved.
#[test]
fn a_parts_proven_rows_report_nothing_through_its_instance() {
    let part = reunited_part();
    let own = unproven(
        &crate::fixture::assemble_world_copies(part.clone()),
        &EvalOptions::default(),
    );
    assert!(
        own.is_empty(),
        "the premise: the part's door proves every row: {own:?}"
    );
    let (doc, _, opts) = host(
        "carried-coincidence-proven",
        PartStore::new(),
        &part,
        &[[0.0; 3]],
    );
    let found = unproven(&doc, &opts);
    assert!(found.is_empty(), "nothing of the part's: {found:?}");
}

/// **A carried contact is quiet by the part's assertion and by nothing
/// of the host's.** Two blocks resting as two copies of a part: the
/// part's census contact is unproven. A host `Gap = 0` over picks of the
/// instance's two pieces, the same two caps, holds and quiets nothing:
/// the row is the part's. The part's own `Gap = 0` quiets it, and the
/// carried finding names that assertion, in the part's ids.
///
/// *Breaks if* a host assertion quiets a part's row, or the part's
/// quiet is lost at the seam.
#[test]
fn a_carried_contact_is_quiet_by_the_parts_assertion_alone() {
    let (part, top, bottom) = stacked_copies("carried-coincidence-contact");
    let copy = |at: &SitedRef| part.output(at.at, 0).expect("a copy");
    let (doc, ids, opts) = host(
        "carried-coincidence-picked",
        PartStore::new(),
        &part,
        &[[0.0; 3]],
    );
    let piece = |at: &SitedRef| {
        Operand::select(
            Operand::pick(Operand::Node(ids[0]), vec![copy(at)]),
            vec![at.name.clone()],
        )
    };
    let (doc, assertion) = gap_zero(&doc, piece(&top), piece(&bottom));
    let run = crate::fixture::run(&doc, &opts);
    assert!(
        matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(editor_core::ValuePayload::Assertion(
                editor_core::AssertionVerdict::Holds { .. }
            ))
        ),
        "the premise: the host's Gap over the picks holds: {:?}",
        run.value(assertion).map(|v| &v.payload)
    );
    let found = unproven(&doc, &opts);
    let contacts: Vec<_> = found
        .iter()
        .filter(|f| {
            matches!(&f.evidence, CheckEvidence::CarriedCoincidence { finding }
                if matches!(finding.finding.subject, FindingSubject::Copies { .. }))
        })
        .collect();
    assert!(
        !contacts.is_empty(),
        "the part's contact, carried: {found:?}"
    );
    assert!(
        contacts.iter().all(|f| f.quiet.is_none()),
        "loud: the host's assertion speaks for the host's own rows: {contacts:?}"
    );

    let (quieted, own_assertion) = gap_zero(&part, top, bottom);
    let (doc, _, opts) = host(
        "carried-coincidence-quiet",
        PartStore::new(),
        &quieted,
        &[[0.0; 3]],
    );
    let found = unproven(&doc, &opts);
    let contacts: Vec<_> = found
        .iter()
        .filter_map(|f| match &f.evidence {
            CheckEvidence::CarriedCoincidence { finding }
                if matches!(finding.finding.subject, FindingSubject::Copies { .. }) =>
            {
                Some((f, finding))
            }
            _ => None,
        })
        .collect();
    assert!(
        !contacts.is_empty(),
        "the part's contact, carried: {found:?}"
    );
    for (f, row) in contacts {
        assert_eq!(
            (f.quiet, row.finding.quiet),
            (Some(own_assertion), Some(own_assertion)),
            "quiet by the part's own assertion"
        );
        let said = editor_core::spoken_by(f, &doc);
        assert!(
            said.contains(", at copies ") && said.contains("quiet: the part's Assertion"),
            "at the part's copies, quiet by the part's assertion, said from the part: {said}"
        );
    }
}

/// **A part two documents down is reported once, by its whole route.**
/// A middle document instances the resting part, and a top document
/// instances the middle: the top reports the part's five rows once
/// each, `of` the part, through the top's instance `via` the middle's,
/// as the part's own door judged them.
///
/// *Breaks if* a carried row stops at the first seam, or is reported
/// again at each level.
#[test]
fn a_part_two_documents_down_is_reported_once_by_its_route() {
    let part = resting_part("carried-coincidence-deep-part");
    let own = unproven(
        &crate::fixture::assemble_world_copies(part.clone()),
        &EvalOptions::default(),
    );
    let mut store = PartStore::new();
    let part_ref = store.insert(
        crate::fixture::assemble_world_copies(part.clone()),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let mid = ProfileDoc::empty_derived("carried-coincidence-deep-mid", Tol::witness());
    let (mid, inner) = insert_pinned(mid, crate::fixture::mated_instance(part_ref), &opts);
    let mut store = PartStore::new();
    store.insert(
        crate::fixture::assemble_world_copies(part.clone()),
        Tol::witness(),
    );
    let (top, ids, opts) = host("carried-coincidence-deep-top", store, &mid, &[[0.0; 3]]);
    let found = unproven(&top, &opts);
    let rows = carried(&found);
    assert_eq!(
        rows.iter().map(|r| &r.finding).collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
        "the part's own findings, each once"
    );
    for row in rows {
        assert_eq!(
            (
                row.route.through,
                row.route.of,
                row.route
                    .via
                    .iter()
                    .map(editor_core::SpokenNode::id)
                    .collect::<Vec<_>>()
            ),
            (ids[0], part.id(), vec![inner]),
            "of the part, through the top's instance, via the middle's"
        );
    }
}

/// **A leaf under two middles is reported once, by the first route.**
/// Two middle documents each instance the resting part, and a top
/// instances both: the leaf's five rows are reported once, through the
/// first middle's instance, with or without a product over the top.
///
/// *Breaks if* a part row reached by two routes is reported once per
/// route.
#[test]
fn a_leaf_under_two_middles_is_reported_once() {
    let leaf = resting_part("carried-coincidence-two-mids-leaf");
    let own = unproven(
        &crate::fixture::assemble_world_copies(leaf.clone()),
        &EvalOptions::default(),
    );
    let mut store = PartStore::new();
    let leaf_ref = store.insert(
        crate::fixture::assemble_world_copies(leaf.clone()),
        Tol::witness(),
    );
    let opts = with_resolver(store.clone());
    let mid = |label: &str| {
        let doc = ProfileDoc::empty_derived(label, Tol::witness());
        insert_pinned(doc, crate::fixture::mated_instance(leaf_ref), &opts)
    };
    let (first, inner) = mid("carried-coincidence-two-mids-a");
    let (second, _) = mid("carried-coincidence-two-mids-b");
    let first = store.insert(first, Tol::witness());
    let second = store.insert(second, Tol::witness());
    let opts = with_resolver(store);
    let top = ProfileDoc::empty_derived("carried-coincidence-two-mids-top", Tol::witness());
    let (top, a) = insert(top, crate::fixture::mated_instance(first));
    let top = crate::fixture::pin(top, a, &Frame::translation([0.0; 3]), &opts);
    let (top, b) = insert(top, crate::fixture::mated_instance(second));
    let top = crate::fixture::pin(top, b, &Frame::translation([9.0, 0.0, 0.0]), &opts);
    for top in [top.clone(), crate::fixture::assemble_world_copies(top)] {
        let found = unproven(&top, &opts);
        let rows = carried(&found);
        assert_eq!(
            rows.iter().map(|r| &r.finding).collect::<Vec<_>>(),
            own.iter().collect::<Vec<_>>(),
            "the leaf's own findings, once"
        );
        for row in rows {
            assert_eq!(
                (
                    row.route.through,
                    row.route
                        .via
                        .iter()
                        .map(editor_core::SpokenNode::id)
                        .collect::<Vec<_>>()
                ),
                (a, vec![inner]),
                "through the first middle"
            );
        }
    }
}

/// **An instance a host boolean consumes still reports its part's rows.**
/// The resting part's instance, unioned with a far block: the part's
/// five rows, once each, and nothing of the host's own.
///
/// *Breaks if* the rows are read only off the product's sources, which
/// a boolean's value does not carry.
#[test]
fn an_instance_under_a_boolean_still_reports_its_parts_rows() {
    let part = resting_part("carried-coincidence-boolean-part");
    let own = unproven(
        &crate::fixture::assemble_world_copies(part.clone()),
        &EvalOptions::default(),
    );
    let mut store = PartStore::new();
    let part_ref = store.insert(
        crate::fixture::assemble_world_copies(part.clone()),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty_derived("carried-coincidence-boolean-host", Tol::witness());
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
    assert_eq!(
        carried(&found)
            .iter()
            .map(|r| &r.finding)
            .collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
        "the part's own findings, once, and none of the host's"
    );
}

/// **A partly proven part carries exactly its own verdict.** One union
/// in the part is the unproven rest, another the reunited split, which
/// its door proves: the host carries the part's own findings and no
/// more.
///
/// *Breaks if* the host re-decides a proven row, or loses an unproven
/// one beside it.
#[test]
fn a_partly_proven_part_carries_its_own_verdict() {
    let doc = ProfileDoc::empty_derived("carried-coincidence-partly", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, plate) = block_on(doc, base, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let (doc, rest) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), plate.into()]),
        },
    );
    let (doc, _) = crate::fixture::place(doc, rest);
    let (doc, the_box) = block(doc, (5.0, 6.0), (0.0, 1.0), 1.0);
    let tool = crate::fixture::plane_on(&doc, the_box, [5.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: the_box.into(),
            tool,
        },
    );
    let half = |half: editor_core::SplitHalf| {
        doc.output(split, half.port())
            .expect("a split defines a port per half")
    };
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
    let own = unproven(
        &crate::fixture::assemble_world_copies(part.clone()),
        &EvalOptions::default(),
    );
    assert!(
        !own.is_empty()
            && own
                .iter()
                .all(|f| f.subject != FindingSubject::Node(reunited)),
        "the premise: the rest is unproven and the reunion proven: {own:?}"
    );
    let (doc, _, opts) = host(
        "carried-coincidence-partly-host",
        PartStore::new(),
        &part,
        &[[0.0; 3]],
    );
    let found = unproven(&doc, &opts);
    assert_eq!(
        carried(&found)
            .iter()
            .map(|r| &r.finding)
            .collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
        "the part's own verdict, finding for finding"
    );
}

/// **A carried census row is the part's verdict, and none is the
/// host's.** Two blocks resting as two copies of a part, loud and then
/// quieted by the part's own `Gap = 0`: every unproven-coincidence
/// finding of the host is a carried one, equal to the part's own,
/// finding for finding and quiet for quiet, with or without a product
/// over the host.
///
/// *Breaks if* the host's census re-finds the contact inside the
/// instance's copy as a row of its own beside the carried one.
#[test]
fn a_carried_census_row_is_the_parts_verdict_and_none_is_the_hosts() {
    let (part, top, bottom) = stacked_copies("carried-coincidence-census-part");
    let (quieted, _) = gap_zero(&part, top, bottom);
    for (label, part) in [("loud", part), ("quiet", quieted)] {
        let own = unproven(
            &crate::fixture::assemble_world_copies(part.clone()),
            &EvalOptions::default(),
        );
        assert!(!own.is_empty(), "{label}: the premise: the part's contact");
        let (doc, _, opts) = host(
            "carried-coincidence-census-host",
            PartStore::new(),
            &part,
            &[[0.0; 3]],
        );
        for doc in [doc.clone(), crate::fixture::assemble_world_copies(doc)] {
            let found = unproven(&doc, &opts);
            let rows = carried(&found);
            assert_eq!(
                rows.iter().map(|r| &r.finding).collect::<Vec<_>>(),
                own.iter().collect::<Vec<_>>(),
                "{label}: the part's verdict, finding for finding"
            );
            for (f, row) in found.iter().zip(&rows) {
                assert_eq!(f.quiet, row.finding.quiet, "{label}: quiet as the part's");
            }
        }
    }
}

/// A resolver holding several versions of one document, by pin, that
/// counts its resolves.
#[derive(Debug, Default)]
struct Versions {
    docs:
        std::collections::BTreeMap<(editor_core::DocumentId, editor_core::ContentPin), ProfileDoc>,
    resolves: std::sync::atomic::AtomicUsize,
}

impl Versions {
    fn insert(&mut self, doc: ProfileDoc) -> editor_core::DocRef {
        let pin = editor_core::content_pin(&doc, Tol::witness()).expect("a pin");
        let id = doc.id();
        self.docs.insert((id, pin), doc);
        editor_core::DocRef { id, pin }
    }

    fn resolves(&self) -> usize {
        self.resolves.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl editor_core::PartResolver for Versions {
    fn resolve(
        &self,
        r: &editor_core::DocRef,
        _tol: Tol,
    ) -> Result<ProfileDoc, editor_core::ResolveFailure> {
        self.resolves
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.docs
            .get(&(r.id, r.pin))
            .cloned()
            .ok_or_else(|| editor_core::ResolveFailure {
                fault: editor_core::ResolveFault::Unresolved,
                message: "not held".into(),
            })
    }
}

/// The options resolving through `versions`.
fn through(versions: std::sync::Arc<Versions>) -> EvalOptions {
    EvalOptions {
        resolver: Some(versions),
        ..EvalOptions::default()
    }
}

/// The resting part with its base labelled `base`, in the world.
fn resting_labelled(id: &str, base: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, block_node) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, plate) = block_on(doc, block_node, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let doc = labelled(doc, block_node, base);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![block_node.into(), plate.into()]),
        },
    );
    crate::fixture::assemble_world_copies(crate::fixture::place(doc, union).0)
}

/// **Two pins of one document are two part rows, each said from its own
/// version.** One document at two versions, labelled apart, instanced
/// once each: five rows per pin, each judged over and said from the
/// version its pin fixes.
///
/// *Breaks if* two pins fold into one row, or a row is said from the
/// other version's labels.
#[test]
fn two_pins_are_two_rows_each_said_from_its_own_version() {
    let mut versions = Versions::default();
    let one = versions.insert(resting_labelled("carried-coincidence-pins", "base-one"));
    let two = versions.insert(resting_labelled("carried-coincidence-pins", "base-two"));
    assert_ne!(one.pin, two.pin, "the premise: two versions");
    let opts = through(std::sync::Arc::new(versions));
    let doc = ProfileDoc::empty_derived("carried-coincidence-pins-host", Tol::witness());
    let (doc, a) = insert(doc, crate::fixture::mated_instance(one));
    let doc = crate::fixture::pin(doc, a, &Frame::translation([0.0; 3]), &opts);
    let (doc, b) = insert(doc, crate::fixture::mated_instance(two));
    let doc = crate::fixture::pin(doc, b, &Frame::translation([9.0, 0.0, 0.0]), &opts);
    let found = unproven(&doc, &opts);
    let rows = carried(&found);
    assert_eq!(rows.len(), 10, "five per pin: {found:?}");
    for (f, row) in found.iter().zip(&rows) {
        assert_eq!(
            editor_core::content_pin(row.doc.get(), Tol::witness()).expect("a pin"),
            row.pin,
            "said from the pinned version"
        );
        let (pin, want, not) = if row.route.through == a {
            (one.pin, "base-one", "base-two")
        } else {
            (two.pin, "base-two", "base-one")
        };
        assert_eq!(row.pin, pin, "the instance's own pin");
        let said = editor_core::spoken_by(f, &doc);
        assert!(said.contains(want) && !said.contains(not), "{want}: {said}");
    }
}

/// The report over `doc` with its door left to run, and the door.
fn split_door(
    doc: &ProfileDoc,
    opts: &EvalOptions,
) -> (editor_core::ChecksReport, editor_core::DoorJob) {
    let ev = crate::fixture::run(doc, opts);
    let product = editor_core::product_recorded(doc, &ev, Tol::witness()).expect("it gathers");
    let (report, job) = editor_core::run_checks_at_rest_on(
        doc,
        &ev,
        editor_core::Subject::Product(&product),
        &ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run");
    (report, job.expect("the rows ask for the door"))
}

/// How many of `report`'s findings are carried coincidences.
fn carried_count(report: &editor_core::ChecksReport) -> usize {
    report
        .findings
        .iter()
        .filter(|f| matches!(f.evidence, CheckEvidence::CarriedCoincidence { .. }))
        .count()
}

/// **A door job whose rows are all carried is pending, cancelable, and
/// settles the part's findings.**
///
/// *Breaks if* a host with no rows of its own reports its parts' rows
/// as checked without running the door, or the carried replay ignores
/// the token.
#[test]
fn a_carried_door_job_is_pending_cancelable_and_settles() {
    let part = resting_part("carried-coincidence-cancel-part");
    let (doc, _, opts) = host(
        "carried-coincidence-cancel-host",
        PartStore::new(),
        &part,
        &[[0.0; 3]],
    );
    let doc = crate::fixture::assemble_world_copies(doc);
    let (mut report, job) = split_door(&doc, &opts);
    assert_eq!(
        report.pending,
        vec![CheckId::UnprovenCoincidence],
        "pending"
    );
    let canceled = editor_core::CancelToken::new();
    canceled.cancel();
    assert!(
        job.run(&doc, &canceled).is_err(),
        "a canceled job answers Canceled"
    );
    report.settle(
        job.run(&doc, &editor_core::CancelToken::new())
            .expect("an uncanceled door"),
    );
    assert!(report.pending.is_empty(), "settled");
    assert_eq!(carried_count(&report), 5, "the part's five");
}

/// **A part instanced twice is replayed once.** A part whose rows rest a
/// plate on an instance of another part replays that part each time its
/// door runs: the host instancing it twice resolves it as often as the
/// part's own door does.
///
/// *Breaks if* the host's door replays a part once per instance.
#[test]
fn a_part_instanced_twice_is_replayed_once() {
    let below = {
        let doc = ProfileDoc::empty_derived("carried-coincidence-replay-below", Tol::witness());
        let (doc, b) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
        crate::fixture::assemble_world_copies(crate::fixture::place(doc, b).0)
    };
    let mut versions = Versions::default();
    let below_ref = versions.insert(below.clone());
    let mut building = Versions::default();
    building.insert(below);
    let build = through(std::sync::Arc::new(building));
    let part = ProfileDoc::empty_derived("carried-coincidence-replay-part", Tol::witness());
    let (part, instance) = insert(part, crate::fixture::mated_instance(below_ref));
    let part = crate::fixture::pin(part, instance, &Frame::translation([0.0; 3]), &build);
    let (part, plate) = block(part, (0.5, 1.5), (0.5, 1.5), 0.5);
    let (part, plate) = crate::fixture::in_world(part, plate, &Frame::translation([0.0, 0.0, 1.0]));
    let (part, union) = insert(
        part,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                editor_core::BodyRead::plain(Operand::Node(instance)),
                plate.into(),
            ]),
        },
    );
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = versions.insert(part.clone());
    let versions = std::sync::Arc::new(versions);
    let opts = through(std::sync::Arc::clone(&versions));
    let ev = crate::fixture::run(&part, &opts);
    assert!(
        ev.value(union).is_some(),
        "the premise: the part's union evaluates: {:?}",
        ev.nodes.get(&union)
    );
    let (mut own, job) = split_door(&part, &opts);
    let before = versions.resolves();
    own.settle(
        job.run(&part, &editor_core::CancelToken::new())
            .expect("an uncanceled door"),
    );
    let own_resolves = versions.resolves() - before;
    let own_findings = own
        .findings
        .iter()
        .filter(|f| f.check == CheckId::UnprovenCoincidence)
        .count();
    let doc = ProfileDoc::empty_derived("carried-coincidence-replay-host", Tol::witness());
    let (doc, a) = insert(doc, crate::fixture::mated_instance(part_ref));
    let doc = crate::fixture::pin(doc, a, &Frame::translation([0.0; 3]), &opts);
    let (doc, b) = insert(doc, crate::fixture::mated_instance(part_ref));
    let doc = crate::fixture::pin(doc, b, &Frame::translation([0.0, 40.0, 0.0]), &opts);
    let doc = crate::fixture::assemble_world_copies(doc);
    let (mut report, job) = split_door(&doc, &opts);
    let before = versions.resolves();
    report.settle(
        job.run(&doc, &editor_core::CancelToken::new())
            .expect("an uncanceled door"),
    );
    let host_resolves = versions.resolves() - before;
    assert!(
        own_resolves > 0,
        "the premise: the part's replay resolves the part below"
    );
    assert_eq!(
        carried_count(&report),
        own_findings,
        "the part's findings, once"
    );
    assert_eq!(
        host_resolves, own_resolves,
        "one replay of the part, not one per instance"
    );
}
