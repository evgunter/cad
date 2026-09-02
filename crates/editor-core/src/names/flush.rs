//! **The detect / declare protocol** (LIB-SEL2; `docs/SELECT-DESIGN.md`
//! §3, ratified #286 incl. the round-2 no-fusion amendment).
//!
//! Three separated pieces, and the separation is the design:
//!
//! - **Detect** — [`find_flush_candidates`] reports the cross-body
//!   face pairs that *would verify* under a declaration, as
//!   [`FlushFinding`] VALUES. A finding is a REPORT: it glues nothing,
//!   changes no topology, and is never stored in a recipe — it is the
//!   coincidence ladder's "these faces coincide exactly — declare the
//!   relation?" affordance given an API.
//! - **Declare** — [`declare`] / [`declare_all`] are thin sugar over
//!   the SHIPPED [`Node::Declare`] vocabulary: they take
//!   explicitly-passed findings and insert a Declare node whose id the
//!   caller wires into the consuming Boolean's `declare` input.
//! - **The menu** — the boolean's undeclared-coincidence refusal names
//!   exactly two arms: declare the found class (→ the sugar) or move
//!   the geometry. NO absorb arm (the #256 ruling applied to contact).
//!
//! # The no-fusion boundary (GS-Q3, RULED)
//!
//! Both `declare(finding)` and `declare_all(findings)` ship — the
//! ruled boundary is FUSION, not arity. A fused detect-and-declare
//! door is forbidden permanently: findings must pass through
//! user-visible hands AS VALUES (separate detect and declare calls,
//! inspectable in between), because that is the enforceable
//! intent-recording property. C4's verify-at-use backstops lies
//! either way.
//!
//! # The anti-twin rule (§3b): the detector IS the verifier
//!
//! The detector has NO predicate triple of its own. It enumerates
//! candidate pairs and asks the kernel's own rung at the seat where
//! that rung lives: [`topo::flush::pair_finding`] — descriptions,
//! oriented identity evidence and the verification arm all live
//! inside [`topo::flush_pair_relation`] under it, whose verdict
//! ladder is the one verify-at-use reaches through `carrier_eq`'s
//! plane delegation (the three-link chain is stated once, in
//! [`topo::flush`]'s module docs, and not restated here).
//! Consequences, all deliberate:
//!
//! - detect-then-declare can never disagree with verify-at-use: the
//!   two paths converge on one verdict function, so there is no
//!   second implementation to keep in step by hand;
//! - the body seat's own detector
//!   ([`topo::flush::find_flush_candidates`]) enumerates over the
//!   SAME rung, so the two seats cannot disagree either: findings are
//!   names at this door, keys at the body door, one verifier under
//!   both;
//! - detection's decisions go through the funnel at the VERIFIER'S
//!   sites (`bool_plane_parallel` / `bool_plane_orient` /
//!   `bool_plane_offset`) — no `sel_flush_*` site exists, no new
//!   ledger row is owed, and GS-Q1's K-census participation is
//!   automatic through those names. The detector interprets nothing
//!   the verifier doesn't.
//!
//! What stays HERE is the name-flavored half, on the `select_where`
//! precedent (VERB-SEAT-DESIGN §1 S2): resolving a name table's face
//! names to their candidate keys — a node value may carry several
//! bodies, and a tied name several keys in each — the GS-Q4 trilean
//! over those candidate combinations, and the refusal payloads that
//! name a `StableName`. None of that is geometry, and none of it can
//! be spelled below the G1 line.
//!
//! # Findings are DEFINITE; in-band pairs refuse (§3a)
//!
//! A [`FlushFinding`] is only ever definite. A pair whose verify-door
//! margin lands inside the ambiguity band is neither reported nor
//! silently dropped: the whole query refuses with
//! [`SelectRefusal::PairInBand`] naming the pair — §2's honesty
//! obligation, applied to detection. Tied names follow GS-Q4's
//! trilean: all candidates flush ⇒ the finding stands (still tied —
//! downstream still owns the ambiguity refusal); none ⇒ no finding;
//! mixed ⇒ [`SelectRefusal::TiedDisagrees`].
//!
//! # What `Rest` means here (v1)
//!
//! Flush/`Rest` planes are the whole v1 detector — the only
//! demand-evidenced case. The [`ContactClass::Rest`] tag names C4's
//! coincident-plane contact class; the evidence records which
//! orientation the verifier decided: [`PlaneRelation::SameOpposite`]
//! is the resting-contact flavor (opposed outward normals — the REST
//! lane's zip), [`PlaneRelation::SameOriented`] the merge-stage
//! flavor (flush walls). Both are exactly the pairs the declared rung
//! verifies and the P9 helper used to declare. `Tangent`/`Fit`
//! findings reuse this shape when their demand arrives — the `class`
//! field is the reserved slot, not a `flush: bool`.
//!
//! # Documented residuals
//!
//! - The verifier encodes its definite-zero-offset verdict and a
//!   NaN-poisoned margin with the same `MarginDiag::Invalid`
//!   diagnostic; the detector takes the verifier's encoding as-is
//!   (anti-twin: it interprets nothing the verifier doesn't), so a
//!   NaN-poisoned plane pair — geometry that is broken well before
//!   detection — would read as a finding whose declaration then
//!   escalates at use. C4's verify-at-use backstop is the answer by
//!   design.
//! - Pairs already declared in the recipe are reported again: a
//!   finding is a report about geometry, not a diff against intent,
//!   and the caller holding the recipe is the one who knows.

use geom_core::{Band, Decide, Tol};
use topo::flush::{finding, pair_finding};
use topo::{Body, FaceKey, PlaneRelation};

use crate::doc::Doc;
use crate::edit::{DocEdit, EditError, apply};
use crate::eval::{Evaluation, NodeResult, NodeValue};
use crate::names::geompred::SelectRefusal;
use crate::names::interrogate;
use crate::names::interrogate::InterrogateError;
use crate::names::role::{EntityKind, StableName};
use crate::names::table::{EntityKey, EntityRef, Entry};
use crate::node::{Node, RecipeNodeId};

/// The contact class a declaration asserts (CONTACT-DESIGN C4) — a
/// RE-EXPORT of the kernel's vocabulary, never a parallel enum.
///
/// The type is defined in `topo` because the boolean's own contact
/// refusals must carry the same words the detector produces
/// (SELECT-DESIGN §3d, "one vocabulary end-to-end") and `topo` cannot
/// depend on this crate. Everything above re-exports it; nothing
/// redefines it.
pub use topo::ContactClass;

/// The rest of the kernel contact vocabulary, re-exported at the same
/// door and for the same reason (see [`ContactClass`]): a refusal the
/// recipe layer renders must quote the kernel's own sentence, not a
/// paraphrase that can drift from it.
///
/// [`FIT_DEFERRAL`] in particular is quoted VERBATIM wherever a
/// designed clearance is steered toward `Fit` — including by the
/// assembly layer's mate verification — so there is exactly one place
/// the deferral is worded.
pub use topo::{CONTACT_RECOURSE, ContactRefusal, ContactVerdict, DeclaredContact, FIT_DEFERRAL};

/// The finding vocabulary, RE-EXPORTED from the kernel for the reason
/// [`ContactClass`] is: the evidence a finding carries is the verify
/// door's own verdict, that door is the kernel's, and a second
/// spelling of its verdict at this layer is exactly the twin the
/// anti-twin rule forbids. For a tied name whose candidates decided
/// through different rungs, the weaker claim
/// ([`FlushRung::DecidedCoincident`]) is what this door records.
pub use topo::flush::{FlushEvidence, FlushRung};

/// One flush-plane finding at the DOCUMENT seat: "this cross-body face
/// pair would verify as declared contact" — a VALUE, inspectable,
/// never itself a declaration (SELECT-DESIGN §3a).
///
/// The kernel's [`topo::flush::FlushFinding`] with this seat's pair
/// vocabulary: names, never keys (G1). `.0` is from the detector's `a`
/// node, `.1` from `b`. The body seat's finding
/// ([`topo::flush::FacePairFinding`]) is the same type over face keys
/// — findings are names at the document door, keys at the body door,
/// one verifier under both.
pub type FlushFinding = topo::flush::FlushFinding<(StableName, StableName)>;

// ---------------------------------------------------------------
// (a) Detect.
// ---------------------------------------------------------------

/// **The cross-body flush-plane candidates between `a`'s and `b`'s
/// outputs, as of THIS evaluation** — the C4 verifier run in
/// candidate-generation mode (module docs; SELECT-DESIGN §3a/b).
///
/// Findings come back in canonical order (sorted by name pair) and
/// are only ever DEFINITE. Like [`select`](fn@super::select), the query
/// answers empty if either node has no value in this evaluation.
///
/// # Errors
///
/// [`SelectRefusal::PairInBand`] when a pair's verify-door margin is
/// indeterminate (never silently included or dropped),
/// [`SelectRefusal::TiedDisagrees`] when a tied name's candidates
/// disagree (GS-Q4), [`SelectRefusal::Unreadable`] on a name-table
/// entry that does not resolve into its node's payload,
/// [`SelectRefusal::Band`] if the ambient tolerance is broken.
pub fn find_flush_candidates<T: Decide>(
    ev: &Evaluation<T>,
    a: RecipeNodeId,
    b: RecipeNodeId,
    tol: Tol,
) -> Result<Vec<FlushFinding>, SelectRefusal> {
    let (Some(NodeResult::Ok(va)), Some(NodeResult::Ok(vb))) = (ev.nodes.get(&a), ev.nodes.get(&b))
    else {
        return Ok(Vec::new());
    };
    let band = Band::linear(tol).map_err(|_| SelectRefusal::Band)?;
    let fa = face_candidates(va)?;
    let fb = face_candidates(vb)?;
    let mut out = Vec::new();
    for (na, ca) in &fa {
        for (nb, cb) in &fb {
            if let Some(finding) = pair_verdict(na, ca, nb, cb, band)? {
                out.push(finding);
            }
        }
    }
    out.sort_by(|x, y| x.pair.cmp(&y.pair));
    Ok(out)
}

/// One side's candidates: each FACE name with the `(body, key)`
/// candidates it resolves to in this evaluation.
type FaceCandidates<'v, T> = Vec<(StableName, Vec<(&'v Body<T>, FaceKey)>)>;

/// Every FACE name of one node's value with its resolved candidate
/// keys — `Unique` gives one candidate, `Tied` all of them (GS-Q4:
/// the tie is the table's fact; detection asks every candidate).
fn face_candidates<T: Decide>(v: &NodeValue<T>) -> Result<FaceCandidates<'_, T>, SelectRefusal> {
    let mut out = Vec::new();
    for (name, entry) in v.name_table.iter() {
        if name.kind != EntityKind::Face {
            continue;
        }
        let refs: &[EntityRef] = match entry {
            Entry::Unique(e) => core::slice::from_ref(e),
            Entry::Tied(v) => v,
        };
        let mut cands = Vec::with_capacity(refs.len());
        for ent in refs {
            let body = interrogate::output_body(&v.payload, ent.body).map_err(|error| {
                SelectRefusal::Unreadable {
                    name: Box::new(name.clone()),
                    error,
                }
            })?;
            let EntityKey::Face(f) = ent.key else {
                // A Face-kind name resolving to a non-face key is an
                // emitter invariant violation — reported, not skipped
                // (skipping would half-select the name).
                return Err(SelectRefusal::Unreadable {
                    name: Box::new(name.clone()),
                    error: InterrogateError::WrongKind {
                        wanted: EntityKind::Face,
                        found: ent.key.kind(),
                    },
                });
            };
            cands.push((body, f));
        }
        out.push((name.clone(), cands));
    }
    Ok(out)
}

/// The GS-Q4 trilean over one name pair's candidate combinations:
/// all combinations flush (with one relation) ⇒ a finding; none ⇒
/// no finding; mixed ⇒ `TiedDisagrees` naming the tied side.
fn pair_verdict<T: Decide>(
    na: &StableName,
    ca: &[(&Body<T>, FaceKey)],
    nb: &StableName,
    cb: &[(&Body<T>, FaceKey)],
    band: Band,
) -> Result<Option<FlushFinding>, SelectRefusal> {
    let mut relation: Option<PlaneRelation> = None;
    let mut all_shared_source = true;
    let mut matched = 0usize;
    let total = ca.len() * cb.len();
    for &(ba, fa) in ca {
        for &(bb, fb) in cb {
            let verdict =
                pair_finding(ba, fa, bb, fb, band).map_err(|source| SelectRefusal::PairInBand {
                    pair: Box::new((na.clone(), nb.clone())),
                    predicate: source.predicate.unwrap_or("flush_pair_relation"),
                    source,
                })?;
            if let Some(FlushEvidence {
                relation: rel,
                rung,
            }) = verdict
            {
                matched += 1;
                all_shared_source &= rung == FlushRung::SharedSource;
                match relation {
                    None => relation = Some(rel),
                    // Tied candidates flush with OPPOSITE orientations:
                    // no single definite finding exists — the mixed-tie
                    // refusal (GS-Q4).
                    Some(prev) if prev != rel => {
                        return Err(tied_disagrees(na, ca, nb, matched, total));
                    }
                    Some(_) => {}
                }
            }
        }
    }
    match (matched, relation) {
        (0, _) | (_, None) => Ok(None),
        // The CLASS is minted at the kernel's one site, not here: this
        // seat contributes the pair vocabulary and the tie-resolved
        // evidence, and takes the classification from the door that
        // decides it (`topo::flush::finding`).
        (m, Some(relation)) if m == total => Ok(Some(finding(
            (na.clone(), nb.clone()),
            FlushEvidence {
                relation,
                rung: if all_shared_source {
                    FlushRung::SharedSource
                } else {
                    FlushRung::DecidedCoincident
                },
            },
        ))),
        _ => Err(tied_disagrees(na, ca, nb, matched, total)),
    }
}

/// The mixed-tie refusal, blaming the side that actually has a tie
/// (the A side when both do). `matched`/`candidates` count pair
/// COMBINATIONS — the quantity the trilean was taken over.
fn tied_disagrees<T: Decide>(
    na: &StableName,
    ca: &[(&Body<T>, FaceKey)],
    nb: &StableName,
    matched: usize,
    total: usize,
) -> SelectRefusal {
    // If neither side is tied this is unreachable: one combination
    // cannot disagree with itself.
    SelectRefusal::TiedDisagrees {
        name: Box::new(if ca.len() > 1 { na.clone() } else { nb.clone() }),
        matched,
        candidates: total,
    }
}

// ---------------------------------------------------------------
// (c) Declare — sugar over the shipped vocabulary.
// ---------------------------------------------------------------

/// Why the declare sugar refused.
#[derive(Debug)]
pub enum DeclareError {
    /// No findings were passed: an empty `Declare` node records no
    /// intent and would only pretend something was declared — refused
    /// loudly rather than inserted silently.
    NoFindings,
    /// The document edit itself refused (a stale finding naming a
    /// node the document no longer has, for instance).
    Edit(EditError),
    /// The insert applied but minted no id — an `apply` contract
    /// violation (`InsertNode` always mints); surfaced typed rather
    /// than panicking.
    NoMintedId,
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in the declare sugar's own vocabulary — findings, the
// node it would insert, the id an insert owes. The `Edit` arm forwards
// the document edit's own refusal, which already carries its node and
// its recourse; re-stating it here would give one refusal two voices.
impl core::fmt::Display for DeclareError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoFindings => f.write_str(
                "declare: no findings were passed — an empty declaration records no intent and \
                 would only pretend something was declared; pass the findings the inspection \
                 actually returned",
            ),
            Self::Edit(error) => write!(f, "declare: the document edit refused: {error}"),
            Self::NoMintedId => f.write_str(
                "declare: the insert applied but minted no node id — an insert always mints \
                 one, so this is a kernel bug",
            ),
        }
    }
}

impl core::error::Error for DeclareError {}

/// The [`Node::Declare`] payload for explicitly-passed findings — the
/// buildable rung under [`declare`]/[`declare_all`] for callers that
/// record their own edit logs (the corpus `Recorder`, an undo stack).
///
/// # Errors
///
/// [`DeclareError::NoFindings`] on an empty slice.
pub fn declare_node<P>(findings: &[FlushFinding]) -> Result<Node<P>, DeclareError> {
    if findings.is_empty() {
        return Err(DeclareError::NoFindings);
    }
    Ok(Node::Declare {
        // The finding's CLASS travels with its pair. Dropping it here
        // was a live bug for as long as the class existed: the node
        // then meant "declare this pair" with no record of WHAT was
        // declared, and the consuming boolean re-defaulted it to the
        // conformal class — so a `Tangent` finding, declared, would
        // have been verified against the `Rest` table.
        pairs: findings.iter().map(|f| (f.pair.clone(), f.class)).collect(),
    })
}

/// Declares ONE inspected finding: inserts a [`Node::Declare`] with
/// its pair and returns the edited document plus the Declare node's
/// id, for the caller to wire into the consuming Boolean's `declare`
/// input. Sugar over shipped vocabulary — nothing here detects
/// (GS-Q3's no-fusion boundary: findings reach this door as VALUES
/// the caller already held).
///
/// # Errors
///
/// [`DeclareError::Edit`] if the insert refuses.
pub fn declare<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    finding: &FlushFinding,
    tol: Tol,
) -> Result<(Doc<P>, RecipeNodeId), DeclareError> {
    declare_all(doc, core::slice::from_ref(finding), tol)
}

/// Declares a SET of inspected findings in one [`Node::Declare`] —
/// the many-pair case the round-2 amendment rules legal (the boundary
/// is fusion, not arity). Same contract as [`declare`].
///
/// # Errors
///
/// [`DeclareError::NoFindings`] on an empty slice,
/// [`DeclareError::Edit`] if the insert refuses.
pub fn declare_all<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    findings: &[FlushFinding],
    tol: Tol,
) -> Result<(Doc<P>, RecipeNodeId), DeclareError> {
    let node = declare_node(findings)?;
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).map_err(DeclareError::Edit)?;
    let id = applied.record.minted.ok_or(DeclareError::NoMintedId)?;
    Ok((applied.doc, id))
}
