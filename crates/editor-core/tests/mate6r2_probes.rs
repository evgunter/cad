//! MATE-6 R2 review probes — cross-tree diff rows.
//!
//! These rows are written to COMPILE ON BOTH TREES (main and the
//! MATE-6 head): they never touch `Product::minted` / `unminted`.
//! Each prints a `P<n>:`-tagged line; the review runs the file on
//! both trees with `--nocapture` and diffs the tagged lines.
//!
//! P1/P2 — refusal precedence and identity with MULTIPLE bad mates
//!         (claims 2 and 3): first bad mate in document order wins,
//!         and the mint refusal preempts the declared gate.
//! P3 — the checks resident over the ×3-stand seam document
//!         (claims 4/6): declared-pair suppression across the seam.
//! P4 — the checks resident over a single correctly-mated document
//!         (claim 6's byte-identical claim, executed).
//! P5 — the checks resident with an unmintable mate BEFORE a good
//!         one (the disclosed wart: later mates now suppress).
//! P6 — a carried declaration over PENETRATING geometry (claim 5):
//!         loud on some arm, never a silent pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CancelToken, CapEnd, ChecksConfig, ContactClass, DocEdit,
    DocRef, DocumentId, EntityKind, EvalOptions, Evaluation, Frame, MateFrame, MatePrimitive, Node,
    ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, StableName, assemble,
    content_pin, evaluate, run_checks,
};
use fixture::{insert, len, on_frame, step};
use geom_core::Tol;

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

/// The extrude in a one-block part document. A block is three nodes
/// — the sketch frame, the profile drawn on it, then the extrude — so
/// a part-local name is minted by node 2.
const PART_BODY: RecipeNodeId = RecipeNodeId(2);

fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

/// A reference whose inner name answers to nothing of the part —
/// `RecipeNodeId(99)` has no face — so mint refuses `Vanished`.
fn vanished(instance: RecipeNodeId) -> StableName {
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

fn mate_node(
    a: StableName,
    b: StableName,
    class: ContactClass,
    seat: f64,
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

fn stand(label: &str, part: DocRef, seat: f64) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(part));
        doc = next;
        ids.push(id);
    }
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[0], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                ContactClass::Rest,
                seat,
            ),
        },
    );
    (doc, ids, mate.expect("the mate inserts"))
}

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

/// A one-line, cross-tree-comparable rendering of an assemble error.
fn headline(result: &Result<editor_core::Assembly<f64>, AssemblyError>) -> String {
    match result {
        Ok(_) => "Ok".to_string(),
        Err(AssemblyError::Reference {
            mate, side, why, ..
        }) => format!("Reference mate={mate:?} side={side:?} why={why:?}"),
        Err(AssemblyError::NoAtRestRecord { mate, class, .. }) => {
            format!("NoAtRestRecord mate={mate:?} class={class:?}")
        }
        Err(AssemblyError::AtRest { findings }) => {
            format!("AtRest findings={}", findings.len())
        }
        Err(other) => format!("{other:?}").chars().take(80).collect(),
    }
}

/// P1: a stand whose declaration is FALSE (seat 1.5 — would refuse at
/// the declared gate), plus a bad-reference mate, plus a Tangent mate,
/// in that document order. The refusal must be the bad reference —
/// first bad mate in document order — on both trees.
#[test]
fn p1_first_bad_mate_wins_badref_before_tangent() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("m6r2-p1-cube"), Tol::witness());
    let (doc, ids, _) = stand("m6r2-p1-stand", part, 1.5);
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                vanished(ids[0]),
                in_part(ids[1], CapEnd::Bottom),
                ContactClass::Rest,
                1.5,
            ),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[0], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                ContactClass::Tangent,
                1.5,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    println!("P1: {}", headline(&result));
    assert!(matches!(result, Err(AssemblyError::Reference { .. })));
}

/// P2: same document, the two bad mates in the OPPOSITE order. The
/// refusal must be the Tangent's `NoAtRestRecord` on both trees.
#[test]
fn p2_first_bad_mate_wins_tangent_before_badref() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("m6r2-p2-cube"), Tol::witness());
    let (doc, ids, _) = stand("m6r2-p2-stand", part, 1.5);
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[0], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                ContactClass::Tangent,
                1.5,
            ),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                vanished(ids[0]),
                in_part(ids[1], CapEnd::Bottom),
                ContactClass::Rest,
                1.5,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    println!("P2: {}", headline(&result));
    assert!(matches!(result, Err(AssemblyError::NoAtRestRecord { .. })));
}

/// P3: the checks resident over the seam document (×3 stands).
#[test]
fn p3_checks_over_the_seam_document() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("m6r2-p3-cube"), Tol::witness());
    let (inner, _, _) = stand("m6r2-p3-stand", part, 1.0);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p3-row", inner_ref, 3, 4.0);
    let ev = run(&outer, &opts(store));
    let report =
        run_checks(&outer, &ev, &ChecksConfig::default(), Tol::witness()).expect("the checks run");
    println!(
        "P3: findings={} {:?}",
        report.findings.len(),
        report.findings
    );
}

/// P4: the checks resident over the single correctly-mated stand.
#[test]
fn p4_checks_over_a_correctly_mated_document() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("m6r2-p4-cube"), Tol::witness());
    let (doc, _, _) = stand("m6r2-p4-stand", part, 1.0);
    let ev = run(&doc, &opts(store));
    let report =
        run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).expect("the checks run");
    println!(
        "P4: findings={} {:?}",
        report.findings.len(),
        report.findings
    );
}

/// P5: an unmintable (Tangent, and non-touching) mate BEFORE the
/// stand's good Rest mate, in document order. Old `mint` stopped at
/// the first bad mate, so the good later declaration did not suppress
/// the separation finding; total mint does suppress it.
#[test]
fn p5_checks_with_a_bad_mate_before_a_good_one() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("m6r2-p5-cube"), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive("m6r2-p5"), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..3 {
        let (next, id) = insert(doc, Node::instantiate_part(part));
        doc = next;
        ids.push(id);
    }
    // Park the third cube far away, then declare a Tangent against it
    // (unmintable, and not touching, so it contributes no pair).
    let (next, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[2],
            frame: Frame::translation([10.0, 0.0, 0.0]),
        },
    );
    doc = next;
    let (next, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[2], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                ContactClass::Tangent,
                5.0,
            ),
        },
    );
    doc = next;
    // The good Rest mate, AFTER the bad one: seats cube 1 on cube 0.
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[0], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                ContactClass::Rest,
                1.0,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    let report =
        run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).expect("the checks run");
    println!(
        "P5: findings={} {:?}",
        report.findings.len(),
        report.findings
    );
}

/// P6: a carried declaration over PENETRATING geometry (inner seat
/// 0.5): whatever arm fires, the outer document must not pass.
#[test]
fn p6_carried_penetration_is_loud() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("m6r2-p6-cube"), Tol::witness());
    let (inner, _, _) = stand("m6r2-p6-stand", part, 0.5);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p6-row", inner_ref, 1, 4.0);
    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    println!("P6: {}", headline(&result));
    assert!(result.is_err(), "penetrating carried geometry must be loud");
}

/// P7: the outer gate over the ×3-stand seam document, counted by arm.
/// On the merge base this printed `AtRest findings=24 undeclared=24`;
/// on the MATE-6 head it must be green.
#[test]
fn p7_seam_gate_by_arm() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("m6r2-p7-cube"), Tol::witness());
    let (inner, _, _) = stand("m6r2-p7-stand", part, 1.0);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p7-row", inner_ref, 3, 4.0);
    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    match &result {
        Ok(_) => println!("P7: Ok"),
        Err(AssemblyError::AtRest { findings }) => {
            let undeclared = findings
                .iter()
                .filter(|f| format!("{:?}", f.error).contains("UndeclaredContact"))
                .count();
            println!(
                "P7: AtRest findings={} undeclared={}",
                findings.len(),
                undeclared
            );
        }
        Err(other) => println!("P7: {other}"),
    }
}

/// P8: the seam drops the inner document's UNMINTED rows too. An inner
/// stand whose only mate is an unmintable Tangent over a GAP (seat
/// 5.0): the inner document's own `assemble` refuses `NoAtRestRecord`,
/// but instantiated into an outer document the outer gate has nothing
/// to see — no contact, no declaration, no carried refusal — and
/// passes. Identical to main by construction (main carried nothing
/// either); printed here to bound "verification runs once at the
/// outermost gate": inner MINT REFUSALS do not reach the outer gate.
#[test]
fn p8_inner_mint_refusals_stop_at_the_seam() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("m6r2-p8-cube"), Tol::witness());
    let mut inner = ProfileDoc::empty(DocumentId::derive("m6r2-p8-stand"), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(inner, Node::instantiate_part(part));
        inner = next;
        ids.push(id);
    }
    let (inner, _) = step(
        inner,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[0], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                ContactClass::Tangent,
                5.0,
            ),
        },
    );
    let inner_ev = run(&inner, &opts(store.clone()));
    let inner_result = assemble(&inner, &inner_ev, Tol::witness());
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p8-row", inner_ref, 1, 4.0);
    let ev = run(&outer, &opts(store));
    let outer_result = assemble(&outer, &ev, Tol::witness());
    println!(
        "P8: inner={} outer={}",
        headline(&inner_result),
        headline(&outer_result)
    );
}
