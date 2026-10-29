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
//! this paragraph predicted it would: `bool_conic_curved_clearance`
//! proves far circle-vs-curved pairs a definite miss, so the two
//! strategies agree on disjoint balls again. The document keeps ONE
//! pip anyway — the multi-ball tool adds twenty nodes and no new
//! machinery here; the 21-pip die is metered in
//! `sweep/tests/m6_surgery.rs` and the surgery's own corpus document
//! is `die_composed`.
//!
//! # One further REPORTED deviation
//!
//! **(a) The pip stands on the face's axis, not a rotated copy.** The
//! sweep unit rotates one +Y-poled ball onto each face normal. Here the
//! ball revolves about its own pole, and its copy stands by an axis
//! mate from that pole onto the +Z face's centre line — a frame read
//! off the cube's corner, slid by exact moves (`moved_frame`) — its
//! spin the constant `0`, so the chart's polar axis lies on the face
//! normal exactly rather than ~1e-16 off it.
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

use super::PlacePart;
use editor_core::Formula;
use editor_core::{
    Coord, FrameMove, LoopProgram, Node, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, TargetAxis,
};

use crate::fixture::{ang, len, len2, revolve_about, scl, value_bump};

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
        loops: vec![square],
        ids: Vec::new(),
        on: None,
    }));
    let cube = r.insert(Node::Extrude {
        profile: cube_p.into(),
        distance: len(DIE_L),
    });

    // ---- the master ball, poled along the +Z face normal ----
    // The revolve axis IS that normal (deviation (a)), so the chart is
    // polar to the plane that cuts it, exactly.
    // The sweep unit's own meridian: one exact half-circle from pole
    // to pole, closed by the on-axis chord. Both vertices are ON the
    // axis; the revolve names them from the sweep's pole export.
    let half_disc = half_disc_program();
    // The pole axis is the meridian's own +y through (0, 0).
    let ball_p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![half_disc],
        ids: Vec::new(),
        on: None,
    }));
    let ball = r.insert(revolve_about(
        ball_p,
        (0.0, 0.0),
        crate::fixture::along_y(),
        ang(std::f64::consts::TAU),
    ));

    // ---- the pip: the face-1 ball, at the +Z face centre ----
    // Placed against the cube on its own pole axis, the meridian's +y
    // along the cube's +Z (deviation (a): the chart is polar to the
    // plane that cuts it).
    let own = r.corner_frame(cube, [0.0; 3], [0.0, 0.0, DIE_L], [DIE_L, 0.0, 0.0]);
    let pole = editor_core::moved_frame(
        own,
        &[
            FrameMove::Slide(TargetAxis::X, len(h)),
            FrameMove::Slide(TargetAxis::Y, len(h)),
        ],
    );
    let pip = r.on_axis(ball, pole, len(PIP_C), crate::fixture::meridian_up());
    let pipped = r.insert(Node::Subtract {
        from: cube.into(),
        tool: pip.into(),
    });

    r.place_part(pipped);

    let bump = value_bump(&r.doc, pip, Coord::Slide(TargetAxis::Z), 0.015625);
    CorpusDoc {
        name: "die_pips",
        about: "M5 shape (v) stage 2: a spherical pip cut into a cube face (the corpus's first sphere)",
        edits: r.edits,
        doc: r.doc,
        result: Some(pipped),
        // π-valued spherical caps are not dyadic — see the module docs.
        pin: None,
        // D2's incremental probe: slide the pip along its pole axis. The
        // cube chain and the master ball are reused; the placement and
        // the cut recompute. The pip stays a cap-cutting ball — same
        // face, its cap a little deeper or shallower.
        bump,
        bump_root: pip,
    }
}

/// The half-disc loop PROGRAM: the bulge-1 semicircle pole to pole,
/// closed by its on-axis diameter — three steps, both vertices on the
/// revolve axis.
pub fn half_disc_program() -> LoopProgram<Formula> {
    LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, -PIP_R])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(len2([0.0, PIP_R])),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}
