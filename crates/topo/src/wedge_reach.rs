//! **The faces' extent an edge's wedge is levered at** — the one home
//! of the `reach` every reader of the dihedral between an edge's two
//! faces passes to [`geom_brep::folded_lever_arm`]: tier 3's check 4,
//! certification of an `Intersection` or `TangentIntersection`, and the
//! must-carry rule wherever a door decides what a join stores.
//!
//! A dihedral is how far the two faces separate as they leave the edge,
//! so it is levered at how far they run (D4 ¶1's face extent) and how
//! they curve (the fold's curvature arms), never at how long the edge
//! is: an edge's own extent reads a short arc between large faces at
//! dihedral δ as a wedge of δ², where the faces open one of δ. The reach
//! is the farther face's: how far either face's boundary runs from the
//! edge's middle, so two small faces at a small angle read a small
//! wedge, and one large face beside a small one reads the gap the large
//! one opens.
//!
//! **The measure never over-states the faces' boundaries**, the unsafe
//! direction for a lever, which would read a corner more definitely
//! than the faces support. One over-statement remains: the reach is
//! measured from the edge's middle, so a face's run ALONG the edge
//! counts as lever, though two faces separate only with distance from
//! the edge (`work/twice/the-wedge-reach-counts-a-faces-run-along-its-edge.md`;
//! the edge's own extent read it the same way). A face is read at its
//! boundary's vertices, so a short
//! arc of a large circle reads the arc's ends and not the circle, and a
//! lune of a large sphere its own extent, not the sphere's diameter.
//! That falls short of the boundary by at most a curved edge's bulge
//! past its ends, and of a curved face that stands farther inside its
//! boundary than on it, both the safe direction; the curvature arm
//! ([`geom_brep::folded_lever_arm`]) caps the lever at the face's
//! radius in any case. The edge's own ends are read too, for a door
//! minting it into a face it does not bound yet. A face's vertices do
//! not depend on the edge being read, so [`WedgeReach`] collects them
//! once per face and a pass over many edges (tier 3) reads each face's
//! boundary once; and they are points, not the trigonometric forms a
//! curved edge's interior point would be, which the symbolic tier pays
//! for at every margin the lever enters.

use std::cell::RefCell;
use std::rc::Rc;

use geom::Curve3;
use geom_core::{Decide, Point3};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey};
use crate::live::{BoundaryMember, proven};

/// The reach of the wedge between `faces` along the edge on `carrier`
/// over `(t0, t1)`: the farthest either face's boundary vertices (module
/// docs) and the edge's own ends stand from the carrier's middle. At
/// rest the edge bounds both faces already; a door minting the edge
/// names the faces it reads (both one face, where the edge is a strut
/// in it or a chord dividing it). [`WedgeReach`] is the same reading
/// with each face's points collected once, for a caller reading many
/// edges of one body.
///
/// # Panics
///
/// Where a face, or a record its boundary walk reaches, does not
/// resolve: every caller read `faces` out of `body`.
#[must_use]
pub fn wedge_reach<T: Decide>(
    body: &Body<T>,
    faces: [FaceKey; 2],
    carrier: &Curve3<T>,
    interval: (T, T),
) -> T {
    WedgeReach::new(body).reach(faces, carrier, interval)
}

/// [`wedge_reach`] of a live `edge`, over its two faces and its stored
/// carrier; `None` where the edge carries no certified curve (null
/// scaffolding, mid-op).
///
/// # Panics
///
/// Where `edge` or a record on the walk from it does not resolve: every
/// caller read `edge` out of `body`.
#[must_use]
pub fn edge_wedge_reach<T: Decide>(body: &Body<T>, edge: EdgeKey) -> Option<T> {
    WedgeReach::new(body).edge(edge)
}

/// **[`wedge_reach`] over one body, each face's boundary points
/// collected once** and kept for every later edge the face bounds. The
/// body is borrowed, so it cannot change under the cache.
pub struct WedgeReach<'a, T: Decide> {
    body: &'a Body<T>,
    points: RefCell<SecondaryMap<FaceKey, Rc<[Point3<T>]>>>,
}

impl<'a, T: Decide> WedgeReach<'a, T> {
    /// A reader of `body`'s wedges, nothing collected yet.
    #[must_use]
    pub fn new(body: &'a Body<T>) -> Self {
        Self {
            body,
            points: RefCell::new(SecondaryMap::new()),
        }
    }

    /// [`wedge_reach`] of the edge on `carrier` over `(t0, t1)` between
    /// `faces`.
    ///
    /// # Panics
    ///
    /// As [`wedge_reach`].
    #[must_use]
    pub fn reach(&self, faces: [FaceKey; 2], carrier: &Curve3<T>, (t0, t1): (T, T)) -> T {
        let at: Point3<T> = carrier.mid_point(t0, t1);
        // `norm_squared` squares each coordinate (`powi(2)`), which an
        // enclosure never reads below zero, where `d.dot(d)` would.
        let square = |p: Point3<T>| (p - at).norm_squared();
        // The edge's own ends: a door minting the edge reads a face it
        // does not bound yet. Its middle is `at`.
        let mut far = square(carrier.eval(t0)).max(square(carrier.eval(t1)));
        let [a, b] = faces;
        let mut fold = |face| {
            for &p in self.face_points(face).iter() {
                far = far.max(square(p));
            }
        };
        fold(a);
        if b != a {
            fold(b);
        }
        far.sqrt()
    }

    /// [`edge_wedge_reach`], reading faces through this cache.
    ///
    /// # Panics
    ///
    /// As [`edge_wedge_reach`].
    #[must_use]
    pub fn edge(&self, edge: EdgeKey) -> Option<T> {
        let data = &self.body.edges[edge];
        let curve = self.body.curves.get(data.curve)?.certified()?;
        Some(self.reach(edge_faces(self.body, edge), curve.carrier(), curve.params()))
    }

    /// `face`'s boundary points (module docs): every loop's vertices, a
    /// lone vertex included. Collected on first ask.
    fn face_points(&self, face: FaceKey) -> Rc<[Point3<T>]> {
        if let Some(points) = self.points.borrow().get(face) {
            return Rc::clone(points);
        }
        let body = self.body;
        let data = proven(&body.faces, face, EntityId::Face);
        let mut points = Vec::new();
        for (_, members) in body.face_boundary_by_loop(face, data) {
            for member in members {
                match member {
                    BoundaryMember::Isolated { point, .. } => points.push(point),
                    BoundaryMember::Edge { he, half, .. } => {
                        points.push(body.linked_vertex_point(
                            half.start,
                            EntityId::HalfEdge(he),
                            "start",
                        ));
                    }
                }
            }
        }
        let points: Rc<[Point3<T>]> = points.into();
        self.points.borrow_mut().insert(face, Rc::clone(&points));
        points
    }
}

/// `edge`'s two faces, `he_plus`'s first.
///
/// # Panics
///
/// As [`edge_wedge_reach`].
#[must_use]
pub fn edge_faces<T: geom_core::Real>(body: &Body<T>, edge: EdgeKey) -> [FaceKey; 2] {
    let data = &body.edges[edge];
    [data.he_plus, data.he_minus].map(|he| {
        body.face_of_half_edge(he)
            .unwrap_or_else(|| unreachable!("{he:?}, a half of live {edge:?}, has no face"))
    })
}
