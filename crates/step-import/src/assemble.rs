//! Topology assembly (Leg C, phase A): realize each `CLOSED_SHELL`'s
//! half-edge structure through `topo`'s public Euler operators.
//!
//! # The method: rotation-system insertion
//!
//! The file's loops determine the target half-edge structure
//! completely: each `EDGE_CURVE` has exactly two uses (one forward,
//! one reversed), `next` is the loop order, `mate` pairs the two uses,
//! and the **fan order around each vertex** (the rotation system) is
//! the derived orbit `σ(u) = next(mate(u))` — clockwise around the
//! vertex viewed from outside, `topo`'s own orbit step. Edges are
//! inserted one at a time, each spliced at its correct fan position by
//! anchoring on the *nearest already-built σ-successor*; because every
//! operator splices "immediately before" its anchor half-edges, the
//! relative fan order of built uses always matches the target, and
//! when the last edge lands the body's loop cycles **are** the file's
//! loops (verified explicitly — assembly cannot silently produce a
//! different complex).
//!
//! The op per edge is Mäntylä's case analysis (ch. 9, the
//! completeness construction) in `topo`'s vocabulary:
//!
//! - far endpoint new → `mev` (vertex splitting; a pendant edge is a
//!   strut in the current loop, which later insertions absorb);
//! - both endpoints built, anchors in one loop → `mef` (polygon
//!   cutting; the transient face partition is re-formed in phase B);
//! - anchors in different loops → `mekr` after making the dying loop
//!   a ring of the surviving loop's face (`ring_move` / `kfmrh` — the
//!   connected sum: same-shell `kfmrh` is exactly how the file's genus
//!   re-enters);
//! - a start vertex no insertable edge can reach (a vertex with only
//!   incoming edges, or a second vertex-graph component — full-period
//!   walls have edge graphs the vertex graph does not connect) is
//!   **planted**: scaffold strut + `kemr` leaves it as a lone vertex
//!   in an empty ring (Mäntylä §9.3's hole-planting state), and the
//!   `mekr` empty-loop sites grow real edges from it;
//! - the one genuinely ambiguous splice — a self-loop whose two uses
//!   share their only built anchor — is resolved with a **temporary
//!   strut** at the anchor position (a distinct fan slot to splice
//!   against), killed by `kev` immediately after.
//!
//! All phase-A geometry is the operators' own chord/self-loop
//! scaffolding sugar (`mev_line` / `mef_chord` / `mekr_chord`); every
//! edge is re-described and re-certified against its real parsed
//! carrier in phase C (`adopt`), so no scaffold survives to rest.
//!
//! # Direction discipline
//!
//! `he_plus` must traverse the carrier's increasing parameter (the
//! kernel's forward contract), and the file's forward use is the
//! `.T.` `ORIENTED_EDGE`. Every insertion site here mints `he_plus`
//! from the **file-start** vertex (`mev` from it, `mef`'s `he1` /
//! `mekr`'s target anchored at it), so the mapping forward-use ↦
//! `he_plus` holds by construction and phase C's specs never need a
//! reversed parameterization (which would move bits).

use std::collections::BTreeMap;

use topo::{Body, HalfEdgeKey, LoopKey, MefSite, MekrSite, MevSite, VertexKey};

use crate::adopt;
use crate::entities::SolidSpec;
use crate::error::StepImportError;
use geom_core::Tol;

/// One edge-use in the flattened target complex.
#[derive(Clone, Copy, Debug)]
struct Use {
    /// The `EDGE_CURVE` id.
    edge: u64,
    /// Traverses start → end (the `.T.` use — the plus half).
    forward: bool,
}

/// The flattened target complex of one shell (module docs).
pub(crate) struct Target {
    /// All uses, indexed by flat use id.
    uses: Vec<Use>,
    /// Per-loop use sequences (cyclic, file order). Outer loop of face
    /// f is `loops[loop_of_face[f].0]`; rings follow.
    pub(crate) loops: Vec<Vec<usize>>,
    /// Per-face (outer, rings) loop indices, in `CLOSED_SHELL` face
    /// order.
    pub(crate) face_loops: Vec<(usize, Vec<usize>)>,
    /// `next` in the loop cycle, per use.
    next: Vec<usize>,
    /// The other use of the same edge, per use.
    mate: Vec<usize>,
    /// The vertex (file id) each use starts at.
    start_vertex: Vec<u64>,
    /// The two uses of each edge: (forward, reversed).
    edge_uses: BTreeMap<u64, (usize, usize)>,
}

impl Target {
    /// Flattens a `SolidSpec` (module docs). The manifold two-use
    /// precondition was checked at resolution.
    fn build(solid: &SolidSpec) -> Result<Self, StepImportError> {
        let mut uses = Vec::new();
        let mut loops = Vec::new();
        let mut face_loops = Vec::new();
        for face in &solid.faces {
            let mut ring_ids = Vec::new();
            let mut outer_id = 0;
            for lp in &face.loops {
                let loop_idx = loops.len();
                let mut seq = Vec::new();
                for use_ in &lp.uses {
                    seq.push(uses.len());
                    uses.push(Use {
                        edge: use_.edge,
                        forward: use_.forward,
                    });
                }
                loops.push(seq);
                if lp.outer {
                    outer_id = loop_idx;
                } else {
                    ring_ids.push(loop_idx);
                }
            }
            face_loops.push((outer_id, ring_ids));
        }
        let mut next = vec![0; uses.len()];
        for seq in &loops {
            for (i, &u) in seq.iter().enumerate() {
                next[u] = seq[(i + 1) % seq.len()];
            }
        }
        let mut edge_uses: BTreeMap<u64, (Option<usize>, Option<usize>)> = BTreeMap::new();
        for (i, use_) in uses.iter().enumerate() {
            let entry = edge_uses.entry(use_.edge).or_insert((None, None));
            if use_.forward {
                entry.0 = Some(i);
            } else {
                entry.1 = Some(i);
            }
        }
        let mut mate = vec![0; uses.len()];
        let mut pairs = BTreeMap::new();
        for (&edge, &(fwd, rev)) in &edge_uses {
            let (Some(fwd), Some(rev)) = (fwd, rev) else {
                return Err(StepImportError::Topology {
                    id: edge,
                    what: "an EDGE_CURVE without exactly one forward and one reversed \
                           use (unreachable past the resolution check)",
                });
            };
            mate[fwd] = rev;
            mate[rev] = fwd;
            pairs.insert(edge, (fwd, rev));
        }
        let mut start_vertex = vec![0; uses.len()];
        for (i, use_) in uses.iter().enumerate() {
            let spec = &solid.edges[&use_.edge];
            start_vertex[i] = if use_.forward { spec.start } else { spec.end };
        }
        Ok(Self {
            uses,
            loops,
            face_loops,
            next,
            mate,
            start_vertex,
            edge_uses: pairs,
        })
    }

    /// The derived orbit step: the next use starting at the same
    /// vertex, clockwise viewed from outside (module docs).
    fn sigma(&self, u: usize) -> usize {
        self.next[self.mate[u]]
    }

    /// The (forward, reversed) use indices of an edge.
    pub(crate) fn edge_uses_of(&self, edge: u64) -> Option<(usize, usize)> {
        self.edge_uses.get(&edge).copied()
    }
}

/// A file vertex's build state during insertion.
#[derive(Clone, Copy, Debug)]
enum VState {
    /// Not in the body yet.
    Unbuilt,
    /// A lone vertex in an empty loop (the `mvfs` seed or a planted
    /// hole anchor) — grown by the `Lone`/empty-loop op sites.
    Lone {
        /// The body vertex.
        vertex: VertexKey,
        /// Its empty loop.
        r#loop: LoopKey,
    },
    /// Carrying at least one built use.
    Built(VertexKey),
}

/// Phase-A output handed to `adopt`: the use → half-edge realization.
pub(crate) struct Assembled {
    /// The flattened target (loops, face partition).
    pub(crate) target: Target,
    /// The body half-edge realizing each target use.
    pub(crate) use_he: Vec<HalfEdgeKey>,
}

/// The per-shell insertion engine.
struct Builder<'a> {
    body: &'a mut Body<f64>,
    target: Target,
    solid: &'a SolidSpec,
    /// `Some(he)` once a use is realized.
    use_he: Vec<Option<HalfEdgeKey>>,
    vstate: BTreeMap<u64, VState>,
}

impl<'a> Builder<'a> {
    /// Wraps an operator refusal with the entity being assembled.
    fn op_err(id: u64) -> impl FnOnce(topo::EulerOpError) -> StepImportError {
        move |source| StepImportError::Assembly { id, source }
    }

    /// The nearest built σ-successor of `u` — the fan-position anchor
    /// every splice is stated against. `None` iff no use at `u`'s
    /// vertex is built yet.
    fn anchor(&self, u: usize) -> Option<HalfEdgeKey> {
        let mut w = self.target.sigma(u);
        for _ in 0..self.target.uses.len() {
            if let Some(he) = self.use_he[w] {
                return Some(he);
            }
            if w == u {
                return None;
            }
            w = self.target.sigma(w);
        }
        None
    }

    /// Marks a vertex built (post-insertion bookkeeping).
    fn set_built(&mut self, vertex_id: u64, key: VertexKey) {
        self.vstate.insert(vertex_id, VState::Built(key));
    }

    /// Inserts edge `edge_id` via `mev`: the start vertex is present,
    /// the end vertex is new. `he_plus` runs start → end (the forward
    /// contract; module docs, direction discipline).
    fn insert_mev(&mut self, edge_id: u64, tol: Tol) -> Result<(), StepImportError> {
        let spec = &self.solid.edges[&edge_id];
        let (fwd, rev) = self.target.edge_uses[&edge_id];
        let p_end = self.solid.vertices[&spec.end];
        let (site, start_key) = match self.vstate[&spec.start] {
            VState::Lone { vertex, r#loop } => (MevSite::Lone { r#loop }, vertex),
            VState::Built(vertex) => {
                let anchor = self.anchor(fwd).ok_or(StepImportError::Topology {
                    id: edge_id,
                    what: "internal: a built vertex with no built fan anchor \
                           (assembly invariant broken)",
                })?;
                (
                    MevSite::Fan {
                        he1: anchor,
                        he2: anchor,
                    },
                    vertex,
                )
            }
            VState::Unbuilt => {
                return Err(StepImportError::Topology {
                    id: edge_id,
                    what: "internal: mev insertion at an unbuilt start vertex",
                });
            }
        };
        let created = self
            .body
            .mev_line(site, p_end, tol)
            .map_err(Self::op_err(edge_id))?;
        self.use_he[fwd] = Some(created.he_plus);
        self.use_he[rev] = Some(created.he_minus);
        self.set_built(spec.start, start_key);
        self.set_built(spec.end, created.vertex);
        Ok(())
    }

    /// Re-partitions loops/faces (pure topology, no certification)
    /// until `dying` is a **ring-designated** loop of `keep`'s face —
    /// the shape every `mekr` site requires of the loop it kills.
    /// Vocabulary: `ring_move` re-homes a ring; `mfkrh` promotes
    /// `keep` to its own face when `dying` is `keep`'s own face's
    /// outer; `kfmrh` demotes `dying`'s whole face when `dying` is an
    /// outer (the connected sum — same-shell genus is exactly the
    /// file's genus re-entering).
    fn make_ring_of(
        &mut self,
        edge_id: u64,
        keep: LoopKey,
        dying: LoopKey,
    ) -> Result<(), StepImportError> {
        let err = |what| StepImportError::Topology { id: edge_id, what };
        let keep_face = self
            .body
            .get_loop(keep)
            .ok_or(err("internal: keep loop does not resolve"))?
            .face;
        let dying_face = self
            .body
            .get_loop(dying)
            .ok_or(err("internal: dying loop does not resolve"))?
            .face;
        let dying_is_outer = self
            .body
            .get_face(dying_face)
            .ok_or(err("internal: dying face does not resolve"))?
            .outer
            == dying;
        if dying_face == keep_face {
            if !dying_is_outer {
                return Ok(()); // Already a ring of the right face.
            }
            // `dying` is the outer of `keep`'s own face, so `keep` is
            // one of its rings: promote `keep` to a fresh face, then
            // fall through to the cross-face demotion below.
            self.body.mfkrh_plug(keep).map_err(Self::op_err(edge_id))?;
            return self.make_ring_of(edge_id, keep, dying);
        }
        if dying_is_outer {
            // Demote `dying`'s face wholesale: rings off first
            // (kfmrh's no-rings rule), then the connected sum.
            let rings = self
                .body
                .get_face(dying_face)
                .ok_or(err("internal: dying face does not resolve"))?
                .rings
                .clone();
            let keep_face = self
                .body
                .get_loop(keep)
                .ok_or(err("internal: keep loop does not resolve"))?
                .face;
            for ring in rings {
                self.body
                    .ring_move(ring, keep_face)
                    .map_err(Self::op_err(edge_id))?;
            }
            self.body
                .kfmrh(keep_face, dying_face)
                .map_err(Self::op_err(edge_id))?;
        } else {
            let keep_face = self
                .body
                .get_loop(keep)
                .ok_or(err("internal: keep loop does not resolve"))?
                .face;
            self.body
                .ring_move(dying, keep_face)
                .map_err(Self::op_err(edge_id))?;
        }
        Ok(())
    }

    /// The parent loop of a body half-edge.
    fn parent_loop(&self, he: HalfEdgeKey, edge_id: u64) -> Result<LoopKey, StepImportError> {
        Ok(self
            .body
            .get_half_edge(he)
            .ok_or(StepImportError::Topology {
                id: edge_id,
                what: "internal: anchor half-edge does not resolve",
            })?
            .parent_loop)
    }

    /// Inserts edge `edge_id` as a chord: both endpoints present
    /// (`Lone` or `Built`). Every site is stated so that `he_plus`
    /// runs file-start → file-end (module docs, direction discipline).
    fn insert_chord(&mut self, edge_id: u64, tol: Tol) -> Result<(), StepImportError> {
        let spec = &self.solid.edges[&edge_id];
        let (fwd, rev) = self.target.edge_uses[&edge_id];
        let no_anchor = StepImportError::Topology {
            id: edge_id,
            what: "internal: a built vertex with no built fan anchor",
        };
        let s_state = self.vstate[&spec.start];
        let e_state = self.vstate[&spec.end];
        let (he_plus, he_minus, start_key, end_key) = match (s_state, e_state) {
            // Self-loop at a lone vertex: ch. 9's circular edge.
            (VState::Lone { vertex, r#loop }, _) if spec.start == spec.end => {
                let c = self
                    .body
                    .mef_chord(MefSite::Lone { r#loop }, tol)
                    .map_err(Self::op_err(edge_id))?;
                (c.he_plus, c.he_minus, vertex, vertex)
            }
            (
                VState::Lone {
                    vertex: v1,
                    r#loop: l1,
                },
                VState::Lone {
                    vertex: v2,
                    r#loop: l2,
                },
            ) => {
                self.make_ring_of(edge_id, l1, l2)?;
                let c = self
                    .body
                    .mekr_chord(
                        MekrSite::BothEmpty {
                            target: l1,
                            ring: l2,
                        },
                        tol,
                    )
                    .map_err(Self::op_err(edge_id))?;
                (c.he_plus, c.he_minus, v1, v2)
            }
            (
                VState::Lone {
                    vertex: v1,
                    r#loop: l1,
                },
                VState::Built(v2),
            ) => {
                let s_r = self.anchor(rev).ok_or(no_anchor)?;
                let dying = self.parent_loop(s_r, edge_id)?;
                self.make_ring_of(edge_id, l1, dying)?;
                let c = self
                    .body
                    .mekr_chord(
                        MekrSite::EmptyTarget {
                            target: l1,
                            ring: s_r,
                        },
                        tol,
                    )
                    .map_err(Self::op_err(edge_id))?;
                (c.he_plus, c.he_minus, v1, v2)
            }
            (
                VState::Built(v1),
                VState::Lone {
                    vertex: v2,
                    r#loop: l2,
                },
            ) => {
                let s_f = self.anchor(fwd).ok_or(no_anchor)?;
                let keep = self.parent_loop(s_f, edge_id)?;
                self.make_ring_of(edge_id, keep, l2)?;
                let c = self
                    .body
                    .mekr_chord(
                        MekrSite::EmptyRing {
                            target: s_f,
                            ring: l2,
                        },
                        tol,
                    )
                    .map_err(Self::op_err(edge_id))?;
                (c.he_plus, c.he_minus, v1, v2)
            }
            (VState::Built(v1), VState::Built(v2)) => {
                let s_f = self.anchor(fwd).ok_or(StepImportError::Topology {
                    id: edge_id,
                    what: "internal: a built vertex with no built fan anchor",
                })?;
                let s_r = self.anchor(rev).ok_or(no_anchor)?;
                if s_f == s_r {
                    // The tied self-loop splice (module docs).
                    let (p, m) = self.insert_selfloop_tied(edge_id, fwd, rev, s_f, tol)?;
                    (p, m, v1, v2)
                } else {
                    let lf = self.parent_loop(s_f, edge_id)?;
                    let lr = self.parent_loop(s_r, edge_id)?;
                    if lf == lr {
                        let c = self
                            .body
                            .mef_chord(MefSite::Chords { he1: s_f, he2: s_r }, tol)
                            .map_err(Self::op_err(edge_id))?;
                        (c.he_plus, c.he_minus, v1, v2)
                    } else {
                        self.make_ring_of(edge_id, lf, lr)?;
                        let c = self
                            .body
                            .mekr_chord(
                                MekrSite::Cycles {
                                    target: s_f,
                                    ring: s_r,
                                },
                                tol,
                            )
                            .map_err(Self::op_err(edge_id))?;
                        (c.he_plus, c.he_minus, v1, v2)
                    }
                }
            }
            (VState::Unbuilt, _) | (_, VState::Unbuilt) => {
                return Err(StepImportError::Topology {
                    id: edge_id,
                    what: "internal: chord insertion with an unbuilt endpoint",
                });
            }
        };
        self.use_he[fwd] = Some(he_plus);
        self.use_he[rev] = Some(he_minus);
        self.set_built(spec.start, start_key);
        self.set_built(spec.end, end_key);
        Ok(())
    }

    /// The tied self-loop: both uses share their single built anchor
    /// `s`, so the fan order between the two new halves must be forced
    /// explicitly. Target order `(fwd, rev, s)` is the plain
    /// empty-run `mef`; `(rev, fwd, s)` takes a **temporary strut** as
    /// a second splice anchor, killed by `kev` immediately after
    /// (module docs).
    fn insert_selfloop_tied(
        &mut self,
        edge_id: u64,
        fwd: usize,
        rev: usize,
        s: HalfEdgeKey,
        tol: Tol,
    ) -> Result<(HalfEdgeKey, HalfEdgeKey), StepImportError> {
        // Which of the two uses comes first in the target fan,
        // walking σ from the forward use until the reversed use or a
        // built use appears?
        let mut fwd_first = true;
        let mut w = self.target.sigma(fwd);
        for _ in 0..self.target.uses.len() {
            if w == rev {
                break; // (fwd, rev, …, s): plain empty-run mef.
            }
            if self.use_he[w].is_some() {
                fwd_first = false; // (fwd, …, s, …, rev) ≡ (rev, fwd, s).
                break;
            }
            w = self.target.sigma(w);
        }
        if fwd_first {
            let c = self
                .body
                .mef_chord(MefSite::Chords { he1: s, he2: s }, tol)
                .map_err(Self::op_err(edge_id))?;
            return Ok((c.he_plus, c.he_minus));
        }
        // Strut before `s`, self-loop spliced around it, strut killed.
        let spec = &self.solid.edges[&edge_id];
        let p = self.solid.vertices[&spec.start];
        let offset = geom_core::Point3::new(p.x + 1.0, p.y, p.z);
        let strut = self
            .body
            .mev_line(MevSite::Fan { he1: s, he2: s }, offset, tol)
            .map_err(Self::op_err(edge_id))?;
        let c = self
            .body
            .mef_chord(
                MefSite::Chords {
                    he1: s,
                    he2: strut.he_plus,
                },
                tol,
            )
            .map_err(Self::op_err(edge_id))?;
        self.body
            .kev(strut.he_plus)
            .map_err(Self::op_err(edge_id))?;
        Ok((c.he_plus, c.he_minus))
    }

    /// Plants `vertex_id` as a lone vertex in an empty ring: scaffold
    /// strut from a built anchor, `kemr` strands the far vertex
    /// (Mäntylä §9.3's hole-planting state). Reaches vertices no
    /// insertable edge can (module docs).
    fn plant(&mut self, edge_id: u64, vertex_id: u64, tol: Tol) -> Result<(), StepImportError> {
        let p = self.solid.vertices[&vertex_id];
        // Deterministic anchor scan: the first realized use whose
        // start position differs from `p` (a zero-length scaffold
        // chord cannot certify).
        for i in 0..self.use_he.len() {
            let Some(he) = self.use_he[i] else { continue };
            let anchor_pos = self.solid.vertices[&self.target.start_vertex[i]];
            // Bitwise coincidence check (a zero-length chord poisons).
            if anchor_pos.x.to_bits() == p.x.to_bits()
                && anchor_pos.y.to_bits() == p.y.to_bits()
                && anchor_pos.z.to_bits() == p.z.to_bits()
            {
                continue;
            }
            let strut = self
                .body
                .mev_line(MevSite::Fan { he1: he, he2: he }, p, tol)
                .map_err(Self::op_err(edge_id))?;
            let kill = self
                .body
                .kemr(strut.he_plus, strut.he_minus)
                .map_err(Self::op_err(edge_id))?;
            self.vstate.insert(
                vertex_id,
                VState::Lone {
                    vertex: strut.vertex,
                    r#loop: kill.ring,
                },
            );
            return Ok(());
        }
        Err(StepImportError::Topology {
            id: edge_id,
            what: "no scaffold anchor for an unreachable vertex (every built vertex \
                   coincides with it, or nothing is built yet)",
        })
    }

    /// The insertion loop: repeatedly the first insertable edge in id
    /// order — `mev` growth first, then chords, then self-loops (so a
    /// self-loop's vertex carries every possible anchor before its
    /// tie-prone splice), planting a start vertex when nothing is
    /// insertable.
    fn run(&mut self, tol: Tol) -> Result<(), StepImportError> {
        let mut remaining: Vec<u64> = self.target.edge_uses.keys().copied().collect();
        while !remaining.is_empty() {
            let state = |b: &Self, v: u64| -> u8 {
                match b.vstate.get(&v) {
                    None | Some(VState::Unbuilt) => 0,
                    _ => 1,
                }
            };
            let mut pick: Option<(usize, bool)> = None; // (index, is_mev)
            // Pass 1: tree growth (start present, end new).
            for (i, &e) in remaining.iter().enumerate() {
                let spec = &self.solid.edges[&e];
                if spec.start != spec.end
                    && state(self, spec.start) == 1
                    && state(self, spec.end) == 0
                {
                    pick = Some((i, true));
                    break;
                }
            }
            // Pass 2: non-self-loop chords.
            if pick.is_none() {
                for (i, &e) in remaining.iter().enumerate() {
                    let spec = &self.solid.edges[&e];
                    if spec.start != spec.end
                        && state(self, spec.start) == 1
                        && state(self, spec.end) == 1
                    {
                        pick = Some((i, false));
                        break;
                    }
                }
            }
            // Pass 3: self-loops.
            if pick.is_none() {
                for (i, &e) in remaining.iter().enumerate() {
                    let spec = &self.solid.edges[&e];
                    if spec.start == spec.end && state(self, spec.start) == 1 {
                        pick = Some((i, false));
                        break;
                    }
                }
            }
            let Some((index, is_mev)) = pick else {
                // Nothing insertable: plant the first remaining
                // edge's start vertex and retry.
                let e = remaining[0];
                let start = self.solid.edges[&e].start;
                self.plant(e, start, tol)?;
                continue;
            };
            let e = remaining.remove(index);
            if is_mev {
                self.insert_mev(e, tol)?;
            } else {
                self.insert_chord(e, tol)?;
            }
        }
        Ok(())
    }

    /// The post-insertion check (module docs): the body's loop cycles
    /// must BE the file's loops — same half-edge sequences, distinct
    /// loops distinct. Assembly cannot silently produce a different
    /// complex.
    fn verify(&self, solid_id: u64) -> Result<(), StepImportError> {
        let defect = |what| StepImportError::Topology { id: solid_id, what };
        let mut seen_loops = Vec::new();
        for seq in &self.target.loops {
            let expected: Vec<HalfEdgeKey> = seq
                .iter()
                .map(|&u| {
                    self.use_he[u].ok_or(defect("internal: an unrealized use survived assembly"))
                })
                .collect::<Result<_, _>>()?;
            let actual = self
                .body
                .loop_cycle(expected[0])
                .ok_or(defect("internal: a realized loop's cycle does not close"))?;
            if actual.len() != expected.len() {
                return Err(defect(
                    "assembly verification: a loop's realized cycle has the wrong length",
                ));
            }
            let start = actual
                .iter()
                .position(|&he| he == expected[0])
                .ok_or(defect("internal: a loop cycle omits its own anchor"))?;
            for (i, &he) in expected.iter().enumerate() {
                if actual[(start + i) % actual.len()] != he {
                    return Err(defect(
                        "assembly verification: a realized loop's cycle order diverges \
                         from the file's loop",
                    ));
                }
            }
            let parent = self
                .body
                .get_half_edge(expected[0])
                .ok_or(defect("internal: a realized half-edge does not resolve"))?
                .parent_loop;
            if seen_loops.contains(&parent) {
                return Err(defect(
                    "assembly verification: two file loops realized into one body loop",
                ));
            }
            seen_loops.push(parent);
        }
        Ok(())
    }
}

/// Assembles one `MANIFOLD_SOLID_BREP` into a new solid of `body`
/// (phase A + verification), then hands the realization to the
/// adoption phases (rings, surfaces, senses, edge descriptions).
fn assemble_solid(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    tol: Tol,
) -> Result<(), StepImportError> {
    let target = Target::build(solid)?;
    // Root: the first non-self-loop edge's start vertex (so the seed
    // grows by `mev`), else the first edge's vertex (an all-self-loop
    // shell opens with `mef Lone`).
    let root = {
        let mut root = None;
        for (&_e, &(fwd, _)) in &target.edge_uses {
            let u = &target.uses[fwd];
            let spec = &solid.edges[&u.edge];
            if spec.start != spec.end {
                root = Some(spec.start);
                break;
            }
        }
        match root {
            Some(v) => v,
            None => {
                let (&first, _) =
                    target
                        .edge_uses
                        .iter()
                        .next()
                        .ok_or(StepImportError::Topology {
                            id: solid.id,
                            what: "a shell with no edges",
                        })?;
                solid.edges[&first].start
            }
        }
    };
    let seed = body
        .mvfs(solid.vertices[&root])
        .map_err(|source| StepImportError::Assembly {
            id: solid.id,
            source,
        })?;
    let mut vstate = BTreeMap::new();
    for &v in solid.vertices.keys() {
        vstate.insert(v, VState::Unbuilt);
    }
    vstate.insert(
        root,
        VState::Lone {
            vertex: seed.vertex,
            r#loop: seed.r#loop,
        },
    );
    let use_count = target.uses.len();
    let mut builder = Builder {
        body: &mut *body,
        target,
        solid,
        use_he: vec![None; use_count],
        vstate,
    };
    builder.run(tol)?;
    builder.verify(solid.id)?;
    let Builder {
        target,
        use_he: realized,
        vstate,
        ..
    } = builder;
    // Every vertex must have ended `Built` (a leftover `Lone` would be
    // an empty ring surviving to rest — assembly incomplete).
    for state in vstate.values() {
        if !matches!(state, VState::Built(_)) {
            return Err(StepImportError::Topology {
                id: solid.id,
                what: "internal: a vertex survived assembly without a built use",
            });
        }
    }
    let use_he: Vec<HalfEdgeKey> = realized
        .iter()
        .map(|he| {
            he.ok_or(StepImportError::Topology {
                id: solid.id,
                what: "internal: an unrealized use survived assembly",
            })
        })
        .collect::<Result<_, _>>()?;
    let assembled = Assembled { target, use_he };
    adopt::finish(body, solid, &assembled, tol)
}

/// One `MANIFOLD_SOLID_BREP` assembled into a body of its OWN, so the
/// shared at-rest gate can be asked about that solid alone: the
/// whole-body invariants the gate checks include summed ones (the +V
/// flux sum over every shell), which cannot see a single inside-out
/// solid whose neighbours cancel it. Same assembly, same pcurve mint,
/// same geometry — only the arena's contents differ.
pub(crate) fn build_one_solid(solid: &SolidSpec, tol: Tol) -> Result<Body<f64>, StepImportError> {
    build(std::slice::from_ref(solid), tol)
}

/// The assembly proper: every solid in `solids` into one arena, then
/// one pcurve mint over the whole body.
///
/// One door reaches it today — [`build_one_solid`], always with a
/// one-element slice. The multi-solid door retired with M8 instancing,
/// which builds each instance in a body of its own and grafts the
/// copies (`lib.rs`), so no caller assembles two specs into one arena.
fn build(solids: &[SolidSpec], tol: Tol) -> Result<Body<f64>, StepImportError> {
    let mut body = Body::new();
    for solid in solids {
        assemble_solid(&mut body, solid, tol)?;
    }
    topo::mint_pcurves(&mut body, tol).map_err(|source| StepImportError::Pcurves { source })?;
    Ok(body)
}
