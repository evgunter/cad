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
//! **Coincidence discipline (the F6/round-8 ladder; N6 retirement,
//! M4 PR 5)**: two adjacent faces merge iff their surfaces are the
//! *same key* (structural), the *same [`crate::GeomSource`]*
//! (declared — shared recipe source, syntactic identity), or a
//! *declared face pair* of the call
//! ([`Body::merge_coplanar_faces_declared`] — recipe intent, verified
//! not trusted). The M3-era bit-identical-description rung is RETIRED:
//! a pair that is merely **numerically or bitwise** value-equal — same
//! plane, independent sources — stays unmerged **by design** (the
//! ladder's ratified rung (b): coincidence is never inferred from
//! values; the boolean's `NonMaximalFaces` gate agrees — the ladder is
//! consistent end to end). Curved same-key neighbors (a revolve's
//! shared-key wall wedges) also stay unmerged: face maximality on
//! curved surfaces is M5's.
//!
//! Serves the ch. 15 boolean pipeline's operand precondition and
//! output stage (M3 PRs 4–5).

use std::collections::BTreeMap;

use geom_core::{Band, BandError, Decide, Indeterminate};
use geom_surfaces::Surface;
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::boolean::{PlaneDesc, PlaneEqError, PlaneIdentity, PlaneRelation, oriented_plane_eq};
use crate::entity::{EdgeKey, FaceKey, LoopKey};
use crate::euler::EulerOpError;
use crate::geometry::SurfaceKey;
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
    /// A declared surface pair references a key that does not
    /// resolve, or a non-plane surface (M4 PR 5) — a caller bug,
    /// refused up front.
    InvalidDeclaration {
        /// The offending surface key.
        surface: SurfaceKey,
        /// What was wrong.
        what: &'static str,
    },
    /// A declared face pair's planes are DEFINITELY distinct — the
    /// declaration contradicts the geometry; refused loudly, never
    /// glued (M4 PR 5; `plane_eq` rung 2's verification direction).
    DeclarationContradicted {
        /// The contradicting predicate's diagnostics.
        diag: Indeterminate,
    },
    /// A declared face pair meets with OPPOSITE orientations at a
    /// shared edge — no valid closed solid merges such a pair; the
    /// declaration cannot be honored here.
    DeclaredOppositeOrientation {
        /// The pair's first face (arena order at the meeting edge).
        f1: FaceKey,
        /// The second face.
        f2: FaceKey,
    },
    /// A plane-identity margin escalated while verifying a declared
    /// pair (in-band sliver) — typed, never guessed.
    Escalated {
        /// The predicate's diagnostics.
        diag: Indeterminate,
    },
    /// The run's tolerance cannot form a valid band (needed only when
    /// declared pairs are present).
    Band {
        /// The band construction failure.
        error: BandError,
    },
}

impl core::fmt::Display for MergeCoplanarError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InputNotClosed { errors } => {
                write!(
                    f,
                    "merge_coplanar_faces: input is not tier-2 ({} errors)",
                    errors.len()
                )
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
            Self::InvalidDeclaration { surface, what } => write!(
                f,
                "merge_coplanar_faces: invalid declared pair at surface {surface:?}: {what}"
            ),
            Self::DeclarationContradicted { diag } => write!(
                f,
                "merge_coplanar_faces: declared coincidence contradicts the geometry ({diag}) \
                 — fix the declaration or the geometry, the op never glues a lie"
            ),
            Self::DeclaredOppositeOrientation { f1, f2 } => write!(
                f,
                "merge_coplanar_faces: declared pair ({f1:?}, {f2:?}) meets with opposite \
                 orientations — unmergeable in a closed solid"
            ),
            Self::Escalated { diag } => write!(
                f,
                "merge_coplanar_faces: plane-identity margin escalated verifying a declared \
                 pair ({diag})"
            ),
            Self::Band { error } => write!(f, "merge_coplanar_faces: {error}"),
        }
    }
}

impl std::error::Error for MergeCoplanarError {}

/// A non-empty declared-pair context: the surface equivalence plus
/// the band its verification decisions run in.
struct DeclaredCtx {
    eq: DeclaredSurfaceEq,
    band: Band,
}

/// The declared surface-key equivalence (M4 PR 5): union-find classes
/// over the declared face pairs' surface keys. Fragments of a face
/// inherit its surface key (`FaceSurface::Inherit`), so surface-level
/// equivalence covers every fragment of a declared pair without
/// key-chasing.
#[derive(Debug, Default)]
struct DeclaredSurfaceEq {
    parent: BTreeMap<SurfaceKey, SurfaceKey>,
}

impl DeclaredSurfaceEq {
    fn find(&self, mut k: SurfaceKey) -> SurfaceKey {
        while let Some(&p) = self.parent.get(&k) {
            if p == k {
                break;
            }
            k = p;
        }
        k
    }

    fn union(&mut self, a: SurfaceKey, b: SurfaceKey) {
        let (ra, rb) = (self.find(a), self.find(b));
        self.parent.entry(ra).or_insert(ra);
        self.parent.entry(rb).or_insert(rb);
        if ra != rb {
            self.parent.insert(rb, ra);
        }
    }

    fn same(&self, a: SurfaceKey, b: SurfaceKey) -> bool {
        if self.parent.is_empty() {
            return false;
        }
        self.find(a) == self.find(b)
    }

    fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}

impl<T: Decide> Body<T> {
    /// Merges every maximal run of adjacent same-plane faces (module
    /// docs: structural or declared coincidence only), killing shared
    /// edges (`kef`; intra-face duplicates via `kemr`, whose new ring
    /// takes the plus half's side — a **provisional designation, not
    /// truth**: which loop is "the" ring is a containment question this
    /// op does not ask, so a region-sensitive consumer must re-home the
    /// ring via containment — PR 2+ machinery, `ring_move` the
    /// mechanism. Until then the convention is simply not detected
    /// wrong) and re-homing absorbed faces' rings onto the survivor.
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
    pub fn merge_coplanar_faces(&mut self) -> Result<MergeCoplanarOutcome, MergeCoplanarError> {
        self.merge_coplanar_faces_declared(&[])
    }

    /// [`Body::merge_coplanar_faces`] with declared coincident
    /// SURFACE pairs (M4 PR 5, F5): each pair's surfaces are declared
    /// to describe one plane by recipe intent — they become
    /// equivalent for the adjacency test (fragments inherit surface
    /// keys, so every fragment of a declared face is covered),
    /// verified at each meeting edge through `plane_eq`'s declared
    /// rung (contradiction refuses typed). Same-source surfaces (N6)
    /// glue with zero declarations — the retired bit rung's
    /// replacement.
    ///
    /// A declared pair whose surfaces never meet at an edge licenses
    /// nothing and is a no-op (the equivalence is consulted only
    /// across shared edges); a pair whose keys do not resolve is a
    /// typed refusal.
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError`], the body untouched in every case.
    pub fn merge_coplanar_faces_declared(
        &mut self,
        declared: &[(SurfaceKey, SurfaceKey)],
    ) -> Result<MergeCoplanarOutcome, MergeCoplanarError> {
        // ---- Gate: tier-valid before. ----
        if let Err(errors) = validate_closed(self) {
            return Err(MergeCoplanarError::InputNotClosed { errors });
        }
        // ---- Declared pairs: validate, then class the surfaces. ----
        let planar = |body: &Self, k: SurfaceKey| -> Result<(), MergeCoplanarError> {
            match body.get_surface(k) {
                Some(Surface::Plane { .. }) => Ok(()),
                Some(_) => Err(MergeCoplanarError::InvalidDeclaration {
                    surface: k,
                    what: "declared surface is not a plane",
                }),
                None => Err(MergeCoplanarError::InvalidDeclaration {
                    surface: k,
                    what: "declared surface key does not resolve",
                }),
            }
        };
        let mut eq = DeclaredSurfaceEq::default();
        for &(k1, k2) in declared {
            planar(self, k1)?;
            planar(self, k2)?;
            eq.union(k1, k2);
        }
        let declared_ctx = if eq.is_empty() {
            None
        } else {
            Some(DeclaredCtx {
                eq,
                band: Band::linear().map_err(|error| MergeCoplanarError::Band { error })?,
            })
        };
        // ---- Mergeable adjacency (read-only, edge-arena order). ----
        let mut neighbors: SecondaryMap<FaceKey, Vec<FaceKey>> = SecondaryMap::new();
        let mut any = false;
        for (edge_key, edge) in self.edges() {
            let Some((fp, fm)) = self.edge_faces(edge.he_plus, edge.he_minus) else {
                continue; // unreachable on tier-2 input
            };
            if fp != fm && self.planes_declared_equal(fp, fm, edge_key, declared_ctx.as_ref())? {
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

    /// The F6 ladder's merge test (M4 PR 5, the N6 retirement): same
    /// surface key (structural), same [`crate::GeomSource`] including
    /// orient (declared — shared recipe source, syntactic identity,
    /// zero numerics), or the pair's surfaces are declared-equivalent
    /// by this call's face pairs (verified through `plane_eq`'s
    /// declared rung at the meeting edge; contradiction refuses).
    ///
    /// The M3-era rung — bit-identical nine-scalar descriptions — is
    /// RETIRED from production: equal bits without shared source stay
    /// unglued (the ladder's ratified rung (b)). The bit comparison
    /// survives as the debug assertion that same-source records agree
    /// with the bits. *No banded comparison certifies coincidence
    /// here by design* — the declared-pair verification only checks
    /// the declaration is not a lie; the INTENT does the gluing.
    ///
    /// Non-plane surfaces never merge, same-key included (curved
    /// maximality is M5's).
    fn planes_declared_equal(
        &self,
        f1: FaceKey,
        f2: FaceKey,
        edge: EdgeKey,
        declared: Option<&DeclaredCtx>,
    ) -> Result<bool, MergeCoplanarError> {
        let (Some(k1), Some(k2)) = (
            self.get_face(f1).map(|f| f.surface),
            self.get_face(f2).map(|f| f.surface),
        ) else {
            return Ok(false);
        };
        let (Some(s1), Some(s2)) = (self.get_surface(k1), self.get_surface(k2)) else {
            return Ok(false);
        };
        let (
            Surface::Plane {
                origin: o1,
                normal: n1,
                u_ref: u1,
            },
            Surface::Plane {
                origin: o2,
                normal: n2,
                u_ref: u2,
            },
        ) = (*s1, *s2)
        else {
            return Ok(false);
        };
        if k1 == k2 {
            return Ok(true); // structural (and planar, checked above)
        }
        // Declared rung, N6 form: same recipe source INCLUDING orient
        // — a provenance lookup, no numerics. The debug assertion is
        // DESIGN.md's "records agree with bits".
        if let (Some(g1), Some(g2)) = (self.surface_source(k1), self.surface_source(k2))
            && g1 == g2
        {
            #[cfg(debug_assertions)]
            debug_assert!(
                crate::source::plane_bits_agree(o1, n1, o2, n2, false)
                    && crate::source::vec3_bits_agree(u1, u2),
                "N6 theorem violated: same-source surface descriptions disagree bitwise \
                 (kernel bug: a source survived a geometric rewrite)"
            );
            return Ok(true);
        }
        #[cfg(not(debug_assertions))]
        let _ = (u1, u2);
        // Declared face pairs (this call's recipe intent), verified.
        if let Some(ctx) = declared
            && ctx.eq.same(k1, k2)
        {
            let band = ctx.band;
            let arm = self.edge_chord_len(edge).unwrap_or_else(T::one);
            let id = PlaneIdentity {
                s1: None,
                s2: None,
                declared: true,
            };
            let p1 = PlaneDesc {
                origin: o1,
                normal: n1,
            };
            let p2 = PlaneDesc {
                origin: o2,
                normal: n2,
            };
            return match oriented_plane_eq(&p1, &p2, id, arm, band) {
                Ok(PlaneRelation::SameOriented) => Ok(true),
                Ok(PlaneRelation::SameOpposite) => {
                    Err(MergeCoplanarError::DeclaredOppositeOrientation { f1, f2 })
                }
                // Unreachable through the declared rung; kept typed.
                Ok(PlaneRelation::Distinct) => Ok(false),
                Err(PlaneEqError::Contradicted(diag)) => {
                    Err(MergeCoplanarError::DeclarationContradicted { diag })
                }
                Err(PlaneEqError::Escalated(diag) | PlaneEqError::Undeclared(diag)) => {
                    Err(MergeCoplanarError::Escalated { diag })
                }
            };
        }
        Ok(false)
    }

    /// The chord length between an edge's endpoints — the lever arm
    /// metering the declared-pair verification at that edge.
    fn edge_chord_len(&self, edge: EdgeKey) -> Option<T> {
        let e = self.get_edge(edge)?;
        let pa = *self.get_point(self.get_vertex(self.get_half_edge(e.he_plus)?.start)?.point)?;
        let pb = *self.get_point(
            self.get_vertex(self.get_half_edge(e.he_minus)?.start)?
                .point,
        )?;
        Some((pb - pa).norm())
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
                    found = Some((
                        edge_key,
                        edge.he_plus,
                        edge.he_minus,
                        hp.parent_loop == hm.parent_loop,
                    ));
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
