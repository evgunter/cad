//! **A loop passing one point more than once**, read from the point keys
//! the body states.
//!
//! A loop's corner is a half-edge of its cycle, at the vertex it leaves
//! from. The loop passes a point key at two corners where two corners'
//! vertices share that key, and two such corners are one of two things:
//!
//! - **one vertex met twice**: a seam's edge walked both ways, or a
//!   wrap edge's tip ([`mod@crate::validate`]'s check 9 reads each such
//!   corner's wedge, so it reads every corner group);
//! - **two distinct vertices on one point**: a **pinch** of the loop,
//!   one vertex per cone of the solid there. On an at-rest body this is
//!   a slit's parted end (`crates/topo/README.md`, tier 3, "a slit of
//!   that loop"), and a section loop's pinch is what the split's
//!   [`crate::splitting`] fusion reads.
//!
//! The point key is the body's statement and no distance is read. Two
//! vertices at one position on two point keys are not read as one point
//! here; in a finished body such a pair is its producer's defect, which
//! the coincidence census reports (`crates/topo/README.md`, "A Zero-gap
//! vertex pair on two point keys in a finished body is its producer's
//! defect"). So on a valid body a positional reading of ONE LOOP agrees
//! with this one. A face's loops are read one at a time here: a ring
//! touching its face's outer loop passes one point in two loops, which no
//! per-loop reading sees, and a reader whose question is positional over
//! the whole face by necessity (a triangulator that merges equal
//! positions, `mesh::planar::Pinches`) keeps its own reading.

use crate::entity::{EntityId, HalfEdgeKey, VertexKey};
use crate::geometry::PointKey;
use crate::live::{CycleLoop, linked, proven};
use geom_core::Real;

/// **One loop's corners, grouped by point key**: the loop's cycle, and
/// each point key it passes at two or more corners.
#[derive(Clone, Debug)]
pub struct LoopCorners {
    cycle: Vec<HalfEdgeKey>,
    groups: Vec<PointCorners>,
}

/// The corners of one loop at one point key: two or more.
#[derive(Clone, Debug)]
pub struct PointCorners {
    point: PointKey,
    /// Indices into the loop's cycle, ascending.
    corners: Vec<usize>,
    /// The vertex each corner leaves from, parallel to `corners`.
    vertices: Vec<VertexKey>,
}

impl LoopCorners {
    /// The loop's half-edges in cycle order: a corner is an index into
    /// it.
    #[must_use]
    pub fn cycle(&self) -> &[HalfEdgeKey] {
        &self.cycle
    }

    /// Each point key the loop passes at two or more corners, in point
    /// key order.
    #[must_use]
    pub fn groups(&self) -> &[PointCorners] {
        &self.groups
    }

    /// Whether the loop pinches: some group holds two distinct vertices.
    #[must_use]
    pub fn pinches(&self) -> bool {
        self.groups.iter().any(PointCorners::pinches)
    }
}

impl PointCorners {
    /// The point key the corners share.
    #[must_use]
    pub fn point(&self) -> PointKey {
        self.point
    }

    /// The corners, as indices into [`LoopCorners::cycle`], ascending.
    #[must_use]
    pub fn corners(&self) -> &[usize] {
        &self.corners
    }

    /// The vertex each corner leaves from, parallel to
    /// [`Self::corners`].
    #[must_use]
    pub fn vertices(&self) -> &[VertexKey] {
        &self.vertices
    }

    /// Whether the corners stand at two distinct vertices: a pinch, and
    /// not one vertex met twice.
    #[must_use]
    pub fn pinches(&self) -> bool {
        self.vertices.iter().any(|&v| v != self.vertices[0])
    }
}

/// **A loop's corners, grouped by point key** ([`LoopCorners`]). A
/// lone-vertex loop has none, and is no [`CycleLoop`]; the handle's
/// proof is that every corner's vertex resolves.
#[must_use]
pub fn loop_corners<T: Real>(lp: CycleLoop<'_, T>) -> LoopCorners {
    let body = lp.body();
    let cycle = lp.cycle();
    let mut keyed: Vec<(PointKey, usize, VertexKey)> = cycle
        .iter()
        .enumerate()
        .map(|(i, &he)| {
            let vertex = proven(&body.half_edges, he, EntityId::HalfEdge).start;
            let point = linked(
                &body.vertices,
                vertex,
                EntityId::Vertex,
                EntityId::HalfEdge(he),
                "start",
            )
            .point;
            (point, i, vertex)
        })
        .collect();
    keyed.sort_unstable_by_key(|&(point, i, _)| (point, i));
    let groups = keyed
        .chunk_by(|x, y| x.0 == y.0)
        .filter(|group| group.len() >= 2)
        .map(|group| PointCorners {
            point: group[0].0,
            corners: group.iter().map(|&(_, i, _)| i).collect(),
            vertices: group.iter().map(|&(_, _, v)| v).collect(),
        })
        .collect();
    LoopCorners { cycle, groups }
}

/// **Whether a loop pinches**: passes one point key at two distinct
/// vertices ([`LoopCorners::pinches`]).
#[must_use]
pub fn loop_pinches<T: Real>(lp: CycleLoop<'_, T>) -> bool {
    loop_corners(lp).pinches()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::construction::MevSite;
    use crate::test_support_fixtures::prism;
    use geom_core::{Point3, Tol};

    /// A square with a notch whose two lips meet at `(0, 0)`: profile
    /// corners 2 and 5 stand at one position.
    const NOTCHED: [(f64, f64); 10] = [
        (-3.0, -3.0),
        (-1.0, -3.0),
        (0.0, 0.0),
        (-1.0, 2.0),
        (1.0, 2.0),
        (0.0, 0.0),
        (1.0, -3.0),
        (3.0, -3.0),
        (3.0, 3.0),
        (-3.0, 3.0),
    ];

    /// The bottom cap's reading, with corners 2 and 5 of each cap on one
    /// point key where `one_key`, else on the two the prism minted.
    fn notched_bottom(one_key: bool) -> (LoopCorners, [VertexKey; 2]) {
        let p = prism::<f64>(&NOTCHED, 1.0, Tol::witness());
        let mut body = p.body;
        if one_key {
            let keep = body.vertices[p.bottom[2]].point;
            body.vertices.get_mut(p.bottom[5]).unwrap().point = keep;
        }
        let outer = body.faces[p.bottom_face].outer;
        let corners = loop_corners(body.cycle_loop(outer).expect("a cycle"));
        (corners, [p.bottom[2], p.bottom[5]])
    }

    #[test]
    fn two_vertices_on_one_point_key_pinch() {
        let (corners, pinch) = notched_bottom(true);
        assert_eq!(
            corners.groups().len(),
            1,
            "one point passed twice: {corners:?}"
        );
        let group = &corners.groups()[0];
        let mut vertices = group.vertices().to_vec();
        vertices.sort_unstable();
        let mut expected = pinch.to_vec();
        expected.sort_unstable();
        assert_eq!(
            vertices, expected,
            "the group's vertices are the pinch's two"
        );
        assert!(
            group.pinches() && corners.pinches(),
            "two distinct vertices pinch"
        );
    }

    /// The point key is read, not the position: two vertices at one
    /// position on two keys group nothing.
    #[test]
    fn two_point_keys_at_one_position_do_not_group() {
        let (corners, _) = notched_bottom(false);
        assert!(
            corners.groups().is_empty(),
            "two keys, no group: {corners:?}"
        );
        assert!(!corners.pinches(), "two keys, no pinch");
    }

    /// A strut walked out and back passes its root vertex at two
    /// corners: a group, and no pinch.
    #[test]
    fn one_vertex_met_twice_groups_and_does_not_pinch() {
        let tol = Tol::witness();
        let p = prism::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 1.0, tol);
        let mut body = p.body;
        let outer = body.faces[p.bottom_face].outer;
        let he = body.loop_halves(outer).expect("the cap's cycle")[0];
        let root = body.half_edges[he].start;
        body.mev_line(
            MevSite::Fan { he1: he, he2: he },
            Point3::new(1.0, 1.0, 0.0),
            tol,
        )
        .expect("a strut into the cap");
        let corners = loop_corners(body.cycle_loop(outer).expect("a cycle"));
        assert_eq!(
            corners.cycle().len(),
            6,
            "four sides and the strut both ways"
        );
        assert_eq!(
            corners.groups().len(),
            1,
            "the root passed twice: {corners:?}"
        );
        assert_eq!(corners.groups()[0].vertices(), [root, root]);
        assert!(!corners.pinches(), "one vertex met twice is no pinch");
    }

    /// A loop key that misses, a cycle that does not walk and a corner
    /// whose vertex is dead are each named at the door that hands out a
    /// [`CycleLoop`] ([`Body::cycle_loop`](crate::Body::cycle_loop)),
    /// never met inside the corners' walk.
    #[test]
    fn a_miss_is_named_at_the_door() {
        use crate::NoCycle;
        let fresh = || {
            let p = prism::<f64>(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], 1.0, Tol::witness());
            let outer = p.body.faces[p.bottom_face].outer;
            let cycle = p.body.loop_halves(outer).expect("the cap's cycle");
            (p.body, outer, cycle)
        };
        let (mut body, outer, _) = fresh();
        body.loops.remove(outer);
        assert_eq!(
            body.cycle_loop(outer).err(),
            Some(NoCycle::Stale),
            "a dead loop"
        );
        let (mut body, outer, cycle) = fresh();
        let vertex = body.half_edges[cycle[1]].start;
        body.vertices.remove(vertex);
        assert_eq!(
            body.cycle_loop(outer).err(),
            Some(NoCycle::Torn),
            "a dead corner vertex"
        );
        let (mut body, outer, cycle) = fresh();
        body.half_edges.remove(cycle[1]);
        assert_eq!(
            body.cycle_loop(outer).err(),
            Some(NoCycle::Torn),
            "a torn cycle"
        );
        let (mut body, outer, cycle) = fresh();
        let other = body
            .faces
            .iter()
            .map(|(_, f)| f.outer)
            .find(|&l| l != outer)
            .unwrap();
        body.half_edges.get_mut(cycle[1]).unwrap().parent_loop = other;
        assert_eq!(
            body.cycle_loop(outer).err(),
            Some(NoCycle::Torn),
            "a member that claims another loop"
        );
    }
}
