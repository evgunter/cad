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
//!   the SHIPPED [`DocEdit::SetDeclare`] edit: they take
//!   explicitly-passed findings and write them into a live Boolean's or
//!   Union's declared pairs — [`declare`] adds its one finding's pair
//!   to the pairs the node already declares, [`declare_all`] sets the
//!   whole list.
//! - **The menu** — the boolean's undeclared-coincidence refusal names
//!   exactly two arms: declare the found class (→ the sugar) or move
//!   the geometry. NO absorb arm (the #256 ruling applied to contact).
//!
//! # The no-fusion boundary (GS-Q3, RULED)
//!
//! Both arities ship — the ruled boundary is FUSION, not arity — and
//! they are two different edits. `declare(node, finding)` ADDS: a
//! refusal names one contact at a time, so following the refusal
//! finding by finding converges on a node that declares every contact
//! it meets. `declare_all(node, findings)` REPLACES: the whole list,
//! `SetDeclare`'s own shape. A fused detect-and-declare
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
//! inside [`topo::boolean::carrier_pair_relation`] under it, which is
//! the very door verify-at-use calls, asked in its `declared: false`
//! posture (the argument is stated once, in [`topo::flush`]'s module
//! docs, and not restated here). Consequences, all deliberate:
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
//!   sites (`bool_plane_*` on the planar rung, `carrier_sphere_*` /
//!   `carrier_cyl_*` / `carrier_torus_*` on the curved ones) — no
//!   `sel_flush_*` site exists, no new ledger row is owed, and
//!   GS-Q1's K-census participation is automatic through those names.
//!   The detector interprets nothing the verifier doesn't.
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
//! # What a finding's class means here
//!
//! Cosurface pairs on every carrier the ladder verifies — plane,
//! sphere, cylinder and torus — are the whole detector, and a peg's
//! wall in its bore is reported exactly as two flush plates' faces
//! are. The class is read off the orientation the verifier decided:
//! [`PlaneRelation::SameOpposite`] is a `Rest` contact (opposed
//! material sides — the REST lane's zip), and
//! [`PlaneRelation::SameOriented`] a continuation
//! ([`BooleanCoincidence::Continuation`]: two stacked parts' outer
//! walls, which the union merges). Both are exactly the pairs the
//! declared rung verifies under that class. `Tangent`/`Fit` findings reuse this shape when their
//! demand arrives — the `class` field is the reserved slot, not a
//! `flush: bool` — and tangency waits on a locus the verifier can
//! check, which is a different kind of gap from the one the curved
//! `Rest` rungs closed.
//!
//! # Documented residuals
//!
//! - The verifier encodes its definite-zero coincidence verdict and a
//!   NaN-poisoned margin with the same `MarginKind::Invalid`
//!   diagnostic; the detector takes the verifier's encoding as-is
//!   (anti-twin: it interprets nothing the verifier doesn't), so a
//!   NaN-poisoned pair — geometry that is broken well before
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
use crate::edit::{Applied, DocEdit, EditError, apply};
use crate::eval::{Evaluation, NodeValue};
use crate::names::geompred::SelectRefusal;
use crate::names::interrogate;
use crate::names::interrogate::InterrogateError;
use crate::names::role::{EntityKind, StableName};
use crate::names::table::{EntityKey, EntityRef, Entry};
use crate::node::{DeclaredPair, RecipeNodeId, SitedRef};

/// The contact class a declaration asserts (CONTACT-DESIGN C4) — a
/// RE-EXPORT of the kernel's vocabulary, never a parallel enum.
///
/// The type is defined in `topo` because the boolean's own contact
/// refusals must carry the same words the detector produces
/// (SELECT-DESIGN §3d, "one vocabulary end-to-end") and `topo` cannot
/// depend on this crate. Everything above re-exports it; nothing
/// redefines it.
pub use topo::{BooleanCoincidence, ContactClass};

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

/// One flush finding at the DOCUMENT seat: "this cross-body face
/// pair would verify as declared contact" — a VALUE, inspectable,
/// never itself a declaration (SELECT-DESIGN §3a).
///
/// The kernel's [`topo::flush::FlushFinding`] with this seat's pair
/// vocabulary: SITED names, never keys (G1). `.0` is from the
/// detector's `a` node, `.1` from `b`. The body seat's finding
/// ([`topo::flush::FacePairFinding`]) is the same type over face keys
/// — findings are names at the document door, keys at the body door,
/// one verifier under both.
///
/// **Each side is a [`SitedRef`]**, and that is what makes a finding
/// declarable without re-deriving anything: a declared pair names
/// sited entities (DM4), and a finding already knows where each of
/// its names was read — the query's two nodes, or the refusing
/// node's operands. [`declared_pairs`] therefore copies the pair
/// through, and a carried same-operand finding sites both sides at
/// the one operand that holds them, which no caller downstream could
/// have recovered from the names alone.
pub type FlushFinding = topo::flush::FlushFinding<(SitedRef, SitedRef)>;

// ---------------------------------------------------------------
// (a) Detect.
// ---------------------------------------------------------------

/// **The cross-body flush candidates between `a`'s and `b`'s
/// outputs, as of THIS evaluation** — the C4 verifier run in
/// candidate-generation mode (module docs; SELECT-DESIGN §3a/b).
///
/// Findings come back in canonical order (sorted by name pair) and
/// are only ever DEFINITE.
///
/// A node with no value refuses rather than answering empty: this is
/// an inspection door, and "no flush pair" about a node that did not
/// build would read as a fact about its geometry.
///
/// # Errors
///
/// [`SelectRefusal::NodeHasNoValue`] when `a` or `b` has no value,
/// carrying its standing; [`SelectRefusal::AcrossSpaces`] when they
/// live in different spaces ([`Evaluation::across_spaces`]);
/// [`SelectRefusal::PairInBand`] when a pair's verify-door margin is
/// indeterminate (never silently included or dropped),
/// [`SelectRefusal::TiedDisagrees`] when a tied name's candidates
/// disagree (GS-Q4), [`SelectRefusal::Unreadable`] on a name-table
/// entry that does not resolve into its node's payload,
/// [`SelectRefusal::Band`], carrying the band constructor's own
/// diagnostic, if the ambient tolerance yields no usable band.
pub fn find_flush_candidates<T: Decide>(
    ev: &Evaluation<T>,
    a: RecipeNodeId,
    b: RecipeNodeId,
    tol: Tol,
) -> Result<Vec<FlushFinding>, SelectRefusal> {
    let va = ev.usable(a).map_err(SelectRefusal::NodeHasNoValue)?;
    let vb = ev.usable(b).map_err(SelectRefusal::NodeHasNoValue)?;
    if let Some((group, cause)) = ev.across_spaces(a, b) {
        return Err(SelectRefusal::AcrossSpaces { group, cause });
    }
    let band = Band::linear(tol)?;
    let fa = face_candidates(va)?;
    let fb = face_candidates(vb)?;
    let mut out = Vec::new();
    for (na, ca) in &fa {
        for (nb, cb) in &fb {
            // The query's two nodes ARE the sites: a name from
            // `a`'s table is read at `a`, one from `b`'s at `b`.
            if let Some(finding) = pair_verdict((a, na), ca, (b, nb), cb, band)? {
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
    (at_a, na): (RecipeNodeId, &StableName),
    ca: &[(&Body<T>, FaceKey)],
    (at_b, nb): (RecipeNodeId, &StableName),
    cb: &[(&Body<T>, FaceKey)],
    band: Band,
) -> Result<Option<FlushFinding>, SelectRefusal> {
    let mut relation: Option<PlaneRelation> = None;
    let mut all_shared_source = true;
    let mut matched = 0usize;
    let total = ca.len() * cb.len();
    for &(ba, fa) in ca {
        for &(bb, fb) in cb {
            let verdict = pair_finding(ba, fa, bb, fb, band).map_err(|undecided| {
                let source = undecided.diag();
                SelectRefusal::PairInBand {
                    pair: Box::new((na.clone(), nb.clone())),
                    predicate: source.predicate.unwrap_or("carrier_pair_relation"),
                    source,
                }
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
        (m, Some(relation)) if m == total => finding(
            (
                SitedRef::new(at_a, na.clone()),
                SitedRef::new(at_b, nb.clone()),
            ),
            FlushEvidence {
                relation,
                rung: if all_shared_source {
                    FlushRung::SharedSource
                } else {
                    FlushRung::DecidedCoincident
                },
            },
        )
        .map(Some)
        .map_err(SelectRefusal::DistinctFinding),
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
    /// [`declare_all`] was passed no findings: a declaration of nothing
    /// records no intent and would only pretend something was declared
    /// — refused loudly rather than set silently. Clearing a
    /// declaration is [`DocEdit::SetDeclare`] with an empty list, said
    /// in so many words; the list builder [`declared_pairs`] builds an
    /// empty list without complaint, because an empty list is a legal
    /// declaration (none) and only the door that is asked to declare
    /// something needs a finding.
    NoFindings,
    /// The document edit itself refused (a stale finding naming a
    /// node the document no longer has, or a node that is not a
    /// Boolean or a Union, for instance).
    Edit(EditError),
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in the declare sugar's own vocabulary. The `Edit` arm
// forwards the document edit's problem, and states the recourse
// itself: the caller passed findings and a node, not the edit the
// edit door's recourse is about.
impl core::fmt::Display for DeclareError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoFindings => f.write_str(
                "declare: no findings were passed — an empty declaration records no intent and \
                 would only pretend something was declared; pass the findings the inspection \
                 actually returned",
            ),
            Self::Edit(error) => {
                write!(f, "declare: the document edit refused: {}", error.problem())?;
                match error {
                    // A finding names what the evaluation it came from
                    // held; a document edited since may not hold it.
                    EditError::DeclareNamesMissingNode { .. }
                    | EditError::NameStepNeverMinted { .. }
                    | EditError::ReadSiteMissingNode { .. } => f.write_str(
                        ". Recourse: declare findings inspected from this document as it now \
                         stands",
                    ),
                    // A finding names the two operands it was inspected
                    // between; another node's operands are not those.
                    EditError::DeclaredSiteNotAnOperand { .. }
                    | EditError::DeclaredNameNotUpstream { .. } => f.write_str(
                        ". Recourse: declare it on the Boolean or Union whose operands the \
                         finding was inspected between",
                    ),
                    EditError::UnknownNode { .. } | EditError::SetDeclareOnNonDeclaring { .. } => {
                        f.write_str(". Recourse: declare on a live Boolean or Union")
                    }
                    // The edit touches nothing else a door checks.
                    _ => write!(f, ". {}", geom_core::KERNEL_DEFECT_ENDING),
                }
            }
        }
    }
}

impl core::error::Error for DeclareError {}

/// The declared pairs for explicitly-passed findings, each with the
/// class its finding carries — the buildable rung under
/// [`declare`]/[`declare_all`] for callers that build their own edits
/// (a new Boolean's `declare`, a [`DocEdit::SetDeclare`] they record
/// themselves). No findings build the empty list, which declares
/// nothing.
#[must_use]
pub fn declared_pairs(findings: &[FlushFinding]) -> Vec<DeclaredPair> {
    findings.iter().map(|f| (f.pair.clone(), f.class)).collect()
}

/// ADDS one inspected finding's pair to the declared pairs of the live
/// Boolean or Union `node`, keeping every pair it declares already —
/// the door an undeclared-contact refusal's recourse names. A refusal
/// carries one contact; following each refusal with this converges on
/// a node that declares every contact it meets, where a whole-list
/// replace would trade one contact for the next forever.
///
/// A pair on the same two sides as one the node declares already
/// replaces it in place (the finding is the later inspection), so the
/// list never holds a side pair twice. The edit is the whole-list
/// [`DocEdit::SetDeclare`] of the result.
///
/// # Errors
///
/// [`DeclareError::Edit`] if the edit refuses — the node not live, or
/// not a Boolean or a Union, among them.
pub fn declare<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    finding: &FlushFinding,
    tol: Tol,
) -> Result<Applied<P>, DeclareError> {
    let mut pairs = doc
        .node(node)
        .map(|n| n.declared_pairs().to_vec())
        .unwrap_or_default();
    let added = (finding.pair.clone(), finding.class);
    match pairs.iter_mut().find(|(sides, _)| *sides == added.0) {
        Some(held) => *held = added,
        None => pairs.push(added),
    }
    set_declared(doc, node, pairs, tol)
}

/// Sets a SET of inspected findings as the declared pairs of the live
/// Boolean or Union `node`, replacing whatever it declared before — the
/// whole-list [`DocEdit::SetDeclare`], so nothing is inferred about the
/// old list. Sugar over shipped vocabulary — nothing here detects
/// (GS-Q3's no-fusion boundary: findings reach this door as VALUES the
/// caller already held). The acceptance is returned whole: the edited
/// document, its record and its maintenance, as one [`Applied`].
///
/// # Errors
///
/// [`DeclareError::NoFindings`] on an empty slice,
/// [`DeclareError::Edit`] if the edit refuses.
pub fn declare_all<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    findings: &[FlushFinding],
    tol: Tol,
) -> Result<Applied<P>, DeclareError> {
    if findings.is_empty() {
        return Err(DeclareError::NoFindings);
    }
    set_declared(doc, node, declared_pairs(findings), tol)
}

/// The [`DocEdit::SetDeclare`] both declare doors end in.
fn set_declared<P: Clone + crate::ProfilePayload>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    pairs: Vec<DeclaredPair>,
    tol: Tol,
) -> Result<Applied<P>, DeclareError> {
    // A declaration moves no group's root: the maintenance never asks
    // the reach, and the refusing one is the honest value here.
    apply(
        doc,
        &DocEdit::SetDeclare { node, pairs },
        tol,
        &crate::mate::RefusingReach,
    )
    .map_err(DeclareError::Edit)
}
