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
//! # Known limitations (PR 5.5 — the honest envelope)
//!
//! The seam lane's WORKING envelope (all exact-oracle-verified):
//! transversal boundary crossings; single-ring pockets/bosses on ALL
//! six face orientations (PR 5's R1 closed); double-ring single-face
//! seams (through-pillar tunnel, inset-leg union); multi-collinear-
//! site seams (R2) and crossing-polygon disconnections (R3);
//! interior-rest flush contacts (pillar standing on a face); and the
//! Fig 15.1 coplanar-overlap ∩ (seam partly on shared cap planes) —
//! the `join` module's derived sense/role discipline is the
//! consistency theorem behind all of them.
//!
//! Still refusing — typed, deterministic, operands untouched; never a
//! silent wrong body:
//!
//! - **Boundary-on-boundary seams**: configurations whose seam
//!   segments lie ALONG existing operand edges (the full-overlap
//!   stacked union; corner-flush rests whose contact-square edges are
//!   collinear with the face's own edges). The on-edge germs have no
//!   facing chord partner — such seams need on-edge RUNS (reusing the
//!   existing edges) rather than chords, a mechanism this pipeline
//!   does not yet have. Refusal:
//!   `Join(UnpairedLooseEnds)`.

use geom_core::{Band, Decide, MarginDiag, Real, Sign};

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
use crate::props::mass_properties_with;
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
    volume_backstop(op, a, b, &body, band)?;
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind: BooleanResultKind::Seamed,
        contacts,
    }))
}

/// The volume-inequality backstop at the op gate (PR 5 review): every
/// `Seamed` result must satisfy the set-theoretic bounds
/// vol(∩) ≤ min(vol A, vol B), vol(∪) ≥ max(vol A, vol B),
/// vol(∖) ≤ vol A — computed with the exact planar
/// [`mass_properties_with`]. The min/max are decomposed into per-operand
/// inequalities, so no operand-vs-operand comparison is needed.
///
/// Comparison posture: each bound margin is classified through the
/// certified trilean (`sign_within` against the op's linear band) —
/// the codebase's only legal comparison (Q1). Only a CERTIFIED
/// violating sign refuses ([`BooleanError::ResultVolumeImplausible`]);
/// `Zero` and in-band indeterminate margins PASS: on the planar
/// corpus the flux sums are exact for dyadic fixtures (margin exactly
/// 0 or macroscopic), and the bug class this backstop guards —
/// wrong-component results — violates its bound by whole regions, so
/// refusing on an ulp-scale tie would make the gate noisier than the
/// property it guards. A POISONED margin (NaN) still refuses loudly
/// ([`BooleanError::Escalated`]) — poison never passes a gate.
///
/// Complement operands: a reverted body's flux volume is NEGATIVE
/// (its true set volume is infinite — the A∖B ≡ A∩revert(B) oracle
/// route feeds such operands legitimately), so each bound applies
/// only when its reference operand's volume is certified POSITIVE
/// (bounded solid); against a complement the set bound is vacuous
/// and is skipped, never misread as a violation.
fn volume_backstop<T: Decide>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    result: &Body<T>,
    band: Band,
) -> Result<(), BooleanError> {
    let corrupt = || BooleanError::ClassificationInvariant {
        what: "volume backstop: mass properties refused on a tier-valid planar body",
    };
    let vol = |body: &Body<T>| -> Result<T, BooleanError> {
        Ok(mass_properties_with(body, band)
            .map_err(|_| corrupt())?
            .volume)
    };
    let (va, vb, vr) = (vol(a)?, vol(b)?, vol(result)?);
    // A bound applies only against a certified-bounded operand
    // (positive flux volume); complement operands (certified negative)
    // make it vacuous. Poison refuses; an in-band operand volume is a
    // degenerate operand — no bound is certifiable from it, skip.
    let bounded = |v: T| -> Result<bool, BooleanError> {
        match v.sign_within(band) {
            Ok(Sign::Positive) => Ok(true),
            Ok(Sign::Zero | Sign::Negative) => Ok(false),
            Err(diag) if matches!(diag.margin, MarginDiag::Invalid) => {
                Err(BooleanError::Escalated {
                    diag: diag.with_predicate("volume_backstop"),
                })
            }
            Err(_) => Ok(false),
        }
    };
    // `margin` ≥ 0 (within band) or the bound named by `which` is
    // violated: margin = bound − got for upper bounds, got − bound for
    // lower bounds.
    let check = |which: &'static str, margin: T, got: T, bound: T| -> Result<(), BooleanError> {
        match margin.sign_within(band) {
            Ok(Sign::Negative) => Err(BooleanError::ResultVolumeImplausible {
                which,
                got: format!("{got:?}"),
                bound: format!("{bound:?}"),
            }),
            Ok(Sign::Zero | Sign::Positive) => Ok(()),
            Err(diag) if matches!(diag.margin, MarginDiag::Invalid) => {
                Err(BooleanError::Escalated {
                    diag: diag.with_predicate("volume_backstop"),
                })
            }
            Err(_) => Ok(()),
        }
    };
    let (ba, bb) = (bounded(va)?, bounded(vb)?);
    match op {
        BooleanOp::Intersect => {
            if ba {
                check("vol(A ∩ B) ≤ vol(A)", va - vr, vr, va)?;
            }
            if bb {
                check("vol(A ∩ B) ≤ vol(B)", vb - vr, vr, vb)?;
            }
        }
        BooleanOp::Union => {
            if ba {
                check("vol(A ∪ B) ≥ vol(A)", vr - va, vr, va)?;
            }
            if bb {
                check("vol(A ∪ B) ≥ vol(B)", vr - vb, vr, vb)?;
            }
        }
        BooleanOp::Subtract => {
            if ba {
                check("vol(A ∖ B) ≤ vol(A)", va - vr, vr, va)?;
            }
        }
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::Band;

    use super::volume_backstop;
    use crate::boolean::{BooleanError, BooleanOp};
    use crate::splitting::reassembly::quad_prism;

    /// The backstop's refusal wiring, by construction: feed it a
    /// "result" whose exact volume violates the op's bound (a half-height
    /// prism vs the unit cube, both surface-certified geometric bodies)
    /// and require the typed `ResultVolumeImplausible`; the non-strict
    /// pass direction (result ≡ operand, margin exactly zero) must pass
    /// all three ops.
    #[test]
    fn volume_backstop_wiring() {
        let band = Band::linear().unwrap();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let cube = quad_prism(&square, 1.0);
        let small = quad_prism(&square, 0.5);
        let implausible = |e: BooleanError| {
            assert!(
                matches!(e, BooleanError::ResultVolumeImplausible { .. }),
                "expected ResultVolumeImplausible, got {e:?}"
            );
        };
        // vol(∪) ≥ max: a union "result" smaller than an operand.
        implausible(volume_backstop(BooleanOp::Union, &cube, &cube, &small, band).unwrap_err());
        // vol(∖) ≤ vol(A): a subtract "result" larger than A.
        implausible(volume_backstop(BooleanOp::Subtract, &small, &cube, &cube, band).unwrap_err());
        // vol(∩) ≤ min: an intersect "result" larger than an operand.
        implausible(volume_backstop(BooleanOp::Intersect, &small, &cube, &cube, band).unwrap_err());
        // Equal volumes: every bound is non-strict — all pass.
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            volume_backstop(op, &cube, &cube, &cube, band).unwrap();
        }
        // Complement operand (negative flux volume): its bound is
        // vacuous and must be SKIPPED — A ∩ revert(B) legitimately
        // exceeds vol(revert B); the A-side bound still applies.
        let rev = quad_prism(&square, 0.5).revert().unwrap();
        volume_backstop(BooleanOp::Intersect, &cube, &rev, &cube, band).unwrap();
        implausible(volume_backstop(BooleanOp::Intersect, &small, &rev, &cube, band).unwrap_err());
    }
}
