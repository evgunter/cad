//! **The flush detector at the body seat** (`docs/VERB-SEAT-DESIGN.md`
//! §1 S3; `docs/SELECT-DESIGN.md` §3) — the detect half of the
//! detect/declare protocol as a pure function of two [`Body`]s, and
//! the [`BooleanDeclarations`] sugar beside it.
//!
//! [`BooleanDeclarations`] is what the declared boolean doors take,
//! and this module is where a caller holding two bodies and no
//! document can come by one. The document layer's detector speaks
//! stable names and delegates its per-pair test HERE; a kernel-direct
//! consumer asks the same question through the same one
//! implementation. **Findings are names at the document door, keys at
//! the body door, one verifier under both** — the `ContactClass`
//! layering precedent (defined lowest, re-exported upward), applied to
//! the finding vocabulary.
//!
//! # The anti-twin rule (SELECT-DESIGN §3b): the detector IS the
//! verifier
//!
//! The detector has NO predicate triple of its own. It enumerates
//! candidate pairs and asks [`flush_pair_relation`], and that call
//! **converges with verify-at-use one link down** — which is the
//! honest statement of the property, and worth stating as a chain
//! because the two are not the same function:
//!
//! 1. this module calls `flush_pair_relation`, which builds the
//!    sense-folded plane descriptions and the oriented identity
//!    evidence and hands them to `oriented_plane_eq`;
//! 2. verify-at-use (`verify_declared_pairs`, and the op's front-door
//!    contact check) calls the kind-generalized
//!    [`carrier_pair_relation`](crate::boolean::carrier_pair_relation),
//!    which builds the SAME description from the same face sense and
//!    the same identity record;
//! 3. `carrier_eq`'s `(Plane, Plane)` arm delegates to
//!    `oriented_plane_eq_verdict` — the function
//!    `oriented_plane_eq` is a projection of. **That delegation is
//!    the load-bearing link**: one verdict ladder, one set of
//!    `decide` sites, one verification arm, so a pair this detector
//!    calls flush cannot be a pair the declared rung then contradicts.
//!
//! Consequences, all deliberate:
//!
//! - detect-then-declare can never disagree with verify-at-use, by the
//!   convergence above rather than by care;
//! - detection's decisions go through the funnel at the VERIFIER'S
//!   sites (`bool_plane_parallel` / `bool_plane_orient` /
//!   `bool_plane_offset`) — the detector mints no site of its own and
//!   owes no ledger row. It interprets nothing the verifier doesn't.
//!
//! # Findings are DEFINITE; in-band pairs refuse
//!
//! A [`FlushFinding`] is only ever definite. A pair whose verify-door
//! margin lands inside the ambiguity band is neither reported nor
//! silently dropped: the query refuses with
//! [`FlushRefusal::PairInBand`] naming the pair.
//!
//! Definite is a claim about the GEOMETRY, and only that. A finding
//! says "declared, this pair verifies"; it does not say the op will
//! build. A true declaration still meets whatever capability frontier
//! lies downstream of verification — a `SameOriented` wall pair
//! declared on a stepped mate verifies and then refuses at
//! `RestZipUnsupported`, typed, at the zip. Detection cannot see those
//! frontiers and does not claim to.
//!
//! The refusal names ONE pair — the FIRST indeterminate pair in the
//! enumeration order below — and abandons the walk there. It is not a
//! survey of every in-band pair a body pair holds: the honesty
//! obligation is that an undecidable pair can never be silently
//! included or dropped, and one named pair discharges it. (The
//! document-seat detector refuses the same way, on the first such
//! pair its own name-order walk meets, and this door is deliberately
//! identical to it.)
//!
//! # Scope: planar `Rest`, v1
//!
//! The detector detects what [`flush_pair_relation`] verifies: the
//! coincident-plane contact class. The `Rest` verify ladder itself is
//! wider — [`carrier_pair_relation`](crate::boolean::carrier_pair_relation)
//! carries sphere, cylinder and torus rungs and is what
//! `verify_declared_pairs` runs — so a curved cosurface pair is
//! DECLARABLE today and not DETECTABLE; that scope step is a widening
//! of this door, not of a verify table, and it is deliberately not
//! taken here (`docs/SEAT-LOG.md`). `Tangent` findings wait on a
//! locus the verifier can check
//! ([`tangent_locus`](crate::boolean::tangent_locus)), per
//! SELECT-DESIGN §3's closing note.
//!
//! # The no-fusion boundary (SELECT-DESIGN GS-Q3, RULED)
//!
//! [`declare`] and [`declare_all`] take findings the caller already
//! HOLDS. No door here detects and declares in one call: findings pass
//! through user-visible hands as values, which is the enforceable
//! intent-recording property, and C4's verify-at-use backstops lies
//! either way.

use geom_core::{Band, BandError, Decide, Indeterminate, MarginDiag, Tol};

use crate::body::Body;
use crate::boolean::{
    BooleanDeclarations, FacePairDeclaration, PlaneEqError, PlaneRelation, flush_pair_relation,
};
use crate::contact::ContactClass;
use crate::entity::FaceKey;
use crate::query::all_faces;

/// Which rung of the verify ladder decided a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushRung {
    /// Rung 1: both descriptions carry the same recipe source (N6) —
    /// syntactic identity, zero numerics.
    SharedSource,
    /// Rung 3's decided margins: definitely parallel, definitely
    /// zero offset (the geometric trilean's coincident arm).
    DecidedCoincident,
}

/// The definite evidence a finding carries — exactly what the C4
/// verify door reported, nothing re-derived (the anti-twin rule
/// constrains evidence to the door's own verdict).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlushEvidence {
    /// The door's definite verdict: [`PlaneRelation::SameOpposite`]
    /// (resting contact) or [`PlaneRelation::SameOriented`] (flush
    /// walls, the merge-stage flavor). Never `Distinct` — a distinct
    /// pair is no finding at all.
    pub relation: PlaneRelation,
    /// Which rung decided.
    pub rung: FlushRung,
}

/// One flush-plane finding: "this face pair would verify as declared
/// contact" — a VALUE, inspectable, never itself a declaration
/// (SELECT-DESIGN §3a).
///
/// `P` is the PAIR VOCABULARY of the seat that produced it: face keys
/// at this seat ([`FacePairFinding`]), stable names at the document
/// seat. One finding type, one verifier under both — the vocabulary of
/// the pair is the only thing a seat gets to choose.
#[derive(Debug, Clone, PartialEq)]
pub struct FlushFinding<P> {
    /// The face pair. `.0` is from the detector's `a` operand, `.1`
    /// from `b`.
    pub pair: P,
    /// The contact class the pair would verify as.
    pub class: ContactClass,
    /// The definite verdict the verify door reported.
    pub evidence: FlushEvidence,
}

/// A finding at the body seat: the pair is arena keys, `.0` in `a`'s
/// arena and `.1` in `b`'s.
pub type FacePairFinding = FlushFinding<(FaceKey, FaceKey)>;

/// Why [`find_flush_candidates`] refused.
#[derive(Debug)]
pub enum FlushRefusal {
    /// The ambient tolerance yields no band.
    Band(BandError),
    /// A pair's verify-door margin is indeterminate: the pair is
    /// neither reported as definite nor silently dropped — the whole
    /// query refuses, naming it.
    PairInBand {
        /// The pair the door could not decide.
        pair: (FaceKey, FaceKey),
        /// The verifier's own diagnostic, carrying its funnel site.
        source: Indeterminate,
    },
}

impl core::fmt::Display for FlushRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Band(error) => write!(f, "flush detection: {error}"),
            Self::PairInBand { pair, source } => write!(
                f,
                "flush detection: face pair {:?}/{:?} is neither definitely flush nor definitely \
                 apart ({source}) — a finding is only ever definite, so the pair is named rather \
                 than reported or dropped; separate the geometry or widen the tolerance",
                pair.0, pair.1
            ),
        }
    }
}

impl core::error::Error for FlushRefusal {}

/// **One candidate pair through the verify door**, in
/// candidate-generation mode: `Ok(Some(_))` = definitely flush (with
/// the door's relation and deciding rung), `Ok(None)` = definitely not
/// a candidate (a non-planar face, or definitely distinct planes),
/// `Err` = the door could not decide definitively (in-band,
/// escalated, or poisoned).
///
/// This is the rung both seats delegate to: the body seat's
/// [`find_flush_candidates`] enumerates over it, and the document
/// seat's name-level detector runs it per resolved candidate
/// combination while keeping its own name-flavored half (name
/// resolution, the tie trilean, refusal payloads) upstairs.
///
/// Everything — descriptions, oriented sources, AND the verification
/// arm — comes from [`flush_pair_relation`], whose verdict ladder is
/// the one verify-at-use reaches through `carrier_eq`'s plane
/// delegation (module docs, the three-link chain). ONE call, in
/// `declared: false` mode: its `Undeclared` refusal with the
/// verifier's definite-zero encoding ([`MarginDiag::Invalid`]) is
/// precisely "would verify if declared", and the refusal itself
/// carries the orientation the ladder decided — the same
/// `bool_plane_orient` verdict the declared rung re-decides
/// deterministically at use, so the carried relation and the
/// verify-at-use verdict cannot disagree.
///
/// # Errors
///
/// The verifier's [`Indeterminate`], carrying the funnel site that
/// could not decide.
pub fn pair_finding<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<Option<FlushEvidence>, Indeterminate> {
    let Some(relation) = flush_pair_relation(a, fa, b, fb, false, band) else {
        // Not a planar pair: not a v1 candidate, honestly.
        return Ok(None);
    };
    match relation {
        Ok(PlaneRelation::Distinct) => Ok(None),
        // Rung 1 fired: same recipe source, exact verdict.
        Ok(relation) => Ok(Some(FlushEvidence {
            relation,
            rung: FlushRung::SharedSource,
        })),
        Err(PlaneEqError::Undeclared { diag, relation }) => {
            if matches!(diag.margin, MarginDiag::Invalid) {
                // The verifier's definite-zero-offset encoding: the
                // pair would verify if declared, with the orientation
                // the refusal itself carries. A NaN-poisoned margin
                // shares that encoding, and the detector takes the
                // verifier's encoding as-is (anti-twin: it interprets
                // nothing the verifier doesn't) — C4's verify-at-use
                // is the backstop for geometry broken this early.
                match relation {
                    PlaneRelation::SameOriented | PlaneRelation::SameOpposite => {
                        Ok(Some(FlushEvidence {
                            relation,
                            rung: FlushRung::DecidedCoincident,
                        }))
                    }
                    // Unreachable by the variant's contract (an
                    // Undeclared refusal never carries `Distinct`);
                    // typed, never silent.
                    PlaneRelation::Distinct => Err(diag),
                }
            } else {
                // In-band coincidence: not definite, not droppable.
                Err(diag)
            }
        }
        Err(PlaneEqError::Escalated(diag)) => Err(diag),
        // Unreachable with `declared: false`; kept typed.
        Err(PlaneEqError::Contradicted(diag)) => Err(diag),
    }
}

/// A finding from a pair and the evidence the verify door reported —
/// **the one place the reported CLASS is minted**, for either seat.
///
/// `Rest` is not a default here, it is the whole v1 detector: this
/// door reports coincident-plane contact and nothing else. When a
/// second class becomes detectable, this function is where it is
/// decided, once, rather than at each seat's own push.
#[must_use]
pub fn finding<P>(pair: P, evidence: FlushEvidence) -> FlushFinding<P> {
    FlushFinding {
        pair,
        class: ContactClass::Rest,
        evidence,
    }
}

/// **The cross-body flush-plane candidates between two bodies** — the
/// C4 verifier run in candidate-generation mode (module docs).
///
/// Every face of `a` is asked against every face of `b` through
/// [`pair_finding`]. Findings come back in ARENA ORDER — `a`'s faces
/// in slot-index order (D9), `b`'s within each — which is the order
/// the enumeration walks and is deterministic given identical
/// construction history. They are only ever DEFINITE.
///
/// This door detects; it declares nothing. Feed what you have
/// inspected to [`declare_all`] (GS-Q3's no-fusion boundary).
///
/// # Errors
///
/// [`FlushRefusal::PairInBand`] when a pair's verify-door margin is
/// indeterminate (never silently included or dropped),
/// [`FlushRefusal::Band`] if the ambient tolerance is broken.
pub fn find_flush_candidates<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    tol: Tol,
) -> Result<Vec<FacePairFinding>, FlushRefusal> {
    let band = Band::linear(tol).map_err(FlushRefusal::Band)?;
    let (fa, fb) = (all_faces(a), all_faces(b));
    let mut out = Vec::new();
    for &ka in &fa {
        for &kb in &fb {
            let evidence =
                pair_finding(a, ka, b, kb, band).map_err(|source| FlushRefusal::PairInBand {
                    pair: (ka, kb),
                    source,
                })?;
            if let Some(evidence) = evidence {
                out.push(finding((ka, kb), evidence));
            }
        }
    }
    Ok(out)
}

/// The [`BooleanDeclarations`] declaring ONE inspected finding — the
/// value a declared boolean door takes, from a finding the caller
/// already held.
#[must_use]
pub fn declare(finding: &FacePairFinding) -> BooleanDeclarations {
    declare_all(core::slice::from_ref(finding))
}

/// The [`BooleanDeclarations`] declaring a SET of inspected findings
/// (the many-pair arm the no-fusion amendment rules legal: the
/// boundary is fusion, not arity).
///
/// An empty slice declares nothing and is exactly
/// [`BooleanDeclarations::none`] — at this seat that is a legal value
/// with a meaning (the plain two-argument ops pass it), which is what
/// separates it from the document seat's `Node::Declare`, where an
/// empty node would record the LOOK of intent with no content and is
/// refused.
#[must_use]
pub fn declare_all(findings: &[FacePairFinding]) -> BooleanDeclarations {
    BooleanDeclarations {
        coincident_faces: findings
            .iter()
            .map(|f| FacePairDeclaration::new(f.pair.0, f.pair.1, f.class))
            .collect(),
        ..BooleanDeclarations::none()
    }
}
