//! **A coincident face is divided where its region's fate changes**
//! (`[topology.records]`' lineage clause: one fate per cell).
//!
//! Where two operands' faces coincide and the Eq. 15.3 lump keeps one
//! copy of their shared region, the region of the other copy, the
//! dropped one, has one of two fates: *substituted* (the result holds
//! its interior through the kept copy) or *removed*. The classifier's
//! sites read both: a vertex–vertex site where an edge bounding the
//! kept copy's sector runs into the dropped copy's sector
//! (`recl::recl_sectors`), and a vertex-on-face site where a lumped
//! sector of the kept copy lies on the pierced dropped face
//! (`vtxfac::classify_vertex_on_face`). Each records the kept edge
//! ([`FateEdge`]): on its one side the dropped face is substituted, on
//! its other side not.
//!
//! After the join, [`cut`] divides the dropped face along each such
//! edge with a one-sided chord, between the vertices the dropped
//! operand holds at the edge's two ends: the partner of a vertex–vertex
//! site, the ring vertex of a pierce, or, at a vertex-on-face site the
//! classification did no surgery at, a lone vertex minted there as the
//! ring door mints one. No null edge is paired with the chord: it lies
//! in a face the selection drops, so no section runs along it. Where
//! the dropped face already holds an edge along the kept edge (a
//! section's chord), it is divided there already and nothing is cut.
//!
//! A site whose dropped sector lies wholly on the kept copy
//! ([`FateCorner`]) needs no cut: its corner's fragment is substituted.
//!
//! The product ([`FateCuts`]) is each substituted fragment with the
//! kept face holding it, and each chord with the kept edge it runs
//! along, which the carve reads for the removed fragment's border
//! (`finish::discarded`).

use geom_core::{Band, Decide, Margin, Point3, Sign, Tol, Vec3};

use super::{BooleanError, BooleanReduction, NullCopies, Operand};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use crate::lineage::FaceDivisions;

/// An edge of a kept coincident copy that bounds its region inside the
/// dropped copy (module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct FateEdge {
    /// The operand of the kept copy, whose clone holds `edge`.
    pub holder: Operand,
    /// The edge, in `holder`'s clone keys.
    pub edge: EdgeKey,
    /// The kept copy's face beside it, in `holder`'s clone keys.
    pub kept: FaceKey,
    /// The dropped copy, in the other operand's clone keys as the
    /// classification read them.
    pub face: FaceKey,
    /// Whether the two copies' outward normals agree.
    pub aligned: bool,
}

/// Where a kept copy holds a dropped sector's region.
#[derive(Debug, Clone, Copy)]
pub(crate) enum KeptAt<T: geom_core::Real> {
    /// At the kept copy's own sector, by the orbit half-edge it follows
    /// in the holder's clone.
    Sector(HalfEdgeKey),
    /// On the pierced face, in the holder's clone keys, at the dropped
    /// operand's piercing vertex, the dropped sector's interior along
    /// the unit direction.
    Pierced(FaceKey, VertexKey, Vec3<T>),
}

/// A dropped sector the kept copy holds wholly (module docs).
#[derive(Debug, Clone, Copy)]
pub(crate) struct FateCorner<T: geom_core::Real> {
    /// The operand of the kept copy.
    pub holder: Operand,
    /// The dropped sector, by the orbit half-edge it follows in the
    /// other operand's clone.
    pub dropped: HalfEdgeKey,
    /// Where the kept copy holds it.
    pub kept: KeptAt<T>,
}

/// What the classification owes the dropped copies.
#[derive(Debug, Clone)]
pub(crate) struct Fates<T: geom_core::Real> {
    /// The kept edges along which a dropped face's fate changes.
    pub edges: Vec<FateEdge>,
    /// The dropped sectors held wholly.
    pub corners: Vec<FateCorner<T>>,
}

impl<T: geom_core::Real> Default for Fates<T> {
    fn default() -> Self {
        Self {
            edges: Vec::new(),
            corners: Vec::new(),
        }
    }
}

/// **A face a boolean holds through a coincident face**
/// (`BooleanNaming::substituted`): where two operands' faces coincide
/// and the Eq. 15.3 lump keeps one copy of their shared region, the
/// fragment of the dropped copy that region covers, with the kept face
/// holding its interior. The dropped face is cut where the region ends
/// ([`super::fate`]), so the fragment is substituted whole, never in
/// part. Clone keys, as [`super::DiscardRow::face`]: chase
/// `face_fragments_a`/`face_fragments_b` for the operand faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubstitutedFace {
    /// The dropped copy's operand.
    pub operand: Operand,
    /// The fragment, in its operand's clone keys.
    pub face: FaceKey,
    /// The kept face holding it, in the other operand's clone keys.
    pub by: FaceKey,
}

/// A one-sided chord and the kept edge it runs along.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Chord {
    /// The dropped operand, whose clone holds `edge`.
    pub operand: Operand,
    /// The chord.
    pub edge: EdgeKey,
    /// The kept edge, in the other operand's clone keys.
    pub along: EdgeKey,
    /// The chord's two ends, each at the kept edge's end in `kept_ends`.
    pub ends: [VertexKey; 2],
    /// The kept edge's two ends, in the other operand's clone keys.
    pub kept_ends: [VertexKey; 2],
}

/// [`cut`]'s product (module docs).
#[derive(Debug, Clone, Default)]
pub(crate) struct FateCuts {
    /// Every substituted fragment, sorted and deduplicated.
    pub substituted: Vec<SubstitutedFace>,
    /// Every chord, in mint order.
    pub chords: Vec<Chord>,
}

impl FateCuts {
    /// The chord `edge` of `operand`'s clone, if it is one.
    pub(crate) fn chord(&self, operand: Operand, edge: EdgeKey) -> Option<&Chord> {
        self.chords
            .iter()
            .find(|c| c.operand == operand && c.edge == edge)
    }

    /// Whether `face` of `operand`'s clone is substituted.
    pub(crate) fn is_substituted(&self, operand: Operand, face: FaceKey) -> bool {
        self.substituted
            .iter()
            .any(|s| s.operand == operand && s.face == face)
    }
}

/// The fate edges that bound a dropped region from outside. A null
/// edge is the split's scaffolding, not an edge of the face, and an
/// edge whose two faces both cover the dropped face (a seam between two
/// coplanar faces of the kept copy, or a face meeting itself) lies
/// inside the held region; both are dropped. Sorted and deduplicated.
pub(super) fn border_fates<T: geom_core::Real>(
    mut fates: Fates<T>,
    covered: &[(FaceKey, FaceKey)],
    null_edges: &[super::BoolNullEdgeRecord<T>],
    a: &Body<T>,
    b: &Body<T>,
) -> Result<Fates<T>, BooleanError> {
    fates.edges.sort();
    fates.edges.dedup();
    let mut edges = Vec::with_capacity(fates.edges.len());
    for h in fates.edges {
        if null_edges
            .iter()
            .any(|r| r.operand == h.holder && r.edge == h.edge)
        {
            continue;
        }
        let body = match h.holder {
            Operand::A => a,
            Operand::B => b,
        };
        let edge = body
            .get_edge(h.edge)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "a fate edge no longer resolves",
            })?;
        let sides = [edge.he_plus, edge.he_minus].map(|he| body.face_of_half_edge(he));
        let covers = |f: Option<FaceKey>| {
            f.is_some_and(|f| {
                covered.contains(&match h.holder {
                    Operand::A => (f, h.face),
                    Operand::B => (h.face, f),
                })
            })
        };
        if sides[0] == sides[1] || sides.iter().all(|&f| covers(f)) {
            continue;
        }
        edges.push(h);
    }
    fates.edges = edges;
    Ok(fates)
}

/// Where a chord meets the dropped face at one of its ends.
#[derive(Debug, Clone, Copy)]
enum Place {
    /// An edge of the face already runs this way from the end.
    Along(HalfEdgeKey),
    /// In the corner before this half-edge, at its start.
    Corner(HalfEdgeKey),
    /// At a lone vertex, in its empty loop.
    Lone(LoopKey, VertexKey),
}

/// The unit direction `he` leaves its start along, with the edge's
/// chord length; `None` for null scaffolding.
fn departure<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Option<(Vec3<T>, T)> {
    let h = body.get_half_edge(he)?;
    let edge = body.get_edge(h.edge)?;
    let curve = body.get_curve_geom(edge.curve)?.certified()?;
    let base = body.half_edge_start_point(he)?;
    let end = body.half_edge_start_point(body.mate(he)?)?;
    let len = (end - base).norm();
    let dir = match curve.carrier() {
        geom::Curve3::Line { .. } | geom::Curve3::Nurbs(_) => end - base,
        geom::Curve3::Circle { .. }
        | geom::Curve3::Ellipse { .. }
        | geom::Curve3::Spiric { .. } => {
            geom_brep::ConicDeparture::of(curve, he == edge.he_plus, len).dir
        }
    };
    Some((dir / dir.norm(), len))
}

/// The sign of `x`, a dimensionless reading levered at `arm`.
fn sign<T: Decide>(x: T, arm: T, band: Band) -> Result<Sign, BooleanError> {
    crate::validate::decide("bool_fate_corner", Margin::levered(x, arm), band).map_err(|diag| {
        BooleanError::coincidence(super::Coincide::Sectors, super::DeclarationRead::Moot, diag)
    })
}

/// Whether unit `d` lies strictly inside the corner swept
/// counterclockwise about `n` from unit `out` to unit `back`, or along
/// `out`.
fn corner_holds<T: Decide>(
    d: Vec3<T>,
    out: Vec3<T>,
    back: Vec3<T>,
    n: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    let s1 = sign(n.dot(out.cross(d)), arm, band)?;
    if s1 == Sign::Zero && sign(out.dot(d), arm, band)? == Sign::Positive {
        return Ok(None);
    }
    let s2 = sign(n.dot(d.cross(back)), arm, band)?;
    let turn = sign(n.dot(out.cross(back)), arm, band)?;
    let holds = match turn {
        Sign::Positive => s1 == Sign::Positive && s2 == Sign::Positive,
        Sign::Negative => s1 == Sign::Positive || s2 == Sign::Positive,
        Sign::Zero if sign(out.dot(back), arm, band)? == Sign::Positive => {
            !(s1 == Sign::Zero && s2 == Sign::Zero)
        }
        Sign::Zero => s1 == Sign::Positive,
    };
    Ok(Some(holds))
}

/// The place, among the faces of `body` divided from `root`, where a
/// chord leaving `at` (or a null-edge copy of it) along unit `d`
/// starts; `None` where none of those faces holds `at`: the kept edge
/// leaves the dropped face where the reduction put no vertex in it.
fn place<T: Decide>(
    body: &Body<T>,
    at: &[VertexKey],
    root: FaceKey,
    divisions: &FaceDivisions,
    d: Vec3<T>,
    band: Band,
) -> Result<Option<Place>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut found = Vec::new();
    let mut touched = false;
    for &w in at {
        let vertex = body
            .get_vertex(w)
            .ok_or_else(|| desync("a fate cut's end vertex no longer resolves"))?;
        let Some(_) = vertex.emanating else {
            for (f, face) in body.faces() {
                if divisions.root(f) != root {
                    continue;
                }
                for &l in &face.rings {
                    if let Some(lp) = body.get_loop(l)
                        && lp.boundary == (LoopBoundary::Empty { vertex: w })
                    {
                        touched = true;
                        found.push(Place::Lone(l, w));
                    }
                }
            }
            continue;
        };
        for h in body.vertex_orbit_linked(w) {
            let f = body
                .face_of_half_edge(h)
                .ok_or_else(|| desync("a fate cut's corner has no face"))?;
            if divisions.root(f) != root || body.null_face_pair(f).is_some() {
                continue;
            }
            touched = true;
            let Some((out, out_len)) = departure(body, h) else {
                continue;
            };
            // The corner's other bound is the first edge before `h` that
            // is not null scaffolding.
            let mut p = h;
            let back = loop {
                p = body
                    .get_half_edge(p)
                    .ok_or_else(|| desync("a fate cut's corner does not walk"))?
                    .prev;
                let mate = body
                    .mate(p)
                    .ok_or_else(|| desync("a fate cut's corner edge has no mate"))?;
                if let Some(back) = departure(body, mate) {
                    break back;
                }
                if p == h {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "a fate cut's corner is bounded by null edges only",
                    });
                }
            };
            let p = body
                .half_edge_start_point(h)
                .ok_or_else(|| desync("a fate cut's corner has no point"))?;
            let normal = crate::face_normal::face_outward_normal_at(body, f, p, band)
                .ok()
                .flatten()
                .ok_or(BooleanError::ClassificationInvariant {
                    what: "a fate cut's corner face has no normal there",
                })?;
            let arm = out_len.min(back.1);
            match corner_holds(d, out, back.0, normal.vec(), arm, band)? {
                None => found.push(Place::Along(h)),
                Some(true) => found.push(Place::Corner(h)),
                Some(false) => {}
            }
        }
    }
    if !touched {
        return Ok(None);
    }
    if let Some(&along) = found.iter().find(|p| matches!(p, Place::Along(_))) {
        return Ok(Some(along));
    }
    match found.as_slice() {
        [one] => Ok(Some(*one)),
        [] => Err(BooleanError::ClassificationInvariant {
            what: "a fate cut's end lies in no corner of the dropped face",
        }),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "a fate cut's end lies in several corners of the dropped face",
        }),
    }
}

/// Whether a face divided from `root` holds one of `at`, a lone vertex
/// in it among them.
fn holds<T: geom_core::Real>(
    body: &Body<T>,
    at: &[VertexKey],
    root: FaceKey,
    divisions: &FaceDivisions,
) -> bool {
    let ours = |f: FaceKey| divisions.root(f) == root && body.null_face_pair(f).is_none();
    at.iter()
        .any(|&w| match body.get_vertex(w).and_then(|v| v.emanating) {
            Some(_) => body
                .vertex_orbit_linked(w)
                .into_iter()
                .any(|h| body.face_of_half_edge(h).is_some_and(ours)),
            None => body.faces().any(|(f, face)| {
                ours(f)
                    && face.rings.iter().any(|&l| {
                        body.get_loop(l)
                            .is_some_and(|lp| lp.boundary == (LoopBoundary::Empty { vertex: w }))
                    })
            }),
        })
}

/// Where the dropped operand meets kept vertex `x`.
enum End {
    /// At these vertices: a vertex–vertex partner or a pierce's ring
    /// vertex, each with its null-edge copies.
    At(Vec<VertexKey>),
    /// At this vertex of the kept operand touching the dropped face with
    /// no surgery, where the face holds no vertex.
    Touch(VertexKey),
}

/// Where the dropped operand meets kept vertex `x`, which lies in the
/// face divided from `root`.
fn end_at<T: geom_core::Real>(
    red: &BooleanReduction<T>,
    holder: Operand,
    x: VertexKey,
    root: FaceKey,
    divisions: &FaceDivisions,
) -> Result<End, BooleanError> {
    let dropped = holder.other();
    let xs = NullCopies::of_operand(&red.null_edges, holder).of(x);
    let theirs = NullCopies::of_operand(&red.null_edges, dropped);
    // A pinch is several vertices on one point, one per cone, each a
    // partner: every one is offered, and the face's own is placed.
    let mut at = Vec::new();
    for c in &red.contacts.vv {
        let (k, d) = match holder {
            Operand::A => (c.a, c.b),
            Operand::B => (c.b, c.a),
        };
        if xs.contains(&k) {
            at.extend(theirs.of(d));
        }
    }
    for ring in &red.pierce_rings {
        if ring.operand == dropped
            && divisions.root(ring.face) == root
            && ring.pierces.iter().any(|v| xs.contains(v))
        {
            at.extend(theirs.of(ring.ring_vertex));
        }
    }
    if !at.is_empty() {
        at.sort();
        at.dedup();
        return Ok(End::At(at));
    }
    let on = match holder {
        Operand::A => &red.contacts.a_on_b,
        Operand::B => &red.contacts.b_on_a,
    };
    on.iter()
        .find(|c| xs.contains(&c.vertex) && divisions.root(c.face) == root)
        .map(|c| End::Touch(c.vertex))
        .ok_or(BooleanError::ClassificationInvariant {
            what: "a fate cut's end is no contact of the dropped face",
        })
}

/// The face divided from `root` that holds `p` inside it.
fn fragment_at<T: Decide>(
    body: &Body<T>,
    root: FaceKey,
    divisions: &FaceDivisions,
    p: Point3<T>,
    band: Band,
) -> Result<FaceKey, BooleanError> {
    let pieces: Vec<FaceKey> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| divisions.root(f) == root && body.null_face_pair(f).is_none())
        .collect();
    if let [one] = pieces.as_slice() {
        return Ok(*one);
    }
    let mut inside = Vec::new();
    for f in pieces {
        let normal = crate::face_normal::face_outward_normal(body, f).ok_or(
            BooleanError::ClassificationInvariant {
                what: "a fate cut's end lies in a divided curved face",
            },
        )?;
        let at = super::contfp(body, f, normal.vec(), p, band).map_err(|_| {
            BooleanError::ClassificationInvariant {
                what: "a fate cut's end is not placed in its face",
            }
        })?;
        if at == super::FaceContainment::In {
            inside.push(f);
        }
    }
    match inside.as_slice() {
        [one] => Ok(*one),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "a fate cut's end lies inside no one piece of its face",
        }),
    }
}

/// The signed area `cycle`'s points enclose about `n`, over its
/// length: positive where it winds counterclockwise.
fn winding<T: Decide>(points: &[Point3<T>], n: Vec3<T>) -> (T, T) {
    let mut area = T::zero();
    let mut len = T::zero();
    for (k, &p) in points.iter().enumerate() {
        let q = points[(k + 1) % points.len()];
        area = area + n.dot((p - points[0]).cross(q - points[0]));
        len = len + (q - p).norm();
    }
    (area, len)
}

/// The loop half-edge `he` belongs to.
fn loop_of<T: geom_core::Real>(body: &Body<T>, he: HalfEdgeKey) -> Result<LoopKey, BooleanError> {
    body.get_half_edge(he)
        .map(|h| h.parent_loop)
        .ok_or(BooleanError::JoinDesync {
            what: "a fate cut's half-edge no longer resolves",
        })
}

/// Joins `from` to `to` in their face with a line, returning the half
/// running from `from`'s vertex to `to`'s; a division it makes is
/// recorded in `divisions`.
fn chord<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    (from, to): (Place, Place),
    divisions: &mut FaceDivisions,
    band: Band,
    tol: Tol,
) -> Result<HalfEdgeKey, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let start_of = |body: &Body<T>, p: Place| -> Result<VertexKey, BooleanError> {
        match p {
            Place::Corner(h) | Place::Along(h) => body
                .get_half_edge(h)
                .map(|h| h.start)
                .ok_or_else(|| desync("a fate cut's corner no longer resolves")),
            Place::Lone(_, w) => Ok(w),
        }
    };
    let face_of = |body: &Body<T>, p: Place| -> Result<FaceKey, BooleanError> {
        let l = match p {
            Place::Corner(h) | Place::Along(h) => loop_of(body, h)?,
            Place::Lone(l, _) => l,
        };
        body.get_loop(l)
            .map(|l| l.face)
            .ok_or_else(|| desync("a fate cut's loop no longer resolves"))
    };
    let (u, w) = (start_of(body, from)?, start_of(body, to)?);
    let face = face_of(body, from)?;
    if face != face_of(body, to)? {
        return Err(BooleanError::ClassificationInvariant {
            what: "a fate cut's two ends lie in different pieces of the dropped face",
        });
    }
    let point = |body: &Body<T>, v: VertexKey| {
        body.get_vertex(v)
            .and_then(|v| body.get_point(v.point))
            .copied()
            .ok_or_else(|| desync("a fate cut's end has no point"))
    };
    let (pu, pw) = (point(body, u)?, point(body, w)?);
    let line = || geom_brep::EdgeCurveSpec::line_between(pu, pw);
    let outer = body
        .get_face(face)
        .map(|f| f.outer)
        .ok_or_else(|| desync("a fate cut's face no longer resolves"))?;
    let made = match (from, to) {
        (Place::Corner(h0), Place::Corner(h1)) if loop_of(body, h0)? == loop_of(body, h1)? => {
            // A ring the chord closes splits into the new face's outer
            // loop, which must wind counterclockwise, and the rest.
            let (he1, he2) = if loop_of(body, h0)? == outer {
                (h0, h1)
            } else {
                let normal = crate::face_normal::face_outward_normal(body, face).ok_or(
                    BooleanError::ClassificationInvariant {
                        what: "a fate cut closes a ring on a curved face",
                    },
                )?;
                let mut points = Vec::new();
                let mut at = h0;
                while at != h1 {
                    points.push(point(body, start_of(body, Place::Corner(at))?)?);
                    at = body
                        .get_half_edge(at)
                        .ok_or_else(|| desync("a fate cut's ring does not walk"))?
                        .next;
                }
                points.push(pw);
                let (area, len) = winding(&points, normal.vec());
                match crate::validate::decide("bool_fate_ring", Margin::of(area / len), band) {
                    Ok(Sign::Positive) => (h0, h1),
                    Ok(Sign::Negative) => (h1, h0),
                    Ok(Sign::Zero) | Err(_) => {
                        return Err(BooleanError::ClassificationInvariant {
                            what: "a fate cut closes a ring of no area",
                        });
                    }
                }
            };
            let made = body.mef(
                MefSite::Chords { he1, he2 },
                geom_brep::EdgeCurveSpec::line_between(
                    point(body, start_of(body, Place::Corner(he1))?)?,
                    point(body, start_of(body, Place::Corner(he2))?)?,
                ),
                FaceSurface::Inherit,
                tol,
            )?;
            divisions.push((made.face, face))?;
            [made.he_plus, made.he_minus]
        }
        (Place::Corner(h0), Place::Corner(h1)) => {
            let (target, ring) = if loop_of(body, h1)? == outer {
                (h1, h0)
            } else {
                (h0, h1)
            };
            let made = body.mekr(
                MekrSite::Cycles { target, ring },
                geom_brep::EdgeCurveSpec::line_between(
                    point(body, start_of(body, Place::Corner(target))?)?,
                    point(body, start_of(body, Place::Corner(ring))?)?,
                ),
                tol,
            )?;
            [made.he_plus, made.he_minus]
        }
        (Place::Corner(target), Place::Lone(ring, _))
        | (Place::Lone(ring, _), Place::Corner(target)) => {
            let far = match (from, to) {
                (Place::Lone(_, v), _) | (_, Place::Lone(_, v)) => v,
                _ => unreachable!("one end is lone"),
            };
            let made = body.mekr(
                MekrSite::EmptyRing { target, ring },
                geom_brep::EdgeCurveSpec::line_between(
                    point(body, start_of(body, Place::Corner(target))?)?,
                    point(body, far)?,
                ),
                tol,
            )?;
            [made.he_plus, made.he_minus]
        }
        (Place::Lone(target, _), Place::Lone(ring, _)) => {
            let made = body.mekr(MekrSite::BothEmpty { target, ring }, line(), tol)?;
            [made.he_plus, made.he_minus]
        }
        (Place::Along(_), _) | (_, Place::Along(_)) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "a fate cut runs along an edge at one end only",
            });
        }
    };
    made.into_iter()
        .find(|&he| body.get_half_edge(he).is_some_and(|h| h.start == u))
        .ok_or_else(|| desync("a fate cut's chord does not start at its end"))
}

/// The face of the sector following orbit half-edge `he`: the face of
/// its mate, which ends at the sector's vertex
/// (`crate::sector_face::face_of`), or the piece of it beside `he`'s
/// edge where a cut divided it since.
fn sector_face<T: geom_core::Real>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> Result<FaceKey, BooleanError> {
    let mate = body.mate(he).ok_or(BooleanError::JoinDesync {
        what: "a sector's half-edge has no mate",
    })?;
    face_of_he(body, mate)
}

/// The face holding `he`, in `body`.
fn face_of_he<T: geom_core::Real>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> Result<FaceKey, BooleanError> {
    body.face_of_half_edge(he).ok_or(BooleanError::JoinDesync {
        what: "a fate cut's half-edge has no face",
    })
}

/// **Divides each dropped coincident face where its fate changes**
/// (module docs), after the join: `divisions` are each operand's face
/// division rows, A's then B's, which each chord that divides a face
/// extends.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where a cut cannot be
/// placed in one piece of the dropped face, or leaves a slit (one face
/// on both sides of a chord); the Euler operators' refusals.
pub(super) fn cut<T: Decide + crate::props::AtRestPolicy>(
    red: &mut BooleanReduction<T>,
    divisions: [&mut FaceDivisions; 2],
    band: Band,
    tol: Tol,
) -> Result<FateCuts, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let [div_a, div_b] = divisions;
    let mut lone = std::collections::BTreeMap::new();
    // Each half whose face is substituted, with the kept half whose face
    // holds it; each chord, with its other half.
    let mut halves: Vec<(Operand, HalfEdgeKey, HalfEdgeKey)> = Vec::new();
    let mut out = FateCuts::default();
    let mut slits = Vec::new();
    let mut done = std::collections::BTreeSet::new();
    // A dropped face a kept edge leaves where the reduction put no vertex
    // in it cannot be cut along that edge, so it is cut along none
    // ([`holds`]). Every lone vertex is placed before any is minted: the
    // containment reading cannot walk an empty loop.
    let mut uncut = std::collections::BTreeSet::new();
    let mut touches: Vec<(Operand, VertexKey, FaceKey, Point3<T>)> = Vec::new();
    for fe in &red.fates.edges {
        let (div_d, body, kept_body) = match fe.holder {
            Operand::A => (&*div_b, &red.b, &red.a),
            Operand::B => (&*div_a, &red.a, &red.b),
        };
        let root = div_d.root(fe.face);
        let edge = kept_body
            .get_edge(fe.edge)
            .ok_or_else(|| desync("a fate edge no longer resolves"))?;
        for he in [edge.he_plus, edge.he_minus] {
            let x = kept_body
                .get_half_edge(he)
                .ok_or_else(|| desync("a fate edge's half-edge no longer resolves"))?
                .start;
            match end_at(red, fe.holder, x, root, div_d)? {
                End::At(ws) => {
                    if !holds(body, &ws, root, div_d) {
                        uncut.insert((fe.holder.other(), root));
                    }
                }
                End::Touch(t) => {
                    if !touches.iter().any(|&(h, k, ..)| (h, k) == (fe.holder, t)) {
                        let p = kept_body
                            .get_vertex(t)
                            .and_then(|v| kept_body.get_point(v.point))
                            .copied()
                            .ok_or_else(|| desync("a fate cut's touching vertex has no point"))?;
                        touches.push((fe.holder, t, root, p));
                    }
                }
            }
        }
    }
    let touches = touches
        .into_iter()
        .filter(|&(h, _, root, _)| !uncut.contains(&(h.other(), root)))
        .map(|(h, t, root, p)| {
            let (div_d, body) = match h {
                Operand::A => (&*div_b, &red.b),
                Operand::B => (&*div_a, &red.a),
            };
            Ok((h, t, fragment_at(body, root, div_d, p, band)?, p))
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    for (holder, t, face, p) in touches {
        let body = match holder {
            Operand::A => &mut red.b,
            Operand::B => &mut red.a,
        };
        let (w, _) = super::vtxfac::ring_vertex_in(body, face, p, tol)?;
        lone.insert((holder, t), w);
    }
    for fe in red.fates.edges.clone() {
        let dropped = fe.holder.other();
        let (div_k, div_d) = match fe.holder {
            Operand::A => (&*div_a, &mut *div_b),
            Operand::B => (&*div_b, &mut *div_a),
        };
        let root = div_d.root(fe.face);
        if uncut.contains(&(dropped, root)) || !done.insert((fe.holder, fe.edge, root)) {
            continue;
        }
        let kept_body = match fe.holder {
            Operand::A => &red.a,
            Operand::B => &red.b,
        };
        let edge = kept_body
            .get_edge(fe.edge)
            .ok_or_else(|| desync("a fate edge no longer resolves"))?;
        let kept_root = div_k.root(fe.kept);
        let kept_he = [edge.he_plus, edge.he_minus]
            .into_iter()
            .find(|&he| {
                kept_body
                    .face_of_half_edge(he)
                    .is_some_and(|f| div_k.root(f) == kept_root)
            })
            .ok_or_else(|| desync("a fate edge does not bound its kept face"))?;
        let kept_mate = kept_body
            .mate(kept_he)
            .ok_or_else(|| desync("a fate edge has no mate"))?;
        let ends = [kept_he, kept_mate].map(|he| {
            kept_body
                .get_half_edge(he)
                .map(|h| h.start)
                .zip(departure(kept_body, he))
        });
        let [Some((x0, (d0, _))), Some((x1, (d1, _)))] = ends else {
            return Err(desync("a fate edge has no direction"));
        };
        let at = |x| match end_at(red, fe.holder, x, root, div_d)? {
            End::At(ws) => Ok(ws),
            End::Touch(t) => lone
                .get(&(fe.holder, t))
                .map(|&w| vec![w])
                .ok_or(desync("a touching fate cut end has no lone vertex")),
        };
        let (w0, w1) = (at(x0)?, at(x1)?);
        let body = match dropped {
            Operand::A => &mut red.a,
            Operand::B => &mut red.b,
        };
        let (Some(p0), Some(p1)) = (
            place(body, &w0, root, div_d, d0, band)?,
            place(body, &w1, root, div_d, d1, band)?,
        ) else {
            return Err(BooleanError::ClassificationInvariant {
                what: "a fate cut's end left the dropped face as it was cut",
            });
        };
        let run = match (p0, p1) {
            (Place::Along(h), Place::Along(_)) => h,
            _ => {
                let c = chord(body, (p0, p1), div_d, band, tol)?;
                let h = body
                    .get_half_edge(c)
                    .cloned()
                    .ok_or_else(|| desync("a fate cut's chord no longer resolves"))?;
                let end = body
                    .half_edge_end(c)
                    .ok_or_else(|| desync("a fate cut's chord has no end"))?;
                out.chords.push(Chord {
                    operand: dropped,
                    edge: h.edge,
                    along: fe.edge,
                    ends: [h.start, end],
                    kept_ends: [x0, x1],
                });
                slits.push((dropped, c));
                c
            }
        };
        let mate = body
            .mate(run)
            .ok_or_else(|| desync("a fate cut's run has no mate"))?;
        halves.push((dropped, if fe.aligned { run } else { mate }, kept_he));
    }
    for (operand, c) in slits {
        let body = match operand {
            Operand::A => &red.a,
            Operand::B => &red.b,
        };
        let mate = body
            .mate(c)
            .ok_or_else(|| desync("a fate cut's chord has no mate"))?;
        if face_of_he(body, c)? == face_of_he(body, mate)? {
            return Err(BooleanError::ClassificationInvariant {
                what: "a fate cut leaves a slit in the dropped face",
            });
        }
    }
    let mut corners: Vec<(Operand, FaceKey, FaceKey)> = Vec::new();
    for &(operand, half, kept_he) in &halves {
        let (body, kept_body) = match operand {
            Operand::A => (&red.a, &red.b),
            Operand::B => (&red.b, &red.a),
        };
        corners.push((
            operand,
            face_of_he(body, half)?,
            face_of_he(kept_body, kept_he)?,
        ));
    }
    for fc in red.fates.corners.clone() {
        let dropped = fc.holder.other();
        let (body, kept_body, div_k) = match fc.holder {
            Operand::A => (&red.b, &red.a, &*div_a),
            Operand::B => (&red.a, &red.b, &*div_b),
        };
        let face = sector_face(body, fc.dropped)?;
        let div_d = match fc.holder {
            Operand::A => &*div_b,
            Operand::B => &*div_a,
        };
        if uncut.contains(&(dropped, div_d.root(face))) {
            continue;
        }
        let by = match fc.kept {
            KeptAt::Sector(he) => sector_face(kept_body, he)?,
            KeptAt::Pierced(f, at, d) => {
                let root = div_k.root(f);
                let ring = red.pierce_rings.iter().find(|r| {
                    r.operand == fc.holder && div_k.root(r.face) == root && r.pierces.contains(&at)
                });
                match ring {
                    Some(ring) => {
                        let copies =
                            NullCopies::of_operand(&red.null_edges, fc.holder).of(ring.ring_vertex);
                        match place(kept_body, &copies, root, div_k, d, band)? {
                            None => {
                                return Err(BooleanError::ClassificationInvariant {
                                    what: "a pierce's ring vertex lies off its kept face",
                                });
                            }
                            Some(Place::Corner(h)) => face_of_he(kept_body, h)?,
                            Some(Place::Lone(l, _)) => kept_body
                                .get_loop(l)
                                .map(|l| l.face)
                                .ok_or_else(|| desync("a ring vertex's loop no longer resolves"))?,
                            Some(Place::Along(_)) => {
                                return Err(BooleanError::ClassificationInvariant {
                                    what: "a dropped sector held at a pierce runs along a kept edge",
                                });
                            }
                        }
                    }
                    None => {
                        let p = body
                            .get_vertex(at)
                            .and_then(|v| body.get_point(v.point))
                            .copied()
                            .ok_or_else(|| desync("a piercing vertex has no point"))?;
                        fragment_at(kept_body, root, div_k, p, band)?
                    }
                }
            }
        };
        corners.push((dropped, face, by));
    }
    let mut substituted: Vec<SubstitutedFace> = corners
        .into_iter()
        .map(|(operand, face, by)| SubstitutedFace { operand, face, by })
        .collect();
    substituted.sort();
    substituted.dedup();
    out.substituted = substituted;
    Ok(out)
}

/// The faces a path that cuts nothing holds through coincident ones
/// (the containment fallback, which keeps or drops whole operands):
/// each dropped face the classification found a kept copy over, whole.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where a recorded key no longer
/// resolves.
pub(super) fn uncut<T: geom_core::Real>(
    red: &BooleanReduction<T>,
) -> Result<Vec<SubstitutedFace>, BooleanError> {
    let mut out = Vec::new();
    for fe in &red.fates.edges {
        out.push(SubstitutedFace {
            operand: fe.holder.other(),
            face: fe.face,
            by: fe.kept,
        });
    }
    for fc in &red.fates.corners {
        let (body, kept_body) = match fc.holder {
            Operand::A => (&red.b, &red.a),
            Operand::B => (&red.a, &red.b),
        };
        let by = match fc.kept {
            KeptAt::Sector(he) => sector_face(kept_body, he)?,
            KeptAt::Pierced(f, ..) => f,
        };
        out.push(SubstitutedFace {
            operand: fc.holder.other(),
            face: sector_face(body, fc.dropped)?,
            by,
        });
    }
    out.sort();
    out.dedup();
    Ok(out)
}
