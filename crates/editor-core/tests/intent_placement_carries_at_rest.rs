//! INTENT stage 3, the carry (`docs/INTENT-STAGE3-SPEC.md` §12 step 5;
//! D10 `[placement.placement]`): a placement's copy carries its
//! source's contact records and census decisions, which stay the
//! source's, at every placement — a placement, an instance, a pattern's
//! members — so a meeting inside the source is decided once, at the
//! door that decided it, and reported once there, however many times
//! the source is placed or read.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Bodies, CheckEvidence, DocEdit, DocumentId, EvalOptions, Evaluation, FindingSubject, Node,
    Operand, ProfileDoc, RecipeNodeId, VarId, apply,
};
use fixture::{len, on_frame, run, square};
use geom_core::Tol;

/// A block of half-width `half` centred at `(x, y)`, from `0` to
/// `height`, a construction in a space of its own.
fn block(doc: ProfileDoc, [x, y]: [f64; 2], half: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(x, y, half)],
    );
    put(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// A block brought into `of`'s space, raised to stand on `z`.
fn beside(
    doc: ProfileDoc,
    of: RecipeNodeId,
    [x, y, z]: [f64; 3],
    half: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = block(doc, [x, y], half, height);
    fixture::slid(doc, of, body, [0.0, 0.0, z])
}

/// Inserts `node` as written, so the product is what a row sets.
fn put(doc: ProfileDoc, node: editor_core::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = fixture::step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
    );
    (doc, minted.expect("the insert mints the node"))
}

fn doc(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
}

fn set_product(doc: &ProfileDoc, product: RecipeNodeId) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::SetProduct {
            product: Some(Operand::Node(product)),
        },
        Tol::witness(),
    )
    .expect("the slot takes a body in the world")
    .doc
}

/// The `⊔` of `members`, each read whole.
fn assembled(doc: ProfileDoc, members: &[RecipeNodeId]) -> (ProfileDoc, RecipeNodeId) {
    put(
        doc,
        Node::Assemble {
            members: Bodies::Spelled(members.iter().map(|&r| r.into()).collect()),
        },
    )
}

/// The `⊔` of a family, each member its own arrival.
fn assembled_family(doc: ProfileDoc, family: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    put(
        doc,
        Node::Assemble {
            members: Bodies::Family(family.into()),
        },
    )
}

/// Two copies of `source` along x, `pitch` apart: a family over an
/// index, member `k` slid `scalar(k)·pitch` (D10 `[repetition]`).
fn patterned(doc: ProfileDoc, source: RecipeNodeId, pitch: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, _, family) = fixture::linear(doc, source, [1.0, 0.0, 0.0], pitch, 2);
    (doc, family)
}

/// Member `i` of `family`, read by an indexed read and placed where it
/// stands.
fn member(doc: ProfileDoc, family: RecipeNodeId, i: i64) -> (ProfileDoc, RecipeNodeId) {
    fixture::member(doc, family, i)
}

/// `a ⊔ a'`: two world blocks overlapping in general position.
fn overlap(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = doc(label);
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, again) = beside(doc, a, [0.3, 0.2, 0.25], 0.5, 1.0);
    let (doc, a) = put(doc, fixture::stood(a));
    let (doc, again) = put(doc, fixture::stood(again));
    assembled(doc, &[a, again])
}

/// The at-rest check's findings, carried ones included.
fn at_rest(doc: &ProfileDoc, ev: &Evaluation<f64>) -> Vec<editor_core::CheckFinding> {
    fixture::at_rest(doc, ev).unwrap_or_else(|e| panic!("the product gathers: {e}"))
}

/// The `unproven-coincidence` findings the checks report.
fn unproven(doc: &ProfileDoc, ev: &Evaluation<f64>) -> usize {
    editor_core::run_checks(
        doc,
        ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run")
    .findings
    .iter()
    .filter(|f| f.check == editor_core::CheckId::UnprovenCoincidence)
    .count()
}

/// **A source placed twice and read three ways is reported once, as
/// the source's.** `a ⊔ a'` overlaps; it is placed once by a placement
/// and twice more by a pattern, and read whole by the product's `⊔`,
/// as a pattern member's projection by the same `⊔`, and as the whole
/// family by a `⊔` the product does not read. Each copy carries the
/// census its source's door kept, so no census reading a copy decides
/// the overlap again: it is one finding, at `a ⊔ a'`.
///
/// *Breaks if* a pattern's member drops its source's census, so the
/// census reading it as one arrival finds the overlap again.
#[test]
fn a_source_placed_twice_and_read_three_ways_is_reported_once() {
    let (base, source) = overlap("carry-three-ways");
    let alone = set_product(&base, source);
    let ev_alone = run(&alone, &EvalOptions::default());
    let once = at_rest(&alone, &ev_alone);
    assert_eq!(once.len(), 1, "the overlap is one finding: {once:?}");

    let (doc, placed) = fixture::slid(base, source, source, [5.0, 0.0, 0.0]);
    let (doc, family) = patterned(doc, source, 10.0);
    let (doc, first) = member(doc, family, 0);
    let (doc, whole) = assembled(doc, &[placed, first]);
    let (doc, each) = assembled_family(doc, family);
    let doc = set_product(&doc, whole);
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the product's census decides nothing inside a copy: {:?}",
        product.census
    );
    let family_census = &ev
        .value(each)
        .and_then(|v| v.carry.census.as_ref())
        .expect("the family's `⊔` keeps its census")
        .verdict(&doc, Tol::witness())
        .expect("its census runs")
        .census;
    assert!(
        family_census.is_empty(),
        "nor does a census reading each member: {family_census:?}"
    );
    let reported = at_rest(&doc, &ev);
    let source_read = doc.output(source, 0).expect("the `⊔` defines its body");
    assert_eq!(
        reported.len(),
        1,
        "the overlap, once, however the copies are read: {reported:?}"
    );
    assert!(
        matches!(
            (&reported[0].subject, &reported[0].evidence),
            (FindingSubject::Census { read }, CheckEvidence::Interference { .. })
                if *read == source_read
        ),
        "as the source's own `⊔`'s: {reported:?}"
    );
}

/// **A patterned seat carries its record to each member.** `bolt ⊔
/// plate` records the seat; a `⊔` of its pattern's two members carries
/// that record once per member, keyed in the member it is, decides no
/// touch of its own, and the checks report the seat once, at the `⊔`
/// that decided it.
///
/// *Breaks if* a pattern drops its master's records, so the family's
/// `⊔` records each seat again.
#[test]
fn a_patterned_seat_carries_its_record_to_each_member() {
    let doc = doc("carry-patterned-seat");
    let (doc, plate) = block(doc, [0.0; 2], 1.0, 0.2);
    let (doc, plate) = put(doc, fixture::stood(plate));
    let (doc, bolt) = beside(doc, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (base, seat) = assembled(doc, &[bolt, plate]);
    let alone = set_product(&base, seat);
    let ev_alone = run(&alone, &EvalOptions::default());
    let seats = unproven(&alone, &ev_alone);
    assert!(seats > 0, "the seat is unproven at its `⊔`");

    let (doc, family) = patterned(base, seat, 5.0);
    let (doc, both) = assembled_family(doc, family);
    let doc = set_product(&doc, both);
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert_eq!(
        product.contacts.face_face().count(),
        2,
        "each member carries the seat's record"
    );
    assert!(
        product.coincidences.is_empty() && product.census.is_empty(),
        "and the family's `⊔` decides no seat again: {:?}",
        product.coincidences
    );
    fixture::could_look(&doc, &ev).expect("the members are at rest");
    assert_eq!(
        unproven(&doc, &ev),
        seats,
        "the seat is reported once, at its `⊔`"
    );
}

/// **An instance's part findings cross a pattern of it.** The part is
/// `a ⊔ a'`, overlapping; its instance is patterned, and the host
/// reads both members as its product and the whole family by a `⊔` the
/// product does not read. Each member carries the part's census
/// decision and its finding, so the host decides nothing inside a
/// member and reports the overlap once, as the part's, through the
/// instance.
///
/// *Breaks if* a pattern of an instance drops the part's findings, so
/// the host loses them and finds the overlap again itself.
#[test]
fn a_parts_finding_crosses_a_pattern_of_its_instance_once() {
    let (part, source) = overlap("carry-part");
    let part = set_product(&part, source);
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let opts = fixture::resolver::with_resolver(store);
    let (doc, instance) = put(
        doc("carry-host"),
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let (doc, family) = patterned(doc, instance, 5.0);
    let (doc, first) = member(doc, family, 0);
    let (doc, second) = member(doc, family, 1);
    let (doc, both) = assembled(doc, &[first, second]);
    let (doc, _) = assembled_family(doc, family);
    let doc = set_product(&doc, both);
    let ev = run(&doc, &opts);
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the host decides nothing inside a member: {:?}",
        product.census
    );
    let reported = at_rest(&doc, &ev);
    assert_eq!(reported.len(), 1, "the part's overlap, once: {reported:?}");
    assert!(
        matches!(
            &reported[0].evidence,
            CheckEvidence::CarriedInterference { finding } if finding.route.through == instance
        ),
        "as the part's, through the instance: {reported:?}"
    );
}

/// **A pick of a moved copy stands where the copy stands.** `a ⊔ c`
/// is slid 7 along y; a pick of its `a` piece, assembled beside a
/// fresh block at the origin, is clear of it, so the at-rest check
/// reports nothing.
///
/// *Breaks if* a copy's pieces are carried unmoved, so the pick stands
/// on the fresh block and overlaps it.
#[test]
fn a_pick_of_a_moved_copy_stands_with_it() {
    let doc = doc("carry-moved-pick");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, a) = put(doc, fixture::stood(a));
    let (doc, c) = beside(doc, a, [3.0, 0.0, 0.0], 0.5, 1.0);
    let (doc, c) = put(doc, fixture::stood(c));
    let (doc, outer) = assembled(doc, &[a, c]);
    let (doc, placed) = fixture::slid(doc, outer, outer, [0.0, 7.0, 0.0]);
    let piece = doc.output(a, 0).expect("the copy defines its body");
    let (doc, picked) = put(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::pick(
            Operand::Node(placed),
            vec![piece],
        ))),
    );
    let (doc, fresh) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, fresh) = put(doc, fixture::stood(fresh));
    let (doc, both) = assembled(doc, &[picked, fresh]);
    let doc = set_product(&doc, both);
    let ev = run(&doc, &EvalOptions::default());
    let reported = at_rest(&doc, &ev);
    assert!(
        reported.is_empty(),
        "the picked piece stands 7 off, clear of the fresh block: {reported:?}"
    );
}
