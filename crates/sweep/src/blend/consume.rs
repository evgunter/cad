//! **The carve that takes a feature wholly inside a straight band's
//! material** — the ruled band's or the plane–plane band's, cut off at
//! its two end faces: predicate 2's inside verdict (`reach::Consumed`),
//! built.
//!
//! A feature is a connected set of faces, joined by the edges they
//! share. The carve takes each whole, as a Boolean takes a feature
//! wholly inside its tool, or refuses the request `WhollyInside` before
//! any mutation ([`plan`]). The shapes it builds are a through-feature
//! between the band's two caps and a cup on one cap or on one support:
//! faces read inside the prism the band removes (convex) or fills
//! (concave), bounded only by each other and by one cycle on each cap,
//! or on the one face, that the cap's meter read inside its sliver or
//! the strip meter inside the support's strip — a bore, a blind bore or
//! a dimple under a convex band, a bar, a pin or a boss under a concave
//! one. Its faces, edges and vertices go; its cycles leave their faces,
//! and the slivers and strips they lay in go with the band, cut away or
//! buried: a support gives up its ring here, before the band's carve
//! takes its strip, so the carve finds no ring there. The walk reads no
//! material side: the region the feature encloses lies inside the
//! prism, so the band's cut-off builds the same body whether that
//! region is first filled (a bore) or removed (a bar).
//!
//! The walk, per feature, on the clone before the band is carved
//! ([`consume`]): each ring of a feature's face is bridged to its outer
//! cycle (`mekr`), so the faces merged are ring-free; a through-feature's
//! cycle on one face is promoted to a plug face (`mfkrh`, which ends the
//! feature's hole; a cup is a disk already); the plug and the feature's
//! faces are merged across their shared edges into one face (`kef`),
//! and the edges left inside it, a tree hanging from the kept cycle (a
//! cup's mouth, or a through-feature's unplugged one), are retired leaf
//! first (`kev`); that face is merged into the kept cycle's face across
//! one of the cycle's edges (`kef`), whose remaining
//! edges are retired the same way down to a lone vertex; and that vertex
//! is joined to that face's outer cycle by a strut (`mekr`) and retired
//! with it (`kev`).

use geom_core::{Decide, Tol};
use topo::construction::Construct;
use topo::construction::{FaceSurface, MekrSite};
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};

use super::BlendError;
use super::naming::BlendNaming;
use super::reach::Consumed;
use super::surgery::{
    SourceFaces, disagrees, face_of_half, invariant_broken, loop_of_half, loop_walk, op,
    unresolved, unwalked,
};

/// One feature the carve takes, read off the source.
pub(super) struct Feature {
    /// Its faces, sorted.
    faces: Vec<FaceKey>,
    /// The cycle that stays on its face until the merge: the face (a cap
    /// or a support), the cycle.
    kept: (FaceKey, LoopKey),
    /// The cycle promoted to the plug: a through-feature's second
    /// cycle; none for a cup.
    plugged: Option<LoopKey>,
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

/// Each half-edge of `lp`'s cycle ([`loop_walk`]); a lone-vertex cycle
/// has none.
fn halves<T: Decide>(body: &Body<T>, lp: LoopKey) -> Result<Vec<HalfEdgeKey>, BlendError> {
    let lone = body
        .get_loop(lp)
        .ok_or_else(|| unresolved(EntityId::Loop(lp), "a consumed feature's cycle"))?;
    if let LoopBoundary::Empty { .. } = lone.boundary {
        return Ok(Vec::new());
    }
    let walk = loop_walk(body, lp)
        .ok_or_else(|| unwalked(EntityId::Loop(lp), "a consumed feature's cycle"))?;
    Ok(walk.into_iter().map(|(he, _, _)| he).collect())
}

/// Every half-edge of `face`'s cycles.
fn face_halves<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<Vec<HalfEdgeKey>, BlendError> {
    let f = body
        .get_face(face)
        .ok_or_else(|| unresolved(EntityId::Face(face), "a consumed feature's face"))?;
    let mut all = Vec::new();
    for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        all.extend(halves(body, lp)?);
    }
    Ok(all)
}

/// The loop and the face of `he`'s mate.
fn across<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<(LoopKey, FaceKey), BlendError> {
    let unread = || unresolved(EntityId::HalfEdge(he), "a consumed feature's half-edge");
    let m = body.mate(he).ok_or_else(unread)?;
    Ok((
        loop_of_half(body, m).ok_or_else(unread)?,
        face_of_half(body, m).ok_or_else(unread)?,
    ))
}

/// **Group what a request consumes into features**, and refuse one the
/// carve does not take: a face joined to a face outside the feature
/// other than through a cycle read inside on that face, a cycle not
/// wholly bounded by one feature, a face not read inside the material of
/// the band whose region's faces carry the feature's cycles, a
/// lone-vertex cycle on a face of it, or a feature that is neither a
/// through-feature between two faces of one band's region — an annulus,
/// `V − E + F − R = 0` over its faces, `R` their rings — so that with the
/// plug it is a disk, nor a cup on one, a disk (`V − E + F − R = 1`):
/// either way its merge
/// leaves only a tree of edges hanging from the kept cycle.
///
/// Sound because the verdict is inside only when every face and cycle
/// not clear read inside: an inside face shares no edge with a clear
/// one (the edge would lie in both), so a feature's faces were all read,
/// and the check here is that they are all inside one band's material,
/// which reading them does not say of a face grouped with them. A
/// feature is a closed surface with its cycles' disks in the caps'
/// slivers and the supports' strips, all in that band's prism, which
/// holds the region it bounds.
///
/// # Errors
///
/// The request's [`BlendError::WhollyInside`] for a feature it does not
/// take; [`BlendError::BodyNotIntact`] for a body the walk cannot read.
pub(super) fn plan<T: Decide>(
    body: &Body<T>,
    consumed: &Consumed<T>,
) -> Result<Vec<Feature>, BlendError> {
    let refuse = || consumed.refusal.clone();
    let mut faces: Vec<FaceKey> = consumed.read.iter().map(|(f, _)| *f).collect();
    faces.sort_unstable();
    faces.dedup();
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
        let f = body
            .get_face(face)
            .ok_or_else(|| unresolved(EntityId::Face(face), "a consumed feature's face"))?;
        let lone = |lp: &LoopKey| {
            body.get_loop(*lp)
                .is_none_or(|l| matches!(l.boundary, LoopBoundary::Empty { .. }))
        };
        if lone(&f.outer) || f.rings.iter().any(lone) {
            return Err(refuse());
        }
        for he in face_halves(body, face)? {
            let (lp, other) = across(body, he)?;
            if let Ok(j) = faces.binary_search(&other) {
                let (a, b) = (root(&mut parent, i), root(&mut parent, j));
                parent[a] = b;
            } else if ring_of(lp).is_some_and(|r| r.face == other) {
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
        let (r0, r1) = match rings[..] {
            [r0] => (r0, None),
            [r0, r1] => (r0, Some(r1)),
            _ => return Err(refuse()),
        };
        let Some(c0) = ring_of(r0) else {
            return Err(refuse());
        };
        if let Some(r1) = r1 {
            match ring_of(r1) {
                Some(c1) if c1.face != c0.face && c1.crease == c0.crease => {}
                _ => return Err(refuse()),
            }
        }
        let mine: Vec<FaceKey> = members.iter().map(|&j| faces[j]).collect();
        // Every face read inside the material of its cycles' band.
        if !mine
            .iter()
            .all(|f| consumed.read.contains(&(*f, c0.crease)))
        {
            return Err(refuse());
        }
        let mut edges = Vec::new();
        let mut vertices = Vec::new();
        let mut holes = 0;
        for &f in &mine {
            holes += body.get_face(f).map_or(0, |f| f.rings.len());
            for he in face_halves(body, f)? {
                let h = body
                    .get_half_edge(he)
                    .ok_or_else(|| unresolved(EntityId::HalfEdge(he), "a feature's half-edge"))?;
                edges.push(h.edge);
                vertices.push(h.start);
            }
        }
        edges.sort_unstable();
        edges.dedup();
        vertices.sort_unstable();
        vertices.dedup();
        // `V − E + F − R`, `R` the faces' rings: 0 for the annulus, 1 for
        // the disk.
        if vertices.len() + mine.len() != edges.len() + holes + usize::from(r1.is_none()) {
            return Err(refuse());
        }
        features.push(Feature {
            faces: mine,
            kept: (c0.face, r0),
            plugged: r1,
            edges,
            vertices,
        });
    }
    // Every cycle read inside is one a feature is bounded by, whole.
    for ring in &consumed.rings {
        let Some(feature) = features
            .iter()
            .find(|f| f.kept.1 == ring.ring || f.plugged == Some(ring.ring))
        else {
            return Err(refuse());
        };
        for he in halves(body, ring.ring)? {
            let (_, other) = across(body, he)?;
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
        let (_, other) = across(body, he)?;
        if other != face {
            continue;
        }
        inner = true;
        let end = body
            .half_edge_end(he)
            .ok_or_else(|| unresolved(EntityId::HalfEdge(he), "a spur's end"))?;
        let valence = body
            .edges_of_vertex(end)
            .ok_or_else(|| unwalked(EntityId::Vertex(end), "a spur's end"))?
            .len();
        if valence == 1 {
            return Ok(Some(he));
        }
    }
    if inner {
        return Err(invariant_broken(
            EntityId::Face(face),
            "a consumed feature leaves a cycle of edges inside one face, which `plan`'s \
             disk and annulus count rules out",
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
/// [`BlendError::BodyNotIntact`] for a body the walk cannot read;
/// [`BlendError::SurgeryInvariant`] where the walk contradicts the plan,
/// or from the face-destroying door.
pub(super) fn consume<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    feature: &Feature,
    sources: &SourceFaces,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(), BlendError> {
    // Bridge each ring of a face to its outer cycle (`mekr`), so every
    // face the merge kills is ring-free; the bridge is an inside edge of
    // the merged face, retired with the rest.
    for &f in &feature.faces {
        let (outer, rings) = {
            let face = body
                .get_face(f)
                .ok_or_else(|| unresolved(EntityId::Face(f), "a consumed feature's face"))?;
            (face.outer, face.rings.clone())
        };
        for ring in rings {
            let (Some(&target), Some(&from)) =
                (halves(body, outer)?.first(), halves(body, ring)?.first())
            else {
                return Err(invariant_broken(
                    EntityId::Face(f),
                    "a consumed feature's face has a lone-vertex cycle, which `plan` rules out",
                ));
            };
            body.mekr_chord(MekrSite::Cycles { target, ring: from }, tol)
                .map_err(|e| op("consumed feature bridge mekr", e))?;
        }
    }
    let mut alive: Vec<FaceKey> = feature.faces.clone();
    if let Some(plugged) = feature.plugged {
        let plug = body
            .mfkrh_minting(plugged, FaceSurface::Inherit, tol)
            .map_err(|e| op("consumed feature plug mfkrh", e))?
            .face;
        alive.push(plug);
    }
    // Merge the plug and the feature's faces into one, across the edges
    // they share.
    while alive.len() > 1 {
        let mut joined = None;
        'find: for &f in &alive {
            for he in face_halves(body, f)? {
                let (_, other) = across(body, he)?;
                if other != f && alive.contains(&other) {
                    joined = Some((he, f));
                    break 'find;
                }
            }
        }
        let Some((he, dying)) = joined else {
            return Err(invariant_broken(
                EntityId::Face(alive[0]),
                "a consumed feature's faces are not joined by their edges, which `plan` \
                 grouped them by",
            ));
        };
        sources.kef_minted(body, he, "consumed feature kef", tol)?;
        alive.retain(|f| *f != dying);
    }
    let whole = alive[0];
    retire_spurs(body, whole)?;
    // Merge it into the kept cycle's face, across one edge of the cycle.
    let (cap, ring) = feature.kept;
    let Some(he) = face_halves(body, whole)?.first().copied() else {
        return Err(invariant_broken(
            EntityId::Face(whole),
            "a consumed feature's merged face has no edge",
        ));
    };
    if across(body, he)?.1 != cap {
        return Err(invariant_broken(
            EntityId::Face(whole),
            "a consumed feature's merged face is not bounded by its kept cycle",
        ));
    }
    sources.kef_minted(body, he, "consumed feature cap kef", tol)?;
    retire_spurs(body, cap)?;
    let lone = body
        .get_loop(ring)
        .ok_or_else(|| unresolved(EntityId::Loop(ring), "a consumed feature's kept cycle"))?;
    if !matches!(lone.boundary, LoopBoundary::Empty { .. }) {
        return Err(invariant_broken(
            EntityId::Loop(ring),
            "a consumed feature's kept cycle is not left a lone vertex",
        ));
    }
    let outer = body
        .get_face(cap)
        .ok_or_else(|| unresolved(EntityId::Face(cap), "a consumed feature's cap"))?
        .outer;
    let Some(&target) = halves(body, outer)?.first() else {
        return Err(disagrees(
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
