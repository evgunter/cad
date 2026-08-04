//! Corpus document **boss_union** — M5 PR 9's acceptance shape (ii):
//! the first transverse CURVED BOOLEAN in the corpus. A three-arc
//! cylinder boss pierces a plate's top face; the union's seam is the
//! rim circle, minted as exact `Circle` arcs on both operands and
//! described intrinsically. Joining the corpus registry gives the
//! document the standard rows for free — evaluation at every CI ε row
//! and under `interval`, persistence round-trip (D6.1), and the
//! latency table (the deviation-9 rows, landed at the fix pass).
//!
//! No dyadic mass pin (the boss volume carries π); the closed-form
//! volume and the interval bracket are pinned by
//! `review_m5_pr9_doc_probe.rs`, and validity + the seam-arc counts
//! are pinned by the boolean's own acceptance suites.

use editor_core::{BooleanOp, DocEdit, Node, ProfileDesc, SlotId};
use geom_core::{Affine3, Point2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};

use super::super::fixture::len;
use super::{CorpusDoc, Recorder};

/// The boss-union corpus document: 3×3×0.8 plate ∪ r = 0.35 three-arc
/// boss at (1.2, 1.7), sketched at z = 0.3, extruded 1.0 (pokes 0.5
/// out of the plate).
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    let plate_loop = ProfileLoop::new(
        [(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex {
                pos: Point2::new(x, y),
                bulge: 0.0,
            })
            .collect(),
    );
    let plate_p = r.insert(Node::Profile(ProfileDesc(Profile::new(
        SketchPlane::xy(),
        vec![plate_loop],
    ))));
    let plate = r.insert(Node::Extrude {
        profile: plate_p,
        distance: len(0.8),
    });

    let theta = 2.0 * std::f64::consts::PI / 3.0;
    let bulge = (theta / 4.0).tan();
    let at = |i: usize| {
        #[allow(clippy::cast_precision_loss)]
        let th = theta * i as f64;
        Point2::new(1.2 + 0.35 * th.cos(), 1.7 + 0.35 * th.sin())
    };
    let boss_loop = ProfileLoop::new(
        (0..3)
            .map(|i| ProfileVertex { pos: at(i), bulge })
            .collect(),
    );
    let boss_p = r.insert(Node::Profile(ProfileDesc(Profile::new(
        SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.3))),
        vec![boss_loop],
    ))));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(1.0),
    });

    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: plate,
        b: boss,
        declare: None,
    });

    CorpusDoc {
        name: "boss_union",
        about: "M5 shape (ii): cylinder boss ∪ plate — the first transverse curved boolean",
        edits: r.edits,
        doc: r.doc,
        result: Some(union),
        pin: None, // π volume is not dyadic; the doc probe pins the closed form
        // D2's incremental probe: grow the boss — the plate chain is
        // reused, the boss extrude + union recompute.
        bump: DocEdit::SetParam {
            node: boss,
            slot: SlotId::Distance,
            expr: len(1.125),
        },
        bump_root: boss,
    }
}
