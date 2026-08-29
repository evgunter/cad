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
//! consistent end to end). Since M5 PR 9 (C12.5) the hard rungs are
//! **kind-agnostic**: same-key and same-source CURVED neighbors merge
//! through the same never-numeric ladder (the cosurface
//! generalization — the boolean zip's cylinder-wall re-merge is the
//! named consumer); only the per-call declared-PAIR rung stays planar
//! (its verification predicate is `oriented_plane_eq`; the curved
//! counterpart's verification is the contact census's — CONTACT-DESIGN
//! C2's decision procedure and C4's per-class tables — not a
//! merge-local predicate).
//!
//! Serves the ch. 15 boolean pipeline's operand precondition and
//! output stage (M3 PRs 4–5).

use std::collections::BTreeMap;

use geom::Surface;
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Tol};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::boolean::{PlaneDesc, PlaneEqError, PlaneIdentity, PlaneRelation, oriented_plane_eq};
use crate::entity::{EdgeKey, FaceKey, LoopKey, VertexKey};
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
    /// Vertices killed by the straight-seam repair (`kev`), in kill
    /// order — a junction that was interior to one straight carrier
    /// and went with its seam.
    ///
    /// Recorded rather than left implicit because this is the one
    /// thing the op destroys that no other field names: `absorbed`
    /// carries the dead faces and `killed_edges` the dead edges, and
    /// without this a caller reconciling the Euler delta would find a
    /// `v −1` with nothing accounting for it.
    pub killed_vertices: Vec<VertexKey>,
}

/// A declared-licensed merge group that was NOT glued (M4 PR 5): its
/// shape is outside the merge's never-elide Euler inventory. Loud in
/// the record, never a silent drop — and never a partial commit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SkippedMerge {
    /// The group's faces (group order).
    pub faces: Vec<FaceKey>,
    /// Why the glue was not performed — the ACTUAL refusing
    /// diagnostics, rendered (the sub-stage's Euler/tier-2 error),
    /// never a generic label (M4 PR 5 review F3).
    pub reason: String,
}

/// The outcome of one [`Body::merge_coplanar_faces`] call.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MergeCoplanarOutcome {
    /// The merged runs, in group order (first face's arena order).
    pub groups: Vec<MergedGroup>,
    /// Declared-licensed groups skipped as outside the inventory
    /// (empty for the no-declaration entry point).
    pub skipped: Vec<SkippedMerge>,
}

/// Which ladder rung licensed one mergeable adjacency (crate-
/// internal: declared-pair-licensed groups get per-group staging).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MergeRung {
    /// Same surface key or same GeomSource — the ratified hard rungs.
    Hard,
    /// A per-call declared surface pair.
    DeclaredPair,
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
    /// A CURVED cosurface run would close its chart's full period
    /// (M5 PR 9, C12.5): killing the last shared edge leaves either a
    /// ring on a curved face or a full-wrap seam-form loop — shapes
    /// the exact-B-rep props cannot integrate yet, so the run refuses
    /// here and the driver records a loud skip (the operands'
    /// cut-carrying canonical form stays; sub-period re-merges — the
    /// through-cut case — commit normally). FLIP NOTE: the du_of_rims
    /// per-level-sum repair may already make the kept-cut seam form
    /// integrable; re-evaluating this skip belongs to M5 PR 11 (the
    /// tessellation/props unit), which owns the curved-face
    /// quadrature story.
    PeriodClosure {
        /// The shared edge whose kill would close the period.
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
    /// After absorbing a group, the survivor's loops admit no unique
    /// positively-wound outline (zero or several positive windings) —
    /// the outer/ring roles cannot be assigned; refused rather than
    /// guessed (M5 S1 fix pass: the intra-face `kemr`'s provisional
    /// ring designation is now RESOLVED by winding, and shapes the
    /// resolution cannot decide are outside the inventory).
    MergedFaceRoleAmbiguous {
        /// The merged survivor face.
        face: FaceKey,
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
    /// The staged result's pcurve RE-MINT refused (M6-3: an input that
    /// carried stored caches re-mints them on the staged clone before
    /// commit — the `topo::pcurves` module docs' rule for ops that
    /// mutate minted bodies; the merged loops' one-branch walks are
    /// derived fresh, never stitched from the absorbed fragments'
    /// rows). The body is untouched, exactly as on every other
    /// variant.
    Pcurve {
        /// The mint pass's typed refusal.
        source: crate::pcurves::PcurveMintError,
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
            Self::PeriodClosure { edge } => write!(
                f,
                "merge_coplanar_faces: killing shared edge {edge:?} would close the \
                 curved cosurface run's full chart period — outside the merge's \
                 inventory: sub-period re-merges commit, full closures stay in their \
                 cut-carrying canonical form and are recorded as a loud skip"
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
            Self::MergedFaceRoleAmbiguous { face } => write!(
                f,
                "merge_coplanar_faces: merged face {face:?} has no unique positively-wound \
                 outline among its loops — outer/ring roles cannot be assigned; refused"
            ),
            Self::Escalated { diag } => write!(
                f,
                "merge_coplanar_faces: plane-identity margin escalated verifying a declared \
                 pair ({diag})"
            ),
            Self::Band { error } => write!(f, "merge_coplanar_faces: {error}"),
            Self::Pcurve { source } => write!(
                f,
                "merge_coplanar_faces: the staged result's pcurve re-mint refused \
                 ({source}) — the body is untouched"
            ),
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
    /// **The straight-seam repair (`kev`).** An intra-face duplicate is
    /// not always a ring. When the group's shared boundary is exactly
    /// two edges meeting at a valence-2 vertex whose two departures are
    /// collinear and OPPOSED — the vertex is interior to one straight
    /// carrier — the surviving duplicate is a dangling STRUT, and the
    /// op kills it with `kev` instead of minting a ring with `kemr`.
    /// The surgery DELETES BOTH seam edges and the junction vertex: the
    /// `kef` takes one, the `kev` takes the other along with the vertex
    /// it dangles from. Nothing is re-described, because the removed
    /// vertex was interior to a straight locus and the union of the two
    /// collinear pieces is that same locus. The motivating instance is
    /// a full revolve's axis-touching cap (the two seam edges are the
    /// halves of the disc's diameter, the vertex is the pole), but the
    /// licence is collinearity and not provenance
    /// ([`Body::redundant_subdivision_vertex`]'s docs carry the
    /// argument and its residue).
    ///
    /// **Atomic and deterministic (D9)**: the op stages on a clone —
    /// on any refusal `self` is untouched; on success the staged body
    /// replaces `self` wholesale. All scans are arena-order; the
    /// surviving face of each group is its first face in face-arena
    /// order; edges die in edge-arena order. Composite Euler delta per
    /// group: `f −(n−1)`, `e −k`, plus `r +m` for intra-face `kemr`
    /// kills, and `v −1` for each straight-seam `kev` (which is what
    /// keeps χ conserved when a ring is NOT minted: `kemr` trades an
    /// edge for a ring, `kev` trades an edge for a vertex). Each step
    /// is an Euler operator, so tier 1 holds throughout and χ is
    /// conserved at every step.
    ///
    /// A body with nothing to merge returns `Ok` with an empty outcome
    /// and is untouched (deterministic no-op).
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError`], the body untouched in every case.
    pub fn merge_coplanar_faces(
        &mut self,
        tol: Tol,
    ) -> Result<MergeCoplanarOutcome, MergeCoplanarError> {
        self.merge_coplanar_faces_declared(&[], tol)
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
        tol: Tol,
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
                band: Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?,
            })
        };
        // ---- Mergeable adjacency (read-only, edge-arena order). ----
        let mut neighbors: SecondaryMap<FaceKey, Vec<FaceKey>> = SecondaryMap::new();
        let mut declared_faces: std::collections::BTreeSet<FaceKey> =
            std::collections::BTreeSet::new();
        let mut any = false;
        for (edge_key, edge) in self.edges() {
            let Some((fp, fm)) = self.edge_faces(edge.he_plus, edge.he_minus) else {
                continue; // unreachable on tier-2 input
            };
            if fp != fm
                && let Some(rung) =
                    self.planes_declared_equal(fp, fm, edge_key, declared_ctx.as_ref())?
            {
                if let Some(entry) = neighbors.entry(fp) {
                    entry.or_default().push(fm);
                }
                if let Some(entry) = neighbors.entry(fm) {
                    entry.or_default().push(fp);
                }
                if rung == MergeRung::DeclaredPair {
                    declared_faces.insert(fp);
                    declared_faces.insert(fm);
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
        //
        // Structural / same-source groups keep the ratified
        // whole-refusal semantics. DECLARED-licensed groups (any
        // member joined through a declared pair) are attempted each
        // on its own sub-stage: shapes outside the never-elide
        // inventory (e.g. chain-shaped shared seam edges, whose full
        // kill would strand interior vertices) are SKIPPED and
        // recorded in [`MergeCoplanarOutcome::skipped`] — the
        // declaration still served the consuming op's classification;
        // the unglued coplanar adjacency persists exactly as the
        // M3-era output did and refuses loudly downstream if reused
        // undeclared. Never a partial commit: each sub-stage is
        // tier-2-gated before adoption.
        let mut work = self.clone();
        let mut outcome = MergeCoplanarOutcome::default();
        for members in groups {
            let licensed = members.iter().any(|f| declared_faces.contains(f));
            // Curved structural runs (C12.5, M5 PR 9) go through the
            // trial/skip stage too: a run that closes the full period
            // refuses `PeriodClosure` inside its trial and is recorded
            // as a LOUD skip (the operands' canonical cut-carrying
            // form stays), while sub-period re-merges commit. Planar
            // structural groups keep the ratified whole-refusal
            // semantics bit-identically.
            let curved = members.first().is_some_and(|&f| {
                work.get_face(f)
                    .and_then(|fd| work.get_surface(fd.surface))
                    .is_some_and(|s| !matches!(s, Surface::Plane { .. }))
            });
            if !licensed && !curved {
                outcome.groups.push(work.merge_group(&members, tol)?);
                continue;
            }
            let mut trial = work.clone();
            match trial.merge_group(&members, tol) {
                Ok(group) => match validate_closed(&trial) {
                    Ok(()) => {
                        work = trial;
                        outcome.groups.push(group);
                    }
                    Err(errors) => {
                        outcome.skipped.push(SkippedMerge {
                            faces: members,
                            reason: format!(
                                "staged declared merge failed tier 2 ({} finding(s), first: \
                                 {:?}) — shape outside the never-elide inventory",
                                errors.len(),
                                errors.first()
                            ),
                        });
                    }
                },
                Err(e) => {
                    outcome.skipped.push(SkippedMerge {
                        faces: members,
                        reason: format!("declared merge group refused: {e}"),
                    });
                }
            }
        }
        // ---- Gate: tier-valid after; commit. ----
        if let Err(errors) = validate_closed(&work) {
            return Err(MergeCoplanarError::ResultNotClosed { errors });
        }
        // A body that carried stored pcurve caches RE-MINTS them on
        // the staged result before commit (the `topo::pcurves` module
        // docs' rule for ops that mutate minted bodies): the merge
        // rebuilds face loops, and two absorbed fragments' walks were
        // branch-anchored independently — the merged loop's one-branch
        // walk must be derived fresh, never stitched from the
        // fragments' rows. Still on the staged clone, so a mint
        // refusal keeps the untouched-on-error contract.
        //
        // LATENT (named, not reachable by any current path): this is
        // the CLOSED-FORM mint pass, so a FITTED cache (at rest since
        // M6-2) on a merged body would come back as the mint pass's
        // honest-skip — the face legally UNCACHED, its fitted
        // certificate silently dropped. Same root as the banked
        // `PcurveFittedLane` mint-wiring item (M6-3 deviation 7);
        // when the fitted route joins the mint pass, this site
        // inherits the fix for free.
        if !self.pcurves.is_empty() {
            crate::pcurves::mint_pcurves(&mut work, tol)
                .map_err(|source| MergeCoplanarError::Pcurve { source })?;
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
    ///
    /// **Shared sense is a precondition of every rung** (S10). Two
    /// faces on one surface whose `sense` bits differ have OPPOSITE
    /// outward normals: they are the two sides of a slit, not one
    /// region cut in two, and gluing them would mint a face that is
    /// its own reverse. The hard rungs therefore stop firing on such a
    /// pair — they answer "same SURFACE", which is no longer the same
    /// question as "same FACE geometry". They fall through to the
    /// declared rung, where the verified `oriented_plane_eq` verdict
    /// on the two OUTWARD normals is `SameOpposite` and the existing
    /// [`MergeCoplanarError::DeclaredOppositeOrientation`] refusal
    /// fires — a declaration that such a pair is mergeable is exactly
    /// the lie that variant was minted to refuse. An UNDECLARED
    /// opposite-sense pair is not refused, it is simply not a merge
    /// candidate: a slit is legal geometry, and this op has no
    /// standing to reject a body for containing one.
    fn planes_declared_equal(
        &self,
        f1: FaceKey,
        f2: FaceKey,
        edge: EdgeKey,
        declared: Option<&DeclaredCtx>,
    ) -> Result<Option<MergeRung>, MergeCoplanarError> {
        let (Some((k1, sense1, sign1)), Some((k2, sense2, sign2))) = (
            self.get_face(f1)
                .map(|f| (f.surface, f.sense, f.sense_sign::<T>())),
            self.get_face(f2)
                .map(|f| (f.surface, f.sense, f.sense_sign::<T>())),
        ) else {
            return Ok(None);
        };
        let (Some(s1), Some(s2)) = (self.get_surface(k1), self.get_surface(k2)) else {
            return Ok(None);
        };
        // The shared-sense precondition (fn docs): a differing bit
        // makes the two outward normals opposite, so neither hard rung
        // — both of which certify the SURFACE, not the face — may
        // conclude the faces are one region. Falling through leaves
        // the declared rung to refuse loudly if the pair was declared.
        let same_sense = sense1 == sense2;
        // The hard rungs are KIND-AGNOSTIC since M5 PR 9 (C12.5, the
        // cosurface generalization): the same-key and same-source
        // tests never touch a numeric coordinate, so nothing about
        // them was planar — the M3-era "curved same-key neighbors
        // stay unmerged" note flips here, with the same ladder, the
        // same never-numeric rule, and N3 naming semantics unchanged.
        // The named consumer: the boolean zip's re-merge of a
        // cylinder wall split by a through cut.
        if k1 == k2 && same_sense {
            return Ok(Some(MergeRung::Hard)); // structural
        }
        // Declared rung, N6 form: same recipe source INCLUDING orient
        // — a provenance lookup, no numerics (M4's GeomSource
        // retirement consumed, NOT bit_identity). The debug assertion
        // is DESIGN.md's "records agree with bits", stated for the
        // planar kind where the bit predicate exists.
        if same_sense
            && let (Some(g1), Some(g2)) = (self.surface_source(k1), self.surface_source(k2))
            && g1 == g2
        {
            #[cfg(debug_assertions)]
            if let (
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
            ) = (s1.clone(), s2.clone())
            {
                debug_assert!(
                    crate::source::plane_bits_agree(o1, n1, o2, n2, false)
                        && crate::source::vec3_bits_agree(u1, u2),
                    "same-source theorem violated: same-source surface descriptions disagree \
                     bitwise (kernel bug: a source survived a geometric rewrite)"
                );
            }
            return Ok(Some(MergeRung::Hard));
        }
        // The declared-PAIR rung stays planar (its verification is
        // `oriented_plane_eq`; the curved-pair verification predicate
        // is the contact census's — CONTACT-DESIGN C2/C4 — not minted
        // here).
        let (
            Surface::Plane {
                origin: o1,
                normal: n1,
                ..
            },
            Surface::Plane {
                origin: o2,
                normal: n2,
                ..
            },
        ) = (s1.clone(), s2.clone())
        else {
            return Ok(None);
        };
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
            // Outward normals, not chart normals (S10): `PlaneDesc`'s
            // contract, and the reason the SameOpposite arm below can
            // stand as the shared-sense refusal — an opposite-sense
            // pair on one plane lands there by construction.
            let p1 = PlaneDesc {
                origin: o1,
                normal: n1 * sign1,
            };
            let p2 = PlaneDesc {
                origin: o2,
                normal: n2 * sign2,
            };
            return match oriented_plane_eq(&p1, &p2, id, arm, band) {
                Ok(PlaneRelation::SameOriented) => Ok(Some(MergeRung::DeclaredPair)),
                Ok(PlaneRelation::SameOpposite) => {
                    Err(MergeCoplanarError::DeclaredOppositeOrientation { f1, f2 })
                }
                // Unreachable through the declared rung; kept typed.
                Ok(PlaneRelation::Distinct) => Ok(None),
                Err(PlaneEqError::Contradicted(diag)) => {
                    Err(MergeCoplanarError::DeclarationContradicted { diag })
                }
                Err(PlaneEqError::Escalated(diag) | PlaneEqError::Undeclared { diag, .. }) => {
                    Err(MergeCoplanarError::Escalated { diag })
                }
            };
        }
        Ok(None)
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

    /// **Is `v` a redundant subdivision vertex of a straight seam?**
    ///
    /// This is the geometric licence for removing a seam vertex, and it
    /// is deliberately NOT a claim about provenance. A vertex of
    /// valence 2 whose two edges lie on ONE straight carrier, leaving
    /// it in OPPOSITE directions, is interior to a single line
    /// segment: deleting it and merging its two edges replaces two
    /// collinear pieces with their union and **no locus changes**. The
    /// repair is then geometry-preserving by construction, whatever
    /// produced the vertex.
    ///
    /// A full revolve's axis-touching cap is the motivating instance —
    /// its two meridians are the two halves of the disc's DIAMETER,
    /// with the pole interior to it — but nothing here mentions poles
    /// or axes, and it should not: the same fact licenses the same
    /// removal on any straight seam.
    ///
    /// What it refuses is the case the F7 rule exists for. Two
    /// coplanar faces meeting along a bent seam — `merge_skip`'s
    /// L-corner, where two overlapping rectangles meet at a
    /// re-entrant corner — have a valence-2 junction too, and an
    /// earlier form of this trigger that tested only valence was
    /// falsified by exactly that fixture. Perpendicular departures
    /// fail the collinearity decision, so the corner survives and the
    /// merge still refuses, which is the pinned behaviour.
    ///
    /// Both decisions are metered on the shorter incident segment (the
    /// honest lever for an angular quantity read as a length).
    fn redundant_subdivision_vertex(
        &self,
        v: VertexKey,
        band: Band,
    ) -> Result<bool, MergeCoplanarError> {
        let Some(em) = self.get_vertex(v).and_then(|vd| vd.emanating) else {
            return Ok(false);
        };
        let Some(orbit) = self.vertex_orbit(em) else {
            return Ok(false);
        };
        if orbit.len() != 2 {
            return Ok(false);
        }
        let point = |vk: VertexKey| {
            self.get_vertex(vk)
                .and_then(|vd| self.get_point(vd.point).copied())
        };
        let Some(pv) = point(v) else {
            return Ok(false);
        };
        // Each orbit member starts at `v`; its mate starts at the far
        // end. Both carriers must be straight — an arc through `v` is
        // not a subdivision of anything.
        let mut departures = Vec::with_capacity(2);
        for &he in &orbit {
            let Some(hd) = self.get_half_edge(he) else {
                return Ok(false);
            };
            let Some(e) = self.get_edge(hd.edge) else {
                return Ok(false);
            };
            let straight = self
                .get_curve_geom(e.curve)
                .and_then(crate::null::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }));
            if !straight {
                return Ok(false);
            }
            let far = if e.he_plus == he {
                e.he_minus
            } else {
                e.he_plus
            };
            let Some(pf) = self.get_half_edge(far).and_then(|h| point(h.start)) else {
                return Ok(false);
            };
            departures.push(pf - pv);
        }
        let (d1, d2) = (departures[0], departures[1]);
        // The lever is the SHORTER incident segment — an angular
        // quantity is read here as a length, and the shorter arm is the
        // conservative one. `Real::min`, not `<`: the scalar backends
        // order intervals, not values, so a bare comparison on `T` is
        // not available and would not mean this if it were (the S10
        // exact-bit discipline). A degenerate zero-length edge makes
        // the normalization poison, which `decide` escalates typed
        // rather than silently answering.
        let (n1, n2) = (d1.norm(), d2.norm());
        let arm = n1.min(n2);
        let (u1, u2) = (d1 / n1, d2 / n2);
        let escalate = |diag| MergeCoplanarError::Escalated { diag };
        // Collinear: the two departures span no angle.
        if crate::validate::decide(
            "merge_seam_collinear",
            Margin::levered(u1.cross(u2).norm(), arm),
            band,
        )
        .map_err(escalate)?
            != geom_core::Sign::Zero
        {
            return Ok(false);
        }
        // ...and OPPOSED, so `v` is interior to the union rather than a
        // point the seam doubles back from.
        Ok(
            crate::validate::decide("merge_seam_opposed", Margin::levered(u1.dot(u2), arm), band)
                .map_err(escalate)?
                == geom_core::Sign::Negative,
        )
    }

    /// Merges one group into its first member (see the public op's
    /// docs for order and refusals). Runs on the staged clone.
    fn merge_group(
        &mut self,
        members: &[FaceKey],
        tol: Tol,
    ) -> Result<MergedGroup, MergeCoplanarError> {
        let rep = members[0];
        let mut group = MergedGroup {
            kept: rep,
            absorbed: Vec::new(),
            killed_edges: Vec::new(),
            rings_made: Vec::new(),
            killed_vertices: Vec::new(),
        };
        let in_group = |f: FaceKey| members.contains(&f);
        // **The straight-seam junction, decided ONCE** on the group as
        // it arrives — before any mutation, because the answer licenses
        // a different repair below and must not be re-derived from a
        // body that repair is halfway through changing.
        //
        // Verified against this crate's whole merge/boolean fixture
        // corpus before it was wired to anything (the method the
        // reviewers' falsifications earned): it fires on a collinear
        // subdivided seam and on nothing else in the corpus — not on
        // `merge_skip`'s L-corner, not on either review arm's bent
        // chords, not on an inset ring.
        let straight_seam = {
            let band = Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?;
            let shared: Vec<_> = self
                .edges()
                .filter_map(|(k, e)| {
                    let (fp, fm) = self.edge_faces(e.he_plus, e.he_minus)?;
                    (fp != fm && in_group(fp) && in_group(fm)).then_some((k, e.he_plus, e.he_minus))
                })
                .collect();
            let mut verdict = false;
            if shared.len() == 2 {
                let ends =
                    |hp, hm| Some([self.get_half_edge(hp)?.start, self.get_half_edge(hm)?.start]);
                if let (Some(a), Some(b)) = (
                    ends(shared[0].1, shared[0].2),
                    ends(shared[1].1, shared[1].2),
                ) {
                    for v in a.iter().filter(|v| b.contains(v)) {
                        if self.redundant_subdivision_vertex(*v, band)? {
                            verdict = true;
                        }
                    }
                }
            }
            verdict
        };
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
        // survivor's loops. On a PLANAR survivor a same-loop duplicate
        // bounds a genuine hole and `kemr` mints the ring. On a CURVED
        // survivor (C12.5, M5 PR 9) a same-face duplicate means the
        // cosurface run CLOSED THE FULL PERIOD — a shape outside the
        // merge's inventory at M5 (neither the ring form nor the
        // kept-cut seam form is integrable by the exact-B-rep props
        // yet), refused typed here; the driver records it as a LOUD
        // skip for curved structural runs, so sub-period re-merges
        // (the C12.5 through-cut case) proceed and full closures stay
        // unmerged exactly as the operands arrived.
        let survivor_curved = {
            let key = self
                .get_face(rep)
                .map(|f| f.surface)
                .ok_or(MergeCoplanarError::Op {
                    error: EulerOpError::StaleKey {
                        key: crate::entity::EntityId::Face(rep),
                    },
                })?;
            !matches!(self.get_surface(key), Some(Surface::Plane { .. }))
        };
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
            if survivor_curved {
                return Err(MergeCoplanarError::PeriodClosure { edge: edge_key });
            }
            if !same_loop {
                return Err(MergeCoplanarError::UnsupportedConfiguration { edge: edge_key });
            }
            // **A straight seam's surviving duplicate is a STRUT, not a
            // ring.** The absorption above killed one of the two seam
            // edges with `kef`; the other is now a duplicate inside the
            // survivor whose junction end is left with valence 1 — a
            // dangling remnant the merge itself created, enclosing
            // nothing. `kemr` would mint a ring from it and the winding
            // pass would then find no unique positive cycle and refuse
            // `MergedFaceRoleAmbiguous`, which is the dead end this op
            // hits on every revolve cap.
            //
            // `kev` is the op for it — it kills the strut AND the far
            // vertex, leaving the face bounded by its outline alone.
            // The licence is that the removed vertex was interior to
            // one straight carrier, so the union of the two collinear
            // pieces is the same locus: geometry-preserving by
            // construction, which is why this is gated on
            // `straight_seam` and not on the strut's shape alone.
            let tip = if straight_seam {
                [(he_plus, he_minus), (he_minus, he_plus)]
                    .into_iter()
                    // `vertex_orbit` walks the halves STARTING at its
                    // argument's start vertex, so this is the orbit at
                    // the far end of the other half.
                    .find(|&(_, toward)| {
                        self.vertex_orbit(toward)
                            .is_some_and(|orbit| orbit.len() == 1)
                    })
            } else {
                None
            };
            if let Some((from_rim, toward)) = tip {
                let killed = self.get_half_edge(toward).map(|h| h.start);
                self.kev(from_rim)
                    .map_err(|error| MergeCoplanarError::Op { error })?;
                group.killed_edges.push(edge_key);
                group.killed_vertices.extend(killed);
                continue;
            }
            let result = self
                .kemr(he_plus, he_minus)
                .map_err(|error| MergeCoplanarError::Op { error })?;
            group.killed_edges.push(edge_key);
            group.rings_made.push(result.ring);
        }
        // Role normalization (M5 S1 fix pass, review MAJOR-1): the
        // intra-face `kemr` above designates its ring PROVISIONALLY
        // (the plus half's side — the module docs' documented
        // containment question). A group absorbed across TWO disjoint
        // shared runs closes a genuine hole, and the provisional side
        // can put the OUTLINE in the ring slot and the hole in the
        // outer slot — every downstream volume gate is role-invariant,
        // but tessellation is not (the silent-corrupt-export class).
        // Resolve by winding: the outline is the unique cycle winding
        // POSITIVELY around the face's outward normal (the same
        // Newell-functional predicate the boolean join's ring lane
        // decides on); swap it into the outer slot if the kemr put it
        // elsewhere; no unique positive cycle refuses typed.
        if !group.rings_made.is_empty() {
            // The survivor is resolved HERE, where its liveness is
            // proven in this call: the curved-survivor gate above
            // resolved `rep`, and the absorption between only runs
            // `kemr`, which kills no face. The role pass then takes
            // resolved data and performs no lookup of its own.
            let Some(survivor) = self.get_face(rep) else {
                unreachable!(
                    "merge_group: `rep` resolved by the curved-survivor gate above and \
                     the absorption's `kemr` kills no face"
                )
            };
            let survivor = survivor.clone();
            if let Some(i) = self.merged_outline_ring(rep, &survivor, tol)? {
                let Some(fm) = self.faces.get_mut(rep) else {
                    unreachable!(
                        "merge_group: `rep` resolved a few lines above and the winding \
                         pass between is read-only"
                    )
                };
                fm.outer = survivor.rings[i];
                fm.rings[i] = survivor.outer;
            }
        }
        Ok(group)
    }

    /// The signed winding of a cycle loop around `normal`, through the
    /// reified `bool_ring_run_winding` predicate (the plane's Newell
    /// functional — twice the enclosed signed area; the same margin
    /// the boolean join's ring lane decides on). `None` for empty
    /// loops (a lone-vertex ring bounds no area and stays a ring).
    ///
    /// Dimension (audit F4): the Newell area is metered to a LENGTH by
    /// the loop's own perimeter — `2A/P`, the region's mean width. The
    /// derivation, and why this predicate must state it identically at
    /// all three of its sites, is in `boolean::join::ring_run_ccw`.
    ///
    /// Behaviour change riding with that metering (the unit's
    /// deviation 1, SECOND site — the join lane's zero-perimeter note
    /// has the same shape): a cycle whose perimeter is exactly zero —
    /// every vertex coincident — now divides `0/0`, poisons, and
    /// escalates typed, where it previously answered `Some(Zero)` and
    /// let `normalize_merged_roles` read it as "not the positively-wound
    /// cycle". Empty loops still return `None` earlier, so reaching this
    /// needs a real cycle of coincident points. The fail-loud direction
    /// is deliberate: a loop with no extent has no winding to report,
    /// and refusing typed beats handing back a role decision derived
    /// from an area and a perimeter that are both nothing.
    ///
    /// `normal` must be the face's OUTWARD normal (S10): the caller
    /// multiplies the chart normal by `sense_sign` exactly once, and
    /// the Newell sum here is left alone. That sum is built from the
    /// loop's STORED cycle order, which `revert` reverses in the same
    /// breath as it flips the sense bit, so it already changes sign on
    /// its own — threading the sense onto both factors would cancel
    /// and leave the outer/ring roles as wrong as threading neither.
    fn loop_winding(
        &self,
        l: LoopKey,
        normal: geom_core::Vec3<T>,
        band: Band,
    ) -> Result<Option<geom_core::Sign>, MergeCoplanarError> {
        let corrupt = || MergeCoplanarError::Op {
            error: EulerOpError::StaleKey {
                key: crate::entity::EntityId::Loop(l),
            },
        };
        let crate::entity::LoopBoundary::Cycle { first } =
            self.get_loop(l).ok_or_else(corrupt)?.boundary
        else {
            return Ok(None);
        };
        let cycle = self.loop_cycle(first).ok_or_else(corrupt)?;
        // Line-bounded cycles only (the tier-3 check-6 scope): an
        // arc's vertex chord is not the boundary, so a curved-bounded
        // loop's chord winding says nothing about the region — treat
        // it as undecidable (`None`; the caller then refuses rather
        // than guesses if roles hinge on it).
        let all_lines = cycle.iter().all(|&he| {
            self.get_half_edge(he)
                .and_then(|hd| self.get_edge(hd.edge))
                .and_then(|e| self.get_curve_geom(e.curve))
                .and_then(crate::null::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
        });
        if !all_lines {
            return Ok(None);
        }
        let point_of = |he| -> Result<geom_core::Point3<T>, MergeCoplanarError> {
            let v = self.get_half_edge(he).ok_or_else(corrupt)?.start;
            self.get_vertex(v)
                .and_then(|vd| self.get_point(vd.point).copied())
                .ok_or_else(corrupt)
        };
        let p0 = point_of(cycle[0])?;
        let mut newell = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
        // The F4 metering lever: this cycle's perimeter, accumulated
        // with the area (line-bounded only, so chords ARE the boundary).
        let mut perimeter = T::zero();
        let mut prev = p0;
        for &he in &cycle[1..] {
            let p = point_of(he)?;
            newell = newell + (prev - p0).cross(p - p0);
            perimeter = perimeter + (p - prev).norm();
            prev = p;
        }
        perimeter = perimeter + (p0 - prev).norm();
        match crate::validate::decide(
            "bool_ring_run_winding",
            Margin::over_lever(normal.dot(newell), perimeter),
            band,
        ) {
            Ok(sign) => Ok(Some(sign)),
            Err(diag) => Err(MergeCoplanarError::Escalated { diag }),
        }
    }

    /// Which of the merged survivor's rings is its outline, decided by
    /// winding (doc at the call site): the unique positively-wound
    /// cycle is the outer loop. `Some(i)` means ring `i` must swap into
    /// the outer slot; `None` means the roles are already correct, or
    /// the survivor is not a planar rung. Zero or multiple positive
    /// cycles refuse [`MergeCoplanarError::MergedFaceRoleAmbiguous`].
    ///
    /// Takes the survivor's resolved data rather than looking it up:
    /// the caller holds the liveness proof, and a helper that cannot
    /// look anything up cannot discard a failed lookup. `face` is the
    /// refusal's payload only — nothing here dereferences it.
    fn merged_outline_ring(
        &self,
        face: FaceKey,
        survivor: &crate::entity::Face,
        tol: Tol,
    ) -> Result<Option<usize>, MergeCoplanarError> {
        let band = Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?;
        // The face's OUTWARD normal (S10): "positively wound" means
        // CCW seen from OUTSIDE the material, so on a reversed face
        // the chart normal names the opposite convention and every
        // role assignment below would come out inverted.
        let normal = match self.get_surface(survivor.surface) {
            Some(Surface::Plane { normal, .. }) => *normal * survivor.sense_sign::<T>(),
            _ => return Ok(None), // merges only fire on planar rungs
        };
        let mut positives: Vec<Option<usize>> = Vec::new(); // None = outer
        if self.loop_winding(survivor.outer, normal, band)? == Some(geom_core::Sign::Positive) {
            positives.push(None);
        }
        for (i, &r) in survivor.rings.iter().enumerate() {
            if self.loop_winding(r, normal, band)? == Some(geom_core::Sign::Positive) {
                positives.push(Some(i));
            }
        }
        match positives[..] {
            [None] => Ok(None), // roles already correct
            [Some(i)] => Ok(Some(i)),
            _ => Err(MergeCoplanarError::MergedFaceRoleAmbiguous { face }),
        }
    }
}
