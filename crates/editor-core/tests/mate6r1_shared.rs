//! **R1 review probes for MATE-6, the tree-portable half.**
//!
//! Every row here compiles and runs on BOTH the merge base and the
//! MATE-6 branch: it touches only `assemble`, `product_recorded`'s
//! Ok/Err discrimination, and `run_checks` — never `Product::minted`
//! or `Product::unminted`, which exist only after the change. That is
//! deliberate: these rows are the DIFFERENTIAL instrument. Each one
//! prints a `R1-PROBE:` line, so the two trees' outputs can be diffed
//! verbatim rather than compared by narration.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CancelToken, CapEnd, ChecksConfig, ContactClass, DocEdit,
    DocRef, DocumentId, EntityKind, EvalOptions, Evaluation, Frame, MateFrame, MatePrimitive, Node,
    ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, StableName, assemble,
    content_pin, evaluate, product_recorded, run_checks,
};
use fixture::{desc, insert, len, step};
use geom_core::Tol;

// ---- store / eval plumbing (ASM-R2a's shape, as the unit's own suite) ----

#[derive(Debug, Default, Clone)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl editor_core::PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        if content_pin(doc, Tol::witness()).expect("the pin computes") != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

fn opts(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
}

fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

fn cube_part(label: &str) -> ProfileDoc {
    let (doc, _) = block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    doc
}

fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

/// A reference that resolves to NO product face: the part's node 1 has
/// caps, but node 99 does not exist in it at all.
fn dangling(instance: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(99),
                path: vec![RoleSeg::Cap(CapEnd::Top)],
            }),
        }],
    }
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    }
}

fn mate_of(
    a: StableName,
    b: StableName,
    seat: f64,
    class: ContactClass,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class,
        alignment: Alignment {
            a: frame([0.0, 0.0, seat], [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// `n` instances of `part`, each displaced `spacing * i` along +x.
fn row_of(
    label: &str,
    part: DocRef,
    count: usize,
    spacing: f64,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..count {
        let (next, id) = insert(doc, Node::instantiate_part(part));
        doc = next;
        if i > 0 {
            #[allow(clippy::cast_precision_loss)]
            let dx = spacing * i as f64;
            let (next, _) = step(
                doc,
                DocEdit::SetPlacement {
                    node: id,
                    frame: Frame::translation([dx, 0.0, 0.0]),
                },
            );
            doc = next;
        }
        ids.push(id);
    }
    (doc, ids)
}

/// The refusal `assemble` raised, as a stable one-line tag. This is the
/// differential observable: the ARM plus the identity it names.
fn verdict(result: &Result<editor_core::Assembly<f64>, AssemblyError>) -> String {
    match result {
        Ok(a) => format!("Ok(minted={})", a.minted.len()),
        Err(AssemblyError::NoAtRestRecord { mate, class, .. }) => {
            format!("NoAtRestRecord(mate={}, class={class:?})", mate.0)
        }
        Err(AssemblyError::Reference { mate, side, .. }) => {
            format!("Reference(mate={}, side={side:?})", mate.0)
        }
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
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-2bad-a-cube"), Tol::witness());
    let (doc, ids) = row_of("r1-2bad-a", part, 3, 4.0);
    // mate #1 (earlier in document order): Tangent -> NoAtRestRecord.
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(ids[0], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                1.0,
                ContactClass::Tangent,
            ),
        },
    );
    // mate #2 (later): a dangling reference -> Reference.
    let (doc, m2) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                dangling(ids[1]),
                in_part(ids[2], CapEnd::Bottom),
                1.0,
                ContactClass::Rest,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    let v = verdict(&assemble(&doc, &ev, Tol::witness()));
    println!(
        "R1-PROBE two_bad_mates_noatrest_then_reference m1={:?} m2={:?} => {v}",
        m1.map(|m| m.0),
        m2.map(|m| m.0)
    );
}

/// The same pair, authored in the OPPOSITE document order.
#[test]
fn r1_two_bad_mates_reference_then_noatrest() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-2bad-b-cube"), Tol::witness());
    let (doc, ids) = row_of("r1-2bad-b", part, 3, 4.0);
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                dangling(ids[0]),
                in_part(ids[1], CapEnd::Bottom),
                1.0,
                ContactClass::Rest,
            ),
        },
    );
    let (doc, m2) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(ids[1], CapEnd::Top),
                in_part(ids[2], CapEnd::Bottom),
                1.0,
                ContactClass::Tangent,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    let v = verdict(&assemble(&doc, &ev, Tol::witness()));
    println!(
        "R1-PROBE two_bad_mates_reference_then_noatrest m1={:?} m2={:?} => {v}",
        m1.map(|m| m.0),
        m2.map(|m| m.0)
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
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-goodafterbad-cube"), Tol::witness());
    let (doc, ids) = row_of("r1-goodafterbad", part, 3, 4.0);
    let (doc, bad) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                dangling(ids[0]),
                in_part(ids[1], CapEnd::Bottom),
                1.0,
                ContactClass::Rest,
            ),
        },
    );
    let (doc, good) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(ids[1], CapEnd::Top),
                in_part(ids[2], CapEnd::Bottom),
                1.0,
                ContactClass::Rest,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    let v = verdict(&assemble(&doc, &ev, Tol::witness()));
    println!(
        "R1-PROBE good_mate_after_bad bad={:?} good={:?} => {v}",
        bad.map(|m| m.0),
        good.map(|m| m.0)
    );
}

// ============ CLAIM 2: refusal PRECEDENCE ============

/// PROBE (claim 2): a document that fails a MINT precondition AND would
/// fail the tier-3′ census. Two cubes are seated exactly touching by a
/// `Tangent` mate — `Tangent` mints no at-rest record, so nothing
/// declares the contact the mate itself created, and the census would
/// answer `UndeclaredContact` if it ran.
///
/// Old order: `mint` returned `NoAtRestRecord` before
/// `gate_at_rest_declared` was ever called. New order: the gather
/// RECORDS the refusal and `assemble` raises it before the same gate.
/// If precedence is preserved the verdict is `NoAtRestRecord` on both
/// trees; if it inverted, this tree answers `AtRest(UndeclaredContact)`.
#[test]
fn r1_mint_refusal_precedes_the_census() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-prec-cube"), Tol::witness());
    let (doc, ids) = row_of("r1-prec", part, 2, 4.0);
    let (doc, m) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(ids[0], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                1.0,
                ContactClass::Tangent,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    // The gather must still ANSWER (record-not-raise), whatever
    // `assemble` does with the refusal.
    let gathers = product_recorded(&doc, &ev, Tol::witness()).is_ok();
    let v = verdict(&assemble(&doc, &ev, Tol::witness()));
    println!(
        "R1-PROBE mint_refusal_precedes_the_census mate={:?} gather_ok={gathers} => {v}",
        m.map(|m| m.0)
    );
}

// ==== CLAIMS 5+6: does a FALSE carried declaration go quiet anywhere? ====

/// PROBE (claims 5 and 6, the silent-wrongness attack): an inner stand
/// whose own mate declares a seat its geometry does not make (the cubes
/// are a half unit apart), instantiated into an outer document.
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
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-false-cube"), Tol::witness());
    // seat 1.5 on a unit cube: a definite half-unit gap. The inner mate
    // declares a Rest that the geometry refutes.
    let mut inner = ProfileDoc::empty(DocumentId::derive("r1-false-stand"), Tol::witness());
    let mut sub = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(inner, Node::instantiate_part(part));
        inner = next;
        sub.push(id);
    }
    let (inner, _) = step(
        inner,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(sub[0], CapEnd::Top),
                in_part(sub[1], CapEnd::Bottom),
                1.5,
                ContactClass::Rest,
            ),
        },
    );
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-false-outer", inner_ref, 1, 4.0);

    let ev = run(&outer, &opts(store.clone()));
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
/// declaration (seat 1.0). Whatever `separation` reports here is the
/// baseline the false-declaration row is read against.
#[test]
fn r1_true_carried_declaration_at_both_doors() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-true-cube"), Tol::witness());
    let mut inner = ProfileDoc::empty(DocumentId::derive("r1-true-stand"), Tol::witness());
    let mut sub = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(inner, Node::instantiate_part(part));
        inner = next;
        sub.push(id);
    }
    let (inner, _) = step(
        inner,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(sub[0], CapEnd::Top),
                in_part(sub[1], CapEnd::Bottom),
                1.0,
                ContactClass::Rest,
            ),
        },
    );
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-true-outer", inner_ref, 1, 4.0);

    let ev = run(&outer, &opts(store.clone()));
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
/// both. This row is the one that can show the resident going quieter.
#[test]
fn r1_declared_pairs_with_a_bad_mate_before_a_good_one() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-dp-cube"), Tol::witness());
    let (doc, ids) = row_of("r1-dp", part, 3, 4.0);
    // Bad mate FIRST (dangling), then a good Rest seating cube 2 on
    // cube 1 — a real, declared contact.
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                dangling(ids[0]),
                in_part(ids[1], CapEnd::Bottom),
                1.0,
                ContactClass::Rest,
            ),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(ids[1], CapEnd::Top),
                in_part(ids[2], CapEnd::Bottom),
                1.0,
                ContactClass::Rest,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    let report =
        run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).expect("the registry runs");
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
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-bare-cube"), Tol::witness());
    let (doc, _) = row_of("r1-bare", part, 3, 4.0);
    let ev = run(&doc, &opts(store));
    let g = product_recorded(&doc, &ev, Tol::witness()).expect("gathers");
    println!(
        "R1-PROBE no_mates solids={} contacts={:?} roots={} names_empty={}",
        g.body.solids().count(),
        g.contacts,
        g.solid_roots.len(),
        format!("{:?}", g.names).len()
    );
}

// ============ CLAIM 4: the red-first numbers, exactly ============

/// PROBE (claim 4): issue 946's own shape at the size the PR quotes —
/// an inner stand instantiated ×3. The PR claims 24 `UndeclaredContact`
/// findings on the merge base and a green gate after, with 6 solids and
/// 3 carried patch records.
#[test]
fn r1_three_stands_exact_counts() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-x3-cube"), Tol::witness());
    let mut inner = ProfileDoc::empty(DocumentId::derive("r1-x3-stand"), Tol::witness());
    let mut sub = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(inner, Node::instantiate_part(part));
        inner = next;
        sub.push(id);
    }
    let (inner, _) = step(
        inner,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(sub[0], CapEnd::Top),
                in_part(sub[1], CapEnd::Bottom),
                1.0,
                ContactClass::Rest,
            ),
        },
    );
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-x3-outer", inner_ref, 3, 4.0);
    let ev = run(&outer, &opts(store));
    let g = product_recorded(&outer, &ev, Tol::witness()).expect("gathers");
    let gate = verdict(&assemble(&outer, &ev, Tol::witness()));
    println!(
        "R1-PROBE three_stands solids={} patches={} => {gate}",
        g.body.solids().count(),
        g.contacts.patches.len()
    );
}

/// PROBE (claim 5, the sharpest silent shape): an inner document whose
/// mate declares a `Rest` between two cubes it seats OVERLAPPING — the
/// declaration names a real pair, and the pair is genuinely too close
/// for `SolidSeparation` to certify, so `separation` HAS a finding to
/// make about it. The declaration is nonetheless false: interpenetrating
/// faces are not a rest contact.
///
/// This is the pair the previous rows could not reach: a false carried
/// declaration whose named pair the advisory resident would otherwise
/// report. If `separation` reports on the merge base and not here, the
/// false declaration bought silence in a lane that re-verifies nothing.
#[test]
fn r1_overlapping_false_carried_declaration() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-ovl-cube"), Tol::witness());
    let mut inner = ProfileDoc::empty(DocumentId::derive("r1-ovl-stand"), Tol::witness());
    let mut sub = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(inner, Node::instantiate_part(part));
        inner = next;
        sub.push(id);
    }
    // seat 0.5 on a unit cube: the two cubes INTERPENETRATE, and the
    // Rest declaration over their caps is a lie about that geometry.
    let (inner, _) = step(
        inner,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(sub[0], CapEnd::Top),
                in_part(sub[1], CapEnd::Bottom),
                0.5,
                ContactClass::Rest,
            ),
        },
    );
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-ovl-outer", inner_ref, 1, 4.0);

    let ev = run(&outer, &opts(store.clone()));
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
/// overlapping-and-falsely-declared stand, so `solid_roots` certainly
/// holds more than one row and `separation` certainly has pairs to
/// judge. Prints the root count beside the finding count, so a zero can
/// be read as "nothing to suppress" rather than "suppressed".
#[test]
fn r1_two_overlapping_false_stands() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1-ovl2-cube"), Tol::witness());
    let mut inner = ProfileDoc::empty(DocumentId::derive("r1-ovl2-stand"), Tol::witness());
    let mut sub = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(inner, Node::instantiate_part(part));
        inner = next;
        sub.push(id);
    }
    let (inner, _) = step(
        inner,
        DocEdit::InsertNode {
            node: mate_of(
                in_part(sub[0], CapEnd::Top),
                in_part(sub[1], CapEnd::Bottom),
                0.5,
                ContactClass::Rest,
            ),
        },
    );
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1-ovl2-outer", inner_ref, 2, 4.0);
    let ev = run(&outer, &opts(store.clone()));
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
        g.solid_roots.len(),
        g.body.solids().count(),
        g.contacts.patches.len()
    );
}
