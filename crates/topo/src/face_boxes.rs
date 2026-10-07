//! **Every face's certified box over one body, with one C10 tree** — a
//! candidate-pruning driver for a caller that meters a region against
//! the faces of a body and wants to touch only the faces that can reach
//! it.
//!
//! Each box is [`crate::boolean::boxes::face_box`] at the sweep pad: a
//! genuine superset of the face's locus on every surface kind, so a face
//! whose box misses a query box cannot meet anything the query box
//! encloses. That is the only claim, and it runs in the conservative
//! direction (a box-level miss is a disjointness certificate; a box-level
//! meet says nothing): a caller prunes with it and decides with its own
//! exact predicates. A POISON box (an unboxable boundary edge) meets every
//! query, so an unboxable face is never pruned away.
//!
//! Every face of the body is boxed, in any shell of any solid: the faces
//! a region can reach are not confined to the shell its own faces lie in.

use bvh::{Aabb, Bvh};
use geom_core::{Band, Bounds, Decide};

use crate::body::Body;
use crate::boolean::BooleanError;
use crate::boolean::boxes::{face_box, sweep_pad};
use crate::entity::FaceKey;

/// A face's certified box as two corners, `lo ≤ hi` per axis — NaN at
/// every coordinate for a poison box.
pub type FaceBox = ([f64; 3], [f64; 3]);

/// Every face of one body with its certified box, and the tree over
/// them (module docs).
#[derive(Debug, Clone)]
pub struct FaceBoxes {
    /// The faces, in the body's face-arena order (D9).
    faces: Vec<FaceKey>,
    /// `faces[i]`'s padded certified box.
    boxes: Vec<Aabb>,
    /// The tree over `boxes`.
    tree: Bvh,
}

impl FaceBoxes {
    /// Boxes every face of `body` under `band`, and builds the tree once.
    ///
    /// # Errors
    ///
    /// The face that could not be boxed, with the box builder's own
    /// [`BooleanError`]: a cylinder carrier whose axis has no decided
    /// length.
    pub fn of<T: Decide + Bounds>(
        body: &Body<T>,
        band: Band,
    ) -> Result<Self, (FaceKey, BooleanError)> {
        let pad = sweep_pad(band);
        let mut faces = Vec::new();
        let mut boxes = Vec::new();
        for (face, _) in body.faces() {
            faces.push(face);
            boxes.push(face_box(body, face, pad, band).map_err(|e| (face, e))?);
        }
        let tree = Bvh::build(&boxes);
        Ok(Self { faces, boxes, tree })
    }

    /// How far every box reaches past its face's locus: a box's cells
    /// that lie outside the face lie within this of it.
    #[must_use]
    pub fn pad(band: Band) -> f64 {
        sweep_pad(band)
    }

    /// The faces whose box meets the closed box `[lo, hi]`, each with its
    /// own box, in arena order. A poison query box meets every face.
    #[must_use]
    pub fn meeting(&self, lo: [f64; 3], hi: [f64; 3]) -> Vec<(FaceKey, FaceBox)> {
        let query = Aabb {
            min_x: lo[0],
            min_y: lo[1],
            min_z: lo[2],
            max_x: hi[0],
            max_y: hi[1],
            max_z: hi[2],
        };
        let hits = self.tree.overlapping(&query);
        hits.into_iter()
            .map(|i| {
                let b = &self.boxes[i];
                (
                    self.faces[i],
                    ([b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]),
                )
            })
            .collect()
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// A query near one face of a brick meets that face and the four
    /// that share its edges, never the face opposite; a poison query
    /// prunes nothing.
    #[test]
    fn a_query_meets_the_faces_whose_boxes_it_meets_and_poison_meets_all() {
        let tol = Tol::witness();
        let body: Body<f64> = crate::test_support::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let band = Band::linear(tol).expect("the witness band");
        let boxes = FaceBoxes::of(&body, band).expect("every face boxes");
        let near_top = boxes.meeting([0.4, 0.4, 0.9], [0.6, 0.6, 1.1]);
        assert_eq!(
            near_top.len(),
            1,
            "only the top face reaches the top's middle"
        );
        let top_edge = boxes.meeting([0.4, -0.1, 0.9], [0.6, 0.1, 1.1]);
        assert_eq!(
            top_edge.len(),
            2,
            "the top and the front meet along their edge"
        );
        let all = boxes.meeting([f64::NAN; 3], [f64::NAN; 3]);
        assert_eq!(all.len(), 6, "a poison query prunes nothing");
    }
}
