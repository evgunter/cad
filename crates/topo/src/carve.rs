//! **Carving a sub-body out of a body**: a clone of it with some of its
//! shells, or every solid but one, removed by arena surgery, every kept
//! entity keeping its key.

use slotmap::SecondaryMap;

use crate::attach::Named;
use crate::body::Body;
use crate::entity::{EntityId, LoopBoundary, ShellKey, SolidKey, VertexKey};
use crate::live::BoundaryMember;
use geom_core::Decide;

/// **A body the carve cannot cut along its shells**: a kept entity names
/// one the carve drops, so the source was mid-operation and not at rest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Corrupt;

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
/// checks it and answers [`Corrupt`] where it fails.
pub(crate) fn carve<T: Decide>(
    src: &Body<T>,
    solid: SolidKey,
    keep: &[ShellKey],
) -> Result<Body<T>, Corrupt> {
    let mut body = src.clone();
    let corrupt = || Corrupt;

    let all: Vec<ShellKey> = body.shells_of_solid(solid).ok_or_else(corrupt)?.to_vec();
    let drop: Vec<ShellKey> = all.iter().copied().filter(|s| !keep.contains(s)).collect();

    // Collect the dropped entity sets (deterministic list walks).
    let mut faces = Vec::new();
    let mut loops = Vec::new();
    let mut hes = Vec::new();
    let mut edges: Vec<crate::entity::EdgeKey> = Vec::new();
    let mut vertices: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for &shell in &drop {
        let shell_data = body.get_shell(shell).ok_or_else(corrupt)?;
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

    let dropped_hes: SecondaryMap<crate::entity::HalfEdgeKey, ()> =
        hes.iter().map(|&he| (he, ())).collect();
    let dropped_loops: SecondaryMap<crate::entity::LoopKey, ()> =
        loops.iter().map(|&l| (l, ())).collect();
    for (he, he_data) in &body.half_edges {
        if dropped_hes.contains_key(he) {
            continue;
        }
        let edge = body.get_edge(he_data.edge).ok_or_else(corrupt)?;
        if vertices.contains_key(he_data.start)
            || dropped_hes.contains_key(edge.he_plus)
            || dropped_hes.contains_key(edge.he_minus)
        {
            return Err(corrupt());
        }
    }
    for (l, loop_data) in &body.loops {
        if let LoopBoundary::Empty { vertex } = loop_data.boundary
            && !dropped_loops.contains_key(l)
            && vertices.contains_key(vertex)
        {
            return Err(corrupt());
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

/// **`src`'s solid `keep` alone**: a clone with every other solid and
/// everything under it removed ([`carve`]), every kept entity keeping its
/// key.
///
/// # Errors
///
/// As [`carve`].
pub(crate) fn solid_alone<T: Decide>(src: &Body<T>, keep: SolidKey) -> Result<Body<T>, Corrupt> {
    let mut body = src.clone();
    for other in src.solids().map(|(s, _)| s).filter(|&s| s != keep) {
        body = carve(&body, other, &[])?;
        body.solids.remove(other);
        body.solid_provenance.remove(other);
    }
    Ok(body)
}
