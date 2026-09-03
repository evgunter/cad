//! Corpus document **declared_tangency** — the #101 discipline as a
//! recipe: a profile whose tangency is DECLARED by construction (the
//! `ProfileLoop` fillet constructor, which records the two tangent
//! joints it creates) alongside one whose tangency is declared BY
//! HAND (the #100 bracket: a line meeting a quarter arc exactly
//! tangentially at `(1.5,1)` and `(1,1.5)` — a definite-Zero joint,
//! which #101 refuses `UndeclaredTangency` until the recipe states
//! it).
//!
//! Note the two REFUSAL doors this document sits between: leaving the
//! bracket's joints undeclared refuses `UndeclaredTangency`, and
//! declaring a same-carrier continuation (two collinear straights)
//! refuses `TangencyContradicted { same_carrier: true }` — declaring
//! identity is not declaring tangency. Both are pinned in
//! `profile/tests/declared_tangency.rs`; the corpus carries only the
//! legal middle.
//!
//! Vocabulary: Profile (arc-bearing, fillet-constructed and
//! hand-declared), Extrude, `InsertNode`, `SetParam`.
//!
//! No exact mass pin: both bodies carry a quarter arc, so their
//! volumes involve `π` and are not dyadic (the bracket's cross
//! section is `5.25 − π/16`). They are pinned on tier-1 + closed
//! validity, and on the corpus-wide "every node evaluates green"
//! obligation — which is what makes this document a #101 regression
//! net at every ε row.
//!
//! D2 bump: the hand-declared bracket's extrude `Distance`.

use editor_core::{
    Dimension, DocEdit, Expr, LoopProgram, Node, ProfileProgram, ProgramStep, ProgramTarget, SlotId,
};
use profile::SketchPlane;

use super::{CorpusDoc, Recorder};
use crate::fixture::len;

/// The declared-tangency corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    // (a) Tangency BY CONSTRUCTION: the chain fillet form (v4 — the
    // G1 exact-director spelling: `toward` components are exact, so
    // the ray carries no sin_cos dirt; the trims are the algebra's own
    // closed forms).
    let pt = |x: f64, y: f64| {
        [
            Expr::literal(x, Dimension::Length).unwrap(),
            Expr::literal(y, Dimension::Length).unwrap(),
        ]
    };
    let scl2 = |v: f64| Expr::literal(v, Dimension::Scalar).unwrap();
    let filleted = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 1.0))),
        ProgramStep::Toward {
            dx: scl2(-1.0),
            dy: scl2(0.0),
        },
        ProgramStep::Fillet(Expr::literal(0.5, Dimension::Length).unwrap()),
        ProgramStep::Toward {
            dx: scl2(0.0),
            dy: scl2(1.0),
        },
        ProgramStep::FarEndTo(pt(1.0, 3.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 3.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let fillet_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![filleted],
    }));
    let fillet_body = r.insert(Node::Extrude {
        profile: fillet_p,
        distance: len(0.5),
    });
    let _ = fillet_body;

    // (b) Tangency BY HAND: the #100 bracket. The quarter arc leaving
    // (1.5,1) is exactly tangent to the line arriving there and to
    // the line leaving (1,1.5) — joints 3 and 4, both declared.
    // Bulge of a 90° arc is tan(90°/4) = √2 − 1.
    let bracket = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.5, 1.0))),
        // The declared-tangent quarter arc and the declared-tangent
        // straight leg out of it: `.tangent()` DECLARES joints 3 and 4
        // structurally (the hand-set `tangent_joints = vec![3, 4]` of
        // the retired form). The arc's bulge is now the tangent-arc
        // derivation (tan(−π/8)) rather than the hand literal
        // −(√2 − 1) — the W1 ulp class, a numbered deviation.
        ProgramStep::Tangent,
        ProgramStep::TangentArcTo(ProgramTarget::Point(pt(1.0, 1.5))),
        ProgramStep::Tangent,
        // The declared-tangent straight leg rides the inherited
        // direction: authored as its LENGTH (the (1,1.5) → (1,3) run).
        ProgramStep::Line(Expr::literal(1.5, Dimension::Length).unwrap()),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 3.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let tangent_p = r.insert(Node::Profile(ProfileProgram {
        // A parallel plane, so the two bodies never interact.
        plane: SketchPlane::from_frame(
            geom_core::Point3::new(0.0, 0.0, 4.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
        ),
        loops: vec![bracket],
    }));
    let tangent_body = r.insert(Node::Extrude {
        profile: tangent_p,
        distance: len(0.25),
    });

    CorpusDoc {
        name: "declared_tangency",
        about: "#101: fillet-constructed joints + a hand-declared line/arc tangency",
        edits: r.edits,
        doc: r.doc,
        result: Some(tangent_body),
        pin: None, // arcs: not dyadic (module docs)
        bump: DocEdit::SetParam {
            node: tangent_body,
            slot: SlotId::Distance,
            expr: Expr::literal(0.5, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: tangent_body,
    }
}
