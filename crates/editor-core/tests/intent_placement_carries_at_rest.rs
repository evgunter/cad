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
/// index, member `k` slid `scalar(k)·pitch` (D10 `[repetition]`), in
/// `source`'s space. Inserted plainly: the rows name their product, so
/// no world `⊔` gathers the family beside it.
fn patterned(doc: ProfileDoc, source: RecipeNodeId, pitch: f64) -> (ProfileDoc, RecipeNodeId) {
    let name = fixture::fresh_index_name(&doc);
    let (doc, k) = fixture::index(doc, &name, 2);
    let target = fixture::space(&doc, source);
    let read = fixture::body_read(&doc, source.into());
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        fixture::slides_along(k, [1.0, 0.0, 0.0], pitch),
    );
    put(
        doc,
        Node::place(
            editor_core::BodyRead::plain(read),
            Operand::Var(target),
            placed.bundle(),
        ),
    )
}

/// Member `i` of `family`, read by an indexed read and placed where it
/// stands, inserted plainly as [`patterned`] is.
fn member(doc: ProfileDoc, family: RecipeNodeId, i: i64) -> (ProfileDoc, RecipeNodeId) {
    let node = fixture::member_node(&doc, family, vec![editor_core::Formula::count(i)]);
    put(doc, node)
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

/// A part store holding `a ⊔ a'`, overlapping, as its product.
fn overlapping_part(label: &str) -> (fixture::resolver::PartStore, editor_core::DocRef) {
    let (part, source) = overlap(label);
    let part = set_product(&part, source);
    let mut store = fixture::resolver::PartStore::default();
    let r = store.insert(part, Tol::witness());
    (store, r)
}

/// An instance of `part` standing in the world.
fn instance(doc: ProfileDoc, part: editor_core::DocRef) -> (ProfileDoc, RecipeNodeId) {
    put(
        doc,
        Node::instantiate_part(
            part,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    )
}

/// **A mirror of an instance reports its part's finding once.** The
/// part is `a ⊔ a'`, overlapping; the host's product is a mirror of its
/// instance. The image carries what the instance carries, so the host
/// decides nothing inside it and the overlap is one finding, the part's.
///
/// *Breaks if* a mirror builds its own carry, so a census reading the
/// image as one arrival finds the overlap again beside the part's.
#[test]
fn a_mirror_of_an_instance_reports_its_parts_finding_once() {
    let (store, part) = overlapping_part("carry-mirror-part");
    let (doc, k) = block(doc("carry-mirror-host"), [-3.5, 0.0], 0.5, 1.0);
    let (doc, k) = put(doc, fixture::stood(k));
    let plane = fixture::plane_on(&doc, k, [-3.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let (doc, i) = instance(doc, part);
    let (doc, image) = put(
        doc,
        Node::Mirror {
            body: Operand::output(i, 0).into(),
            plane,
        },
    );
    let (doc, stood) = put(doc, fixture::stood(image));
    let doc = set_product(&doc, stood);
    let ev = run(&doc, &fixture::resolver::with_resolver(store));
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the host decides nothing inside the image: {:?}",
        product.census
    );
    let reported = at_rest(&doc, &ev);
    assert_eq!(reported.len(), 1, "the part's overlap, once: {reported:?}");
    assert!(
        matches!(
            &reported[0].evidence,
            CheckEvidence::CarriedInterference { finding } if finding.route.through == i
        ),
        "as the part's, through the instance: {reported:?}"
    );
}

/// **A mirror of a `⊔` beside its source reports the overlap once.**
/// `a ⊔ a'` overlaps; its mirror stands clear of it, and the product
/// assembles the two. The image carries its source's census, so the
/// overlap inside it is the source's, reported once, at its `⊔`.
///
/// *Breaks if* a mirror's image is a body no door censused, so the
/// product's census finds the overlap inside it again.
#[test]
fn a_mirror_of_an_assemble_beside_its_source_reports_the_overlap_once() {
    let (doc, source) = overlap("carry-mirror-assemble");
    let plane = fixture::plane_on(&doc, source, [-3.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let (doc, image) = put(
        doc,
        Node::Mirror {
            body: Operand::output(source, 0).into(),
            plane,
        },
    );
    let (doc, stood) = put(doc, fixture::stood(image));
    let (doc, both) = assembled(doc, &[source, stood]);
    let doc = set_product(&doc, both);
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the product decides nothing inside the source or its image: {:?}",
        product.census
    );
    let reported = at_rest(&doc, &ev);
    let source_read = doc.output(source, 0).expect("the `⊔` defines its body");
    assert_eq!(reported.len(), 1, "the overlap, once: {reported:?}");
    assert!(
        matches!(
            (&reported[0].subject, &reported[0].evidence),
            (FindingSubject::Census { read }, CheckEvidence::Interference { .. })
                if *read == source_read
        ),
        "as the source's own `⊔`'s: {reported:?}"
    );
}

/// **A pick of an instance's piece keeps its part's records.** The
/// part is `bolt ⊔ plate`, whose seat is recorded; the host assembles
/// its instance beside a clear block, places that `⊔`, and its product
/// is a pick of the instance's piece. The pick carries the seat's
/// record as the instance does.
///
/// *Breaks if* a pick drops the records its piece carries because they
/// cite inputs the pick does not name.
#[test]
fn a_pick_of_an_instance_piece_keeps_its_records() {
    let d = doc("carry-pick-instance-part");
    let (d, plate) = block(d, [0.0; 2], 1.0, 0.2);
    let (d, plate) = put(d, fixture::stood(plate));
    let (d, bolt) = beside(d, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (d, seat) = assembled(d, &[bolt, plate]);
    let d = set_product(&d, seat);
    let mut store = fixture::resolver::PartStore::default();
    let part = store.insert(d, Tol::witness());
    let opts = fixture::resolver::with_resolver(store);

    let (host, i) = instance(doc("carry-pick-instance-host"), part);
    let direct = set_product(&host, i);
    let ev_direct = run(&direct, &opts);
    let seats = editor_core::product_recorded(&direct, &ev_direct, Tol::witness())
        .expect("the instance gathers")
        .contacts
        .rows()
        .len();
    assert!(seats > 0, "the instance carries the part's seat");

    let (host, c) = block(host, [5.0, 0.0], 0.5, 1.0);
    let (host, c) = put(host, fixture::stood(c));
    let (host, outer) = assembled(host, &[i, c]);
    let (host, placed) = put(host, fixture::stood(outer));
    let piece = host.output(i, 0).expect("the instance defines its body");
    let (host, picked) = put(
        host,
        fixture::stood(editor_core::BodyRead::plain(Operand::pick(
            Operand::Node(placed),
            vec![piece],
        ))),
    );
    let host = set_product(&host, picked);
    let ev = run(&host, &opts);
    let product = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("it gathers");
    assert_eq!(
        product.contacts.rows().len(),
        seats,
        "the pick keeps the part's seat record"
    );
    assert_eq!(
        unproven(&host, &ev),
        unproven(&direct, &ev_direct),
        "and the checks read the seat as the instance's"
    );
}

/// **A union of one instance is that instance, its part's finding
/// with it.** A union or an intersect of one body is that body (DM4):
/// it carries what the body carries, so the host decides nothing inside
/// it and reports the part's overlap once, through the instance.
///
/// *Breaks if* a lone member is handed on bare, so the host loses the
/// part's finding and its census finds the overlap again.
#[test]
fn a_union_of_one_instance_carries_its_parts_finding() {
    let (store, part) = overlapping_part("carry-lone-member-part");
    let (doc, i) = instance(doc("carry-lone-member-host"), part);
    let (doc, lone) = put(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![i.into()]),
        },
    );
    let doc = set_product(&doc, lone);
    let ev = run(&doc, &fixture::resolver::with_resolver(store));
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the host decides nothing inside the lone member: {:?}",
        product.census
    );
    let reported = at_rest(&doc, &ev);
    assert_eq!(reported.len(), 1, "the part's overlap, once: {reported:?}");
    assert!(
        matches!(
            &reported[0].evidence,
            CheckEvidence::CarriedInterference { finding } if finding.route.through == i
        ),
        "as the part's, through the instance: {reported:?}"
    );
}

/// The image of `of` across the plane `x = at`, read off a small block
/// standing there, stood in the world.
fn mirrored(doc: ProfileDoc, of: RecipeNodeId, at: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, k) = block(doc, [at - 0.5, 0.0], 0.1, 0.1);
    let (doc, k) = put(doc, fixture::stood(k));
    let plane = fixture::plane_on(&doc, k, [at, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let (doc, image) = put(
        doc,
        Node::Mirror {
            body: Operand::output(of, 0).into(),
            plane,
        },
    );
    put(doc, fixture::stood(image))
}

/// The product's census rows and the at-rest check's findings.
fn census_and_reported(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
) -> (usize, Vec<editor_core::CheckFinding>) {
    let product = editor_core::product_recorded(doc, ev, Tol::witness()).expect("it gathers");
    (product.census.len(), at_rest(doc, ev))
}

/// **A mirror of an overlapping `⊔` is reported once, at the `⊔`.**
/// The product is the image alone: the image's census decides nothing
/// inside it, and the `⊔` the product does not read reports the
/// overlap.
///
/// *Breaks if* an image carries its source's census decision with no
/// door left to report it, so the overlap is reported nowhere.
#[test]
fn a_mirror_of_an_assemble_alone_is_reported_once_at_the_assemble() {
    let (doc, source) = overlap("carry-mirror-alone");
    let (doc, image) = mirrored(doc, source, -3.0);
    let doc = set_product(&doc, image);
    let ev = run(&doc, &EvalOptions::default());
    let (census, reported) = census_and_reported(&doc, &ev);
    assert_eq!(census, 0, "the image's census decides nothing inside it");
    let source_read = doc.output(source, 0).expect("the `⊔` defines its body");
    assert!(
        matches!(
            reported.as_slice(),
            [only] if matches!(only.subject, FindingSubject::Census { read } if read == source_read)
        ),
        "the overlap, once, at the `⊔`: {reported:?}"
    );
}

/// **An instance of a part whose product is a copy of its `⊔` reports
/// the part's overlap once.** The part's product is a mirror (`image`)
/// or a placement (`slid`) of `a ⊔ a'`; the part's own check reports
/// the overlap at its `⊔`, which its product does not read, and the
/// seam carries that, so the host reports it once, as the part's.
///
/// *Breaks if* the seam carries the part's product census alone, so
/// what the part's `⊔` decided is reported nowhere in the host.
#[test]
fn an_instance_of_a_part_whose_product_copies_its_assemble_reports_once() {
    for copy in ["image", "slid"] {
        let (part, source) = overlap(&format!("carry-seam-{copy}-part"));
        let (part, product) = match copy {
            "image" => mirrored(part, source, -3.0),
            _ => fixture::slid(part, source, source, [5.0, 0.0, 0.0]),
        };
        let part = set_product(&part, product);
        let ev_part = run(&part, &EvalOptions::default());
        let (_, alone) = census_and_reported(&part, &ev_part);
        assert_eq!(
            alone.len(),
            1,
            "{copy}: the part reports its overlap once: {alone:?}"
        );

        let mut store = fixture::resolver::PartStore::default();
        let part_ref = store.insert(part, Tol::witness());
        let (doc, i) = instance(doc(&format!("carry-seam-{copy}-host")), part_ref);
        let doc = set_product(&doc, i);
        let ev = run(&doc, &fixture::resolver::with_resolver(store));
        let (census, reported) = census_and_reported(&doc, &ev);
        assert_eq!(
            census, 0,
            "{copy}: the host decides nothing inside the instance"
        );
        assert!(
            matches!(
                reported.as_slice(),
                [only] if matches!(
                    &only.evidence,
                    CheckEvidence::CarriedInterference { finding } if finding.route.through == i
                )
            ),
            "{copy}: the part's overlap, once, through the instance: {reported:?}"
        );
    }
}

/// **A mirror of a recorded seat keeps the seat's record**, and the
/// checks read it as the source's: as many records and as many
/// unproven rows as the seat's `⊔` alone.
///
/// *Breaks if* an image drops its source's records or decides the seat
/// again.
#[test]
fn a_mirror_of_a_seat_keeps_its_record() {
    let d = doc("carry-mirror-seat");
    let (d, plate) = block(d, [0.0; 2], 1.0, 0.2);
    let (d, plate) = put(d, fixture::stood(plate));
    let (d, bolt) = beside(d, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (d, seat) = assembled(d, &[bolt, plate]);
    let direct = set_product(&d, seat);
    let ev_direct = run(&direct, &EvalOptions::default());
    let records = |doc: &ProfileDoc, ev: &Evaluation<f64>| {
        editor_core::product_recorded(doc, ev, Tol::witness())
            .expect("it gathers")
            .contacts
            .rows()
            .len()
    };
    let seats = records(&direct, &ev_direct);
    assert!(seats > 0, "the seat is recorded at its `⊔`");
    let (d, image) = mirrored(d, seat, -3.0);
    let d = set_product(&d, image);
    let ev = run(&d, &EvalOptions::default());
    assert_eq!(
        records(&d, &ev),
        seats,
        "the image carries the seat's record"
    );
    assert_eq!(
        unproven(&d, &ev),
        unproven(&direct, &ev_direct),
        "and the checks read it as the source's"
    );
}

/// **A union of one `⊔` is that `⊔`**, its census decision with it: a
/// placement of it beside its source reports the overlap once, at the
/// source's `⊔`.
///
/// *Breaks if* a lone member is handed on bare, so a census reading
/// its copy as one arrival finds the overlap again.
#[test]
fn a_union_of_one_assemble_placed_beside_it_reports_the_overlap_once() {
    let (doc, source) = overlap("carry-lone-assemble");
    let (doc, lone) = put(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![source.into()]),
        },
    );
    let (doc, placed) = fixture::slid(doc, lone, lone, [5.0, 0.0, 0.0]);
    let (doc, both) = assembled(doc, &[source, placed]);
    let doc = set_product(&doc, both);
    let ev = run(&doc, &EvalOptions::default());
    let (census, reported) = census_and_reported(&doc, &ev);
    assert_eq!(
        census, 0,
        "the product decides nothing inside either arrival"
    );
    assert_eq!(reported.len(), 1, "the overlap, once: {reported:?}");
}
