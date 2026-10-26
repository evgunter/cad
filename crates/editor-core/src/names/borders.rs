//! **N2's `Borders`: a split face's pieces are told apart by the divider
//! walls they border**, read off the kernel's record of what each
//! boolean discarded (`topo::BooleanNaming::discards`).
//!
//! A face the result holds as several pieces lost the rest of its
//! region to the boolean. An OBSTACLE is a connected part of that lost
//! region, connected through its closure: one discarded fragment, or
//! several joined where
//!
//! - they border one edge of the result;
//! - both sides of an edge that died with them were discarded region of
//!   the parent: two fragments of one face across a chord that cut it,
//!   or two of a corner's faces across an edge between them
//!   (`Discard::bounds`);
//! - a later step discarded the region across an earlier step's seam,
//!   its boundary descending from that seam (`Discard::touches`);
//! - they meet at a point the result holds as a vertex: one ends a
//!   bordered stretch there, and an edge at it is, or descends
//!   from, an edge the other borders end to end, of its own step or an
//!   earlier one (`Discard::meets`);
//! - they meet at a point the output stage joined away: a piece edge
//!   that descends from the joined edge reaches the discards along every
//!   edge the join made it of (`Obstacles::joined_from`).
//!
//! Each piece edge is chased to the stretch a discard recorded along it,
//! through the edge's split lineage (kept across the steps that killed
//! its ancestors) and the edges a later step's join made it of, and so
//! to its obstacle; discards one edge reaches are one obstacle. Where a
//! coincident kept face holds part of a dropped face's region, the
//! boolean cut the dropped face where the held part ends, so its
//! removed fragment borders the kept face there, and the held fragment
//! is no discard (`topo::BooleanNaming::substituted`). An obstacle that
//! borders two or more
//! pieces DIVIDES the face; a piece's qualifier is the set of walls —
//! the faces across its edges — along which it meets a divider, each
//! cited by its parent's name. Nothing beyond a piece's own boundary is
//! read, so a boss standing on one piece, or a notch in it, is an
//! obstacle that borders one piece and is never cited; one whose corner
//! touches a divider is part of that divider.
//!
//! An operand corner the result holds as several vertices is read the
//! same way ([`Obstacles::split_corner`]): its parent is the corner's
//! faces, and each copy's boundary is those faces' pieces' edges at it.
//!
//! One record serves the pair boolean, which records one step, and the
//! union, which records each fold step and asks after the finished body
//! ([`Obstacles::split`]). Both readings are one rule
//! ([`Obstacles::walls`]), and both refuse a piece or copy, among
//! several, that meets no divider.

use core::convert::Infallible;
use core::ops::ControlFlow;
use std::collections::{BTreeMap, BTreeSet};

use topo::{Body, Cell, EdgeKey, FaceKey, Provenance, VertexKey};

use super::chase;
use super::emit::{NamingError, edge_ends, face_half_edges};
use super::least_root::LeastRoot;

/// One discarded face, resolved into the result of its step.
struct Discard {
    /// The result edges it bordered a kept face along, some only in
    /// part: an edge the output stage joined holds the stretch and the
    /// edge it was joined to.
    seams: Vec<EdgeKey>,
    /// The seams it borders end to end, each still the edge it bordered:
    /// none a later step's join made longer or killed. Every point of
    /// one is in its closure.
    whole: Vec<EdgeKey>,
    /// Earlier steps' seam edges its boundary descends from, the region
    /// across which was discarded before: it is one obstacle with their
    /// discards.
    touches: Vec<EdgeKey>,
    /// Edges others border end to end, of its own step or an earlier
    /// one, that an edge at a live vertex where one of its stretches
    /// ends is, or descends from: it is one obstacle with the discards
    /// whole along them, which hold the point in their closure.
    meets: Vec<EdgeKey>,
    /// Its other boundary edges, which it died with: another discard of
    /// the parent across one of them — a fragment of the same face
    /// across a chord, or of another of a corner's faces across the edge
    /// between them — is region of the parent joined to this one there.
    /// Each is the edge in its operand's clone keys, with the boolean
    /// that recorded it and the operand: B's died before the graft, so
    /// the result has no key for them, and only two discards of one
    /// operand in one boolean can share one.
    bounds: Vec<(usize, topo::Operand, EdgeKey)>,
}

/// One discard row of a step, read before any row's meets.
struct Read<'a> {
    row: &'a topo::DiscardRow,
    chains: Vec<Vec<EdgeKey>>,
    seams: Vec<EdgeKey>,
    whole: Vec<EdgeKey>,
    /// The live vertices its stretches end at.
    ends: BTreeSet<VertexKey>,
}

/// The discards of one boolean, or of each step of a union's fold.
pub(crate) struct Obstacles<K> {
    discards: Vec<Discard>,
    /// Parent key → the discards descending from it (what the caller's
    /// `parents_of` named each a fragment of).
    by_parent: BTreeMap<K, Vec<usize>>,
    /// Edge → the edge it was split from, as each recorded result's
    /// provenance, each step's joined lineage and each discard's chains
    /// state it, so an edge a step killed — with a discarded face, or by
    /// joining it into another — still leads to its parent.
    split_from: BTreeMap<EdgeKey, EdgeKey>,
    /// Edge → the edges an output stage's join made it out of, besides
    /// itself. Discards along two of them meet at the vertex the join
    /// deleted, which no stretch end names live, so a piece edge's chase
    /// through them is what makes them one obstacle.
    joined_from: BTreeMap<EdgeKey, Vec<EdgeKey>>,
    /// The booleans recorded so far.
    records: usize,
}

/// Records that `edge` was split from `from`. Every source copies the
/// one record a body held for `edge`, so a second write agrees.
fn note_split(split_from: &mut BTreeMap<EdgeKey, EdgeKey>, edge: EdgeKey, from: EdgeKey) {
    let held = *split_from.entry(edge).or_insert(from);
    debug_assert_eq!(held, from, "{edge:?} recorded with two parents");
}

/// The emission error this module refuses with.
fn bug(what: &'static str) -> NamingError {
    NamingError::Emission { what }
}

impl<K: Ord + Clone> Obstacles<K> {
    pub(crate) fn new() -> Self {
        Self {
            discards: Vec::new(),
            by_parent: BTreeMap::new(),
            split_from: BTreeMap::new(),
            joined_from: BTreeMap::new(),
            records: 0,
        }
    }

    /// Records one boolean's discards against its result `body`.
    /// `parents_of(operand, face)` names what a discarded face — in its
    /// operand's clone keys — is a fragment of.
    pub(crate) fn record<T: geom_core::Real>(
        &mut self,
        naming: &topo::BooleanNaming,
        body: &Body<T>,
        mut parents_of: impl FnMut(topo::Operand, FaceKey) -> Result<BTreeSet<K>, NamingError>,
    ) -> Result<(), NamingError> {
        // What was recorded before is operand A's, in A's keys, and every
        // record reads it as the result's: A held directly. A result
        // that is B alone holds none of A's region, so none of it
        // borders anything after, and its keys name another arena.
        match (naming.a_keys, naming.b_keys) {
            (topo::OperandKeys::Direct, _) => {}
            (topo::OperandKeys::Absent, topo::OperandKeys::Direct) => {
                *self = Self {
                    records: self.records,
                    ..Self::new()
                };
            }
            layout => unreachable!(
                "a boolean recorded operand key layout {layout:?}: every boolean result is A's \
                 clone with B grafted or absent, or B's clone with A absent (`BooleanNaming`'s \
                 constructors)"
            ),
        }
        let settler = naming.settler(body);
        // A bordered stretch settles on a seam edge: the zip made the
        // kept face's section edge one edge with the wall's. A stretch
        // read through a substituted fragment settles on the kept face's
        // own edge, whichever it is.
        let seam_edges: BTreeSet<EdgeKey> = naming
            .seam_edges
            .iter()
            .map(|&e| naming.joined_edge(e))
            .collect();
        let mut by_ends: BTreeMap<(VertexKey, VertexKey), Vec<EdgeKey>> = BTreeMap::new();
        let mut any_by_ends: BTreeMap<(VertexKey, VertexKey), Vec<EdgeKey>> = BTreeMap::new();
        for (k, _) in body.edges() {
            let (s, t) = edge_ends(body, k);
            if seam_edges.contains(&k) {
                by_ends.entry((s.min(t), s.max(t))).or_default().push(k);
            }
            any_by_ends.entry((s.min(t), s.max(t))).or_default().push(k);
        }
        // An earlier step's seam this step's output stage joined into
        // another edge lies along the joined edge, whichever of the two
        // the join kept, and neither is one it borders end to end.
        let joined: BTreeSet<EdgeKey> = naming
            .edge_joins
            .iter()
            .flat_map(|j| [j.gone, j.kept])
            .collect();
        for d in &mut self.discards {
            let extended: Vec<EdgeKey> = d
                .seams
                .iter()
                .map(|&e| naming.joined_edge(e))
                .filter(|j| !d.seams.contains(j))
                .collect();
            d.seams.extend(extended);
            d.whole.retain(|e| !joined.contains(e));
        }
        for j in &naming.edge_joins {
            self.joined_from.entry(j.kept).or_default().push(j.gone);
        }
        for (k, _) in body.edges() {
            if let Some(Provenance::SplitEdge { edge }) = body.edge_provenance_of(k) {
                note_split(&mut self.split_from, k, *edge);
            }
        }
        // An edge this step split and its output stage (merge or join)
        // killed lost its record with the kill; the boolean copied it
        // out first.
        for (&g, &f) in &naming.killed_lineage {
            note_split(&mut self.split_from, g, f);
        }
        let earlier: BTreeSet<EdgeKey> = self
            .discards
            .iter()
            .flat_map(|d| d.seams.iter().copied())
            .collect();
        // A row's chains are in its operand's clone keys: a direct
        // operand's are the result's, a grafted one's are read through
        // the graft's edge rows, where an ancestor the graft does not
        // carry ends the chain, and an absent one's name no edge of the
        // result.
        let grafted: BTreeMap<EdgeKey, EdgeKey> = naming
            .graft_edges
            .iter()
            .chain(&naming.graft_dead_edges)
            .copied()
            .collect();
        // Each row read before any row's meets, which reach this step's
        // whole seams too.
        let mut read = Vec::with_capacity(naming.discards.len());
        for row in &naming.discards {
            let chains: Vec<Vec<EdgeKey>> = row
                .boundary_chains
                .iter()
                .map(|chain| {
                    let keys = match row.operand {
                        topo::Operand::A => naming.a_keys,
                        topo::Operand::B => naming.b_keys,
                    };
                    match keys {
                        topo::OperandKeys::Direct => chain.clone(),
                        topo::OperandKeys::Grafted => chain
                            .iter()
                            .map_while(|k| grafted.get(k).copied())
                            .collect(),
                        topo::OperandKeys::Absent => Vec::new(),
                    }
                })
                .collect();
            let mut seams = Vec::new();
            let mut whole = Vec::new();
            let mut ends = BTreeSet::new();
            // A stretch no live edge joins merged away with the faces
            // beside it: nothing of a piece lies along it.
            // A stretch an end of which the output stage joined away lies
            // along the edge the join made.
            for (stretches, edges, seam_only) in [
                (&row.bordered, &by_ends, true),
                (&row.bordered_through, &any_by_ends, false),
            ] {
                // An end the zip split per cone lies at each of its
                // copies, so the stretch is read between every pair of
                // copies, and every edge found is kept, not the first:
                // each lies along the stretch's locus, where the discard's
                // region meets it, so each is where its obstacle borders
                // a piece (N2). `topo`'s `boolean_discards::bordered_edges`
                // reads it alike.
                for &(u, w) in stretches {
                    let (us, ws) = (settler.settle_cones(u)?, settler.settle_cones(w)?);
                    ends.extend(us.iter().chain(&ws).filter_map(|c| match c {
                        Cell::Vertex(v) => Some(*v),
                        Cell::Edge(_) | Cell::Face(_) => None,
                    }));
                    for (&u, &w) in us.iter().flat_map(|u| ws.iter().map(move |w| (u, w))) {
                        let between = match (u, w) {
                            (Cell::Vertex(u), Cell::Vertex(w)) => edges.get(&(u.min(w), u.max(w))),
                            // An end the join deleted lies inside the
                            // joined edge, and one the merge pruned inside
                            // the merged face: neither ends an edge, so the
                            // stretch is read through the joins or lies
                            // along none.
                            (Cell::Edge(_) | Cell::Face(_), _)
                            | (_, Cell::Edge(_) | Cell::Face(_)) => None,
                        };
                        if let Some(es) = between {
                            whole.extend(es.iter().copied());
                        } else if let Some(e) = naming.stretch_through_joins(body, (u, w))
                            && (!seam_only || seam_edges.contains(&e))
                        {
                            seams.push(e);
                        }
                    }
                }
            }
            for chain in &chains {
                for w in chain.windows(2) {
                    note_split(&mut self.split_from, w[0], w[1]);
                }
            }
            seams.extend(whole.iter().copied());
            read.push(Read {
                row,
                chains,
                seams,
                whole,
                ends,
            });
        }
        let whole_so_far: BTreeSet<EdgeKey> = self
            .discards
            .iter()
            .flat_map(|d| &d.whole)
            .chain(read.iter().flat_map(|r| &r.whole))
            .copied()
            .collect();
        for r in read {
            // A chain edge an earlier seam was split into since that
            // seam's step still touches it: its lineage reaches the seam.
            let touches = self.lineage_hits(body, r.chains.concat(), &earlier);
            // An edge at a stretch's end, a piece of an edge another
            // discard borders end to end, has the point in that edge's
            // closure.
            let mut at_ends = Vec::new();
            for v in r.ends {
                at_ends.extend(body.edges_of_vertex(v).unwrap_or_else(|| {
                    unreachable!(
                        "stretch end {v:?}'s star does not resolve in the result: a stretch end \
                         is read off the result's own live edges"
                    )
                }));
            }
            let meets = self.lineage_hits(body, at_ends, &whole_so_far);
            let row = r.row;
            let parents = parents_of(row.operand, row.face)?;
            for k in &parents {
                self.by_parent
                    .entry(k.clone())
                    .or_default()
                    .push(self.discards.len());
            }
            let bounds = row
                .boundary_chains
                .iter()
                .filter_map(|c| c.first().copied())
                .map(|k| (self.records, row.operand, k))
                .collect();
            self.discards.push(Discard {
                seams: r.seams,
                whole: r.whole,
                touches,
                meets,
                bounds,
            });
        }
        self.records += 1;
        Ok(())
    }

    /// The edges of `targets` that some edge of `edges` is, or descends
    /// from ([`Obstacles::up_lineage`]), each walk ending at its first.
    fn lineage_hits<T: geom_core::Real>(
        &self,
        body: &Body<T>,
        edges: Vec<EdgeKey>,
        targets: &BTreeSet<EdgeKey>,
    ) -> Vec<EdgeKey> {
        let mut hits = BTreeSet::new();
        for k in edges {
            self.up_lineage(body, k, |up| {
                let hit = targets.contains(&up);
                if hit {
                    hits.insert(up);
                }
                hit
            });
        }
        hits.into_iter().collect()
    }

    /// Walks edge `e`'s split lineage upward, `e` first: the edge `body`'s
    /// provenance says it was split from, else the one the records say
    /// (`split_from`). `stop` sees each edge and ends the walk by
    /// answering true; it also ends where the lineage does, or at a
    /// provenance other than a split.
    ///
    /// # Panics
    ///
    /// Where the lineage cycles ([`chase::to_root`]), even where `stop`
    /// would end the walk at a key it has seen before: `stop` sees each
    /// edge once. `split_from` holds copies of the recorded booleans'
    /// `SplitEdge` records, and a copy cycles only where the records do
    /// (`topo::Provenance::SplitEdge`).
    fn up_lineage<T: geom_core::Real>(
        &self,
        body: &Body<T>,
        e: EdgeKey,
        mut stop: impl FnMut(EdgeKey) -> bool,
    ) {
        let bound = body.edges().count() + self.split_from.len();
        let mut chain = BTreeSet::new();
        let Ok(_) = chase::to_root::<_, Infallible>(
            e,
            bound,
            "see `borders::Obstacles::up_lineage`'s Panics",
            |e| {
                if chain.insert(e) && stop(e) {
                    return Ok(ControlFlow::Break(e));
                }
                Ok(match body.edge_provenance_of(e) {
                    Some(Provenance::SplitEdge { edge }) => ControlFlow::Continue(*edge),
                    Some(_) => ControlFlow::Break(e),
                    None => match self.split_from.get(&e) {
                        Some(&up) => ControlFlow::Continue(up),
                        None => ControlFlow::Break(e),
                    },
                })
            },
        );
    }

    /// The rule on a face: `pieces`, the faces of `body` that one face
    /// `parent` is held as, grouped by the divider walls each borders
    /// ([`Obstacles::walls`]), each wall named by `wall`. Several pieces under one set are N2's
    /// tie. The sets come out sorted, in `W`'s order.
    ///
    /// `merged` are the parent's other faces, the ones the pair boolean
    /// names apart from `pieces`: for an operand face, the merges that
    /// list it (named `Merged`, N3); for a merged parent, its operand
    /// faces' unmerged pieces and the merges that list them beside other
    /// faces. They hold part of the parent's region, so an obstacle they
    /// border counts toward dividing it, and they are neither grouped
    /// nor checked. Every face of a union's parent is a piece, and it passes
    /// none.
    ///
    /// What only a face has is `merged`: a corner's copies are all
    /// grouped. Refuses as [`Obstacles::walls`] does, the face's merged
    /// faces counted among its several.
    pub(crate) fn split<T: geom_core::Real, W: Ord + Clone>(
        &self,
        body: &Body<T>,
        parent: &BTreeSet<K>,
        pieces: &[FaceKey],
        merged: &[FaceKey],
        wall: impl FnMut(FaceKey) -> Result<W, NamingError>,
    ) -> Result<BTreeMap<Vec<W>, Vec<FaceKey>>, NamingError> {
        let boundaries = pieces
            .iter()
            .chain(merged)
            .map(|&f| face_half_edges(body, f))
            .collect::<Vec<_>>();
        let walls = self.walls(
            body,
            parent,
            &boundaries,
            pieces.len(),
            "a piece of a face held as several borders no recorded discard between them",
            wall,
        )?;
        Ok(group(walls, pieces))
    }

    /// The rule at a corner: `copies`, the vertices of `body` that one
    /// operand corner is held as, grouped by the divider walls each
    /// meets. `parent` is the corner's faces and `pieces` the faces of
    /// `body` that hold part of them; a copy's boundary is those
    /// faces' half-edges at the copy, read as a face piece's boundary is
    /// ([`Obstacles::split`]), so a wall is a face across an edge at the
    /// copy along which a piece of the corner meets an obstacle that
    /// borders two or more copies. Several copies under one set are
    /// N2's tie. Refuses as [`Obstacles::walls`] does, and where a copy
    /// has no edge at all.
    pub(crate) fn split_corner<T: geom_core::Real, W: Ord + Clone>(
        &self,
        body: &Body<T>,
        parent: &BTreeSet<K>,
        copies: &[VertexKey],
        pieces: &BTreeSet<FaceKey>,
        wall: impl FnMut(FaceKey) -> Result<W, NamingError>,
    ) -> Result<BTreeMap<Vec<W>, Vec<VertexKey>>, NamingError> {
        let mut boundaries = Vec::with_capacity(copies.len());
        for &v in copies {
            let mut at = Vec::new();
            let star = body.edges_of_vertex(v).unwrap_or_else(|| {
                unreachable!(
                    "corner copy {v:?}'s star does not resolve: its caller read the copy off \
                     this body's live vertices"
                )
            });
            if star.is_empty() {
                unreachable!(
                    "corner copy {v:?} has no edge: a copy is a vertex of a boolean's result, \
                     a solid's boundary, where every vertex bounds an edge"
                );
            }
            for e in star {
                let edge = body.get_edge(e).unwrap_or_else(|| {
                    unreachable!(
                        "corner copy {v:?}'s edge {e:?} is dangling: a vertex's star lists \
                         live edges of a body every door keeps tier-1-valid"
                    )
                });
                for he in [edge.he_plus, edge.he_minus] {
                    let f = body.face_of_half_edge(he).unwrap_or_else(|| {
                        unreachable!(
                            "corner copy {v:?}'s half-edge {he:?} has no face: every half-edge \
                             of a tier-1-valid body lies in a loop of a face"
                        )
                    });
                    if pieces.contains(&f) {
                        at.push(he);
                    }
                }
            }
            boundaries.push(at);
        }
        let walls = self.walls(
            body,
            parent,
            &boundaries,
            copies.len(),
            "a copy of a corner held as several meets no recorded discard between them",
            wall,
        )?;
        Ok(group(walls, copies))
    }

    /// `mine` grouped by each key `keys` lists for a discard, each
    /// group joined in `link`, and each discard joined to the group of
    /// every key `reaches` lists for it.
    fn join_by<Q: Ord + Copy>(
        &self,
        link: &mut LeastRoot<usize>,
        mine: &BTreeSet<usize>,
        keys: impl Fn(&Discard) -> &[Q],
        reaches: impl Fn(&Discard) -> &[Q],
    ) -> BTreeMap<Q, Vec<usize>> {
        let mut of: BTreeMap<Q, Vec<usize>> = BTreeMap::new();
        for &i in mine {
            for &k in keys(&self.discards[i]) {
                of.entry(k).or_default().push(i);
            }
        }
        for ds in of.values() {
            for w in ds.windows(2) {
                link.join(w[0], w[1]);
            }
        }
        for &i in mine {
            for k in reaches(&self.discards[i]) {
                for &j in of.get(k).into_iter().flatten() {
                    link.join(i, j);
                }
            }
        }
        of
    }

    /// **The one rule**: each of the first `counted` `boundaries`'
    /// divider walls, the faces across its half-edges along which it
    /// meets an obstacle of `parent` that two or more boundaries meet.
    ///
    /// Refuses with `refusal` when, of two or more boundaries, a counted
    /// one meets no divider: the parent is connected, so what lies
    /// between the things it is held as is discarded region, and one
    /// that borders none of it is a region the kernel discarded without
    /// recording it.
    fn walls<T: geom_core::Real, W: Ord + Clone>(
        &self,
        body: &Body<T>,
        parent: &BTreeSet<K>,
        boundaries: &[Vec<topo::HalfEdgeKey>],
        counted: usize,
        refusal: &'static str,
        mut wall: impl FnMut(FaceKey) -> Result<W, NamingError>,
    ) -> Result<Vec<BTreeSet<W>>, NamingError> {
        let mine: BTreeSet<usize> = parent
            .iter()
            .filter_map(|k| self.by_parent.get(k))
            .flatten()
            .copied()
            .collect();
        // This parent's discards, joined into obstacles: two that border
        // one edge, and two that both bounded an edge which died with
        // them, are one connected part of the parent's lost region (N2's
        // obstacle).
        let mut link = LeastRoot::new();
        for &i in &mine {
            link.insert(i);
        }
        let seam_of = self.join_by(&mut link, &mine, |d| &d.seams, |d| &d.touches);
        self.join_by(&mut link, &mine, |d| &d.whole, |d| &d.meets);
        self.join_by(&mut link, &mine, |d| &d.bounds, |_| &[]);
        // Each boundary edge's discards, and the wall across it. An edge
        // reaches a discard along its split lineage, the first recorded
        // seam on it, and through every edge a join made it of, so an
        // edge the join made of two seams meets the discards of both,
        // and they are one obstacle: they meet where the join deleted a
        // vertex.
        let reached = |e: EdgeKey| -> BTreeSet<usize> {
            let mut out = BTreeSet::new();
            let mut seen = BTreeSet::new();
            let mut todo = vec![e];
            while let Some(e) = todo.pop() {
                let mut joined: Vec<EdgeKey> = Vec::new();
                self.up_lineage(body, e, |up| {
                    if !seen.insert(up) {
                        return true;
                    }
                    joined.extend(self.joined_from.get(&up).into_iter().flatten());
                    match seam_of.get(&up) {
                        Some(ds) => {
                            out.extend(ds.iter().copied());
                            true
                        }
                        None => false,
                    }
                });
                todo.extend(joined);
            }
            out
        };
        let mut sides: Vec<(BTreeSet<usize>, usize, FaceKey)> = Vec::new();
        for (p, hes) in boundaries.iter().enumerate() {
            for &he in hes {
                let edge = body
                    .get_half_edge(he)
                    .unwrap_or_else(|| {
                        unreachable!(
                            "piece half-edge {he:?} is dangling: a boundary is read off this \
                             body's live faces' loops"
                        )
                    })
                    .edge;
                let ds = reached(edge);
                let Some(&first) = ds.first() else {
                    continue;
                };
                for &d in &ds {
                    link.join(first, d);
                }
                let across = body
                    .mate(he)
                    .and_then(|m| body.face_of_half_edge(m))
                    .unwrap_or_else(|| {
                        unreachable!(
                            "piece half-edge {he:?} has no face across it: every half-edge of a \
                             tier-1-valid body has a mate, and every half-edge lies in a face's \
                             loop"
                        )
                    });
                sides.push((ds, p, across));
            }
        }
        let met: Vec<(usize, usize, FaceKey)> = sides
            .into_iter()
            .filter_map(|(ds, p, across)| Some((link.root(*ds.first()?), p, across)))
            .collect();
        let mut touched: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for &(o, p, _) in &met {
            touched.entry(o).or_default().insert(p);
        }
        let divides = |o: usize| touched.get(&o).is_some_and(|ps| ps.len() >= 2);
        let mut walls: Vec<BTreeSet<W>> = vec![BTreeSet::new(); counted];
        for (o, p, across) in met {
            if divides(o) && p < counted {
                walls[p].insert(wall(across)?);
            }
        }
        if boundaries.len() >= 2 && walls.iter().any(BTreeSet::is_empty) {
            return Err(bug(refusal));
        }
        Ok(walls)
    }
}

/// `items` grouped by their wall sets, the sets sorted.
fn group<W: Ord, I: Copy>(walls: Vec<BTreeSet<W>>, items: &[I]) -> BTreeMap<Vec<W>, Vec<I>> {
    let mut out: BTreeMap<Vec<W>, Vec<I>> = BTreeMap::new();
    for (set, &i) in walls.into_iter().zip(items) {
        out.entry(set.into_iter().collect()).or_default().push(i);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::Obstacles;
    use std::collections::BTreeSet;
    use topo::{EdgeKey, FaceKey};

    /// A stop that ends the walk at a key an earlier walk saw does not
    /// end a walk that comes back to its own key: the cycle panics.
    #[test]
    #[should_panic(expected = "so its rows cycle")]
    fn a_lineage_cycle_panics_under_a_stop_at_seen_keys() {
        let [e1, e2]: [EdgeKey; 2] =
            [1, 2].map(|i| slotmap::KeyData::from_ffi((1 << 32) | i).into());
        let mut obstacles = Obstacles::<FaceKey>::new();
        obstacles.split_from.extend([(e1, e2), (e2, e1)]);
        let mut seen = BTreeSet::new();
        obstacles.up_lineage(&topo::Body::<f64>::new(), e1, |up| !seen.insert(up));
    }
}
