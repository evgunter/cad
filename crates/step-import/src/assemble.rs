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
//! Both scaffold mints hold one rule: a strut whose two endpoints
//! coincide has no direction to certify, so neither site mints one.
//! Planting skips a coincident anchor and refuses when the scan
//! exhausts them; the self-loop's strut has a single candidate
//! endpoint and refuses as soon as it turns out to be the vertex
//! again (`coincide`, `strut_endpoint`).
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

/// The absolute offset, along +x, that mints a temporary strut's far
/// endpoint beside an existing vertex ([`strut_endpoint`]).
const STRUT_OFFSET: f64 = 1.0;

/// Two scaffold endpoints are the same point — the state no strut may
/// be minted in, at either mint site.
///
/// A scaffold strut is a `mev_line` chord between two points, and a
/// chord whose endpoints coincide has no direction to certify
/// against. The pair is therefore checked before the mint rather than
/// left to the operator's certificate, which refuses in the kernel's
/// vocabulary about carriers where the reader owes one about the
/// file.
///
/// VALUE equality on all three components, because the carrier is
/// built from the difference and the question is exactly whether
/// that difference is zero. Three consequences, each with where it
/// is decided:
///
/// * `0.0` and `-0.0` are one point. A parsed coordinate is never
///   `-0.0` — [`crate::signed_zero`] owns that argument and
///   `entities::as_real` flushes every literal at the read — so this
///   is not the case the change is FOR; it is what the question
///   being asked implies, and a predicate that answered it the other
///   way would be answering a different question.
/// * A coordinate that is not a number cannot arrive: a Part 21 real
///   token is `+`, `-` or a digit followed by digits, `.` and an
///   exponent (`Parser::value`), so `NaN` has no spelling to be
///   lexed from. `==`'s one gap is therefore closed upstream rather
///   than here.
/// * An infinity DOES have a spelling — `1E999` parses to one, and
///   nothing on the read path rejects it — and two equal infinities
///   compare equal here, which is the answer that makes
///   [`strut_endpoint`] withhold a strut at one.
///
/// **Exact, not toleranced**, though `Tol` is in scope at both call
/// sites. The certificate a scaffold chord has to pass refuses a
/// zero-span parameter interval, not a short one; asking `tol` here
/// would refuse struts the operators accept, and would answer a
/// different question (is this scaffold USEFUL) than the one the two
/// sites share (can this chord be certified at all).
fn coincide(a: geom_core::Point3<f64>, b: geom_core::Point3<f64>) -> bool {
    a.x == b.x && a.y == b.y && a.z == b.z
}

/// The far endpoint of the temporary strut minted beside `p`, or
/// `None` where that point would BE `p`.
///
/// The offset is absolute, so it is lost wherever `p.x`'s own `f64`
/// spacing swallows it. **Which coordinates those are is not a
/// magnitude test**, and that is why this asks the sum rather than
/// `p.x`:
///
/// * `p.x >= 2^54` or `p.x <= -2^54` — the spacing is at least four
///   times the offset, which always rounds away. Lost everywhere.
/// * `2^53 <= p.x < 2^54`, and `-2^54 < p.x <= -2^53` — the offset is
///   exactly HALF a step, so round-half-to-even decides, and it
///   decides on the value's own mantissa parity: `2^53` loses it,
///   `2^53 + 2` keeps it. Lost at about half the values in each band,
///   interleaved.
/// * `|p.x| < 2^53` — the spacing is at most the offset. Never lost.
///
/// The two bands are not mirror images, because a POSITIVE offset
/// added to a negative coordinate moves toward zero, into the next
/// finer binade: `-2^53 + 1.0` is exactly `-(2^53 - 1)` while
/// `2^53 + 1.0` is `2^53`.
///
/// The reader takes untrusted coordinates, so the sum is checked
/// rather than assumed: the caller refuses, which is the same
/// standing [`Builder::plant`] gives the same state.
fn strut_endpoint(p: geom_core::Point3<f64>) -> Option<geom_core::Point3<f64>> {
    let offset = geom_core::Point3::new(p.x + STRUT_OFFSET, p.y, p.z);
    (!coincide(offset, p)).then_some(offset)
}

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
        let offset = strut_endpoint(p).ok_or(StepImportError::Topology {
            id: edge_id,
            what: "a self-loop's scaffold strut would be zero-length (the strut's \
                   offset rounds away at this vertex's x coordinate)",
        })?;
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
        // start position differs from `p` ([`coincide`]).
        for i in 0..self.use_he.len() {
            let Some(he) = self.use_he[i] else { continue };
            let anchor_pos = self.solid.vertices[&self.target.start_vertex[i]];
            if coincide(anchor_pos, p) {
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

/// The assembly proper: one `MANIFOLD_SOLID_BREP` into a body of its
/// OWN (phase A + verification + adoption), then one pcurve mint over
/// that body.
///
/// A body per solid is what lets the shared at-rest gate be asked
/// about that solid alone: the whole-body invariants the gate checks
/// include summed ones (the +V flux sum over every shell), which
/// cannot see a single inside-out solid whose neighbours cancel it.
/// Nothing assembles two specs into one arena — each instance is
/// built into a body of its own and the copies are grafted (`lib.rs`).
pub(crate) fn build_one_solid(solid: &SolidSpec, tol: Tol) -> Result<Body<f64>, StepImportError> {
    let mut body = Body::new();
    // The import IS a door: it runs the operator sequence a foreign
    // file describes, and D1's whole-body tier-1 postcondition is paid
    // once over the finished solid rather than once per operator
    // (`topo::surgery`). A body of n faces costs one sweep here where
    // it used to cost one per mint. The guard owns the borrow, so a
    // refusal part-way closes the scope by dropping it.
    let mut door = body.begin_surgery();
    assemble_solid(&mut door, solid, tol)?;
    topo::mint_pcurves(&mut door, tol).map_err(|source| StepImportError::Pcurves { source })?;
    door.sweep_and_close();
    Ok(body)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    use geom_core::Point3;

    /// The scaffold rule where an absolute offset stops being one.
    /// `strut_endpoint`'s docs say which coordinates those are and
    /// why it is not a magnitude test; this pins the answer at every
    /// boundary that argument turns on, so a guard rewritten as
    /// `|p.x| >= 2^53` — the claim this row's own prose carried
    /// before it was checked — reddens at `-2^53`.
    ///
    /// Both directions are covered: a guard deleted reds on the
    /// withheld rows, a guard widened until it refuses ordinary
    /// coordinates reds on the minted ones.
    #[test]
    fn a_scaffold_strut_is_withheld_where_its_offset_vanishes() {
        let ordinary = Point3::new(3.0, -1.0, 0.5);
        let minted = strut_endpoint(ordinary).expect("an ordinary coordinate mints a strut");
        assert_eq!(minted.x, 4.0, "the offset is absolute and along +x");
        assert_eq!((minted.y, minted.z), (ordinary.y, ordinary.z));
        assert!(!coincide(minted, ordinary));

        // `true` means the strut is MINTED at that coordinate. The
        // two tie bands are sampled on both parities, and each band's
        // edge is paired with its neighbour on the other side, so a
        // guard rewritten as a magnitude test reddens here whichever
        // magnitude it picks.
        for (x, mints) in [
            // Below the first tie band the spacing is at most the
            // offset, both ways.
            (9_007_199_254_740_991.0_f64, true), //  2^53 - 1
            (-9_007_199_254_740_991.0, true),    // -2^53 + 1
            // `[2^53, 2^54)`: the offset is half a step and
            // round-half-to-even decides on mantissa parity.
            (9_007_199_254_740_992.0, false), // 2^53
            (9_007_199_254_740_994.0, true),  // 2^53 + 2
            (9_007_199_254_740_996.0, false), // 2^53 + 4
            // `(-2^54, -2^53]`: the SAME band on the other side is
            // shifted by one representable step, because a positive
            // offset moves a negative coordinate toward zero into the
            // next finer binade. `-2^53` mints where `2^53` does not.
            (-9_007_199_254_740_992.0, true),  // -2^53
            (-9_007_199_254_740_994.0, true),  // -2^53 - 2
            (-9_007_199_254_740_996.0, false), // -2^53 - 4
            // From `2^54` out the spacing is four times the offset or
            // more, so it always rounds away — no parity left.
            (18_014_398_509_481_984.0, false),  //  2^54
            (18_014_398_509_481_988.0, false),  //  2^54 + 4
            (-18_014_398_509_481_984.0, false), // -2^54
            (-18_014_398_509_481_988.0, false), // -2^54 - 4
            (f64::MAX, false),
            (f64::MIN, false),
            // The overflow a Part 21 real literal can reach.
            (f64::INFINITY, false),
            (f64::NEG_INFINITY, false),
        ] {
            let p = Point3::new(x, 0.0, 0.0);
            assert_eq!(
                strut_endpoint(p).is_some(),
                mints,
                "a strut at {x} must {}",
                if mints { "be minted" } else { "be withheld" }
            );
        }
    }

    /// Coincidence is the question "is this chord zero-length", asked
    /// on all three components: a difference in any one of them is a
    /// strut with a direction, and the two spellings of zero are one
    /// point.
    #[test]
    fn scaffold_coincidence_is_all_three_components() {
        let p = Point3::new(1.0, 2.0, 3.0);
        assert!(coincide(p, p));
        assert!(!coincide(p, Point3::new(1.0, 2.0, 3.000_000_000_000_001)));
        assert!(!coincide(p, Point3::new(1.0, 2.000_000_000_000_001, 3.0)));
        assert!(!coincide(p, Point3::new(1.000_000_000_000_000_2, 2.0, 3.0)));
        assert!(
            coincide(Point3::new(0.0, 0.0, 0.0), Point3::new(-0.0, -0.0, -0.0)),
            "a chord between the two spellings of the origin is still zero-length"
        );
    }

    /// **The site, not the rule.** Drives `insert_selfloop_tied` with
    /// a start vertex whose coordinate swallows the strut offset and
    /// asserts the door refuses `Topology` naming the case — rather
    /// than minting a chord `mev_line` then rejects as
    /// `Assembly { source }`, which blames the file for the reader's
    /// own arithmetic.
    ///
    /// The builder is assembled here rather than driven from a file
    /// because **no fixture in the suite reaches this branch**: the
    /// arm needs a self-loop whose two uses share their only built
    /// anchor AND whose target fan order is the reversed one, and the
    /// corpus takes the plain `mef` order every time (this unit's PR
    /// carries the instrumented count). So the state the arm exists
    /// for is set up directly. Everything it reads before the refusal
    /// is real — `sigma`, `use_he` and the solid's own vertex table —
    /// and it reads the body only after, which is why a body holding
    /// one unrelated half-edge is enough.
    ///
    /// The ordinary-coordinate control is the half that makes this a
    /// row about the guard rather than about the arm: the same call
    /// at `x = 0` must get PAST the guard, so a guard that refuses
    /// everything reds here.
    #[test]
    fn the_tied_selfloop_door_refuses_a_strut_it_cannot_mint() {
        use crate::entities::EdgeSpec;
        use geom::Curve3;
        use geom_core::{Point3, Tol, Vec3};

        const SELF_LOOP: u64 = 10;
        const NEIGHBOUR: u64 = 20;
        const VERTEX: u64 = 100;
        // `2^53`: the offset is exactly half a step there and the
        // value's mantissa parity loses the tie, so the strut would
        // land back on its own start.
        const SWALLOWED: f64 = 9_007_199_254_740_992.0;

        let tol = Tol::witness();
        let solid_at = |x: f64| SolidSpec {
            id: 1,
            faces: Vec::new(),
            edges: BTreeMap::from([(
                SELF_LOOP,
                EdgeSpec {
                    start: VERTEX,
                    end: VERTEX,
                    carrier: Curve3::Line {
                        origin: Point3::new(x, 0.0, 0.0),
                        dir: Vec3::new(1.0, 0.0, 0.0),
                    },
                    t0: 0.0,
                    t1: 1.0,
                    reversed: false,
                },
            )]),
            vertices: BTreeMap::from([(VERTEX, Point3::new(x, 0.0, 0.0))]),
            band_seams: std::collections::BTreeSet::new(),
        };
        // Use 0 is the self-loop's forward use, 1 its reversed use, 2
        // the built neighbour. `sigma(0) = next[mate[0]] = next[1] =
        // 2`, which is built — so the target fan order is the
        // reversed one and the arm takes the strut branch.
        let target = || Target {
            uses: vec![
                Use {
                    edge: SELF_LOOP,
                    forward: true,
                },
                Use {
                    edge: SELF_LOOP,
                    forward: false,
                },
                Use {
                    edge: NEIGHBOUR,
                    forward: true,
                },
            ],
            loops: Vec::new(),
            face_loops: Vec::new(),
            next: vec![0, 2, 0],
            mate: vec![1, 0, 2],
            start_vertex: vec![VERTEX, VERTEX, VERTEX],
            edge_uses: BTreeMap::from([(SELF_LOOP, (0, 1))]),
        };
        // A body with one real half-edge, so `use_he` can hold a key
        // the fan walk reads as built.
        let anchor = |body: &mut Body<f64>| {
            let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0)).expect("mvfs");
            body.mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(1.0, 0.0, 0.0),
                tol,
            )
            .expect("a spur to anchor on")
            .he_plus
        };

        let mut body = Body::new();
        let s = anchor(&mut body);
        let swallowed = solid_at(SWALLOWED);
        let refusal = Builder {
            body: &mut body,
            target: target(),
            solid: &swallowed,
            use_he: vec![None, None, Some(s)],
            vstate: BTreeMap::new(),
        }
        .insert_selfloop_tied(SELF_LOOP, 0, 1, s, tol);
        match refusal {
            Err(StepImportError::Topology { id, what }) => {
                assert_eq!(id, SELF_LOOP, "the refusal names the edge it is about");
                assert!(
                    what.contains("scaffold strut"),
                    "the refusal names the scaffold strut, not a carrier: {what}"
                );
            }
            other => panic!("a strut that cannot be minted must refuse Topology, got {other:?}"),
        }

        // The control: the same arm at an ordinary coordinate gets
        // past the guard. What it meets afterwards is the operators'
        // own business — this body is not a real self-loop site — so
        // the assertion is only that the guard let it through.
        let mut body = Body::new();
        let s = anchor(&mut body);
        let ordinary = solid_at(0.0);
        let past = Builder {
            body: &mut body,
            target: target(),
            solid: &ordinary,
            use_he: vec![None, None, Some(s)],
            vstate: BTreeMap::new(),
        }
        .insert_selfloop_tied(SELF_LOOP, 0, 1, s, tol);
        assert!(
            !matches!(past, Err(StepImportError::Topology { .. })),
            "an ordinary coordinate must reach the mint, not the guard's refusal: {past:?}"
        );
    }
}
