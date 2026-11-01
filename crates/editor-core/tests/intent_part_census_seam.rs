//! **A part's census findings cross the seam with it**
//! (`work/intent/a-part-drops-its-census-findings-at-the-seam.md`).
//!
//! A part's product crosses into the document that instantiates it as
//! one copy, and the census over that document's product finds the
//! part's own findings again inside the copy. The part's verdict
//! crosses with it instead, and each finding is reported once, as the
//! part's, by the at-rest check: an overlap between its copies as a
//! carried interference finding, and a pair it could not look at as a
//! carried could-not-look finding, neither ever a refusal (D10). The
//! census's re-finding inside the copy is not a second finding where it
//! names the pair it is about.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::resolver::{PartStore, with_resolver};
use crate::fixture::{in_world, insert, insert_pinned, len, on_frame, pin, square};
use editor_core::{
    AtRestEvidence, CancelToken, CarriedAtRestFinding, CarriedInterference, CheckEvidence,
    EvalOptions, Evaluation, Frame, InterferenceFinding, Node, ProfileDoc, RecipeNodeId, evaluate,
};
use geom_core::Tol;

/// A block extruded `height` from a square of half-side `h` at
/// `(cx, cy)` on its own xy, its copy standing in the world `z` up the
/// world's z axis: the placement.
fn block(
    doc: ProfileDoc,
    (cx, cy, z): (f64, f64, f64),
    h: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, cy, h)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    );
    in_world(doc, body, &Frame::translation([0.0, 0.0, z]))
}

/// A part of two copies: a unit block, and a smaller block whose −x
/// wall stands at `x` (0.5 is flush with the first's +x wall), strictly
/// inside the first across y and z.
fn two_blocks(label: &str, x: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, _) = block(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
    block(doc, (x + 0.3, 0.0, 0.25), 0.3, 0.5).0
}

/// The two copies overlap in one slab, 0.05 deep: one interference
/// finding, which the part's own gate reports and does not refuse.
pub(crate) fn overlapping(label: &str) -> ProfileDoc {
    two_blocks(label, 0.45)
}

/// A gap inside the sliver band at the ambient ε: three ε, between the
/// band's zero (ε) and its escalation (10 ε).
fn in_band_gap() -> f64 {
    3.0 * Tol::witness().eps()
}

/// The two copies stand three ε apart, inside the sliver band: the faces
/// they stand off over read one carrier in band, one pair the census
/// cannot look at, which the part's own check reports.
pub(crate) fn in_band(label: &str) -> ProfileDoc {
    two_blocks(label, 0.5 + in_band_gap())
}

fn run(doc: &ProfileDoc, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness())
}

/// **What the at-rest check reports**, by kind.
#[derive(Debug)]
struct AtRest {
    interference: Vec<InterferenceFinding>,
    carried_interference: Vec<CarriedInterference>,
    could_not_look: Vec<AtRestEvidence>,
}

impl AtRest {
    /// What a part below could not look at, carried.
    fn carried_at_rest(&self) -> Vec<&CarriedAtRestFinding> {
        self.could_not_look
            .iter()
            .filter_map(|evidence| match evidence {
                AtRestEvidence::Carried(row) => Some(&**row),
                _ => None,
            })
            .collect()
    }
}

fn gate(doc: &ProfileDoc, opts: &EvalOptions) -> Result<AtRest, editor_core::ChecksError> {
    let findings = crate::fixture::at_rest(doc, &run(doc, opts))?;
    Ok(AtRest {
        interference: crate::fixture::interference(&findings),
        carried_interference: findings
            .iter()
            .filter_map(|f| match &f.evidence {
                CheckEvidence::CarriedInterference { finding } => Some((**finding).clone()),
                _ => None,
            })
            .collect(),
        could_not_look: crate::fixture::could_not_look(&findings),
    })
}

/// A document instantiating `part` `n` times, each copy pinned to the
/// world 3 m along x from the last; the instances, and the options that
/// resolve the part.
pub(crate) fn instantiating(
    label: &str,
    part: &ProfileDoc,
    n: usize,
) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions) {
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part.clone());
    let doc_ref = store.insert(part.clone(), Tol::witness());
    let opts = with_resolver(store);
    let mut doc = ProfileDoc::empty_derived(label, Tol::witness());
    let mut ids = Vec::new();
    for i in 0..n {
        let along = 3.0 * f64::from(u32::try_from(i).expect("a few instances"));
        let (next, id) = insert(doc, crate::fixture::mated_instance(doc_ref));
        doc = pin(next, id, &Frame::translation([along, 0.0, 0.0]), &opts);
        ids.push(id);
    }
    (doc, ids, opts)
}

/// **A part's overlap crosses the seam as the part's finding.** The
/// part's own gate reports one overlap between its copies. The document
/// instantiating it gates `Ok`: the census over its product finds the
/// overlap again inside the part's one copy, and that is the part's
/// finding, carried, not a refusal of this document's. It is reported
/// on `carried_interference`, equal to the part's own, by the route
/// through the instance.
///
/// *Breaks if* the part's verdict is dropped at the seam (the
/// instantiating document refuses the re-found overlap unattributed, as
/// it did), or the carried finding is not the part's.
#[test]
fn a_parts_overlap_crosses_the_seam_as_the_parts_finding() {
    let part = overlapping("seam-overlap-part");
    let own = gate(&part, &EvalOptions::default()).expect("the part's own check reports it");
    let [finding] = own.interference.as_slice() else {
        panic!("one overlap in the part: {:?}", own.interference)
    };
    let (doc, ids, opts) = instantiating("seam-overlap", &part, 1);
    let outer = gate(&doc, &opts).unwrap_or_else(|e| panic!("the outer check could not run: {e}"));
    assert!(
        outer.interference.is_empty(),
        "the overlap is inside one copy here: not this document's {:?}",
        outer.interference
    );
    let [carried] = outer.carried_interference.as_slice() else {
        panic!("the part's finding, once: {:?}", outer.carried_interference)
    };
    assert_eq!(
        &carried.finding, finding,
        "the part's own finding, verbatim"
    );
    assert_eq!(
        (
            carried.route.through,
            carried.route.of,
            carried.route.via.len()
        ),
        (ids[0], part.id(), 0),
        "by the route through the instance"
    );
}

/// **A part placed twice reports its overlap once.** Both copies hold
/// it and the census finds it inside each; it is one finding of one
/// part row, reported by the first instance's route.
///
/// *Breaks if* each instance reports the part's finding again, or the
/// second copy's re-found overlap is refused.
#[test]
fn a_part_placed_twice_reports_its_overlap_once() {
    let part = overlapping("seam-twice-part");
    let (doc, ids, opts) = instantiating("seam-twice", &part, 2);
    let outer = gate(&doc, &opts).unwrap_or_else(|e| panic!("the outer check could not run: {e}"));
    assert!(outer.interference.is_empty(), "{:?}", outer.interference);
    let [carried] = outer.carried_interference.as_slice() else {
        panic!("once per part row: {:?}", outer.carried_interference)
    };
    assert_eq!(carried.route.through, ids[0], "the first instance's route");
}

/// **An overlap two documents down carries up by its whole route.** A
/// sub-assembly instantiates the overlapping part, and a top document
/// instantiates the sub-assembly: the top reports the part's finding
/// once, `of` the part, `via` the sub-assembly's instance.
///
/// *Breaks if* a carried finding stops at the first seam, or is
/// reported again at each level.
#[test]
fn an_overlap_two_documents_down_carries_up_by_its_route() {
    let part = overlapping("seam-deep-part");
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part.clone(), Tol::witness());
    let opts = with_resolver(store);
    let mid = ProfileDoc::empty_derived("seam-deep-mid", Tol::witness());
    let (mid, inner) = insert_pinned(mid, crate::fixture::mated_instance(part_ref), &opts);
    let mid_own =
        gate(&mid, &opts).unwrap_or_else(|e| panic!("the middle's check could not run: {e}"));
    assert_eq!(mid_own.carried_interference.len(), 1);
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    store.insert(part.clone(), Tol::witness());
    let mid = crate::fixture::assemble_world_copies(mid);
    let mid_ref = store.insert(mid, Tol::witness());
    let opts = with_resolver(store);
    let top = ProfileDoc::empty_derived("seam-deep-top", Tol::witness());
    let (top, outer) = insert_pinned(top, crate::fixture::mated_instance(mid_ref), &opts);
    let assembly =
        gate(&top, &opts).unwrap_or_else(|e| panic!("the top's check could not run: {e}"));
    assert!(
        assembly.interference.is_empty(),
        "{:?}",
        assembly.interference
    );
    let [carried] = assembly.carried_interference.as_slice() else {
        panic!("once: {:?}", assembly.carried_interference)
    };
    assert_eq!(
        (
            carried.route.through,
            carried.route.of,
            carried
                .route
                .via
                .iter()
                .map(editor_core::SpokenNode::id)
                .collect::<Vec<_>>()
        ),
        (outer, part.id(), vec![inner]),
        "of the part, through the top's instance, via the middle's"
    );
}

/// **What the part could not look at is reported as the part's.** The
/// part's two copies stand in the sliver band, so its census cannot look
/// at the faces they stand off over: its own check reports could-not-look
/// findings. The document instantiating it reports each, verbatim and by
/// the route through the instance, and refuses nothing. The census's
/// re-finding inside the copy names the copy's faces, so the in-arrival
/// discard drops it, and it is reported once.
///
/// *Breaks if* the carried findings are not the part's, the re-finding
/// is reported again as this document's, or a part's finding refuses
/// the instantiating document.
#[test]
fn what_a_part_could_not_look_at_is_reported_as_the_parts() {
    let part = in_band("seam-in-band-part");
    let own = gate(&part, &EvalOptions::default())
        .expect("the part's own check runs")
        .could_not_look;
    assert!(
        !own.is_empty(),
        "the premise: the part could not look: {own:?}"
    );
    let (doc, ids, opts) = instantiating("seam-in-band", &part, 1);
    let outer = gate(&doc, &opts).expect("nothing refuses");
    let findings = outer.carried_at_rest();
    assert_eq!(
        findings.len(),
        outer.could_not_look.len(),
        "every finding is the part's, once: {outer:?}"
    );
    assert_eq!(
        findings.iter().map(|f| &f.finding).collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
        "the part's own findings, verbatim, once each"
    );
    assert!(
        findings
            .iter()
            .all(|f| (f.route.through, f.route.of) == (ids[0], part.id())),
        "each by the route through the instance"
    );
    let line = findings[0].to_string();
    assert!(
        line.contains("reports what its own census could not look at"),
        "the sentence sends the author to the part: {line}"
    );
}

/// **An in-band pair two documents down is reported once, as the
/// part's, by its whole route.** A sub-assembly instantiates the in-band
/// part, and a top document instantiates the sub-assembly: the top
/// reports each of the part's findings once, by the route through the
/// sub-assembly. The sub-assembly's census finds the part's pair again
/// inside its copy and discards it there, so nothing of the
/// sub-assembly's own crosses.
///
/// *Breaks if* a carried finding stops at the first seam, or a
/// re-finding is reported as the sub-assembly's or the top's own.
#[test]
fn an_in_band_pair_two_documents_down_is_reported_once_as_the_parts() {
    let part = in_band("seam-deep-band-part");
    let own = gate(&part, &EvalOptions::default())
        .expect("the part's own check runs")
        .could_not_look;
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part.clone(), Tol::witness());
    let opts = with_resolver(store);
    let mid = ProfileDoc::empty_derived("seam-deep-band-mid", Tol::witness());
    let (mid, inner) = insert_pinned(mid, crate::fixture::mated_instance(part_ref), &opts);
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    store.insert(part.clone(), Tol::witness());
    let mid = crate::fixture::assemble_world_copies(mid);
    let mid_ref = store.insert(mid, Tol::witness());
    let opts = with_resolver(store);
    let top = ProfileDoc::empty_derived("seam-deep-band-top", Tol::witness());
    let (top, outer) = insert_pinned(top, crate::fixture::mated_instance(mid_ref), &opts);
    let reported = gate(&top, &opts).expect("nothing refuses");
    let findings = reported.carried_at_rest();
    assert_eq!(
        findings.len(),
        reported.could_not_look.len(),
        "nothing of the sub-assembly's or the top's own: {reported:?}"
    );
    assert_eq!(
        findings.iter().map(|f| &f.finding).collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
        "the part's findings, once each"
    );
    assert!(
        findings.iter().all(|f| (
            f.route.through,
            f.route.of,
            f.route
                .via
                .iter()
                .map(editor_core::SpokenNode::id)
                .collect::<Vec<_>>()
        ) == (outer, part.id(), vec![inner])),
        "each by the route through the sub-assembly"
    );
}

/// **A part in the world and in a space of its own reports its overlap
/// once.** One instance is pinned to the world and another placed into
/// a `new_space()`; the part's overlap inside its copies is one finding
/// of one part row, by the world instance's route. (The census reads
/// the world's copies alone until D′3 gathers a space's; either way the
/// row is one.)
///
/// *Breaks if* the world's and a space's carried rows are not read as
/// one list.
#[test]
fn a_part_in_the_world_and_a_loose_space_reports_its_overlap_once() {
    let part = overlapping("seam-space-part");
    let (doc, ids, opts) = instantiating("seam-space", &part, 1);
    let Some(Node::InstantiatePart { doc_ref, .. }) = doc.node(ids[0]) else {
        panic!("an instance")
    };
    let (doc, second) = insert(doc.clone(), crate::fixture::mated_instance(*doc_ref));
    let (doc, _) = crate::fixture::into_new_space(doc, second);
    let outer = gate(&doc, &opts).unwrap_or_else(|e| panic!("the outer check could not run: {e}"));
    assert!(outer.interference.is_empty(), "{:?}", outer.interference);
    let [carried] = outer.carried_interference.as_slice() else {
        panic!("once per part row: {:?}", outer.carried_interference)
    };
    assert_eq!(
        carried.route.through, ids[0],
        "the world instance's route, first"
    );
}

/// **A part placed twice reports what it could not look at once.** Both
/// copies carry the in-band part's finding up, and it is one part
/// row's findings, reported by the first instance's route.
///
/// *Breaks if* each instance reports the part's findings again (the
/// could-not-look arm of the once-per-part-row rule).
#[test]
fn a_part_placed_twice_reports_what_it_could_not_look_at_once() {
    let part = in_band("seam-band-twice-part");
    let own = gate(&part, &EvalOptions::default())
        .expect("the part's own check runs")
        .could_not_look;
    let (doc, ids, opts) = instantiating("seam-band-twice", &part, 2);
    let reported = gate(&doc, &opts).expect("nothing refuses");
    let findings = reported.carried_at_rest();
    assert_eq!(findings.len(), own.len(), "once per part row: {reported:?}");
    assert!(
        findings.iter().all(|f| f.route.through == ids[0]),
        "by the first instance's route"
    );
}

/// A resolver holding several versions of one document, each by its
/// pin.
#[derive(Debug, Default)]
struct Versions(std::collections::BTreeMap<editor_core::DocRef, ProfileDoc>);

impl Versions {
    fn insert(&mut self, doc: ProfileDoc) -> editor_core::DocRef {
        let doc_ref = editor_core::DocRef {
            id: doc.id(),
            pin: editor_core::content_pin(&doc, Tol::witness()).expect("the pin computes"),
        };
        self.0.insert(doc_ref, doc);
        doc_ref
    }
}

impl editor_core::PartResolver for Versions {
    fn resolve(
        &self,
        doc_ref: &editor_core::DocRef,
        _tol: Tol,
    ) -> Result<ProfileDoc, editor_core::ResolveFailure> {
        self.0
            .get(doc_ref)
            .cloned()
            .ok_or_else(|| editor_core::ResolveFailure {
                fault: editor_core::ResolveFault::Unresolved,
                message: "no such version".to_string(),
            })
    }
}

/// **Two pins of one document are two part rows.** One version of the
/// in-band part holds one in-band pair; a later version of the same
/// document adds a second pair beside it. A document instantiating both
/// reports each version's findings, whichever it places first: equal
/// findings of two versions are not one row.
///
/// *Breaks if* the once-per-part-row rule reads the document and not
/// its pin (the later version's findings equal to the earlier's are
/// dropped, and which depends on the order).
#[test]
fn two_pins_of_one_document_are_two_part_rows() {
    let v1 = in_band("seam-two-pins-part");
    let (v2, _) = block(v1.clone(), (10.0, 0.0, 0.0), 0.5, 1.0);
    let v2 = block(v2, (10.8 + in_band_gap(), 0.0, 0.25), 0.3, 0.5).0;
    let count = |doc: &ProfileDoc| {
        gate(doc, &EvalOptions::default())
            .expect("each version's check runs")
            .could_not_look
            .len()
    };
    let (n1, n2) = (count(&v1), count(&v2));
    assert!(n2 > n1, "the premise: the later version finds more");
    for order in [[&v1, &v2], [&v2, &v1]] {
        let mut store = Versions::default();
        let refs = order.map(|doc| store.insert(doc.clone()));
        let opts = EvalOptions {
            resolver: Some(std::sync::Arc::new(store)),
            ..EvalOptions::default()
        };
        let doc = ProfileDoc::empty_derived("seam-two-pins", Tol::witness());
        let (doc, _) = insert_pinned(doc, crate::fixture::mated_instance(refs[0]), &opts);
        let (doc, id) = insert(doc, crate::fixture::mated_instance(refs[1]));
        let doc = pin(doc, id, &Frame::translation([30.0, 0.0, 0.0]), &opts);
        let reported = gate(&doc, &opts).expect("nothing refuses");
        assert_eq!(
            reported.carried_at_rest().len(),
            n1 + n2,
            "every finding of both versions"
        );
    }
}
