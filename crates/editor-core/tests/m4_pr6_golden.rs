//! M4 PR 6 review MINOR-3 — the committed GOLDEN v1 fixture.
//!
//! D6.1's round-trip row proves save∘load is a fixpoint, but a
//! fixpoint is BLIND to format drift: rename a field and save/load
//! stay self-consistent while every existing v1 file breaks. This row
//! pins the frozen wire shape to CHECKED-IN BYTES
//! (`tests/golden/v19_golden.cad`): the fixture document must save to
//! exactly those bytes, and the bytes must load. Any change to either
//! is a format change and demands a ratified schema bump + migration
//! step — re-bless ONLY then (run with `M4_PR6_BLESS_GOLDEN=1` to
//! regenerate, and say so loudly in the PR).
//!
//! ε note: the golden snapshot PINS ε = 1e-9 via `SetTolerance` (a
//! committed byte stream cannot record the ambient ε — it varies by
//! CI row). Full `load` therefore ε-reconciles: under an ambient of
//! 1e-9 it succeeds; under any other row it refuses
//! `ToleranceConflict` — which still proves the bytes parsed,
//! validated, and replayed, because that door is the LAST in the load
//! sequence. Both outcomes are asserted exactly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Attr, CancelToken, Dimension, Distribution, DocEdit, DocParam, EntityKind, EvalOptions, Expr,
    LoopProgram, MetaValue, Node, NodeResult, ParamName, PersistError, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, Rgba8, RoleSeg, StableName, WitnessDatum, apply,
    evaluate, load, save,
};
use fixture::desc;
use geom_core::Tol;

const GOLDEN: &str = include_str!("golden/v19_golden.cad");
const GOLDEN_PATH: &str = "tests/golden/v19_golden.cad";

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
        apply(d, e, Tol::witness()).expect("golden edit").doc
    };
    let lpt = |x: f64, y: f64| {
        [
            Expr::literal(x, Dimension::Length).expect("finite"),
            Expr::literal(y, Dimension::Length).expect("finite"),
        ]
    };
    doc = push(&doc, &DocEdit::SetTolerance { eps: 1e-9 });
    // v15: `depth` carries a distribution, so the frozen bytes pin the
    // populated `distribution` key rather than only its absence.
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.75,
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
            name: ParamName::new("clearance"),
            value: DocParam::continuous(Dimension::Length, 0.001),
        },
    );
    // v4 re-authoring (content-preserving): the quad with one arc
    // segment authors as a chain whose arc step carries its AUTHORED
    // bulge — the same 0.25 the retired form stored on vertex 1.
    let mut d = desc([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![]);
    d.loops = vec![LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(2.0, 0.0))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(lpt(2.0, 1.0)),
            b: Expr::literal(0.25, Dimension::Scalar).expect("finite"),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(0.0, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])];
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(d),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: editor_core::RecipeNodeId(0),
                distance: Expr::param(ParamName::new("depth"), Dimension::Length),
            },
        },
    );
    // #101 tangency coverage in the FROZEN bytes: a hand-DECLARED
    // line/arc tangency (node 2 — the #100 bracket: the quarter arc
    // leaving (1.5,1), bulge −(√2−1), is exactly tangent to both
    // neighboring lines; joints 3 and 4 declared BY HAND) and a
    // fillet-CONSTRUCTED loop (node 3, joints declared by
    // construction) — the wire's tangent_joints field is pinned by
    // the golden from day one. (#120: this replaced the original
    // COLLINEAR declaration, which the #101 same-carrier rule
    // refuses — the old exemplar was sick.)
    // v4: the hand-declared joints author STRUCTURALLY — `.tangent()`
    // before the arc and before the leg out of it (the corpus
    // bracket's own program form; the arc bulge is now the tangent-arc
    // derivation, the W1 ulp class).
    let bracket = LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(3.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(3.0, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(1.5, 1.0))),
        ProgramStep::Tangent,
        ProgramStep::TangentArcTo(ProgramTarget::Point(lpt(1.0, 1.5))),
        ProgramStep::Tangent,
        // A declared-tangent straight leg RIDES the inherited
        // direction, so it authors as a LENGTH (`line(1.5)` — the
        // (1, 1.5) → (1, 3) run), not a second target.
        ProgramStep::Line(Expr::literal(1.5, Dimension::Length).expect("finite")),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(0.0, 3.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: profile::SketchPlane::xy(),
                loops: vec![bracket],
            }),
        },
    );
    // v4: the constructed fillet authors as the chain fillet form
    // (exact `toward` directors — G1/VQ4).
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite");
    let fillet_loop = LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(3.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(3.0, 1.0))),
        ProgramStep::Toward {
            dx: scl(-1.0),
            dy: scl(0.0),
        },
        ProgramStep::Fillet(Expr::literal(0.5, Dimension::Length).expect("finite")),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(1.0),
        },
        ProgramStep::FarEndTo(lpt(1.0, 3.0)),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(0.0, 3.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: profile::SketchPlane::xy(),
                loops: vec![fillet_loop],
            }),
        },
    );
    // v16's own wire shape, in the frozen bytes: a `Node::Chamfer`
    // with its `distance` slot and its canonical frozen selection.
    // Without this, the one variant the v16 break EXISTS for would be
    // pinned by no golden, against this fixture's shape-covering
    // charter.
    //
    // It gets its OWN square prism (nodes 4 and 5) rather than reusing
    // node 1, and the reason is the door rather than tidiness: node 1's
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
    //
    // Appended, so every existing node id — and every name the
    // appearance rows above address — is untouched.
    let square = desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(square),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: editor_core::RecipeNodeId(4),
                distance: Expr::literal(0.5, Dimension::Length).expect("finite"),
            },
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::chamfer(
                editor_core::RecipeNodeId(5),
                Expr::literal(0.1, Dimension::Length).expect("finite"),
                fixture::prism_edges(editor_core::RecipeNodeId(5), 4),
            ),
        },
    );
    doc = push(
        &doc,
        &DocEdit::ReWitness {
            node: editor_core::RecipeNodeId(0),
            witness: WitnessDatum {
                schema: 1,
                bytes: vec![0x00, 0x7f, 0x80, 0xff],
            },
        },
    );
    let body = StableName {
        kind: EntityKind::Body,
        node: editor_core::RecipeNodeId(1),
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
        MetaValue::List(vec![MetaValue::Null, MetaValue::Bool(true)]),
    );
    doc = push(
        &doc,
        &DocEdit::SetAppearanceMeta {
            name: body.clone(),
            key: "tool.example/pin".into(),
            value: MetaValue::Map(m),
        },
    );
    // v17: the measurement vocabulary on the wire (E3/E10) — a
    // `Measure` carrying a reference list and a measured expression,
    // and an `Assertion` bounding it. The measured expression is
    // arithmetic over a parameter and a literal rather than a
    // primitive: the golden must evaluate GREEN, and a primitive over
    // this document's only well-known name (a whole BODY) has no
    // closed form. The primitive leaves' wire forms are pinned by
    // round-trip in `m10_2_schema_v17.rs`, where a document with real
    // carriers can be built.
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                editor_core::MeasureExpr::sub(
                    editor_core::MeasureExpr::value(Expr::param(
                        ParamName::new("depth"),
                        Dimension::Length,
                    )),
                    editor_core::MeasureExpr::value(
                        Expr::literal(0.25, Dimension::Length).expect("finite"),
                    ),
                )
                .expect("same-dimension subtraction"),
                // Read at node 1, the extrude that owns the body: the
                // reference is unindexed by this expression, so it is
                // carried data the measure never reads.
                vec![editor_core::MeasureRef::new(
                    editor_core::RecipeNodeId(1),
                    body.clone(),
                )],
            )
            .expect("every index addresses a reference"),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                // The `Measure` pushed immediately above. Ids in this
                // document are positional, so a node inserted EARLIER
                // shifts this one — the insert door catches that
                // typed (`AssertionTarget`) rather than letting a
                // golden freeze an assertion over the wrong node.
                measure: editor_core::RecipeNodeId(7),
                bound: Expr::literal(0.1, Dimension::Length).expect("finite"),
                dir: editor_core::AssertionDir::AtLeast,
            },
        },
    );
    // The committed EDIT LOG half: one trailing continuous edit —
    // authored through the TEXT door with a display unit, so the v4
    // wire's per-literal `unit` field is pinned in the FROZEN bytes
    // (§4g: value canonical meters, `"unit": "mm"` on the wire).
    let edits = vec![DocEdit::SetParam {
        node: editor_core::RecipeNodeId(1),
        slot: editor_core::SlotId::Distance,
        expr: editor_core::parse_expr("500 mm", &std::collections::BTreeMap::new())
            .expect("golden unit literal"),
    }];
    (doc, edits)
}

#[test]
fn golden_bytes_are_frozen() {
    let (doc, edits) = golden();
    let text = save(&doc, &edits, Tol::witness()).expect("golden saves");
    if std::env::var("M4_PR6_BLESS_GOLDEN").is_ok() {
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_PATH),
            &text,
        )
        .expect("bless writes");
        panic!(
            "golden re-blessed — commit the file WITH its ratified schema change, then rerun without the env var"
        );
    }
    assert_eq!(
        text, GOLDEN,
        "wire bytes drifted from the committed golden — this is a FORMAT \
         CHANGE: it needs a ratified schema bump + migration step, never a re-bless in passing"
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
            assert_eq!(loaded.edits, edits, "golden edit log drifted");
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
        doc = apply(&doc, e, Tol::witness()).expect("golden edit").doc;
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
