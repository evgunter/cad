//! Corpus document **die_pips** — M5 PR 12's acceptance shape (v),
//! stage 2: the sharp cube with a spherical pip, expressed as a RECIPE.
//!
//! The geometry is `sweep/tests/m5_pr12_die.rs`'s, constant for
//! constant (`DIE_L`, `PIP_R`, `PIP_H`): a ball whose centre stands
//! `R − H` OUTSIDE the face plane, so the cavity it cuts is exactly a
//! spherical cap of height `H`. It keeps that file's chart discipline —
//! **the ball's polar axis points along the face it is cut by**, because
//! a plane×sphere section taken against a chart whose polar axis is
//! tilted to the plane is a typed frontier of the split-join (the
//! azimuth-anchored arc-side rule needs a polar section).
//!
//! This is the corpus's first SPHERE, at every CI ε row and under
//! Interval.
//!
//! # ONE pip, not twenty-one — and not even three (REPORTED)
//!
//! The sweep unit cuts all 21 pips as ONE boolean against a 21-shell
//! tool, and that discipline is load-bearing there: cutting them one at
//! a time presents a body already carrying a TRIMMED sphere face as the
//! next operand, which S13 refuses (the extent certificate needs the
//! closed-group discipline). **That tool is not corpus-expressible at
//! M5**, and the blocker is not node count — it is the recipe layer's
//! only way to assemble a multi-shell body, `Node::Boolean` with
//! `Union`:
//!
//! > `Boolean(Union)` of two DISJOINT pip balls evaluates green under
//! > the production `SweepStrategy::Realized` and refuses typed under
//! > `SweepStrategy::Idealized` with
//! > `BooleanError::CurvedPierceUnsupported { operand: A, .. }` — the
//! > unconditional conic-carrier-against-curved-face arm that
//! > `m5_pr12_die.rs`'s Deviation 1 Door A pins. The BVH prunes the
//! > pair because the balls are far apart; the brute-force sweep
//! > examines it and hits the arm, which fires regardless of distance.
//!
//! A corpus document is run through BOTH strategies by
//! `m5_pr8_bvh_diff.rs`, whose whole claim is that pruning changes
//! nothing. At M5 a ball∪ball tool would have broken that claim — the
//! union only "worked" because the tree hid a pair the unconditional
//! conic arm refused on. **The M6 rider retired that divergence** as
//! this paragraph predicted it would: `bool_circle_curved_clearance`
//! proves far circle-vs-curved pairs a definite miss, so the two
//! strategies agree on disjoint balls again. The document keeps ONE
//! pip anyway — the multi-ball tool adds twenty nodes and no new
//! machinery here; the 21-pip die is metered in
//! `sweep/tests/m6_surgery.rs` and the surgery's own corpus document
//! is `die_composed`.
//!
//! # One further REPORTED deviation
//!
//! **(a) A per-pole master ball, not a rotated copy.** The sweep unit
//! rotates one +Y-poled ball onto each face normal. `Node::Transform`
//! could do the same, but `sin`/`cos` of ±π/2 are not exact in `f64`,
//! so the rotated chart's polar axis would sit ~1e-16 off the face
//! normal — inside every ε row's band, but an approximation where an
//! exact placement was available. Instead the ball's revolve axis IS
//! the face normal (the sketch frame is chosen so its v-axis is +Z),
//! and the `Node::Transform` is translation-only, rotation angle
//! exactly `0`. This is the discipline `tests/fixture/mod.rs` already
//! records for the M4 die, applied to the chart instead of the volume
//! oracle.
//!
//! # No mass pin
//!
//! The oracle is `L³ − cap(R, H)` with `cap(r, h) = π h²(3r − h)/3` —
//! π-valued, hence not dyadic, while [`MassPin`](super::MassPin) is
//! asserted with `==` against an EXACT value. Pinning it would pin
//! `f64` rounding of a transcendental, not the geometry, so the pin is
//! `None` on purpose (the `cut_cylinder` / `boss_union` precedent). The
//! evaluated volume does meet the closed form at rounding scale
//! (0.999_424_041_346_841_8 against 0.999_424_040_759_… — the residue
//! is the cap quadrature's, not a modelling error), and it is the sweep
//! unit that meters that at a stated relative tolerance. What this
//! document pins is validity (tier 1 + closed) at every ε row, under
//! Interval, and through BOTH sweep strategies.

use editor_core::{
    Axis3, BooleanOp, Datum, DocEdit, LoopProgram, Node, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, SlotId,
};
use geom_core::{Point3, Vec3};
use profile::SketchPlane;

use super::super::fixture::{ang, len, scl};
use super::{CorpusDoc, Recorder};

/// The die's side, meters.
pub const DIE_L: f64 = 1.0;
/// The pip ball's radius, meters.
pub const PIP_R: f64 = 0.09;
/// How deep the pip ball dips into its face, meters.
pub const PIP_H: f64 = 0.05;

/// The pip ball's centre coordinate ALONG its face normal, in the
/// cube's `[0, L]³` frame: the face plane at `L`, plus the `R − H` the
/// centre stands outside it so the cavity is a cap of height exactly
/// `H`.
const PIP_C: f64 = DIE_L + (PIP_R - PIP_H);

/// The pipped-cube corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let h = DIE_L / 2.0;

    // ---- the sharp cube, [0, L]³ ----
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
    // The revolve axis IS that normal (deviation (a)), so the chart is
    // polar to the plane that cuts it, exactly.
    let axis = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    // The sweep unit's own meridian: one exact half-circle from pole
    // to pole, closed by the on-axis chord. Both vertices are ON the
    // axis; the revolve names them from the sweep's pole export.
    let half_disc = half_disc_program();
    let ball_p = r.insert(Node::Profile(ProfileProgram {
        // u = +X, v = +Z: the sketch's revolve axis lands on the world
        // +Z axis, which is the face normal.
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

    // ---- the pip: the face-1 ball, at the +Z face centre ----
    let pip = r.insert(Node::Transform {
        input: ball,
        translation: [len(h), len(h), len(PIP_C)],
        // Translation-only: the chart is already poled (deviation
        // (a)), so the rotation is the exact identity.
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    });
    let pipped = r.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a: cube,
        b: pip,
        declare: None,
    });

    CorpusDoc {
        name: "die_pips",
        about: "M5 shape (v) stage 2: a spherical pip cut into a cube face (the corpus's first sphere)",
        edits: r.edits,
        doc: r.doc,
        result: Some(pipped),
        // π-valued spherical caps are not dyadic — see the module docs.
        pin: None,
        // D2's incremental probe: slide the pip across its face. The
        // cube chain and the master ball are reused; the placement and
        // the cut recompute. The pip stays a cap-cutting ball — same
        // face, clear of the rim.
        bump: DocEdit::SetParam {
            node: pip,
            slot: SlotId::Translation(Axis3::Y),
            expr: len(0.53125),
        },
        bump_root: pip,
    }
}

/// The half-disc loop PROGRAM: the bulge-1 semicircle pole to pole,
/// closed by its on-axis diameter — three steps, both vertices on the
/// revolve axis.
pub fn half_disc_program() -> LoopProgram {
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
