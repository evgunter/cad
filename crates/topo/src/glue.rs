//! **The per-shell glue relation**: a face glues all its loops, a
//! cycle loop glues across each of its edges to the mate's face, and an
//! empty loop glues only its vertex, so a face whose loops are all empty
//! is a dartless component of its own. [`face_set_glue`] is the one
//! walk of it that partitions a face set: `movefac`'s plan, the open-face
//! designation check and the sequence generator's `movefac` row read
//! their components from it. The validator's pass 11 enumerates the
//! same relation independently.

use slotmap::SecondaryMap;

use crate::body::{Body, CYCLES_ARE_CLAIMANTS, WALKS_CLOSE, Walk};
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey};
use crate::euler::are_halves;
use crate::live::{NAMES_ONLY_LIVE, OPERATORS_KEEP_LINKS};
use geom_core::Real;

/// A face set's components under the glue relation.
#[derive(Clone, Debug)]
pub(crate) struct FaceSetComponents {
    /// Each face of the set, labelled with its component. Labels are
    /// numbered in the order the set lists the components' first faces.
    pub(crate) labels: SecondaryMap<FaceKey, usize>,
    /// How many components the set falls into.
    pub(crate) count: usize,
    /// Every mate hop that left the set, in walk order. The walk does
    /// not continue past one.
    pub(crate) leaving: Vec<LeavingHop>,
}

/// A mate hop from a face of the set to a face outside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LeavingHop {
    /// The face of the set the hop starts from.
    pub(crate) from: FaceKey,
    /// The member of `from`'s loop whose mate lies outside the set.
    pub(crate) member: HalfEdgeKey,
    /// The face the mate's loop names.
    pub(crate) to: FaceKey,
}

impl FaceSetComponents {
    /// The set's faces grouped by component, component `k` at index
    /// `k`, each listing its faces in the set's order.
    pub(crate) fn components(&self, faces: &[FaceKey]) -> Vec<Vec<FaceKey>> {
        let mut out = vec![Vec::new(); self.count];
        for &face in faces {
            out[self.labels[face]].push(face);
        }
        out
    }
}

/// A record the glue walk read that a tier-1-valid body does not hold.
/// Each variant names the record; [`core::fmt::Display`] says which
/// tier-1 premise it breaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GlueError {
    /// `holder`'s field `field` names `key`, which does not resolve.
    Dangling {
        holder: EntityId,
        field: &'static str,
        key: EntityId,
    },
    /// `face` lists `r#loop`, which names `names`.
    LoopNamesAnotherFace {
        face: FaceKey,
        r#loop: LoopKey,
        names: FaceKey,
    },
    /// A half-edge claims the empty loop `r#loop`.
    EmptyLoopClaimed { r#loop: LoopKey },
    /// `r#loop`'s `next` walk from `first` breaks at `at`, or, with no
    /// `at`, does not close within the half-edge arena's length.
    WalkOpen {
        r#loop: LoopKey,
        first: HalfEdgeKey,
        at: Option<HalfEdgeKey>,
    },
    /// `member`, walked as a member of `r#loop`'s cycle, claims another.
    NotClaimant {
        member: HalfEdgeKey,
        r#loop: LoopKey,
    },
    /// `r#loop`'s cycle walk closed short of the half-edges that claim it.
    ClosedShort { r#loop: LoopKey },
    /// `he`'s edge `edge` claims it in neither slot.
    Unclaimed { he: HalfEdgeKey, edge: EdgeKey },
    /// `he` and `mate`, read as `edge`'s halves, are not: `mate` names
    /// `mate_edge`, or is `he`.
    NotHalves {
        edge: EdgeKey,
        he: HalfEdgeKey,
        mate: HalfEdgeKey,
        mate_edge: EdgeKey,
    },
    /// `r#loop` names `face`, which does not list it.
    LoopUnlisted { r#loop: LoopKey, face: FaceKey },
}

impl core::fmt::Display for GlueError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Dangling { holder, field, key } => write!(
                f,
                "{holder}'s {field} names {key}, which does not resolve: {NAMES_ONLY_LIVE}; \
                 {OPERATORS_KEEP_LINKS}"
            ),
            Self::LoopNamesAnotherFace {
                face,
                r#loop,
                names,
            } => write!(
                f,
                "{face:?} lists loop {loop:?}, which names face {names:?}: on a tier-1-valid \
                 body a face's loops name it",
                loop = r#loop
            ),
            Self::EmptyLoopClaimed { r#loop } => write!(
                f,
                "empty loop {loop:?} is claimed by a half-edge: on a tier-1-valid body an empty \
                 loop has no half-edges",
                loop = r#loop
            ),
            Self::WalkOpen {
                r#loop,
                first,
                at: Some(at),
            } => write!(
                f,
                "the loop walk from {first:?} breaks at {at:?}, in loop {loop:?}: its step from \
                 there does not resolve, and {WALKS_CLOSE}; {OPERATORS_KEEP_LINKS}",
                loop = r#loop
            ),
            Self::WalkOpen {
                r#loop,
                first,
                at: None,
            } => write!(
                f,
                "the loop walk from {first:?} does not close within the half-edge arena's \
                 length, in loop {loop:?}, and {WALKS_CLOSE}; {OPERATORS_KEEP_LINKS}",
                loop = r#loop
            ),
            Self::NotClaimant { member, r#loop } => write!(
                f,
                "{member:?}, walked as a member of loop {loop:?}'s cycle, does not claim it: \
                 {CYCLES_ARE_CLAIMANTS}",
                loop = r#loop
            ),
            Self::ClosedShort { r#loop } => write!(
                f,
                "loop {loop:?}'s cycle walk is not every half-edge that claims it: \
                 {CYCLES_ARE_CLAIMANTS}",
                loop = r#loop
            ),
            Self::Unclaimed { he, edge } => write!(
                f,
                "{he:?}'s edge {edge:?} does not claim it in either slot: on a tier-1-valid \
                 body an edge claims the two half-edges that name it"
            ),
            Self::NotHalves {
                edge,
                he,
                mate,
                mate_edge,
            } => write!(
                f,
                "{he:?} and {mate:?}, read as edge {edge:?}'s halves, are not ({mate:?} names \
                 {mate_edge:?}): on a tier-1-valid body the edge <-> half-edge bijection holds"
            ),
            Self::LoopUnlisted { r#loop, face } => write!(
                f,
                "loop {loop:?} names face {face:?}, which does not list it: on a tier-1-valid \
                 body a loop's face lists it",
                loop = r#loop
            ),
        }
    }
}

/// `key`'s record in `arena`, or [`GlueError::Dangling`] naming
/// `holder`'s field `field`.
fn resolve<'a, K: slotmap::Key, V>(
    arena: &'a slotmap::SlotMap<K, V>,
    key: K,
    id: fn(K) -> EntityId,
    holder: EntityId,
    field: &'static str,
) -> Result<&'a V, GlueError> {
    arena.get(key).ok_or(GlueError::Dangling {
        holder,
        field,
        key: id(key),
    })
}

/// **`faces`' components under the glue relation**, seeded in `faces`'
/// order; `holder` is the record whose field `faces` the set is read
/// from, which a seed that does not resolve is named by.
///
/// Every hop is proven before the walk takes it, so that a torn record
/// cannot join two components or split one without an error: each loop
/// a face lists names the face; no half-edge claims an empty loop; a
/// cycle loop's `next` walk closes on half-edges that all claim it, as
/// many as claim it at all (the half-edge arena is read once for the
/// counts); each member's edge claims it and its mate, which names the
/// same edge; and the face the mate's loop names lists that loop. Under
/// those proofs a mate hop is symmetric and every walked loop is whole,
/// so a face is reached by its component's walk alone.
///
/// A hop to a face outside `faces` is recorded in
/// [`FaceSetComponents::leaving`] and not followed: what one means is
/// the caller's contract.
///
/// # Errors
///
/// [`GlueError`] naming the first record a proof finds torn.
pub(crate) fn face_set_glue<T: Real>(
    body: &Body<T>,
    faces: &[FaceKey],
    holder: EntityId,
) -> Result<FaceSetComponents, GlueError> {
    let mut claims: SecondaryMap<LoopKey, usize> = SecondaryMap::new();
    for (_, half) in &body.half_edges {
        match claims.get_mut(half.parent_loop) {
            Some(count) => *count += 1,
            None => {
                claims.insert(half.parent_loop, 1);
            }
        }
    }
    let mut in_set: SecondaryMap<FaceKey, ()> = SecondaryMap::new();
    for &face in faces {
        in_set.insert(face, ());
    }
    let mut labels: SecondaryMap<FaceKey, usize> = SecondaryMap::new();
    let mut leaving = Vec::new();
    let mut count = 0_usize;
    for &seed in faces {
        if labels.contains_key(seed) {
            continue;
        }
        let label = count;
        count += 1;
        resolve(&body.faces, seed, EntityId::Face, holder, "faces")?;
        labels.insert(seed, label);
        let mut pending = vec![seed];
        while let Some(face_key) = pending.pop() {
            // Every face pushed was resolved where it was found.
            let face = &body.faces[face_key];
            for (role, loop_key) in face.loop_roles() {
                let loop_data = resolve(
                    &body.loops,
                    loop_key,
                    EntityId::Loop,
                    EntityId::Face(face_key),
                    role.field(),
                )?;
                if loop_data.face != face_key {
                    return Err(GlueError::LoopNamesAnotherFace {
                        face: face_key,
                        r#loop: loop_key,
                        names: loop_data.face,
                    });
                }
                if matches!(loop_data.boundary, LoopBoundary::Empty { .. })
                    && claims.contains_key(loop_key)
                {
                    return Err(GlueError::EmptyLoopClaimed { r#loop: loop_key });
                }
                let LoopBoundary::Cycle { first } = loop_data.boundary else {
                    continue; // empty loop: glues only its vertex
                };
                let cycle = match body.loop_walk(first) {
                    Walk::Closed(members) => members,
                    Walk::Broken { at } => {
                        return Err(GlueError::WalkOpen {
                            r#loop: loop_key,
                            first,
                            at: Some(at),
                        });
                    }
                    Walk::Overrun => {
                        return Err(GlueError::WalkOpen {
                            r#loop: loop_key,
                            first,
                            at: None,
                        });
                    }
                };
                for &member in &cycle {
                    if body.half_edges[member].parent_loop != loop_key {
                        return Err(GlueError::NotClaimant {
                            member,
                            r#loop: loop_key,
                        });
                    }
                }
                if claims.get(loop_key) != Some(&cycle.len()) {
                    return Err(GlueError::ClosedShort { r#loop: loop_key });
                }
                for member in cycle {
                    let to = mate_face(body, member)?;
                    if !in_set.contains_key(to) {
                        leaving.push(LeavingHop {
                            from: face_key,
                            member,
                            to,
                        });
                        continue;
                    }
                    // A labelled neighbour carries this label. Only an
                    // earlier, finished walk could have given it another,
                    // and that walk popped `to` and walked the mate's loop,
                    // which `to` lists. The proofs above make that loop a
                    // cycle whose walk is all of its claimants, the mate
                    // among them, and the hop symmetric, so that walk
                    // hopped back to `face_key` and labelled it.
                    match labels.get(to) {
                        Some(&reached) if reached == label => {}
                        Some(&reached) => unreachable!(
                            "the glue walk's mate hop {member:?} -> {to:?} reached label \
                             {reached} from {face_key:?} at label {label}; the walk that \
                             labelled {to:?} walked the mate's loop whole and hopped back"
                        ),
                        None => {
                            labels.insert(to, label);
                            pending.push(to);
                        }
                    }
                }
            }
        }
    }
    Ok(FaceSetComponents {
        labels,
        count,
        leaving,
    })
}

/// The face across `member`, a resolved member of a cycle: its mate,
/// proven its pair, and the face its loop names, proven to list it.
fn mate_face<T: Real>(body: &Body<T>, member: HalfEdgeKey) -> Result<FaceKey, GlueError> {
    let edge = body.half_edges[member].edge;
    let edge_data = resolve(
        &body.edges,
        edge,
        EntityId::Edge,
        EntityId::HalfEdge(member),
        "edge",
    )?;
    let claim = edge_data
        .claim(member)
        .ok_or(GlueError::Unclaimed { he: member, edge })?;
    let mate_data = resolve(
        &body.half_edges,
        claim.mate,
        EntityId::HalfEdge,
        EntityId::Edge(edge),
        claim.mate_field(),
    )?;
    if !are_halves(edge, edge_data, member, (claim.mate, mate_data.edge)) {
        return Err(GlueError::NotHalves {
            edge,
            he: member,
            mate: claim.mate,
            mate_edge: mate_data.edge,
        });
    }
    let mate_loop = mate_data.parent_loop;
    let face = resolve(
        &body.loops,
        mate_loop,
        EntityId::Loop,
        EntityId::HalfEdge(claim.mate),
        "parent_loop",
    )?
    .face;
    let face_data = resolve(
        &body.faces,
        face,
        EntityId::Face,
        EntityId::Loop(mate_loop),
        "face",
    )?;
    if !face_data.holds_loop(mate_loop) {
        return Err(GlueError::LoopUnlisted {
            r#loop: mate_loop,
            face,
        });
    }
    Ok(face)
}
