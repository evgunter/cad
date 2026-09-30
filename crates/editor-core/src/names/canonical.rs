//! **A role path's canonical form**: which positions in a path are in
//! NAME ORDER, and what putting a seam's sides there does to a rank.
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
//!   else: the run is sorted and NOT deduplicated. The lines are
//!   distinct by construction — the mint gathers them as a set, and a
//!   rewrite that makes two of them equal is not one-to-one, which the
//!   collapse refuses — so a repeat is a fault to refuse, never a
//!   duplicate to drop;
//! - a [`RoleSeg::Seam`]'s two sides, in a name whose seams are
//!   [`Seams::ByName`]: a union has no A and B, so its pair is put in
//!   name order. A pair boolean's seams are [`Seams::Sided`]: `a` is
//!   the A operand's entity, and that order is data.
//!
//! Any rewrite that changes the names in those positions has to put
//! them back in order, or it publishes a name the emitter would not
//! mint for the same entity in the rewritten space. This module is the
//! one list of those positions. Every path is built through it: the
//! pair emitter's mint ([`minted`]: `emit_topo`, `emit_blend`), the
//! union's collapse ([`collapsed`]: `emit_union`), and every rewrite of
//! a published name ([`rewritten`], through
//! [`StableName::rewrite_path`]: the anchor, the whole-program edit and
//! the split re-map).
//!
//! # A value that depends on a name order
//!
//! A seam line's `Fragment(OrderAlong)` ranks are measured along
//! `n_a × n_b`, the line oriented by its pair's first side AS WRITTEN,
//! and that pair need not be in the ranked name's own path: the pieces
//! of a union's seam that a later boolean cut are `[FromA(<union seam
//! edge>), OrderAlong]`, ranked along a pair written inside the name
//! they embed. So a rewrite that reorders a pair anywhere below a rank —
//! the collapse ordering a union seam, or a re-map reordering the sides
//! of a union seam some other name embeds — reverses every rank along
//! that line: `of − 1 − rank`. A seam VERTEX group's rank lies along
//! its edge parent's line, and follows that line.
//!
//! That rule is [`RankRule`], and it is derived HERE, from the name as
//! it was and the name as the rewrite leaves it: the line each rank lies
//! on is found in both ([`seam_line_pair`], through every wrapper that
//! carries an entity), and the rank reverses exactly where the new line's
//! pair is the old pair's images in the other order. Nothing hands a
//! rule in, so no caller can apply one to a name it was not derived
//! from, and a name already canonical derives `Keep` for every rank.

use super::role::{EntityKind, Qualifier, RoleSeg, StableName, name_free_seg};
use super::seam_pair::{seam_line_pair, seam_vertex_parents};

/// Whether a name's own [`RoleSeg::Seam`] sides are an operand order
/// or a name order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Seams {
    /// A pair boolean's seams: `a` is the A operand's entity and `b` the
    /// B operand's, so the two sides stay where they are.
    Sided,
    /// A union's seams: the pair is put in name order.
    ByName,
}

impl Seams {
    /// The order a PUBLISHED name's own seams are in, read off the name.
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
    /// emitter's, which mints through [`minted`], and they never reach
    /// a rewrite (the collapse publishes their rows).
    pub(crate) fn of_published(name: &StableName) -> Self {
        let union = name.path.iter().any(|seg| match seg {
            RoleSeg::Seam { a, .. } => a.node == name.node,
            _ => false,
        });
        if union { Self::ByName } else { Self::Sided }
    }
}

/// What a rewrite did to the line a name's `Fragment(OrderAlong)` ranks
/// lie on (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RankRule {
    /// The ranks lie along a direction the rewrite did not change: an
    /// edge on no seam line (it ranks along its own carrier), an edge
    /// on a seam line whose pair kept its order, a seam vertex group
    /// whose edge parent is such an edge, a group with no edge parent,
    /// and every face.
    Keep,
    /// An edge on a seam line whose pair came out in the other order,
    /// or a seam vertex group whose edge parent is one. The line is
    /// negated, and the rank reads from the other end: `of − 1 − rank`.
    Reverse,
    /// A UNION's seam vertex group whose two parents are both edges.
    /// The pair emitter ranks it along the A side's edge, so which
    /// carrier was used depends on operand order and a union has no
    /// canonical one. Two straight edges cross at most once, so such a
    /// group needs curved edges, and none is known. The collapse
    /// refuses rather than publish a rank a member reorder could
    /// rebind.
    Refuse,
}

/// Why a rank could not be re-read against a canonical line.
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

/// Why [`canonical`] stopped: the rewrite's image refused, or a rank
/// could not be re-read.
pub(crate) enum Stop<E> {
    /// The caller's image of an embedded name refused.
    Image(E),
    /// A rank could not be re-read.
    Unrankable(Unrankable),
}

/// The image a rewrite gives an embedded name ([`canonical`]).
pub(crate) type Image<'a, E> = &'a mut dyn FnMut(&StableName) -> Result<StableName, E>;

impl RankRule {
    /// The rule for `now`'s ranks, `now` being `was` rewritten — every
    /// embedded name carried to its image under `image` — and put in
    /// order (module docs).
    ///
    /// An EDGE's ranks lie along the seam line [`seam_line_pair`] finds
    /// for it — the edge's own pair, or the pair of the seam it is a
    /// piece of, through every wrapper that carries an entity — which
    /// is the line the pair emitter ranked it along (`emit_topo`'s
    /// `seam_line_dir`, the same answer). A seam VERTEX group's rank
    /// lies along its edge parent's line ([`seam_vertex_parents`]):
    /// the one edge parent, or, where both are edges, the A side's in
    /// a pair boolean's name and none in a union's.
    fn derive<E>(
        was: &StableName,
        now: &StableName,
        seams: Seams,
        image: Image<'_, E>,
    ) -> Result<Self, E> {
        let is_edge = |n: &StableName| n.kind == EntityKind::Edge;
        match (was.kind, seam_vertex_parents(was), seam_vertex_parents(now)) {
            (EntityKind::Edge, ..) => Self::along(was, now, image),
            (EntityKind::Vertex, Some((a, b)), Some((a2, b2))) => {
                // The edge parent in `now`: the one of kind edge, which a
                // rewrite does not change, wherever ordering put it.
                let edge_now = if is_edge(a2) { a2 } else { b2 };
                match (is_edge(a), is_edge(b), seams) {
                    (true, true, Seams::ByName) => Ok(Self::Refuse),
                    (true, true, Seams::Sided) => Self::along(a, a2, image),
                    (true, false, _) => Self::along(a, edge_now, image),
                    (false, true, _) => Self::along(b, edge_now, image),
                    (false, false, _) => Ok(Self::Keep),
                }
            }
            (EntityKind::Vertex | EntityKind::Face | EntityKind::Body, ..) => Ok(Self::Keep),
        }
    }

    /// The rule for a rank along EDGE `was`'s line, the edge rewritten
    /// to `now`: `Reverse` exactly where `now`'s pair is the images of
    /// `was`'s pair in the other order.
    fn along<E>(was: &StableName, now: &StableName, image: Image<'_, E>) -> Result<Self, E> {
        let (Some((a, b)), Some((a2, b2))) = (seam_line_pair(was), seam_line_pair(now)) else {
            return Ok(Self::Keep);
        };
        let (ia, ib) = (image(a)?, image(b)?);
        Ok(if *a2 == ib && *b2 == ia && ia != ib {
            Self::Reverse
        } else {
            Self::Keep
        })
    }

    /// One rank, re-read against the canonical line.
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

/// **The canonical form of `now`, the rewrite of `was`**: every
/// name-ordered position in order, and every rank re-read by the rule
/// [`RankRule::derive`] finds for the two.
///
/// Only the name's OWN positions are ordered: a name it embeds reaches
/// here already rewritten through this same function by the caller's
/// `image` (the collapse and every rewriter canonicalize each embedded
/// name on the way down). Its RANKS are the exception, since the line
/// they lie on may be written in an embedded name, which is why the
/// rule reads both names whole.
///
/// # Errors
///
/// [`Stop::Image`] where `image` refuses; [`Stop::Unrankable`] where a
/// rank cannot be re-read.
pub(crate) fn canonical<E>(
    was: &StableName,
    now: StableName,
    seams: Seams,
    image: Image<'_, E>,
) -> Result<StableName, Stop<E>> {
    let mut now = order(now, seams);
    if !has_rank(&now) {
        return Ok(now);
    }
    let rule = RankRule::derive(was, &now, seams, image).map_err(Stop::Image)?;
    for seg in &mut now.path {
        if let RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }) = seg {
            *seg = RoleSeg::Fragment(rule.apply(*rank, *of).map_err(Stop::Unrankable)?);
        }
    }
    Ok(now)
}

/// **A name the pair emitter just minted**, in canonical form: its
/// positions ordered with its seams [`Seams::Sided`]. Its ranks were
/// measured along the pair it writes, so there is no earlier spelling
/// to re-read them against: [`canonical`] with the name as its own
/// `was` and nothing rewritten, which derives `Keep` for every rank.
pub(crate) fn minted(name: StableName) -> StableName {
    order(name, Seams::Sided)
}

/// **One freshly minted segment in canonical form**, for an emitter
/// that mints a segment rather than a whole path (`emit_blend`'s band
/// face): [`minted`]'s ordering, one segment at a time.
pub(crate) fn minted_segment(seg: RoleSeg) -> RoleSeg {
    segment(seg, Seams::Sided)
}

/// **A fold-table name collapsed into a union's space**, in canonical
/// form: [`canonical`] with the union's seams [`Seams::ByName`]. A rank
/// it cannot re-read is refused: the collapse is an emission door.
pub(crate) fn collapsed<E>(
    was: &StableName,
    now: StableName,
    image: Image<'_, E>,
) -> Result<StableName, Stop<E>> {
    canonical(was, now, Seams::ByName, image)
}

/// **A published name after a rewrite**, in canonical form:
/// [`canonical`] with the name's own seams in the order
/// [`Seams::of_published`] reads off it.
///
/// A name with a rank that cannot be re-read is carried EXACTLY as the
/// segment rewrite left it — nothing reordered, nothing re-ranked. A
/// rewrite re-spells a reference; it is not a door. Both refusals are
/// emission-time facts: the collapse refuses [`RankRule::Refuse`]'s
/// shape, and no emitter mints a rank at or past its count. A name
/// carrying either was never published, so it resolved to nothing
/// before the rewrite and resolves to nothing after it; every other
/// malformed name crosses a rewrite the same way.
///
/// # Errors
///
/// Whatever `image` refuses.
pub(crate) fn rewritten<E>(
    was: &StableName,
    now: StableName,
    image: Image<'_, E>,
) -> Result<StableName, E> {
    let seams = Seams::of_published(was);
    if !has_rank(&now) {
        return Ok(order(now, seams));
    }
    let carried = now.clone();
    match canonical(was, now, seams, image) {
        Ok(name) => Ok(name),
        Err(Stop::Image(e)) => Err(e),
        Err(Stop::Unrankable(_)) => Ok(carried),
    }
}

/// The ordering half: every position in order, the ranks untouched.
fn order(mut name: StableName, seams: Seams) -> StableName {
    name.path = name
        .path
        .into_iter()
        .map(|seg| segment(seg, seams))
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
fn segment(seg: RoleSeg, seams: Seams) -> RoleSeg {
    match seg {
        // The name-ordered positions.
        RoleSeg::Seam { a, b } if seams == Seams::ByName && a > b => RoleSeg::Seam { a: b, b: a },
        RoleSeg::Merged(set) => RoleSeg::Merged(sorted_set(set)),
        RoleSeg::BandFace(set) => RoleSeg::BandFace(sorted_set(set)),
        RoleSeg::BandCross { edge, band } => RoleSeg::BandCross {
            edge,
            band: sorted_set(band),
        },
        RoleSeg::BandSlit { edge, band } => RoleSeg::BandSlit {
            edge,
            band: sorted_set(band),
        },
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
        | RoleSeg::BandCut(_)
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

/// Whether `name` carries a rank to re-read.
fn has_rank(name: &StableName) -> bool {
    name.path
        .iter()
        .any(|seg| matches!(seg, RoleSeg::Fragment(Qualifier::OrderAlong { .. })))
}

#[cfg(test)]
mod tests {
    //! Per segment kind: the canonical form is idempotent, and it does
    //! not depend on the order the positions were written in. And the
    //! rank rule: derived from the two spellings, whichever name the
    //! line is written in.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::{CapEnd, NameRef, SideVerdict};
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

    fn rank(rank: u32, of: u32) -> RoleSeg {
        RoleSeg::Fragment(Qualifier::OrderAlong { rank, of })
    }

    /// The identity image: the name written is the name meant.
    fn same(n: &StableName) -> Result<StableName, ()> {
        Ok(n.clone())
    }

    /// `written` in canonical form, read as its own earlier spelling.
    fn form(written: StableName, seams: Seams) -> StableName {
        let was = written.clone();
        match canonical(&was, written, seams, &mut same) {
            Ok(n) => n,
            Err(_) => panic!("refused"),
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

    /// `build` over every order of `items`, in canonical form under
    /// `seams`: one form for all of them, and that form is a fixed
    /// point of [`canonical`], [`minted`] (Sided) and [`rewritten`].
    fn one_form<T: Clone>(
        items: &[T],
        seams: Seams,
        build: impl Fn(Vec<T>) -> StableName,
    ) -> StableName {
        let forms: Vec<StableName> = permutations(items)
            .into_iter()
            .map(|p| form(build(p), seams))
            .collect();
        for f in &forms {
            assert_eq!(
                f, &forms[0],
                "the canonical form depends on the written order"
            );
        }
        let f = forms[0].clone();
        assert_eq!(form(f.clone(), seams), f, "not a fixed point");
        if seams == Seams::Sided {
            assert_eq!(minted(f.clone()), f);
        }
        assert_eq!(rewritten(&f, f.clone(), &mut same).unwrap(), f);
        f
    }

    #[test]
    fn a_merged_set_is_one_form_in_every_order() {
        let cs = [face(9, 3), face(9, 1), face(9, 2), face(9, 1)];
        for seams in [Seams::Sided, Seams::ByName] {
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
        assert_eq!(
            minted_segment(RoleSeg::BandFace(es.to_vec())),
            out.path[0],
            "the one-segment door is the same ordering"
        );
    }

    #[test]
    fn a_side_of_vector_is_one_form_in_every_order() {
        let ps = [
            (face(9, 2), SideVerdict::Negative),
            (face(9, 1), SideVerdict::Positive),
            (face(9, 3), SideVerdict::Positive),
        ];
        for seams in [Seams::Sided, Seams::ByName] {
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
        let by_name = one_form(&lines, Seams::ByName, build);
        assert_eq!(by_name, one_form(&flipped, Seams::ByName, build));
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
    fn a_union_seam_edge_is_one_form_from_either_side_with_its_rank_read_from_the_other_end() {
        // A chain of three along one union seam, written from either
        // side: rank r along n_y × n_x is rank 2 − r along n_x × n_y,
        // so the two spellings of each piece are one form.
        let (x, y) = (face(9, 1), face(9, 2));
        for r in 0..3 {
            let from_y = name(
                EntityKind::Edge,
                9,
                vec![seam(y.clone(), x.clone()), rank(r, 3)],
            );
            let from_x = name(
                EntityKind::Edge,
                9,
                vec![seam(x.clone(), y.clone()), rank(2 - r, 3)],
            );
            let out = form(from_y, Seams::ByName);
            assert_eq!(out, form(from_x, Seams::ByName));
            assert_eq!(
                out.path,
                vec![seam(x.clone(), y.clone()), rank(2 - r, 3)],
                "the pair in name order, the rank read from the other end"
            );
            assert_eq!(form(out.clone(), Seams::ByName), out, "not a fixed point");
        }
        // A pair boolean's seam is sided: nothing moves.
        let sided = name(
            EntityKind::Edge,
            9,
            vec![seam(y.clone(), x.clone()), rank(0, 3)],
        );
        assert_eq!(form(sided.clone(), Seams::Sided), sided);
    }

    #[test]
    fn a_rank_along_a_seam_an_embedded_name_writes_follows_that_seams_order() {
        // A pair boolean (node 12) cut a union's (node 9) seam edge in
        // two: the pieces are ranked along the union seam's line, which
        // is written INSIDE the embedded name. A rewrite that reorders
        // that seam's sides reverses the rank, though the outer name
        // has no seam of its own.
        let (x, y) = (face(9, 1), face(9, 2));
        let union_seam = |a: &StableName, b: &StableName| {
            name(EntityKind::Edge, 9, vec![seam(a.clone(), b.clone())])
        };
        let piece = |inner: StableName, r| {
            name(
                EntityKind::Edge,
                12,
                vec![RoleSeg::FromA(NameRef::new(inner)), rank(r, 2)],
            )
        };
        let was = piece(union_seam(&x, &y), 0);
        // The rewrite swaps x and y (a re-map that reorders members),
        // and the embedded union name comes out re-ordered: (x', y')
        // with y' < x'.
        let (x2, y2) = (face(9, 5), face(9, 4));
        let mut image = |n: &StableName| -> Result<StableName, ()> {
            Ok(if *n == x {
                x2.clone()
            } else if *n == y {
                y2.clone()
            } else {
                n.clone()
            })
        };
        let now = piece(union_seam(&y2, &x2), 0);
        let out = rewritten(&was, now, &mut image).unwrap();
        assert_eq!(out, piece(union_seam(&y2, &x2), 1));
        assert_eq!(
            Seams::of_published(&was),
            Seams::Sided,
            "no seam of its own"
        );
    }

    #[test]
    fn a_rank_that_cannot_be_reread_refuses_at_a_door_and_is_carried_untouched_by_a_rewrite() {
        let (x, y) = (face(9, 1), face(9, 2));
        let bad = name(EntityKind::Edge, 9, vec![seam(y, x), rank(3, 3)]);
        assert!(matches!(
            collapsed(&bad, bad.clone(), &mut same),
            Err(Stop::Unrankable(Unrankable::RankOutsideCount))
        ));
        assert_eq!(
            rewritten(&bad, bad.clone(), &mut same).unwrap(),
            bad,
            "nothing reordered, nothing re-ranked"
        );
    }

    #[test]
    fn a_published_union_name_orders_its_seams_by_name_and_a_pair_booleans_does_not() {
        let union = name(EntityKind::Edge, 9, vec![seam(face(9, 1), face(9, 2))]);
        assert_eq!(Seams::of_published(&union), Seams::ByName);
        let pair = name(EntityKind::Edge, 9, vec![seam(face(7, 1), face(8, 2))]);
        assert_eq!(Seams::of_published(&pair), Seams::Sided);
    }
}
