//! **The material parts an edge of the result lying in one of its own
//! faces** (`crates/topo/README.md`, tier 3: where more than two sheets
//! meet on a line, the material decides the edges, and no face passes
//! through an edge's material wedge).
//!
//! The join pairs each operand's own faces into an edge. Where an edge of
//! one operand lies in a face of the other and both its faces leave the
//! line on that face's material side (a V-trench whose keel rests on a
//! floor), the result's material about the line is two wedges, each
//! between the face and one of the edge's faces, and the edge as the join
//! leaves it claims the face's empty side as well. [`part_riders`] reads
//! every such edge of the result and builds what the material decides:
//! two coincident distinct edges, face | wall and wall | face, carried by
//! the face as
//!
//! - a zero-area ring, where neither end lies on the face's boundary;
//! - a slit of the loop an end lies on, that end parted into one vertex
//!   per wedge;
//! - two faces, where both ends lie on one loop of the face: each end
//!   parts, and so does the face, along the line.
//!
//! The parting is Euler surgery on the result. `mef` in the wall the
//! edge's plus half runs in mints the second edge, and the two-edge face
//! between them; `kfmrh` makes that face a ring of the floor, or, at an
//! end on the floor's boundary, a `mev_null` parts the end and `kef`
//! merges the two-edge face's loop into the floor's, as the zip parts a
//! pinch (`zip::split_cones`). The two edges are then described on the
//! faces they bound.

use std::collections::BTreeSet;

use geom_core::{Band, Decide, Margin, Sign, Tol};

use super::ops::{PieceRead, piece_holds};
use super::{BooleanDecision, BooleanError, Coincide, DeclarationRead};
use crate::Cell;
use crate::body::Body;
use crate::boolean::records::{Contact, ContactKind, ContactRecords};
use crate::coincidence::Cited;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopKey, VertexKey};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::live::proven;

/// Why an edge of the result lying in one of its faces is not parted
/// ([`BooleanError::EdgeInFace`]).
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// Every case but one is a construction gap with no lever, and
/// [`EdgeInFaceGap::Crossed`] is a kernel defect: nothing a person does
/// differs by case, and the refusal's sentence names it. A Rust caller
/// can already name and match it, since `pncad` re-exports `topo`
/// whole.
///
/// **The falsifier is a caller that acts on the case by itself**: a
/// driver that moves an end off the face's boundary when it reads
/// [`EdgeInFaceGap::EndOnBoundary`], say. Then carry it and delete this
/// paragraph and the disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeInFaceGap {
    /// One of the edge's own faces has no normal at the edge's midpoint
    /// to read its side of the face by.
    NoNormal,
    /// One of the edge's faces is tangent to the face along the line,
    /// on a surface whose bend off its tangent plane is not read here.
    Tangent,
    /// An end of the edge lies on the face's loops more than once, or
    /// the edge's faces go into the material of two faces it lies in.
    Shared,
    /// The edge lies in the face along part of its length and runs on
    /// past the face's boundary, which the parting would first divide
    /// it at.
    Partly,
    /// An end of the edge touches the face's boundary at a cell of it
    /// other than the end itself: inside one of its edges, or at another
    /// vertex on the end's point.
    EndOnBoundary,
    /// Both ends of the edge lie on one hole of the face, which the
    /// parting would divide into a hole and a piece of face.
    Pocket,
    /// A contact record names a face the parting divides, whose cell
    /// lies on one of its pieces.
    Recorded,
    /// The edge's two faces leave the line on opposite sides of the
    /// face: the face crosses the edge's material, which no Boolean
    /// result holds. A kernel defect.
    Crossed,
}

impl EdgeInFaceGap {
    /// The words a refusal says it in.
    pub(super) const fn words(self) -> &'static str {
        match self {
            Self::NoNormal => "one of the edge's faces has no normal there to read its side by",
            Self::Shared => {
                "an end of the edge lies on the face's boundary more than once, or the edge's \
                 faces go into the material of two faces it lies in"
            }
            Self::Tangent => {
                "one of the edge's faces is tangent to the face there, on a surface whose bend is \
                 not read"
            }
            Self::Partly => "the edge runs on past the face's boundary",
            Self::EndOnBoundary => {
                "an end of the edge touches the face's boundary away from the end itself"
            }
            Self::Pocket => "both ends of the edge lie on one hole of the face",
            Self::Recorded => "a contact record names the face the parting would divide",
            Self::Crossed => "the edge's faces leave the line on opposite sides of the face",
        }
    }
}

/// One edge the material parted ([`part_riders`]).
#[derive(Clone, Debug)]
pub(super) struct Parted {
    /// The edge as the join left it, now face | one wall.
    pub(super) kept: EdgeKey,
    /// Its coincident twin, face | the other wall.
    pub(super) minted: EdgeKey,
    /// The face it lay in.
    pub(super) face: FaceKey,
    /// The face the parting divided off `face`, where both ends lay on
    /// its boundary.
    pub(super) divided: Option<FaceKey>,
    /// The edge's ends, before the parting.
    pub(super) ends: (VertexKey, VertexKey),
    /// Each end the parting divided, as `(copy, end)`: one vertex per
    /// wedge, on the end's point key.
    pub(super) copies: Vec<(VertexKey, VertexKey)>,
}

/// **Parts every edge of `body` lying in one of its faces whose material
/// both of the edge's faces leave the line into**, and returns the
/// records that hold after it: the edge-on-face and vertex-on-face
/// records the parting makes structure are dropped, and a record of the
/// edge, or of an end it divides, is held by the twin too.
///
/// An edge lies in a face `f` it does not bound where both its ends lie
/// on `f` — on its loops, or by a vertex-on-face record — or an
/// edge-on-face record names the pair, and the edge between them lies
/// in `f`: a line edge whose midpoint `f`, a plane, holds; a conic edge
/// whose plane is `f`'s and whose midpoint `f` holds. An edge whose
/// faces both leave the line on `f`'s empty side touches `f` from
/// outside its wedge, and stays one edge in contact with it.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where a reading is in band;
/// [`BooleanError::EdgeInFace`] where the parting is not built
/// ([`EdgeInFaceGap`]); the Euler operators' and the re-description's
/// refusals.
pub(super) fn part_riders<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    contacts: &ContactRecords,
    band: Band,
    tol: Tol,
) -> Result<(Vec<Parted>, ContactRecords), BooleanError> {
    let riders = riders(body, contacts, band)?;
    let mut parted = Vec::new();
    let mut dropped: BTreeSet<[Cell; 2]> = BTreeSet::new();
    // Each parted edge and divided end, and its twin: every contact the
    // one holds the other holds too, at one point or along one line.
    let mut twins: Vec<(Cell, Cell)> = Vec::new();
    let mut divided: BTreeSet<FaceKey> = BTreeSet::new();
    for (edge, faces) in riders {
        let mut parting = Vec::new();
        for (face, partly) in faces {
            if let Some(tangent) = walls_inside(body, edge, face, band)? {
                if partly {
                    return Err(BooleanError::EdgeInFace {
                        edge,
                        face,
                        gap: EdgeInFaceGap::Partly,
                    });
                }
                parting.push((face, tangent));
            }
        }
        let (face, tangent) = match parting[..] {
            [] => continue,
            [one] => one,
            [(face, _), ..] => {
                return Err(BooleanError::EdgeInFace {
                    edge,
                    face,
                    gap: EdgeInFaceGap::Shared,
                });
            }
        };
        let (a, b) = ends(body, edge).ok_or(BooleanError::JoinDesync {
            what: "an edge lying in a face does not resolve",
        })?;
        if [a, b]
            .into_iter()
            .any(|v| touches_boundary_between(body, contacts, v, face))
        {
            return Err(BooleanError::EdgeInFace {
                edge,
                face,
                gap: EdgeInFaceGap::EndOnBoundary,
            });
        }
        let p = part(body, edge, face, tangent, tol)?;
        // A record's cells are in canonical order, the lesser first.
        for c in [Cell::Vertex(a), Cell::Vertex(b), Cell::Edge(edge)] {
            dropped.insert([c, Cell::Face(face)]);
        }
        twins.push((Cell::Edge(edge), Cell::Edge(p.parted.minted)));
        twins.extend(
            p.parted
                .copies
                .iter()
                .map(|&(copy, of)| (Cell::Vertex(of), Cell::Vertex(copy))),
        );
        if p.parted.divided.is_some() {
            divided.insert(face);
        }
        parted.push(p.parted);
    }
    if parted.is_empty() {
        return Ok((parted, contacts.clone()));
    }
    let worklist: Vec<EdgeKey> = parted.iter().flat_map(|p| [p.kept, p.minted]).collect();
    super::ops::describe_edges(body, worklist, &[], band, tol).map_err(super::ops::of_describe)?;
    let shells: Vec<crate::entity::ShellKey> = body
        .solids()
        .flat_map(|(_, solid)| solid.shells.clone())
        .collect();
    for shell in shells {
        body.movefac(shell)?;
    }
    let mut kept = ContactRecords::default();
    for row in contacts.rows() {
        let cells = row.record.cells();
        if dropped.contains(&cells) {
            continue;
        }
        // A face the parting divided holds a record's cell on one of its
        // pieces, which nothing here reads.
        if let Some(&face) = divided.iter().find(|&&f| cells.contains(&Cell::Face(f))) {
            let edge = parted
                .iter()
                .find(|p| p.face == face)
                .map_or(parted[0].kept, |p| p.kept);
            return Err(BooleanError::EdgeInFace {
                edge,
                face,
                gap: EdgeInFaceGap::Recorded,
            });
        }
        kept.push(Cited::new(row.record, row.cites.clone()));
        for &(of, twin) in &twins {
            let Some(k) = cells.iter().position(|&c| c == of) else {
                continue;
            };
            if let Ok(contact) = Contact::new(body, twin, cells[1 - k]) {
                kept.push(Cited::new(contact, row.cites.clone()));
            }
        }
    }
    Ok((parted, kept))
}

/// A face an edge lies in, and whether the edge runs on past it
/// ([`riders`]).
type Holding = (FaceKey, bool);

/// Every edge of `body` lying in faces it does not bound, with those
/// faces ([`part_riders`]'s reading), in edge order, each with whether
/// the edge runs on past it: an edge-on-face record names the pair and
/// the edge's midpoint lies in the face, but an end of the edge does not.
fn riders<T: Decide>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
) -> Result<Vec<(EdgeKey, Vec<Holding>)>, BooleanError> {
    let mut vf: BTreeSet<(VertexKey, FaceKey)> = BTreeSet::new();
    let mut ef: BTreeSet<(EdgeKey, FaceKey)> = BTreeSet::new();
    for row in contacts.rows() {
        match row.record.kind() {
            ContactKind::VertexFace(v, f) => {
                vf.insert((v, f));
            }
            ContactKind::EdgeFace(e, f) => {
                ef.insert((e, f));
            }
            _ => {}
        }
    }
    let on = |v: VertexKey| -> BTreeSet<FaceKey> {
        body.faces_of_vertex(v)
            .unwrap_or_default()
            .into_iter()
            .chain(vf.iter().filter(|&&(w, _)| w == v).map(|&(_, f)| f))
            .collect()
    };
    let mut out = Vec::new();
    for (e, _) in body.edges() {
        let Some((a, b)) = ends(body, e) else {
            continue;
        };
        if a == b {
            continue;
        }
        let Ok(sides) = crate::readback::edge_sides(body, e) else {
            continue;
        };
        let (fa, fb) = sides.faces();
        let both: BTreeSet<FaceKey> = on(a).intersection(&on(b)).copied().collect();
        let mut faces = both.clone();
        faces.extend(ef.iter().filter(|&&(g, _)| g == e).map(|&(_, f)| f));
        faces.remove(&fa);
        faces.remove(&fb);
        let mut holding = Vec::new();
        for f in faces {
            if lies_in(body, e, f, band)? {
                holding.push((f, !both.contains(&f)));
            }
        }
        if !holding.is_empty() {
            out.push((e, holding));
        }
    }
    Ok(out)
}

/// Whether edge `e`, both of whose ends lie on the plane face `f`, lies
/// in it: a line whose midpoint `f` holds (two points of a plane hold
/// the line between them), or a conic in `f`'s plane whose midpoint `f`
/// holds. A face that is no plane holds nothing here.
fn lies_in<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    f: FaceKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let Some(&geom::Surface::Plane { origin, normal, .. }) =
        body.get_face(f).and_then(|d| body.get_surface(d.surface))
    else {
        return Ok(false);
    };
    let Some(curve) = body
        .get_edge(e)
        .and_then(|d| body.get_curve_geom(d.curve))
        .and_then(|c| c.certified())
    else {
        return Ok(false);
    };
    let mid = match curve.carrier() {
        geom::Curve3::Line { .. } => curve.mid_point(),
        carrier @ (geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. }) => {
            let (t0, t1) = curve.params();
            match crate::splitting::plane_crossing_lane(carrier, t0, t1, origin, normal, band) {
                crate::splitting::PlaneCrossingLane::Conic(
                    crate::splitting::ConicPlaneMeet::Parallel { offset },
                ) => match crate::validate::decide(
                    "bool_conic_face_plane_offset",
                    Margin::of(offset),
                    band,
                ) {
                    Ok(Sign::Zero) => curve.mid_point(),
                    Ok(Sign::Positive | Sign::Negative) => return Ok(false),
                    Err(diag) => {
                        return Err(BooleanError::coincidence(
                            Coincide::EdgeOnPlane,
                            DeclarationRead::Moot,
                            diag,
                        ));
                    }
                },
                _ => return Ok(false),
            }
        }
        _ => return Ok(false),
    };
    Ok(matches!(piece_holds(body, f, mid, band)?, PieceRead::In))
}

/// Whether both faces of edge `e` leave its line on the material side of
/// the face `f` it lies in (`bool_rider_wall_side`): at the edge's
/// midpoint, each face's direction into itself off the line (its
/// outward normal crossed with its half's tangent: a face lies to the
/// left of its half) against `f`'s outward normal, levered at the
/// edge's chord. `Some` where both go in, with whether the face the
/// edge's plus half runs in is tangent to `f` along the line; `None`
/// where both go out (the edge touches `f` from outside its wedge).
///
/// # Errors
///
/// [`BooleanError::EdgeInFace`] ([`EdgeInFaceGap::Crossed`]) where they
/// go to opposite sides, or ([`EdgeInFaceGap::NoNormal`]) where a face
/// has no normal there; [`BooleanError::Escalated`] where a side, or a
/// normal, is in band.
fn walls_inside<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    f: FaceKey,
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    let refused = |gap| BooleanError::EdgeInFace {
        edge: e,
        face: f,
        gap,
    };
    let desync = || BooleanError::JoinDesync {
        what: "an edge lying in a face does not resolve",
    };
    let edge = body.get_edge(e).ok_or_else(desync)?;
    let curve = body
        .get_curve_geom(edge.curve)
        .and_then(|c| c.certified())
        .ok_or_else(desync)?;
    let (t0, t1) = curve.params();
    let mid = curve.mid_point();
    let tangent = curve
        .carrier()
        .deriv((t0 + t1) * T::from_f64(0.5))
        .normalize();
    let (a, b) = ends(body, e).ok_or_else(desync)?;
    let arm = vertex_point(body, a)
        .zip(vertex_point(body, b))
        .map(|(p, q)| p.distance(q))
        .ok_or_else(desync)?;
    let floor = crate::face_normal::face_outward_normal(body, f)
        .ok_or_else(desync)?
        .vec();
    let mut inside = [false; 2];
    let mut flat = [false; 2];
    for (k, (he, dir)) in [(edge.he_plus, tangent), (edge.he_minus, -tangent)]
        .into_iter()
        .enumerate()
    {
        let wall = loop_face(body, he).ok_or_else(desync)?;
        let normal = match crate::face_normal::face_outward_normal_at(body, wall, mid, band) {
            Ok(Some(n)) => n.vec(),
            Ok(None) => return Err(desync()),
            Err(crate::face_normal::NormalAtError::Escalated { decision, diag }) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::of_normal(decision),
                    diag,
                });
            }
            Err(_) => return Err(refused(EdgeInFaceGap::NoNormal)),
        };
        let into = normal.cross(dir);
        let sectors =
            |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
        inside[k] = match crate::validate::decide(
            "bool_rider_wall_side",
            Margin::levered(into.dot(floor), arm),
            band,
        )
        .map_err(sectors)?
        {
            Sign::Negative => true,
            Sign::Positive => false,
            // Tangent to the face along the line: the side its sheet
            // lies on is the side it bends to, toward its centre of
            // curvature (`bool_rider_wall_bend`), in metres.
            Sign::Zero => {
                flat[k] = true;
                let surface = body
                    .get_face(wall)
                    .and_then(|d| body.get_surface(d.surface))
                    .ok_or_else(desync)?;
                let toward = match *surface {
                    geom::Surface::Cylinder { origin, axis, .. } => {
                        origin + axis * (mid - origin).dot(axis) - mid
                    }
                    geom::Surface::Cone { apex, axis, .. } => {
                        apex + axis * (mid - apex).dot(axis) - mid
                    }
                    geom::Surface::Sphere { center, .. } => center - mid,
                    _ => return Err(refused(EdgeInFaceGap::Tangent)),
                };
                match crate::validate::decide_nonzero(
                    "bool_rider_wall_bend",
                    Margin::of(toward.dot(floor)),
                    band,
                )
                .map_err(sectors)?
                {
                    geom_core::k_stats::NonzeroSign::Negative => true,
                    geom_core::k_stats::NonzeroSign::Positive => false,
                }
            }
        };
    }
    match inside {
        [true, true] => Ok(Some(flat[0])),
        [false, false] => Ok(None),
        _ => Err(refused(EdgeInFaceGap::Crossed)),
    }
}

/// The surgery of one parting.
struct Parting {
    parted: Parted,
}

/// **Parts edge `edge` lying in face `face`** ([`part_riders`], module
/// docs for the sequence).
fn part<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    face: FaceKey,
    tangent: bool,
    tol: Tol,
) -> Result<Parting, BooleanError> {
    let refused = |gap| BooleanError::EdgeInFace { edge, face, gap };
    let desync = || BooleanError::JoinDesync {
        what: "an edge lying in a face does not resolve",
    };
    let data = body.get_edge(edge).ok_or_else(desync)?;
    let h = data.he_plus;
    let curve = body
        .get_curve_geom(data.curve)
        .and_then(|c| c.certified())
        .cloned();
    let (a, b) = ends(body, edge).ok_or_else(desync)?;
    let wall = loop_face(body, h).ok_or_else(desync)?;
    let surfaces = [face, wall].map(|f| body.get_face(f).map(|d| d.surface));
    let [Some(floor_surface), Some(wall_surface)] = surfaces else {
        return Err(desync());
    };
    let (pa, pb) = vertex_point(body, a)
        .zip(vertex_point(body, b))
        .ok_or_else(desync)?;
    let pair = geom_brep::SurfacePair::new(floor_surface, wall_surface);
    let draft = geom_brep::IntersectionDraft::of(curve.as_ref(), pa, pb);
    let spec = if tangent {
        let (carrier, (t0, t1)) = draft.span();
        geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::TangentIntersection {
                pair,
                witness: draft.witness,
            },
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
        }
    } else {
        draft.into_spec(pair)
    };
    let next = proven(&body.half_edges, h, EntityId::HalfEdge).next;
    let made = body.mef(
        MefSite::Chords { he1: h, he2: next },
        spec,
        FaceSurface::Inherit,
        tol,
    )?;
    // The two-edge face: `h` from `a` to `b`, the minted minus half back.
    let gusset = made.face;
    let tip_out = |v: VertexKey| if v == a { h } else { made.he_minus };
    let on_loops = |body: &Body<T>, v: VertexKey| -> Result<Vec<HalfEdgeKey>, BooleanError> {
        let leaving: Vec<HalfEdgeKey> = face_halves(body, face)
            .ok_or_else(desync)?
            .into_iter()
            .filter(|&he| proven(&body.half_edges, he, EntityId::HalfEdge).start == v)
            .collect();
        Ok(leaving)
    };
    let mut boundary: Vec<(VertexKey, HalfEdgeKey)> = Vec::new();
    for v in [a, b] {
        match on_loops(body, v)?.as_slice() {
            [] => {}
            [he] => boundary.push((v, *he)),
            _ => return Err(refused(EdgeInFaceGap::Shared)),
        }
    }
    let mut copies = Vec::new();
    let mut divided = None;
    match boundary.as_slice() {
        [] => {
            body.kfmrh_minting(face, gusset, tol)?;
        }
        [(v, out), rest @ ..] => {
            let made_v = body.mev_null(
                MevSite::Fan {
                    he1: *out,
                    he2: tip_out(*v),
                },
                crate::null::NewVertexSide::Above,
            )?;
            copies.push((made_v.vertex, *v));
            body.kef_minting(made_v.he_minus, tol)?;
            if let [(w, out_w)] = rest {
                let outer = body.get_face(face).ok_or_else(desync)?.outer;
                let made_w = body.mev_null(
                    MevSite::Fan {
                        he1: *out_w,
                        he2: tip_out(*w),
                    },
                    crate::null::NewVertexSide::Above,
                )?;
                copies.push((made_w.vertex, *w));
                let loop_of =
                    |he: HalfEdgeKey| proven(&body.half_edges, he, EntityId::HalfEdge).parent_loop;
                let (lp, lm) = (loop_of(made_w.he_plus), loop_of(made_w.he_minus));
                if lp == lm {
                    if lp != outer {
                        return Err(refused(EdgeInFaceGap::Pocket));
                    }
                    let sense = body.get_face(face).ok_or_else(desync)?.sense;
                    let ring = body
                        .kemr_minting(made_w.he_plus, made_w.he_minus, tol)?
                        .ring;
                    let piece = body.mfkrh_minting(ring, FaceSurface::Inherit, tol)?.face;
                    // The piece is the face's own region on its own chart,
                    // not a hole of it promoted: it keeps the face's side.
                    body.set_face_sense(piece, sense)?;
                    divided = Some(piece);
                } else {
                    let (ring, he) = if lp == outer {
                        (lm, made_w.he_minus)
                    } else {
                        (lp, made_w.he_plus)
                    };
                    body.mfkrh_minting(ring, FaceSurface::Inherit, tol)?;
                    body.kef_minting(he, tol)?;
                }
            }
        }
    }
    Ok(Parting {
        parted: Parted {
            kept: edge,
            minted: made.edge,
            face,
            divided,
            ends: (a, b),
            copies,
        },
    })
}

/// Whether a record puts vertex `v`, which no loop of `face` passes
/// through, on `face`'s boundary: inside an edge of it, or on another
/// vertex of it.
fn touches_boundary_between<T: Decide>(
    body: &Body<T>,
    contacts: &ContactRecords,
    v: VertexKey,
    face: FaceKey,
) -> bool {
    let Some(halves) = face_halves(body, face) else {
        return false;
    };
    let on_loop = |w: VertexKey| {
        halves
            .iter()
            .any(|&he| body.get_half_edge(he).is_some_and(|h| h.start == w))
    };
    if on_loop(v) {
        return false;
    }
    contacts.rows().iter().any(|row| match row.record.kind() {
        ContactKind::VertexEdge(x, g) if x == v => halves
            .iter()
            .any(|&he| body.get_half_edge(he).is_some_and(|h| h.edge == g)),
        ContactKind::VertexVertex(x, y) if x == v => on_loop(y),
        ContactKind::VertexVertex(x, y) if y == v => on_loop(x),
        _ => false,
    })
}

/// The face the loop of `he` bounds.
fn loop_face<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Option<FaceKey> {
    let l: LoopKey = body.get_half_edge(he)?.parent_loop;
    Some(body.get_loop(l)?.face)
}

/// Every half-edge of `face`'s loops.
fn face_halves<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec<HalfEdgeKey>> {
    let data = body.get_face(face)?;
    let mut out = Vec::new();
    for l in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        if let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(l)?.boundary {
            out.extend(body.loop_cycle(first)?);
        }
    }
    Some(out)
}

/// An edge's start and end vertices.
fn ends<T: Decide>(body: &Body<T>, e: EdgeKey) -> Option<(VertexKey, VertexKey)> {
    let he = body.get_edge(e)?.he_plus;
    Some((body.get_half_edge(he)?.start, body.half_edge_end(he)?))
}

/// A vertex's point.
fn vertex_point<T: Decide>(body: &Body<T>, v: VertexKey) -> Option<geom_core::Point3<T>> {
    body.get_vertex(v)
        .and_then(|d| body.get_point(d.point))
        .copied()
}
