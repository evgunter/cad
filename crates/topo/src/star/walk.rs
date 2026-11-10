//! The star's walk: a vertex's bounds and the sectors between them, read
//! off the body with no side (the module docs of [`crate::star`],
//! "The walk").

use geom_brep::OutwardNormal;
use geom_brep::sector_shape::{
    CornerShape, SectorFault, SectorShape, TurnOrder, corner_shape, metering_arm,
};
use geom_core::{Band, Decide, Indeterminate, Lever, Margin, Real, Sign, Vec3};

use crate::body::Body;
use crate::bound_side::{BoundSupport, ConicArc, FaceBend};
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::live::{Proven, linked, proven};
use crate::sector_face::{ConeApex, SectorCarrier, SectorFaceError, VertexFace};
use crate::sector_shape::{SlitTips, apex_corner_shape, apex_extrema_inside, apex_over_half_turn};
use crate::validate::decide;

/// One bound out of the star's vertex: an edge of its orbit, or a
/// divider the walk cut a sector at.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Bound<T: Real> {
    /// The orbit half-edge whose corner the bound lies in: its own edge,
    /// or the edge a divider's corner follows.
    pub he: HalfEdgeKey,
    /// Whether the bound is the edge of `he`, not a divider.
    pub real: bool,
    /// The unit direction the bound leaves the vertex along: a line's
    /// chord, a curved edge's tangent there, a divider's direction.
    pub dir: Vec3<T>,
    /// What stands behind the direction: how its side of a datum is read
    /// ([`crate::bound_side`]).
    pub reach: BoundSupport<T>,
}

impl<T: Real> Bound<T> {
    /// The direction scaled to the bound's reach.
    pub(crate) fn ray(&self) -> Vec3<T> {
        self.dir * self.reach.lever().length()
    }
}

/// What a sector's face is at the star's vertex: a tangent plane, or a
/// cone's apex, whose directions there are its rulings. A vertex within
/// the band of a cone's axis but off its apex has neither decided, and
/// the walk refuses it ([`StarFault::NearAxis`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum StarFace<T: Real> {
    /// The face's outward unit normal at the vertex.
    Tangent(OutwardNormal<T>),
    /// The vertex is the apex of the face's cone.
    Apex(ConeApex<T>),
}

impl<T: Decide> StarFace<T> {
    /// The face's outward normal along the direction `d` out of the
    /// vertex: its tangent plane's, or at a cone's apex the normal along
    /// the ruling `d` lies on, which is constant along it
    /// ([`ConeApex::normal_along`]).
    pub(crate) fn normal_along(&self, d: Vec3<T>) -> OutwardNormal<T> {
        match self {
            Self::Tangent(normal) => *normal,
            Self::Apex(apex) => apex.normal_along(d),
        }
    }
}

#[cfg(test)]
impl<T: Real> StarFace<T> {
    /// The tangent plane's normal; a row that reads an apex panics.
    #[allow(clippy::panic)]
    pub(crate) fn tangent(&self) -> Vec3<T> {
        match self {
            Self::Tangent(normal) => normal.vec(),
            Self::Apex(apex) => panic!("a tangent plane, not {apex:?}"),
        }
    }
}

/// The sector between two consecutive bounds of a star.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Sector<T: Real> {
    /// The sector's face.
    pub face: FaceKey,
    /// The arm the shared walk read the face by.
    pub carrier: SectorCarrier,
    /// What the face is at the vertex.
    pub at: StarFace<T>,
    /// How the face bends at the vertex.
    pub bend: FaceBend<T>,
    /// The metering arm: the shorter chord of the corner the sector is
    /// part of, in metres.
    pub arm: Lever<T>,
}

/// **A vertex's star**: its bounds in orbit order, and the sector after
/// each. Sector `k` is the corner between `bounds[k]`, its CCW-last
/// bound (its own, the end), and `bounds[k + 1]`, its CCW-first (the
/// start), cyclically; every sector is convex in its own geometry and,
/// once refined ([`Star::refine`]), every datum's side is monotone
/// across it.
#[derive(Clone, Debug)]
pub(crate) struct Star<T: Real> {
    /// The vertex.
    pub vertex: VertexKey,
    /// The bounds, in orbit order.
    pub bounds: Vec<Bound<T>>,
    /// `sectors[k]` follows `bounds[k]`.
    pub sectors: Vec<Sector<T>>,
}

/// A datum the star is refined against: a surface through the vertex
/// whose side a verb reads off the star's bounds.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Datum<T: Real> {
    /// A plane through the vertex, by its unit normal: a splitting plane,
    /// a pierced face's tangent plane, a partner's.
    Plane(Vec3<T>),
    /// A partner's cone with its apex at the vertex.
    Cone(ConeApex<T>),
}

/// Why the walk read no star at a vertex.
#[derive(Clone, Copy, Debug)]
pub(crate) enum StarFault {
    /// The vertex has no edges.
    Lone,
    /// An orbit edge is a null edge: no curve, no direction.
    NullEdge(EdgeKey),
    /// The shared walk did not resolve a sector's face.
    Face(SectorFaceError),
    /// The vertex lies within the band of a cone face's axis but off its
    /// apex: neither the tangent plane nor the apex is decided there.
    NearAxis(FaceKey),
    /// A corner's shape refused at its rungs.
    Shape(FaceKey, SectorFault),
    /// Whether a datum's extremum lies inside a sector is in the band.
    Cut(FaceKey, Indeterminate),
}

impl<T: Real> Star<T> {
    /// Sector `k`'s CCW-first bound, its start.
    pub(crate) fn start(&self, k: usize) -> &Bound<T> {
        &self.bounds[(k + 1) % self.bounds.len()]
    }
}

impl<T: Decide> Star<T> {
    /// **The star of `vertex`, convex in its own geometry** (module docs
    /// of [`crate::star`], "The walk"). `tips` are the body's slit tips,
    /// read at rest.
    ///
    /// # Errors
    ///
    /// [`StarFault`].
    pub(crate) fn walk(
        body: &Body<T>,
        vertex: VertexKey,
        tips: &SlitTips,
        band: Band,
    ) -> Result<Self, StarFault> {
        let orbit = body.vertex_orbit_linked(vertex);
        if orbit.is_empty() {
            return Err(StarFault::Lone);
        }
        let p = body.resolve_vertex_point(vertex, Proven);
        let n = orbit.len();
        let departures = orbit
            .iter()
            .map(|&he| departure(body, vertex, he))
            .collect::<Result<Vec<_>, _>>()?;
        let mut star = Self {
            vertex,
            bounds: Vec::with_capacity(n + 2),
            sectors: Vec::with_capacity(n + 2),
        };
        for i in 0..n {
            let he = orbit[i];
            let (own, own_reach) = departures[i];
            let (next, _) = departures[(i + 1) % n];
            let resolved =
                crate::sector_face::resolve(body, vertex, he, band).map_err(StarFault::Face)?;
            let face = resolved.face;
            let view = body.proven_face(face);
            let bend = FaceBend::at(view.surface(), view.sense(), p);
            let bounds = tips.bounds(vertex, face, n == 1);
            let shape = |fault| StarFault::Shape(face, fault);
            let sector = |at, arm| Sector {
                face,
                carrier: resolved.carrier,
                at,
                bend,
                arm,
            };
            let mut push = |dir: Vec3<T>, reach, real, at, arm| {
                star.bounds.push(Bound {
                    he,
                    real,
                    dir,
                    reach,
                });
                star.sectors.push(sector(at, arm));
            };
            match resolved.at_vertex {
                VertexFace::NearAxis(_) => return Err(StarFault::NearAxis(face)),
                VertexFace::Tangent(normal) => {
                    let at = StarFace::Tangent(normal);
                    match corner_shape(own, next, normal, bounds, band).map_err(shape)? {
                        CornerShape::Sector(SectorShape {
                            arm,
                            unit_own,
                            bisector,
                            ..
                        }) => {
                            let arm = metering_arm(arm);
                            push(unit_own, own_reach, true, at, arm);
                            if let Some(b) = bisector {
                                let b = b.normalize();
                                let reach = BoundSupport::bisector(arm, b, normal.vec(), bend);
                                push(b, reach, false, at, arm);
                            }
                        }
                        CornerShape::FullTurn(turn) => {
                            let arm = metering_arm(turn.arm);
                            push(turn.unit_own, own_reach, true, at, arm);
                            for q in turn.quarters(TurnOrder::FromOwn) {
                                let q = q.normalize();
                                let reach = BoundSupport::bisector(arm, q, normal.vec(), bend);
                                push(q, reach, false, at, arm);
                            }
                        }
                    }
                }
                VertexFace::Apex(apex) => {
                    let at = StarFace::Apex(apex);
                    let read =
                        apex_corner_shape((own, next), &apex, bounds, band).map_err(shape)?;
                    let unit_next = next.normalize();
                    // A whole turn is quartered, a window over a
                    // half-turn halved; the fractions run from the own
                    // bound.
                    let fractions: &[f64] = if read.whole_turn {
                        &[0.75, 0.5, 0.25]
                    } else if apex_over_half_turn(&apex, read.window, read.arm, band)
                        .map_err(shape)?
                    {
                        &[0.5]
                    } else {
                        &[]
                    };
                    let arm = metering_arm(read.arm);
                    push(own.normalize(), own_reach, true, at, arm);
                    for &k in fractions {
                        let r = apex.ruling_from(unit_next, read.window * T::from_f64(k));
                        push(r, BoundSupport::straight_bisector(arm), false, at, arm);
                    }
                }
            }
        }
        Ok(star)
    }

    /// **The star refined against `datums`** (module docs of
    /// [`crate::star`], "Refinement"): each sector is cut at every
    /// datum's extremum strictly inside it, so that every datum's side
    /// is monotone across every sector.
    ///
    /// # Errors
    ///
    /// [`StarFault::Shape`] or [`StarFault::Cut`] where whether an
    /// extremum lies inside a sector is in the band.
    pub(crate) fn refine(&mut self, datums: &[Datum<T>], band: Band) -> Result<(), StarFault> {
        let mut k = 0;
        while k < self.sectors.len() {
            let mut cut = None;
            for datum in datums {
                if let Some(dir) = self.extremum_inside(k, datum, band)? {
                    cut = Some(dir);
                    break;
                }
            }
            let Some(dir) = cut else {
                k += 1;
                continue;
            };
            let s = self.sectors[k];
            let reach = match s.at {
                StarFace::Tangent(normal) => {
                    BoundSupport::bisector(s.arm, dir, normal.vec(), s.bend)
                }
                StarFace::Apex(_) => BoundSupport::straight_bisector(s.arm),
            };
            let he = self.bounds[k].he;
            self.bounds.insert(
                k + 1,
                Bound {
                    he,
                    real: false,
                    dir,
                    reach,
                },
            );
            self.sectors.insert(k + 1, s);
        }
        Ok(())
    }

    /// The first extremum of `datum`'s side strictly inside sector `k`,
    /// nearest its own bound, if any.
    fn extremum_inside(
        &self,
        k: usize,
        datum: &Datum<T>,
        band: Band,
    ) -> Result<Option<Vec3<T>>, StarFault> {
        let s = &self.sectors[k];
        let (end, start) = (self.bounds[k].dir, self.start(k).dir);
        match (&s.at, datum) {
            (StarFace::Tangent(_), Datum::Plane(_)) => Ok(None),
            (StarFace::Apex(apex), Datum::Plane(_) | Datum::Cone(_)) => {
                let across = match datum {
                    Datum::Plane(n) => *n,
                    Datum::Cone(other) => other.axis,
                };
                let window = apex.azimuth_from(start, end);
                Ok(
                    apex_extrema_inside(apex, (start, window), across, s.arm.length(), band)
                        .map_err(|fault| StarFault::Shape(s.face, fault))?
                        .into_iter()
                        .next(),
                )
            }
            (StarFace::Tangent(normal), Datum::Cone(other)) => {
                let n = normal.vec();
                let cut = |diag| StarFault::Cut(s.face, diag);
                let tilt = other.axis - n * other.axis.dot(n);
                if decide(
                    "bool_sector_partner_tilt",
                    Margin::levered(tilt.norm(), s.arm),
                    band,
                )
                .map_err(cut)?
                    != Sign::Positive
                {
                    return Ok(None);
                }
                let extrema = geom_brep::cone_plane_apex_extrema(&other.surface(), n)
                    .unwrap_or_else(|| unreachable!("an apex reading's surface is a cone"));
                for d in extrema {
                    let d = d.normalize();
                    let inside = |a: Vec3<T>, b: Vec3<T>| {
                        decide(
                            "bool_sector_within",
                            Margin::levered(a.cross(b).dot(n), s.arm),
                            band,
                        )
                        .map_err(cut)
                    };
                    if inside(start, d)? == Sign::Positive && inside(d, end)? == Sign::Positive {
                        return Ok(Some(d));
                    }
                }
                Ok(None)
            }
        }
    }
}

/// The outgoing direction of orbit half-edge `he` out of `vertex`,
/// scaled to the edge's extent from below, and its reach: a line's
/// chord, a curved edge's departure tangent at its extent with its bend
/// and its arc ([`ConicArc::of`]).
pub(crate) fn departure<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<(Vec3<T>, BoundSupport<T>), StarFault> {
    let far = body.proven_half_edge_end(he);
    let p_base = body.resolve_vertex_point(vertex, Proven);
    let p_far = body.resolve_vertex_point(far, Proven);
    let he_data = proven(&body.half_edges, he, EntityId::HalfEdge);
    let edge = linked(
        &body.edges,
        he_data.edge,
        EntityId::Edge,
        EntityId::HalfEdge(he),
        "edge",
    );
    let curve = body
        .edge_curve_linked(he_data.edge, edge)
        .certified()
        .ok_or(StarFault::NullEdge(he_data.edge))?;
    Ok(match curve.carrier() {
        geom::Curve3::Line(_) => (
            p_far - p_base,
            BoundSupport::Chord {
                base: p_base,
                far: p_far,
            },
        ),
        geom::Curve3::Circle(..)
        | geom::Curve3::Ellipse(..)
        | geom::Curve3::Spiric { .. }
        | geom::Curve3::Nurbs(_) => {
            let he_plus = he == edge.he_plus;
            let geom_brep::ConicDeparture { dir, extent, bend } =
                geom_brep::ConicDeparture::of(curve, he_plus, (p_far, p_base));
            let arc = ConicArc::of(curve, he_plus);
            (dir, BoundSupport::Conic { extent, bend, arc })
        }
    })
}
