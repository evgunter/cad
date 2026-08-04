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
//! # Two further REPORTED deviations
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
//! **(b) The ball's meridian is split at the equator.** The sweep
//! unit's ball is the two-vertex half-disc (one half-circle arc, one
//! on-axis chord) revolved fully. That loop's every vertex is ON the
//! axis, and the revolve NAME EMITTER refuses it typed — "revolve
//! vertex resolution exceeded elimination (all-on-axis loop)": pole
//! vertices resolve by elimination along the meridian chains, and a
//! two-pole loop offers no off-axis anchor to eliminate from (the
//! part-1 limitation `names::emit_sweep` reports, deferred, never
//! guessed). So no sphere at all reaches a `Node::Revolve` until that
//! emitter grows a discriminator. The document works within the
//! frontier rather than around it: the meridian is authored as TWO
//! quarter arcs meeting at an equator vertex, which is off-axis, sweeps
//! a rim circle, and anchors the elimination. The cost is that the ball
//! carries two band faces instead of one, and that the quarter arcs'
//! circles are derived from `tan(π/8)` rather than being the one exact
//! half-circle — both invisible here (the cut circle lies wholly inside
//! ONE band; the document claims no exact pin) and both worth deleting
//! the moment the emitter resolves two-pole loops.
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

use editor_core::{Axis3, BooleanOp, Datum, DocEdit, Node, ProfileDesc, SlotId};
use geom_core::{Point2, Point3, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};

use super::{CorpusDoc, Recorder};
use crate::fixture::{ang, len, scl};

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
    let square = ProfileLoop::new(
        [(0.0, 0.0), (DIE_L, 0.0), (DIE_L, DIE_L), (0.0, DIE_L)]
            .into_iter()
            .map(|(x, y)| ProfileVertex {
                pos: Point2::new(x, y),
                bulge: 0.0,
            })
            .collect(),
    );
    let cube_p = r.insert(Node::Profile(ProfileDesc(Profile::new(
        SketchPlane::xy(),
        vec![square],
    ))));
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
    // Two quarter arcs meeting at the OFF-AXIS equator vertex —
    // deviation (b): the anchor the pole elimination needs.
    let quarter = std::f64::consts::FRAC_PI_8.tan();
    let half_disc = ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(0.0, -PIP_R),
            bulge: quarter,
        },
        ProfileVertex {
            pos: Point2::new(PIP_R, 0.0),
            bulge: quarter,
        },
        ProfileVertex {
            pos: Point2::new(0.0, PIP_R),
            bulge: 0.0,
        },
    ]);
    let ball_p = r.insert(Node::Profile(ProfileDesc(Profile::new(
        // u = +X, v = +Z: the sketch's revolve axis lands on the world
        // +Z axis, which is the face normal.
        SketchPlane::from_frame(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        vec![half_disc],
    ))));
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
