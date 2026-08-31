//! Corpus document **die_composed** — M6 unit 1's composition
//! surgery, as a RECIPE: the pipped cube from `die_pips` with a
//! `Node::Fillet` on top, so ONE evaluation carries the in-place
//! box-edge blends (rings carried through), the pip cavity, AND the
//! pip-rim torus band — the corpus's first TORUS minted by an op
//! rather than a revolve, and its first body where the fillet's two
//! assembly doors both stand behind one node.
//!
//! # Registered by M6-5, and exactly what it took
//!
//! For a milestone this document sat BESIDE the registry (the M5
//! `die_fillet` precedent) with its refusal pinned. `Node::Fillet`
//! targeted EVERY edge of its input by design — no stable edge names
//! meant nothing to go stale under a bump — and on a pipped cube
//! "every edge" includes the cap's MERIDIAN seam edges. Two half-cap
//! faces share ONE sphere surface, so a meridian has no dihedral
//! wedge, and the battery honestly refuses
//! `BlendError::TangentialEdge` at a margin of exactly zero. That
//! refusal is CORRECT — a co-surface seam cannot be blended at any
//! radius — so the composed die was not unbuildable, it was
//! UNSAYABLE.
//!
//! M6-5 says it. The node grew a `selection` of stable names, the
//! composition surgery grew per-entity birth records, and the naming
//! emitter that reads them turns the result into a full name table.
//! The selection below is fourteen names — every edge of the target
//! except the two co-surface meridians — so the refusal is EXCLUDED
//! rather than loosened, and it is still executed as such in
//! `m6_composed_node.rs`, which now pins the flip instead of the
//! blocker.
//!
//! ONE `fillet_edges` call carries all of it: the M6-5 spec's F-e
//! measurement established that twelve open plane–plane chains and one
//! closed plane–sphere rim go through together
//! (`sweep/tests/m6_5_fillet_naming.rs`). The surgery's own live rows
//! still ride `sweep/tests/m6_surgery.rs` (+ `_interval`) and the demo
//! tour's `diecomposed` stop.
//!
//! ONE pip, exactly as `die_pips` and for `die_pips`' documented
//! reason: this document's subject is the surgery, not the pip count.
//!
//! # No mass pin
//!
//! π-valued closed forms are not dyadic (the `die_fillet` /
//! `die_pips` precedent), so `pin: None`; the sweep unit
//! (`sweep/tests/m6_surgery.rs`) meters the composed volume against
//! its derived closed form at a stated relative tolerance.

use editor_core::{
    Axis3, BooleanOp, CapEnd, Datum, DocEdit, EntityKind, LoopProgram, MeridianEnd, NamePat, Node,
    ProfileEdgeRef, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId,
    RoleSeg, SegPat, SegTag, Selector, SlotId, StableName,
};
use geom_core::{Point3, Vec3};
use profile::SketchPlane;

use super::super::fixture::{ang, len, prism_edges, scl};
use super::{CorpusDoc, Recorder};

/// The die's side, meters.
pub const DIE_L: f64 = 1.0;
/// The pip ball's radius, meters.
pub const PIP_R: f64 = 0.09;
/// How deep the pip ball dips into its face, meters.
pub const PIP_H: f64 = 0.05;
/// The one blend radius (box edges AND the pip rim), meters —
/// dyadic, under `PIP_R` (predicate 1 on the sphere support) and
/// under the rim's spine radius (predicate 3).
pub const R: f64 = 0.05;

/// The pip ball's centre coordinate along its face normal (the
/// `die_pips` geometry, verbatim).
const PIP_C: f64 = DIE_L + (PIP_R - PIP_H);

/// **The selection: fourteen names, each with its provenance.**
///
/// The target is the pipped cube (`Node::Boolean::Subtract`), whose
/// edge table has exactly sixteen rows. The selection is fourteen of
/// them — everything except the two cavity meridians:
///
/// | count | name | what it is |
/// |---|---|---|
/// | 8 | `FromA(RimEdge(Top\|Bottom, seg))` | the cube's cap–wall rims, surviving the subtraction |
/// | 4 | `FromA(LateralEdge(vertex))` | the cube's four vertical struts |
/// | 2 | `Seam { Cap(Top), Band(0) \| BandPi(0) }` | the PIP RIM — the two arcs the zip minted where the cube's top cap crosses the ball's lower band (a full revolve's band is two half-faces, split at the seam meridians, so the rim is two arcs, not one circle) |
/// | *2 excluded* | `FromB(Meridian(Seam\|Pi, 0))` | the cavity's meridian seams |
///
/// **Why the meridians are excluded, and why this document exists.**
/// The two half-cap faces of the cavity share ONE sphere surface, so a
/// meridian between them has no dihedral wedge — nothing to roll a
/// ball into. `fillet_edges` refuses it `TangentialEdge` at margin
/// exactly zero, correctly, at any radius. That refusal is what kept
/// this document out of the registry while `Node::Fillet` meant
/// "every edge" (`m6_composed_node.rs`, now flipped): the composed die
/// was not INEXPRESSIBLE, it was unnameable. Fourteen names later it
/// is neither.
///
/// **This list is now the ORACLE, not the source** (LIB-U7). The
/// document below no longer hand-authors its selection: it evaluates
/// the target and materializes [`selector`] against the target's own
/// name table, which is what a document author can do too. The
/// fourteen names spelled out here stay because they are the
/// independent statement of what the selector must produce — the
/// equivalence is pinned in `lib_u7_select.rs`, and a drift in
/// either the boolean emitter or the selector fails loudly against
/// this list rather than silently reselecting.
///
/// The result is still FROZEN: materializing happens once, at
/// authoring time, and the names go into the recipe (`Node::Fillet`'s
/// payload docs; the growth path is still `DocEdit::Rebind`).
pub fn selection(cube: RecipeNodeId, ball: RecipeNodeId, pipped: RecipeNodeId) -> Vec<StableName> {
    let at = |seg: RoleSeg| StableName {
        kind: EntityKind::Edge,
        node: pipped,
        path: vec![seg],
    };
    let cap_top = StableName {
        kind: EntityKind::Face,
        node: cube,
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    };
    let ball_face = |seg: RoleSeg| StableName {
        kind: EntityKind::Face,
        node: ball,
        path: vec![seg],
    };
    let lower = ProfileEdgeRef {
        loop_index: 0,
        segment: 0,
    };
    let mut out: Vec<StableName> = prism_edges(cube, 4)
        .into_iter()
        .map(|e| at(RoleSeg::FromA(Box::new(e))))
        .collect();
    for band in [RoleSeg::Band(lower), RoleSeg::BandPi(lower)] {
        out.push(at(RoleSeg::Seam {
            a: Box::new(cap_top.clone()),
            b: Box::new(ball_face(band)),
        }));
    }
    out
}

/// **The same fourteen names, said STRUCTURALLY** (LIB-U7): a
/// selector over role-path shape, materialized against the target's
/// table. It takes no node ids at all — the shape of the names IS the
/// description.
///
/// Two alternatives, one per row of `selection`'s table:
///
/// 1. `FromA(_)` — every edge the subtraction carried through from
///    operand A, i.e. the cube's twelve box edges (eight cap rims and
///    four struts). Saying "carried through from A" is exactly the
///    intent; enumerating twelve `RimEdge`/`LateralEdge` names was
///    the P10 relocation.
/// 2. `Seam { a: Cap(Top), b: Band | BandPi }` — the pip rim, the two
///    arcs the zip minted where the cube's top cap crosses the ball's
///    lower band. A full revolve's band is two half-faces split at the
///    seam meridians, so the rim is two arcs and the band role has two
///    variants; the union says both.
///
/// What is NOT selected falls out of the shapes: the cavity meridians
/// are `FromB(Meridian(..))`, and neither alternative admits a `FromB`
/// segment. The co-surface refusal stays EXCLUDED by description
/// rather than by a hand-maintained omission.
///
/// No geometric field appears anywhere above — no carrier kind, no
/// adjacent-surface pair, no position. That is the LB7 fence
/// (`crates/editor-core/src/names/select.rs` module docs).
pub fn selector() -> Selector {
    let edge = || NamePat::of_kind(EntityKind::Edge);
    let face = |tag: SegTag| NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(tag));
    let cap_top =
        NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top));
    let pip_rim =
        |band: SegTag| edge().seg(SegPat::tag(SegTag::Seam).of([cap_top.clone(), face(band)]));
    Selector::any_of([
        edge().seg(SegPat::tag(SegTag::FromA)),
        pip_rim(SegTag::Band),
        pip_rim(SegTag::BandPi),
    ])
}

/// The two cavity meridians — the names the selection deliberately
/// LEAVES OUT (the co-surface seams). Named here so the pin that
/// checks the exclusion reads the same list the docs above describe.
pub fn excluded_meridians(ball: RecipeNodeId, pipped: RecipeNodeId) -> Vec<StableName> {
    [MeridianEnd::Seam, MeridianEnd::Pi]
        .into_iter()
        .map(|end| StableName {
            kind: EntityKind::Edge,
            node: pipped,
            path: vec![RoleSeg::FromB(Box::new(StableName {
                kind: EntityKind::Edge,
                node: ball,
                path: vec![RoleSeg::Meridian(
                    end,
                    ProfileEdgeRef {
                        loop_index: 0,
                        segment: 0,
                    },
                )],
            }))],
        })
        .collect()
}

/// The composed-die corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let h = DIE_L / 2.0;

    // ---- the sharp cube, [0, L]³ (die_pips' chain, verbatim) ----
    let square =
        LoopProgram::polygon([(0.0, 0.0), (DIE_L, 0.0), (DIE_L, DIE_L), (0.0, DIE_L)]).unwrap();
    let cube_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![square],
    }));
    let cube = r.insert(Node::Extrude {
        profile: cube_p,
        distance: len(DIE_L),
    });

    // ---- the master ball, poled along the +Z face normal ----
    let axis = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    // die_pips' half-disc: the bulge-1 semicircle closed on-axis.
    let half_disc = half_disc_program();
    let ball_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::from_frame(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        loops: vec![half_disc],
    }));
    let ball = r.insert(Node::Revolve {
        profile: ball_p,
        axis,
        angle: ang(std::f64::consts::TAU),
    });

    // ---- the pip, then the SURGERY on the pipped cube ----
    let pip = r.insert(Node::Transform {
        input: ball,
        translation: [len(h), len(h), len(PIP_C)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    });
    let pipped = r.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a: cube,
        b: pip,
        declare: None,
    });
    // The fourteen selected edges — twelve box edges and the pip
    // rim's two arcs; the cavity meridians are NOT in the set (see
    // `selection`'s table). One node, one call: the F-e measurement
    // (M6-5 §0) established that a single `fillet_edges` takes the
    // twelve open chains and the closed rim together.
    //
    // LIB-U7: MATERIALIZED, not authored — evaluate the target, run
    // the structural `selector()` over its name table, store the
    // result. This is the whole authoring loop a document author has
    // (the `all_edges` precedent, `names/mod.rs`), and it is why the
    // fourteen names below are no longer spelled by hand here. The
    // stored set is frozen from this point exactly as before.
    let ev = super::eval::<f64>(&r.doc);
    let composed = r.insert(Node::fillet(
        pipped,
        len(R),
        editor_core::select(&ev, pipped, &selector()),
    ));

    CorpusDoc {
        name: "die_composed",
        about: "M6 unit 1: the pipped cube filleted in place — box blends, octants, and the \
                pip-rim torus band in one body",
        edits: r.edits,
        doc: r.doc,
        result: Some(composed),
        // π-valued closed forms are not dyadic — module docs.
        pin: None,
        // D2's incremental probe: slide the pip (die_pips' bump). The
        // cube chain and the master ball are reused; the placement,
        // the cut and the whole surgery recompute. The bump mints no
        // edge and retires none, so the FROZEN selection still
        // resolves — bit-identically, because every selected name is a
        // function of the target's names and those do not move. That
        // is the covariance claim, and `m6_composed_node.rs` executes
        // it.
        bump: DocEdit::SetParam {
            node: pip,
            slot: SlotId::Translation(Axis3::Y),
            expr: len(0.53125),
        },
        bump_root: pip,
    }
}

/// The half-disc loop PROGRAM (die_pips' twin).
fn half_disc_program() -> LoopProgram {
    let lpt = |x: f64, y: f64| {
        [
            editor_core::Expr::literal(x, editor_core::Dimension::Length).unwrap(),
            editor_core::Expr::literal(y, editor_core::Dimension::Length).unwrap(),
        ]
    };
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, -PIP_R)),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(lpt(0.0, PIP_R)),
            b: editor_core::Expr::literal(1.0, editor_core::Dimension::Scalar).unwrap(),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}
