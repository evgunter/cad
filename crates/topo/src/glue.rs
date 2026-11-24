//! **The per-shell glue relation**: a face glues all its loops, a
//! cycle loop glues across each of its edges to the mate's face, and an
//! empty loop glues only its vertex, so a face whose loops are all empty
//! is a dartless component of its own. [`face_set_glue`] is the one
//! walk of it that partitions a face set: `movefac`'s plan, the open-face
//! designation check and the sequence generator's `movefac` row read
//! their components from it. The validator's pass 11 enumerates the
//! same relation independently.

use geom_core::Decide;
use slotmap::SecondaryMap;

use crate::body::{Body, CYCLES_ARE_CLAIMANTS};
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey};
use crate::live::{link, linked};

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

/// How many half-edges claim each loop of a body: the arena read once,
/// for every [`face_set_glue`] call on the body until it changes.
pub(crate) struct LoopClaims(SecondaryMap<LoopKey, usize>);

impl LoopClaims {
    /// The claim counts of `body`'s half-edge arena.
    pub(crate) fn of<T: geom_core::Real>(body: &Body<T>) -> Self {
        let mut claims: SecondaryMap<LoopKey, usize> = SecondaryMap::new();
        for (_, half) in &body.half_edges {
            match claims.get_mut(half.parent_loop) {
                Some(count) => *count += 1,
                None => {
                    claims.insert(half.parent_loop, 1);
                }
            }
        }
        Self(claims)
    }
}

/// **`faces`' components under the glue relation**, seeded in `faces`'
/// order; `holder` is the record whose field `faces` the set is read
/// from, and `claims` is [`LoopClaims::of`] this body.
///
/// Every hop is proven before the walk takes it, so that a torn record
/// cannot join two components or split one: each loop a face lists
/// names the face; no half-edge claims an empty loop; a cycle loop's
/// `next` walk from its first half-edge closes on half-edges that all
/// claim it ([`Body::loop_walk_of`]), as many as claim it at all; each member's mate is proven its pair
/// ([`Body::proven_mate`]); and the face the mate's loop names lists
/// that loop ([`Body::face_of_linked`]). Under those proofs a mate hop is
/// symmetric and every walked loop is whole, so a face is reached by its
/// component's walk alone.
///
/// A hop to a face outside `faces` is recorded in
/// [`FaceSetComponents::leaving`] and not followed: what one means is
/// the caller's contract.
///
/// # Panics
///
/// Where a proof fails, naming the record (D2 row 4): on a tier-1-valid
/// body each holds.
#[track_caller]
pub(crate) fn face_set_glue<T: Decide>(
    body: &Body<T>,
    claims: &LoopClaims,
    faces: &[FaceKey],
    holder: EntityId,
) -> FaceSetComponents {
    let claims = &claims.0;
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
        // Every face pushed is resolved where it is found: a seed
        // through the set's holder, a neighbour through its loop.
        linked(&body.faces, seed, EntityId::Face, holder, "faces");
        labels.insert(seed, label);
        let mut pending = vec![seed];
        while let Some(face_key) = pending.pop() {
            for (loop_key, loop_data, _) in body.proven_face(face_key).loops() {
                assert!(
                    loop_data.face == face_key,
                    "{face_key:?} lists loop {loop_key:?}, which names face {:?}: on a \
                     tier-1-valid body a face's loops name it",
                    loop_data.face
                );
                assert!(
                    matches!(loop_data.boundary, LoopBoundary::Cycle { .. })
                        || !claims.contains_key(loop_key),
                    "empty loop {loop_key:?} is claimed by a half-edge: on a tier-1-valid body \
                     an empty loop has no half-edges"
                );
                let LoopBoundary::Cycle { first } = loop_data.boundary else {
                    continue; // empty loop: glues only its vertex
                };
                assert!(
                    body.claims(first, loop_key),
                    "loop {loop_key:?}'s first half-edge {first:?} does not claim it: \
                     {CYCLES_ARE_CLAIMANTS}"
                );
                let cycle = body
                    .loop_walk_of(first, loop_key)
                    .closed_in(loop_key, first);
                assert!(
                    claims.get(loop_key) == Some(&cycle.len()),
                    "loop {loop_key:?}'s cycle walk is not every half-edge that claims it: \
                     {CYCLES_ARE_CLAIMANTS}"
                );
                for member in cycle {
                    let mate = body.proven_mate(member, link(EntityId::Loop(loop_key), "cycle"));
                    let to = body.face_of_linked(mate.mate);
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
    FaceSetComponents {
        labels,
        count,
        leaving,
    }
}
