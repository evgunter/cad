//! `merge_coplanar_faces` — explicit opt-in maximal-faces normalization
//! (M3 PR 1, fork F7): merge maximal runs of adjacent faces whose
//! planes are **structurally or declaredly** the same, killing the
//! shared edges and re-homing rings.
//!
//! Ch. 15's booleans require maximal-faced operands (no two adjacent
//! coplanar faces), and the seam zip *manufactures* coplanar pairs by
//! construction — so F7 ratified a fail-loud precondition on the
//! boolean side plus this **public, explicit** normalization op (the
//! M2 no-automatic-face-merging ratification: merging is never
//! silent; boolean outputs run this op as a documented final stage of
//! their own contract).
//!
//! **Coincidence discipline (the F6/round-8 ladder, applied)**: two
//! adjacent faces merge iff their surfaces are the *same key*
//! (structural) or *bit-identical `Plane` descriptions* (declared —
//! equality of every field by exact scalar `==`, no tolerance
//! anywhere). A pair that is merely **numerically** coplanar — same
//! plane up to ε, different descriptions — is out of scope **by
//! design**: coincidence is never inferred from values; such a pair
//! stays unmerged (and PR 4's `NonMaximalFaces` gate will not see it
//! as coplanar either — the ladder is consistent end to end). Curved
//! same-key neighbors (a revolve's shared-key wall wedges) also stay
//! unmerged: face maximality on curved surfaces is M5's.
//!
//! Serves the ch. 15 boolean pipeline's operand precondition and
//! output stage (M3 PRs 4–5).

use geom_core::Decide;
use geom_surfaces::Surface;
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopKey};
use crate::euler::EulerOpError;
use crate::validate::{ValidationError, validate_closed};

/// One merged run: the surviving face and what was consumed into it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergedGroup {
    /// The surviving face (the group's first face in face-arena order).
    pub kept: FaceKey,
    /// The absorbed faces (dead keys), in kill order.
    pub absorbed: Vec<FaceKey>,
    /// The killed shared edges (dead keys), in kill order.
    pub killed_edges: Vec<EdgeKey>,
    /// Rings minted by intra-face shared-edge kills (`kemr` — a merged
    /// run that surrounds a hole grows a genuine ring), in mint order.
    pub rings_made: Vec<LoopKey>,
}

/// The outcome of one [`Body::merge_coplanar_faces`] call.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MergeCoplanarOutcome {
    /// The merged runs, in group order (first face's arena order).
    pub groups: Vec<MergedGroup>,
}

/// A refused [`Body::merge_coplanar_faces`] call (closed enum, D3
/// style); the body is untouched on every variant (the op stages its
/// work on a clone and commits only a tier-2-valid result).
#[derive(Clone, Debug, PartialEq)]
pub enum MergeCoplanarError {
    /// The input is not a tier-2 closed solid — normalization is
    /// defined on at-rest bodies ("tier-valid before").
    InputNotClosed {
        /// The tier-1/2 failures.
        errors: Vec<ValidationError>,
    },
    /// The merged result failed tier 2 ("tier-valid after") — the
    /// configuration is outside what this op can safely merge (e.g. a
    /// kill sequence that would strand scaffolding); refused whole.
    ResultNotClosed {
        /// The tier-1/2 failures of the abandoned attempt.
        errors: Vec<ValidationError>,
    },
    /// A shared edge's two halves lie in **different loops of one
    /// face** after absorption (a ring-adjacent merge shape) — outside
    /// the M2+PR-7 inventory this op handles; refused rather than
    /// guessed at (the kev/ring bookkeeping for it arrives with the
    /// pipeline that produces it, if any does).
    UnsupportedConfiguration {
        /// The edge the op cannot safely kill.
        edge: EdgeKey,
    },
    /// An internal Euler step refused — surfaced typed (unreachable on
    /// tier-2 input in the supported inventory; never a panic, D9).
    Op {
        /// The refusing operator's error.
        error: EulerOpError,
    },
}

impl core::fmt::Display for MergeCoplanarError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InputNotClosed { errors } => {
                write!(f, "merge_coplanar_faces: input is not tier-2 ({} errors)", errors.len())
            }
            Self::ResultNotClosed { errors } => write!(
                f,
                "merge_coplanar_faces: merged result failed tier 2 ({} errors); refused",
                errors.len()
            ),
            Self::UnsupportedConfiguration { edge } => write!(
                f,
                "merge_coplanar_faces: shared edge {edge:?} spans two loops of \
                 one face — unsupported configuration, refused"
            ),
            Self::Op { error } => write!(f, "merge_coplanar_faces: {error}"),
        }
    }
}

impl std::error::Error for MergeCoplanarError {}

impl<T: Decide> Body<T> {
    /// Merges every maximal run of adjacent same-plane faces (module
    /// docs: structural or declared coincidence only), killing shared
    /// edges (`kef`; intra-face duplicates via `kemr`, whose new ring
    /// takes the plus half's side — a designation later re-homed by
    /// the caller through `ring_move` once PR 2's containment exists)
    /// and re-homing absorbed faces' rings onto the survivor.
    ///
    /// **Atomic and deterministic (D9)**: the op stages on a clone —
    /// on any refusal `self` is untouched; on success the staged body
    /// replaces `self` wholesale. All scans are arena-order; the
    /// surviving face of each group is its first face in face-arena
    /// order; edges die in edge-arena order. Composite Euler delta per
    /// group: `f −(n−1)`, `e −k`, plus `r +m` for intra-face kills —
    /// each step is an Euler operator, so tier 1 holds throughout and
    /// χ is conserved at every step.
    ///
    /// A body with nothing to merge returns `Ok` with an empty outcome
    /// and is untouched (deterministic no-op).
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError`], the body untouched in every case.
    pub fn merge_coplanar_faces(
        &mut self,
    ) -> Result<MergeCoplanarOutcome, MergeCoplanarError> {
        // ---- Gate: tier-valid before. ----
        if let Err(errors) = validate_closed(self) {
            return Err(MergeCoplanarError::InputNotClosed { errors });
        }
        // ---- Mergeable adjacency (read-only, edge-arena order). ----
        let mut neighbors: SecondaryMap<FaceKey, Vec<FaceKey>> = SecondaryMap::new();
        let mut any = false;
        for (_, edge) in self.edges() {
            let Some((fp, fm)) = self.edge_faces(edge.he_plus, edge.he_minus) else {
                continue; // unreachable on tier-2 input
            };
            if fp != fm && self.planes_declared_equal(fp, fm) {
                if let Some(entry) = neighbors.entry(fp) {
                    entry.or_default().push(fm);
                }
                if let Some(entry) = neighbors.entry(fm) {
                    entry.or_default().push(fp);
                }
                any = true;
            }
        }
        if !any {
            return Ok(MergeCoplanarOutcome::default());
        }
        // ---- Group labeling (face-arena order seeds, DFS worklist). ----
        let mut label: SecondaryMap<FaceKey, usize> = SecondaryMap::new();
        let mut groups: Vec<Vec<FaceKey>> = Vec::new();
        for (face_key, _) in self.faces() {
            if !neighbors.contains_key(face_key) || label.contains_key(face_key) {
                continue;
            }
            let id = groups.len();
            let mut members = vec![face_key];
            label.insert(face_key, id);
            let mut pending = vec![face_key];
            while let Some(next) = pending.pop() {
                for &n in neighbors.get(next).map(Vec::as_slice).unwrap_or(&[]) {
                    if !label.contains_key(n) {
                        label.insert(n, id);
                        members.push(n);
                        pending.push(n);
                    }
                }
            }
            groups.push(members);
        }
        // ---- Staged surgery on a clone. ----
        let mut work = self.clone();
        let mut outcome = MergeCoplanarOutcome::default();
        for members in groups {
            outcome.groups.push(work.merge_group(&members)?);
        }
        // ---- Gate: tier-valid after; commit. ----
        if let Err(errors) = validate_closed(&work) {
            return Err(MergeCoplanarError::ResultNotClosed { errors });
        }
        *self = work;
        Ok(outcome)
    }

    /// The faces of an edge's two halves (via parent loops), if all
    /// links resolve.
    fn edge_faces(
        &self,
        he_plus: crate::entity::HalfEdgeKey,
        he_minus: crate::entity::HalfEdgeKey,
    ) -> Option<(FaceKey, FaceKey)> {
        let fp = self
            .get_loop(self.get_half_edge(he_plus)?.parent_loop)?
            .face;
        let fm = self
            .get_loop(self.get_half_edge(he_minus)?.parent_loop)?
            .face;
        Some((fp, fm))
    }

    /// The F6 ladder's merge test: same surface key (structural), or
    /// both `Plane` with **bit-identical descriptions** (declared —
    /// arising from shared recipe data). The comparison is the `Debug`
    /// dump of the two `Plane` values — the same bit-faithful channel
    /// the D9 determinism pins compare bodies through (`f64`'s `Debug`
    /// is shortest-roundtrip, hence injective on bits; the scalar
    /// types deliberately expose no generic `==` — the interval scalar
    /// bans `PartialEq` — and *no banded comparison belongs here by
    /// design*: coincidence is never inferred from values). Non-plane
    /// surfaces never merge, same-key included (curved maximality is
    /// M5's).
    fn planes_declared_equal(&self, f1: FaceKey, f2: FaceKey) -> bool {
        let (Some(k1), Some(k2)) = (
            self.get_face(f1).map(|f| f.surface),
            self.get_face(f2).map(|f| f.surface),
        ) else {
            return false;
        };
        let (Some(s1), Some(s2)) = (self.get_surface(k1), self.get_surface(k2)) else {
            return false;
        };
        if !matches!(s1, Surface::Plane { .. }) || !matches!(s2, Surface::Plane { .. }) {
            return false;
        }
        if k1 == k2 {
            return true; // structural (and planar, checked above)
        }
        format!("{s1:?}") == format!("{s2:?}")
    }

    /// Merges one group into its first member (see the public op's
    /// docs for order and refusals). Runs on the staged clone.
    fn merge_group(&mut self, members: &[FaceKey]) -> Result<MergedGroup, MergeCoplanarError> {
        let rep = members[0];
        let mut group = MergedGroup {
            kept: rep,
            absorbed: Vec::new(),
            killed_edges: Vec::new(),
            rings_made: Vec::new(),
        };
        let in_group = |f: FaceKey| members.contains(&f);
        // Absorption: repeatedly kill the first (edge-arena order)
        // edge shared between rep and another group member.
        loop {
            let mut found = None;
            for (edge_key, edge) in self.edges() {
                let Some((fp, fm)) = self.edge_faces(edge.he_plus, edge.he_minus) else {
                    continue;
                };
                if fp == rep && fm != rep && in_group(fm) {
                    found = Some((edge_key, edge.he_minus, fm));
                    break;
                }
                if fm == rep && fp != rep && in_group(fp) {
                    found = Some((edge_key, edge.he_plus, fp));
                    break;
                }
            }
            let Some((edge_key, dying_he, other)) = found else {
                break;
            };
            // Re-home the dying face's rings onto the survivor, then
            // kill the shared edge and the face together (kef).
            let rings = self
                .get_face(other)
                .map(|f| f.rings.clone())
                .unwrap_or_default();
            for ring in rings {
                self.ring_move(ring, rep)
                    .map_err(|error| MergeCoplanarError::Op { error })?;
            }
            self.kef(dying_he)
                .map_err(|error| MergeCoplanarError::Op { error })?;
            group.absorbed.push(other);
            group.killed_edges.push(edge_key);
        }
        // Intra-face duplicates: edges now occurring twice within the
        // survivor's loops.
        loop {
            let mut found = None;
            for (edge_key, edge) in self.edges() {
                let (Some(hp), Some(hm)) = (
                    self.get_half_edge(edge.he_plus),
                    self.get_half_edge(edge.he_minus),
                ) else {
                    continue;
                };
                let Some((fp, fm)) = self.edge_faces(edge.he_plus, edge.he_minus) else {
                    continue;
                };
                if fp == rep && fm == rep {
                    found = Some((edge_key, edge.he_plus, edge.he_minus,
                        hp.parent_loop == hm.parent_loop));
                    break;
                }
            }
            let Some((edge_key, he_plus, he_minus, same_loop)) = found else {
                break;
            };
            if !same_loop {
                return Err(MergeCoplanarError::UnsupportedConfiguration { edge: edge_key });
            }
            let result = self
                .kemr(he_plus, he_minus)
                .map_err(|error| MergeCoplanarError::Op { error })?;
            group.killed_edges.push(edge_key);
            group.rings_made.push(result.ring);
        }
        Ok(group)
    }
}
