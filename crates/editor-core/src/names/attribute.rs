//! **Which node MINTED a named entity** — the carry-through walk down
//! a [`StableName`]'s role path.
//!
//! A name's own [`StableName::node`] is the node whose op EMITTED it,
//! which is not the same question. A fillet emits a name for every
//! entity of its output, its target's untouched faces included:
//! `FromTarget(inner)` says "this is the target's entity `inner`,
//! carried through", and `inner` carries the target's own node. So the
//! node a drawn face should be attributed to is found by descending
//! the carry-through segments until a CREATING role is reached, and
//! the entity was minted where that descent stops.
//!
//! # The partition is the vocabulary's, read one segment at a time
//!
//! Only the OUTERMOST segment decides ([`RolePath`](super::RolePath)'s
//! composition puts the op's own role first and its qualifiers after,
//! so `[FromA(f), Fragment(q)]` is "A's face `f`, the `q` piece of it"
//! — carried, not minted). Three answers exist and each is a
//! statement about the entity, not about the op:
//!
//! - **carried** — the entity existed in an operand and this op passed
//!   it through, whole (`FromA`/`FromB`/`FromTarget`), copied
//!   (`Instance`, `OnToolVertex`) or shortened (`SplitFragment`,
//!   `BandCut`). The argument IS the operand's name, so the walk
//!   continues there;
//! - **minted** — nothing upstream was this entity: a blend face, a
//!   seam, a section, a cap. Where such a role carries names they are
//!   REFERENCES to what the op cut against, never the entity itself,
//!   so the walk stops;
//! - **unclassified** — a segment this partition cannot place. It is
//!   answered as such rather than guessed into either bucket, because
//!   a wrong attribution marks confidently wrong geometry and a
//!   missing one only falls back.
//!
//! An op that emits no segment at all (a `Transform`, a split-intact
//! entity) needs no arm here: it leaves the ORIGINAL minting node in
//! [`StableName::node`] by construction, so the walk answers it
//! without ever seeing the transform.

use crate::names::role::{RoleSeg, StableName, name_free_seg};
use crate::node::RecipeNodeId;

/// **Where a named entity came from**: the recipe nodes its derivation
/// passes through, and the one that minted it.
///
/// The chain is ordered outermost first — the node whose table the
/// name was read from — and the minting node is its last element
/// whenever one was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameOrigin {
    /// The nodes the entity passed through, outermost first.
    chain: Vec<RecipeNodeId>,
    /// The node that minted it, or `None` for a role path this
    /// vocabulary walk could not classify.
    minted: Option<RecipeNodeId>,
}

impl NameOrigin {
    /// The node whose operation MINTED the entity — `None` when the
    /// walk met a segment it could not classify.
    pub fn minted_by(&self) -> Option<RecipeNodeId> {
        self.minted
    }

    /// Whether the entity's derivation passes through `node`: it was
    /// minted there, or an op above carried it through from there.
    ///
    /// True at the minting node itself, so "minted here" implies
    /// "passes through here" and a caller need not test both.
    pub fn passes_through(&self, node: RecipeNodeId) -> bool {
        self.chain.contains(&node)
    }

    /// The whole chain, outermost first.
    pub fn chain(&self) -> &[RecipeNodeId] {
        &self.chain
    }
}

/// What one role segment says about the entity it names.
enum SegOrigin<'a> {
    /// The entity existed in an operand; the name is its name there.
    Carried(&'a StableName),
    /// This op made the entity.
    Minted,
    /// A segment the partition does not place.
    Unclassified,
}

/// Read the outermost segment's verdict — the whole classification,
/// stated once (module docs).
fn origin(seg: &RoleSeg) -> SegOrigin<'_> {
    match seg {
        // The name-free roles are the sweep, split and boolean
        // primitives: an entity born of the recipe rather than of an
        // operand entity.
        name_free_seg!() => SegOrigin::Minted,

        // Carried through: the argument is the entity's own name one
        // level down.
        RoleSeg::FromA(of)
        | RoleSeg::FromB(of)
        | RoleSeg::FromTarget(of)
        | RoleSeg::SplitFragment { parent: of, .. }
        | RoleSeg::OnToolVertex { of, .. }
        | RoleSeg::Instance { of, .. }
        | RoleSeg::BandCut(of) => SegOrigin::Carried(of),

        // Minted here. The names these carry are the entities the op
        // worked AGAINST — a blend's source edge, a seam's two
        // crossing faces, a section's operand face — and the entity
        // named is none of them.
        RoleSeg::Seam { .. }
        | RoleSeg::SectionEdge { .. }
        | RoleSeg::CrossingVertex { .. }
        | RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::CornerArc { .. }
        | RoleSeg::BandFace(_)
        | RoleSeg::BandTrim { .. }
        | RoleSeg::BandFoot(_)
        | RoleSeg::BandCross(_)
        | RoleSeg::BandSlit(_) => SegOrigin::Minted,

        // A merged face has SEVERAL parents and is identical to none
        // of them, so there is no single operand entity to descend
        // into and the merge itself is what made this face. The cost
        // of the reading, stated: a merged face answers the boolean
        // rather than either operand it retired.
        RoleSeg::Merged(_) => SegOrigin::Minted,

        // The instantiate node is where a referenced part's entity
        // enters THIS document, and the walk stops there on purpose:
        // the inner name's node is an id in the OTHER document, and a
        // chain mixing the two would name a local node by coincidence
        // of numbering.
        RoleSeg::InPart { .. } => SegOrigin::Minted,

        // A qualifier, which composes AFTER the parent-bearing segment
        // and never leads a path. Leading one is a name this walk
        // cannot read.
        RoleSeg::Fragment(_) => SegOrigin::Unclassified,
    }
}

/// **Attribute `name` to the node that made the entity** — the
/// carry-through walk of the module docs.
///
/// Total and pure: a name whose path is empty, or whose outermost
/// segment is a qualifier, answers a chain with no minting node rather
/// than a guess.
pub fn attribute(name: &StableName) -> NameOrigin {
    let mut chain = Vec::new();
    let mut at = name;
    loop {
        chain.push(at.node);
        let Some(seg) = at.path.first() else {
            return NameOrigin {
                chain,
                minted: None,
            };
        };
        match origin(seg) {
            SegOrigin::Carried(inner) => at = inner,
            SegOrigin::Minted => {
                let minted = Some(at.node);
                return NameOrigin { chain, minted };
            }
            SegOrigin::Unclassified => {
                return NameOrigin {
                    chain,
                    minted: None,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::{CapEnd, EntityKind, ProfileEdgeRef, Qualifier};

    const EXTRUDE: RecipeNodeId = RecipeNodeId(1);
    const CUT: RecipeNodeId = RecipeNodeId(2);
    const FILLET: RecipeNodeId = RecipeNodeId(3);
    const PATTERN: RecipeNodeId = RecipeNodeId(4);
    const INSTANCE: RecipeNodeId = RecipeNodeId(5);

    /// A cap face of the extrude — a name-free role, minted where it
    /// is emitted.
    fn cap() -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: EXTRUDE,
            path: vec![RoleSeg::Cap(CapEnd::Top)],
        }
    }

    /// A wall face of the extrude, used where a name is needed only as
    /// a REFERENCE argument.
    fn wall() -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: EXTRUDE,
            path: vec![RoleSeg::Lateral(ProfileEdgeRef {
                loop_index: 0,
                segment: 0,
            })],
        }
    }

    fn at(node: RecipeNodeId, seg: RoleSeg) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node,
            path: vec![seg],
        }
    }

    #[test]
    fn a_sweeps_own_role_is_minted_where_it_is_emitted() {
        let it = attribute(&cap());
        assert_eq!(it.minted_by(), Some(EXTRUDE));
        assert_eq!(it.chain(), [EXTRUDE].as_slice());
    }

    /// **The unit's whole claim, in one name.** A fillet emits a name
    /// for the target face it merely shrank; that face is the
    /// extrude's, and the walk says so.
    #[test]
    fn a_carried_face_belongs_to_the_operand_it_came_from() {
        let carried = at(FILLET, RoleSeg::FromTarget(Box::new(cap())));
        let it = attribute(&carried);
        assert_eq!(it.minted_by(), Some(EXTRUDE));
        assert_eq!(it.chain(), [FILLET, EXTRUDE].as_slice());
        assert!(it.passes_through(FILLET), "the fillet carried it");
        assert!(it.passes_through(EXTRUDE), "and it was minted below");
    }

    /// A blend face is the fillet's own, and the source edge it rounds
    /// is a REFERENCE — so the chain stops at the fillet rather than
    /// walking into the edge's operand.
    #[test]
    fn a_blend_face_is_the_fillets_own_and_names_no_operand() {
        let blend = at(FILLET, RoleSeg::BlendFace(Box::new(wall())));
        let it = attribute(&blend);
        assert_eq!(it.minted_by(), Some(FILLET));
        assert_eq!(it.chain(), [FILLET].as_slice());
        assert!(!it.passes_through(EXTRUDE));
    }

    /// Composition: the qualifier rides BEHIND the parent-bearing
    /// segment, so the outermost segment is still the carry-through
    /// and the fragment of A's face is A's face.
    #[test]
    fn a_fragment_qualifier_does_not_move_the_attribution() {
        let fragment = StableName {
            kind: EntityKind::Face,
            node: CUT,
            path: vec![
                RoleSeg::FromA(Box::new(cap())),
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }),
            ],
        };
        assert_eq!(attribute(&fragment).minted_by(), Some(EXTRUDE));
    }

    /// Depth is not special: a face carried through a cut and then a
    /// fillet is still the extrude's.
    #[test]
    fn the_walk_descends_as_far_as_the_carry_through_goes() {
        let cut = at(CUT, RoleSeg::FromA(Box::new(cap())));
        let filleted = at(FILLET, RoleSeg::FromTarget(Box::new(cut)));
        let it = attribute(&filleted);
        assert_eq!(it.minted_by(), Some(EXTRUDE));
        assert_eq!(it.chain(), [FILLET, CUT, EXTRUDE].as_slice());
    }

    /// A pattern instance is a COPY of the master's entity, so it
    /// belongs to the node that made the master — which is what makes
    /// selecting that feature light every copy.
    #[test]
    fn a_pattern_instance_belongs_to_the_master() {
        let copy = at(
            PATTERN,
            RoleSeg::Instance {
                i: 3,
                of: Box::new(cap()),
            },
        );
        let it = attribute(&copy);
        assert_eq!(it.minted_by(), Some(EXTRUDE));
        assert!(it.passes_through(PATTERN));
    }

    /// The walk stops at the instantiate node: the inner name's node
    /// is an id in the REFERENCED document, and a chain mixing the two
    /// documents' ids would name a local node by coincidence of
    /// numbering.
    #[test]
    fn a_part_entity_stops_at_the_instantiate_node() {
        let in_part = at(
            INSTANCE,
            RoleSeg::InPart {
                of: Box::new(cap()),
            },
        );
        let it = attribute(&in_part);
        assert_eq!(it.minted_by(), Some(INSTANCE));
        assert_eq!(it.chain(), [INSTANCE].as_slice());
        assert!(
            !it.passes_through(EXTRUDE),
            "EXTRUDE here is another document's id"
        );
    }

    /// A path this partition cannot read answers NO minting node —
    /// the honest empty answer a caller can fall back from, never a
    /// guess.
    #[test]
    fn an_unreadable_path_answers_no_minting_node() {
        let headless = StableName {
            kind: EntityKind::Face,
            node: CUT,
            path: Vec::new(),
        };
        assert_eq!(attribute(&headless).minted_by(), None);
        assert_eq!(attribute(&headless).chain(), [CUT].as_slice());
        let qualifier_first = at(
            CUT,
            RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }),
        );
        assert_eq!(attribute(&qualifier_first).minted_by(), None);
    }
}
