//! **ASM-R2b acceptance** — the assembly at-rest gate (acceptance rows
//! 1, 3–5 and 7; ASSEMBLY-DESIGN A4/A5/A13 clause 4).
//!
//! One assertion per acceptance row, each comment stating the
//! INVARIANT the row pins rather than the mechanics it exercises.
//!
//! A mate places a copy and declares nothing: the census records every
//! touch between copies, mated or not, as a row and a record citing it,
//! and the coincidence door proves a mated seat's row by its mate.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Dimension;
use editor_core::Formula;
use test_utils::refusal::tagged;

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, Coord, DocEdit, DocumentId, EntityKey, EntityKind, EntityRef, Entry, EvalOptions,
    FaceName, InterfaceCrossing, Mate, MateFault, MateRef, MateSide, Node, NodeErrorClass,
    NodeErrorKind, Operand, ProfileDoc, RecipeNodeId, RoleSeg, StableName, TargetAxis, content_pin,
    inline, product_recorded,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame, run, solve, split_world as split, step};
use geom_core::Tol;
use std::sync::Arc;

// ---- Evaluation through the shared part store ----

// ---- Documents ----

/// An axis-aligned block: `[x]×[y]` on its own xy, extruded `dz`. Its
/// extrude node is the returned id.
fn block(doc: ProfileDoc, x: (f64, f64), y: (f64, f64), dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

/// A one-block part document: `[0,1]³`, and its body.
fn cube_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    )
}

/// The corner-kiss part (`m4_pr5_glue`'s `kiss_base`, as a
/// document): `[0,1]³` ∪ `[1,2]³`, whose union DISCOVERS the v-v kiss
/// at (1,1,1) and records it, placed in its world. This is a part whose product carries
/// contact records of its own — row 1's subject.
fn kiss_part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (1.0, 2.0), 1.0);
    let (doc, b) = crate::fixture::slid(doc, a, b, [0.0, 0.0, 1.0]);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    crate::fixture::place(doc, union).0
}

/// **A seating mate and the freedoms it leaves, set**: a plane mate of
/// one face on another, and the in-plane slides of the seated copy's
/// own origin from its target's.
struct Seat {
    mate: Mate<Formula>,
    slide: [f64; 2],
}

/// A rest of instance `b`'s BOTTOM cap on instance `a`'s TOP cap, seating `b` at height `seat` over `a`'s base.
///
/// The rest's three freedoms are set by values ([`seat`]) because ONE
/// bare planar rest leaves a positive-dimensional residual, and a
/// bundle that pins less than the copy needs leaves `b` a loose copy
/// outside the product — the pair must be DETERMINED for `b` to join
/// the world at all, and this unit is about the at-rest check, not about
/// re-testing the coset fold.
/// `seat = 1.0` puts `b`'s bottom exactly on `a`'s top (the unit cube
/// is z ∈ [0,1]); any other height is a standoff, a plane constructed
/// off `a`'s cap, which places `b` and declares nothing.
fn rest_mate(body: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId, seat: f64) -> Seat {
    rest_mate_at(body, a, b, [0.0, 0.0, seat])
}

/// [`rest_mate`] with `b`'s origin placed anywhere over `a`'s, not just
/// up its axis — what a laterally offset seat needs (#1063's declined
/// and stale configurations both live off the axis): `origin`'s x and
/// y are the in-plane slides, its z the seat.
fn rest_mate_at(body: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId, origin: [f64; 3]) -> Seat {
    let top = fixture::plane(fixture::head_at(a, in_part(a, body, CapEnd::End)));
    let top = if origin[2] == 1.0 {
        top
    } else {
        Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::plain(top.clone()),
            along: RoleRead::through(top, PoseRole::Normal),
            by: len(origin[2] - 1.0),
        }))
    };
    Seat {
        mate: fixture::mate(
            fixture::plane(fixture::head_at(b, in_part(b, body, CapEnd::Start))),
            fixture::flip(top),
        ),
        slide: [origin[0], origin[1]],
    }
}

/// `seat`'s mate added to `placement`'s bundle with its three freedoms
/// set — the two slides and no spin — so it pins the copy.
fn seat(doc: ProfileDoc, placement: RecipeNodeId, seat: Seat) -> (ProfileDoc, MateRef) {
    let (doc, mate) = crate::fixture::mated(
        doc,
        placement,
        seat.mate,
        &[
            (Coord::Slide(TargetAxis::X), len(seat.slide[0])),
            (Coord::Slide(TargetAxis::Y), len(seat.slide[1])),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    (doc, mate)
}

/// Two instances of the unit cube, the first pinned to the world and
/// the second placed by the seating mate at `seat`. Returns (document,
/// instance ids, mate, store, the cube's body).
fn stacked(
    label: &str,
    seat: f64,
) -> (
    ProfileDoc,
    Vec<RecipeNodeId>,
    MateRef,
    PartStore,
    RecipeNodeId,
) {
    stacked_as(label, [0.0, 0.0, seat], |_| {})
}

/// [`stacked`] with `b`'s origin at `origin` over `a`'s
/// ([`rest_mate_at`]) and the seating mate rewritten by `edit` before it
/// enters the second instance's bundle — the one mate that places it.
fn stacked_as(
    label: &str,
    origin: [f64; 3],
    edit: impl FnOnce(&mut Mate<Formula>),
) -> (
    ProfileDoc,
    Vec<RecipeNodeId>,
    MateRef,
    PartStore,
    RecipeNodeId,
) {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(cube_part(&format!("{label}-part")), Tol::witness());
    let (doc, ids) = seated_pair(label, doc_ref, &store);
    let mut seating = rest_mate_at(body, ids[0], ids[1], origin);
    edit(&mut seating.mate);
    let (doc, mate) = seat(doc, ids[1], seating);
    (doc, ids, mate, store, body)
}

/// Two instances of `doc_ref` in a document named `label`: the first
/// pinned to the world at the identity, the second a loose copy until a
/// constraint places it. `store` holds the part.
fn seated_pair(
    label: &str,
    doc_ref: editor_core::DocRef,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(doc_ref),
        &with_resolver(store.clone()),
    );
    let (doc, seated) = insert(doc, fixture::mated_instance(doc_ref));
    (fixture::assemble_world_copies(doc), vec![base, seated])
}

/// `instance` standing on its bottom: by the identity bundle, its part's
/// start cap on the world's `xy` whatever the part's height.
fn standing_on_its_bottom(
    doc: ProfileDoc,
    instance: RecipeNodeId,
    _body: RecipeNodeId,
) -> ProfileDoc {
    fixture::stand_again(doc, instance)
}

/// An instance of `doc_ref` carrying `record`, pinned to the world at
/// the identity. `store` holds the part.
fn pinned_with(
    doc: ProfileDoc,
    doc_ref: editor_core::DocRef,
    record: editor_core::InterfaceRecord,
    store: &PartStore,
) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert_pinned(
        doc,
        Node::instantiate_part_with(
            doc_ref,
            record,
            editor_core::Operand::Var(editor_core::VarId::WORLD),
            editor_core::stand().bundle(),
        ),
        &with_resolver(store.clone()),
    )
}

/// **Every mate-placed copy pinned to the world where its bundle
/// solves it**: each such bundle removed — its values first, since a
/// value names its mate — and replaced by one pin at the solved pose.
fn pinned_where_solved(doc: ProfileDoc, o: &EvalOptions) -> ProfileDoc {
    let poses = solve(&doc, o, Tol::witness());
    let placed: Vec<(RecipeNodeId, editor_core::Frame)> = doc
        .placements()
        .into_iter()
        .filter(|&p| !fixture::bundle(&doc, p).is_empty() && !fixture::on_world(&doc, p))
        .filter_map(|p| Some((p, poses.placement(&doc, p).ok()?)))
        .collect();
    let mut doc = doc;
    for (placement, frame) in placed {
        doc = fixture::stand_again(doc, placement);
        doc = fixture::pin(doc, placement, &frame, o);
    }
    doc
}

/// **What the at-rest check reports over a document**, beside the
/// product it read.
#[derive(Debug)]
struct AtRest {
    product: editor_core::Product<f64>,
    findings: Vec<editor_core::CheckFinding>,
}

/// The at-rest check over `doc`, or the gather's refusal.
fn at_rest(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
) -> Result<AtRest, editor_core::ProductError> {
    let product = product_recorded(doc, ev, Tol::witness())?;
    let findings = fixture::at_rest(doc, ev).expect("the check runs over a product that gathers");
    Ok(AtRest { product, findings })
}

/// The census's own verdicts among what the at-rest check could not
/// look at, typed.
fn could_not_look_errors(at_rest: &AtRest) -> Vec<topo::ValidationError> {
    fixture::could_not_look(&at_rest.findings)
        .into_iter()
        .filter_map(|evidence| match evidence {
            editor_core::AtRestEvidence::NoLane(e)
            | editor_core::AtRestEvidence::Malformed(e)
            | editor_core::AtRestEvidence::InsideArrival(e) => Some(e),
            _ => None,
        })
        .collect()
}

/// The census verdicts the at-rest check could not look at; the empty
/// vector where it judged every pair.
fn findings(result: &Result<AtRest, editor_core::ProductError>) -> Vec<String> {
    match result {
        Ok(at_rest) => fixture::could_not_look(&at_rest.findings)
            .into_iter()
            .map(|evidence| match evidence {
                editor_core::AtRestEvidence::NoLane(e)
                | editor_core::AtRestEvidence::Malformed(e)
                | editor_core::AtRestEvidence::InsideArrival(e) => format!("{e:?}"),
                other => format!("{other:?}"),
            })
            .collect(),
        Err(other) => panic!("expected the product to gather, got {other}"),
    }
}

/// **The record set of a gathered product.**
fn gate_records(result: &Result<AtRest, editor_core::ProductError>) -> &topo::ContactRecords {
    match result {
        Ok(at_rest) => &at_rest.product.contacts,
        Err(other) => panic!("the product gathers: {other}"),
    }
}

fn names_of(table: &editor_core::NameTable, key: EntityKey) -> Option<StableName> {
    table
        .name_of(&EntityRef { body: 0, key })
        .cloned()
        .or_else(|| {
            table.iter().find_map(|(n, e)| match e {
                Entry::Unique(r) if r.key == key => Some(n.clone()),
                Entry::Tied(rs) if rs.iter().any(|r| r.key == key) => Some(n.clone()),
                _ => None,
            })
        })
}

/// One seam: the assembly's product name for a part entity is the
/// part's own product name, which the instance's copy carries verbatim.
fn unwrap_in_part(name: &StableName) -> StableName {
    name.clone()
}

// ---- Row 1: the contacts channel (D-1) ----

/// INVARIANT: a part's own contact records survive instantiation
/// into the assembly's record set, under the keys the GRAFT says its
/// entities became — lineage through the descendant map, never
/// re-derivation from the gathered geometry. Observably: the assembly
/// carries exactly the part's records, and each record still names the
/// same part entities (unwrapping the instance qualifier).
#[test]
fn row1_a_parts_declared_contacts_survive_instantiation() {
    let mut store = PartStore::default();
    let part = kiss_part("asm-r2b-row1-part");
    let doc_ref = store.insert(part.clone(), Tol::witness());

    // What the part's own product records, on its own keys.
    let part_ev = run(&part, &EvalOptions::default());
    let part_product = product_recorded(&part, &part_ev, Tol::witness()).expect("the part gathers");
    assert_eq!(
        part_product.contacts.vertex_vertex().count(),
        1,
        "the kiss union records one v-v contact: {:?}",
        part_product.contacts
    );
    let part_pair = {
        let (a, b) = part_product.contacts.vertex_vertex().next().unwrap();
        (
            names_of(&part_product.names, EntityKey::Vertex(a)),
            names_of(&part_product.names, EntityKey::Vertex(b)),
        )
    };

    // The same part, instantiated and pinned in the world.
    let o = with_resolver(store);
    let (doc, instance) = fixture::insert_pinned(
        ProfileDoc::empty(DocumentId::derive("asm-r2b-row1"), Tol::witness()),
        crate::fixture::mated_instance(doc_ref),
        &o,
    );
    let ev = run(&doc, &o);
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("the assembly gathers");
    let _ = instance;

    assert_eq!(
        product.contacts.vertex_vertex().count(),
        1,
        "the record crossed the seam, exactly once: {:?}",
        product.contacts
    );
    let (a, b) = product.contacts.vertex_vertex().next().unwrap();
    let moved = (
        names_of(&product.names, EntityKey::Vertex(a)).map(|n| unwrap_in_part(&n)),
        names_of(&product.names, EntityKey::Vertex(b)).map(|n| unwrap_in_part(&n)),
    );
    assert_eq!(
        moved, part_pair,
        "the carried record names the SAME part entities — the key \
         correspondence is the graft's descendant map, not a re-scan"
    );
}

// ---- Row 3: the F1 pair, both directions ----

/// INVARIANT (the unmated direction, D1 (iii), D10): two instances that
/// touch with no mate between them are no refusal. The gather records
/// each touch as a row of the two copies and a record citing it, which
/// the census's next pass confirms; the gate assembles, and the
/// `unproven-coincidence` check reads the rows. Red if a record cites
/// anything but a row of the two copies, or if the pair refuses.
#[test]
fn row3_a_an_unmated_touching_pair_is_rows_of_the_two_copies() {
    // The same touching geometry, with the mate REPLACED by a world
    // mate at the pose it solved — the mate is document data, so the
    // instances still touch and no mate relates them.
    let (doc, ids, mate, store, _) = stacked("asm-r2b-row3a", 1.0);
    let o = with_resolver(store);
    let doc = pinned_where_solved(doc, &o);
    assert!(
        fixture::bundle(&doc, mate.placement)
            .iter()
            .all(|c| c.id != mate.id),
        "the seating mate is gone"
    );
    let ev = run(&doc, &o);
    let product =
        editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("the pair gathers");
    assert!(
        !product.coincidences.is_empty(),
        "the contact is rows of the two copies: {:?} (instances {ids:?})",
        product.coincidences
    );
    let rows = u32::try_from(product.coincidences.len()).expect("few rows");
    assert!(
        !product.contacts.is_empty() && product.contacts.decided().iter().all(|&k| k < rows),
        "each record cites one of those rows: {:?}",
        product.contacts
    );
    at_rest(&doc, &ev).expect("an unmated touching pair assembles");
}

/// INVARIANT (the mated direction): the same geometry seated by its
/// mate is recorded the same way — one face–face record, citing a row
/// of the two copies — and the gate certifies it. The mate declares
/// nothing; it is what the coincidence door proves the row by.
#[test]
fn row3_b_the_mated_touching_pair_is_recorded_as_the_unmated_one_is() {
    let (doc, ids, _, store, body) = stacked("asm-r2b-row3b", 1.0);
    let ev = run(&doc, &with_resolver(store));
    let result = at_rest(&doc, &ev);
    let errs = findings(&result);
    assert!(errs.is_empty(), "the mated seat assembles: {errs:?}");
    let contacts = gate_records(&result);
    assert_eq!(
        contacts.face_face().count(),
        1,
        "one face–face record, of the seat: {contacts:?}"
    );
    let (fa, fb) = contacts.face_face().next().unwrap();
    let assembly = result.as_ref().expect("the seat certifies");
    // The product's `⊔` keys each copy's names by the read it came
    // through; what each wraps is the instance's copy's own name.
    let named = [fa, fb].map(|f| {
        names_of(&assembly.product.names, EntityKey::Face(f)).map(|n| match n.path.as_slice() {
            [RoleSeg::From { of, .. }] => (**of).clone(),
            _ => n,
        })
    });
    for want in [
        in_part(ids[1], body, CapEnd::Start),
        in_part(ids[0], body, CapEnd::End),
    ] {
        assert!(
            named.contains(&Some(want.clone().into())),
            "keyed to the seated caps: {named:?} lacks {want:?}"
        );
    }
}

/// INVARIANT (spec row 4's in-band clause): a seat whose gap is
/// AUTHORED INTO THE BAND — a standoff off the base's top, which places
/// the cube and declares nothing — is neither certified nor
/// contradicted. The two caps read one carrier in band, so they are one
/// face pair the census cannot look at (`[assertions]`), which hides no
/// overlap: one could-not-look finding, typed, once. Each
/// pair of side walls lies flush with its boundaries a band apart, the
/// flush door's finding, once; and the cube's vertical edges lying
/// along those walls escalate where they end in band of the base's top,
/// a reading no one point locates, so it stands.
///
/// The gap is DERIVED FROM THE COMMITTED BAND, never spelled as a
/// literal: the hosted matrix runs ε = 1e-12 and 1e-6 as well as the
/// default, and a hard-coded 3e-9 is definitely-separated in one lane
/// and definitely-zero in another — the row would pin an ε, not an
/// invariant. `Band::linear(Tol::witness())` is the same band the at-rest door
/// builds, and the geometric mean of its two thresholds is strictly
/// between them in every lane, which is exactly what "in band" means.
/// (Row adopted from the review's probe; the band-relative derivation
/// is the repo's own precedent for literal-ε rows.)
#[test]
fn row4_b_an_in_band_authored_gap_is_its_in_band_pairs() {
    let band =
        geom_core::predicate::Band::linear(Tol::witness()).expect("the committed band builds");
    let gap = (band.zero() * band.escalate()).sqrt();
    assert!(
        gap > band.zero() && gap < band.escalate(),
        "the derived gap is strictly inside the band in THIS lane: \
         {gap} vs [{}, {}]",
        band.zero(),
        band.escalate()
    );
    let (doc, _, _, store, _) = stacked("asm-r2b-row4b", 1.0 + gap);
    let ev = run(&doc, &with_resolver(store));
    let checked = at_rest(&doc, &ev).expect("the seat gathers");
    let findings = could_not_look_errors(&checked);
    let mut pairs: Vec<[topo::FaceKey; 2]> = findings
        .iter()
        .filter_map(|f| match *f {
            topo::ValidationError::CensusUndecidable {
                a: topo::EntityId::Face(a),
                b: topo::EntityId::Face(b),
                ..
            } => Some([a.min(b), a.max(b)]),
            _ => None,
        })
        .collect();
    assert!(
        findings.iter().any(|f| matches!(
            f,
            topo::ValidationError::CensusUndecidable {
                reason: topo::Undecidable::ThinInBand,
                ..
            }
        )),
        "the caps' pair hides no overlap: {findings:?}"
    );
    assert!(
        findings.iter().all(|f| matches!(
            f,
            topo::ValidationError::CensusUndecidable { .. }
                | topo::ValidationError::CensusEscalated { .. }
        )),
        "nothing is decided, either way: {findings:?}"
    );
    let found = pairs.len();
    pairs.sort();
    pairs.dedup();
    assert_eq!(pairs.len(), found, "each pair is one finding: {pairs:?}");
}

// ---- Rows 5 and 6: the crossing record ----
//
// A4 speaks of "every mate EDGE crossing the cut": a constraint whose
// placement is cut while the copy it reads stays, or the other way
// round. A cut takes whole placements, each with its bundle, so a cut
// of BOTH ends of a seated mate carries the constraint with them and
// nothing crosses; `row5_a` pins that, and what a cut of one end says.

/// INVARIANT: a cut that takes both ends of a seated mate — the base
/// and the instance the mate places, its bundle with it — carries the
/// constraint whole, so its crossing record is empty. A cut of the
/// placed instance alone leaves the copy its constraint reads behind,
/// and the split says so rather than splitting.
#[test]
fn row5_a_a_proper_mate_edge_cannot_cross_a_cut_and_split_says_so_both_ways() {
    let (doc, ids, _, store, _) = stacked("asm-r2b-row5", 1.0);
    let o = with_resolver(store);

    // The placed instance alone: its constraint reads the base, which
    // stays.
    let alone = split(
        &doc,
        &[ids[1]].into_iter().collect(),
        DocumentId::derive("asm-r2b-row5-alone"),
        Tol::witness(),
        o.resolver.as_ref(),
    );
    assert!(
        alone.is_err(),
        "a cut whose constraint reads a copy left behind refuses: {:?}",
        alone.map(|out| out.instance)
    );

    // Both instances: accepted, and nothing crosses.
    let out = split(
        &doc,
        &ids.iter().copied().collect(),
        DocumentId::derive("asm-r2b-row5-whole"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("a cut of both ends splits");
    let Some(Node::InstantiatePart { interface, .. }) = out.remainder.node(out.instance) else {
        panic!("the split minted an instance");
    };
    assert!(
        interface.is_empty(),
        "both ends of the EDGE moved together, so no declaration \
         crossed: {:?}",
        interface.crossings
    );
}

/// **A remainder document holding the two ends of a seated mate and
/// the remainder-side face name a crossing keeps** — the fixture
/// every hand-built record below starts from.
///
/// A crossing's one reference into this document is checked at the
/// insert door: the `outer` is a payload name (`Node::payload_names`
/// lists it, and the door checks its node is live). So a record
/// cannot go into an empty document — the face the mate kept has to
/// be on something that is there. A split mints exactly this shape:
/// the crossing mate is a constraint in the remainder, and its
/// remainder-side head is the `outer`.
fn remainder_with_a_neighbour(
    label: &str,
    doc_ref: editor_core::DocRef,
    body: RecipeNodeId,
    store: &PartStore,
) -> (ProfileDoc, FaceName) {
    let (doc, ids) = seated_pair(label, doc_ref, store);
    let neighbour = ids[0];
    let (doc, _) = seat(doc, ids[1], rest_mate(body, neighbour, ids[1], 1.0));
    // A remainder name, spelled on the live neighbour: an instance's
    // copy carries its part's names, in the part's id space.
    let outer = FaceName::new(fixture::resolver::in_world(neighbour, CapEnd::End))
        .expect("a crossing's references are face names");
    (doc, outer)
}

/// INVARIANT (A4's "does it actually fit" + A13 clause 4): an
/// instance carrying a crossing declaration RE-VERIFIES it against
/// the pinned part at every evaluation. Pre-move it resolves and the
/// instance evaluates; a pin move to a part that no longer names the
/// declared entity refuses TYPED, naming the crossing — the swap is
/// never accepted with the declaration quietly dropped.
#[test]
fn row5_b_a_pin_move_that_breaks_a_crossing_refuses_at_evaluation() {
    let part_id = DocumentId::derive("asm-r2b-row5b-part");
    let mut store = PartStore::default();
    let (part, body) = block(
        ProfileDoc::empty(part_id, Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let (doc_ref, body) = store.insert_part((part, body), Tol::witness());
    // The part-local name of the cube's end cap.
    let inner = FaceName::new(fixture::resolver::in_world(body, CapEnd::End))
        .expect("a crossing's references are face names");
    let (doc, outer) = remainder_with_a_neighbour("asm-r2b-row5b", doc_ref, body, &store);
    let outer_probe = (*outer).clone();
    let record = editor_core::InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            outer,
            inner: inner.clone(),
        }],
    };
    let (doc, instance) = pinned_with(doc, doc_ref, record, &store);

    let ev = run(&doc, &with_resolver(store.clone()));
    assert!(
        ev.node_error(instance).is_none(),
        "the crossing re-verifies against the pinned part: {:?}",
        ev.node_error(instance).map(ToString::to_string)
    );

    // The move: the SAME document id, re-modelled so the declared
    // entity's minting node is no longer the one the crossing names.
    // A leading profile shifts the extrude off the id `body` names — the
    // part still has a product, and the crossing's reference is simply
    // gone.
    let (shifted, _) = insert(
        ProfileDoc::empty(part_id, Tol::witness()),
        Node::Profile(crate::fixture::desc(vec![crate::fixture::square(
            5.0, 5.0, 0.5,
        )])),
    );
    let (shifted, shifted_body) = block(shifted, (0.0, 1.0), (0.0, 1.0), 1.0);
    let shifted = crate::fixture::place(shifted, shifted_body).0;
    assert_ne!(shifted_body, body, "the move re-mints the extrude");
    let new_pin = content_pin(&shifted, Tol::witness()).expect("the pin computes");
    store.replace_without_repinning(part_id, shifted);
    let moved = editor_core::apply(
        &doc,
        &DocEdit::UpdateReference {
            node: instance,
            new_pin,
        },
        Tol::witness(),
    )
    .expect("the pin moves")
    .doc;

    let ev = run(&moved, &with_resolver(store));
    let failure = ev.node_error(instance).expect("the moved pin refuses");
    let err = failure.to_string();
    assert_eq!(
        failure.kind.class(),
        NodeErrorClass::CrossingUnverified,
        "the refusal is the crossing that no longer fits: {err}"
    );
    // REVIEW PROBE (mateid-rv): the refusal built by the ONE
    // construction site names the crossing by its `outer`, not by its
    // `inner`. `the_crossing_refusal_is_a_named_node_error` renders a
    // hand-built value, so it cannot see which field `eval::wire`
    // hands in; swapping them there survives the whole suite without
    // this line. The two nodes differ (`outer` is minted in the
    // REMAINDER, `inner` in the part), so the rendering tells them
    // apart.
    assert!(
        err.contains(&format!(
            "the end cap of node {} on the remainder",
            test_utils::refusal::tag(outer_probe.node.0.digest())
        )),
        "the refusal names the crossing by its `outer`: {err}"
    );
    assert_ne!(
        outer_probe.node, inner.node,
        "the row needs the two references to be distinguishable in the prose"
    );
}

/// INVARIANT: inline CONSUMES the record — every crossing's part-side
/// reference must re-anchor onto a spliced local name — and then the
/// record dissolves with the instance it rode on.
#[test]
fn row5_c_inline_dissolves_the_crossing_record() {
    let part_id = DocumentId::derive("asm-r2b-row5c-part");
    let mut store = PartStore::default();
    let (part, body) = block(
        ProfileDoc::empty(part_id, Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let (doc_ref, body) = store.insert_part((part, body), Tol::witness());
    let inner = FaceName::new(fixture::resolver::in_world(body, CapEnd::End))
        .expect("a crossing's references are face names");
    let (doc, outer) = remainder_with_a_neighbour("asm-r2b-row5c", doc_ref, body, &store);
    let record = editor_core::InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate { outer, inner }],
    };
    let (doc, instance) = pinned_with(doc, doc_ref, record, &store);
    let before = doc.ids().len();
    let part_nodes = store.doc(doc_ref.id).ids().len();
    let back = inline(
        &doc,
        instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("inline succeeds");
    assert!(
        back.doc.node(instance).is_none(),
        "the instance is gone, and its record with it"
    );
    // What the SPLICE contributed, not what survived it: this
    // document also holds the two ends of the crossing mate, which
    // `inline` never touches, so `!order().is_empty()` would hold
    // however little came in. The count can only come from the part
    // (`ids` leaves out each document's world, which every document
    // has its own of), beside the new space the part's recipe stands in
    // and the placement of its copy under the instance's bundle.
    assert_eq!(
        back.doc.ids().len(),
        before - 1 + part_nodes + 2,
        "the instance went and the part's own recipe came in its place"
    );
}

/// INVARIANT (spec row 5's "a pin update that CHANGES the part's
/// contact face", the geometric half; review MINOR-2): a pin move that
/// leaves the recipe's node layout alone but MOVES the contact face is
/// invisible to the instance's own structural re-verification — the
/// name still resolves — and the mate reads the moved face: a mate
/// relates two poses read off the geometry, so the seated copy follows
/// the face and the seat stays a seat. The at-rest door, where a
/// geometric question belongs, records it again.
///
/// Both halves stated together so the boundary is not a gap: `row5_b`
/// pins the structural half at the wire, and this row pins the
/// geometric half at the at-rest check and at the solve. A crossing
/// declaration on a WIRE record has no geometric gate today, which is
/// the honest limit of D-5 and is stated in `wire_instantiate_part`'s
/// own docs.
#[test]
fn row5_e_a_pin_move_that_moves_the_contact_face_carries_the_seat_with_it() {
    let part_id = DocumentId::derive("asm-r2b-row5e-part");
    let mut store = PartStore::default();
    let (part, body) = block(
        ProfileDoc::empty(part_id, Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let (doc_ref, body) = store.insert_part((part.clone(), body), Tol::witness());
    // The stored part, placed: what the value edit below re-models.
    let part = crate::fixture::place(part, body).0;
    // The base stands on its bottom, so the pin move below moves its
    // top, the contact face, and nothing else.
    let doc = ProfileDoc::empty(DocumentId::derive("asm-r2b-row5e"), Tol::witness());
    let (doc, base) = insert(doc, crate::fixture::mated_instance(doc_ref));
    let doc = standing_on_its_bottom(doc, base, body);
    let (doc, seated) = insert(doc, fixture::mated_instance(doc_ref));
    let ids = [base, seated];
    let (doc, _) = seat(doc, ids[1], rest_mate(body, ids[0], ids[1], 1.0));

    // Pre-move: the seat assembles.
    let ev = run(&doc, &with_resolver(store.clone()));
    let before = findings(&at_rest(&doc, &ev));
    assert!(before.is_empty(), "pre-move the seat assembles: {before:?}");

    // The move: SAME node layout, different geometry — a value edit
    // makes the cube half as tall, so its top cap is at z = 0.5.
    let (shorter, _) = step(
        part,
        DocEdit::SetParam {
            node: body,
            slot: editor_core::SlotId::Distance,
            value: len(0.5).into(),
            fresh: Vec::new(),
        },
    );
    let new_pin = content_pin(&shorter, Tol::witness()).expect("the pin computes");
    store.replace_without_repinning(part_id, shorter);
    let moved = editor_core::apply(
        &doc,
        &DocEdit::UpdateReference {
            node: ids[0],
            new_pin,
        },
        Tol::witness(),
    )
    .expect("the pin moves")
    .doc;
    let moved = editor_core::apply(
        &moved,
        &DocEdit::UpdateReference {
            node: ids[1],
            new_pin,
        },
        Tol::witness(),
    )
    .expect("the pin moves")
    .doc;

    let o = with_resolver(store);
    let seated = solve(&moved, &o, Tol::witness())
        .placement(&moved, ids[1])
        .expect("the seated cube places");
    assert!(
        seated
            .translation
            .iter()
            .zip([0.0, 0.0, 0.5])
            .all(|(got, want)| (got - want).abs() <= 1e-12),
        "the seated cube's bottom follows the base's top down to z = 0.5: {:?}",
        seated.translation
    );
    let ev2 = run(&moved, &o);
    let after = findings(&at_rest(&moved, &ev2));
    assert!(after.is_empty(), "the moved seat assembles: {after:?}");
    let product = product_recorded(&moved, &ev2, Tol::witness()).expect("the pair gathers");
    assert_eq!(
        product.contacts.face_face().count(),
        1,
        "and the census records the seat the mate makes: {:?}",
        product.contacts
    );
}

// ---- Row 6: the content key feeds on the inhabited record ----

/// INVARIANT (the content-key half of ASM-4's hook obligation): a
/// crossing-record edit MOVES the instantiate node's content key. Two
/// documents differing ONLY in the record must not share a memo
/// entry — otherwise a crossing edit would be served the pre-edit
/// answer, re-verification and all.
#[test]
fn row6_a_crossing_record_edit_moves_the_content_key() {
    let part_id = DocumentId::derive("asm-r2b-row6-part");
    let mut store = PartStore::default();
    let (part, body) = block(
        ProfileDoc::empty(part_id, Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let (doc_ref, body) = store.insert_part((part, body), Tol::witness());
    let inner = FaceName::new(fixture::resolver::in_world(body, CapEnd::End))
        .expect("a crossing's references are face names");
    let (host, outer) = remainder_with_a_neighbour("asm-r2b-row6", doc_ref, body, &store);
    let record = editor_core::InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate { outer, inner }],
    };
    let (with, id_with) = pinned_with(host.clone(), doc_ref, record, &store);
    let (without, id_without) = pinned_with(
        host,
        doc_ref,
        editor_core::InterfaceRecord::default(),
        &store,
    );

    let key = |d: &ProfileDoc, id| {
        run(d, &with_resolver(store.clone()))
            .value(id)
            .expect("the instance evaluates")
            .content_key
    };
    assert_ne!(
        key(&with, id_with),
        key(&without, id_without),
        "the inhabited record feeds the key"
    );
}

/// REVIEW PROBE (mateid-rv): **each of a crossing's surviving fields
/// feeds the content key** — the row above only distinguishes an
/// inhabited record from an empty one, so dropping
/// `feed_stable_name(&mut h, outer)` from the instantiate arm
/// survives the whole `editor-core` suite. Two records differing in
/// the `outer` alone must key apart, or a crossing edit can hit the
/// memo on the pre-edit answer, which is the obligation the arm's own
/// comment claims (ASM-R2b D-4).
#[test]
fn a_crossing_record_keys_on_each_of_its_fields() {
    let part_id = DocumentId::derive("asm-r2b-row6-fields-part");
    let mut store = PartStore::default();
    let (part, body) = block(
        ProfileDoc::empty(part_id, Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let (doc_ref, body) = store.insert_part((part, body), Tol::witness());
    let part_face = |cap| {
        FaceName::new(fixture::resolver::in_world(body, cap))
            .expect("a crossing's references are face names")
    };
    let (host, outer) = remainder_with_a_neighbour("asm-r2b-row6-fields", doc_ref, body, &store);
    // The SAME live node, a different face of it: only the `outer`
    // moves between the two records below.
    let other_outer = FaceName::new(in_part(outer.node, body, CapEnd::Start).into())
        .expect("a crossing's references are face names");
    let record = |outer: FaceName, inner: FaceName| editor_core::InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate { outer, inner }],
    };
    let key = |record| {
        let (doc, id) = pinned_with(host.clone(), doc_ref, record, &store);
        run(&doc, &with_resolver(store.clone()))
            .value(id)
            .expect("the instance evaluates")
            .content_key
    };
    let base = key(record(outer.clone(), part_face(CapEnd::End)));
    assert_ne!(
        base,
        key(record(other_outer, part_face(CapEnd::End))),
        "the crossing's `outer` feeds the key"
    );
    assert_ne!(
        base,
        key(record(outer.clone(), part_face(CapEnd::Start))),
        "the crossing's `inner` feeds the key"
    );
}

// ---- Row 7: D9 determinism ----

/// INVARIANT (D9): a mated document's evaluation is a pure
/// function of the document — two independent evaluations produce the
/// bit-identical record set and the same at-rest verdict.
#[test]
fn row7_the_recorded_set_is_deterministic() {
    let (doc, _, _, store, _) = stacked("asm-r2b-row7", 1.0);
    let a = run(&doc, &with_resolver(store.clone()));
    let b = run(&doc, &with_resolver(store));
    let pa = product_recorded(&doc, &a, Tol::witness()).expect("gathers");
    let pb = product_recorded(&doc, &b, Tol::witness()).expect("gathers");
    assert_eq!(pa.contacts, pb.contacts, "the carried records replay");
    let ra = at_rest(&doc, &a);
    let rb = at_rest(&doc, &b);
    assert_eq!(
        findings(&ra),
        findings(&rb),
        "the at-rest verdict replays, finding for finding"
    );
    match (&ra, &rb) {
        (Ok(x), Ok(y)) => assert_eq!(
            x.product.contacts, y.product.contacts,
            "and so does the recorded set"
        ),
        (Err(_), Err(_)) => {}
        _ => panic!("two evaluations of one document disagreed on the verdict"),
    }
}

/// INVARIANT: a mate reference that names no face of the product
/// refuses typed — never resolved by picking, never widened. The name is
/// resolved at the target pose's selection, so the placement refuses
/// there, where it evaluates, and the gate does not let a mate with no
/// value pass silently.
#[test]
fn a_mate_reference_that_names_nothing_refuses_typed() {
    let (doc, mate, store) = {
        let mut store = PartStore::default();
        let (doc_ref, body) = store.insert_part(cube_part("asm-r2b-vanish-part"), Tol::witness());
        let (doc, ids) = seated_pair("asm-r2b-vanish", doc_ref, &store);
        // The target keeps its KIND — a face name that answers to
        // nothing, read at the base instance as a real cap is.
        let nothing = fixture::resolver::PartName {
            at: ids[0],
            name: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, tagged(99)),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
        };
        let mut seating = rest_mate(body, ids[0], ids[1], 1.0);
        seating.mate = fixture::rest(
            fixture::head_at(ids[1], in_part(ids[1], body, CapEnd::Start)),
            fixture::head(nothing),
        );
        let (doc, mate) = seat(doc, ids[1], seating);
        (doc, mate, store)
    };
    let ev = run(&doc, &with_resolver(store));
    assert!(
        matches!(
            ev.node_error(mate.placement).map(|e| &e.kind),
            Some(NodeErrorKind::Mate(fault)) if matches!(
                &**fault,
                MateFault::SideUnevaluated { mate: m, side: MateSide::To, refusal }
                    if *m == mate && matches!(refusal.kind(), NodeErrorKind::SelectResolve { .. })
            )
        ),
        "the target's selection refuses: {:?}",
        ev.node_error(mate.placement)
    );
    // The product's `⊔` reads the copy the constraint's placement would
    // place, so the gather meets the `⊔` poisoned through the failed
    // placement and refuses it typed.
    match at_rest(&doc, &ev) {
        Err(refusal) => assert!(
            matches!(
                refusal,
                editor_core::ProductError::Read(editor_core::NodeStanding::Poisoned {
                    through,
                    ..
                }) if through == mate.placement
            ),
            "the failed placement is the gather's refusal: {refusal:?}"
        ),
        other => panic!("an unresolvable reference must refuse typed: {other:?}"),
    }
}

/// Sanity: `NodeErrorKind::CrossingUnverified` is reachable as a typed
/// value, so the D-5 refusal is nameable by a caller (the pncad
/// binding lifts these by name) — and it says WHICH crossing by the
/// reference the remainder keeps, the only key a record has.
#[test]
fn the_crossing_refusal_is_a_named_node_error() {
    let e = NodeErrorKind::CrossingUnverified {
        instance: RecipeNodeId::new(0, tagged(1)),
        outer: Box::new(
            FaceName::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, tagged(2)),
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            })
            .expect("a crossing's references are face names"),
        ),
        name: Box::new(StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, tagged(3)),
            path: vec![RoleSeg::Cap(CapEnd::End)],
        }),
    };
    let msg = e.to_string();
    assert!(msg.contains("re-verify"), "{msg}");
    assert!(
        msg.contains("crosses at the start cap of node 000000000002"),
        "the refusal names the crossing by its `outer`: {msg}"
    );
}

// ---------------------------------------------------------------------
// The flush seat: a part seated so the two mated faces SHARE a
// boundary — the obvious way to draw a post under the end of a shelf,
// and the configuration that induces vertex-on-edge and edge-edge
// events beside the seat's face pair.
// ---------------------------------------------------------------------

/// A one-block part document with an arbitrary footprint, and its
/// body.
fn slab_part(label: &str, x: (f64, f64), y: (f64, f64), dz: f64) -> (ProfileDoc, RecipeNodeId) {
    block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        x,
        y,
        dz,
    )
}

/// The seat: a post standing under a shelf, mated top-cap to
/// underside, with the shelf's END plane and the post's cap sharing the
/// line x = 0. The post is inset in y, so the flush end is the whole
/// of the shared boundary.
///
/// One physical scene, stated independently at three layers: here at
/// the assembly gate, at census granularity (`flush_seat` in
/// `topo/tests/m9_c1_rest_face_rung.rs`), and as a user's document
/// (the bench-stand dimensions in `demos/tour/src/assembly.rs`). The
/// numbers agree by construction and are deliberately not shared —
/// each layer's fixture reads as the thing that layer is about.
fn flush_seat(label: &str) -> (ProfileDoc, MateRef, PartStore) {
    seat_off_the_end(label, 0.0)
}

/// [`flush_seat`] with the post's end standing `off` in from the
/// shelf's end plane.
fn seat_off_the_end(label: &str, off: f64) -> (ProfileDoc, MateRef, PartStore) {
    let mut store = PartStore::default();
    let (post, post_body) = store.insert_part(
        slab_part(&format!("{label}-post"), (off, 0.12), (0.09, 0.21), 0.5),
        Tol::witness(),
    );
    let (shelf, shelf_body) = store.insert_part(
        slab_part(&format!("{label}-shelf"), (0.0, 0.9), (0.0, 0.30), 0.04),
        Tol::witness(),
    );
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, post_id) = fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(post),
        &with_resolver(store.clone()),
    );
    let (doc, shelf_id) = insert(doc, fixture::mated_instance(shelf));
    let doc = fixture::assemble_world_copies(doc);
    let (doc, mate) = seat(
        doc,
        shelf_id,
        Seat {
            mate: fixture::rest(
                fixture::head_at(shelf_id, in_part(shelf_id, shelf_body, CapEnd::Start)),
                fixture::head_at(post_id, in_part(post_id, post_body, CapEnd::End)),
            ),
            slide: [0.0, 0.0],
        },
    );
    (doc, mate, store)
}

/// INVARIANT: a seat whose mated faces share a boundary is recorded as
/// one face pair, and the events it induces — a cap corner on the shelf
/// edge's interior, the collinear overlap they bound — are answered by
/// it. So the gate's verdict on the natural drawing of a bench is `Ok`.
#[test]
fn a_flush_seat_certifies_at_the_gate() {
    let (doc, _, store) = flush_seat("asm-r2b-flush");
    let ev = run(&doc, &with_resolver(store));
    let result = at_rest(&doc, &ev);
    assert!(
        findings(&result).is_empty(),
        "nothing is left unanswered: {:?}",
        findings(&result)
    );
    assert_eq!(
        gate_records(&result).face_face().count(),
        1,
        "the seat is recorded once, at face granularity"
    );
}

/// **A near-flush end is one finding.** The same seat with the post's
/// end standing in from the shelf's by a distance inside the band: the
/// two ends read one carrier in band, so they are one face pair the
/// census cannot look at (`[assertions]`): one could-not-look finding,
/// which refuses nothing. Nothing of the two ends' cells escalates
/// beside it, and the seat's own pair is recorded. Red if the check
/// reports anything but the one in-band pair.
#[test]
fn a_near_flush_seat_is_one_in_band_pair_it_could_not_look_at() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let off = 0.5 * (band.zero() + band.escalate());
    let (doc, _, store) = seat_off_the_end("asm-r2b-near-flush", off);
    let ev = run(&doc, &with_resolver(store));
    let checked = at_rest(&doc, &ev).expect("the seat gathers");
    let [
        editor_core::AtRestEvidence::NoLane(topo::ValidationError::CensusUndecidable {
            a: topo::EntityId::Face(_),
            b: topo::EntityId::Face(_),
            reason: topo::Undecidable::ThinInBand,
            ..
        }),
    ] = fixture::could_not_look(&checked.findings).as_slice()
    else {
        panic!("one finding, the in-band pair's: {:?}", checked.findings);
    };
    assert_eq!(checked.findings.len(), 1, "{:?}", checked.findings);
}

/// **Discovery is never a record** (F1/F2(iii)), and between copies
/// it is a finding rather than a refusal (D1 (iii), D10): the same
/// flush seat with its mate removed assembles, the gather records each
/// contact the census finds between the two instances as a row of the
/// product, and the `unproven-coincidence` check reports every one,
/// against the two copies; the records citing those rows quiet no
/// at-rest finding, which reports no contact. Red if the seat refuses,
/// if a contact reaches the product with no row, if the check proves
/// one, or if the at-rest check reports the touch.
#[test]
fn the_same_flush_seat_unmated_is_a_finding_not_a_refusal() {
    let (doc, mate, store) = flush_seat("asm-r2b-flush-bare");
    // The mate replaced by a world mate at the seat it solved: the
    // instances still touch, and no record backs it.
    let o = with_resolver(store);
    let doc = pinned_where_solved(doc, &o);
    assert!(
        fixture::bundle(&doc, mate.placement)
            .iter()
            .all(|c| c.id != mate.id),
        "the seating mate is gone"
    );
    let ev = run(&doc, &o);
    let product =
        editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("the seat gathers");
    assert!(
        product.census.is_empty(),
        "every finding was a contact between the copies: {:?}",
        product.census
    );
    assert!(
        !product.coincidences.is_empty()
            && product
                .coincidences
                .iter()
                .all(|r| r.row.site == topo::DecisionSite::CensusAtRest),
        "the seat's contacts are the copies' rows: {:?}",
        product.coincidences
    );
    let rows = product.coincidences.len();
    at_rest(&doc, &ev).expect("an unmated seat assembles");
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("checks run over the seat");
    let reported = report
        .findings
        .iter()
        .filter(|f| {
            f.check == editor_core::CheckId::UnprovenCoincidence
                && matches!(f.subject, editor_core::FindingSubject::Copies { .. })
        })
        .count();
    assert_eq!(reported, rows, "each row is reported: {report}");
    // A contact is the lint's, and only the lint's.
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.check != editor_core::CheckId::AtRest),
        "the at-rest check reports no contact: {report}"
    );
}

/// **A could-not-look finding forwards the kernel's verdict verbatim**:
/// the kernel's own `Display`, never a `Debug` dump. The kernel's
/// tier-3′ messages end in their own recourse, so the document layer
/// appends none.
#[test]
fn a_could_not_look_finding_renders_the_kernels_prose_never_debug_guts() {
    let msg = editor_core::AtRestEvidence::Malformed(topo::ValidationError::NegativeVolume {
        solid: topo::SolidKey::default(),
    })
    .to_string();
    assert_eq!(
        msg.matches("a solid encloses negative volume, so it is inside-out")
            .count(),
        1,
        "{msg}"
    );
    for guts in ["{", "NegativeVolume", "Malformed", "ValidationError"] {
        assert!(!msg.contains(guts), "Debug guts leaked ({guts}): {msg}");
    }
}

/// **The gather's own refusals render as prose too** — `ProductError`
/// forwards its payloads' `Display`s (the kernel refusal, each
/// validity finding on its own indented line, names as kind + minting
/// node), so a gather refusal leaks no `Debug` structure.
///
/// A key spelled `SolidKey(1v1)` in that prose is NOT what this row
/// bans: naming the entity a finding is about is `ValidationError`'s
/// house vocabulary, used by every variant of it that names one. What
/// is banned is the error TYPES' own `Debug` — the names in `guts`
/// below. The keys here are fabricated defaults, so the needle stops
/// at the prose stem rather than pinning a null key's spelling as
/// though it were user-facing text.
#[test]
fn the_gather_refusals_render_prose_never_debug_guts() {
    use editor_core::ProductError;

    let cases = vec![
        ProductError::PieceInvalid {
            findings: vec![editor_core::SourceFinding {
                node: RecipeNodeId::new(0, tagged(3)),
                output: 1,
                errors: vec![
                    topo::ValidationError::NegativeVolume {
                        solid: topo::SolidKey::default(),
                    },
                    topo::ValidationError::NegativeVolume {
                        solid: topo::SolidKey::default(),
                    },
                ],
            }],
        },
        ProductError::StrandedProduct {
            read: editor_core::VarId::new(0, tagged(2)),
        },
        ProductError::Graft {
            node: RecipeNodeId::new(0, tagged(5)),
            source: Box::new(topo::BooleanError::Band(geom_core::BandError::Empty {
                zero: 1.0,
                escalate: 0.5,
            })),
        },
    ];
    let expected: [&[&str]; 3] = [
        &[
            "product: 1 piece not valid at rest:",
            "\n  piece 000000000003 output 1: a solid encloses negative volume, so it is \
             inside-out",
        ],
        &["the product reads", "a body that is gone"],
        &["the kernel could not graft piece 000000000005's body: the band's "],
    ];
    for (error, needles) in cases.into_iter().zip(expected) {
        let msg = error.to_string();
        assert!(msg.starts_with("product:"), "{msg}");
        for needle in needles {
            assert!(msg.contains(needle), "{needle:?} not in: {msg}");
        }
        for guts in [
            "{",
            "PieceInvalid",
            "SourceFinding",
            "NegativeVolume",
            "ProductError",
            "ValidationError",
            "BooleanError",
            "StableName",
            "RecipeNodeId",
        ] {
            assert!(!msg.contains(guts), "Debug guts leaked ({guts}): {msg}");
        }
    }
}

/// INVARIANT: a correctly-mated assembly reports nothing at rest, and
/// it is the MATE that makes it so: the census recorded the touch, and
/// the coincidence door's first rung proves its row by the mate; the
/// same touch unmated is the lint's finding
/// (`the_same_flush_seat_unmated_is_a_finding_not_a_refusal`).
#[test]
fn a_mated_assembly_is_silent_and_the_mate_is_why() {
    let (doc, _ids, _mate, store, _) = stacked("asm-r2b-mated-silent", 1.0);
    let ev = run(&doc, &with_resolver(store));
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("gathers");
    assert!(
        !product.coincidences.is_empty(),
        "the premise: the census recorded the seat as a row"
    );
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("checks run over a completed evaluation");
    // The flush pairs the seat's side walls make are the lint's, and a
    // mate establishes no flush pair: only the rest is the mate's.
    assert!(
        report.findings.iter().all(|f| match &f.evidence {
            editor_core::CheckEvidence::UnprovenCoincidence { row, .. } =>
                row.relation == topo::Relation::SameOriented,
            _ => f.check != editor_core::CheckId::AtRest,
        }),
        "a mated assembly reports no rest and nothing at rest: {report}"
    );
}

/// A part document whose OWN product has two co-located copies: two
/// extrudes over the same block, never joined, each placed. Its product is one body
/// carrying two coincident solids — `diefillet.pncad`'s defect, one
/// document level down, and the only shape that puts two solids of ONE
/// subject where the box rule cannot separate them.
fn twinned_part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    crate::fixture::place_all(doc, &[a, b])
}

/// INVARIANT: the census reads nothing inside one arrival: two solids an
/// instance's one copy brings in are the part's own meeting, decided by
/// the part's census, and the host reports none of it as its own.
///
/// The fixture is an instantiated part whose own product has two
/// co-located copies, so the instance — itself the one placement — is a
/// single arrival carrying two COINCIDENT solids, and the only records
/// are the part's own, carried up from its gather — which the row
/// asserts rather than assumes.
#[test]
fn two_solids_of_one_arrival_are_the_parts_own_meeting() {
    let mut store = PartStore::default();
    let doc_ref = store.insert(twinned_part("asm-r2b-twinned-part"), Tol::witness());
    let o = with_resolver(store);
    let (doc, instance) = fixture::insert_pinned(
        ProfileDoc::empty(DocumentId::derive("asm-r2b-twinned"), Tol::witness()),
        crate::fixture::mated_instance(doc_ref),
        &o,
    );
    let placement = instance;
    let ev = run(&doc, &o);
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("gathers");

    // ONE subject, TWO solids — the configuration the guard is for.
    assert_eq!(product.solid_copies.len(), 2, "two solids");
    assert!(
        product
            .solid_copies
            .iter()
            .all(|o| (o.node, o.output) == (placement, 0)),
        "both solids come from the instance's one copy: {:?}",
        product.solid_copies
    );

    // And no mate's record relates them either: every record rode up
    // from the part, so the suppression under test is the subject check
    // and only the subject check.
    let contacts = &product.contacts;
    assert!(
        contacts.rows().iter().all(|record| record
            .cites
            .iter()
            .all(|backing| matches!(backing, topo::Backing::Carried { .. }))),
        "every record is the part's, carried: {contacts:?}"
    );

    // Therefore: nothing of the host's own; a part's finding is carried.
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("checks run over a completed evaluation");
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.check != editor_core::CheckId::AtRest
                || matches!(
                    &f.evidence,
                    editor_core::CheckEvidence::CarriedInterference { .. }
                        | editor_core::CheckEvidence::AtRest {
                            evidence: editor_core::AtRestEvidence::Carried(_)
                        }
                )),
        "two solids of one arrival are the part's meeting, not the host's: {report}"
    );
}

/// **A mate proves a row only in the sense it states.** The seated
/// pair's census records the two mated caps resting on each other,
/// opposed, and the door's first rung proves that row from the mate
/// alone (`Rung::SameConstruction`, no replay). The same cells read as
/// a same-side pair are not what the mate states, whose flip says the
/// caps face each other: the first rung leaves that row, whatever the
/// replay then answers.
///
/// Red if `stated_by_a_mate` stops comparing a row's sense with the
/// mate's flip.
#[test]
fn a_mate_proves_its_faces_only_in_the_sense_it_states() {
    let (doc, ids, _mate, store, _) = stacked_as("sense", [0.5, 0.5, 1.0], |_| {});
    let o = with_resolver(store);
    let ev = run(&doc, &o);
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("gathers");
    let caps = product
        .coincidences
        .iter()
        .map(|r| &r.row)
        .find(|row| {
            row.relation == topo::Relation::SameOpposite
                && row.cells.iter().all(|cell| {
                    matches!(cell, editor_core::NamedCell::Entity { name, .. }
                        if name.kind == EntityKind::Face)
                })
        })
        .expect("the census records the mated caps resting on each other")
        .clone();
    let same_side = editor_core::NamedCoincidence {
        relation: topo::Relation::SameOriented,
        ..caps.clone()
    };
    let proofs = editor_core::coincide::prove(
        &doc,
        &[(ids[1], &[caps, same_side][..])],
        ev.resolver(),
        Tol::witness(),
        &editor_core::CancelToken::new(),
    )
    .expect("not canceled");
    let first_rung = editor_core::Proof::Structural(editor_core::Rung::SameConstruction);
    assert_eq!(
        proofs[0][0], first_rung,
        "the mate proves the caps it rests"
    );
    assert_ne!(
        proofs[0][1], first_rung,
        "the mate does not prove its caps on the same side"
    );
}

/// The seating mate with its flip stripped: a plain plane mate, which
/// equates the two caps' outward normals, written under the orientation
/// that turns the copy over about `x`.
fn unflip(mate: &mut Mate<Formula>) {
    if let Operand::Pose(p) = &mate.to
        && let PoseDef::Flip { pose } = &**p
    {
        mate.to = pose.read.clone();
    }
    mate.orient = editor_core::Orientation::new(
        editor_core::SignedAxis::PlusX,
        editor_core::SignedAxis::MinusZ,
    )
    .expect("x and z lie along two axes");
}

/// **A seat written without its flip turns the copy over beside its
/// base, and the two abut.** The copy's start cap is set facing up on
/// the base's top plane, so the copy turns over about x and lands at
/// y in [-1, 0], sharing the base's y = 0 wall face to face. That is a
/// rest (one opposed row) beside four flush pairs (the caps and the
/// x walls, facing the same way with only their edges meeting), and no
/// overlap: the gate certifies it with no interference, mated or pinned
/// where it solved. Red if the copy lands anywhere else, or if a flush
/// pair's shared boundary is read as shared material.
#[test]
fn a_seat_without_its_flip_turns_the_copy_over_beside_its_base() {
    let (doc, _ids, _mate, store, _) = stacked_as("unflipped", [0.0, 0.0, 1.0], unflip);
    let o = with_resolver(store);
    let pinned = pinned_where_solved(doc.clone(), &o);
    for (label, doc) in [("mated", doc), ("pinned", pinned)] {
        let ev = run(&doc, &o);
        let product = product_recorded(&doc, &ev, Tol::witness()).expect("the pair gathers");
        let ys: Vec<f64> = product.body.vertex_points().map(|(_, p)| p.y).collect();
        assert!(
            ys.contains(&-1.0) && ys.iter().all(|&y| (-1.0..=1.0).contains(&y)),
            "{label}: the copy lies at y in [-1, 0], beside the base: {ys:?}"
        );
        let count = |relation| {
            product
                .coincidences
                .iter()
                .filter(|r| r.row.relation == relation)
                .count()
        };
        assert_eq!(
            (
                count(topo::Relation::SameOpposite),
                count(topo::Relation::SameOriented)
            ),
            (1, 4),
            "{label}: the y = 0 walls rest, and the caps and x walls lie flush"
        );
        let assembly = at_rest(&doc, &ev).expect("abutting copies gather");
        let interference = fixture::interference(&assembly.findings);
        assert!(
            interference.is_empty(),
            "{label}: abutting copies do not interfere: {interference:?}"
        );
    }
}

/// **Copies whose material coincides over an area are one interference.**
/// A seat at height 0 is a standoff that sets the copy exactly on the
/// base: the two copies fill one volume, with no vertex of either
/// strictly inside the other and nothing piercing, mated or pinned where
/// it solved. The seat without its flip at (0.5, 0.5, 1) turns the copy
/// over half across the base, the two caps of each sharing a quarter.
/// Each pair is one at-rest interference finding, bounded by the
/// intersection of the two copies, loud, and its evidence carries the
/// census's decided `SameSideInterference` and no `CensusUndecidable`.
/// Red if the census reports such a pair as could-not-look.
#[test]
fn copies_filling_one_volume_are_one_interference() {
    let (doc, _ids, _mate, store, _) = stacked_as("coincident", [0.0, 0.0, 0.0], |_| {});
    let o = with_resolver(store);
    let pinned = pinned_where_solved(doc.clone(), &o);
    let (overhang, _ids, _mate, store, _) = stacked_as("overhang", [0.5, 0.5, 1.0], unflip);
    let overhang_o = with_resolver(store);
    for (label, doc, o) in [
        ("mated", doc, &o),
        ("pinned", pinned, &o),
        ("overhang", overhang, &overhang_o),
    ] {
        let ev = run(&doc, o);
        let assembly = at_rest(&doc, &ev)
            .unwrap_or_else(|e| panic!("{label}: an interference is a finding: {e}"));
        let interference = fixture::interference(&assembly.findings);
        let [finding] = &interference[..] else {
            panic!("{label}: one interference: {interference:?}");
        };
        assert!(
            matches!(finding.overlap, editor_core::Overlap::Bounded { .. }) && finding.is_loud(),
            "{label}: bounded and loud: {finding:?}"
        );
        assert!(
            finding
                .evidence
                .iter()
                .any(|e| matches!(e, topo::ValidationError::SameSideInterference { .. }))
                && !finding
                    .evidence
                    .iter()
                    .any(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })),
            "{label}: decided by the coincident faces: {:?}",
            finding.evidence
        );
    }
}

/// A one-revolve part document: the unit square on its own `xy`
/// revolved a full turn about its `y`, a cylinder of radius 1 and
/// height 1 whose walls are two half-bands and whose caps are two discs.
fn cylinder_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        fixture::revolve_about(
            p,
            (0.0, 0.0),
            fixture::along_y(),
            fixture::ang(core::f64::consts::TAU),
        ),
    )
}

/// **Two copies of one revolved cylinder pinned at one pose fill one
/// volume, and the gate reads them as one interference**, at the
/// identity (the item's probe) and at a pose turned off every world axis
/// and moved off the origin. The copies' discs, bounded by an arc, and
/// their walls, on two keys, each lie on one carrier facing the same way
/// sharing interior: the finding carries the `SameSideInterference`. Its
/// overlap is bounded by the copies' intersection, the one cylinder,
/// and named by the first copy's four faces, the ones it keeps. It
/// is loud, and unquietable, since no face of one copy opposes a face of
/// the other for a gap to hold over, so stage 5's check never refuses
/// it. What the census leaves undecidable between
/// the two copies rides the finding as evidence and is never refused on
/// its own: a disc against the other copy's wall, two carriers meeting
/// along the rim. The two half-bands, one of each copy, that share a seam
/// ruling and no interior are certified apart, and ride it no longer.
///
/// Red if the copies read as could-not-look again (a refusal, or a
/// finding with no same-side evidence), if the overlap goes unbounded,
/// or if the undecidable pairs stop riding the decided pair.
#[test]
fn copies_of_one_cylinder_filling_one_volume_are_one_interference() {
    let tilted = editor_core::Frame::rotate_then_translate(
        [0.3, 0.2, 1.0],
        0.7,
        [0.31, -0.17, 0.42],
        geom_core::Band::linear(Tol::witness()).unwrap(),
    )
    .unwrap();
    for (label, pose) in [
        ("identity", editor_core::Frame::IDENTITY),
        ("tilted", tilted),
    ] {
        let mut store = PartStore::default();
        let (doc_ref, _) =
            store.insert_part(cylinder_part(&format!("{label}-part")), Tol::witness());
        let o = with_resolver(store);
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, a) = insert(doc, fixture::mated_instance(doc_ref));
        let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
        let doc = fixture::pin(doc, a, &pose, &o);
        let doc = fixture::assemble_world_copies(fixture::pin(doc, b, &pose, &o));
        let ev = run(&doc, &o);
        let assembly = at_rest(&doc, &ev)
            .unwrap_or_else(|e| panic!("{label}: an interference is a finding: {e}"));
        let interference = fixture::interference(&assembly.findings);
        let [finding] = &interference[..] else {
            panic!("{label}: one interference: {interference:?}");
        };
        let editor_core::Overlap::Bounded { faces } = &finding.overlap else {
            panic!("{label}: the overlap is bounded: {finding:?}");
        };
        assert!(
            finding.is_loud() && !finding.quietable && !finding.refusable(),
            "{label}: loud and unquietable: {finding:?}"
        );
        // Each copy's faces lie on the other's, and the intersection keeps
        // the first copy's (Eq. 15.3): the overlap is named by those four,
        // the two discs and the two half-bands.
        assert!(
            faces.len() == 4 && faces.iter().all(|f| f.copy == finding.a),
            "{label}: the first copy's four faces name the overlap: {faces:?}"
        );
        let count = |f: fn(&topo::ValidationError) -> bool| {
            finding.evidence.iter().filter(|&e| f(e)).count()
        };
        let same_side = count(|e| matches!(e, topo::ValidationError::SameSideInterference { .. }));
        let undecided = count(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. }));
        assert_eq!(
            (same_side, undecided, finding.evidence.len()),
            (1, 8, 9),
            "{label}: one decided pair, and the rim pairs riding it: {:?}",
            finding.evidence
        );
    }
}
