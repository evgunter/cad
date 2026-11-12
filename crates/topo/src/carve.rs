//! **Carving shells out of a solid**: the sub-body a set of a solid's
//! shells spans, as a body of its own ([`carve`]), and the one solid an
//! operation's operand is read as ([`single_solid`]). Arena surgery
//! over a [`Body`], refused in its own type ([`CarveError`]).

use geom_core::Decide;
use slotmap::SecondaryMap;

use crate::attach::Named;
use crate::body::Body;
use crate::entity::{
    EdgeKey, EntityId, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey, VertexKey,
};
use crate::live::BoundaryMember;

/// Why [`carve`] or [`single_solid`] refused. The body it read is an
/// operation's scratch state, so every arm is a kernel defect in the
/// operation that built that state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarveError {
    /// The body holds `solids` solids where the operation reads one.
    NotOneSolid {
        /// How many it holds.
        solids: usize,
    },
    /// The solid carved from, or a record reached from it, does not
    /// resolve.
    Unresolved(EntityId),
    /// A kept half-edge starts at a vertex of a dropped shell.
    KeptStartsAtDropped {
        /// The kept half-edge.
        half_edge: HalfEdgeKey,
        /// The dropped vertex it starts at.
        vertex: VertexKey,
    },
    /// A kept half-edge's edge holds a half-edge of a dropped shell.
    KeptSharesEdge {
        /// The kept half-edge.
        half_edge: HalfEdgeKey,
        /// The edge it shares.
        edge: EdgeKey,
    },
    /// A kept lone-vertex loop holds a vertex of a dropped shell.
    KeptLoopHoldsDropped {
        /// The kept loop.
        lone: LoopKey,
        /// The dropped vertex it holds.
        vertex: VertexKey,
    },
}

impl core::fmt::Display for CarveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotOneSolid { solids } => {
                write!(f, "the body holds {solids} solids where one is read")
            }
            Self::Unresolved(entity) => write!(f, "{entity} does not resolve"),
            Self::KeptStartsAtDropped { half_edge, vertex } => write!(
                f,
                "kept half-edge {half_edge:?} starts at vertex {vertex:?} of a dropped shell"
            ),
            Self::KeptSharesEdge { half_edge, edge } => write!(
                f,
                "kept half-edge {half_edge:?} shares edge {edge:?} with a half-edge of a \
                 dropped shell"
            ),
            Self::KeptLoopHoldsDropped { lone, vertex } => write!(
                f,
                "kept lone-vertex loop {lone:?} holds vertex {vertex:?} of a dropped shell"
            ),
        }
    }
}

impl std::error::Error for CarveError {}

/// The operand's single solid. The pipelines read every operand as one
/// solid (`split` and the boolean hand them over through
/// [`Body::merge_all_solids`]), so any other count is a desync.
///
/// # Errors
///
/// [`CarveError::NotOneSolid`].
pub(crate) fn single_solid<T: Decide>(body: &Body<T>) -> Result<SolidKey, CarveError> {
    let mut it = body.solids();
    match (it.next(), it.next()) {
        (Some((k, _)), None) => Ok(k),
        _ => Err(CarveError::NotOneSolid {
            solids: body.solids().count(),
        }),
    }
}

/// Carves the sub-body spanned by `keep` shells out of `src`: a clone
/// with every other shell's entities removed and orphaned geometry
/// swept. Kept entities keep their keys (lineage-scoped identity —
/// deterministic, replay-stable).
///
/// **Every surviving link resolves.** The removal is arena surgery, not
/// an Euler operator, so it keeps the links by removing only records
/// no kept record names: no kept half-edge starts at a dropped vertex
/// or shares an edge with a dropped half-edge, and no kept lone-vertex
/// loop holds a dropped vertex. That is tier-1 pass 6's "no split
/// orbits" on a body at rest; `src` may be mid-operation, so the carve
/// checks it and refuses where it fails.
///
/// # Errors
///
/// [`CarveError`]: `solid` or a record it reaches does not resolve, or
/// a kept record names a dropped one.
pub(crate) fn carve<T: Decide>(
    src: &Body<T>,
    solid: SolidKey,
    keep: &[ShellKey],
) -> Result<Body<T>, CarveError> {
    let mut body = src.clone();
    let all: Vec<ShellKey> = body
        .shells_of_solid(solid)
        .ok_or(CarveError::Unresolved(EntityId::Solid(solid)))?
        .to_vec();
    let drop: Vec<ShellKey> = all.iter().copied().filter(|s| !keep.contains(s)).collect();

    // Collect the dropped entity sets (deterministic list walks).
    let mut faces = Vec::new();
    let mut loops = Vec::new();
    let mut hes = Vec::new();
    let mut edges: Vec<EdgeKey> = Vec::new();
    let mut vertices: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for &shell in &drop {
        let shell_data = body
            .get_shell(shell)
            .ok_or(CarveError::Unresolved(EntityId::Shell(shell)))?;
        for &face in &shell_data.faces {
            faces.push(face);
            let holder = EntityId::Shell(shell);
            let view = body.linked_face(face, holder, "faces");
            for (l, members, _) in view.boundary_by_loop() {
                loops.push(l);
                for member in members {
                    vertices.insert(member.vertex(), ());
                    if let BoundaryMember::Edge { he, ek, edge, .. } = member {
                        hes.push(he);
                        // Each edge is claimed by its he_plus only (one
                        // removal per edge).
                        if edge.he_plus == he {
                            edges.push(ek);
                        }
                    }
                }
            }
        }
    }

    let dropped_hes: SecondaryMap<HalfEdgeKey, ()> = hes.iter().map(|&he| (he, ())).collect();
    let dropped_loops: SecondaryMap<LoopKey, ()> = loops.iter().map(|&l| (l, ())).collect();
    for (he, he_data) in &body.half_edges {
        if dropped_hes.contains_key(he) {
            continue;
        }
        let edge = body
            .get_edge(he_data.edge)
            .ok_or(CarveError::Unresolved(EntityId::Edge(he_data.edge)))?;
        if vertices.contains_key(he_data.start) {
            return Err(CarveError::KeptStartsAtDropped {
                half_edge: he,
                vertex: he_data.start,
            });
        }
        if dropped_hes.contains_key(edge.he_plus) || dropped_hes.contains_key(edge.he_minus) {
            return Err(CarveError::KeptSharesEdge {
                half_edge: he,
                edge: he_data.edge,
            });
        }
    }
    for (l, loop_data) in &body.loops {
        if let LoopBoundary::Empty { vertex } = loop_data.boundary
            && !dropped_loops.contains_key(l)
            && vertices.contains_key(vertex)
        {
            return Err(CarveError::KeptLoopHoldsDropped { lone: l, vertex });
        }
    }

    // ---- Removal (crate-internal arena surgery; deterministic). ----
    let Some(solid_data) = body.solids.get_mut(solid) else {
        unreachable!("carve: `solid` resolved above and nothing has been removed yet")
    };
    solid_data.shells.retain(|s| keep.contains(s));
    for &shell in &drop {
        body.shells.remove(shell);
        body.shell_provenance.remove(shell);
    }
    for &face in &faces {
        body.faces.remove(face);
        body.face_provenance.remove(face);
        body.null_faces.remove(face);
    }
    for &l in &loops {
        body.loops.remove(l);
        body.loop_provenance.remove(l);
    }
    for &he in &hes {
        body.half_edges.remove(he);
        body.half_edge_provenance.remove(he);
    }
    for &edge in &edges {
        body.edges.remove(edge);
        body.edge_provenance.remove(edge);
    }
    let vertex_keys: Vec<VertexKey> = vertices.keys().collect();
    for &v in &vertex_keys {
        body.vertices.remove(v);
        body.vertex_provenance.remove(v);
    }

    // ---- Orphan geometry sweep (tier-1 pass 8 hygiene): keep only
    // points/curves/surfaces still referenced by survivors. ----
    let mut live_points: SecondaryMap<crate::geometry::PointKey, ()> = SecondaryMap::new();
    for (_, v) in body.vertices() {
        live_points.insert(v.point, ());
    }
    let orphan_points: Vec<_> = body
        .points
        .keys()
        .filter(|k| !live_points.contains_key(*k))
        .collect();
    for k in orphan_points {
        body.points.remove(k);
    }
    let mut live_curves: SecondaryMap<crate::geometry::CurveKey, ()> = SecondaryMap::new();
    for (_, e) in body.edges() {
        live_curves.insert(e.curve, ());
    }
    let orphan_curves: Vec<_> = body
        .curves
        .keys()
        .filter(|k| !live_curves.contains_key(*k))
        .collect();
    for k in orphan_curves {
        body.curves.remove(k);
    }
    let mut live_surfaces: SecondaryMap<crate::geometry::SurfaceKey, ()> = SecondaryMap::new();
    for (_, face) in body.faces() {
        live_surfaces.insert(face.surface, ());
    }
    // Description references keep surfaces alive exactly like faces do
    // (the `remove_surface_if_orphaned` rule): an `Intersection`/`Seam`
    // description on a surviving edge must never dangle (extrude-built
    // operands carry them — M3 PR 5).
    for (_, curve) in body.curves() {
        for s in Named::of(curve).keys() {
            live_surfaces.insert(s, ());
        }
    }
    let orphan_surfaces: Vec<_> = body
        .surfaces
        .keys()
        .filter(|k| !live_surfaces.contains_key(*k))
        .collect();
    for k in orphan_surfaces {
        body.surfaces.remove(k);
    }
    Ok(body)
}
