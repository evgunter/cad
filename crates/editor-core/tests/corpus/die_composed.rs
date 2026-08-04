//! Corpus document **die_composed** — M6 unit 1's composition
//! surgery, as a RECIPE: the pipped cube from `die_pips` with a
//! `Node::Fillet` on top, so ONE evaluation carries the in-place
//! box-edge blends (rings carried through), the pip cavity, AND the
//! pip-rim torus band — the corpus's first TORUS minted by an op
//! rather than a revolve, and its first body where the fillet's two
//! assembly doors both stand behind one node.
//!
//! # NOT in the corpus registry, and exactly why (M6 unit 1)
//!
//! `Node::Fillet` targets EVERY edge of its input BY DESIGN (no
//! stable edge names to go stale under a bump). On the pipped cube
//! "every edge" includes the cap's MERIDIAN seam edges — the two
//! half-cap faces share ONE sphere surface, so a meridian has no
//! dihedral wedge, and the battery honestly refuses
//! `FilletError::TangentialEdge` with a zero margin. That refusal is
//! CORRECT (a co-surface seam cannot be blended at any radius), so
//! the composed die is not expressible until the recipe layer grows
//! an edge-SELECTION vocabulary — the N4 fillet-naming emitter,
//! banked in `eval/wire.rs`'s `wire_fillet` docs. This document is
//! written to registry shape and sits BESIDE the registry (the M5
//! `die_fillet` precedent), with the refusal pinned, executed, in
//! `m6_composed_node.rs`. The surgery's live rows ride
//! `sweep/tests/m6_surgery.rs` (+ `_interval`) and the demo tour's
//! `diecomposed` stop instead.
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
/// The one blend radius (box edges AND the pip rim), meters —
/// dyadic, under `PIP_R` (predicate 1 on the sphere support) and
/// under the rim's spine radius (predicate 3).
pub const R: f64 = 0.05;

/// The pip ball's centre coordinate along its face normal (the
/// `die_pips` geometry, verbatim).
const PIP_C: f64 = DIE_L + (PIP_R - PIP_H);

/// The composed-die corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let h = DIE_L / 2.0;

    // ---- the sharp cube, [0, L]³ (die_pips' chain, verbatim) ----
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
    let axis = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
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
    let composed = r.insert(Node::Fillet {
        target: pipped,
        radius: len(R),
    });

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
        // the cut and the whole surgery recompute — with no edge
        // selection to go stale, because `Node::Fillet` is every-edge.
        bump: DocEdit::SetParam {
            node: pip,
            slot: SlotId::Translation(Axis3::Y),
            expr: len(0.53125),
        },
        bump_root: pip,
    }
}
