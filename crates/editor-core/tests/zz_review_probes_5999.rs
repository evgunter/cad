//! PR 5999 review probes (R1): adversarial "reported once" rows.

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

fn mirrored(doc: ProfileDoc, of: RecipeNodeId, x: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, k) = block(doc, [x - 0.5, 0.0], 0.1, 0.1);
    let (doc, k) = put(doc, fixture::stood(k));
    let plane = fixture::plane_on(&doc, k, [x, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let (doc, image) = put(
        doc,
        Node::Mirror {
            body: Operand::output(of, 0).into(),
            plane,
        },
    );
    put(doc, fixture::stood(image))
}

fn report(tag: &str, doc: &ProfileDoc, ev: &Evaluation<f64>) -> (usize, usize) {
    let product = editor_core::product_recorded(doc, ev, Tol::witness());
    let census = product
        .as_ref()
        .map(|p| p.census.len())
        .unwrap_or(usize::MAX);
    let reported = fixture::at_rest(doc, ev);
    let n = reported.as_ref().map(|r| r.len()).unwrap_or(usize::MAX);
    eprintln!(
        "PROBE {tag}: census={census} reported={n} {:?}",
        reported
            .map(|r| r
                .iter()
                .map(|f| format!("{:?}", f.subject))
                .collect::<Vec<_>>())
            .map_err(|e| e.to_string())
    );
    (census, n)
}

/// p1: a mirror of a mirror of an instance whose part overlaps.
#[test]
fn p1_mirror_of_mirror_of_instance() {
    let (store, part) = overlapping_part("p5999-1-part");
    let (doc, i) = instance(doc("p5999-1-host"), part);
    let (doc, m1) = mirrored(doc, i, -3.0);
    let (doc, m2) = mirrored(doc, m1, -10.0);
    let doc = set_product(&doc, m2);
    let ev = run(&doc, &fixture::resolver::with_resolver(store));
    assert_eq!(report("p1", &doc, &ev), (0, 1));
}

/// p2: a mirror of an overlapping `⊔`, the product alone: the meeting
/// is reported once (at the `⊔`'s own gather), not zero times.
#[test]
fn p2_mirror_of_assemble_alone_is_not_missed() {
    let (doc, source) = overlap("p5999-2");
    let (doc, m) = mirrored(doc, source, -3.0);
    let doc = set_product(&doc, m);
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(report("p2", &doc, &ev), (0, 1));
}

/// p3: a family of instances under `⊔`.
#[test]
fn p3_family_of_instances_under_assemble() {
    let (store, part) = overlapping_part("p5999-3-part");
    let (doc, i) = instance(doc("p5999-3-host"), part);
    let (doc, fam) = patterned(doc, i, 10.0);
    let (doc, all) = assembled_family(doc, fam);
    let doc = set_product(&doc, all);
    let ev = run(&doc, &fixture::resolver::with_resolver(store));
    assert_eq!(report("p3", &doc, &ev), (0, 1));
}

/// p4: an instance of a part whose product is a mirror of an
/// overlapping `⊔`.
#[test]
fn p4_instance_of_a_mirror_part() {
    let (part, source) = overlap("p5999-4-part");
    let (part, m) = mirrored(part, source, -3.0);
    let part = set_product(&part, m);
    let mut store = fixture::resolver::PartStore::default();
    let r = store.insert(part, Tol::witness());
    let (doc, i) = instance(doc("p5999-4-host"), r);
    let doc = set_product(&doc, i);
    let ev = run(&doc, &fixture::resolver::with_resolver(store));
    assert_eq!(report("p4", &doc, &ev), (0, 1));
}

/// p5: a pick out of a placed mirror of a `⊔`: the image holds no
/// pieces, so the pick refuses typed (not a panic, not a silent body).
#[test]
fn p5_pick_out_of_a_mirrored_assemble() {
    let d = doc("p5999-5");
    let (d, a) = block(d, [0.0; 2], 0.5, 1.0);
    let (d, again) = beside(d, a, [0.3, 0.2, 0.25], 0.5, 1.0);
    let (d, a) = put(d, fixture::stood(a));
    let (d, again) = put(d, fixture::stood(again));
    let (d, source) = assembled(d, &[a, again]);
    let (d, m) = mirrored(d, source, -3.0);
    let piece = d.output(a, 0).unwrap();
    let refused = d.apply(
        &DocEdit::InsertNode {
            node: Box::new(fixture::stood(editor_core::BodyRead::plain(Operand::pick(
                Operand::Node(m),
                vec![piece],
            )))),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    let refused = refused.map(|_| ()).map_err(|e| format!("{e:?}"));
    eprintln!("PROBE p5: {refused:?}");
    assert!(
        refused.as_ref().is_err_and(|e| e.starts_with("PickPath")),
        "a mirror's image holds no pieces to pick, refused typed at the edit"
    );
}

/// p6: a mirror of a recorded seat keeps the seat's record and the
/// checks read it as the source's.
#[test]
fn p6_mirror_of_seat_keeps_records() {
    let d = doc("p5999-6");
    let (d, plate) = block(d, [0.0; 2], 1.0, 0.2);
    let (d, plate) = put(d, fixture::stood(plate));
    let (d, bolt) = beside(d, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (d, seat) = assembled(d, &[bolt, plate]);
    let direct = set_product(&d, seat);
    let ev_d = run(&direct, &EvalOptions::default());
    let rows_d = editor_core::product_recorded(&direct, &ev_d, Tol::witness())
        .unwrap()
        .contacts
        .rows()
        .len();
    let (d, m) = mirrored(d, seat, -3.0);
    let d = set_product(&d, m);
    let ev = run(&d, &EvalOptions::default());
    let rows = editor_core::product_recorded(&d, &ev, Tol::witness())
        .unwrap()
        .contacts
        .rows()
        .len();
    eprintln!(
        "PROBE p6: direct={rows_d} mirrored={rows} unproven {} vs {}",
        unproven(&direct, &ev_d),
        unproven(&d, &ev)
    );
    assert!(rows_d > 0);
    assert_eq!(rows, rows_d);
    assert_eq!(unproven(&d, &ev), unproven(&direct, &ev_d));
}

/// p7: an indexed read of a placed family (a placement of a family
/// read whole) carries the member's records, as an indexed read of the
/// family itself does.
#[test]
fn p7_member_of_a_placed_family_keeps_records() {
    let d = doc("p5999-7");
    let (d, plate) = block(d, [0.0; 2], 1.0, 0.2);
    let (d, plate) = put(d, fixture::stood(plate));
    let (d, bolt) = beside(d, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (d, seat) = assembled(d, &[bolt, plate]);
    let (d, fam) = patterned(d, seat, 10.0);
    let (d, direct_m) = member(d, fam, 1);
    let (d, placed) = put(d, fixture::stood(fam));
    let (d, m) = member(d, placed, 1);
    let a = set_product(&d, direct_m);
    let ev_a = run(&a, &EvalOptions::default());
    let rows_a = editor_core::product_recorded(&a, &ev_a, Tol::witness())
        .unwrap()
        .contacts
        .rows()
        .len();
    let b = set_product(&d, m);
    let ev_b = run(&b, &EvalOptions::default());
    let rows_b = editor_core::product_recorded(&b, &ev_b, Tol::witness())
        .unwrap()
        .contacts
        .rows()
        .len();
    eprintln!("PROBE p7: family member={rows_a} placed-family member={rows_b}");
    assert!(rows_a > 0);
    assert_eq!(rows_b, rows_a);
}

/// p8: a union of one `⊔` keeps its pieces: subtracting a tool from it
/// is the same as from the `⊔` itself (records and census reported once).
#[test]
fn p8_union_of_one_assemble_beside_source() {
    let (doc, source) = overlap("p5999-8");
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
    assert_eq!(report("p8", &doc, &ev), (0, 1));
}

/// p4b: control for p4 — an instance of a part whose product is a
/// placement (not a mirror) of an overlapping `⊔`.
#[test]
fn p4b_instance_of_a_placed_assemble_part() {
    let (part, source) = overlap("p5999-4b-part");
    let (part, m) = fixture::slid(part, source, source, [5.0, 0.0, 0.0]);
    let part = set_product(&part, m);
    let ev_part = run(&part, &EvalOptions::default());
    report("p4b-part-alone", &part, &ev_part);
    let mut store = fixture::resolver::PartStore::default();
    let r = store.insert(part, Tol::witness());
    let (doc, i) = instance(doc("p5999-4b-host"), r);
    let doc = set_product(&doc, i);
    let ev = run(&doc, &fixture::resolver::with_resolver(store));
    assert_eq!(report("p4b", &doc, &ev), (0, 1));
}

/// p4c: p4's part alone.
#[test]
fn p4c_mirror_part_alone() {
    let (part, source) = overlap("p5999-4-part");
    let (part, m) = mirrored(part, source, -3.0);
    let part = set_product(&part, m);
    let ev = run(&part, &EvalOptions::default());
    assert_eq!(report("p4c", &part, &ev), (0, 1));
}
