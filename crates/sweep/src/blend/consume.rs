//! **The carve that takes a feature wholly inside a convex ruled band's
//! material** — predicate 2's inside verdict (`reach::Consumed`), built.
//!
//! A feature is a connected set of faces, joined by the edges they
//! share. The carve takes each whole, as a Boolean takes a feature
//! wholly inside its tool, or refuses the request `WhollyInside` before
//! any mutation ([`plan`]). The shape it builds is a through-feature
//! between the band's two caps: faces read inside the prism the band
//! removes, bounded only by each other and by one cycle on each cap that
//! the cap's meter read inside its sliver. Its faces, edges and vertices
//! go; the two cycles leave their caps, and the slivers they lay in go
//! with the cut-off.
//!
//! The walk, per feature, on the clone before the band is carved
//! ([`consume`]): the cycle on one cap is promoted to a plug face
//! (`mfkrh`, which ends the feature's hole); the plug and the feature's
//! faces are merged across their shared edges into one face (`kef`),
//! and the edges left inside it, a tree hanging from the other cap's
//! cycle, are retired leaf first (`kev`); that face is merged into the
//! other cap across one of the cycle's edges (`kef`), whose remaining
//! edges are retired the same way down to a lone vertex; and that vertex
//! is joined to the cap's outer cycle by a strut (`mekr`) and retired
//! with it (`kev`).

use geom_core::{Decide, Tol};
use topo::{
    Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, LoopBoundary, LoopKey, MekrSite,
    VertexKey,
};

use super::BlendError;
use super::naming::BlendNaming;
use super::reach::Consumed;
use super::surgery::{SourceFaces, face_of_half, not_intact, op};

/// One feature the carve takes, read off the source.
pub(super) struct Feature {
    /// Its faces, sorted.
    faces: Vec<FaceKey>,
    /// The cycle that stays on its cap until the merge: the cap, the
    /// cycle.
    kept: (FaceKey, LoopKey),
    /// The cycle promoted to the plug.
    plugged: LoopKey,
    /// Every edge and vertex of its faces, sorted.
    edges: Vec<EdgeKey>,
    vertices: Vec<VertexKey>,
}

impl Feature {
    /// The faces the carve kills, of every feature.
    pub(super) fn faces(features: &[Self]) -> Vec<FaceKey> {
        let mut all: Vec<FaceKey> = features.iter().flat_map(|f| f.faces.clone()).collect();
        all.sort_unstable();
        all
    }
}

/// Each half-edge of `lp`'s cycle; a lone-vertex cycle has none.
fn halves<T: Decide>(body: &Body<T>, lp: LoopKey) -> Result<Vec<HalfEdgeKey>, BlendError> {
    let l = body
        .get_loop(lp)
        .ok_or_else(|| not_intact(EntityId::Loop(lp), "a consumed feature's cycle"))?;
    match l.boundary {
        LoopBoundary::Empty { .. } => Ok(Vec::new()),
        LoopBoundary::Cycle { first } => body
            .loop_cycle(first)
            .ok_or_else(|| not_intact(EntityId::Loop(lp), "a consumed feature's cycle")),
    }
}

/// Every half-edge of `face`'s cycles.
fn face_halves<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<Vec<HalfEdgeKey>, BlendError> {
    let f = body
        .get_face(face)
        .ok_or_else(|| not_intact(EntityId::Face(face), "a consumed feature's face"))?;
    let mut all = Vec::new();
    for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        all.extend(halves(body, lp)?);
    }
    Ok(all)
}

/// `he`'s mate, its loop and its face.
fn across<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> Result<(HalfEdgeKey, LoopKey, FaceKey), BlendError> {
    let unread = || not_intact(EntityId::HalfEdge(he), "a consumed feature's half-edge");
    let m = body.mate(he).ok_or_else(unread)?;
    let lp = body.get_half_edge(m).ok_or_else(unread)?.parent_loop;
    let f = body.get_loop(lp).ok_or_else(unread)?.face;
    Ok((m, lp, f))
}

/// **Group what a request consumes into features**, and refuse one the
/// carve does not take: a face joined to a face outside the feature
/// other than through a cycle its cap read inside, a cycle not wholly
/// bounded by one feature, or a feature that is not a through-feature
/// between two caps.
///
/// Sound because the verdict is inside only when every face and cycle
/// not clear read inside: an inside face shares no edge with a clear
/// one (the edge would lie in both), so a feature's faces were all read,
/// and the check here is that they are all inside, which reading them
/// does not say of a face grouped with them.
///
/// # Errors
///
/// The request's [`BlendError::WhollyInside`] for a feature it does not
/// take; [`BlendError::BodyNotIntact`] for a body the walk cannot read.
pub(super) fn plan<T: Decide>(
    body: &Body<T>,
    consumed: &Consumed<T>,
) -> Result<Vec<Feature>, BlendError> {
    if consumed.is_empty() {
        return Ok(Vec::new());
    }
    let refuse = || {
        consumed.refusal.clone().unwrap_or_else(|| {
            not_intact(
                EntityId::Face(FaceKey::default()),
                "an inside verdict with no refusal",
            )
        })
    };
    let faces = &consumed.faces;
    let ring_of = |lp: LoopKey| consumed.rings.iter().find(|r| r.ring == lp);
    // Union-find over the faces, by shared edge.
    let mut parent: Vec<usize> = (0..faces.len()).collect();
    fn root(parent: &mut [usize], mut i: usize) -> usize {
        while parent[i] != i {
            parent[i] = parent[parent[i]];
            i = parent[i];
        }
        i
    }
    let mut touches: Vec<(usize, LoopKey)> = Vec::new();
    for (i, &face) in faces.iter().enumerate() {
        for he in face_halves(body, face)? {
            let (_, lp, other) = across(body, he)?;
            if let Ok(j) = faces.binary_search(&other) {
                let (a, b) = (root(&mut parent, i), root(&mut parent, j));
                parent[a] = b;
            } else if ring_of(lp).is_some_and(|r| r.cap == other) {
                touches.push((i, lp));
            } else {
                return Err(refuse());
            }
        }
    }
    let mut features = Vec::new();
    let mut seen: Vec<usize> = Vec::new();
    for i in 0..faces.len() {
        let r = root(&mut parent, i);
        if seen.contains(&r) {
            continue;
        }
        seen.push(r);
        let members: Vec<usize> = (0..faces.len())
            .filter(|&j| root(&mut parent, j) == r)
            .collect();
        let mut rings: Vec<LoopKey> = touches
            .iter()
            .filter(|(j, _)| members.contains(j))
            .map(|(_, lp)| *lp)
            .collect();
        rings.sort_unstable();
        rings.dedup();
        let [r0, r1] = rings[..] else {
            return Err(refuse());
        };
        let (Some(c0), Some(c1)) = (ring_of(r0), ring_of(r1)) else {
            return Err(refuse());
        };
        if c0.cap == c1.cap {
            return Err(refuse());
        }
        let mine: Vec<FaceKey> = members.iter().map(|&j| faces[j]).collect();
        let mut edges = Vec::new();
        let mut vertices = Vec::new();
        for &f in &mine {
            for he in face_halves(body, f)? {
                let h = body
                    .get_half_edge(he)
                    .ok_or_else(|| not_intact(EntityId::HalfEdge(he), "a feature's half-edge"))?;
                edges.push(h.edge);
                vertices.push(h.start);
            }
        }
        edges.sort_unstable();
        edges.dedup();
        vertices.sort_unstable();
        vertices.dedup();
        features.push(Feature {
            faces: mine,
            kept: (c0.cap, r0),
            plugged: r1,
            edges,
            vertices,
        });
    }
    // Every cycle read inside is one a feature is bounded by, whole.
    for ring in &consumed.rings {
        let Some(feature) = features
            .iter()
            .find(|f| f.kept.1 == ring.ring || f.plugged == ring.ring)
        else {
            return Err(refuse());
        };
        for he in halves(body, ring.ring)? {
            let (_, _, other) = across(body, he)?;
            if feature.faces.binary_search(&other).is_err() {
                return Err(refuse());
            }
        }
    }
    Ok(features)
}

/// A half-edge of `face`'s cycles whose mate lies in `face` too and
/// ends at a vertex of no other edge: a spur, retired by `kev` with that
/// vertex. `None` when no edge of `face` has both halves in it.
fn spur<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<Option<HalfEdgeKey>, BlendError> {
    let mut inner = false;
    for he in face_halves(body, face)? {
        let (_, _, other) = across(body, he)?;
        if other != face {
            continue;
        }
        inner = true;
        let end = body
            .half_edge_end(he)
            .ok_or_else(|| not_intact(EntityId::HalfEdge(he), "a spur's end"))?;
        let valence = body
            .edges_of_vertex(end)
            .ok_or_else(|| not_intact(EntityId::Vertex(end), "a spur's end"))?
            .len();
        if valence == 1 {
            return Ok(Some(he));
        }
    }
    if inner {
        return Err(not_intact(
            EntityId::Face(face),
            "a consumed feature leaves a cycle of edges inside one face",
        ));
    }
    Ok(None)
}

/// Retire every edge with both halves in `face`, leaf first.
fn retire_spurs<T: Decide>(body: &mut Body<T>, face: FaceKey) -> Result<(), BlendError> {
    while let Some(he) = spur(body, face)? {
        body.kev(he).map_err(|e| op("consumed spur kev", e))?;
    }
    Ok(())
}

/// **Carve one feature away** (module docs), recording its faces, edges
/// and vertices as retired.
///
/// # Errors
///
/// [`BlendError::Op`] when an Euler operator refuses;
/// [`BlendError::BodyNotIntact`] where the body disagrees with the plan;
/// [`BlendError::SurgeryInvariant`] from the face-destroying door.
pub(super) fn consume<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    feature: &Feature,
    sources: &SourceFaces,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(), BlendError> {
    let plug = body
        .mfkrh_minting(feature.plugged, FaceSurface::Inherit, tol)
        .map_err(|e| op("consumed feature plug mfkrh", e))?
        .face;
    let mut alive: Vec<FaceKey> = feature.faces.clone();
    alive.push(plug);
    // Merge the plug and the feature's faces into one, across the edges
    // they share.
    while alive.len() > 1 {
        let mut joined = None;
        'find: for &f in &alive {
            for he in face_halves(body, f)? {
                let (_, _, other) = across(body, he)?;
                if other != f && alive.contains(&other) {
                    joined = Some((he, f));
                    break 'find;
                }
            }
        }
        let Some((he, dying)) = joined else {
            return Err(not_intact(
                EntityId::Face(alive[0]),
                "a consumed feature's faces are not joined by their edges",
            ));
        };
        sources.kef_minted(body, he, "consumed feature kef", tol)?;
        alive.retain(|f| *f != dying);
    }
    let whole = alive[0];
    retire_spurs(body, whole)?;
    // Merge it into the kept cap, across one edge of the cycle there.
    let (cap, ring) = feature.kept;
    let Some(he) = face_halves(body, whole)?.first().copied() else {
        return Err(not_intact(
            EntityId::Face(whole),
            "a consumed feature's merged face has no edge",
        ));
    };
    if face_of_half(body, body.mate(he).unwrap_or(he)) != Some(cap) {
        return Err(not_intact(
            EntityId::Face(whole),
            "a consumed feature's merged face is not bounded by its kept cycle",
        ));
    }
    sources.kef_minted(body, he, "consumed feature cap kef", tol)?;
    retire_spurs(body, cap)?;
    let lone = body
        .get_loop(ring)
        .ok_or_else(|| not_intact(EntityId::Loop(ring), "a consumed feature's kept cycle"))?;
    if !matches!(lone.boundary, LoopBoundary::Empty { .. }) {
        return Err(not_intact(
            EntityId::Loop(ring),
            "a consumed feature's kept cycle is not left a lone vertex",
        ));
    }
    let outer = body
        .get_face(cap)
        .ok_or_else(|| not_intact(EntityId::Face(cap), "a consumed feature's cap"))?
        .outer;
    let Some(&target) = halves(body, outer)?.first() else {
        return Err(not_intact(
            EntityId::Face(cap),
            "a consumed feature's cap has no outer cycle",
        ));
    };
    let strut = body
        .mekr_chord(MekrSite::EmptyRing { target, ring }, tol)
        .map_err(|e| op("consumed feature strut mekr", e))?;
    body.kev(strut.he_plus)
        .map_err(|e| op("consumed feature strut kev", e))?;
    rec.dead.faces.extend(&feature.faces);
    rec.dead.edges.extend(&feature.edges);
    rec.dead.vertices.extend(&feature.vertices);
    Ok(())
}
