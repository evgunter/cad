//! M4 PR 6 review MINOR-3 — the committed GOLDEN fixture.
//!
//! D6.1's round-trip row proves save∘load is a fixpoint, but a
//! fixpoint is BLIND to format drift: rename a field and save/load
//! stay self-consistent while every existing file breaks. This row
//! pins the frozen wire shape to CHECKED-IN BYTES
//! (`tests/golden/golden.cad`): the fixture document must save to
//! exactly those bytes, and the bytes must load. A change to either is
//! a FORMAT CHANGE — deliberate, never in passing: re-bless (run with
//! `M4_PR6_BLESS_GOLDEN=1`), regenerate the rest of the checked-in
//! corpus the same way, and say so in the PR. The format carries no
//! schema version (the persist module docs say why), so the re-bless
//! IS the whole procedure.
//!
//! ε note: the golden snapshot PINS ε = 1e-9 via `SetTolerance` (a
//! committed byte stream cannot record the ambient ε — it varies by
//! CI row). Full `load` therefore ε-reconciles: under an ambient of
//! 1e-9 it succeeds; under any other row it refuses
//! `ToleranceConflict` — which still proves the bytes parsed,
//! validated, and replayed, because that door is the LAST in the load
//! sequence. Both outcomes are asserted exactly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::UnitSym;
use editor_core::{
    Attr, CancelToken, Dimension, Distribution, DocEdit, DocParam, EntityKind, EvalOptions, Expr,
    LoopProgram, MetaValue, Node, NodeResult, ParamName, PersistError, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, Rgba8, RoleSeg, StableName, WitnessDatum, apply,
    evaluate, load, save,
};
use fixture::{ang, desc, len, len2, scl};
use geom_core::Tol;

const GOLDEN: &str = include_str!("golden/golden.cad");
const GOLDEN_PATH: &str = "tests/golden/golden.cad";

/// The golden document: deterministic (no ambient reads — ε pinned by
/// the SetTolerance edit) and shape-covering: params, an arc-bearing
/// profile with a hand-DECLARED line/arc tangency (#101), a
/// fillet-CONSTRUCTED profile (tangent joints by construction), a
/// param-expression slot, witness bytes, appearance attrs + D7
/// metadata (floats, -0.0, bytes, list, nesting).
///
/// #120 history: the original golden hand-declared a COLLINEAR
/// tangency — exactly what #101's same-carrier-is-identity rule
/// refuses — so the frozen exemplar evaluated sick (node 2 Failed,
/// invisible to the byte rows). Regenerated 8b-fix-pass from this
/// HEALTHY document (the corpus's legal line/arc bracket pattern,
/// same `tangent_joints` wire coverage); content change only, format
/// unchanged — both byte generations parse under the same schema-1
/// loader.
fn golden() -> (ProfileDoc, Vec<DocEdit<ProfileProgram>>) {
    let mut doc = ProfileDoc::empty_derived("m4_pr6_golden", Tol::witness());
    let push = |d: &ProfileDoc, e: &DocEdit<ProfileProgram>| {
        apply(d, e, Tol::witness(), &editor_core::RefusingReach)
            .expect("golden edit")
            .doc
    };
    // The node an insert just minted: the document's last.
    let last = |d: &ProfileDoc| *d.order().last().expect("an insert landed");
    doc = push(&doc, &DocEdit::SetTolerance { eps: 1e-9 });
    // v15: `depth` carries a distribution, so the frozen bytes pin the
    // populated `distribution` key rather than only its absence.
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::from_static("depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.75,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::TruncatedNormal {
                    sigma: 0.002,
                    lo: -0.005,
                    hi: 0.004,
                }),
            },
        },
    );
    // A second parameter with NO distribution, so the same bytes also
    // pin the degenerate carry: an unannotated param writes no key.
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::from_static("clearance"),
            value: DocParam::continuous(Dimension::Length, 0.001),
        },
    );
    // Every sketch in this fixture is drawn on the world xy plane, so
    // ONE frame node serves them all — a profile names its
    // plane now, and four copies of the same frame would say four
    // planes where the document has one.
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(fixture::xy_frame()),
        },
    );
    let plane = last(&doc);
    // v4 re-authoring (content-preserving): the quad with one arc
    // segment authors as a chain whose arc step carries its AUTHORED
    // bulge — the same 0.25 the retired form stored on vertex 1.
    let mut d = desc(plane, vec![]);
    d.loops = vec![LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, 0.0])),
        ProgramStep::LineTo(ProgramTarget::Point(len2([2.0, 0.0]))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(len2([2.0, 1.0])),
            b: scl(0.25),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, 1.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])];
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Profile(d)),
        },
    );
    let arc_profile = last(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: arc_profile,
                distance: Expr::param(ParamName::from_static("depth"), Dimension::Length),
            }),
        },
    );
    let bulged = last(&doc);
    // #101 tangency coverage in the FROZEN bytes: a hand-DECLARED
    // line/arc tangency (the #100 bracket: the quarter arc leaving
    // (1.5,1), bulge −(√2−1), is exactly tangent to both neighboring
    // lines; joints 3 and 4 declared BY HAND) and a fillet-CONSTRUCTED
    // loop (joints declared by construction) — the wire's
    // tangent_joints field is pinned by the golden from day one. (#120:
    // this replaced the original COLLINEAR declaration, which the #101
    // same-carrier rule refuses — the old exemplar was sick.)
    // v4: the hand-declared joints author STRUCTURALLY — `.tangent()`
    // before the arc and before the leg out of it (the corpus
    // bracket's own program form; the arc bulge is now the tangent-arc
    // derivation, the W1 ulp class).
    let bracket = LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, 0.0])),
        ProgramStep::LineTo(ProgramTarget::Point(len2([3.0, 0.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([3.0, 1.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([1.5, 1.0]))),
        ProgramStep::Tangent,
        ProgramStep::TangentArcTo(ProgramTarget::Point(len2([1.0, 1.5]))),
        ProgramStep::Tangent,
        // A declared-tangent straight leg RIDES the inherited
        // direction, so it authors as a LENGTH (`line(1.5)` — the
        // (1, 1.5) → (1, 3) run), not a second target.
        ProgramStep::Line(len(1.5)),
        ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, 3.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Profile(ProfileProgram {
                plane,
                loops: vec![bracket],
                ids: Vec::new(),
            })),
        },
    );
    // v4: the constructed fillet authors as the chain fillet form
    // (exact `toward` directors — G1/VQ4).
    let len0 = || len(0.0);
    let fillet_loop = LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, 0.0])),
        ProgramStep::LineTo(ProgramTarget::Point(len2([3.0, 0.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([3.0, 1.0]))),
        ProgramStep::Toward {
            dx: scl(-1.0),
            dy: scl(0.0),
        },
        ProgramStep::Fillet(len(0.5)),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(1.0),
        },
        ProgramStep::FarEndTo(len2([1.0, 3.0])),
        ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, 3.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Profile(ProfileProgram {
                plane,
                loops: vec![fillet_loop],
                ids: Vec::new(),
            })),
        },
    );
    // v16's own wire shape, in the frozen bytes: a `Node::Chamfer`
    // with its `distance` slot and its canonical frozen selection.
    // Without this, the one variant the v16 break EXISTS for would be
    // pinned by no golden, against this fixture's shape-covering
    // charter.
    //
    // It gets its OWN square prism rather than reusing the bulged
    // block, and the reason is the door rather than tidiness: that
    // profile carries an ARC, so its barrel is a cylinder; the
    // chamfer's v1 door is plane-plane, and every closed edge chain on
    // that body runs into the curved lateral and refuses
    // `ChamferArmUnsupported`. A single edge does not work either — the
    // assembly admits only a FULLY-REQUESTED chain set, so one lateral
    // edge terminating at a trivalent corner refuses
    // `UnsupportedRunOut`. A four-sided prism with all twelve edges
    // requested is the smallest thing the door actually accepts, and a
    // golden that froze a refusing node would be the sick-bytes failure
    // #117/#120 named.
    let square = desc(
        plane,
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Profile(square)),
        },
    );
    let square = last(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: square,
                distance: len(0.5),
            }),
        },
    );
    let prism = last(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::chamfer(
                prism,
                len(0.1),
                fixture::prism_edges(&doc, prism, 4),
            )),
        },
    );
    doc = push(
        &doc,
        &DocEdit::ReWitness {
            node: arc_profile,
            witness: WitnessDatum {
                schema: 1,
                bytes: vec![0x00, 0x7f, 0x80, 0xff],
            },
        },
    );
    let body = StableName {
        kind: EntityKind::Body,
        node: bulged,
        path: vec![RoleSeg::OutputBody],
    };
    doc = push(
        &doc,
        &DocEdit::SetAppearance {
            name: body.clone(),
            attr: Attr::Color(Rgba8::opaque(10, 20, 30)),
        },
    );
    let mut m = std::collections::BTreeMap::new();
    m.insert("v".into(), MetaValue::Int(1));
    m.insert("neg_zero".into(), MetaValue::Float(-0.0));
    m.insert("blob".into(), MetaValue::Bytes(vec![0xde, 0xad]));
    m.insert(
        "list".into(),
        MetaValue::list(vec![MetaValue::Null, MetaValue::Bool(true)]).expect("a shallow value"),
    );
    doc = push(
        &doc,
        &DocEdit::SetAppearanceMeta {
            name: body.clone(),
            key: "tool.example/pin".into(),
            value: MetaValue::map(m).expect("a shallow value"),
        },
    );
    // v17: the measurement vocabulary on the wire (E3/E10) — a
    // `Measure` carrying a reference list and a measured expression,
    // and an `Assertion` bounding it. The measured expression is
    // arithmetic over a parameter and a literal rather than a
    // primitive: the golden must evaluate GREEN, and a primitive over
    // this document's only well-known name (a whole BODY) has no
    // closed form. The primitive leaves' wire forms are pinned by
    // round-trip in `m10_2_measure_wire.rs`, where a document with real
    // carriers can be built.
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(
                Node::measure(
                    editor_core::MeasureExpr::sub(
                        editor_core::MeasureExpr::value(Expr::param(
                            ParamName::from_static("depth"),
                            Dimension::Length,
                        )),
                        editor_core::MeasureExpr::value(len(0.25)),
                    )
                    .expect("same-dimension subtraction"),
                    // Read at the extrude that owns the body: the
                    // reference is unindexed by this expression, so it is
                    // carried data the measure never reads.
                    vec![editor_core::SitedRef::new(bulged, body.clone())],
                )
                .expect("every index addresses a reference"),
            ),
        },
    );
    let measure = last(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Assertion {
                // The `Measure` pushed immediately above.
                measure,
                bound: len(0.1),
                dir: editor_core::AssertionDir::AtLeast,
            }),
        },
    );
    // BOTH tube kinds, and both window spellings between them. Two
    // kinds arrived in one vocabulary change, so a golden pinning one
    // of them would leave the other's wire shape frozen by nothing —
    // and the window variant is recipe payload that decides which
    // slots the node has, so `Full` and `Arc` are two shapes, not one
    // with different numbers. (There is no schema version to pin any
    // of this against: #1553 retired the version machinery, and these
    // bytes are the whole freeze.)
    //
    // The solid kind takes the full ring and the hollow kind the arc,
    // rather than the reverse, because that pairing puts the wall
    // slot beside the two window-angle slots — the widest slot list
    // either kind can carry — in the same node.
    //
    // R > r holds for both (the ring-torus convention), and the hollow
    // one's wall clears its own bore.
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Datum(editor_core::Datum::Axis {
                origin: [len0(), len0(), len0()],
                direction: [scl(0.0), scl(0.0), scl(1.0)],
            })),
        },
    );
    let spine = last(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Tube {
                spine,
                u_ref: [scl(1.0), scl(0.0), scl(0.0)],
                major_radius: len(2.0),
                window: editor_core::TubeWindow::Full,
                minor_radius: len(0.5),
            }),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::HollowTube {
                spine,
                u_ref: [scl(1.0), scl(0.0), scl(0.0)],
                major_radius: len(2.0),
                window: editor_core::TubeWindow::Arc {
                    t0: ang(0.0),
                    t1: ang(1.5),
                },
                minor_radius: len(0.5),
                wall: len(0.125),
            }),
        },
    );
    // The shell's wire shape: an `open` list of face names in
    // DESIGNATION ORDER (the first named face carries the rim), here
    // one name — a box's end cap. The box is its own three nodes (a
    // square on the sketch frame, its extrude, the shell) rather than
    // a shell of the bulged block: the shell verb refuses the bulged block's
    // cylindrical wall at its inward offset (`ReanchorOffCarrier`), a
    // kernel scope fact this fixture is not the place to argue.
    // The wall clears every dimension of
    // the box by an order of magnitude, so the golden evaluates green.
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Profile(desc(
                plane,
                vec![vec![(3.0, 0.0), (4.0, 0.0), (4.0, 1.0), (3.0, 1.0)]],
            ))),
        },
    );
    let box_profile = last(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: box_profile,
                distance: len(0.5),
            }),
        },
    );
    let block = last(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::shell(
                block,
                len(0.0625),
                vec![StableName {
                    kind: EntityKind::Face,
                    node: block,
                    path: vec![RoleSeg::Cap(editor_core::CapEnd::End)],
                }],
            )),
        },
    );
    // The committed EDIT LOG half: one trailing continuous edit —
    // authored through the TEXT door with a display unit, so the v4
    // wire's per-literal `unit` field is pinned in the FROZEN bytes
    // (§4g: value canonical meters, `"unit": "mm"` on the wire).
    let edits = vec![DocEdit::SetParam {
        node: bulged,
        slot: editor_core::SlotId::Distance,
        expr: editor_core::parse_expr("500 mm", &std::collections::BTreeMap::new())
            .expect("golden unit literal"),
    }];
    (doc, edits)
}

#[test]
fn golden_bytes_are_frozen() {
    let (doc, edits) = golden();
    let text = save(&doc, &edits.to_vec(), Tol::witness()).expect("golden saves");
    if std::env::var("M4_PR6_BLESS_GOLDEN").is_ok() {
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_PATH),
            &text,
        )
        .expect("bless writes");
        panic!(
            "golden re-blessed — commit the file WITH the format change it records, then rerun without the env var"
        );
    }
    assert_eq!(
        text, GOLDEN,
        "wire bytes drifted from the committed golden — this is a FORMAT \
         CHANGE: re-bless deliberately and regenerate the corpus with it, never in passing"
    );
}

#[test]
fn golden_bytes_load() {
    let ambient = geom_core::Tol::witness().get().eps;
    match load(GOLDEN, Tol::witness()) {
        Ok(loaded) => {
            // Only reachable when the process ε IS the golden's 1e-9.
            assert_eq!(ambient.to_bits(), 1e-9f64.to_bits());
            let (doc, edits) = golden();
            assert!(loaded.snapshot.bit_eq(&doc), "golden snapshot drifted");
            assert_eq!(loaded.edits, edits.to_vec(), "golden edit log drifted");
        }
        Err(PersistError::ToleranceConflict { process, document }) => {
            // The ε door is the LAST load door, so this outcome still
            // proves the golden bytes parse, validate, and replay.
            assert_eq!(document.to_bits(), 1e-9f64.to_bits());
            assert_eq!(process.to_bits(), ambient.to_bits());
            assert_ne!(ambient.to_bits(), 1e-9f64.to_bits());
        }
        Err(other) => panic!("golden v2 file failed to load: {other:?}"),
    }
}

/// #117 follow-through (the green gate; #120): byte identity is as
/// blind to evaluation health as fingerprint identity — the ORIGINAL
/// golden froze a document whose declared collinear tangency #101's
/// same-carrier rule refuses (node 2 Failed) while both byte rows
/// stayed green. The exemplar is now healthy, and this gate keeps the
/// class structurally dead: corpus docs assert green in their rows,
/// the persistence fingerprints assert green (#117), and the golden
/// asserts green HERE. Only meaningful at the golden's own pinned ε
/// (every other matrix row refuses `ToleranceConflict` at the load
/// door, asserted above), so other rows skip.
#[test]
fn golden_document_evaluates_green_at_its_pinned_eps() {
    if geom_core::Tol::witness().get().eps.to_bits() != 1e-9f64.to_bits() {
        return;
    }
    let (mut doc, edits) = golden();
    for e in &edits {
        doc = apply(&doc, e, Tol::witness(), &editor_core::RefusingReach)
            .expect("golden edit")
            .doc;
    }
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let bad: Vec<String> = ev
        .nodes
        .iter()
        .filter_map(|(id, r)| match r {
            NodeResult::Ok(_) => None,
            NodeResult::Failed(e) => Some(format!("{id:?} FAILED: {e:?}")),
            NodeResult::Poisoned { through } => {
                Some(format!("{id:?} poisoned through {through:?}"))
            }
        })
        .collect();
    assert!(
        bad.is_empty(),
        "the golden document must evaluate green (#117/#120 — a sick \
         golden freezes sick bytes):\n{}",
        bad.join("\n")
    );
}
