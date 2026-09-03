//! Corpus document **measured_web** — the measurement vocabulary in
//! the registry (ERROR-DESIGN E3/E10; M10-2).
//!
//! **Why it exists.** M10-2 added `Measure` and `Assertion` arms to
//! the Dual/Interval corpus digests, and those arms were UNREACHABLE:
//! no registered document carried either node, so the digest walk
//! never entered them while the PR's own sweep table presented them as
//! coverage. Registering this document is what makes them live —
//! every scalar lane, every ε row, the persistence round trip and the
//! incremental probe now run over a measured document.
//!
//! **The shape**: a plate with two cylindrical hole tools beside it —
//! the worked example's geometry — plus a `Measure` of the web between
//! the two hole walls (`distance(wall, wall) − 2·hole_r`) and an
//! `Assertion` that the web clears a minimum. The tools are separate
//! extrudes on purpose: it makes the measure CROSS-NODE, so the
//! digest sees a measure whose two references are two different DAG
//! edges rather than a degenerate one.
//!
//! No mass pin: the head is a measurement sink, not a body, so
//! `result` is `None` for the reason `cut_cylinder`'s is — the
//! document's point is not a single solid.
//!
//! The bump edits `hole_r`, which moves the measured value through a
//! parameter rather than through geometry: the measure's own key must
//! move with it (the payload-expression channel), which is exactly the
//! property the incremental probe is there to exercise.

use editor_core::UnitSym;
use editor_core::{
    AssertionDir, Dimension, DocEdit, DocParam, Expr, LoopProgram, MeasureExpr, MeasurePrimitive,
    MeasureRef, Node, ParamName, ProfileProgram,
};
use geom_core::Tol;
use profile::SketchPlane;

use super::{CorpusDoc, Recorder};
use crate::fixture::len;

/// The parameter driving both holes.
pub const HOLE_R: &str = "hole_r";
/// Hole centres at x = ±`HOLE_X`.
pub const HOLE_X: f64 = 0.30;
/// The authored hole radius.
pub const R0: f64 = 0.2;
/// The assertion's bound.
pub const MIN_WEB: f64 = 0.0005;

/// The measured-web corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new(HOLE_R),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: R0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: None,
        },
    });

    let plate_profile = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(-1.0, -0.5), (1.0, -0.5), (1.0, 0.5), (-1.0, 0.5)])
                .expect("finite plate corners"),
        ],
    }));
    let plate = r.insert(Node::Extrude {
        profile: plate_profile,
        distance: len(0.1),
    });

    let hole = |cx: f64| {
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: Expr::param(ParamName::new(HOLE_R), Dimension::Length),
            }],
        })
    };
    let pa = r.insert(hole(-HOLE_X));
    let hole_a = r.insert(Node::Extrude {
        profile: pa,
        distance: len(0.1),
    });
    let pb = r.insert(hole(HOLE_X));
    let hole_b = r.insert(Node::Extrude {
        profile: pb,
        distance: len(0.1),
    });

    // The wall names come from the SELECTION door, the way a user gets
    // them — evaluate what is built so far, then ask each hole for its
    // cylindrical faces. A circular extrude's wall is two faces
    // sharing one cylinder carrier, and the closed form reads the
    // carrier, so the first in canonical order answers for the hole.
    //
    // Read AT the extrude that owns it: nothing places this geometry,
    // so the reading site is the minting node — spelled explicitly
    // rather than assumed.
    let ev = editor_core::evaluate::<f64>(
        &r.doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    let wall = |node| {
        let mut faces = editor_core::select_where(
            &ev,
            node,
            &editor_core::Selector::of(editor_core::NamePat::of_kind(
                editor_core::EntityKind::Face,
            )),
            &[editor_core::GeomPred::SurfaceKind(
                editor_core::SurfaceKindSet::just(geom_brep::SurfaceKind::Cylinder),
            )],
            &r.doc.param_env::<f64>(),
            Tol::witness(),
        )
        .expect("the surface-kind atom is exact");
        faces.sort();
        assert!(!faces.is_empty(), "a hole extrude has a cylindrical wall");
        MeasureRef::new(node, faces.remove(0))
    };
    let radius = || MeasureExpr::value(Expr::param(ParamName::new(HOLE_R), Dimension::Length));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(radius(), radius()).expect("Length + Length"),
    )
    .expect("Length - Length");
    let measure = r.insert(
        Node::measure(web, vec![wall(hole_a), wall(hole_b)]).expect("both indices in range"),
    );
    let _assertion = r.insert(Node::Assertion {
        measure,
        bound: Expr::literal(MIN_WEB, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });

    CorpusDoc {
        name: "measured_web",
        about: "M10-2: a measured web with an assertion over it (E3/E10)",
        edits: r.edits,
        doc: r.doc,
        // The head is a measurement sink, not a body.
        result: None,
        pin: None,
        // The plate's own thickness: a cone of exactly one node, so
        // the hole chains and the measure over them are the reuse the
        // incremental probe is there to observe.
        //
        // NOT the `hole_r` parameter, tempting as that is: a doc-param
        // edit reaches both holes AND the measure that reads them, so
        // its cone is most of the document and there is little left to
        // reuse. That the measured value moves with `hole_r` is a real
        // claim and it is asserted where it belongs — over the
        // two-hole plate in `m10_2_measure.rs`, which flips an
        // assertion by exactly that edit.
        bump: DocEdit::SetParam {
            node: plate,
            slot: editor_core::SlotId::Distance,
            expr: len(0.125),
        },
        bump_root: plate,
    }
}
