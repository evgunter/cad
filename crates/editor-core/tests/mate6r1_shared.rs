//! **R1 review probes for MATE-6, the tree-portable half.**
//!
//! Every row touches only `assemble`, `product_recorded`'s Ok/Err
//! discrimination, and `run_checks`, and every row ends in a
//! `R1-PROBE:` line and NO assertion. That shape was a differential
//! instrument while MATE-6 was a branch and its outputs could be
//! diffed against the merge base's. **The branch merged; there is no
//! second tree.** So nothing here can go red, and the printed answers
//! are the whole record —
//! `work/tint/mate6r1-shared-has-eleven-tests-and-no-assertions.md`
//! owns deciding, row by row, what assertion replaces the diff.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Formula;

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    AssemblyError, CapEnd, ChecksConfig, ContactClass, Coord, DocEdit, DocRef, DocumentId,
    EntityKind, Frame, MateRef, Node, Operand, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    TargetAxis, Value, assemble, product_recorded, run_checks,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, constrain_at, insert, len, mated_instance, on_frame, run, step};
use geom_core::Tol;

// ---- store / eval plumbing (the shared resolver, `fixture::resolver`) ----

fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
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

fn cube_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    )
}

/// A reference that resolves to NO product face: the part's extrude
/// has caps, but node 99 does not exist in it at all.
fn dangling(instance: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, 99),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            }
            .into(),
        }],
    }
}

/// A rest of `b`'s face on `a`'s, declaring `class`, standing `b`
/// `apart` beside `a` ([`Seat`]).
fn mate_of(a: StableName, b: StableName, apart: f64, class: ContactClass) -> Seat {
    Seat {
        mate: fixture::rest(fixture::head(b), fixture::head(a), class),
        apart,
    }
}

/// `b` sunk `depth` into `a`: `b`'s face on `a`'s plane slid `-depth`
/// along its own normal, facing it. A plane constructed off a face
/// declares no contact, so the overlap is geometry nothing declares.
fn sunk(a: StableName, b: StableName, depth: f64) -> Seat {
    let under = fixture::plane(fixture::head(a));
    let under = Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(under.clone()),
        along: RoleRead::through(under, PoseRole::Normal),
        by: len(-depth),
    }));
    Seat {
        mate: fixture::mate(
            fixture::plane(fixture::head(b)),
            fixture::flip(under),
            ContactClass::Rest,
        ),
        apart: 0.0,
    }
}

/// **A rest and how far beside its target it stands**: the copy's
/// own origin slid `apart` along x from the target's in the rested
/// plane. `0.0` stands the two unit cubes square, face to face; `2.0`
/// leaves the faces coplanar with a unit of clear air between their
/// trims, so the declaration is FALSE.
struct Seat {
    mate: editor_core::Mate<Formula>,
    apart: f64,
}

/// `seat`'s mate added to `placement`'s bundle with the three freedoms
/// a plane mate leaves set: a slide of `apart` along x, none along y,
/// and no spin, so it pins the copy.
fn seated_at(doc: ProfileDoc, placement: RecipeNodeId, seat: Seat) -> (ProfileDoc, MateRef) {
    let (doc, m) = constrain_at(doc, placement, seat.mate);
    let doc = [
        (Coord::Slide(TargetAxis::X), len(seat.apart)),
        (Coord::Slide(TargetAxis::Y), len(0.0)),
        (Coord::Spin, ang(0.0)),
    ]
    .into_iter()
    .fold(doc, |doc, (coord, by)| {
        step(
            doc,
            DocEdit::AddValue {
                placement: m.placement,
                value: Box::new(Value {
                    mate: Some(m.id),
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
        )
        .0
    });
    (doc, m)
}

/// [`seated_at`] on the placement the seat's `on` reads.
fn seated(doc: ProfileDoc, seat: Seat) -> (ProfileDoc, MateRef) {
    let placement = fixture::placed_by(&doc, &seat.mate);
    seated_at(doc, placement, seat)
}

/// `count` instances of `part`: the first pinned to the world, the rest
/// with empty bundles, each loose until a mate places it.
fn seated_row(
    label: &str,
    part: DocRef,
    count: usize,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (mut doc, first) = fixture::insert_pinned(
        doc,
        Node::instantiate_part(part),
        &with_resolver(store.clone()),
    );
    let mut ids = vec![first];
    for _ in 1..count {
        let (next, id) = insert(doc, mated_instance(part));
        doc = next;
        ids.push(id);
    }
    (doc, ids)
}

/// A two-cube stand: the lower cube pinned to the world, the upper one
/// stood on it by `seat`'s mate.
fn stand_with(
    label: &str,
    part: DocRef,
    body: RecipeNodeId,
    store: &PartStore,
    seat: impl FnOnce(StableName, StableName) -> Seat,
) -> ProfileDoc {
    let (inner, sub) = seated_row(label, part, 2, store);
    seated(
        inner,
        seat(
            in_part(sub[0], body, CapEnd::End),
            in_part(sub[1], body, CapEnd::Start),
        ),
    )
    .0
}

/// [`stand_with`] a `Rest` seat, the upper cube `apart` beside the
/// lower.
fn stand(
    label: &str,
    part: DocRef,
    body: RecipeNodeId,
    apart: f64,
    store: &PartStore,
) -> ProfileDoc {
    stand_with(label, part, body, store, |a, b| {
        mate_of(a, b, apart, ContactClass::Rest)
    })
}

/// `n` instances of `part`, each pinned `spacing * i` along +x.
fn row_of(
    label: &str,
    part: DocRef,
    count: usize,
    spacing: f64,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let o = with_resolver(store.clone());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..count {
        let (next, id) = insert(doc, mated_instance(part));
        #[allow(clippy::cast_precision_loss)]
        let dx = spacing * i as f64;
        doc = fixture::pin(next, id, &Frame::translation([dx, 0.0, 0.0]), &o);
        ids.push(id);
    }
    (doc, ids)
}

/// The refusal `assemble` raised, as a stable one-line tag. This is the
/// differential observable: the ARM plus the identity it names.
fn verdict(result: &Result<editor_core::Assembly<f64>, AssemblyError>) -> String {
    match result {
        Ok(a) => format!("Ok(minted={})", a.minted.len()),
        Err(AssemblyError::Mint { refusals }) => refusals
            .iter()
            .map(|r| match r {
                editor_core::MintRefusal::NoAtRestRecord { mate, class, .. } => {
                    format!("NoAtRestRecord(mate={mate:?}, class={class:?})")
                }
                editor_core::MintRefusal::Reference { mate, side, .. } => {
                    format!("Reference(mate={mate:?}, side={side:?})")
                }
                editor_core::MintRefusal::Unevaluated { mate, .. } => {
                    format!("Unevaluated(mate={mate:?})")
                }
            })
            .collect::<Vec<_>>()
            .join("+"),
        Err(AssemblyError::AtRest { findings }) => format!(
            "AtRest({} findings: {})",
            findings.len(),
            findings
                .iter()
                .map(|f| format!("{:?}", f.error))
                .map(|s| s.split_whitespace().next().unwrap_or("?").to_string())
                .collect::<Vec<_>>()
                .join(",")
        ),
        Err(AssemblyError::Uncertified { findings, .. }) => {
            format!("Uncertified({} findings)", findings.len())
        }
        Err(other) => format!("Other({other})"),
    }
}

// ============ CLAIM 3: mint totality vs. which refusal surfaces ============

/// PROBE (claim 3): a document with TWO bad mates. Old `mint` returned
/// on the first one it could not mint; new `mint` records both and
/// `assemble` raises `unminted.first()`. The raised arm and the mate it
/// names must be IDENTICAL across the two trees, or the deviation is
/// real. Both orderings are run, because "first in document order" is
/// only a claim if the order can change the answer.
#[test]
fn r1_two_bad_mates_noatrest_then_reference() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-2bad-a-cube"), Tol::witness());
    let (doc, ids) = seated_row("r1-2bad-a", part, 3, &store);
    // mate #1 (earlier in document order): Tangent -> NoAtRestRecord.
    let (doc, m1) = seated(
        doc,
        mate_of(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            0.0,
            ContactClass::Tangent,
        ),
    );
    // mate #2 (later): a dangling target, which its placement refuses
    // where it evaluates.
    let (doc, m2) = seated(
        doc,
        mate_of(
            dangling(ids[1]),
            in_part(ids[2], body, CapEnd::Start),
            0.0,
            ContactClass::Rest,
        ),
    );
    let ev = run(&doc, &with_resolver(store));
    let v = verdict(&assemble(&doc, &ev, Tol::witness()));
    println!(
        "R1-PROBE two_bad_mates_noatrest_then_reference m1={:?} m2={:?} => {v}",
        m1, m2
    );
}

/// The same pair, authored in the OPPOSITE document order.
#[test]
fn r1_two_bad_mates_reference_then_noatrest() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-2bad-b-cube"), Tol::witness());
    let (doc, ids) = seated_row("r1-2bad-b", part, 3, &store);
    let (doc, m1) = seated(
        doc,
        mate_of(
            dangling(ids[0]),
            in_part(ids[1], body, CapEnd::Start),
            0.0,
            ContactClass::Rest,
        ),
    );
    // Seated on the base, not on the copy the bad mate places, which
    // has no pose for a second copy to stand on.
    let (doc, m2) = seated(
        doc,
        mate_of(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[2], body, CapEnd::Start),
            0.0,
            ContactClass::Tangent,
        ),
    );
    let ev = run(&doc, &with_resolver(store));
    let v = verdict(&assemble(&doc, &ev, Tol::witness()));
    println!(
        "R1-PROBE two_bad_mates_reference_then_noatrest m1={:?} m2={:?} => {v}",
        m1, m2
    );
}

/// PROBE (claim 3, the divergence hunt): a BAD mate followed by a GOOD
/// one. Old `mint` returned before the good mate was ever read, so its
/// declaration never reached `contacts`; new `mint` continues and mints
/// it. `assemble` refuses either way (the bad mate is raised first), so
/// the ASSEMBLE verdict must still match — this row exists to show that
/// the totality change is invisible at this door.
#[test]
fn r1_a_good_mate_after_a_bad_one() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-goodafterbad-cube"), Tol::witness());
    let (doc, ids) = seated_row("r1-goodafterbad", part, 3, &store);
    let (doc, bad) = seated(
        doc,
        mate_of(
            dangling(ids[0]),
            in_part(ids[1], body, CapEnd::Start),
            0.0,
            ContactClass::Rest,
        ),
    );
    // Seated on the base, not on the copy the bad mate places, which
    // has no pose for a second copy to stand on.
    let (doc, good) = seated(
        doc,
        mate_of(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[2], body, CapEnd::Start),
            0.0,
            ContactClass::Rest,
        ),
    );
    let ev = run(&doc, &with_resolver(store));
    let v = verdict(&assemble(&doc, &ev, Tol::witness()));
    println!(
        "R1-PROBE good_mate_after_bad bad={:?} good={:?} => {v}",
        bad, good
    );
}

// ============ CLAIM 2: refusal PRECEDENCE ============

/// PROBE (claim 2): a document that fails a MINT precondition AND would
/// fail the tier-3′ census. Two cubes are seated exactly touching by a
/// `Tangent` mate — `Tangent` mints no at-rest record, so nothing
/// declares the contact the mate itself created, and the census would
/// answer `UnrecordedContact` if it ran.
///
/// Old order: `mint` returned `NoAtRestRecord` before
/// `gate_at_rest_declared` was ever called. New order: the gather
/// RECORDS the refusal and `assemble` raises it before the same gate.
/// If precedence is preserved the verdict is `NoAtRestRecord` on both
/// trees; if it inverted, this tree answers `AtRest(UnrecordedContact)`.
#[test]
fn r1_mint_refusal_precedes_the_census() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-prec-cube"), Tol::witness());
    let (doc, ids) = seated_row("r1-prec", part, 2, &store);
    let (doc, m) = seated(
        doc,
        mate_of(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            0.0,
            ContactClass::Tangent,
        ),
    );
    let ev = run(&doc, &with_resolver(store));
    // The gather must still ANSWER (record-not-raise), whatever
    // `assemble` does with the refusal.
    let gathers = product_recorded(&doc, &ev, Tol::witness()).is_ok();
    let v = verdict(&assemble(&doc, &ev, Tol::witness()));
    println!(
        "R1-PROBE mint_refusal_precedes_the_census mate={:?} gather_ok={gathers} => {v}",
        m
    );
}

// ==== CLAIMS 5+6: does a FALSE carried declaration go quiet anywhere? ====

/// PROBE (claims 5 and 6, the silent-wrongness attack): an inner stand
/// whose own mate declares a seat its geometry does not make (the cubes'
/// faces coplanar, their trims a unit apart), instantiated into an
/// outer document.
///
/// Two doors are asked, and they are asked SEPARATELY because they are
/// separately reachable by a user:
///
/// - `assemble` — the at-rest gate. It re-verifies, so it must be LOUD.
/// - `run_checks` — the advisory residents' lane, which reads
///   `gathered.contacts` and re-verifies NOTHING. `separation`
///   suppresses a `NotSeparated` finding for any pair a record
///   declares. On the merge base the inner mate's declaration was never
///   in the outer gather at all; after the change it is. If the count
///   drops, a FALSE declaration has bought silence in this resident.
#[test]
fn r1_false_carried_declaration_at_both_doors() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-false-cube"), Tol::witness());
    // Two units beside on a unit cube: the faces coplanar, their trims
    // a unit apart. The inner mate declares a Rest that the geometry
    // refutes.
    let inner = stand("r1-false-stand", part, body, 2.0, &store);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-false-outer", inner_ref, 1, 4.0, &store);

    let ev = run(&outer, &with_resolver(store.clone()));
    let gate = verdict(&assemble(&outer, &ev, Tol::witness()));
    let report = run_checks(&outer, &ev, &ChecksConfig::default(), Tol::witness())
        .expect("the registry runs");
    let sep = report
        .findings
        .iter()
        .filter(|f| format!("{:?}", f.check).contains("Separation"))
        .count();
    println!("R1-PROBE false_carried gate => {gate}");
    println!("R1-PROBE false_carried separation_findings => {sep}");
}

/// PROBE (claim 6, the control): the SAME outer shape with a TRUE inner
/// declaration (the cubes square). Whatever `separation` reports here is the
/// baseline the false-declaration row is read against.
#[test]
fn r1_true_carried_declaration_at_both_doors() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-true-cube"), Tol::witness());
    let inner = stand("r1-true-stand", part, body, 0.0, &store);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-true-outer", inner_ref, 1, 4.0, &store);

    let ev = run(&outer, &with_resolver(store.clone()));
    let gate = verdict(&assemble(&outer, &ev, Tol::witness()));
    let report = run_checks(&outer, &ev, &ChecksConfig::default(), Tol::witness())
        .expect("the registry runs");
    let sep = report
        .findings
        .iter()
        .filter(|f| format!("{:?}", f.check).contains("Separation"))
        .count();
    println!("R1-PROBE true_carried gate => {gate}");
    println!("R1-PROBE true_carried separation_findings => {sep}");
}

/// PROBE (claim 6, `declared_pairs` byte-identity): a SINGLE-document
/// assembly with one bad mate and one good one, run through
/// `run_checks`. On the merge base `declared_pairs` re-minted into a
/// clone and `mint` STOPPED at the bad mate, so the good mate that
/// follows it did not suppress. After the change the gather's set holds
/// the good one. A bad reference fails its placement, so the bad mate's
/// placement is put out of the product (its target is a loose copy),
/// where its failure refuses nothing the checks gather.
#[test]
fn r1_declared_pairs_with_a_bad_mate_before_a_good_one() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-dp-cube"), Tol::witness());
    let (doc, ids) = seated_row("r1-dp", part, 4, &store);
    // Bad mate FIRST (dangling, on the loose `ids[3]`), then a good Rest
    // on a later placement seating cube 2 on the base — a real,
    // declared contact.
    let (doc, _) = seated_at(
        doc,
        ids[1],
        mate_of(
            dangling(ids[3]),
            in_part(ids[1], body, CapEnd::Start),
            0.0,
            ContactClass::Rest,
        ),
    );
    let (doc, _) = seated(
        doc,
        mate_of(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[2], body, CapEnd::Start),
            0.0,
            ContactClass::Rest,
        ),
    );
    let ev = run(&doc, &with_resolver(store));
    // Connectedness reads every placement's value, loose ones included,
    // and would refuse the failed one before separation runs.
    let cfg = ChecksConfig {
        connectedness: editor_core::Severity::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks(&doc, &ev, &cfg, Tol::witness()).expect("the registry runs");
    let sep = report
        .findings
        .iter()
        .filter(|f| format!("{:?}", f.check).contains("Separation"))
        .count();
    println!("R1-PROBE declared_pairs_bad_before_good separation_findings => {sep}");
}

// ============ CLAIM 7: the no-mates document ============

/// PROBE (claim 7): a mate-less document's gather, rendered as a
/// content digest of everything the gather produces that existed on
/// both trees. Bit-identity is the claim; this row is the observable.
#[test]
fn r1_no_mates_document_digest() {
    let mut store = PartStore::default();
    let (part, _) = store.insert_part(cube_part("r1-bare-cube"), Tol::witness());
    let (doc, _) = row_of("r1-bare", part, 3, 4.0, &store);
    let ev = run(&doc, &with_resolver(store));
    let g = product_recorded(&doc, &ev, Tol::witness()).expect("gathers");
    println!(
        "R1-PROBE no_mates solids={} contacts={:?} roots={} names_empty={}",
        g.body.solids().count(),
        g.contacts,
        g.solid_copies.len(),
        format!("{:?}", g.names).len()
    );
}

// ============ CLAIM 4: the red-first numbers, exactly ============

/// PROBE (claim 4): issue 946's own shape at the size the PR quotes —
/// an inner stand instantiated ×3. The PR claims 24 `UnrecordedContact`
/// findings on the merge base and a green gate after, with 6 solids and
/// 3 carried patch records.
#[test]
fn r1_three_stands_exact_counts() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-x3-cube"), Tol::witness());
    let inner = stand("r1-x3-stand", part, body, 0.0, &store);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-x3-outer", inner_ref, 3, 4.0, &store);
    let ev = run(&outer, &with_resolver(store));
    let g = product_recorded(&outer, &ev, Tol::witness()).expect("gathers");
    let gate = verdict(&assemble(&outer, &ev, Tol::witness()));
    println!(
        "R1-PROBE three_stands solids={} patches={} => {gate}",
        g.body.solids().count(),
        g.contacts.face_face().count()
    );
}

/// PROBE (claim 5, the sharpest silent shape): an inner document whose
/// mate seats two cubes OVERLAPPING — the pair is genuinely too close
/// for `SolidSeparation` to certify, so `separation` HAS a finding to
/// make about it. A rest makes its faces coplanar, so no declaration
/// can claim interpenetrating faces: the mate sinking the cube is a
/// standoff, which declares nothing, and nothing may buy the pair
/// silence in a lane that re-verifies nothing.
#[test]
fn r1_overlapping_false_carried_declaration() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-ovl-cube"), Tol::witness());
    // Sunk half a unit into a unit cube: the two cubes INTERPENETRATE,
    // and nothing declares their caps.
    let inner = stand_with("r1-ovl-stand", part, body, &store, |a, b| sunk(a, b, 0.5));
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-ovl-outer", inner_ref, 1, 4.0, &store);

    let ev = run(&outer, &with_resolver(store.clone()));
    let gate = verdict(&assemble(&outer, &ev, Tol::witness()));
    let report = run_checks(&outer, &ev, &ChecksConfig::default(), Tol::witness());
    let sep = match &report {
        Ok(r) => r
            .findings
            .iter()
            .filter(|f| format!("{:?}", f.check).contains("Separation"))
            .count()
            .to_string(),
        Err(e) => format!("ChecksError({e})"),
    };
    println!("R1-PROBE overlapping_false_carried gate => {gate}");
    println!("R1-PROBE overlapping_false_carried separation_findings => {sep}");
}

/// PROBE (claim 5, closing the `checks` lane): TWO instances of the
/// overlapping stand, so `solid_copies` certainly
/// holds more than one row and `separation` certainly has pairs to
/// judge. Prints the root count beside the finding count, so a zero can
/// be read as "nothing to suppress" rather than "suppressed".
#[test]
fn r1_two_overlapping_false_stands() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("r1-ovl2-cube"), Tol::witness());
    let inner = stand_with("r1-ovl2-stand", part, body, &store, |a, b| sunk(a, b, 0.5));
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-ovl2-outer", inner_ref, 2, 4.0, &store);
    let ev = run(&outer, &with_resolver(store.clone()));
    let g = product_recorded(&outer, &ev, Tol::witness()).expect("gathers");
    let report = run_checks(&outer, &ev, &ChecksConfig::default(), Tol::witness());
    let sep = match &report {
        Ok(r) => r
            .findings
            .iter()
            .filter(|f| format!("{:?}", f.check).contains("Separation"))
            .count()
            .to_string(),
        Err(e) => format!("ChecksError({e})"),
    };
    println!(
        "R1-PROBE two_overlapping_false_stands roots={} solids={} patches={} separation_findings => {sep}",
        g.solid_copies.len(),
        g.body.solids().count(),
        g.contacts.face_face().count()
    );
}
