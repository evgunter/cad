//! **A part's census findings cross the seam with it**
//! (`work/intent/a-part-drops-its-census-findings-at-the-seam.md`).
//!
//! A part's product crosses into the document that instantiates it as
//! one copy, and the census over that document's product finds the
//! part's own findings again inside the copy. The part's verdict
//! crosses with it instead, and each finding is reported once, as the
//! part's: an overlap between its copies on
//! `Assembly::carried_interference`, never a refusal (D10), and a
//! finding its own gate refuses as `AssemblyError::CarriedAtRest`, raised
//! before the instantiating document's own verdict. The census's
//! re-finding inside the copy is not a second finding.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::resolver::{PartStore, with_resolver};
use crate::fixture::{insert, insert_pinned, len, on_frame, pin, place, square};
use editor_core::{
    Assembly, AssemblyError, CancelToken, EvalOptions, Evaluation, ExtrudeSide, Frame, Node,
    ProfileDoc, RecipeNodeId, assemble, evaluate,
};
use geom_core::Tol;

/// A block extruded `height` from a square of half-side `h` at
/// `(cx, cy)`, on a frame `z` up the world's z axis.
fn block(
    doc: ProfileDoc,
    (cx, cy, z): (f64, f64, f64),
    h: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, z],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, cy, h)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
            side: ExtrudeSide::Along,
        },
    )
}

/// A part of two copies: a unit block, and a smaller block whose −x
/// wall stands at `x` (0.5 is flush with the first's +x wall), strictly
/// inside the first across y and z.
fn two_blocks(label: &str, x: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
    let (doc, b) = block(doc, (x + 0.3, 0.0, 0.25), 0.3, 0.5);
    let (doc, _) = place(doc, a);
    place(doc, b).0
}

/// The two copies overlap in one slab, 0.05 deep: one interference
/// finding, which the part's own gate reports and does not refuse.
fn overlapping(label: &str) -> ProfileDoc {
    two_blocks(label, 0.45)
}

/// The two copies stand 3 nm apart, inside the sliver band: the census
/// escalates on whether they touch, which the part's own gate refuses,
/// naming none of its declarations.
fn in_band(label: &str) -> ProfileDoc {
    two_blocks(label, 0.5 + 3e-9)
}

fn run(doc: &ProfileDoc, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness())
}

fn gate(doc: &ProfileDoc, opts: &EvalOptions) -> Result<Assembly<f64>, AssemblyError> {
    assemble(doc, &run(doc, opts), Tol::witness())
}

/// A document instantiating `part` `n` times, each copy pinned to the
/// world 3 m along x from the last; the instances, and the options that
/// resolve the part.
fn instantiating(
    label: &str,
    part: &ProfileDoc,
    n: usize,
) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions) {
    let mut store = PartStore::new();
    let doc_ref = store.insert(part.clone(), Tol::witness());
    let opts = with_resolver(store);
    let mut doc = ProfileDoc::empty_derived(label, Tol::witness());
    let mut ids = Vec::new();
    for i in 0..n {
        let along = 3.0 * f64::from(u32::try_from(i).expect("a few instances"));
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
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
    let own = gate(&part, &EvalOptions::default()).expect("the part's own gate reports it");
    let [finding] = own.interference.as_slice() else {
        panic!("one overlap in the part: {:?}", own.interference)
    };
    let (doc, ids, opts) = instantiating("seam-overlap", &part, 1);
    let outer = gate(&doc, &opts).unwrap_or_else(|e| panic!("the outer gate refused: {e}"));
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
    let outer = gate(&doc, &opts).unwrap_or_else(|e| panic!("the outer gate refused: {e}"));
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
    let part_ref = store.insert(part.clone(), Tol::witness());
    let opts = with_resolver(store);
    let mid = ProfileDoc::empty_derived("seam-deep-mid", Tol::witness());
    let (mid, inner) = insert_pinned(mid, Node::instantiate_part(part_ref), &opts);
    let mid_own = gate(&mid, &opts).unwrap_or_else(|e| panic!("the middle refused: {e}"));
    assert_eq!(mid_own.carried_interference.len(), 1);
    let mut store = PartStore::new();
    store.insert(part.clone(), Tol::witness());
    let mid_ref = store.insert(mid, Tol::witness());
    let opts = with_resolver(store);
    let top = ProfileDoc::empty_derived("seam-deep-top", Tol::witness());
    let (top, outer) = insert_pinned(top, Node::instantiate_part(mid_ref), &opts);
    let assembly = gate(&top, &opts).unwrap_or_else(|e| panic!("the top refused: {e}"));
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

/// **A finding the part's own gate refuses refuses the instantiating
/// gate as the part's.** The part's two copies stand in the sliver
/// band, so its census escalates on whether they touch, and its gate
/// refuses that, naming none of its declarations. An escalation names
/// no pair, so nothing matches it to the census's re-finding inside the
/// copy (`work/contacthold/an-escalated-census-finding-names-no-pair.md`);
/// the part's refusal is raised first, so the outer document's own
/// verdict is never reached. The document
/// instantiating it refuses `CarriedAtRest` with the part's findings,
/// verbatim and by the route through the instance, rather than refusing
/// the census's re-finding inside the copy as its own.
///
/// *Breaks if* the outer gate refuses the re-found pair unattributed
/// (`AtRest`, as it did), or the carried findings are not the part's.
#[test]
fn a_parts_own_census_refusal_refuses_the_outer_gate_as_the_parts() {
    let part = in_band("seam-in-band-part");
    let Err(AssemblyError::AtRest { findings: own }) = gate(&part, &EvalOptions::default()) else {
        panic!("the part's own gate refuses the undecided pair")
    };
    assert!(
        own.iter().all(|f| f.attribution.is_none()) && own.len() > 1,
        "the premise: several findings, no declaration of the part's named"
    );
    let (doc, ids, opts) = instantiating("seam-in-band", &part, 1);
    let err = gate(&doc, &opts).expect_err("the part is not at rest, so neither is this");
    let AssemblyError::CarriedAtRest { findings } = &err else {
        panic!("the part's refusal, carried: {err}")
    };
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
    let line = err.spoken(&doc);
    assert!(
        line.contains("documents below this one")
            && line.contains("is not at rest by its own census"),
        "the sentence sends the author to the part: {line}"
    );
}

/// **An escalation two documents down is reported once, as the
/// part's.** A sub-assembly instantiates the in-band part, and a top
/// document instantiates the sub-assembly. The sub-assembly's census
/// finds the part's escalations again inside its copy, and an
/// escalation names no pair, so nothing matches them; but its gate
/// refuses over the part's carried refusal before its own at-rest
/// stage, so it crosses none of its own census, and the top reports the
/// part's findings once, by the route through the sub-assembly.
///
/// *Breaks if* the sub-assembly's seam verdict reads its census though
/// its gate never reaches it (the top then also reports the re-found
/// escalations, as the sub-assembly's own).
#[test]
fn an_escalation_two_documents_down_is_reported_once_as_the_parts() {
    let part = in_band("seam-deep-band-part");
    let Err(AssemblyError::AtRest { findings: own }) = gate(&part, &EvalOptions::default()) else {
        panic!("the part's own gate refuses its escalations")
    };
    let mut store = PartStore::new();
    let part_ref = store.insert(part.clone(), Tol::witness());
    let opts = with_resolver(store);
    let mid = ProfileDoc::empty_derived("seam-deep-band-mid", Tol::witness());
    let (mid, inner) = insert_pinned(mid, Node::instantiate_part(part_ref), &opts);
    let mut store = PartStore::new();
    store.insert(part.clone(), Tol::witness());
    let mid_ref = store.insert(mid, Tol::witness());
    let opts = with_resolver(store);
    let top = ProfileDoc::empty_derived("seam-deep-band-top", Tol::witness());
    let (top, outer) = insert_pinned(top, Node::instantiate_part(mid_ref), &opts);
    let err = gate(&top, &opts).expect_err("the part is not at rest, so neither is the top");
    let AssemblyError::CarriedAtRest { findings } = &err else {
        panic!("the part's refusal, carried: {err}")
    };
    assert_eq!(
        findings.iter().map(|f| &f.finding).collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
        "the part's findings, once each, and nothing of the sub-assembly's"
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

/// **A part in the world and in a loose space reports its overlap
/// once.** One instance is pinned to the world and another is loose, a
/// space of its own; the gate checks both, each finds the part's
/// overlap inside its copy, and it is one finding of one part row.
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
    let (doc, _) = insert(doc.clone(), crate::fixture::mated_instance(*doc_ref));
    let outer = gate(&doc, &opts).unwrap_or_else(|e| panic!("the outer gate refused: {e}"));
    assert!(outer.interference.is_empty(), "{:?}", outer.interference);
    let [carried] = outer.carried_interference.as_slice() else {
        panic!("once per part row: {:?}", outer.carried_interference)
    };
    assert_eq!(
        carried.route.through, ids[0],
        "the world instance's route, first"
    );
}

/// **A part placed twice reports its own refusal once.** Both copies
/// carry the in-band part's escalations up, and they are one part row's
/// findings, reported by the first instance's route.
///
/// *Breaks if* each instance reports the part's refusal again (the
/// at-rest arm of the once-per-part-row rule).
#[test]
fn a_refusing_part_placed_twice_reports_its_refusal_once() {
    let part = in_band("seam-band-twice-part");
    let Err(AssemblyError::AtRest { findings: own }) = gate(&part, &EvalOptions::default()) else {
        panic!("the part's own gate refuses its escalations")
    };
    let (doc, ids, opts) = instantiating("seam-band-twice", &part, 2);
    let err = gate(&doc, &opts).expect_err("the part is not at rest");
    let AssemblyError::CarriedAtRest { findings } = &err else {
        panic!("the part's refusal, carried: {err}")
    };
    assert_eq!(findings.len(), own.len(), "once per part row: {err}");
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
    let (v2, a) = block(v1.clone(), (10.0, 0.0, 0.0), 0.5, 1.0);
    let (v2, b) = block(v2, (10.8 + 3e-9, 0.0, 0.25), 0.3, 0.5);
    let (v2, _) = place(v2, a);
    let v2 = place(v2, b).0;
    let count = |doc: &ProfileDoc| match gate(doc, &EvalOptions::default()) {
        Err(AssemblyError::AtRest { findings }) => findings.len(),
        other => panic!("each version refuses in band: {other:?}"),
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
        let (doc, _) = insert_pinned(doc, Node::instantiate_part(refs[0]), &opts);
        let (doc, id) = insert(doc, Node::instantiate_part(refs[1]));
        let doc = pin(doc, id, &Frame::translation([30.0, 0.0, 0.0]), &opts);
        let err = gate(&doc, &opts).expect_err("neither version is at rest");
        let AssemblyError::CarriedAtRest { findings } = &err else {
            panic!("the versions' refusals, carried: {err}")
        };
        assert_eq!(findings.len(), n1 + n2, "every finding of both versions");
    }
}
