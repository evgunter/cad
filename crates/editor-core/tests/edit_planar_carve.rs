//! **A bore a radius edit puts wholly inside a plane–plane band's
//! sliver vanishes on the blend's verdict** — the planar twin of
//! `edit_ruled_carve`'s bored D-rod. A unit square extruded a unit along
//! `+z`, bored along `z` by a circle of radius `0.006` at
//! `(0.988, 0.988)`, its lateral edge at `(1, 1)` filleted or
//! chamfered. The band's cut-off at each cap removes the sliver between
//! the end curve and the old vertex, and the prism over it between the
//! caps.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;
use editor_core::{
    EntityKind, EvalOptions, LoopProgram, NameRef, Node, ProfileDoc, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, RoleSeg,
};
use fixture::{len, minted, table, tol};
use topo::Body;

/// The bore's radius.
const A: f64 = 0.006;

fn volume(body: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "closed-form mass properties",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
    )
    .volume
}

/// The bored square blended at its edge at `(1, 1)` at size `r`, a
/// chamfer's setback or a fillet's radius: the document, its extrude
/// node and its blend node.
fn bored_square_doc(chamfer: bool, r: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let at = |x: f64, y: f64| ProgramStep::LineTo(ProgramTarget::Point([len(x), len(y)]));
    let loops = vec![
        LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            at(1.0, 0.0),
            at(1.0, 1.0),
            at(0.0, 1.0),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]),
        LoopProgram::Circle {
            centre: [len(0.988), len(0.988)],
            radius: len(A),
        },
    ];
    let doc = ProfileDoc::empty_derived("bored_square", tol());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let crease = fixture::ename(
        block,
        RoleSeg::LateralEdge(fixture::vpiece(&doc, block, 0, 2)),
    );
    let selection = editor_core::Operand::select(block, vec![crease]);
    let (doc, blend) = fixture::insert(
        doc,
        if chamfer {
            Node::Chamfer {
                distance: len(r),
                selection,
            }
        } else {
            Node::Fillet {
                radius: len(r),
                selection,
            }
        },
    );
    (doc, block, blend)
}

/// **The bore's wall, carried through a blend of size `0.005`, is
/// consumed by one of `0.1` and vanishes on the face-clearance flip**,
/// under a fillet and under a chamfer. At `0.005` the band removes a
/// part of `[0.995, 1]²`, which the bore (out to `0.994`) is clear of;
/// at `0.1` the bore lies wholly inside the sliver and the carve takes
/// it, at `ΔV = −(A − π·a²)` (`A` the fillet's `(1 − π/4)·r²` or the
/// chamfer's `r²/2`). The wall's name at the blend node, which resolved
/// before the edit, resolves `Vanished` after it, its cause a
/// `Positive → Negative` flip of one of the two clearance verdicts the
/// consumption is logged under. Which one is the first net flip in
/// predicate-name order (N5's population diff, `resolve/vdiff.rs`), and
/// which flips there are depends on what the old run's reach metered:
/// the chamfer's smaller box at `0.005` never meters the bore's walls,
/// so its face clearance only gains verdicts and has no net flip, and
/// its name vanishes on ring clearance where the fillet's vanishes on
/// face clearance. The row holds the cause to the two, not to one.
#[test]
fn a_bore_a_size_edit_puts_inside_a_planar_sliver_vanishes_on_the_blends_verdict() {
    for chamfer in [false, true] {
        a_bore_vanishes_after_the_size_edit(chamfer);
    }
}

fn a_bore_vanishes_after_the_size_edit(chamfer: bool) {
    let verb = if chamfer { "chamfer" } else { "fillet" };
    let (doc, block, fillet) = bored_square_doc(chamfer, 0.005);
    let wall = fixture::fname(
        block,
        RoleSeg::Lateral(fixture::piece(&doc, block, 1, 0).into()),
    );
    let carried = minted(
        EntityKind::Face,
        fillet,
        RoleSeg::From {
            read: fixture::out(&doc, block),
            of: NameRef::new(wall),
        },
    );
    let ev1 = fixture::run(&doc, &EvalOptions::default());
    assert!(
        table(&ev1, fillet).lookup(&carried).is_some(),
        "{verb}: at 0.005 the blend carries the bore's wall through"
    );
    let r = 0.1;
    let (doc2, _) = fixture::step(
        doc.clone(),
        editor_core::DocEdit::SetParam {
            node: fillet,
            slot: if chamfer {
                editor_core::SlotId::ChamferDistance
            } else {
                editor_core::SlotId::Radius
            },
            value: len(r).into(),
            fresh: Vec::new(),
        },
    );
    let ev2 = fixture::run(&doc2, &EvalOptions::default());
    assert!(
        table(&ev2, fillet).lookup(&carried).is_none(),
        "{verb}: at 0.1 the carve takes the bore's wall"
    );
    let out = corpus::body_of(&ev2, fillet);
    topo::validate_geometric(&out, tol()).unwrap_or_else(|e| panic!("{verb}: tier 3, {e:?}"));
    topo::validate_pseudomanifold(&out, &topo::ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{verb}: tier 3′, {e:?}"));
    let pi = core::f64::consts::PI;
    let dv = volume(&out) - volume(&(corpus::body_of(&ev2, block)));
    let section = if chamfer {
        r * r / 2.0
    } else {
        (1.0 - pi / 4.0) * r * r
    };
    let want = -(section - pi * A * A);
    assert!((dv - want).abs() < 1e-12, "{verb}: ΔV {dv} vs {want}");
    let res = editor_core::resolve_with_prior(
        editor_core::RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        editor_core::RunCtx {
            doc: &doc,
            eval: &ev1,
        },
        doc2.read_of_node(fillet).expect("the fillet's read"),
        &carried,
    )
    .expect("a body read");
    let editor_core::Resolution::Failed(f) = res else {
        panic!("{verb}: the consumed wall's name resolves Failed, got {res:?}");
    };
    let editor_core::ResolveError::Vanished { diagnosis, .. } = f.error else {
        panic!(
            "{verb}: the consumed wall's name vanishes, got {:?}",
            f.error
        );
    };
    assert!(
        matches!(
            diagnosis,
            editor_core::Diagnosis::PredicateFlip {
                predicate: "fillet3_face_clearance" | "fillet3_ring_clearance",
                from: geom_core::Sign::Positive,
                to: geom_core::Sign::Negative,
            }
        ),
        "{verb}: the wall vanishes on a flip of the blend's clearance verdicts, got {diagnosis:?}"
    );
}
