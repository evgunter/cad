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
//! - a [`Qualifier::Borders`] or [`Qualifier::Keeps`] set: sorted, and
//!   deduplicated, because the set is the qualifier (N2);
//! - a [`Qualifier::Ends`] pair: sorted and NOT deduplicated, because a
//!   piece whose two ends share a name is still a piece with two ends;
//! - a seam JUNCTION, the vertex where k ≥ 2 seam lines meet, named by
//!   the run of those lines' [`RoleSeg::Seam`] segments and nothing
//!   else: the run is sorted and NOT deduplicated. The lines are
//!   distinct by construction — the mint gathers them as a set, and a
//!   rewrite that makes two of them equal is not one-to-one, which the
//!   collapse refuses — so a repeat is a fault to refuse, never a
//!   duplicate to drop;
//! - a [`RoleSeg::EdgeCrossing`]'s run, each line with its sense:
//!   sorted, in every name;
//! - a [`RoleSeg::Seam`]'s two sides, in every name: the pair spells
//!   a set, and which input a side came from is in the side's own name,
//!   the face's name in the minting node's own table (N3).
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
//! A seam vertex group's `Fragment(OrderAlong)` ranks are measured along
//! the crossed edge. Where that edge lies on a seam line, it is oriented
//! as the loop of its pair's first side, the lesser name, runs along it,
//! and that pair need not be in the ranked name's own path: it is
//! written inside the edge parent the vertex name embeds. So a rewrite
//! whose images put that pair in the other order — the collapse
//! carrying a fold step's seam into the union's space, or a re-map of
//! a published name — reverses every rank along it: `of − 1 − rank`.
//!
//! That rule is [`RankRule`], and it is derived HERE, from the name as
//! it was and the name as the rewrite leaves it: the line each rank lies
//! on is found in both ([`seam_line_pair`], through every wrapper that
//! carries an entity), and the rank reverses exactly where the new line's
//! pair is the old pair's images in the other order. Nothing hands a
//! rule in, so no caller can apply one to a name it was not derived
//! from, and a name already canonical derives `Keep` for every rank.
//! Edge pieces carry no rank (`Qualifier::Ends` orders nothing), so an
//! edge's own name derives `Keep`.
//!
//! A crossing's [`Sense`](super::role::Sense) is read along the same
//! edge, so the same reordering flips it ([`flip_senses`]): a
//! `Crossing`'s along its edge, and each of an `EdgeCrossing`'s edges
//! along that edge.

use super::role::{EntityKind, Qualifier, RoleSeg, RunLine, StableName, name_free_seg};
use super::seam_pair::{VertexLines, seam_line_pair, seam_vertex_parents};

/// What a rewrite did to the line a name's `Fragment(OrderAlong)` ranks
/// lie on (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RankRule {
    /// The ranks lie along a direction the rewrite did not change: a
    /// seam vertex group whose edge parent lies on no seam line, or on
    /// one whose pair kept its order, a group with no edge parent, and
    /// every edge and face.
    Keep,
    /// A seam vertex group whose edge parent lies on a seam line whose
    /// pair came out in the other order. The edge is read the other
    /// way, and the rank from the other end: `of − 1 − rank`.
    Reverse,
    /// A run whose ranks the rewrite moved off the line they were
    /// measured along: its least edge is the image of another line, or,
    /// in a run with no edge, a seam's pair or the lines' order changed.
    /// Where the run lies along the new least line no name says.
    RefuseRun,
    /// A seam vertex group whose pair the rewrite made a run, or whose
    /// run it made a pair: no line holds the rank on both sides of it.
    Reshaped,
}

/// Why a rank could not be re-read against a canonical line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Unrankable {
    /// [`RankRule::RefuseRun`]: a run's least line moved.
    RunLeastLineMoved,
    /// [`RankRule::Reshaped`]: a pair became a run, or a run a pair.
    Reshaped,
    /// A rank at or beyond its count.
    RankOutsideCount,
}

impl Unrankable {
    /// The emission bug this is, in `NamingError::Emission`'s words.
    pub(crate) fn what(self) -> &'static str {
        match self {
            Self::RunLeastLineMoved => {
                "a ranked edge-crossing run's least line is another line once rewritten, so \
                 nothing says where its vertices lie along it"
            }
            Self::Reshaped => {
                "a ranked seam vertex group's lines are a pair on one side of a rewrite and a run \
                 on the other"
            }
            Self::RankOutsideCount => "a seam vertex group's rank lies outside its count",
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
pub(crate) type Image<'a, 'n, E> = &'a mut dyn FnMut(&'n StableName) -> Result<StableName, E>;

impl RankRule {
    /// The rule for `now`'s ranks, `now` being `was` rewritten — every
    /// embedded name carried to its image under `image` — and put in
    /// order (module docs).
    ///
    /// A seam VERTEX group's rank lies along its edge parent
    /// ([`seam_vertex_parents`]): the one edge parent, or a run's least
    /// edge ([`RankRule::along_run`]). That edge is oriented by the seam line
    /// [`seam_line_pair`] finds for it, where it lies on one — its own
    /// pair, or the pair of the seam it is a piece of, through every
    /// wrapper that carries an entity — which is the orientation the
    /// pair emitter ranked along (`emit_topo`'s `crossed_edge_order`).
    /// A Split's crossing group ranks along the edge it crosses, oriented
    /// the same way ([`split_crossed_edges`]).
    fn derive<'n, E>(
        was: &'n StableName,
        now: &StableName,
        image: Image<'_, 'n, E>,
    ) -> Result<Self, E> {
        let is_edge = |n: &StableName| n.kind == EntityKind::Edge;
        if let Some((was_edge, now_edge)) = split_crossed_edges(was, now) {
            return Self::along(was_edge, now_edge, image);
        }
        match (was.kind, seam_vertex_parents(was), seam_vertex_parents(now)) {
            (
                EntityKind::Vertex,
                Some(VertexLines::Pair(a, b)),
                Some(VertexLines::Pair(a2, b2)),
            ) => {
                // The edge parent in `now`: the one of kind edge, which a
                // rewrite does not change, wherever ordering put it. A
                // pair holds one edge at most: a `Crossing`'s second
                // side is a face, and a seam vertex's sides are faces.
                let edge_now = if is_edge(a2) { a2 } else { b2 };
                match (is_edge(a), is_edge(b)) {
                    (true, _) => Self::along(a, edge_now, image),
                    (false, true) => Self::along(b, edge_now, image),
                    (false, false) => Ok(Self::Keep),
                }
            }
            (
                EntityKind::Vertex,
                Some(VertexLines::Run(was_run)),
                Some(VertexLines::Run(now_run)),
            ) => Self::along_run(was_run, now_run, image),
            (EntityKind::Vertex, Some(_), Some(_)) => Ok(Self::Reshaped),
            (EntityKind::Vertex | EntityKind::Edge | EntityKind::Face | EntityKind::Body, ..) => {
                Ok(Self::Keep)
            }
        }
    }

    /// The rule for a rank along run `was`'s least edge, the run
    /// rewritten to `now`: [`RankRule::along`] that edge, where `now`'s
    /// least edge is its image. A run with no edge ranks along a seam in
    /// it, so its rule is `Keep` only where the rewrite left every line
    /// in place with its pair as it was.
    fn along_run<'n, E>(
        was: &'n [RunLine],
        now: &[RunLine],
        image: Image<'_, 'n, E>,
    ) -> Result<Self, E> {
        let least = |run: &[RunLine]| run.iter().position(|l| l.sense.is_some());
        match (least(was), least(now)) {
            (Some(w), Some(n)) => {
                if image(&was[w].line)? != *now[n].line {
                    return Ok(Self::RefuseRun);
                }
                Self::along(&was[w].line, &now[n].line, image)
            }
            (None, None) if was.len() == now.len() => {
                for (w, n) in was.iter().zip(now) {
                    if image(&w.line)? != *n.line
                        || Self::along(&w.line, &n.line, image)? != Self::Keep
                    {
                        return Ok(Self::RefuseRun);
                    }
                }
                Ok(Self::Keep)
            }
            _ => Ok(Self::RefuseRun),
        }
    }

    /// The rule for a rank along EDGE `was`'s line, the edge rewritten
    /// to `now`: `Reverse` exactly where `now`'s pair is the images of
    /// `was`'s pair in the other order.
    fn along<'n, E>(
        was: &'n StableName,
        now: &StableName,
        image: Image<'_, 'n, E>,
    ) -> Result<Self, E> {
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
            Self::RefuseRun => Err(Unrankable::RunLeastLineMoved),
            Self::Reshaped => Err(Unrankable::Reshaped),
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
pub(crate) fn canonical<'n, E>(
    was: &'n StableName,
    now: StableName,
    image: Image<'_, 'n, E>,
) -> Result<StableName, Stop<E>> {
    let mut now = order(now);
    if has_rank(&now) {
        let rule = RankRule::derive(was, &now, image).map_err(Stop::Image)?;
        for seg in &mut now.path {
            if let RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }) = seg {
                *seg = RoleSeg::Fragment(rule.apply(*rank, *of).map_err(Stop::Unrankable)?);
            }
        }
    }
    flip_senses(was, &mut now, image).map_err(Stop::Image)?;
    Ok(now)
}

/// **A crossing's senses, re-read along the edges the rewrite left**:
/// each sense flips where the edge it is read along lies on a seam line
/// whose pair the rewrite put in the other order ([`RankRule::along`]),
/// as a rank along that edge reverses. The edges are matched by image:
/// `now`'s edge is the image of `was`'s, wherever ordering put it.
fn flip_senses<'n, E>(
    was: &'n StableName,
    now: &mut StableName,
    image: Image<'_, 'n, E>,
) -> Result<(), E> {
    match (was.path.first(), now.path.first_mut()) {
        (
            Some(RoleSeg::Crossing { edge: was_edge, .. }),
            Some(RoleSeg::Crossing { edge, sense, .. }),
        ) => {
            if RankRule::along(was_edge, edge, image)? == RankRule::Reverse {
                *sense = sense.flipped();
            }
        }
        (
            Some(RoleSeg::CrossingVertex { edge: was_edge, .. }),
            Some(RoleSeg::CrossingVertex { edge, sense, .. }),
        ) => {
            if RankRule::along(was_edge, edge, image)? == RankRule::Reverse {
                *sense = sense.flipped();
            }
        }
        (Some(RoleSeg::EdgeCrossing(was_run)), Some(RoleSeg::EdgeCrossing(run))) => {
            for l in run.iter_mut() {
                let Some(sense) = l.sense.as_mut() else {
                    continue;
                };
                let mut was_line = None;
                for w in was_run {
                    if image(&w.line)? == *l.line {
                        was_line = Some(&w.line);
                        break;
                    }
                }
                if let Some(w) = was_line
                    && RankRule::along(w, &l.line, image)? == RankRule::Reverse
                {
                    *sense = sense.flipped();
                }
            }
        }
        _ => {}
    }
    Ok(())
}

/// **A name an emitter just minted**, in canonical form: its positions
/// in order. Its ranks were
/// measured along the pair it writes, so there is no earlier spelling
/// to re-read them against: [`canonical`] with the name as its own
/// `was` and nothing rewritten, which derives `Keep` for every rank.
pub(crate) fn minted(name: StableName) -> StableName {
    order(name)
}

/// **One freshly minted segment in canonical form**, for an emitter
/// that mints a segment rather than a whole path (`emit_blend`'s band
/// face): [`minted`]'s ordering, one segment at a time.
pub(crate) fn minted_segment(seg: RoleSeg) -> RoleSeg {
    segment(seg)
}

/// **A fold-table name collapsed into a union's space**, in canonical
/// form ([`canonical`]). A rank it cannot re-read is refused: the
/// collapse is an emission door.
pub(crate) fn collapsed<'n, E>(
    was: &'n StableName,
    now: StableName,
    image: Image<'_, 'n, E>,
) -> Result<StableName, Stop<E>> {
    canonical(was, now, image)
}

/// What [`rewritten`] made of a published name.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Rewritten {
    /// The name in canonical form.
    Canonical(StableName),
    /// A name with a rank that cannot be re-read, exactly as the
    /// segment rewrite left it, and why.
    Carried(StableName, Unrankable),
}

/// **A published name after a rewrite**, in canonical form
/// ([`canonical`]).
///
/// A name with a rank that cannot be re-read is answered
/// [`Rewritten::Carried`], nothing reordered and nothing re-ranked: a
/// rewrite re-spells a reference, and whether it may carry one is the
/// rewriter's to say (`SegRewrite::unrankable`). Two of the reasons are
/// emission-time facts: no emitter mints a rank at or past its count,
/// and no rewrite changes the head a seam vertex's lines are read from,
/// so a name carrying either was never published and resolves to
/// nothing before the rewrite and after it. [`RankRule::RefuseRun`] is
/// not: a pair boolean publishes a ranked run, and a rewrite whose map does not
/// keep name order can make another of its lines least, so the name
/// carried resolves to nothing though it resolved before.
///
/// # Errors
///
/// Whatever `image` refuses.
pub(crate) fn rewritten<'n, E>(
    was: &'n StableName,
    now: StableName,
    image: Image<'_, 'n, E>,
) -> Result<Rewritten, E> {
    if !has_rank(&now) && !has_sense(&now) {
        return Ok(Rewritten::Canonical(order(now)));
    }
    let carried = now.clone();
    match canonical(was, now, image) {
        Ok(name) => Ok(Rewritten::Canonical(name)),
        Err(Stop::Image(e)) => Err(e),
        Err(Stop::Unrankable(why)) => Ok(Rewritten::Carried(carried, why)),
    }
}

/// The ordering half: every position in order, the ranks untouched.
fn order(mut name: StableName) -> StableName {
    name.path = core::mem::take(&mut name.path)
        .into_iter()
        .map(segment)
        .collect();
    if is_junction(&name) {
        name.path.sort();
    }
    one_constituent(name)
}

/// **A `Merged` whose set has one element, minted by the node that minted
/// the set, is that element** (N3): a face glued to its twin, the same
/// read spelled twice (REFERENCES DM5), is named as the face, with
/// whatever follows the set kept after it. A set of one name minted
/// upstream stays a set: hoisting that name would publish another node's
/// row in this node's table.
fn one_constituent(name: StableName) -> StableName {
    match name.path.split_first() {
        Some((RoleSeg::Merged(set), rest)) if set.len() == 1 && set[0].node == name.node => {
            let one = &set[0];
            let mut path = one.path.clone();
            path.extend(rest.iter().cloned());
            StableName {
                kind: name.kind,
                node: one.node,
                path,
            }
        }
        _ => name,
    }
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
fn segment(seg: RoleSeg) -> RoleSeg {
    match seg {
        // The name-ordered positions.
        RoleSeg::Seam { a, b } if a > b => RoleSeg::Seam { a: b, b: a },
        RoleSeg::EdgeCrossing(mut run) => {
            run.sort();
            RoleSeg::EdgeCrossing(run)
        }
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
        RoleSeg::Fragment(Qualifier::Borders(walls)) => {
            RoleSeg::Fragment(Qualifier::Borders(sorted_set(walls)))
        }
        RoleSeg::Fragment(Qualifier::Keeps(edges)) => {
            RoleSeg::Fragment(Qualifier::Keeps(sorted_set(edges)))
        }
        RoleSeg::Fragment(Qualifier::Ends(mut ends)) => {
            ends.sort();
            RoleSeg::Fragment(Qualifier::Ends(ends))
        }
        // Everything else carries its names, or none, in an order of
        // its own.
        RoleSeg::Seam { .. }
        | RoleSeg::Crossing { .. }
        | RoleSeg::Fragment(Qualifier::OrderAlong { .. })
        | RoleSeg::From { .. }
        | RoleSeg::SectionEdge { .. }
        | RoleSeg::SplitFragment { .. }
        | RoleSeg::CrossingVertex { .. }
        | RoleSeg::OnToolVertex { .. }
        | RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::EndArc { .. }
        | RoleSeg::Mitre { .. }
        | RoleSeg::TurnFoot { .. }
        | RoleSeg::BandTrim { .. }
        | RoleSeg::BandFoot(_)
        | RoleSeg::BandCut(_)
        | RoleSeg::Inner(_)
        | RoleSeg::Rim(_)
        | RoleSeg::HoleRim { .. }
        | RoleSeg::InPart { .. }
        | RoleSeg::Placed { .. }
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

/// Whether `name` is a crossing whose sense a rewrite may re-read.
fn has_sense(name: &StableName) -> bool {
    matches!(
        name.path.first(),
        Some(RoleSeg::Crossing { .. } | RoleSeg::EdgeCrossing(_) | RoleSeg::CrossingVertex { .. })
    )
}

/// The edge a Split's crossing vertex `was` crosses, and the edge its
/// rewrite `now` crosses: its senses and ranks are read along that
/// edge, oriented by the seam line it lies on where it lies on one
/// (`emit_topo`'s `crossed_edge_orientation`), as a boolean crossing's
/// are.
fn split_crossed_edges<'w, 'n>(
    was: &'w StableName,
    now: &'n StableName,
) -> Option<(&'w StableName, &'n StableName)> {
    match (was.path.first(), now.path.first()) {
        (
            Some(RoleSeg::CrossingVertex { edge: w, .. }),
            Some(RoleSeg::CrossingVertex { edge: n, .. }),
        ) if was.kind == EntityKind::Vertex => Some((w, n)),
        _ => None,
    }
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
    use crate::names::role::{CapEnd, NameRef};
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
            node: RecipeNodeId::new(0, node),
            path: vec![RoleSeg::From {
                read: crate::VarId::new(1, member),
                of: NameRef::new(StableName {
                    kind: EntityKind::Face,
                    node: RecipeNodeId::new(0, member),
                    path: vec![RoleSeg::Cap(CapEnd::Start)],
                }),
            }],
        }
    }

    fn edge(node: u64, member: u64) -> StableName {
        let mut edge = face(node, member);
        edge.kind = EntityKind::Edge;
        edge
    }

    fn name(kind: EntityKind, node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId::new(0, node),
            path,
        }
    }

    fn rank(rank: u32, of: u32) -> RoleSeg {
        RoleSeg::Fragment(Qualifier::OrderAlong { rank, of })
    }

    /// The canonical form a rewrite answers, where it answers one.
    fn formed(r: Result<Rewritten, ()>) -> StableName {
        match r {
            Ok(Rewritten::Canonical(n)) => n,
            other => panic!("a canonical form: {other:?}"),
        }
    }

    /// The identity image: the name written is the name meant.
    fn same(n: &StableName) -> Result<StableName, ()> {
        Ok(n.clone())
    }

    /// `written` in canonical form, read as its own earlier spelling.
    fn form(written: StableName) -> StableName {
        let was = written.clone();
        match canonical(&was, written, &mut same) {
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

    /// `build` over every order of `items`, in canonical form: one form
    /// for all of them, and that form is a fixed point of
    /// [`canonical`], [`minted`] and [`rewritten`].
    fn one_form<T: Clone>(items: &[T], build: impl Fn(Vec<T>) -> StableName) -> StableName {
        let forms: Vec<StableName> = permutations(items)
            .into_iter()
            .map(|p| form(build(p)))
            .collect();
        for f in &forms {
            assert_eq!(
                f, &forms[0],
                "the canonical form depends on the written order"
            );
        }
        let f = forms[0].clone();
        assert_eq!(form(f.clone()), f, "not a fixed point");
        assert_eq!(minted(f.clone()), f);
        assert_eq!(formed(rewritten(&f, f.clone(), &mut same)), f);
        f
    }

    #[test]
    fn a_merged_set_is_one_form_in_every_order() {
        let cs = [face(9, 3), face(9, 1), face(9, 2), face(9, 1)];
        let out = one_form(&cs, |p| name(EntityKind::Face, 9, vec![RoleSeg::Merged(p)]));
        assert_eq!(
            out.path,
            vec![RoleSeg::Merged(vec![face(9, 1), face(9, 2), face(9, 3)])],
            "sorted, each constituent once"
        );
    }

    #[test]
    fn a_band_face_set_is_one_form_in_every_order() {
        let es = [edge(4, 2), edge(4, 0), edge(4, 1), edge(4, 0)];
        let out = one_form(&es, |p| {
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
    fn a_borders_set_is_one_form_in_every_order() {
        let ws = [face(9, 2), face(9, 1), face(9, 3), face(9, 1)];
        let out = one_form(&ws, |w| {
            name(
                EntityKind::Face,
                9,
                vec![
                    RoleSeg::Cap(CapEnd::Start),
                    RoleSeg::Fragment(Qualifier::Borders(w)),
                ],
            )
        });
        assert_eq!(
            out.path.last(),
            Some(&RoleSeg::Fragment(Qualifier::Borders(vec![
                face(9, 1),
                face(9, 2),
                face(9, 3)
            ]))),
            "the walls are sorted and a repeat is one wall"
        );
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
        // Each line's sides AND the run are in name order, so a line
        // written the other way round is the same line.
        let flipped: Vec<(StableName, StableName)> =
            lines.iter().map(|(a, b)| (b.clone(), a.clone())).collect();
        let by_name = one_form(&lines, build);
        assert_eq!(by_name, one_form(&flipped, build));
        assert_eq!(
            by_name.path,
            vec![
                seam(face(9, 1), face(9, 2)),
                seam(face(9, 1), face(9, 3)),
                seam(face(9, 2), face(9, 3)),
            ]
        );
    }

    fn run_line(line: StableName, sense: Option<crate::names::Sense>) -> RunLine {
        RunLine {
            line: NameRef::new(line),
            sense,
        }
    }

    #[test]
    fn an_edge_crossing_run_is_one_form_in_every_order_each_sense_with_its_line() {
        use crate::names::Sense;
        let lines = [
            run_line(edge(9, 3), Some(Sense::Leaves)),
            run_line(edge(9, 1), Some(Sense::Outside)),
            run_line(
                name(EntityKind::Edge, 9, vec![seam(face(9, 1), face(9, 2))]),
                None,
            ),
        ];
        let out = one_form(&lines, |p| {
            name(EntityKind::Vertex, 9, vec![RoleSeg::EdgeCrossing(p)])
        });
        let [RoleSeg::EdgeCrossing(run)] = out.path.as_slice() else {
            panic!("one run: {out:?}")
        };
        assert!(run.windows(2).all(|w| w[0].line < w[1].line), "{run:?}");
        for l in &lines {
            assert!(run.contains(l), "{l:?} kept its sense: {run:?}");
        }
    }

    #[test]
    fn a_runs_rank_follows_its_least_edge_and_refuses_where_another_line_becomes_least() {
        use crate::names::Sense;
        let (x, y) = (face(9, 1), face(9, 2));
        // The least edge is a union seam edge, so a rewrite that puts its
        // pair in the other order reverses the rank along it.
        let on = |a: &StableName, b: &StableName| {
            name(EntityKind::Edge, 9, vec![seam(a.clone(), b.clone())])
        };
        let ranked = |least: StableName, other: StableName| {
            name(
                EntityKind::Vertex,
                9,
                vec![
                    RoleSeg::EdgeCrossing(vec![
                        run_line(least, Some(Sense::Enters)),
                        run_line(other, Some(Sense::Outside)),
                    ]),
                    rank(0, 3),
                ],
            )
        };
        // A joined edge: a set orders after a seam, so the seam is least.
        let other = name(
            EntityKind::Edge,
            9,
            vec![RoleSeg::Merged(vec![edge(9, 7), edge(9, 8)])],
        );
        let was = ranked(on(&y, &x), other.clone());
        let out = collapsed(&was, ranked(on(&x, &y), other.clone()), &mut |n| {
            Ok::<_, ()>(if *n == on(&y, &x) {
                on(&x, &y)
            } else {
                n.clone()
            })
        });
        let Ok(out) = out else { panic!("re-read") };
        assert_eq!(
            out.path.last(),
            Some(&rank(2, 3)),
            "reversed along the least edge"
        );
        let RoleSeg::EdgeCrossing(run) = &out.path[0] else {
            panic!("a run: {out:?}")
        };
        assert_eq!(
            run[0].sense,
            Some(Sense::Leaves),
            "its sense flips with its rank"
        );
        // A rewrite under which another line is least has no line to
        // re-read the rank along: a member edge orders before a seam.
        let lesser = edge(9, 0);
        let moved = collapsed(&was, ranked(lesser.clone(), on(&x, &y)), &mut |n| {
            Ok::<_, ()>(if *n == other {
                lesser.clone()
            } else if *n == on(&y, &x) {
                on(&x, &y)
            } else {
                n.clone()
            })
        });
        assert!(
            matches!(moved, Err(Stop::Unrankable(Unrankable::RunLeastLineMoved))),
            "the least edge changed"
        );
    }

    fn vertex(node: u64, member: u64) -> StableName {
        let mut vertex = face(node, member);
        vertex.kind = EntityKind::Vertex;
        vertex
    }

    fn ends(v0: StableName, v1: StableName) -> RoleSeg {
        RoleSeg::Fragment(Qualifier::Ends(vec![v0, v1]))
    }

    #[test]
    fn a_seam_edge_piece_is_one_form_from_either_side_and_whatever_order_its_ends_are_in() {
        let (x, y) = (face(9, 1), face(9, 2));
        let (v, w) = (vertex(9, 3), vertex(9, 4));
        let want = vec![seam(x.clone(), y.clone()), ends(v.clone(), w.clone())];
        for (a, b) in [(&x, &y), (&y, &x)] {
            for (v0, v1) in [(&v, &w), (&w, &v)] {
                let written = name(
                    EntityKind::Edge,
                    9,
                    vec![seam(a.clone(), b.clone()), ends(v0.clone(), v1.clone())],
                );
                let out = form(written);
                assert_eq!(out.path, want, "the pair and the ends in name order");
                assert_eq!(form(out.clone()), out, "not a fixed point");
            }
        }
        // A repeated end is kept: a piece has two ends.
        let closed = name(
            EntityKind::Edge,
            9,
            vec![seam(x.clone(), y.clone()), ends(v.clone(), v.clone())],
        );
        assert_eq!(form(closed.clone()), closed);
    }

    #[test]
    fn a_rank_along_a_seam_an_embedded_name_writes_follows_that_seams_order() {
        // A pair boolean (node 12) crossed a union's (node 9) seam edge
        // twice: the crossings are ranked along the union seam, oriented
        // by the pair written INSIDE the embedded name. A rewrite that
        // reorders that seam's sides reverses the rank, though the outer
        // name's own pair keeps its order.
        let (x, y) = (face(9, 1), face(9, 2));
        let cutter = face(11, 1);
        let union_seam = |a: &StableName, b: &StableName| {
            name(EntityKind::Edge, 9, vec![seam(a.clone(), b.clone())])
        };
        let crossing = |edge: StableName, r| {
            minted(name(
                EntityKind::Vertex,
                12,
                vec![seam(edge, cutter.clone()), rank(r, 2)],
            ))
        };
        let was = crossing(union_seam(&x, &y), 0);
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
        let now = crossing(union_seam(&y2, &x2), 0);
        let out = formed(rewritten(&was, now, &mut image));
        assert_eq!(out, crossing(union_seam(&y2, &x2), 1));
        // An edge piece carries no rank, so the same rewrite of a piece
        // of that seam only re-spells it.
        let piece = |inner: StableName| {
            name(
                EntityKind::Edge,
                12,
                vec![
                    RoleSeg::From {
                        read: crate::names::FOLD_A,
                        of: NameRef::new(inner),
                    },
                    ends(vertex(12, 1), vertex(12, 2)),
                ],
            )
        };
        let was = piece(union_seam(&x, &y));
        let now = piece(union_seam(&y2, &x2));
        assert_eq!(formed(rewritten(&was, now.clone(), &mut image)), now);
    }

    #[test]
    fn a_crossings_sense_along_a_seam_an_embedded_name_writes_flips_with_its_rank() {
        // A pair boolean (node 12) crossed a union's (node 9) seam edge:
        // the crossing's sense, like its rank, is read along the union
        // seam as the pair written inside the embedded name orients it,
        // so a rewrite that reorders that pair flips the sense and
        // reverses the rank. One that keeps the pair's order keeps both.
        use crate::names::role::Sense;

        let (x, y) = (face(9, 1), face(9, 2));
        let cutter = face(11, 1);
        let union_seam = |a: &StableName, b: &StableName| {
            name(EntityKind::Edge, 9, vec![seam(a.clone(), b.clone())])
        };
        let crossing = |edge: StableName, sense, r| {
            name(
                EntityKind::Vertex,
                12,
                vec![
                    RoleSeg::Crossing {
                        edge: NameRef::new(edge),
                        face: NameRef::new(cutter.clone()),
                        sense,
                    },
                    rank(r, 2),
                ],
            )
        };
        let was = crossing(union_seam(&x, &y), Sense::Enters, 0);
        let swap = |(x2, y2): (StableName, StableName)| {
            let (x, y) = (x.clone(), y.clone());
            move |n: &StableName| -> Result<StableName, ()> {
                Ok(if *n == x {
                    x2.clone()
                } else if *n == y {
                    y2.clone()
                } else {
                    n.clone()
                })
            }
        };
        // x and y come out with y' < x': the seam's pair reorders.
        let (x2, y2) = (face(9, 5), face(9, 4));
        let now = crossing(union_seam(&y2, &x2), Sense::Enters, 0);
        let out = formed(rewritten(&was, now, &mut swap((x2.clone(), y2.clone()))));
        assert_eq!(out, crossing(union_seam(&y2, &x2), Sense::Leaves, 1));
        // x and y come out with x' < y': the pair keeps its order.
        let (x3, y3) = (face(9, 4), face(9, 5));
        let now = crossing(union_seam(&x3, &y3), Sense::Enters, 0);
        let out = formed(rewritten(&was, now.clone(), &mut swap((x3, y3))));
        assert_eq!(out, now);
    }

    #[test]
    fn a_split_crossings_sense_along_a_seam_an_embedded_name_writes_flips_with_its_rank() {
        // A Split (node 12) crossed a union's (node 9) seam edge: its
        // crossing's sense and rank are read along the union seam as the
        // pair written inside the crossed edge's name orients it, as a
        // boolean's are, so a rewrite that reorders that pair flips the
        // sense and reverses the rank, and one that keeps it keeps both.
        use crate::names::role::Sense;

        let (x, y) = (face(9, 1), face(9, 2));
        let union_seam = |a: &StableName, b: &StableName| {
            name(EntityKind::Edge, 9, vec![seam(a.clone(), b.clone())])
        };
        let crossing = |edge: StableName, sense, r| {
            name(
                EntityKind::Vertex,
                12,
                vec![
                    RoleSeg::CrossingVertex {
                        edge: NameRef::new(edge),
                        sense,
                    },
                    rank(r, 2),
                ],
            )
        };
        let was = crossing(union_seam(&x, &y), Sense::Enters, 0);
        let swap = |(x2, y2): (StableName, StableName)| {
            let (x, y) = (x.clone(), y.clone());
            move |n: &StableName| -> Result<StableName, ()> {
                Ok(if *n == x {
                    x2.clone()
                } else if *n == y {
                    y2.clone()
                } else {
                    n.clone()
                })
            }
        };
        let (x2, y2) = (face(9, 5), face(9, 4));
        let now = crossing(union_seam(&y2, &x2), Sense::Enters, 0);
        let out = formed(rewritten(&was, now, &mut swap((x2.clone(), y2.clone()))));
        assert_eq!(out, crossing(union_seam(&y2, &x2), Sense::Leaves, 1));
        let (x3, y3) = (face(9, 4), face(9, 5));
        let now = crossing(union_seam(&x3, &y3), Sense::Enters, 0);
        let out = formed(rewritten(&was, now.clone(), &mut swap((x3, y3))));
        assert_eq!(out, now);
        // A lone crossing, with no rank to re-read, still flips.
        let lone = |edge: StableName, sense| {
            let mut n = crossing(edge, sense, 0);
            n.path.truncate(1);
            n
        };
        let was = lone(union_seam(&x, &y), Sense::Enters);
        let now = lone(union_seam(&y2, &x2), Sense::Enters);
        let out = formed(rewritten(&was, now, &mut swap((x2.clone(), y2.clone()))));
        assert_eq!(out, lone(union_seam(&y2, &x2), Sense::Leaves));
    }

    #[test]
    fn a_rank_that_cannot_be_reread_refuses_at_a_door_and_is_carried_untouched_by_a_rewrite() {
        let (x, y) = (face(9, 1), face(9, 2));
        let cutter = face(9, 3);
        let crossing = |a: &StableName, b: &StableName| {
            name(
                EntityKind::Vertex,
                9,
                vec![
                    seam(
                        name(EntityKind::Edge, 9, vec![seam(a.clone(), b.clone())]),
                        cutter.clone(),
                    ),
                    rank(3, 3),
                ],
            )
        };
        let (was, now) = (crossing(&y, &x), crossing(&x, &y));
        assert!(matches!(
            collapsed(&was, now.clone(), &mut same),
            Err(Stop::Unrankable(Unrankable::RankOutsideCount))
        ));
        assert_eq!(
            rewritten(&was, now.clone(), &mut same).unwrap(),
            Rewritten::Carried(now, Unrankable::RankOutsideCount),
            "nothing reordered, nothing re-ranked"
        );
    }

    #[test]
    fn a_seams_sides_are_in_name_order_whichever_node_minted_them() {
        // A pair seam beside a face the pair merged cites that face's
        // `Merged` parent and the other face's name in the pair's own
        // table (N3), both minted by the pair; a union's sides are its
        // own too. Either way the two are put in name order.
        let parent = name(
            EntityKind::Face,
            9,
            vec![RoleSeg::Merged(vec![face(9, 1), face(9, 3)])],
        );
        let beside = face(9, 2);
        let want = name(
            EntityKind::Edge,
            9,
            vec![seam(beside.clone(), parent.clone())],
        );
        assert!(
            parent > beside,
            "the premise: the merged parent names after"
        );
        for written in [
            name(
                EntityKind::Edge,
                9,
                vec![seam(parent.clone(), beside.clone())],
            ),
            want.clone(),
        ] {
            assert_eq!(minted(written.clone()), want, "{written:?}");
            assert_eq!(
                rewritten(&written, written.clone(), &mut same).unwrap(),
                Rewritten::Canonical(want.clone()),
                "{written:?}"
            );
        }
    }
}
