//! Sweep-op name emission (spec D2): a mechanical zip of the sweep
//! emitters' output maps (`Extruded`, `Revolved` — already indexed by
//! the profile's canonical combinatorial identities) with the role
//! vocabulary. Rim edges and cap vertices are derived by
//! COMBINATORIAL adjacency between emitted anchors (unique shared
//! edge; endpoint intersection) — exact wiring facts, never geometric
//! matching.

use std::sync::Arc;

use geom_core::Decide;
use sweep::Extruded;
use topo::{Body, EdgeKey, VertexKey};

use super::emit::{NamingError, edge_ends, ent, name1, unique_shared_edge};
use super::role::{CapEnd, EntityKind, MeridianEnd, ProfileEdgeRef, ProfileVertexRef, RoleSeg};
use super::table::{EntityKey, NameTable};
use crate::node::RecipeNodeId;

/// The typed refusal when an OFF-AXIS meridian vertex stays
/// unresolved. Not a shape limitation: an off-axis vertex always has
/// a rim, and a meridian edge at it (partial chains are total; the
/// full case omits only on-axis segments, whose endpoints are poles),
/// so the rim ∩ meridian anchor always runs — reaching this means the
/// two share both endpoints or none, i.e. the built topology
/// contradicts the key bundle. Refused typed, never guessed. Poles do
/// not come this way at all: they are looked up in the sweep's
/// `poles` export.
const UNRESOLVED: NamingError = NamingError::Emission {
    what: "revolve meridian vertex unresolved: rim and meridian are not incident",
};

/// Truncating-safe index cast (loop/segment counts are far below
/// `u32::MAX`; a table this large is unrepresentable upstream).
fn ix(i: usize) -> u32 {
    u32::try_from(i).unwrap_or(u32::MAX)
}

/// Names every boundary entity of an extrusion (spec D2's extrude
/// vocabulary): caps, laterals, rims, struts, cap vertices, and the
/// output body.
pub(crate) fn name_extrude<T: Decide>(
    node: RecipeNodeId,
    built: &Extruded<T>,
) -> Result<Arc<NameTable>, NamingError> {
    name_swept_topology(
        node,
        &built.body,
        built.top,
        built.bottom,
        &built.side_faces,
        &built.strut_edges,
    )
}

/// Names every boundary entity of a loft/sweep body (M6-3): the
/// Lofted bundle is the Extruded one with seam edges where the struts
/// were, so the SAME combinatorial zip applies — caps, laterals,
/// rims, seams-as-lateral-edges, cap vertices, output body.
pub(crate) fn name_loft<T: Decide>(
    node: RecipeNodeId,
    built: &sweep::Lofted<T>,
) -> Result<Arc<NameTable>, NamingError> {
    name_swept_topology(
        node,
        &built.body,
        built.top,
        built.bottom,
        &built.side_faces,
        &built.seam_edges,
    )
}

/// The shared swept-solid zip (extrude and loft produce the same
/// combinatorial shape: two caps, per-(loop, segment) walls, per-
/// (loop, vertex) lateral edges).
fn name_swept_topology<T: Decide>(
    node: RecipeNodeId,
    body: &Body<T>,
    top: topo::FaceKey,
    bottom: topo::FaceKey,
    side_faces: &[Vec<topo::FaceKey>],
    lateral_edges: &[Vec<EdgeKey>],
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    for (end, face) in [(CapEnd::Top, top), (CapEnd::Bottom, bottom)] {
        t.insert(
            name1(EntityKind::Face, node, RoleSeg::Cap(end)),
            ent(0, EntityKey::Face(face)),
        )?;
    }

    // Laterals + rims, indexed by the emitter's canonical (loop,
    // segment) maps.
    for (l, segs) in side_faces.iter().enumerate() {
        for (s, &wall) in segs.iter().enumerate() {
            let pe = ProfileEdgeRef {
                loop_index: ix(l),
                segment: ix(s),
            };
            t.insert(
                name1(EntityKind::Face, node, RoleSeg::Lateral(pe)),
                ent(0, EntityKey::Face(wall)),
            )?;
            for (end, cap) in [(CapEnd::Top, top), (CapEnd::Bottom, bottom)] {
                let rim = unique_shared_edge(body, wall, cap)?;
                t.insert(
                    name1(EntityKind::Edge, node, RoleSeg::RimEdge(end, pe)),
                    ent(0, EntityKey::Edge(rim)),
                )?;
            }
        }
    }

    // Struts + cap vertices. Strut `j` joins the walls of segments
    // `j − 1` and `j`; its cap-`end` endpoint is the unique common
    // vertex with the `end` rim of segment `j`.
    for (l, struts) in lateral_edges.iter().enumerate() {
        for (j, &strut) in struts.iter().enumerate() {
            let pv = ProfileVertexRef {
                loop_index: ix(l),
                vertex: ix(j),
            };
            t.insert(
                name1(EntityKind::Edge, node, RoleSeg::LateralEdge(pv)),
                ent(0, EntityKey::Edge(strut)),
            )?;
            let (s0, s1) = edge_ends(body, strut)?;
            for (end, cap) in [(CapEnd::Top, top), (CapEnd::Bottom, bottom)] {
                let wall = side_faces[l][j];
                let rim = unique_shared_edge(body, wall, cap)?;
                let (r0, r1) = edge_ends(body, rim)?;
                let vtx = common_vertex((s0, s1), (r0, r1)).ok_or(NamingError::Emission {
                    what: "extrude cap vertex: strut and rim share no endpoint",
                })?;
                t.insert(
                    name1(EntityKind::Vertex, node, RoleSeg::CapVertex(end, pv)),
                    ent(0, EntityKey::Vertex(vtx)),
                )?;
            }
        }
    }

    super::emit::check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// Names every boundary entity of a revolution (spec D2: the M2
/// band/pole/seam taxonomy, read off the `Revolved` maps).
///
/// Vertex resolution: off-axis meridian vertices anchor as
/// rim ∩ meridian endpoint intersections, then eliminate along the
/// meridian chains (an edge with one resolved endpoint resolves the
/// other); on-axis (pole) vertices are LOOKED UP in the sweep's
/// `poles` export — a construction record, since the builders know
/// each pole by the operator that minted it. A pole absent from the
/// export is a vertex the sweep deleted (the full case's omitted axis
/// run), so nothing is named for it; had one been wrongly omitted,
/// `check_total` would catch the unnamed body vertex.
pub(crate) fn name_revolve<T: Decide>(
    node: RecipeNodeId,
    built: &sweep::Revolved<T>,
) -> Result<Arc<NameTable>, NamingError> {
    let body = &built.body;
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    let pe = |l: usize, s: usize| ProfileEdgeRef {
        loop_index: ix(l),
        segment: ix(s),
    };
    let pv = |l: usize, v: usize| ProfileVertexRef {
        loop_index: ix(l),
        vertex: ix(v),
    };
    let insert_face = |t: &mut NameTable, seg: RoleSeg, f| {
        t.insert(
            name1(EntityKind::Face, node, seg),
            ent(0, EntityKey::Face(f)),
        )
    };
    let insert_edge = |t: &mut NameTable, seg: RoleSeg, e| {
        t.insert(
            name1(EntityKind::Edge, node, seg),
            ent(0, EntityKey::Edge(e)),
        )
    };
    let insert_vertex = |t: &mut NameTable, seg: RoleSeg, v| {
        t.insert(
            name1(EntityKind::Vertex, node, seg),
            ent(0, EntityKey::Vertex(v)),
        )
    };

    for (l, segs) in built.walls.iter().enumerate() {
        for (s, wall) in segs.iter().enumerate() {
            if let Some(f) = wall {
                insert_face(&mut t, RoleSeg::Band(pe(l, s)), *f)?;
            }
        }
    }
    for (l, vs) in built.rims.iter().enumerate() {
        for (v, rim) in vs.iter().enumerate() {
            if let Some(e) = rim {
                insert_edge(&mut t, RoleSeg::BandRim(pv(l, v)), *e)?;
            }
        }
    }

    match &built.kind {
        sweep::RevolvedKind::Partial {
            start_cap,
            end_cap,
            start_meridians,
            end_meridians,
        } => {
            insert_face(&mut t, RoleSeg::RevolveCap(MeridianEnd::Start), *start_cap)?;
            insert_face(&mut t, RoleSeg::RevolveCap(MeridianEnd::End), *end_cap)?;
            for (l, (ss, es)) in start_meridians.iter().zip(end_meridians).enumerate() {
                for (s, (&se, &ee)) in ss.iter().zip(es).enumerate() {
                    if se == ee {
                        // The shared axis edge of an on-axis segment.
                        insert_edge(&mut t, RoleSeg::AxisEdge(pe(l, s)), se)?;
                    } else {
                        insert_edge(&mut t, RoleSeg::Meridian(MeridianEnd::Start, pe(l, s)), se)?;
                        insert_edge(&mut t, RoleSeg::Meridian(MeridianEnd::End, pe(l, s)), ee)?;
                    }
                }
                let rims = &built.rims[l];
                let start = resolve_chain(body, ss, rims)?;
                let end = resolve_chain(body, es, rims)?;
                for v in 0..rims.len() {
                    if rims[v].is_some() {
                        insert_vertex(
                            &mut t,
                            RoleSeg::MeridianVertex(MeridianEnd::Start, pv(l, v)),
                            start[v].ok_or(UNRESOLVED)?,
                        )?;
                        insert_vertex(
                            &mut t,
                            RoleSeg::MeridianVertex(MeridianEnd::End, pv(l, v)),
                            end[v].ok_or(UNRESOLVED)?,
                        )?;
                    } else if let Some(p) = built.poles[l][v] {
                        // Pole: the same physical vertex in both chains.
                        insert_vertex(&mut t, RoleSeg::Pole(pv(l, v)), p)?;
                    }
                }
            }
        }
        sweep::RevolvedKind::Full {
            meridians,
            pi_walls,
            pi_meridians,
            pi_rims,
        } => {
            let wire = pi_walls.iter().any(Option::is_some);
            for (l, ms) in meridians.iter().enumerate() {
                for (s, m) in ms.iter().enumerate() {
                    if let Some(e) = m {
                        insert_edge(&mut t, RoleSeg::Meridian(MeridianEnd::Seam, pe(l, s)), *e)?;
                    }
                }
            }
            for (s, w) in pi_walls.iter().enumerate() {
                if let Some(f) = w {
                    insert_face(&mut t, RoleSeg::BandPi(pe(0, s)), *f)?;
                }
            }
            for (s, m) in pi_meridians.iter().enumerate() {
                if let Some(e) = m {
                    insert_edge(&mut t, RoleSeg::Meridian(MeridianEnd::Pi, pe(0, s)), *e)?;
                }
            }
            for (v, r) in pi_rims.iter().enumerate() {
                if let Some(e) = r {
                    insert_edge(&mut t, RoleSeg::BandRimPi(pv(0, v)), *e)?;
                }
            }
            let rims = &built.rims[0];
            if wire {
                let seam_chain: Vec<_> = meridians[0].clone();
                let pi_chain: Vec<_> = pi_meridians.clone();
                let seam = resolve_chain_opt(body, &seam_chain, rims)?;
                let pi = resolve_chain_opt(body, &pi_chain, rims)?;
                for v in 0..rims.len() {
                    if rims[v].is_some() {
                        insert_vertex(
                            &mut t,
                            RoleSeg::MeridianVertex(MeridianEnd::Seam, pv(0, v)),
                            seam[v].ok_or(UNRESOLVED)?,
                        )?;
                        insert_vertex(
                            &mut t,
                            RoleSeg::MeridianVertex(MeridianEnd::Pi, pv(0, v)),
                            pi[v].ok_or(UNRESOLVED)?,
                        )?;
                    } else if let Some(p) = built.poles[0][v] {
                        insert_vertex(&mut t, RoleSeg::Pole(pv(0, v)), p)?;
                    }
                }
            } else {
                // Lamina: full-period rim self-loops — the rim's
                // (doubled) endpoint IS the meridian vertex.
                for (v, r) in rims.iter().enumerate() {
                    let e = r.ok_or(NamingError::Emission {
                        what: "lamina full revolve with an on-axis vertex",
                    })?;
                    let (a, b) = edge_ends(body, e)?;
                    if a != b {
                        return Err(NamingError::Emission {
                            what: "lamina rim is not a self-loop",
                        });
                    }
                    insert_vertex(
                        &mut t,
                        RoleSeg::MeridianVertex(MeridianEnd::Seam, pv(0, v)),
                        a,
                    )?;
                }
            }
            // Hole loops (a holed full revolve's cavity shells) are
            // always lamina-shaped, off-axis by validated containment:
            // every rim is a full-period self-loop whose (doubled)
            // endpoint is the meridian vertex.
            for l in 1..built.rims.len() {
                for (v, r) in built.rims[l].iter().enumerate() {
                    let e = r.ok_or(NamingError::Emission {
                        what: "full-revolve hole with an on-axis vertex",
                    })?;
                    let (a, b) = edge_ends(body, e)?;
                    if a != b {
                        return Err(NamingError::Emission {
                            what: "hole cavity rim is not a self-loop",
                        });
                    }
                    insert_vertex(
                        &mut t,
                        RoleSeg::MeridianVertex(MeridianEnd::Seam, pv(l, v)),
                        a,
                    )?;
                }
            }
        }
    }

    super::emit::check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// Resolves the per-vertex copies along one meridian chain (chain
/// edge `s` runs between the copies of profile vertices `s` and
/// `s + 1`, cyclically): off-axis vertices anchor by rim ∩ meridian
/// endpoint intersection, the rest by elimination to fixpoint.
fn resolve_chain_opt<T: Decide>(
    body: &Body<T>,
    chain: &[Option<EdgeKey>],
    rims: &[Option<EdgeKey>],
) -> Result<Vec<Option<VertexKey>>, NamingError> {
    let n = chain.len();
    if n != rims.len() || n < 2 {
        return Err(NamingError::Emission {
            what: "revolve chain/rim length mismatch",
        });
    }
    let mut out: Vec<Option<VertexKey>> = vec![None; n];
    for v in 0..n {
        if let (Some(rim), Some(m)) = (rims[v], chain[v]) {
            out[v] = common_vertex(edge_ends(body, rim)?, edge_ends(body, m)?);
        }
    }
    // Elimination to fixpoint: an edge with one resolved endpoint
    // resolves the other (bounded by n rounds).
    for _ in 0..n {
        let mut progressed = false;
        for s in 0..n {
            let Some(e) = chain[s] else { continue };
            let (a, b) = edge_ends(body, e)?;
            let s1 = (s + 1) % n;
            match (out[s], out[s1]) {
                (Some(k), None) => {
                    out[s1] = Some(if a == k { b } else { a });
                    progressed = true;
                }
                (None, Some(k)) => {
                    out[s] = Some(if a == k { b } else { a });
                    progressed = true;
                }
                _ => {}
            }
        }
        if !progressed {
            break;
        }
    }
    Ok(out)
}

/// [`resolve_chain_opt`] over a total chain (partial revolve).
fn resolve_chain<T: Decide>(
    body: &Body<T>,
    chain: &[EdgeKey],
    rims: &[Option<EdgeKey>],
) -> Result<Vec<Option<VertexKey>>, NamingError> {
    let opts: Vec<Option<EdgeKey>> = chain.iter().copied().map(Some).collect();
    resolve_chain_opt(body, &opts, rims)
}

/// The unique vertex two endpoint pairs share (`None` when disjoint
/// or when both endpoints coincide — callers treat as an emission
/// inconsistency).
pub(crate) fn common_vertex(
    a: (VertexKey, VertexKey),
    b: (VertexKey, VertexKey),
) -> Option<VertexKey> {
    let hit0 = a.0 == b.0 || a.0 == b.1;
    let hit1 = a.1 == b.0 || a.1 == b.1;
    match (hit0, hit1) {
        (true, false) => Some(a.0),
        (false, true) => Some(a.1),
        _ => None,
    }
}
