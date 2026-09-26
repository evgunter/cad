//! **The persisted ARRANGEMENT of the profile program, frozen as
//! bytes.**
//!
//! The corpus is a program reaching every `ProgramStep` variant, every
//! `ProgramArcData` mode, both sides, both windings, every
//! `ProgramTarget` form, all three `LoopProgram` forms and every
//! expression shape; its serialization is checked in and compared byte
//! for byte. It began as a review probe (lane `wire-rv`, PR #2738),
//! which is why its rows carry that prefix, and its corpus is
//! unchanged from the one the review diffed across the two trees.
//!
//! It reaches the wire through the public API only, so it says what a
//! caller's document would say and nothing about a private path.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Dimension, Expr, LoopProgram, ParamName, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget,
};

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).unwrap()
}
fn sca(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}
fn pt(x: f64, y: f64) -> [Expr; 2] {
    [len(x), len(y)]
}
fn point(x: f64, y: f64) -> ProgramTarget {
    ProgramTarget::Point(pt(x, y))
}

fn every_spec() -> Vec<ProgramArcData> {
    let mut out = Vec::new();
    for side in [profile::ArcSide::Left, profile::ArcSide::Right] {
        out.push(ProgramArcData::Radius { r: len(2.0), side });
        out.push(ProgramArcData::Sweep {
            r: len(1.5),
            side,
            angle: ang(0.6),
        });
        out.push(ProgramArcData::ArcLen {
            r: len(1.75),
            side,
            len: len(0.9),
        });
    }
    for target in [
        ProgramTarget::Start,
        ProgramTarget::StartArriving,
        point(2.0, 1.0),
    ] {
        out.push(ProgramArcData::Bulge {
            target: target.clone(),
            b: sca(0.3),
        });
        out.push(ProgramArcData::Via {
            q: pt(4.5, 0.5),
            target: target.clone(),
        });
        for winding in [profile::ArcSweep::Ccw, profile::ArcSweep::Cw] {
            out.push(ProgramArcData::Center {
                c: pt(6.0, 1.0),
                winding,
                target: target.clone(),
            });
        }
    }
    out
}

fn steps() -> Vec<ProgramStep> {
    // The compound expressions ride the four steps whose argument
    // dimensions they match. Every other slot stays a bare literal:
    // the expression wire is one tree shape wherever it appears, so
    // covering it once per dimension covers it.
    let e = exprs();
    let mut steps = vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Angle(ang(0.25)),
        ProgramStep::Toward {
            dx: sca(1.0),
            dy: sca(0.5),
        },
        ProgramStep::Tangent,
        ProgramStep::Cusp,
        ProgramStep::Turn(ang(0.1)),
        ProgramStep::Line(len(1.0)),
        ProgramStep::Fillet(len(0.2)),
        ProgramStep::FarEndTo(pt(7.0, 2.0)),
        ProgramStep::CloseTo,
        ProgramStep::Line(e.length),
        ProgramStep::Angle(e.angle),
        ProgramStep::Toward {
            dx: e.scalar,
            dy: e.counted,
        },
        ProgramStep::Fillet(e.millimetres),
    ];
    for target in [
        ProgramTarget::Start,
        ProgramTarget::StartArriving,
        point(3.0, 0.0),
    ] {
        steps.push(ProgramStep::LineTo(target.clone()));
        steps.push(ProgramStep::ContinueTo(target.clone()));
        steps.push(ProgramStep::TangentArcTo(target.clone()));
    }
    for spec in every_spec() {
        steps.push(ProgramStep::ArcTo(spec.clone()));
        steps.push(ProgramStep::FilletArc {
            radius: len(0.3),
            spec: spec.clone(),
        });
        steps.push(ProgramStep::ArcFillet {
            spec: spec.clone(),
            radius: len(0.4),
        });
        steps.push(ProgramStep::ArcFilletArc {
            spec: spec.clone(),
            radius: len(0.5),
            spec2: spec.clone(),
        });
    }
    steps
}

fn program() -> ProfileProgram {
    ProfileProgram {
        plane: editor_core::RecipeNodeId(0),
        loops: vec![
            LoopProgram::Chain(steps()),
            LoopProgram::circle(1.0, 1.0, 0.5).unwrap(),
            LoopProgram::circle_split(2.0, 2.0, 0.75, 5, 0.2).unwrap(),
        ],
        ids: Vec::new(),
    }
}

/// Every [`crate::Dimension`]-legal expression shape, spread over the
/// slots that take each dimension.
///
/// The step corpus above is all bare literals, and a program of bare
/// literals pins two of the expression wire's fifteen variants. The
/// `Expr` tree is on this wire too and is persisted by the same module,
/// so a reordered `Literal` record or a renamed operator is the same
/// class of format change as a renamed verb — and unpinned is unpinned
/// whichever type it lives on.
///
/// Built through the public dimension-checking constructors only: each
/// is placed where its dimension belongs, so the tree that reaches the
/// wire is one the authoring API would actually produce.
struct Exprs {
    /// Length: `Add`, `Sub`, `Neg`, `Mul`, `Div`, `Min`, `Max`, `Param`.
    length: Expr,
    /// Angle: `Atan2` over two lengths.
    angle: Expr,
    /// Scalar: `Sin`, `Cos`, `Tan`.
    scalar: Expr,
    /// Scalar: `CountToScalar` over `Count` — the exact-integer leaf and
    /// its one promotion, which no other slot on this wire reaches.
    counted: Expr,
    /// Length, authored in millimetres: the `unit` field carrying a
    /// symbol other than the canonical one.
    millimetres: Expr,
}

fn exprs() -> Exprs {
    let length = Expr::max(
        Expr::min(
            Expr::add(
                Expr::sub(Expr::neg(len(3.0)), len(0.5)).unwrap(),
                Expr::mul(len(2.0), sca(1.5)).unwrap(),
            )
            .unwrap(),
            Expr::div(len(8.0), sca(4.0)).unwrap(),
        )
        .unwrap(),
        Expr::param(ParamName::new("width"), Dimension::Length),
    )
    .unwrap();
    let angle = Expr::atan2(len(1.0), len(2.0)).unwrap();
    let scalar = Expr::mul(
        Expr::sin(ang(0.3)).unwrap(),
        Expr::mul(Expr::cos(ang(0.4)).unwrap(), Expr::tan(ang(0.5)).unwrap()).unwrap(),
    )
    .unwrap();
    let counted = Expr::count_to_scalar(Expr::count(7)).unwrap();
    let mm = quantity::unit_by_symbol("mm").expect("mm is a table row");
    let millimetres = Expr::literal_with_unit(0.012, Dimension::Length, mm).unwrap();
    Exprs {
        length,
        angle,
        scalar,
        counted,
        millimetres,
    }
}

/// Every `WireExpr` variant the module declares, so the row below can
/// say what it covers instead of a reader counting arms by eye.
const EXPRESSION_VARIANTS: &[&str] = &[
    "Literal",
    "Count",
    "Param",
    "Add",
    "Sub",
    "Neg",
    "Mul",
    "Div",
    "Sin",
    "Cos",
    "Tan",
    "Atan2",
    "Min",
    "Max",
    "CountToScalar",
];

/// The committed bytes, relative to the crate manifest.
const FILE: &str = "tests/corpus/wire_rv_bytes.json";

/// **The frozen bytes of a program reaching every vocabulary member.**
///
/// `switch_program_vocabulary`'s `PERSISTED_SPELLING` pins the persisted
/// vocabulary as a SET of tokens, which is what makes it cheap to read
/// and is also its blind spot: a swapped `Ccw`/`Cw`, a `spec`/`spec2`
/// exchanged between two fused verbs, or a reordered `Literal` record
/// all leave the token set identical. This row pins the BYTES, so each
/// of those reds it.
///
/// The two are not redundant. The set pin says which words the format
/// uses and localises a rename to the word; this one says where each
/// word goes and localises nothing. A reader chasing a red starts at
/// the set pin if both fired and here if only this one did — "the
/// spelling is unchanged and the arrangement moved" is the whole of
/// what that difference means.
///
/// A red is never repaired by blessing. It says a document the previous
/// build saved no longer reads the same: decide whether the new
/// arrangement is right, THEN regenerate the checked-in corpus
/// (`lib_dietool_crossing`'s header, `corpus/die_composed_tour.rs`'s)
/// and this file with it.
#[test]
fn wire_rv_the_bytes_of_every_variant_are_pinned() {
    let text = serde_json::to_string_pretty(&program()).expect("serializes");
    let back: ProfileProgram = serde_json::from_str(&text).expect("deserializes");
    assert_eq!(back, program(), "round trip");

    // The subject is asserted rich before the comparison: bytes that
    // agree because the program collapsed to nothing agree for the
    // wrong reason.
    for variant in EXPRESSION_VARIANTS {
        assert!(
            text.contains(&format!("\"{variant}\"")),
            "the corpus offered to the byte pin carries no `{variant}` expression, so \
             these bytes say nothing about how one persists"
        );
    }

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(FILE);
    if std::env::var_os("PNCAD_BLESS").is_some() {
        std::fs::write(&path, &text).expect("the fixture writes");
        return;
    }
    let recourse = "regenerate it: PNCAD_BLESS=1 cargo test -p editor-core \
                    --test all wire_rv_bytes (default env) — after deciding the new \
                    arrangement is right, and with the other checked-in documents";
    let committed = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the every-variant fixture is missing: {e} — {recourse}"));
    if text == committed {
        return;
    }
    let (line, was, now) = text
        .lines()
        .zip(committed.lines())
        .enumerate()
        .find(|(_, (a, b))| a != b)
        .map_or_else(
            || {
                (
                    text.lines().count().min(committed.lines().count()),
                    "<the shorter text ends here>",
                    "<and the longer one continues>",
                )
            },
            |(i, (a, b))| (i + 1, b.trim(), a.trim()),
        );
    panic!(
        "the persisted arrangement of the profile program moved, first at line {line}: \
         the committed bytes have `{was}` and this build writes `{now}`. A document the \
         previous build saved no longer reads the same — {recourse}"
    );
}
