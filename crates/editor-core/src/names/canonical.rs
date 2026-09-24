//! **A role path's canonical form**: which positions in a path are in
//! NAME ORDER, and what putting them there does to a rank.
//!
//! Some positions in a [`RolePath`](super::RolePath) have no order of
//! their own. Their canonical order is the order of the names they
//! hold, not the order an emitter found them in:
//!
//! - a [`RoleSeg::Merged`] constituent set and a [`RoleSeg::BandFace`]
//!   edge set: sorted, and deduplicated, because the SET is the name
//!   (N3);
//! - a [`Qualifier::SideOf`] vector: one entry per partner, sorted by
//!   partner name;
//! - a seam JUNCTION, the vertex where k ≥ 2 seam lines meet, named by
//!   the run of those lines' [`RoleSeg::Seam`] segments and nothing
//!   else: the run is sorted, and NOT deduplicated (k distinct lines
//!   meet there, so two equal lines are a rewrite that was not
//!   one-to-one, which the caller refuses);
//! - a [`RoleSeg::Seam`]'s two sides, in a name whose seams are
//!   [`Seams::ByName`]: a union has no A and B, so its pair is put in
//!   name order. A pair boolean's seams are [`Seams::Sided`]: `a` is
//!   the A operand's entity, and that order is data.
//!
//! Any rewrite that changes the names in those positions has to put
//! them back in order, or it publishes a name the emitter would not
//! mint for the same entity in the rewritten space. This module is the
//! one list of those positions. The pair emitter's mint
//! (`emit_topo`, `emit_blend`), the union's collapse (`emit_union`) and
//! every rewrite of a published path ([`StableName::rewrite_path`]:
//! the anchor, the whole-program edit and the split re-map) end in it.
//!
//! # A value that depends on a name order
//!
//! Putting a seam pair in name order can rewrite a VALUE later in the
//! path. A seam line's `Fragment(OrderAlong)` ranks are measured along
//! `n_a × n_b`, the line oriented by the pair's first side, so where
//! the pair comes out swapped the rank reads from the other end:
//! `of − 1 − rank`. A seam VERTEX group's rank lies along its edge
//! parent's line, so it follows that line's swap instead of its own
//! head's. That rule is [`RankRule`], and it lives here because the
//! swap does: a caller works out the rule from the name as it was and
//! the map it applied ([`RankRule::across`]), and hands it in as
//! [`Seams::ByName`]'s argument.
//!
//! The rule holds in every space the pair is ordered in. In the fold's
//! space the pair is A-first and the rank is along the A-first line; in
//! a published union name the pair is in name order and the rank is
//! along the name-first line. Either way the rank is along the line of
//! the pair AS WRITTEN, and a rewrite that reorders the pair reverses
//! the rank.

use super::role::{EntityKind, Qualifier, RoleSeg, StableName, name_free_seg};
use super::seam_pair::{seam_line_pair, seam_vertex_parents};

/// Whether a name's [`RoleSeg::Seam`] sides are an operand order or a
/// name order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Seams {
    /// A pair boolean's seams: `a` is the A operand's entity and `b` the
    /// B operand's, so the two sides stay where they are.
    Sided,
    /// A union's seams: the pair is put in name order, and a rank the
    /// swap reverses is read from the other end by the rule carried.
    ByName(RankRule),
}

/// Whether a PUBLISHED name's seams are in name order, read off the
/// name.
///
/// A pair boolean's seam sides are its OPERANDS' names (N2), minted
/// by other nodes. A union's are its own member-space names, minted
/// by the union: the collapse rewrites every side into this node's
/// space. So a seam whose side is minted by the name's own node is
/// a union's.
///
/// Only published names are read this way. Inside a union's fold
/// every row, a seam's sides included, is minted under the union's
/// id while the pair is still sided; those tables are the pair
/// emitter's, which says [`Seams::Sided`] itself, and they never
/// reach a rewrite (the collapse publishes their rows).
pub(crate) fn published_by_name(name: &StableName) -> bool {
    name.path.iter().any(|seg| match seg {
        RoleSeg::Seam { a, .. } => a.node == name.node,
        _ => false,
    })
}

/// What putting a name's seam pair in name order does to its
/// `Fragment(OrderAlong)` ranks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RankRule {
    /// The rank lies along a direction the ordering does not change: an
    /// edge on no seam line (it ranks along its own carrier), an edge
    /// on a seam line whose pair stays as written, a seam vertex group
    /// whose edge parent is such an edge, a group with no edge parent,
    /// and every face.
    Keep,
    /// An edge on a seam line whose pair comes out swapped, or a seam
    /// vertex group whose edge parent is one. The line is negated, and
    /// the rank reads from the other end: `of − 1 − rank`.
    Reverse,
    /// A seam VERTEX group whose two parents are both edges. The pair
    /// emitter ranks it along the A side's edge, so which carrier was
    /// used depends on operand order and there is no canonical one.
    /// Two straight edges cross at most once, so such a group needs
    /// curved edges, and none is known. The collapse refuses rather
    /// than publish a rank a member reorder could rebind.
    Refuse,
}

/// Why a rank could not be re-read against a canonical head.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Unrankable {
    /// [`RankRule::Refuse`]'s shape carried a rank.
    SidedVertexRank,
    /// A rank at or beyond its count.
    RankOutsideCount,
}

impl Unrankable {
    /// The emission bug this is, in `NamingError::Emission`'s words.
    pub(crate) fn what(self) -> &'static str {
        match self {
            Self::SidedVertexRank => {
                "a union's seam vertex group is ranked along one of two edge parents, chosen by \
                 operand side"
            }
            Self::RankOutsideCount => "a seam chain's rank lies outside its count",
        }
    }
}

impl RankRule {
    /// The rule for `was`'s ranks once every name it embeds is carried
    /// to its image under `image` (itself canonical) and the seam pairs
    /// are put in name order.
    ///
    /// An EDGE's ranks lie along the seam line [`seam_line_pair`] finds
    /// for it: the edge's own pair, or the pair of the seam it is a
    /// piece of, through any wrapping. That is the line the pair
    /// emitter ranked it along (`emit_topo`'s `seam_line_dir`, the same
    /// answer). A seam VERTEX group's rank lies along its edge parent's
    /// line, found through the same wrapping. The pair is swapped
    /// exactly where the images of its two sides come out in the other
    /// order.
    ///
    /// # Errors
    ///
    /// Whatever `image` refuses.
    pub(crate) fn across<E>(
        was: &StableName,
        image: &mut dyn FnMut(&StableName) -> Result<StableName, E>,
    ) -> Result<Self, E> {
        match (was.kind, seam_vertex_parents(was)) {
            (EntityKind::Edge, _) => Self::along(was, image),
            (EntityKind::Vertex, Some((a, b))) => {
                match (a.kind == EntityKind::Edge, b.kind == EntityKind::Edge) {
                    (true, true) => Ok(Self::Refuse),
                    (true, false) => Self::along(a, image),
                    (false, true) => Self::along(b, image),
                    (false, false) => Ok(Self::Keep),
                }
            }
            (EntityKind::Vertex | EntityKind::Face | EntityKind::Body, _) => Ok(Self::Keep),
        }
    }

    /// The rule for a rank along EDGE `edge`'s line.
    fn along<E>(
        edge: &StableName,
        image: &mut dyn FnMut(&StableName) -> Result<StableName, E>,
    ) -> Result<Self, E> {
        Ok(match seam_line_pair(edge) {
            None => Self::Keep,
            Some((a, b)) if image(a)? > image(b)? => Self::Reverse,
            Some(_) => Self::Keep,
        })
    }

    /// One rank, re-read against the canonical head.
    fn apply(self, rank: u32, of: u32) -> Result<Qualifier, Unrankable> {
        match self {
            Self::Keep => Ok(Qualifier::OrderAlong { rank, of }),
            Self::Reverse => of
                .checked_sub(1)
                .and_then(|last| last.checked_sub(rank))
                .map(|rank| Qualifier::OrderAlong { rank, of })
                .ok_or(Unrankable::RankOutsideCount),
            Self::Refuse => Err(Unrankable::SidedVertexRank),
        }
    }
}

/// **`name` in canonical form, at an emission door**: the mint and the
/// collapse. Every name-ordered position is put in order, and under
/// [`Seams::ByName`] every rank is re-read by the rule.
///
/// Only the name's OWN path is ordered. A name it embeds is canonical
/// already, because it was minted, collapsed or rewritten through here
/// in its own right.
///
/// # Errors
///
/// A rank the rule cannot re-read ([`Unrankable`]).
pub(crate) fn canonicalize(name: StableName, seams: Seams) -> Result<StableName, Unrankable> {
    let by_name = matches!(seams, Seams::ByName(_));
    let mut name = order(name, by_name);
    if let Seams::ByName(rule) = seams {
        for seg in &mut name.path {
            if let RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }) = seg {
                *seg = RoleSeg::Fragment(rule.apply(*rank, *of)?);
            }
        }
    }
    Ok(name)
}

/// **A pair emitter's name in canonical form**: [`canonicalize`] under
/// [`Seams::Sided`], which re-reads no rank and so cannot refuse. The
/// mint's door (`emit_topo`).
pub(crate) fn sided(name: StableName) -> StableName {
    order(name, false)
}

/// **One freshly minted segment in canonical form**, for an emitter
/// that mints a segment rather than a whole path (`emit_blend`'s band
/// face). No seam is ordered by name outside a union's path.
pub(crate) fn sided_segment(seg: RoleSeg) -> RoleSeg {
    segment(seg, false)
}

/// **`name` in canonical form, after a rewrite of a published name**.
/// The same form as [`canonicalize`], with one difference: a rank the
/// rule cannot re-read is carried as written.
///
/// A rewrite re-spells a reference; it is not a door. Both refusals are
/// emission-time facts: the collapse refuses [`RankRule::Refuse`]'s
/// shape, and no emitter mints a rank at or past its count. A name
/// carrying either was never published, so it resolved to nothing
/// before the rewrite and resolves to nothing after it, however its
/// rank is spelled; every other malformed name crosses a rewrite the
/// same way.
pub(crate) fn recanonicalize(name: StableName, seams: Seams) -> StableName {
    let by_name = matches!(seams, Seams::ByName(_));
    let mut name = order(name, by_name);
    if let Seams::ByName(rule) = seams {
        for seg in &mut name.path {
            if let RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }) = seg
                && let Ok(q) = rule.apply(*rank, *of)
            {
                *seg = RoleSeg::Fragment(q);
            }
        }
    }
    name
}

/// The ordering half, with the ranks untouched.
fn order(mut name: StableName, by_name: bool) -> StableName {
    name.path = name
        .path
        .into_iter()
        .map(|seg| segment(seg, by_name))
        .collect();
    if is_junction(&name) {
        name.path.sort();
    }
    name
}

/// Whether `name` is a seam JUNCTION: a vertex named by a run of two
/// or more `Seam` lines and nothing else.
pub(crate) fn is_junction(name: &StableName) -> bool {
    name.kind == EntityKind::Vertex
        && name.path.len() >= 2
        && name
            .path
            .iter()
            .all(|seg| matches!(seg, RoleSeg::Seam { .. }))
}

/// **One segment's own name-ordered positions**, in order.
///
/// EXHAUSTIVE, with no wildcard (the `walk_names` rule): a variant
/// added to [`RoleSeg`] says here whether it carries a name-ordered
/// position, or stops the build.
#[allow(clippy::too_many_lines)] // one arm per name-carrying RoleSeg variant
fn segment(seg: RoleSeg, by_name: bool) -> RoleSeg {
    match seg {
        // The name-ordered positions.
        RoleSeg::Seam { a, b } if by_name && a > b => RoleSeg::Seam { a: b, b: a },
        RoleSeg::Merged(set) => RoleSeg::Merged(sorted_set(set)),
        RoleSeg::BandFace(set) => RoleSeg::BandFace(sorted_set(set)),
        RoleSeg::Fragment(Qualifier::SideOf(mut partners)) => {
            partners.sort();
            RoleSeg::Fragment(Qualifier::SideOf(partners))
        }
        // Everything else carries its names, or none, in an order of
        // its own.
        RoleSeg::Seam { .. }
        | RoleSeg::Fragment(Qualifier::OrderAlong { .. })
        | RoleSeg::FromA(_)
        | RoleSeg::FromB(_)
        | RoleSeg::FromMember { .. }
        | RoleSeg::SectionEdge { .. }
        | RoleSeg::SplitFragment { .. }
        | RoleSeg::CrossingVertex { .. }
        | RoleSeg::OnToolVertex { .. }
        | RoleSeg::FromTarget(_)
        | RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::EndArc { .. }
        | RoleSeg::BandTrim { .. }
        | RoleSeg::BandFoot(_)
        | RoleSeg::BandCross(_)
        | RoleSeg::BandCut(_)
        | RoleSeg::BandSlit(_)
        | RoleSeg::Inner(_)
        | RoleSeg::Rim(_)
        | RoleSeg::HoleRim { .. }
        | RoleSeg::InPart { .. }
        | RoleSeg::Instance { .. }
        | name_free_seg!() => seg,
    }
}

/// A set of names in its canonical order: sorted, each name once.
fn sorted_set(mut set: Vec<StableName>) -> Vec<StableName> {
    set.sort();
    set.dedup();
    set
}

#[cfg(test)]
mod tests {
    //! Per segment kind: the canonical form is idempotent, and it does
    //! not depend on the order the positions were written in.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::NameRef;
    use crate::names::role::{CapEnd, SideVerdict};
    use crate::node::RecipeNodeId;

    fn seam(a: StableName, b: StableName) -> RoleSeg {
        RoleSeg::Seam {
            a: NameRef::new(a),
            b: NameRef::new(b),
        }
    }

    fn face(node: u64, member: u64) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![RoleSeg::FromMember {
                member: RecipeNodeId(member),
                of: NameRef::new(StableName {
                    kind: EntityKind::Face,
                    node: RecipeNodeId(member),
                    path: vec![RoleSeg::Cap(CapEnd::Start)],
                }),
            }],
        }
    }

    fn edge(node: u64, member: u64) -> StableName {
        StableName {
            kind: EntityKind::Edge,
            ..face(node, member)
        }
    }

    fn name(kind: EntityKind, node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId(node),
            path,
        }
    }

    /// Every permutation of `xs` (at most a handful of items).
    fn permutations<T: Clone>(xs: &[T]) -> Vec<Vec<T>> {
        if xs.len() <= 1 {
            return vec![xs.to_vec()];
        }
        let mut out = Vec::new();
        for i in 0..xs.len() {
            let mut rest = xs.to_vec();
            let x = rest.remove(i);
            for mut p in permutations(&rest) {
                p.insert(0, x.clone());
                out.push(p);
            }
        }
        out
    }

    /// `build` over every order of `items`, canonicalized under `seams`:
    /// one canonical form for all of them, and that form is a fixed
    /// point.
    fn one_form<T: Clone>(
        items: &[T],
        seams: Seams,
        build: impl Fn(Vec<T>) -> StableName,
    ) -> StableName {
        let forms: Vec<StableName> = permutations(items)
            .into_iter()
            .map(|p| canonicalize(build(p), seams).unwrap())
            .collect();
        for f in &forms {
            assert_eq!(f, &forms[0], "the canonical form depends on the written order");
        }
        assert_eq!(
            canonicalize(forms[0].clone(), seams).unwrap(),
            forms[0],
            "the canonical form is not a fixed point"
        );
        assert_eq!(recanonicalize(forms[0].clone(), seams), forms[0]);
        forms[0].clone()
    }

    #[test]
    fn a_merged_set_is_one_form_in_every_order() {
        let cs = [face(9, 3), face(9, 1), face(9, 2), face(9, 1)];
        for seams in [Seams::Sided, Seams::ByName(RankRule::Keep)] {
            let out = one_form(&cs, seams, |p| {
                name(EntityKind::Face, 9, vec![RoleSeg::Merged(p)])
            });
            assert_eq!(
                out.path,
                vec![RoleSeg::Merged(vec![face(9, 1), face(9, 2), face(9, 3)])],
                "sorted, each constituent once"
            );
        }
    }

    #[test]
    fn a_band_face_set_is_one_form_in_every_order() {
        let es = [edge(4, 2), edge(4, 0), edge(4, 1), edge(4, 0)];
        let out = one_form(&es, Seams::Sided, |p| {
            name(EntityKind::Face, 5, vec![RoleSeg::BandFace(p)])
        });
        assert_eq!(
            out.path,
            vec![RoleSeg::BandFace(vec![edge(4, 0), edge(4, 1), edge(4, 2)])]
        );
    }

    #[test]
    fn a_side_of_vector_is_one_form_in_every_order() {
        let ps = [
            (face(9, 2), SideVerdict::Negative),
            (face(9, 1), SideVerdict::Positive),
            (face(9, 3), SideVerdict::Positive),
        ];
        for seams in [Seams::Sided, Seams::ByName(RankRule::Keep)] {
            let out = one_form(&ps, seams, |p| {
                name(
                    EntityKind::Face,
                    9,
                    vec![
                        RoleSeg::Cap(CapEnd::Start),
                        RoleSeg::Fragment(Qualifier::SideOf(p)),
                    ],
                )
            });
            let Some(RoleSeg::Fragment(Qualifier::SideOf(v))) = out.path.last() else {
                panic!("the qualifier is kept");
            };
            let partners: Vec<&StableName> = v.iter().map(|(n, _)| n).collect();
            assert_eq!(partners, vec![&face(9, 1), &face(9, 2), &face(9, 3)]);
        }
    }

    #[test]
    fn a_junction_run_is_one_form_in_every_order() {
        let lines = [
            (face(9, 2), face(9, 3)),
            (face(9, 1), face(9, 2)),
            (face(9, 1), face(9, 3)),
        ];
        let build = |p: Vec<(StableName, StableName)>| {
            name(
                EntityKind::Vertex,
                9,
                p.into_iter().map(|(a, b)| seam(a, b)).collect(),
            )
        };
        // Sided: the run is sorted and each line keeps its sides.
        let sided = one_form(&lines, Seams::Sided, build);
        assert!(sided.path.windows(2).all(|w| w[0] < w[1]));
        // By name: each line's sides AND the run are in name order, so
        // a line written the other way round is the same line.
        let flipped: Vec<(StableName, StableName)> =
            lines.iter().map(|(a, b)| (b.clone(), a.clone())).collect();
        let by_name = one_form(&lines, Seams::ByName(RankRule::Keep), build);
        assert_eq!(
            by_name,
            one_form(&flipped, Seams::ByName(RankRule::Keep), build)
        );
        assert_eq!(
            by_name.path,
            vec![
                seam(face(9, 1), face(9, 2)),
                seam(face(9, 1), face(9, 3)),
                seam(face(9, 2), face(9, 3)),
            ]
        );
    }

    #[test]
    fn a_union_seam_edge_is_one_form_from_either_side_and_its_rank_follows() {
        let (x, y) = (face(9, 1), face(9, 2));
        // The pair written y-first, ranked 0 of 3 along n_y × n_x.
        let written = name(
            EntityKind::Edge,
            9,
            vec![
                seam(y.clone(), x.clone()),
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 3 }),
            ],
        );
        let rule = RankRule::across(&written, &mut |n| Ok::<_, ()>(n.clone())).unwrap();
        assert_eq!(rule, RankRule::Reverse);
        let out = canonicalize(written, Seams::ByName(rule)).unwrap();
        assert_eq!(
            out.path,
            vec![
                seam(x.clone(), y.clone()),
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 2, of: 3 }),
            ],
            "the pair in name order, the rank read from the other end"
        );
        // Canonical in, canonical out: the rule of a canonical name is
        // Keep, so the form is a fixed point.
        let again = RankRule::across(&out, &mut |n| Ok::<_, ()>(n.clone())).unwrap();
        assert_eq!(again, RankRule::Keep);
        assert_eq!(canonicalize(out.clone(), Seams::ByName(again)).unwrap(), out);
        // A pair boolean's seam is sided: nothing moves.
        let sided = name(
            EntityKind::Edge,
            9,
            vec![
                seam(y.clone(), x.clone()),
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 3 }),
            ],
        );
        assert_eq!(canonicalize(sided.clone(), Seams::Sided).unwrap(), sided);
    }

    #[test]
    fn a_rank_that_cannot_be_reread_refuses_at_a_door_and_is_carried_by_a_rewrite() {
        let (x, y) = (face(9, 1), face(9, 2));
        let bad = name(
            EntityKind::Edge,
            9,
            vec![
                seam(y, x),
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 3, of: 3 }),
            ],
        );
        assert_eq!(
            canonicalize(bad.clone(), Seams::ByName(RankRule::Reverse)),
            Err(Unrankable::RankOutsideCount)
        );
        let carried = recanonicalize(bad, Seams::ByName(RankRule::Reverse));
        assert_eq!(
            carried.path.last(),
            Some(&RoleSeg::Fragment(Qualifier::OrderAlong { rank: 3, of: 3 }))
        );
    }

    #[test]
    fn a_published_union_name_orders_its_seams_by_name_and_a_pair_booleans_does_not() {
        let union = name(EntityKind::Edge, 9, vec![seam(face(9, 1), face(9, 2))]);
        assert!(published_by_name(&union));
        let pair = name(EntityKind::Edge, 9, vec![seam(face(7, 1), face(8, 2))]);
        assert!(!published_by_name(&pair));
    }
}
