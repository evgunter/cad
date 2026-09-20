//! MATE-6 R2 probes — the refusal-precedence and seam rows that
//! review left behind.
//!
//! Each prints a `P<n>:`-tagged line; the rows were written to compile
//! on the MATE-6 head AND on its merge base, so the review could diff
//! the tagged lines. **That property is spent** — the branch merged —
//! and four of the eight rows still assert nothing, so their printed
//! answers are unguarded;
//! `work/tint/mate6r1-shared-has-eleven-tests-and-no-assertions.md`
//! owns that. What the rows are FOR now is what each one says below.
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

use crate::fixture;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CapEnd, ChecksConfig, ContactClass, DocEdit, DocRef,
    DocumentId, EntityKind, Frame, MateFrame, MatePrimitive, Node, ProfileDoc, RecipeNodeId,
    RoleSeg, StableName, assemble, run_checks,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame, run, step};
use geom_core::Tol;

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

/// A reference whose inner name answers to nothing of the part —
/// `RecipeNodeId(99)` has no face — so mint refuses `Vanished`.
fn vanished(instance: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(99),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            }
            .into(),
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
        a: crate::fixture::head(a),
        b: crate::fixture::head(b),
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
                in_part(ids[0], CapEnd::End),
                in_part(ids[1], CapEnd::Start),
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
        Err(AssemblyError::Mint { refusals }) => refusals
            .iter()
            .map(|r| match r {
                editor_core::MintRefusal::Reference {
                    mate, side, why, ..
                } => format!("Reference mate={mate:?} side={side:?} why={why:?}"),
                editor_core::MintRefusal::NoAtRestRecord { mate, class, .. } => {
                    format!("NoAtRestRecord mate={mate:?} class={class:?}")
                }
            })
            .collect::<Vec<_>>()
            .join(" + "),
        Err(AssemblyError::AtRest { findings }) => {
            format!("AtRest findings={}", findings.len())
        }
        Err(other) => format!("{other:?}").chars().take(80).collect(),
    }
}

/// P1: a stand whose declaration is FALSE (seat 1.5 — would refuse at
/// the declared gate), plus a bad-reference mate, plus a Tangent mate,
/// in that document order. Both bad mates are refused, and the bad
/// reference — first in document order — heads the list, on both
/// trees.
#[test]
fn p1_both_bad_mates_refuse_badref_heading_the_list() {
    let mut store = PartStore::default();
    let part = store.insert(cube_part("m6r2-p1-cube"), Tol::witness());
    let (doc, ids, _) = stand("m6r2-p1-stand", part, 1.5);
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                vanished(ids[0]),
                in_part(ids[1], CapEnd::Start),
                ContactClass::Rest,
                1.5,
            ),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[0], CapEnd::End),
                in_part(ids[1], CapEnd::Start),
                ContactClass::Tangent,
                1.5,
            ),
        },
    );
    let ev = run(&doc, &with_resolver(store));
    let result = assemble(&doc, &ev, Tol::witness());
    println!("P1: {}", headline(&result));
    assert!(matches!(
        &result,
        Err(AssemblyError::Mint { refusals })
            if matches!(
                refusals.as_slice(),
                [
                    editor_core::MintRefusal::Reference { .. },
                    editor_core::MintRefusal::NoAtRestRecord { .. },
                ]
            )
    ));
}

/// P2: same document, the two bad mates in the OPPOSITE order. The
/// same two refusals, with the Tangent's `NoAtRestRecord` at the head,
/// on both trees — the list is the DOCUMENT's order, not the walk's.
#[test]
fn p2_both_bad_mates_refuse_tangent_heading_the_list() {
    let mut store = PartStore::default();
    let part = store.insert(cube_part("m6r2-p2-cube"), Tol::witness());
    let (doc, ids, _) = stand("m6r2-p2-stand", part, 1.5);
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[0], CapEnd::End),
                in_part(ids[1], CapEnd::Start),
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
                in_part(ids[1], CapEnd::Start),
                ContactClass::Rest,
                1.5,
            ),
        },
    );
    let ev = run(&doc, &with_resolver(store));
    let result = assemble(&doc, &ev, Tol::witness());
    println!("P2: {}", headline(&result));
    assert!(matches!(
        &result,
        Err(AssemblyError::Mint { refusals })
            if matches!(
                refusals.as_slice(),
                [
                    editor_core::MintRefusal::NoAtRestRecord { .. },
                    editor_core::MintRefusal::Reference { .. },
                ]
            )
    ));
}

/// P3: the checks resident over the seam document (×3 stands).
#[test]
fn p3_checks_over_the_seam_document() {
    let mut store = PartStore::default();
    let part = store.insert(cube_part("m6r2-p3-cube"), Tol::witness());
    let (inner, _, _) = stand("m6r2-p3-stand", part, 1.0);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p3-row", inner_ref, 3, 4.0);
    let ev = run(&outer, &with_resolver(store));
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
    let mut store = PartStore::default();
    let part = store.insert(cube_part("m6r2-p4-cube"), Tol::witness());
    let (doc, _, _) = stand("m6r2-p4-stand", part, 1.0);
    let ev = run(&doc, &with_resolver(store));
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
    let mut store = PartStore::default();
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
                in_part(ids[2], CapEnd::End),
                in_part(ids[1], CapEnd::Start),
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
                in_part(ids[0], CapEnd::End),
                in_part(ids[1], CapEnd::Start),
                ContactClass::Rest,
                1.0,
            ),
        },
    );
    let ev = run(&doc, &with_resolver(store));
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
    let mut store = PartStore::default();
    let part = store.insert(cube_part("m6r2-p6-cube"), Tol::witness());
    let (inner, _, _) = stand("m6r2-p6-stand", part, 0.5);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p6-row", inner_ref, 1, 4.0);
    let ev = run(&outer, &with_resolver(store));
    let result = assemble(&outer, &ev, Tol::witness());
    println!("P6: {}", headline(&result));
    assert!(result.is_err(), "penetrating carried geometry must be loud");
}

/// P7: the outer gate over the ×3-stand seam document, counted by arm.
/// On the merge base this printed `AtRest findings=24 undeclared=24`;
/// on the MATE-6 head it must be green.
#[test]
fn p7_seam_gate_by_arm() {
    let mut store = PartStore::default();
    let part = store.insert(cube_part("m6r2-p7-cube"), Tol::witness());
    let (inner, _, _) = stand("m6r2-p7-stand", part, 1.0);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p7-row", inner_ref, 3, 4.0);
    let ev = run(&outer, &with_resolver(store));
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

/// P8: an inner document's UNMINTED rows cross the seam. An inner
/// stand whose only mate is an unmintable Tangent over a GAP (seat
/// 5.0) refuses its own `assemble` with `NoAtRestRecord`, and
/// instantiated into an outer document that refusal reaches the outer
/// gate as `CarriedMintRefusal`: an outer assembly is not at rest over
/// a part whose contact nothing verified. "Verification runs once at
/// the outermost gate" bounds where the KERNEL is asked, not which
/// documents' mint health the gate reads.
#[test]
fn p8_inner_mint_refusals_reach_the_outer_gate() {
    let mut store = PartStore::default();
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
                in_part(ids[0], CapEnd::End),
                in_part(ids[1], CapEnd::Start),
                ContactClass::Tangent,
                5.0,
            ),
        },
    );
    let inner_ev = run(&inner, &with_resolver(store.clone()));
    let inner_result = assemble(&inner, &inner_ev, Tol::witness());
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p8-row", inner_ref, 1, 4.0);
    let ev = run(&outer, &with_resolver(store));
    let outer_result = assemble(&outer, &ev, Tol::witness());
    println!(
        "P8: inner={} outer={}",
        headline(&inner_result),
        headline(&outer_result)
    );
    assert!(matches!(
        outer_result,
        Err(AssemblyError::CarriedMintRefusal { .. })
    ));
}
