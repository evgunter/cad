//! **N2's `Borders`: a split face's pieces are told apart by the divider
//! walls they border**, read off the kernel's record of what each
//! boolean discarded (`topo::BooleanNaming::discards`).
//!
//! A face the result holds as several pieces lost the rest of its
//! region to the boolean. An OBSTACLE is a connected part of that lost
//! region: one discarded fragment, or several joined where they border
//! one edge of the result or where a later step discarded the region
//! across an earlier step's seam. Each piece edge is chased to the
//! stretch a discard recorded along it, through the edge's split
//! lineage and the lineage the discards themselves carry for edges that
//! died with them, and so to its obstacle. An obstacle that borders two
//! or more pieces DIVIDES the face; a piece's qualifier is the set of
//! walls — the faces across its edges — along which it meets a divider,
//! each cited by its parent's name. Nothing beyond a piece's own
//! boundary is read, so a boss standing on one piece, or a notch in it,
//! is an obstacle that borders one piece and is never cited.
//!
//! One record serves the pair boolean, which records one step, and the
//! union, which records each fold step and asks after the finished body
//! ([`Obstacles::split`]).

use std::collections::{BTreeMap, BTreeSet};

use topo::{Body, EdgeKey, FaceKey, Provenance, VertexKey};

use super::emit::{NamingError, face_half_edges};

/// One discarded face, resolved into the result of its step.
struct Discard<K> {
    /// What the discarded face is a fragment of, in the caller's keys.
    parents: BTreeSet<K>,
    /// The result edges it bordered a kept face along.
    seams: Vec<EdgeKey>,
    /// Seam edges of earlier steps its boundary descends from: the
    /// region across them was discarded before, so it is one obstacle
    /// with this one.
    touches: Vec<EdgeKey>,
}

/// The discards of one boolean, or of each step of a union's fold.
pub(crate) struct Obstacles<K> {
    discards: Vec<Discard<K>>,
    /// Edge → the edge it was split from, for edges that died with a
    /// discarded face and so left the result's own provenance.
    dead_split: BTreeMap<EdgeKey, EdgeKey>,
}

/// The emission error this module refuses with.
fn bug(what: &'static str) -> NamingError {
    NamingError::Emission { what }
}

impl<K: Ord + Clone> Obstacles<K> {
    pub(crate) fn new() -> Self {
        Self {
            discards: Vec::new(),
            dead_split: BTreeMap::new(),
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
        let merges: BTreeMap<VertexKey, VertexKey> = naming.vertex_merges.iter().copied().collect();
        let settle = |mut v: VertexKey| -> Result<VertexKey, NamingError> {
            for _ in 0..=merges.len() {
                match merges.get(&v) {
                    Some(&k) if k != v => v = k,
                    _ => return Ok(v),
                }
            }
            Err(bug("a boolean's vertex fusions form a cycle"))
        };
        let mut by_ends: BTreeMap<(VertexKey, VertexKey), Vec<EdgeKey>> = BTreeMap::new();
        for (k, e) in body.edges() {
            let s = body
                .get_half_edge(e.he_plus)
                .ok_or_else(|| bug("a result edge's half-edge is dangling"))?
                .start;
            let t = body
                .half_edge_end(e.he_plus)
                .ok_or_else(|| bug("a result edge has no end"))?;
            by_ends.entry((s.min(t), s.max(t))).or_default().push(k);
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
            for &(u, w) in &row.bordered {
                let (u, w) = (settle(u)?, settle(w)?);
                // A stretch no live edge joins merged away with the
                // faces beside it: nothing of a piece lies along it.
                if let Some(es) = by_ends.get(&(u.min(w), u.max(w))) {
                    seams.extend(es.iter().copied());
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
            self.discards.push(Discard {
                parents: parents_of(row.operand, row.face)?,
                seams,
                touches,
            });
        }
        Ok(())
    }

    /// **The one rule**: `pieces`, the faces of `body` that one face
    /// `parent` is held as, grouped by the divider walls each borders,
    /// each wall named by `wall`. Several pieces under one set are N2's
    /// tie. The sets come out sorted, in `W`'s order.
    ///
    /// Refuses when the record holds nothing that divides the pieces:
    /// a region the kernel discarded without recording it.
    pub(crate) fn split<T: geom_core::Real, W: Ord + Clone>(
        &self,
        body: &Body<T>,
        parent: &BTreeSet<K>,
        pieces: &[FaceKey],
        mut wall: impl FnMut(FaceKey) -> Result<W, NamingError>,
    ) -> Result<BTreeMap<Vec<W>, Vec<FaceKey>>, NamingError> {
        let mine: Vec<usize> = (0..self.discards.len())
            .filter(|&i| !self.discards[i].parents.is_disjoint(parent))
            .collect();
        // Union–find over this parent's discards; a class's root is its
        // least index, so no visiting order changes it.
        let mut link: BTreeMap<usize, usize> = mine.iter().map(|&i| (i, i)).collect();
        fn root(link: &BTreeMap<usize, usize>, mut i: usize) -> usize {
            while let Some(&up) = link.get(&i).filter(|up| **up != i) {
                i = up;
            }
            i
        }
        let join = |link: &mut BTreeMap<usize, usize>, a: usize, b: usize| {
            let (ra, rb) = (root(link, a), root(link, b));
            link.insert(ra.max(rb), ra.min(rb));
        };
        let mut seam_of: BTreeMap<EdgeKey, Vec<usize>> = BTreeMap::new();
        for &i in &mine {
            for &k in &self.discards[i].seams {
                seam_of.entry(k).or_default().push(i);
            }
        }
        for ds in seam_of.values() {
            for w in ds.windows(2) {
                join(&mut link, w[0], w[1]);
            }
        }
        for &i in &mine {
            for k in &self.discards[i].touches {
                for &j in seam_of.get(k).into_iter().flatten() {
                    join(&mut link, i, j);
                }
            }
        }
        // Each piece edge's obstacle, and the wall across it.
        let bound = self.dead_split.len() + body.edges().count() + 1;
        let obstacle = |mut e: EdgeKey| -> Result<Option<usize>, NamingError> {
            for _ in 0..bound {
                if let Some(ds) = seam_of.get(&e) {
                    return Ok(Some(root(&link, ds[0])));
                }
                e = match body.edge_provenance_of(e) {
                    Some(Provenance::SplitEdge { edge }) => *edge,
                    Some(_) => return Ok(None),
                    None => match self.dead_split.get(&e) {
                        Some(&up) => up,
                        None => return Ok(None),
                    },
                };
            }
            Err(bug("a piece edge's split lineage is cyclic"))
        };
        let mut met: Vec<(usize, usize, FaceKey)> = Vec::new();
        for (p, &f) in pieces.iter().enumerate() {
            for he in face_half_edges(body, f)? {
                let h = body
                    .get_half_edge(he)
                    .ok_or_else(|| bug("a piece's half-edge is dangling"))?;
                let Some(o) = obstacle(h.edge)? else {
                    continue;
                };
                let edge = body
                    .get_edge(h.edge)
                    .ok_or_else(|| bug("a piece's edge is dangling"))?;
                let mate = if edge.he_plus == he {
                    edge.he_minus
                } else {
                    edge.he_plus
                };
                let across = body
                    .get_loop(
                        body.get_half_edge(mate)
                            .ok_or_else(|| bug("a piece edge's mate is dangling"))?
                            .parent_loop,
                    )
                    .ok_or_else(|| bug("a piece edge's mate loop is dangling"))?
                    .face;
                met.push((o, p, across));
            }
        }
        let mut touched: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for &(o, p, _) in &met {
            touched.entry(o).or_default().insert(p);
        }
        let divides = |o: usize| touched.get(&o).is_some_and(|ps| ps.len() >= 2);
        if pieces.len() >= 2 && !touched.keys().any(|&o| divides(o)) {
            return Err(bug(
                "a face held as several pieces has no recorded discard between them",
            ));
        }
        let mut walls: Vec<BTreeSet<W>> = vec![BTreeSet::new(); pieces.len()];
        for (o, p, across) in met {
            if divides(o) {
                walls[p].insert(wall(across)?);
            }
        }
        let mut out: BTreeMap<Vec<W>, Vec<FaceKey>> = BTreeMap::new();
        for (set, &f) in walls.into_iter().zip(pieces) {
            out.entry(set.into_iter().collect()).or_default().push(f);
        }
        Ok(out)
    }
}
