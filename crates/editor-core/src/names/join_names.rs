//! **Naming an edge a door's closing join made** (the 3881 step-3
//! naming ruling; `names/README.md`): by the door's INPUT cells it
//! covers, never by the transient pieces the join took. Every door that
//! ends with the join reads its records through `topo::join_covers`, and
//! asks each covered edge one question — which input edge is it the
//! image of — which only the door can answer ([`Member`]). The rest is
//! shared: one image is the edge's own name, several are the flat
//! `Merged` set of them ([`joined_name`]), and a member the door minted
//! outright, which reads no input edge, refuses.

use std::collections::{BTreeMap, BTreeSet};

use topo::{Body, EdgeJoin, EdgeKey};

use super::emit::{JoinShape, NamingError, name1};
use super::role::{EntityKind, RoleSeg, StableName};
use super::table::EntityKey;
use crate::node::RecipeNodeId;

/// What one edge a join covered reads as, in the door's own rows.
pub(crate) enum Member {
    /// The image of one input edge — a survivor's `From`, a cavity
    /// twin's `Inner`, a blend's trimline `TrimEdge` or rim trim
    /// `BandTrim` — under this segment, and whether its row is tied.
    Image { seg: RoleSeg, tied: bool },
    /// An edge the door minted outright — an end arc, a mitre, a band's
    /// slit, a remnant — which is the image of no input edge.
    Outright,
}

/// **The name of a joined edge over `names`**, the names of the input
/// cells its cover holds, with whether any is tied: one name is the
/// edge's own, several are the flat `Merged` set of them, minted by
/// `node` (`merged::edge_set`, N3).
pub(crate) fn joined_name(
    node: RecipeNodeId,
    names: impl IntoIterator<Item = (StableName, bool)>,
) -> (StableName, bool) {
    let mut tied = false;
    let mut set = BTreeSet::new();
    for (name, t) in names {
        tied |= t;
        set.insert(name);
    }
    let name = match set.pop_first() {
        Some(one) if set.is_empty() => one,
        first => super::merged::edge_set(node, first.into_iter().chain(set)),
    };
    (name, tied)
}

/// **Every joined edge of `body`, named** ([`joined_name`]) from the
/// door's `joins`, each covered edge read by `member`: the kept edge →
/// its one segment under `node`, and whether it is tied. The door's
/// table takes these in place of whatever its rows gave the kept edge.
///
/// # Errors
///
/// [`NamingError::UnsupportedJoin`] where a cover holds an edge the door
/// minted outright (the ruling's stop case), or as `member` refuses.
///
/// # Panics
///
/// Where a record's kept edge is not in `body`, or a joined name is not
/// one segment: both are the door's own records disagreeing with it.
pub(crate) fn name_joins<T: geom_core::Real>(
    node: RecipeNodeId,
    body: &Body<T>,
    joins: &[EdgeJoin],
    mut member: impl FnMut(EdgeKey) -> Result<Member, NamingError>,
) -> Result<BTreeMap<EntityKey, (RoleSeg, bool)>, NamingError> {
    let mut out = BTreeMap::new();
    for (kept, cover) in topo::join_covers(joins) {
        if body.get_edge(kept).is_none() {
            unreachable!(
                "a join's kept edge {kept:?} is not in the body the door recorded it on: a join \
                 keeps `kept` live, and `topo::join_covers` hands a kept edge a later join \
                 took on to that join's kept edge"
            );
        }
        let mut images = Vec::with_capacity(cover.len());
        for m in cover {
            match member(m)? {
                Member::Image { seg, tied } => {
                    images.push((name1(EntityKind::Edge, node, seg), tied));
                }
                Member::Outright => {
                    return Err(NamingError::UnsupportedJoin {
                        edge: kept,
                        shape: JoinShape::Outright,
                    });
                }
            }
        }
        let (name, tied) = joined_name(node, images);
        let [seg] = name.path.as_slice() else {
            unreachable!(
                "joined edge {kept:?}'s name {name:?} is not one segment: `joined_name` returns \
                 one image, each one `name1` segment, or one flat `Merged` set of them"
            );
        };
        out.insert(EntityKey::Edge(kept), (seg.clone(), tied));
    }
    Ok(out)
}
