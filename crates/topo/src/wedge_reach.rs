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
//! **The reach is known from both sides** ([`geom_brep::LeverBounds`]),
//! each read from the carrier's middle, and each caller takes the bound
//! its action needs (`crates/geom-core/README.md`, "Float rounding", the
//! lever bullet). From below: the faces' boundary vertices and the
//! edge's own ends, each a point of the region, so a short arc of a
//! large circle reads the arc's ends and not the circle. From above:
//! the faces' covers ([`crate::cover::face_cover`]), which hold what
//! bulges past the boundary (a curved edge's bulge, a sphere's far
//! side), and the edge's own cover over its span, for a door minting it
//! into a face it does not bound yet; a face with no cover reads poison
//! there, which a reading levered at it escalates. The reach is measured
//! from the edge's middle, so a face's run ALONG the edge counts as
//! lever, though two faces separate only with distance from the edge
//! (`work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`).
//! A face's vertices and cover do not depend on the edge being read, so
//! [`WedgeReach`] reads them once per face and a pass over many edges
//! (tier 3) reads each face once; the vertices are points, not the
//! trigonometric forms a curved edge's interior point would be, which
//! the symbolic tier pays for at every margin the lever enters.

use std::cell::RefCell;
use std::rc::Rc;

use geom::Curve3;
use geom::cover::Cover;
use geom_brep::{ExtentBall, LeverBounds};
use geom_core::{Band, Decide, Point3};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::cover::FaceCovered;
use crate::entity::{EdgeKey, EntityId, FaceKey};
use crate::live::BoundaryMember;

/// The reach of the wedge between `faces` along the edge on `carrier`
/// over `(t0, t1)`, from the carrier's middle and bounded from both
/// sides (module docs): from below the farthest of the faces' boundary
/// vertices and the edge's own ends, from above the farthest of the
/// faces' covers ([`crate::cover::face_cover`]) and the edge's own
/// cover over the span. At rest the edge bounds both faces already; a
/// door minting the edge names the faces it reads (both one face, where
/// the edge is a strut in it or a chord dividing it). [`WedgeReach`] is
/// the same reading with each face's points and cover read once, for a
/// caller reading many edges of one body.
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
    band: Band,
) -> LeverBounds<T> {
    WedgeReach::new(body, band).reach(faces, carrier, interval)
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
pub fn edge_wedge_reach<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
) -> Option<LeverBounds<T>> {
    WedgeReach::new(body, band).edge(edge)
}

/// **[`wedge_reach`] over one body, each face's boundary points and
/// cover read once** and kept for every later edge the face bounds. The
/// body is borrowed, so it cannot change under the cache.
pub struct WedgeReach<'a, T: Decide> {
    body: &'a Body<T>,
    band: Band,
    /// Whether a face's cover reads its lift ([`crate::cover::face_cover`])
    /// or not ([`crate::cover::face_cover_unlifted`]).
    lifts: bool,
    faces: RefCell<SecondaryMap<FaceKey, Rc<FaceRead<T>>>>,
}

/// What a face contributes to every wedge it bounds: its boundary's
/// vertices, points of the face, and its cover, which holds it.
struct FaceRead<T: geom_core::Real> {
    points: Vec<Point3<T>>,
    cover: FaceCovered<T>,
}

impl<'a, T: Decide> WedgeReach<'a, T> {
    /// A reader of `body`'s wedges, its faces' covers read under
    /// `band`; nothing read yet.
    #[must_use]
    pub fn new(body: &'a Body<T>, band: Band) -> Self {
        Self {
            body,
            band,
            lifts: true,
            faces: RefCell::new(SecondaryMap::new()),
        }
    }

    /// [`Self::new`] for the turns a lift decides: each face's cover read
    /// without its lift ([`crate::cover::face_cover_unlifted`]), since
    /// a face's cover can ask that lift.
    #[must_use]
    pub(crate) fn unlifted(body: &'a Body<T>, band: Band) -> Self {
        Self {
            lifts: false,
            ..Self::new(body, band)
        }
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
    ) -> LeverBounds<T> {
        let at: Point3<T> = carrier.mid_point(t0, t1);
        let ends = (carrier.eval(t0), carrier.eval(t1));
        let [a, b] = faces;
        let reads = [Some(self.face(a)), (b != a).then(|| self.face(b))];
        let reads = reads.iter().flatten();
        let points = [ends.0, ends.1]
            .into_iter()
            .chain(reads.clone().flat_map(|f| f.points.iter().copied()))
            .collect();
        let far = |cover: Option<&Cover<T>>| ExtentBall::of_cover_from(cover, at).lever();
        let own = crate::cover::edge_cover_over(carrier, (t0, t1), ends).ok();
        // The points bound the region from above too: mid-door a face's
        // vertex can stand off its carrier, past the cover its carrier
        // trims.
        let below = far(Cover::points(points).as_ref());
        LeverBounds::Between {
            below,
            above: reads.fold(far(own.as_ref()).max(below), |above, f| {
                above.max(far(f.cover.as_ref().ok()))
            }),
        }
    }

    /// [`edge_wedge_reach`], reading faces through this cache.
    ///
    /// # Panics
    ///
    /// As [`edge_wedge_reach`].
    #[must_use]
    pub fn edge(&self, edge: EdgeKey) -> Option<LeverBounds<T>> {
        let data = &self.body.edges[edge];
        let curve = self.body.curves.get(data.curve)?.certified()?;
        Some(self.reach(edge_faces(self.body, edge), curve.carrier(), curve.params()))
    }

    /// `face`'s read (module docs): every loop's vertices, a lone vertex
    /// included, and its cover. Read on first ask.
    fn face(&self, face: FaceKey) -> Rc<FaceRead<T>> {
        if let Some(read) = self.faces.borrow().get(face) {
            return Rc::clone(read);
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
        let read = Rc::new(FaceRead {
            points,
            cover: if self.lifts {
                crate::cover::face_cover(body, face, self.band)
            } else {
                crate::cover::face_cover_unlifted(body, face, self.band)
            },
        });
        self.faces.borrow_mut().insert(face, Rc::clone(&read));
        read
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
