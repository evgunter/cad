//! **N2's `Borders`: a split face's pieces are told apart by the divider
//! walls they border**, read off the kernel's record of what each
//! boolean discarded (`topo::BooleanNaming::discards`).
//!
//! A face the result holds as several pieces lost the rest of its
//! region to the boolean. An OBSTACLE is a connected part of that lost
//! region: one discarded fragment, or several joined where they border
//! one edge of the result, where a later step discarded the region
//! across an earlier step's seam, or where both sides of an edge that
//! died with them were discarded region of the parent: two fragments of
//! one face across a chord that cut it, or two of a corner's faces
//! across an edge between them. Each piece edge is chased to the
//! stretch a discard recorded along it, through the edge's split
//! lineage, the lineage the discards themselves carry for edges that
//! died with them, and the edges a later step's join made it of, and so
//! to its obstacle; discards one edge reaches are one obstacle. A face discarded where a
//! coincident kept face holds part of its region borders that face
//! where the held part ends (`topo::DiscardRow::held`). An obstacle that borders two
//! or more pieces DIVIDES the face; a piece's qualifier is the set of
//! walls — the faces across its edges — along which it meets a divider,
//! each cited by its parent's name. Nothing beyond a piece's own
//! boundary is read, so a boss standing on one piece, or a notch in it,
//! is an obstacle that borders one piece and is never cited.
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

use std::collections::{BTreeMap, BTreeSet};

use topo::{Body, Cell, EdgeKey, FaceKey, Provenance, VertexKey};

use super::emit::{NamingError, face_half_edges};
use super::least_root::LeastRoot;

/// One discarded face, resolved into the result of its step.
struct Discard {
    /// The result edges it bordered a kept face along.
    seams: Vec<EdgeKey>,
    /// Seam edges of earlier steps its boundary descends from: the
    /// region across them was discarded before, so it is one obstacle
    /// with this one.
    touches: Vec<EdgeKey>,
    /// Its other boundary edges, which it died with: another discard of
    /// the parent across one of them — a fragment of the same face
    /// across a chord, or of another of a corner's faces across the edge
    /// between them — is region of the parent joined to this one there.
    bounds: Vec<EdgeKey>,
}

/// The discards of one boolean, or of each step of a union's fold.
pub(crate) struct Obstacles<K> {
    discards: Vec<Discard>,
    /// Parent key → the discards descending from it (what the caller's
    /// `parents_of` named each a fragment of).
    by_parent: BTreeMap<K, Vec<usize>>,
    /// Edge → the edge it was split from, for edges that died with a
    /// discarded face and so left the result's own provenance.
    dead_split: BTreeMap<EdgeKey, EdgeKey>,
    /// Edge → the edges an output stage's join made it out of, besides
    /// itself.
    joined_from: BTreeMap<EdgeKey, Vec<EdgeKey>>,
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
            dead_split: BTreeMap::new(),
            joined_from: BTreeMap::new(),
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
        let settler = naming.settler(body);
        // A bordered stretch settles on a seam edge: the zip made the
        // kept face's section edge one edge with the wall's. A held
        // stretch settles on the kept face's own edge, whichever it is.
        let seam_edges: BTreeSet<EdgeKey> = naming
            .seam_edges
            .iter()
            .map(|&e| naming.joined_edge(e))
            .collect();
        let mut by_ends: BTreeMap<(VertexKey, VertexKey), Vec<EdgeKey>> = BTreeMap::new();
        let mut any_by_ends: BTreeMap<(VertexKey, VertexKey), Vec<EdgeKey>> = BTreeMap::new();
        for (k, e) in body.edges() {
            let s = body
                .get_half_edge(e.he_plus)
                .ok_or_else(|| bug("a result edge's half-edge is dangling"))?
                .start;
            let t = body
                .half_edge_end(e.he_plus)
                .ok_or_else(|| bug("a result edge has no end"))?;
            if seam_edges.contains(&k) {
                by_ends.entry((s.min(t), s.max(t))).or_default().push(k);
            }
            any_by_ends.entry((s.min(t), s.max(t))).or_default().push(k);
        }
        // An earlier step's seam this step's output stage joined into
        // another edge lies along the joined edge, whichever of the two
        // the join kept.
        for d in &mut self.discards {
            let joined: Vec<EdgeKey> = d
                .seams
                .iter()
                .map(|&e| naming.joined_edge(e))
                .filter(|j| !d.seams.contains(j))
                .collect();
            d.seams.extend(joined);
        }
        for j in &naming.edge_joins {
            self.joined_from.entry(j.kept).or_default().push(j.gone);
        }
        let earlier: BTreeSet<EdgeKey> = self
            .discards
            .iter()
            .flat_map(|d| d.seams.iter().copied())
            .collect();
        // A row's chains are in its operand's clone keys: A's are the
        // result's, B's are read through the graft's edge rows, and an
        // ancestor the graft does not carry ends the chain.
        if !naming.discards.is_empty()
            && (naming.a_keys, naming.b_keys)
                != (topo::OperandKeys::Direct, topo::OperandKeys::Grafted)
        {
            return Err(bug(
                "a boolean recorded discards in a key layout it has no rows for",
            ));
        }
        let grafted: BTreeMap<EdgeKey, EdgeKey> = naming
            .graft_edges
            .iter()
            .chain(&naming.graft_dead_edges)
            .copied()
            .collect();
        for row in &naming.discards {
            let chains: Vec<Vec<EdgeKey>> = row
                .boundary_chains
                .iter()
                .map(|chain| match row.operand {
                    topo::Operand::A => chain.clone(),
                    topo::Operand::B => chain
                        .iter()
                        .map_while(|k| grafted.get(k).copied())
                        .collect(),
                })
                .collect();
            let mut seams = Vec::new();
            // A stretch no live edge joins merged away with the faces
            // beside it: nothing of a piece lies along it.
            // A stretch an end of which the output stage joined away lies
            // along the edge the join made.
            for (stretches, edges, seam_only) in [
                (&row.bordered, &by_ends, true),
                (&row.held, &any_by_ends, false),
            ] {
                for &(u, w) in stretches {
                    let (u, w) = (settler.settle(u)?, settler.settle(w)?);
                    let between = match (u, w) {
                        (Cell::Vertex(u), Cell::Vertex(w)) => edges.get(&(u.min(w), u.max(w))),
                        // An end the join deleted lies inside the joined
                        // edge, and one the merge pruned inside the merged
                        // face: neither ends an edge, so the stretch is
                        // read through the joins or lies along none.
                        (Cell::Edge(_) | Cell::Face(_), _) | (_, Cell::Edge(_) | Cell::Face(_)) => {
                            None
                        }
                    };
                    if let Some(es) = between {
                        seams.extend(es.iter().copied());
                    } else if let Some(e) = naming.stretch_through_joins(body, (u, w))
                        && (!seam_only || seam_edges.contains(&e))
                    {
                        seams.push(e);
                    }
                }
            }
            for chain in &chains {
                for w in chain.windows(2) {
                    self.dead_split.insert(w[0], w[1]);
                }
            }
            let touches = chains
                .iter()
                .flatten()
                .filter(|k| earlier.contains(k))
                .copied()
                .collect();
            let parents = parents_of(row.operand, row.face)?;
            for k in &parents {
                self.by_parent
                    .entry(k.clone())
                    .or_default()
                    .push(self.discards.len());
            }
            let bounds = chains.iter().filter_map(|c| c.first().copied()).collect();
            self.discards.push(Discard {
                seams,
                touches,
                bounds,
            });
        }
        Ok(())
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
            .collect::<Result<Vec<_>, _>>()?;
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
            let star = body
                .edges_of_vertex(v)
                .ok_or_else(|| bug("a corner copy's star does not resolve"))?;
            if star.is_empty() {
                return Err(bug("a corner copy has no edge"));
            }
            for e in star {
                let edge = body
                    .get_edge(e)
                    .ok_or_else(|| bug("a corner copy's edge is dangling"))?;
                for he in [edge.he_plus, edge.he_minus] {
                    let f = body
                        .face_of_half_edge(he)
                        .ok_or_else(|| bug("a corner copy's half-edge has no face"))?;
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
        // This parent's discards, joined into obstacles.
        let mut link = LeastRoot::new();
        let mut seam_of: BTreeMap<EdgeKey, Vec<usize>> = BTreeMap::new();
        for &i in &mine {
            link.insert(i);
            for &k in &self.discards[i].seams {
                seam_of.entry(k).or_default().push(i);
            }
        }
        for ds in seam_of.values() {
            for w in ds.windows(2) {
                link.join(w[0], w[1]);
            }
        }
        // Two discards that both bounded an edge which died with them
        // are one connected part of the parent's lost region (N2's
        // obstacle).
        let mut bound_of: BTreeMap<EdgeKey, Vec<usize>> = BTreeMap::new();
        for &i in &mine {
            for &k in &self.discards[i].bounds {
                bound_of.entry(k).or_default().push(i);
            }
        }
        for ds in bound_of.values() {
            for w in ds.windows(2) {
                link.join(w[0], w[1]);
            }
        }
        for &i in &mine {
            for k in &self.discards[i].touches {
                for &j in seam_of.get(k).into_iter().flatten() {
                    link.join(i, j);
                }
            }
        }
        // Each boundary edge's discards, and the wall across it. An edge
        // reaches a discard along its split lineage, the first recorded
        // seam on it, and through every edge a join made it of, so an
        // edge the join made of two seams meets the discards of both,
        // and they are one obstacle: both border it on its far side.
        let reached = |e: EdgeKey| -> Result<BTreeSet<usize>, NamingError> {
            let mut out = BTreeSet::new();
            let mut seen = BTreeSet::new();
            let mut todo = vec![e];
            'chain: while let Some(mut e) = todo.pop() {
                let mut chain = BTreeSet::new();
                loop {
                    if !chain.insert(e) {
                        return Err(bug("a piece edge's split lineage is cyclic"));
                    }
                    if !seen.insert(e) {
                        continue 'chain;
                    }
                    todo.extend(self.joined_from.get(&e).into_iter().flatten());
                    if let Some(ds) = seam_of.get(&e) {
                        out.extend(ds.iter().copied());
                        continue 'chain;
                    }
                    e = match body.edge_provenance_of(e) {
                        Some(Provenance::SplitEdge { edge }) => *edge,
                        Some(_) => continue 'chain,
                        None => match self.dead_split.get(&e) {
                            Some(&up) => up,
                            None => continue 'chain,
                        },
                    };
                }
            }
            Ok(out)
        };
        let mut sides: Vec<(BTreeSet<usize>, usize, FaceKey)> = Vec::new();
        for (p, hes) in boundaries.iter().enumerate() {
            for &he in hes {
                let edge = body
                    .get_half_edge(he)
                    .ok_or_else(|| bug("a piece's half-edge is dangling"))?
                    .edge;
                let ds = reached(edge)?;
                let Some(&first) = ds.first() else {
                    continue;
                };
                for &d in &ds {
                    link.join(first, d);
                }
                let across = body
                    .mate(he)
                    .and_then(|m| body.face_of_half_edge(m))
                    .ok_or_else(|| bug("a piece edge has no face across it"))?;
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
