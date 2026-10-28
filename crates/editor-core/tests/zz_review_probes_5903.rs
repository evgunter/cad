//! REVIEW PROBES (R1 of PR 5903) on the carry (`docs/INTENT-STAGE3-SPEC.md` §12 step 5;
//! D10 `[placement.placement]`): a placement's copy carries its
//! source's contact records and census decisions, which stay the
//! source's, at every placement — a placement, an instance, a pattern's
//! members — so a meeting inside the source is decided once, at the
//! door that decided it, and reported once there, however many times
//! the source is placed or read.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Bodies, DocEdit, DocumentId, EvalOptions, Evaluation, Node,
    Operand, PartSelect, ProfileDoc, RecipeNodeId, VarId, apply,
};
use fixture::{len, on_frame, run, scl, square};
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

/// Two copies of `source` along x, `pitch` apart: a pattern.
fn patterned(doc: ProfileDoc, source: RecipeNodeId, pitch: f64) -> (ProfileDoc, RecipeNodeId) {
    put(
        doc,
        Node::Pattern {
            input: source.into(),
            count: editor_core::Formula::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(pitch),
            },
        },
    )
}

/// Member `i` of `family`, read through a projection.
fn member(doc: ProfileDoc, family: RecipeNodeId, i: i64) -> (ProfileDoc, RecipeNodeId) {
    put(
        doc,
        Node::Part {
            of: family.into(),
            select: PartSelect::Instance(editor_core::Formula::count(i)),
        },
    )
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


fn summary(f: &[editor_core::CheckFinding]) -> Vec<String> {
    f.iter()
        .map(|f| {
            let ev = format!("{:?}", f.evidence);
            let sub = format!("{:?}", f.subject);
            format!("{} | {}", &sub[..sub.len().min(60)], &ev[..ev.len().min(70)])
        })
        .collect()
}

fn part_of(label: &str) -> (fixture::resolver::PartStore, editor_core::DocRef) {
    let (part, source) = overlap(label);
    let part = set_product(&part, source);
    let mut store = fixture::resolver::PartStore::default();
    let r = store.insert(part, Tol::witness());
    (store, r)
}

fn inst(doc: ProfileDoc, r: editor_core::DocRef) -> (ProfileDoc, RecipeNodeId) {
    put(
        doc,
        Node::instantiate_part(r, Operand::Var(VarId::WORLD), editor_core::stand().bundle()),
    )
}

/// An instance of an instance: the part's overlap reported once.
#[test]
fn probe_a_nested_instance() {
    let (mut store, p) = part_of("probe-nested-part");
    let (mid, i) = inst(doc("probe-nested-mid"), p);
    let mid = set_product(&mid, i);
    let m = store.insert(mid, Tol::witness());
    let (host, j) = inst(doc("probe-nested-host"), m);
    let host = set_product(&host, j);
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    let product = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&host, &ev);
    eprintln!("PROBE A census={} reported={:#?}", product.census.len(), summary(&r));
    assert_eq!(r.len(), 1);
    assert!(product.census.is_empty());
}

/// A pick of a carried nested piece: its door's overlap once.
#[test]
fn probe_b_pick_of_a_nested_piece() {
    let (d, inner) = overlap("probe-pick");
    let (d, c) = block(d, [5.0, 0.0], 0.5, 1.0);
    let (d, c) = put(d, fixture::stood(c));
    let (d, outer) = assembled(d, &[inner, c]);
    let (d, placed) = put(d, fixture::stood(outer));
    let inner_var = d.output(inner, 0).unwrap();
    let (d, picked) = put(
        d,
        fixture::stood(editor_core::BodyRead::plain(Operand::pick(
            Operand::Node(placed),
            vec![inner_var],
        ))),
    );
    let d = set_product(&d, picked);
    let ev = run(&d, &EvalOptions::default());
    eprintln!("PROBE B err={:?}", ev.node_error(picked));
    let product = editor_core::product_recorded(&d, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&d, &ev);
    eprintln!("PROBE B census={} reported={:#?}", product.census.len(), summary(&r));
    assert!(product.census.is_empty(), "the pick's product decides nothing inside");
    assert_eq!(r.len(), 1);
}

/// A family whose members overlap each other, of a censused master:
/// the member-member meeting is the family `⊔`'s to find.
#[test]
fn probe_c_overlapping_members_of_a_censused_master() {
    let (d, s) = overlap("probe-members-overlap");
    let (d, fam) = patterned(d, s, 0.1);
    let (d, both) = assembled_family(d, fam);
    let d = set_product(&d, both);
    let ev = run(&d, &EvalOptions::default());
    let product = editor_core::product_recorded(&d, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&d, &ev);
    eprintln!("PROBE C census={} reported={:#?}", product.census.len(), summary(&r));
    assert!(r.len() >= 2, "the source's own overlap plus the members'");
}

/// The same with the family read by projection, spelled.
#[test]
fn probe_c2_overlapping_projected_members() {
    let (d, s) = overlap("probe-members-overlap-2");
    let (d, fam) = patterned(d, s, 0.1);
    let (d, a) = member(d, fam, 0);
    let (d, b) = member(d, fam, 1);
    let (d, both) = assembled(d, &[a, b]);
    let d = set_product(&d, both);
    let ev = run(&d, &EvalOptions::default());
    let product = editor_core::product_recorded(&d, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&d, &ev);
    eprintln!("PROBE C2 census={} reported={:#?}", product.census.len(), summary(&r));
    assert!(r.len() >= 2);
}

/// One instance read twice by a `⊔`: the part's finding once, and the
/// two copies' interference found.
#[test]
fn probe_d_instance_read_twice() {
    let (store, p) = part_of("probe-twice-part");
    let (host, i) = inst(doc("probe-twice-host"), p);
    let (host, both) = assembled(host, &[i, i]);
    let host = set_product(&host, both);
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    let product = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&host, &ev);
    eprintln!("PROBE D census={} reported={:#?}", product.census.len(), summary(&r));
    assert!(r.len() >= 2, "the part's own, and the two copies'");
}

/// A pattern of an instance whose members overlap: the host finds it.
#[test]
fn probe_e_pattern_of_instance_overlapping() {
    let (store, p) = part_of("probe-pat-inst-part");
    let (host, i) = inst(doc("probe-pat-inst-host"), p);
    let (host, fam) = patterned(host, i, 0.1);
    let (host, both) = assembled_family(host, fam);
    let host = set_product(&host, both);
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    let product = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&host, &ev);
    eprintln!("PROBE E census={} reported={:#?}", product.census.len(), summary(&r));
    assert!(r.len() >= 2);
}

/// A pattern of an instance, three members, disjoint: the part's once.
#[test]
fn probe_e2_pattern_of_instance_three() {
    let (store, p) = part_of("probe-pat3-part");
    let (host, i) = inst(doc("probe-pat3-host"), p);
    let (host, fam) = put(
        host,
        Node::Pattern {
            input: i.into(),
            count: editor_core::Formula::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let (host, placed) = assembled_family(host, fam);
    let host = set_product(&host, placed);
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    let product = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&host, &ev);
    eprintln!("PROBE E2 census={} carried={} reported={:#?}", product.census.len(), product.carried_interference.len(), summary(&r));
    assert_eq!(r.len(), 1);
}

/// A mirror of an instance whose part overlaps.
#[test]
fn probe_f_mirror_of_instance() {
    let (store, p) = part_of("probe-mirror-part");
    let (host, k) = block(doc("probe-mirror-host"), [-3.5, 0.0], 0.5, 1.0);
    let (host, k) = put(host, fixture::stood(k));
    let plane = fixture::plane_on(&host, k, [-3.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let (host, i) = inst(host, p);
    let (host, m) = put(
        host,
        Node::Mirror {
            body: Operand::output(i, 0).into(),
            plane,
        },
    );
    let (host, mp) = put(host, fixture::stood(m));
    let host = set_product(&host, mp);
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    eprintln!("PROBE F err={:?} {:?}", ev.node_error(m), ev.node_error(mp));
    let product = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&host, &ev);
    eprintln!("PROBE F census={} reported={:#?}", product.census.len(), summary(&r));
    assert_eq!(r.len(), 1, "the part's overlap, once");
}

/// A mirror of a `⊔` that overlaps, beside its source.
#[test]
fn probe_g_mirror_of_assemble_beside_source() {
    let (d, s) = overlap("probe-mirror-asm");
    let plane = fixture::plane_on(&d, s, [-3.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let (d, m) = put(
        d,
        Node::Mirror {
            body: Operand::output(s, 0).into(),
            plane,
        },
    );
    let (d, mp) = put(d, fixture::stood(m));
    let (d, both) = assembled(d, &[s, mp]);
    let d = set_product(&d, both);
    let ev = run(&d, &EvalOptions::default());
    eprintln!("PROBE G err={:?} {:?}", ev.node_error(m), ev.node_error(mp));
    let product = editor_core::product_recorded(&d, &ev, Tol::witness()).expect("gathers");
    let r = at_rest(&d, &ev);
    eprintln!("PROBE G census={} reported={:#?}", product.census.len(), summary(&r));
}

fn seat_part(label: &str) -> (fixture::resolver::PartStore, editor_core::DocRef) {
    let d = doc(label);
    let (d, plate) = block(d, [0.0; 2], 1.0, 0.2);
    let (d, plate) = put(d, fixture::stood(plate));
    let (d, bolt) = beside(d, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (d, seat) = assembled(d, &[bolt, plate]);
    let d = set_product(&d, seat);
    let mut store = fixture::resolver::PartStore::default();
    let r = store.insert(d, Tol::witness());
    (store, r)
}

/// A pick of an instance's piece: does it keep the part's seat record?
#[test]
fn probe_h_pick_of_an_instance_piece_keeps_records() {
    let (store, p) = seat_part("probe-pick-inst-part");
    let opts = fixture::resolver::with_resolver(store);
    let (host, i) = inst(doc("probe-pick-inst-host"), p);
    let direct = set_product(&host, i);
    let ev_d = run(&direct, &opts);
    let pd = editor_core::product_recorded(&direct, &ev_d, Tol::witness()).expect("gathers");
    let (host, c) = block(host, [5.0, 0.0], 0.5, 1.0);
    let (host, c) = put(host, fixture::stood(c));
    let (host, outer) = assembled(host, &[i, c]);
    let (host, placed) = put(host, fixture::stood(outer));
    let ivar = host.output(i, 0).unwrap();
    let (host, picked) = put(
        host,
        fixture::stood(editor_core::BodyRead::plain(Operand::pick(
            Operand::Node(placed),
            vec![ivar],
        ))),
    );
    let host = set_product(&host, picked);
    let ev = run(&host, &opts);
    eprintln!("PROBE H err={:?}", ev.node_error(picked));
    let pp = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("gathers");
    eprintln!(
        "PROBE H direct: contacts={} census={} unproven={} | pick: contacts={} census={} unproven={}",
        pd.contacts.rows().len(), pd.census.len(), unproven(&direct, &ev_d),
        pp.contacts.rows().len(), pp.census.len(), unproven(&host, &ev)
    );
    assert_eq!(pd.contacts.rows().len(), pp.contacts.rows().len(), "the pick keeps the part's records");
}

/// A cut of a pick of an instance piece vs a cut of the instance.
#[test]
fn probe_h2_cut_of_a_pick_of_an_instance_piece() {
    let (store, p) = seat_part("probe-cut-pick-part");
    let opts = fixture::resolver::with_resolver(store);
    let (host, i) = inst(doc("probe-cut-pick-host"), p);
    let (host, t) = block(host, [0.8, 0.8], 0.1, 0.1);
    let (host, t) = put(host, fixture::stood(t));
    let (direct, cut) = put(host.clone(), Node::Subtract { from: i.into(), tool: t.into() });
    let direct = set_product(&direct, cut);
    let ev_d = run(&direct, &opts);
    let (host, c) = block(host, [5.0, 0.0], 0.5, 1.0);
    let (host, c) = put(host, fixture::stood(c));
    let (host, outer) = assembled(host, &[i, c]);
    let (host, placed) = put(host, fixture::stood(outer));
    let ivar = host.output(i, 0).unwrap();
    let (host, picked) = put(
        host,
        fixture::stood(editor_core::BodyRead::plain(Operand::pick(
            Operand::Node(placed),
            vec![ivar],
        ))),
    );
    let (host, cut2) = put(host, Node::Subtract { from: picked.into(), tool: t.into() });
    let host = set_product(&host, cut2);
    let ev = run(&host, &opts);
    eprintln!("PROBE H2 errs={:?} {:?}", ev_d.node_error(cut), ev.node_error(cut2));
    let rd = at_rest(&direct, &ev_d);
    eprintln!("PROBE H2 direct ok: {:#?}", summary(&rd));
    let pp = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("gathers");
    eprintln!("PROBE H2 pick product census={} contacts={}", pp.census.len(), pp.contacts.rows().len());
    let rp = at_rest(&host, &ev);
    eprintln!("PROBE H2 direct: unproven={} at_rest={:#?}", unproven(&direct, &ev_d), summary(&rd));
    eprintln!("PROBE H2 pick: unproven={} at_rest={:#?}", unproven(&host, &ev), summary(&rp));
    assert_eq!(rd.len(), rp.len());
}

/// A cut of a pick of a local seat piece (no instance).
#[test]
fn probe_h3_cut_of_a_pick_of_a_local_seat() {
    let d = doc("probe-cut-pick-local");
    let (d, plate) = block(d, [0.0; 2], 1.0, 0.2);
    let (d, plate) = put(d, fixture::stood(plate));
    let (d, bolt) = beside(d, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (d, seat) = assembled(d, &[bolt, plate]);
    let (d, t) = block(d, [0.8, 0.8], 0.1, 0.1);
    let (d, t) = put(d, fixture::stood(t));
    let (d, c) = block(d, [5.0, 0.0], 0.5, 1.0);
    let (d, c) = put(d, fixture::stood(c));
    let (d, outer) = assembled(d, &[seat, c]);
    let (d, placed) = put(d, fixture::stood(outer));
    let svar = d.output(seat, 0).unwrap();
    let (d, picked) = put(
        d,
        fixture::stood(editor_core::BodyRead::plain(Operand::pick(
            Operand::Node(placed),
            vec![svar],
        ))),
    );
    let (d, cut) = put(d, Node::Subtract { from: picked.into(), tool: t.into() });
    let d = set_product(&d, cut);
    let ev = run(&d, &EvalOptions::default());
    eprintln!("PROBE H3 err={:?}", ev.node_error(cut));
    let rp = at_rest(&d, &ev);
    eprintln!("PROBE H3 unproven={} at_rest={:#?}", unproven(&d, &ev), summary(&rp));
}

/// A cut of an instance placed into a ⊔ and picked back, no other piece.
#[test]
fn probe_h4_cut_of_a_placement_of_an_instance() {
    let (store, p) = seat_part("probe-cut-place-inst-part");
    let opts = fixture::resolver::with_resolver(store);
    let (host, i) = inst(doc("probe-cut-place-inst-host"), p);
    let (host, t) = block(host, [0.8, 0.8], 0.1, 0.1);
    let (host, t) = put(host, fixture::stood(t));
    let (host, placed) = put(host, fixture::stood(i));
    let (host, cut) = put(host, Node::Subtract { from: placed.into(), tool: t.into() });
    let host = set_product(&host, cut);
    let ev = run(&host, &opts);
    let rp = at_rest(&host, &ev);
    eprintln!("PROBE H4 unproven={} at_rest={:#?}", unproven(&host, &ev), summary(&rp));
}

/// A pick of a piece of a moved copy stands where the copy stands.
#[test]
fn probe_m1_a_pick_of_a_moved_copy_stands_with_it() {
    let d = doc("probe-m1");
    let (d, a) = block(d, [0.0; 2], 0.5, 1.0);
    let (d, a) = put(d, fixture::stood(a));
    let (d, c) = beside(d, a, [3.0, 0.0, 0.0], 0.5, 1.0);
    let (d, c) = put(d, fixture::stood(c));
    let (d, outer) = assembled(d, &[a, c]);
    let (d, placed) = fixture::slid(d, outer, outer, [0.0, 7.0, 0.0]);
    let avar = d.output(a, 0).unwrap();
    let (d, picked) = put(
        d,
        fixture::stood(editor_core::BodyRead::plain(Operand::pick(
            Operand::Node(placed),
            vec![avar],
        ))),
    );
    let (d, a2) = block(d, [0.0; 2], 0.5, 1.0);
    let (d, a2) = put(d, fixture::stood(a2));
    let (d, both) = assembled(d, &[picked, a2]);
    let d = set_product(&d, both);
    let ev = run(&d, &EvalOptions::default());
    let r = at_rest(&d, &ev);
    eprintln!("PROBE M1 reported={:#?}", summary(&r));
    assert!(r.is_empty(), "the picked piece stands 7 off, clear of a fresh block at the origin");
}
