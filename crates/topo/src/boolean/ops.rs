//! The public boolean set operations (M3 PR 5): [`union`],
//! [`intersect`], [`subtract`] — functional (operands untouched),
//! composing reduce → classify (PR 4) → join → `setopfinish` → the
//! combine door → seam zip → the `merge_coplanar_faces` output stage
//! (F7) → tier gates. Every stage's refusal passes through typed as a
//! [`BooleanError`] variant.
//!
//! # Results (F8)
//!
//! ∅ is a typed SUCCESS ([`BooleanResult::Empty`]) — GQ2's per-node
//! result DAG wants a value, not an error. Real results carry a
//! [`BooleanResultKind`]:
//!
//! - [`Seamed`](BooleanResultKind::Seamed): boundaries intersected;
//!   the seam was zipped.
//! - [`OperandA`](BooleanResultKind::OperandA) /
//!   [`OperandB`](BooleanResultKind::OperandB): one operand's material
//!   is the whole answer (disjoint ∖, nested ∩, …).
//! - [`Assembly`](BooleanResultKind::Assembly): a multi-shell body
//!   combining components of both operands without a seam — the
//!   disjoint union (∪ of separated bodies), including
//!   touching-at-declared-contacts assemblies (the carried
//!   [`ContactRecords`] say where; genuinely 3′, certified by PR 6's
//!   validator).
//! - [`Voided`](BooleanResultKind::Voided): **the first legitimate
//!   voids** — A∖B with B strictly inside A yields the outer shell
//!   plus the reverted inner shell, a tier-2-legal multi-shell body
//!   (exactly the voids-born-only-from-booleans ratification; sweeps
//!   never produce these — `FullRevolveHoles` points here).
//!
//! When operand boundaries do not intersect, classification falls back
//! to per-shell vertex-in-solid containment
//! ([`point_in_solid`](super::solid_contain::point_in_solid), F8's ray
//! design promoted to 3-D), probing non-contact vertices against the
//! pristine other operand.
//!
//! # The merge output stage (F7)
//!
//! The seam zip manufactures coplanar same-surface-key face pairs by
//! construction (a cut face's fragments), so each op runs
//! `merge_coplanar_faces` as a documented final stage — part of the
//! op's contract, not hidden healing (the recipe records ONE boolean
//! node). The mergeable pairs are structural/declared by construction;
//! cross-operand *numeric* coplanarity is honestly left unmerged (the
//! coincidence ladder has no numeric rung).
//!
//! # Carried contacts
//!
//! Result bodies carry the declared-contact records whose entities
//! survive into the result, remapped to result keys (B-side keys
//! through the combine door's graft map). Records referencing
//! discarded entities are dropped — a contact between A and B is only
//! meaningful in a result containing both sides.
//!
//! # Known limitations (PR 5 review — the honest envelope)
//!
//! The seam lane's WORKING envelope: transversal boundary crossings
//! plus interior-rest coplanar unions, all verified against exact
//! volume/area oracles. Outside it the ops REFUSE — every refusal
//! typed, deterministic, operands untouched; never a silent wrong
//! body. The refusing configurations (review findings R1–R3 plus the
//! previously pinned pair):
//!
//! - **Single-ring pockets/bosses are orientation-dependent** (R1):
//!   the identical blind pocket succeeds with the exact volume on a
//!   brick's {+z, −x, −y} faces and refuses
//!   [`BooleanError::SeamOrientation`] on {−z, +x, +y} — a
//!   handedness-correlated HALF of face orientations, NOT "fixed
//!   except for double-ring configs".
//! - **Double-ring single-face seams** (through-pillar tunnel,
//!   inset-leg union): `SeamOrientation`.
//! - **Multi-collinear-site seams** (R2, four collinear crossing
//!   sites on one line): [`BooleanError::JoinDesync`].
//! - **Crossing-polygon face disconnection** (R3, crossing-polygon
//!   operands whose seams disconnect a face): `SeamOrientation`.
//! - **Coplanar-overlap ∩** (Fig 15.1, deferred acceptance item) and
//!   **corner-flush** contacts: refuse.
//!
//! Root cause (one sentence): the cross-solid null-edge
//! ordering/orientation discipline is not enforced — `choose_roles`'
//! prefer-mirror heuristic carries no consistency theorem — which is
//! exactly PR 5.5's charter (M3-LOG resumption entry).

use geom_core::{Band, Decide, Real};

use super::combine::{GraftMap, graft_solid};
use super::finish::{contact_skip_set, kept_side, setopfinish};
use super::join::bool_connect;
use super::solid_contain::{PointInSolidError, SolidContainment, point_in_solid};
use super::zip::zip_seam;
use super::{
    BooleanError, BooleanOp, BooleanReduction, ContactRecords, Operand, SideCode, VfContact,
    VvContact, boolean_reduce,
};
use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary, ShellKey, VertexKey};
use crate::splitting::finish::{carve, single_solid};
use crate::validate::{validate, validate_closed};

/// How a boolean result body came to be (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanResultKind {
    /// Boundaries intersected; the seam was joined and zipped.
    Seamed,
    /// The result is operand A's material.
    OperandA,
    /// The result is operand B's material.
    OperandB,
    /// A multi-shell combination of components from both operands
    /// without a seam (disjoint or touching-only).
    Assembly,
    /// A∖B with B inside A: outer shell + reverted inner void shell.
    Voided,
}

/// A real (non-empty) boolean result.
#[derive(Debug)]
pub struct BooleanBody<T: Real> {
    /// The result body: one solid, possibly multi-shell.
    pub body: Body<T>,
    /// How it was produced.
    pub kind: BooleanResultKind,
    /// Declared contacts surviving into the result, in result keys
    /// (module docs) — the tier-3′ declarations.
    pub contacts: ContactRecords,
}

/// The typed result of a boolean op: a body, or the typed empty
/// success (F8: ∅ is a value, not an error).
// Size skew vs `Empty` is inherent (same posture as `SplitPart`).
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum BooleanResult<T: Real> {
    /// The regularized result is empty.
    Empty,
    /// A real result body.
    Body(BooleanBody<T>),
}

impl<T: Real> BooleanResult<T> {
    /// The result body, if non-empty.
    pub fn body(&self) -> Option<&BooleanBody<T>> {
        match self {
            Self::Body(b) => Some(b),
            Self::Empty => None,
        }
    }
}

/// A ∪* B (module docs; functional, planar-only per F5).
///
/// # Errors
///
/// [`BooleanError`] — every stage's typed refusals pass through.
pub fn union<T: Decide>(a: &Body<T>, b: &Body<T>) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op(BooleanOp::Union, a, b)
}

/// A ∩* B (module docs).
///
/// # Errors
///
/// [`BooleanError`].
pub fn intersect<T: Decide>(a: &Body<T>, b: &Body<T>) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op(BooleanOp::Intersect, a, b)
}

/// A ∖* B (module docs).
///
/// # Errors
///
/// [`BooleanError`].
pub fn subtract<T: Decide>(a: &Body<T>, b: &Body<T>) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op(BooleanOp::Subtract, a, b)
}

/// The shared pipeline (module docs).
fn boolean_op<T: Decide>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
) -> Result<BooleanResult<T>, BooleanError> {
    let band = Band::linear()?;
    let mut red = boolean_reduce(op, a, b)?;

    if red.null_pairs.is_empty() {
        if !red.null_edges.is_empty() {
            return Err(BooleanError::ClassificationInvariant {
                what: "null edges without pairs reached the op",
            });
        }
        return fallback(op, &red, a, b, band);
    }

    let completed = bool_connect(&mut red, a, b, band)?;
    if completed.is_empty() {
        return Err(BooleanError::JoinDesync {
            what: "null pairs joined into no completed polygon",
        });
    }
    let contacts = red.contacts.clone();
    let fin = setopfinish(op, red, &completed, a, b, band)?;
    let mut body = fin.body;
    for &(a_face, b_face) in &fin.seams {
        zip_seam(&mut body, a_face, b_face, &fin.vertex_map)?;
    }
    body.merge_coplanar_faces().map_err(BooleanError::Merge)?;
    let contacts = remap_contacts(
        &body,
        &contacts,
        KeyView::Direct,
        KeyView::Graft(&fin.graft),
    );
    gate(&body)?;
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind: BooleanResultKind::Seamed,
        contacts,
    }))
}

/// How one operand's keys map into the result body.
enum KeyView<'a> {
    /// Keys carried through unchanged (carve preserves keys).
    Direct,
    /// Keys bridged by the combine door's graft.
    Graft(&'a GraftMap),
    /// The operand is not part of the result.
    Absent,
}

impl KeyView<'_> {
    fn vertex<T: Real>(&self, body: &Body<T>, v: VertexKey) -> Option<VertexKey> {
        let mapped = match self {
            Self::Direct => Some(v),
            Self::Graft(g) => g.vertices.get(v).copied(),
            Self::Absent => None,
        }?;
        body.get_vertex(mapped).map(|_| mapped)
    }

    fn face<T: Real>(&self, body: &Body<T>, f: FaceKey) -> Option<FaceKey> {
        let mapped = match self {
            Self::Direct => Some(f),
            Self::Graft(g) => g.faces.get(f).copied(),
            Self::Absent => None,
        }?;
        body.get_face(mapped).map(|_| mapped)
    }
}

/// Remaps the declared contacts into result keys, dropping records
/// whose entities did not survive (module docs).
fn remap_contacts<T: Real>(
    body: &Body<T>,
    contacts: &ContactRecords,
    a_view: KeyView<'_>,
    b_view: KeyView<'_>,
) -> ContactRecords {
    let mut out = ContactRecords::default();
    for c in &contacts.vv {
        if let (Some(a), Some(b)) = (a_view.vertex(body, c.a), b_view.vertex(body, c.b)) {
            out.vv.push(VvContact { a, b });
        }
    }
    for c in &contacts.a_on_b {
        if let (Some(vertex), Some(face)) =
            (a_view.vertex(body, c.vertex), b_view.face(body, c.face))
        {
            out.a_on_b.push(VfContact { vertex, face });
        }
    }
    for c in &contacts.b_on_a {
        if let (Some(vertex), Some(face)) =
            (b_view.vertex(body, c.vertex), a_view.face(body, c.face))
        {
            out.b_on_a.push(VfContact { vertex, face });
        }
    }
    out
}

/// The tier gates: tier 1 + tier 2 on the finished result (tier 3 is
/// an at-rest posture with the PR 3 description gap — see the
/// acceptance suite's documented posture).
fn gate<T: Real>(body: &Body<T>) -> Result<(), BooleanError> {
    validate(body).map_err(|errors| BooleanError::ResultInvalid { errors })?;
    validate_closed(body).map_err(|errors| BooleanError::ResultInvalid { errors })?;
    Ok(())
}

/// Per-shell classification of one operand's clone against the other
/// pristine operand (containment fallback; contact vertices skipped,
/// `OnBoundary` probes advanced past).
fn classify_shells<T: Decide>(
    body: &Body<T>,
    other: &Body<T>,
    contacts: &ContactRecords,
    operand: Operand,
    band: Band,
) -> Result<Vec<(ShellKey, SideCode)>, BooleanError> {
    let corrupt = || BooleanError::JoinDesync {
        what: "fallback operand clone is not walkable",
    };
    let skip = contact_skip_set(contacts, operand);
    let mut out = Vec::new();
    for (shell, shell_data) in body.shells() {
        let mut verdict = None;
        'probe: for &face in &shell_data.faces {
            let face_data = body.get_face(face).ok_or_else(corrupt)?;
            for l in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
                let LoopBoundary::Cycle { first } = body.get_loop(l).ok_or_else(corrupt)?.boundary
                else {
                    continue;
                };
                for he in body.loop_cycle(first).ok_or_else(corrupt)? {
                    let v = body.get_half_edge(he).ok_or_else(corrupt)?.start;
                    if skip.contains_key(v) {
                        continue;
                    }
                    let q = *body
                        .get_vertex(v)
                        .and_then(|vd| body.get_point(vd.point))
                        .ok_or_else(corrupt)?;
                    match point_in_solid(other, q, band).map_err(BooleanError::Containment)? {
                        SolidContainment::In => {
                            verdict = Some(SideCode::In);
                            break 'probe;
                        }
                        SolidContainment::Out => {
                            verdict = Some(SideCode::Out);
                            break 'probe;
                        }
                        SolidContainment::OnBoundary => continue,
                    }
                }
            }
        }
        let side = verdict.ok_or(BooleanError::Containment(PointInSolidError::RayExhausted))?;
        out.push((shell, side));
    }
    Ok(out)
}

/// The containment fallback (F8): no crossings — classify whole
/// shells, keep per Eq. 15.1's sides, and assemble the typed result.
fn fallback<T: Decide>(
    op: BooleanOp,
    red: &BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    band: Band,
) -> Result<BooleanResult<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let a_sides = classify_shells(&red.a, b_pristine, &red.contacts, Operand::A, band)?;
    let b_sides = classify_shells(&red.b, a_pristine, &red.contacts, Operand::B, band)?;
    let keep_a = kept_side(op, Operand::A);
    let keep_b = kept_side(op, Operand::B);
    let a_keep: Vec<ShellKey> = a_sides
        .iter()
        .filter(|(_, s)| *s == keep_a)
        .map(|(k, _)| *k)
        .collect();
    let b_keep: Vec<ShellKey> = b_sides
        .iter()
        .filter(|(_, s)| *s == keep_b)
        .map(|(k, _)| *k)
        .collect();

    let carve_kept = |body: &Body<T>, keep: &[ShellKey]| -> Result<Body<T>, BooleanError> {
        let solid = single_solid(body).map_err(|_| desync("fallback operand not one solid"))?;
        carve(body, solid, keep).map_err(|_| desync("fallback carve failed"))
    };

    match (a_keep.is_empty(), b_keep.is_empty()) {
        (true, true) => {
            if op == BooleanOp::Union {
                // ∪ can never be empty; only complement operands (both
                // boundaries inside the other's material) reach here.
                Err(BooleanError::UnrepresentableResult)
            } else {
                Ok(BooleanResult::Empty)
            }
        }
        (false, true) => {
            let body = carve_kept(&red.a, &a_keep)?;
            finish_fallback(op, body, &red.contacts, BooleanResultKind::OperandA)
        }
        (true, false) => {
            let body = carve_kept(&red.b, &b_keep)?;
            finish_fallback(op, body, &red.contacts, BooleanResultKind::OperandB)
        }
        (false, false) => {
            let mut body = carve_kept(&red.a, &a_keep)?;
            let mut b_body = carve_kept(&red.b, &b_keep)?;
            if op == BooleanOp::Subtract {
                b_body = b_body.revert().map_err(BooleanError::Revert)?;
            }
            let solid =
                single_solid(&body).map_err(|_| desync("fallback A carve not one solid"))?;
            let graft = graft_solid(&mut body, solid, &b_body)?;
            let kind = match op {
                BooleanOp::Subtract => BooleanResultKind::Voided,
                _ => BooleanResultKind::Assembly,
            };
            body.merge_coplanar_faces().map_err(BooleanError::Merge)?;
            let contacts = remap_contacts(
                &body,
                &red.contacts,
                KeyView::Direct,
                KeyView::Graft(&graft),
            );
            gate(&body)?;
            Ok(BooleanResult::Body(BooleanBody {
                body,
                kind,
                contacts,
            }))
        }
    }
}

/// Finishes a single-operand fallback result (the merge output stage
/// is a documented no-op on a maximal-faced operand but runs anyway —
/// the contract is uniform), applying ∖'s B-side revert when needed.
fn finish_fallback<T: Decide>(
    op: BooleanOp,
    body: Body<T>,
    contacts: &ContactRecords,
    kind: BooleanResultKind,
) -> Result<BooleanResult<T>, BooleanError> {
    let mut body = body;
    if kind == BooleanResultKind::OperandB && op == BooleanOp::Subtract {
        body = body.revert().map_err(BooleanError::Revert)?;
    }
    body.merge_coplanar_faces().map_err(BooleanError::Merge)?;
    let (a_view, b_view) = match kind {
        BooleanResultKind::OperandA => (KeyView::Direct, KeyView::Absent),
        _ => (KeyView::Absent, KeyView::Direct),
    };
    let contacts = remap_contacts(&body, contacts, a_view, b_view);
    gate(&body)?;
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind,
        contacts,
    }))
}
