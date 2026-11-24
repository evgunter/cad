//! **The faces' extent an edge's wedge is levered at** — the one home
//! of the `reach` of the dihedral between an edge's two faces. Tier 3's
//! check 4, certification of an `Intersection` or `TangentIntersection`,
//! and the must-carry rule wherever a door decides what a join stores
//! pass it to [`geom_brep::folded_lever_arm`]. The embedding question
//! (`crate::injective`'s `arm_of`) levers at it alone, with no fold
//! (`work/embed/the-departure-is-levered-at-the-reach-alone-where-check-4-folds-the-faces-curvature-arms.md`).
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
//! the edge (`work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`;
//! the edge's own extent read it the same way). A face is read at its
//! boundary's vertices, so a short
//! arc of a large circle reads the arc's ends and not the circle, and a
//! lune of a large sphere its own extent, not the sphere's diameter.
//! That falls short of the boundary by at most a curved edge's bulge
//! past its ends, and of a curved face that stands farther inside its
//! boundary than on it, both the safe direction; for the readers that
//! fold, the curvature arm ([`geom_brep::folded_lever_arm`]) caps the
//! lever at the face's radius in any case. The embedding question does
//! not fold, so nothing caps its lever there. The edge's own ends are read too, for a door
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
use crate::live::BoundaryMember;

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
) -> geom_core::Lever<T> {
    WedgeReach::new(body).reach(faces, carrier, interval)
}

/// **The reach's one recipe**: the farthest of `faces`, the two faces'
/// boundary vertices (module docs), and the edge's own ends from the
/// carrier's middle over `(t0, t1)`. The ends are read for a door
/// minting the edge into a face it does not bound yet. Every reader of
/// an edge's reach builds it here, at whichever scalar it holds the
/// carrier and the points: [`WedgeReach`] off the body, the embedding
/// question off its lifted boundaries (`crate::injective`).
pub(crate) fn reach_over<T: geom_core::spline::SpanLocate>(
    carrier: &Curve3<T>,
    (t0, t1): (T, T),
    faces: impl IntoIterator<Item = Point3<T>>,
) -> geom_core::Lever<T> {
    reach_from_vertex(
        carrier.mid_point(t0, t1),
        [carrier.eval(t0), carrier.eval(t1)]
            .into_iter()
            .chain(faces),
    )
}

/// **The reach of two faces from a vertex**: the farthest of `faces`,
/// both faces' boundary vertices, from `vertex`. [`reach_over`] reads an
/// edge's from its middle through this; two faces that share a vertex
/// and no edge on it are read from that vertex, there being no edge
/// whose middle to read from: each face ε off over its run from the
/// vertex turns by ε over that run there, so how the two leave the
/// vertex is levered at the farther face's run.
///
/// It reads vertices alone, so a face bounded by a closed edge on that
/// one vertex (a disc's rim) adds nothing, and two such faces read a
/// zero arm. A margin levered at a zero or undecided arm is not
/// definite, so the reader refuses; no default stands in.
///
/// It hands its caller's points to the points' door, as
/// [`WedgeReach::reach`] always has: the reach is not yet read off the
/// faces' covers
/// (`work/oneround/lever-doors-pinned-sites-await-a-geometry-read.md`).
pub(crate) fn reach_from_vertex<T: geom_core::Real>(
    vertex: Point3<T>,
    faces: impl IntoIterator<Item = Point3<T>>,
) -> geom_core::Lever<T> {
    geom_brep::points_lever_from(vertex, faces)
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
pub fn edge_wedge_reach<T: Decide>(body: &Body<T>, edge: EdgeKey) -> Option<geom_core::Lever<T>> {
    WedgeReach::new(body).edge(edge)
}

/// **[`wedge_reach`] over one body, each face's boundary points
/// collected once** and kept for every later edge the face bounds. The
/// body is borrowed, so it cannot change under the cache.
pub struct WedgeReach<'a, T: Decide> {
    body: &'a Body<T>,
    points: RefCell<SecondaryMap<FaceKey, Rc<[Point3<T>]>>>,
    stations: RefCell<SecondaryMap<FaceKey, Rc<geom_brep::BoundaryStations<T>>>>,
}

impl<'a, T: Decide> WedgeReach<'a, T> {
    /// A reader of `body`'s wedges, nothing collected yet.
    #[must_use]
    pub fn new(body: &'a Body<T>) -> Self {
        Self {
            body,
            points: RefCell::new(SecondaryMap::new()),
            stations: RefCell::new(SecondaryMap::new()),
        }
    }

    /// **How far `face` runs from the edge on `carrier` over `span`**,
    /// measured across it and from below
    /// ([`geom_brep::BoundaryStations::run_from_edge`]): the extent the
    /// embedding question levers a pair's parting at is the lesser of
    /// the two faces' runs. Not the reach above, which is the farther
    /// face's, from the edge's middle.
    ///
    /// # Panics
    ///
    /// As [`wedge_reach`].
    #[must_use]
    pub fn run_from_edge(
        &self,
        face: FaceKey,
        carrier: &Curve3<T>,
        span: (T, T),
    ) -> geom_core::Lever<T> {
        self.face_stations(face).run_from_edge(carrier, span)
    }

    /// **How far `face` runs from `vertex`**, from below
    /// ([`geom_brep::BoundaryStations::run_from_vertex`]).
    ///
    /// # Panics
    ///
    /// As [`wedge_reach`].
    #[must_use]
    pub fn run_from_vertex(&self, face: FaceKey, vertex: Point3<T>) -> geom_core::Lever<T> {
        self.face_stations(face).run_from_vertex(vertex)
    }

    /// `face`'s boundary stations: each certified edge of its loops
    /// once, and each lone vertex. Collected on first ask.
    fn face_stations(&self, face: FaceKey) -> Rc<geom_brep::BoundaryStations<T>> {
        if let Some(stations) = self.stations.borrow().get(face) {
            return Rc::clone(stations);
        }
        let body = self.body;
        let view = body.proven_face(face);
        let (mut edges, mut lone) = (std::collections::BTreeMap::new(), Vec::new());
        for (_, members, _) in view.boundary_by_loop() {
            for member in members {
                match member {
                    BoundaryMember::Isolated { point, .. } => lone.push(point),
                    BoundaryMember::Edge { edge, .. } => {
                        edges.insert(edge.key(), edge);
                    }
                }
            }
        }
        let stations = Rc::new(geom_brep::BoundaryStations::of_boundary(
            edges.into_values().filter_map(|edge| {
                let curve = edge.certified()?;
                Some((curve.carrier(), curve.params()))
            }),
            lone,
        ));
        self.stations
            .borrow_mut()
            .insert(face, Rc::clone(&stations));
        stations
    }

    /// [`wedge_reach`] of the edge on `carrier` over `(t0, t1)` between
    /// `faces`.
    ///
    /// # Panics
    ///
    /// As [`wedge_reach`].
    #[must_use]
    pub fn reach(
        &self,
        faces: [FaceKey; 2],
        carrier: &Curve3<T>,
        (t0, t1): (T, T),
    ) -> geom_core::Lever<T> {
        let [a, b] = faces;
        let (pa, pb) = (self.face_points(a), (b != a).then(|| self.face_points(b)));
        reach_over(
            carrier,
            (t0, t1),
            pa.iter()
                .copied()
                .chain(pb.iter().flat_map(|p| p.iter().copied())),
        )
    }

    /// [`edge_wedge_reach`], reading faces through this cache.
    ///
    /// # Panics
    ///
    /// As [`edge_wedge_reach`].
    #[must_use]
    pub fn edge(&self, edge: EdgeKey) -> Option<geom_core::Lever<T>> {
        let curve = self.body.proven_edge(edge).certified()?;
        Some(self.reach(edge_faces(self.body, edge), curve.carrier(), curve.params()))
    }

    /// `face`'s boundary points (module docs): every loop's vertices, a
    /// lone vertex included. Collected on first ask.
    fn face_points(&self, face: FaceKey) -> Rc<[Point3<T>]> {
        if let Some(points) = self.points.borrow().get(face) {
            return Rc::clone(points);
        }
        let body = self.body;
        let view = body.proven_face(face);
        let mut points = Vec::new();
        for (_, members, _) in view.boundary_by_loop() {
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
