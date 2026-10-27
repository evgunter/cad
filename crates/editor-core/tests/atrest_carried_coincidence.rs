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
        Node::Assertion {
            value,
            bound: len(0.0),
            relation: AssertionRelation::Equal,
        },
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
    let own = unproven(
        &crate::fixture::assemble_world_copies(part.clone()),
        &EvalOptions::default(),
    );
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
                && said.contains("leaves a coincidence of its own unproven"),
            "the caps said by the part's labels: {said}"
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

// ---- review probes (review/atrest-carried-nwQ4HW) ----

/// The part's own unproven findings, as its own `run_checks` reports them.
fn own_of(part: &ProfileDoc) -> Vec<CheckFinding> {
    unproven(
        &crate::fixture::assemble_world_copies(part.clone()),
        &EvalOptions::default(),
    )
}

/// Probe: a leaf instanced by two mids, both instanced by the host:
/// the leaf's rows reported once, via the first mid.
#[test]
fn probe_a_leaf_under_two_mids_is_reported_once() {
    let part = resting_part("probe-two-mids-leaf");
    let own = own_of(&part);
    let mut store = PartStore::new();
    let leaf = store.insert(
        crate::fixture::assemble_world_copies(part.clone()),
        Tol::witness(),
    );
    let opts = with_resolver(store.clone());
    let mid1 = ProfileDoc::empty_derived("probe-mid-1", Tol::witness());
    let (mid1, _) = insert_pinned(mid1, crate::fixture::mated_instance(leaf), &opts);
    let mid2 = ProfileDoc::empty_derived("probe-mid-2", Tol::witness());
    let (mid2, _) = insert_pinned(mid2, crate::fixture::mated_instance(leaf), &opts);
    let m1 = store.insert(mid1.clone(), Tol::witness());
    let m2 = store.insert(mid2.clone(), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty_derived("probe-two-mids-top", Tol::witness());
    let (doc, a) = insert(doc, crate::fixture::mated_instance(m1));
    let doc = crate::fixture::pin(doc, a, &Frame::translation([0.0; 3]), &opts);
    let (doc, b) = insert(doc, crate::fixture::mated_instance(m2));
    let doc = crate::fixture::pin(doc, b, &Frame::translation([5.0, 0.0, 0.0]), &opts);
    for doc in [doc.clone(), crate::fixture::assemble_world_copies(doc)] {
        let found = unproven(&doc, &opts);
        let rows = carried(&found);
        assert_eq!(
            rows.iter().map(|r| &r.finding).collect::<Vec<_>>(),
            own.iter().collect::<Vec<_>>(),
            "the leaf's own, once: {found:?}"
        );
        assert!(rows.iter().all(|r| r.route.through == a && r.route.of == part.id()));
    }
}

/// Probe: two instances unioned by the host (a boolean consumes them):
/// the part's rows still reported once each, and the host's own union
/// rows (if any) are the host's.
#[test]
fn probe_instances_consumed_by_a_boolean_still_carry() {
    let part = resting_part("probe-boolean-part");
    let own = own_of(&part);
    let (doc, ids, opts) = host(
        "probe-boolean-host",
        PartStore::new(),
        &part,
        &[[0.0; 3], [1.0, 0.0, 0.0]],
    );
    let (doc, _union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                crate::fixture::copy(ids[0], ids[0]),
                crate::fixture::copy(ids[1], ids[1]),
            ]),
        },
    );
    let found = unproven(&doc, &opts);
    let rows: Vec<_> = found
        .iter()
        .filter_map(|f| match &f.evidence {
            CheckEvidence::CarriedCoincidence { finding } => Some(&finding.finding),
            _ => None,
        })
        .collect();
    let hosts = found.len() - rows.len();
    eprintln!("probe boolean: {} carried, {hosts} host-own", rows.len());
    assert_eq!(rows, own.iter().collect::<Vec<_>>(), "{found:?}");
}

/// Probe: a part of two resting copies, loud and quieted by its own
/// `Gap = 0`: the host's carried findings equal the part's own
/// run_checks findings verbatim, and the host has no row of its own.
#[test]
fn probe_census_rows_equal_the_parts_verdict_and_none_are_the_hosts() {
    let (part, top, bottom) = stacked_copies("probe-census-part");
    let (quieted, _) = gap_zero(&part, top, bottom);
    for (label, part) in [("loud", part), ("quiet", quieted)] {
        let own = own_of(&part);
        let (doc, _, opts) = host("probe-census-host", PartStore::new(), &part, &[[0.0; 3]]);
        for doc in [doc.clone(), crate::fixture::assemble_world_copies(doc)] {
            let found = unproven(&doc, &opts);
            let rows = carried(&found);
            assert_eq!(
                rows.iter().map(|r| &r.finding).collect::<Vec<_>>(),
                own.iter().collect::<Vec<_>>(),
                "{label}: the part's verdict, finding for finding"
            );
            for (f, r) in found.iter().zip(&rows) {
                assert_eq!(f.quiet, r.finding.quiet, "{label}");
            }
        }
    }
}

/// Probe: the door job over carried rows alone is cancelable.
#[test]
fn probe_a_carried_only_door_job_cancels() {
    let part = resting_part("probe-cancel-part");
    let (doc, _, opts) = host("probe-cancel-host", PartStore::new(), &part, &[[0.0; 3]]);
    let ev = crate::fixture::run(&doc, &opts);
    let gathered =
        editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("the product gathers");
    let (_, job) = editor_core::run_checks_at_rest_on(
        &doc,
        &ev,
        editor_core::Subject::Product(&gathered),
        &ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run");
    let job = job.expect("the carried rows ask for the door");
    let token = editor_core::CancelToken::new();
    token.cancel();
    assert!(job.run(&doc, &token).is_err(), "a canceled token cancels the carried replay");
}

/// Probe (prints): the part's own sentence beside the host's carried one.
#[test]
fn probe_print_the_sentences() {
    let (part, top, bottom) = stacked_copies("probe-said-part");
    let (quieted, _) = gap_zero(&part, top, bottom);
    for part in [resting_part("probe-said-rest"), quieted] {
        let asm = crate::fixture::assemble_world_copies(part.clone());
        let own = own_of(&part);
        let (doc, _, opts) = host("probe-said-host", PartStore::new(), &part, &[[0.0; 3]]);
        let found = unproven(&doc, &opts);
        eprintln!("PART: {}", editor_core::spoken_by(&own[0], &asm));
        eprintln!("HOST: {}", editor_core::spoken_by(&found[0], &doc));
    }
}
