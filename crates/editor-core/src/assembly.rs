//! **The assembly at-rest gate** (ASSEMBLY-DESIGN A5; ASM-R2b D-2/D-3).
//!
//! A5 says an assembly's validity is "mate-minted records + census +
//! two-directional certification, exactly the boolean 3′ shape" — an
//! AT-REST door, not a zip. This module is the two halves that makes
//! true:
//!
//! - **Minting (D-2)**: every solved mate's declaration becomes a
//!   kernel contact record in the gathered product's record set. Same
//!   type as the boolean wrapper's records, no adapter — A3's ratified
//!   sentence, and C4's second home landing. A DECLARING (non-tree)
//!   mate mints IDENTICALLY to a determining one: minting is
//!   declaration, not verification, and a mate that placed nothing
//!   still says what touches what.
//! - **Verification (D-3)**: the minted set goes through the kernel's
//!   own tier-3′ door, [`topo::validate_pseudomanifold`]. Declared
//!   contacts certify through the per-class doors; undeclared ones are
//!   the hard error — F1's scan-to-bless ban, now executing ACROSS the
//!   document seam exactly as it does within it.
//!
//! # What this module does NOT do
//!
//! It runs no predicate of its own. Every geometric decision here is
//! the kernel's, called as-is: this layer resolves names to faces,
//! mints records, and ATTRIBUTES the kernel's refusals back to the
//! mate that authored the declaration. A refusal the kernel does not
//! make is not made here either — there is no second opinion about
//! whether two faces touch, which is the whole point of minting into
//! the kernel's own currency instead of an assembly-side shadow.
//!
//! # The two frontiers, and where each one is written
//!
//! This door admits less than the shape of its types suggests. Both
//! narrowings are values, not sentences here, because a boundary a
//! caller can only read about is one they cannot tell apart from a
//! defect when it fires:
//!
//! - **Which classes mint** is [`crate::mate::class_admission`], the
//!   one table this door and the solve door both read.
//! - **What a declared contact can reach** is
//!   [`AssemblyError::Uncertified`], the arm every declared
//!   cross-instance pair lands in today, and which a caller must match
//!   apart from the verdicts against their own document.
//!
//! Each says why at its own definition. The kernel's finding passes
//! straight through either way — stated, never swallowed, never
//! re-labelled — and the declaration still does its job: it is what
//! suppresses the F1 `UndeclaredContact` refusal.

use geom_core::Decide;
use topo::{ContactRecords, FaceKey, PatchContact, PropsQuadLane, ValidationError};

use crate::doc::Doc;
use crate::eval::{Evaluation, NodeResult, ValuePayload};
use crate::mate::{ClassAdmission, ContactClass, MateSide, class_admission};
use crate::names::{EntityKey, EntityKind, Entry, NameTable, StableName};
use crate::node::{Node, RecipeNodeId};
use crate::product::{Product, ProductError, product_recorded};
use geom_core::Tol;

/// One mate's minted declaration: the mate that authored it, both of
/// its references, the class it asserts, and the PRODUCT faces the
/// references resolved to.
///
/// Kept beside the record set because the kernel's record types carry
/// arena keys and nothing else — attribution back to a mate is this
/// layer's bookkeeping, not a field the kernel should grow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintedDeclaration {
    /// The mate node that authored the declaration.
    pub mate: RecipeNodeId,
    /// The `a` reference.
    pub a: StableName,
    /// The `b` reference.
    pub b: StableName,
    /// The class it asserts.
    pub class: ContactClass,
    /// The product faces the two references resolved to.
    pub faces: (FaceKey, FaceKey),
}

/// A validated assembly: the gathered product, its names, the record
/// set the at-rest gate certified, and what each mate minted into it.
#[derive(Debug)]
pub struct Assembly<T: Decide> {
    /// The gathered product body.
    pub body: topo::Body<T>,
    /// Its stable names.
    pub names: NameTable,
    /// The certified record set: the parts' own carried declarations
    /// (D-1) PLUS this document's minted mate declarations (D-2).
    pub contacts: ContactRecords,
    /// One row per mate, in document order.
    pub minted: Vec<MintedDeclaration>,
}

/// Why a mate reference did not resolve to a product face.
///
/// The vocabulary is `resolve_declarations`' — the same three shapes a
/// `Declare` node's names refuse with — stated separately because the
/// SUBJECT differs (a mate's reference resolves against the assembly's
/// product table, not a boolean operand's).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusedRef {
    /// The name's minting node is not in this document.
    NodeGone,
    /// No entity of the product answers to the name.
    Vanished,
    /// Several entities answer to it — a mate declaration must name
    /// ONE face, and a tie is never broken by picking.
    Ambiguous {
        /// How many entities the tie holds.
        width: u32,
    },
    /// The name resolves, but not to a FACE. A mate's declaration is a
    /// face-pair contact; an edge or vertex reference is a different
    /// statement, refused rather than widened.
    NotAFace {
        /// What it did name.
        kind: EntityKind,
    },
}

/// **What a kernel finding says about the document's declarations.**
///
/// One field rather than a mate plus a flag: the relation and the
/// declaration it names are decided together, in `attribute`'s single
/// dispatch, and cannot disagree.
#[derive(Debug, Clone, PartialEq)]
pub enum Attribution {
    /// The kernel REFUTED this declaration — the faces do not meet as
    /// it says. A finding against the document.
    Refuted(MintedDeclaration),
    /// The census DECLINED to certify this declaration: it has no
    /// certifier lane for a face the declaration names, so the
    /// declaration was neither certified nor contradicted and nothing
    /// was decided about the geometry either way.
    ///
    /// **The finding names ONE face and a declaration names two**, so
    /// WHICH declaration this is can be settled by arena order rather
    /// than by the census's own subject ([`attribute`] says how). What
    /// the relation itself guarantees — and all the two
    /// [`AssemblyError`] arms read — is that nothing was refuted.
    Declined(MintedDeclaration),
    /// The finding names no declaration. An UNDECLARED contact is
    /// exactly this — by definition no mate authored it, which is what
    /// makes it the F1 hard error.
    Unattributed,
}

impl Attribution {
    /// The declaration named, for a finding that names one.
    pub fn declaration(&self) -> Option<&MintedDeclaration> {
        match self {
            Self::Refuted(m) | Self::Declined(m) => Some(m),
            Self::Unattributed => None,
        }
    }
}

// The finding-subject vocabulary ([`crate::finding`]): what a rendered
// at-rest finding is ABOUT — the mate a user can act on and the
// relation the kernel's arm decided — never the enum's guts. The
// kernel's own finding is the STORY and rides separately
// ([`AtRestFinding`]'s `Display`).
impl core::fmt::Display for Attribution {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Refuted(m) => write!(
                f,
                "mate {}'s declared {} contact, refuted",
                m.mate.0,
                m.class.name()
            ),
            Self::Declined(m) => write!(
                f,
                "mate {}'s declared {} contact, uncertified",
                m.mate.0,
                m.class.name()
            ),
            Self::Unattributed => f.write_str("no declaration answers for this finding"),
        }
    }
}

// One at-rest finding through the document layer's one sink
// ([`crate::finding`]): the attribution is the subject, the kernel's
// finding — FORWARDED verbatim through its own `Display`, never
// restated — is the story, and the recourse is `""` because the
// kernel's tier-3′ messages already end in their own (the contact
// arms carry `topo`'s two-armed menu; the structural arms carry their
// own levers). Appending a document-layer sentence on top would
// render two recourses, or a generic one — both forbidden.
impl crate::finding::Finding for AtRestFinding {
    fn subject(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.attribution)
    }

    fn story(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.error)
    }

    fn recourse(&self) -> &str {
        ""
    }
}

impl core::fmt::Display for AtRestFinding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::finding::compose(f, self)
    }
}

/// One at-rest refusal: the kernel's finding, and what it says about
/// what the mates declared.
#[derive(Debug, Clone, PartialEq)]
pub struct AtRestFinding {
    /// Which declaration the finding names, and in what relation.
    pub attribution: Attribution,
    /// The kernel's own finding, verbatim.
    pub error: ValidationError,
}

/// Why the assembly gate refused.
#[derive(Debug)]
pub enum AssemblyError {
    /// The gather itself refused.
    Product(Box<ProductError>),
    /// A mate's reference did not resolve to a product face.
    Reference {
        /// The mate.
        mate: RecipeNodeId,
        /// Which side.
        side: MateSide,
        /// The reference.
        name: Box<StableName>,
        /// Why it did not resolve.
        why: RefusedRef,
    },
    /// The mate's class mints no record at rest — the mint door's half
    /// of [`crate::mate::class_admission`].
    NoAtRestRecord {
        /// The mate.
        mate: RecipeNodeId,
        /// Its class.
        class: ContactClass,
        /// Why that class carries nothing at rest, in the class's own
        /// terms. Sourced from the table, never restated here, so a
        /// class admitted later cannot inherit another's reason.
        why: &'static str,
    },
    /// The kernel's tier-3′ door refused the assembled product with
    /// its records (A5): at least one finding is a verdict AGAINST the
    /// document — a refuted declaration or an undeclared contact.
    /// Every finding travels, in the kernel's own deterministic sweep
    /// order, each carrying what it says about the declarations.
    ///
    /// A mixed refusal lands here: one refuted declaration makes this
    /// a finding against the document however many declines ride with
    /// it.
    AtRest {
        /// Every finding.
        findings: Vec<AtRestFinding>,
    },
    /// **The declared direction's frontier** — a distinct refusal
    /// because it is a distinct fact, and a caller matching this enum
    /// must say which of the two they mean.
    ///
    /// Nothing was refuted and nothing was undeclared: every finding
    /// is the census DECLINING to certify a face that a declaration
    /// names ([`Attribution::Declined`]) — which is what
    /// [`attribute`] establishes, exactly — so the assembly is
    /// unrefuted and uncertified, and NOTHING was decided about this
    /// geometry.
    ///
    /// Today that is the whole declared direction. The census's patch
    /// certifier gates on STRUCTURAL chart identity — a shared
    /// `SurfaceKey` within one body, or the same `GeomSource` across
    /// bodies — which two instances of a part satisfy by neither half,
    /// so a declared cross-instance pair ends here whatever its
    /// geometry. Closing that is a cross-instance chart rung in the
    /// census, not work this layer can do; the day it lands,
    /// [`assemble`] returns `Ok` for these documents and every arm
    /// matching here goes dead.
    Uncertified {
        /// The record set the gate was given: the parts' carried
        /// declarations plus this document's minted ones, uncertified
        /// rather than rejected. Boxed like the enum's other bulky
        /// payloads, so one arm does not set every caller's `Result`
        /// width.
        contacts: Box<ContactRecords>,
        /// Every finding, each a [`Attribution::Declined`].
        findings: Vec<AtRestFinding>,
    },
}

// Why a mate reference did not resolve, in prose — the WHY clause of
// [`AssemblyError::Reference`]'s message; the typed variant stays the
// machine contract.
impl core::fmt::Display for RefusedRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NodeGone => f.write_str("its minting node is not in this document"),
            Self::Vanished => f.write_str("no entity of the product answers to it"),
            Self::Ambiguous { width } => write!(
                f,
                "{width} entities answer to it — a mate declaration names ONE face, and \
                 a tie is never broken by picking"
            ),
            Self::NotAFace { kind } => {
                write!(f, "it names a {}, not a face", kind.noun())
            }
        }
    }
}

impl core::fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Product(e) => write!(f, "assembly: {e}"),
            Self::Reference {
                mate,
                side,
                name,
                why,
            } => write!(
                f,
                "assembly: mate {}'s {} reference (a {} name minted by node {}) does \
                 not name a face of the product: {why}",
                mate.0,
                side.name(),
                name.kind.noun(),
                name.node.0
            ),
            Self::NoAtRestRecord { mate, class, why } => write!(
                f,
                "assembly: mate {}'s class {} has no at-rest kernel record — \
                 {why}; the record is not minted with an invented witness",
                mate.0,
                class.name()
            ),
            Self::AtRest { findings } => {
                write!(
                    f,
                    "assembly: the at-rest gate refused ({} finding(s))",
                    findings.len()
                )?;
                crate::finding::render_list(f, findings)
            }
            Self::Uncertified { findings, .. } => {
                write!(
                    f,
                    "assembly: the at-rest gate could not certify {} declared \
                     face(s) and did not refute any — no certifier lane, so \
                     nothing was decided about this geometry either way (the \
                     declared direction's frontier, not a finding against the \
                     document)",
                    findings.len()
                )?;
                crate::finding::render_list(f, findings)
            }
        }
    }
}

impl core::error::Error for AssemblyError {}

/// **The A5 gate, assembled** (D-2 + D-3): gather the document's
/// product, mint every solved mate's declaration into its contact
/// record set, and run the kernel's tier-3′ at-rest door over the
/// two together.
///
/// This is what "an assembly is valid at rest" MEANS for a document
/// with mates: not a stronger check bolted onto the product gather,
/// but the same tier-3′ door the boolean pipeline's results go
/// through, fed the records the mates declared.
///
/// # Errors
///
/// [`AssemblyError`]: the gather's own refusals, a mate reference that
/// names no product face, a class with no at-rest record
/// ([`crate::mate::class_admission`]), and the kernel's tier-3′
/// findings attributed back to their mates.
///
/// The success arm is narrower than the signature, and the type says
/// so: a document whose mates declare a cross-instance contact
/// refuses [`AssemblyError::Uncertified`], which a caller must match
/// separately from the verdicts against their own document.
pub fn assemble<P, T: Decide + PropsQuadLane>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<Assembly<T>, AssemblyError> {
    let product =
        product_recorded(doc, evaluation, tol).map_err(|e| AssemblyError::Product(Box::new(e)))?;
    let Product {
        body,
        names,
        mut contacts,
    } = product;
    let minted = mint(doc, evaluation, &names, &mut contacts)?;
    match topo::validate_pseudomanifold(&body, &contacts, tol) {
        Ok(()) => Ok(Assembly {
            body,
            names,
            contacts,
            minted,
        }),
        Err(errors) => {
            let findings: Vec<AtRestFinding> = errors
                .into_iter()
                .map(|error| AtRestFinding {
                    attribution: attribute(&error, &minted),
                    error,
                })
                .collect();
            // The split is the whole point of the two arms: ONE
            // finding against the document makes this a refusal of the
            // document, however many declines ride with it. Only a
            // refusal that is declines and nothing else is the
            // frontier.
            // (Non-empty because `Uncertified` promises at least one
            // declined pair; the kernel never refuses with no finding.)
            if !findings.is_empty()
                && findings
                    .iter()
                    .all(|f| matches!(f.attribution, Attribution::Declined(_)))
            {
                Err(AssemblyError::Uncertified {
                    contacts: Box::new(contacts),
                    findings,
                })
            } else {
                Err(AssemblyError::AtRest { findings })
            }
        }
    }
}

/// **Declaration minting** (D-2): each live mate's declaration, in
/// DOCUMENT ORDER, appended to `contacts` as the kernel's own record
/// type and keyed to the placed faces its references resolve to.
///
/// INVARIANT: a mate's ROLE does not enter. A determining mate and a
/// declaring one mint the same record, because minting is the act of
/// DECLARING and a mate that solved nothing still states a contact.
/// (Verification is the caller's next line, and it too is
/// role-blind.)
///
/// A mate whose node failed or never ran mints nothing: the gather
/// already refused the document in that case, so reaching here means
/// every root evaluated, and a mate value that is absent is a node
/// that is not live.
fn mint<P, T: Decide>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    names: &NameTable,
    contacts: &mut ContactRecords,
) -> Result<Vec<MintedDeclaration>, AssemblyError> {
    let mut minted = Vec::new();
    for &id in doc.order() {
        let Some(Node::Mate { a, b, class, .. }) = doc.node(id) else {
            continue;
        };
        // A mate that is not a live value of this evaluation declares
        // nothing here (see the doc comment).
        if !matches!(
            evaluation.result(id),
            Some(NodeResult::Ok(v)) if matches!(v.payload, ValuePayload::Mate(_))
        ) {
            continue;
        }
        let face_a = resolve_face(names, id, MateSide::A, a)?;
        let face_b = resolve_face(names, id, MateSide::B, b)?;
        // The class table is the policy (`mate::class_admission`); this
        // door enforces its own half of it, and takes the REASON from
        // the table too, so the message can never be another class's.
        match class_admission(*class) {
            // Face granularity (M9-1): a rest between two placed faces
            // IS a `PatchContact`. Same type as the boolean wrapper's
            // records, no adapter.
            ClassAdmission::Mints => contacts.patches.push(PatchContact { face_a, face_b }),
            other => {
                return Err(AssemblyError::NoAtRestRecord {
                    mate: id,
                    class: *class,
                    why: other.no_record_reason(),
                });
            }
        }
        minted.push(MintedDeclaration {
            mate: id,
            a: a.clone(),
            b: b.clone(),
            class: *class,
            faces: (face_a, face_b),
        });
    }
    Ok(minted)
}

/// One mate reference → the product face it names, or the typed
/// refusal. A tie is never broken by picking a side, and a non-face
/// reference is never widened into one.
fn resolve_face(
    names: &NameTable,
    mate: RecipeNodeId,
    side: MateSide,
    name: &StableName,
) -> Result<FaceKey, AssemblyError> {
    let refuse = |why| AssemblyError::Reference {
        mate,
        side,
        name: Box::new(name.clone()),
        why,
    };
    match names.lookup(name) {
        Some(Entry::Unique(ent)) => match ent.key {
            EntityKey::Face(f) => Ok(f),
            other => Err(refuse(RefusedRef::NotAFace { kind: other.kind() })),
        },
        Some(Entry::Tied(ents)) => Err(refuse(RefusedRef::Ambiguous {
            width: u32::try_from(ents.len()).unwrap_or(u32::MAX),
        })),
        None => Err(refuse(RefusedRef::Vanished)),
    }
}

/// **What a kernel finding says about what was minted** — which
/// declaration it names, and whether it REFUTES that declaration or
/// merely declines to certify it.
///
/// INVARIANT: one dispatch decides both. The relation is a property
/// of the kernel's arm, not a second reading of it, so a widened arm
/// cannot leave a caller's classification behind.
///
/// INVARIANT: attribution is by ARENA KEY against what was minted —
/// the same lineage discipline the record carry uses. A finding whose
/// faces no declaration names is UNATTRIBUTED, and that is the honest
/// answer for the F1 case: an undeclared contact has no mate, which
/// is exactly what makes it the hard error.
///
/// **The match is EXHAUSTIVE, with no wildcard arm**, which is
/// [`ValidationError`]'s own rule for a site that CLASSIFIES — one
/// that maps the enum onto a smaller vocabulary, as against one that
/// extracts a variant and answers `None` to the rest — and it is
/// load-bearing here rather than tidy: [`Attribution::Declined`] is the only relation
/// that can reach [`AssemblyError::Uncertified`], so this
/// classification decides whether the kernel refused THIS DOCUMENT or
/// merely could not certify it. A wildcard hands that decision to
/// whoever adds the next variant, and nothing goes red when they get
/// it wrong — every acceptance row here exercises a variant one of
/// the classified arms already names.
///
/// The dividing line, stated once so each arm need only cite it: a
/// finding is attributable only where its subject is a **contact
/// record** — the tier-3′ certification vocabulary. Every other
/// variant states something about the body's own structure or
/// geometry, which no mate declared and none can answer for; sharing
/// a face with a declaration is not being named by one.
fn attribute(error: &ValidationError, minted: &[MintedDeclaration]) -> Attribution {
    let by_pair = |a: FaceKey, b: FaceKey| {
        minted
            .iter()
            .find(|m| m.faces == (a, b) || m.faces == (b, a))
    };
    let by_face = |f: FaceKey| minted.iter().find(|m| m.faces.0 == f || m.faces.1 == f);
    // A lookup that misses is a finding about a record no mate of THIS
    // document minted.
    let named = |m: Option<&MintedDeclaration>, relation: fn(MintedDeclaration) -> Attribution| {
        m.map_or(Attribution::Unattributed, |m| relation(m.clone()))
    };
    match error {
        // A declared pair the kernel CONTRADICTED: the declaration is
        // in the error, so the pair names its mate exactly.
        ValidationError::ContactContradicted { declaration, .. } => {
            named(by_pair(declaration.a, declaration.b), Attribution::Refuted)
        }
        // A declared FACE-PAIR record the kernel could not confirm —
        // the other direction of the certification diff. The record's
        // own faces are in the error, so it names its mate exactly.
        //
        // **UNREACHABLE through this door today, and no row can pin
        // its `Refuted` label.** Every `PatchContact` the gate sees
        // here is one `mint` made (the gather itself contributes none
        // — `row2_a` asserts that), and a minted record is
        // cross-instance by construction, because a same-instance
        // mate refuses `SelfMate` at the solve door. For such a pair
        // the census's chart door answers DIVERGENCE, never `Empty`,
        // so the staleness arm never fires; both faces are in the
        // gathered body by construction, so its other trigger cannot
        // fire either; and the `CurveLocus` half needs a
        // `CurveContact`, which `mint` refuses to make at all.
        //
        // The label still has to be right, because it is exactly the
        // dangerous direction: relabelled `Declined`, a REFUTED
        // declaration would be promoted into
        // `AssemblyError::Uncertified` and reported as an unrefuted
        // frontier. What makes the arm live is the same census
        // cross-instance chart rung that closes `Uncertified` — when
        // the chart door starts answering `Empty` instead of
        // diverging, this arm executes and wants its own acceptance
        // row in the same change.
        ValidationError::StaleContactDeclaration {
            declaration:
                topo::StaleDeclaration::Patch { face_a, face_b }
                | topo::StaleDeclaration::CurveLocus { face_a, face_b, .. },
        } => named(by_pair(*face_a, *face_b), Attribution::Refuted),
        // A carrier kind the census inventory cannot certify: it
        // neither certified nor contradicted the pair, which is the
        // decline relation exactly.
        //
        // The finding's subject is a PAIR and it carries one face, so
        // the lookup is width-1 where the question is not: a face two
        // mates declare answers to the first of them, and the census's
        // conformal sweep reaches this arm on an undeclared pair as
        // well — one of whose faces may still be declared against a
        // third. The relation survives both (nothing was refuted
        // either way); the mate the message names may not be the one
        // whose declaration the census could not certify. Narrowing it
        // needs the pair in the error, which is `topo`'s to carry.
        ValidationError::CensusUnsupported {
            entity: topo::EntityId::Face(f),
        } => named(by_face(*f), Attribution::Declined),
        // The rest of the tier-3′ contact vocabulary, each
        // unattributable for a reason of its own rather than by
        // default.
        //
        // `UndeclaredContact` is the definition of unattributed: no
        // mate authored it, which is what makes it F1's hard error.
        //
        // The VERTEX-granular staleness arms name a v-v or v-on-f
        // record, and [`mint`] makes `PatchContact` and nothing else
        // — so no declaration of this document is the subject, and a
        // stale record a PART carries is a finding against the
        // document that a mate cannot answer for. Sharing a face with
        // a mate's declaration would not make it that mate's.
        //
        // `CensusEscalated` carries the classifier's diagnostic and
        // NO entity, so there is nothing to attribute — and the
        // relation would be `Unattributed` regardless: an escalation
        // is indeterminate geometry at rest, which the trilean
        // discipline refuses outright. It is not the census declining
        // a lane, and it must not be reported as an unrefuted
        // frontier.
        //
        // `CensusUnsupported` on any NON-face entity is unattributable
        // by key: a minted declaration names two faces. Its live case
        // is the curve-record confirm pass, which names its witness
        // EDGE — a carried `CurveContact`, never a minted one.
        //
        // `CensusUndecidable` cannot name a minted declaration in
        // either of its two arms. The cross-solid face-pair arm skips
        // a pair the records declare (in both orientations) and leaves
        // it to the confirm pass, so a declared pair never reaches it;
        // the instance-containment arm names SOLIDS, which no lookup
        // over face keys can match.
        ValidationError::UndeclaredContact { .. }
        | ValidationError::StaleContactDeclaration {
            declaration:
                topo::StaleDeclaration::VertexVertex { .. }
                | topo::StaleDeclaration::VertexOnFace { .. },
        }
        | ValidationError::CensusEscalated { .. }
        | ValidationError::CensusUnsupported {
            entity:
                topo::EntityId::Solid(_)
                | topo::EntityId::Shell(_)
                | topo::EntityId::Loop(_)
                | topo::EntityId::HalfEdge(_)
                | topo::EntityId::Edge(_)
                | topo::EntityId::Vertex(_),
        }
        | ValidationError::CensusUndecidable { .. } => Attribution::Unattributed,
        // Everything the tier-1/2/3 passes find: the body's own
        // structure and geometry. None of these is a statement about
        // a contact record, so none can be a verdict on a
        // declaration, and each is a finding against the document in
        // its own right. Listed rather than swept up, so a new
        // structural or geometric variant has to be put on one side
        // of the line above by hand.
        ValidationError::Band { .. }
        | ValidationError::DanglingDescription { .. }
        | ValidationError::UncertifiableSurface { .. }
        | ValidationError::DegenerateTorus { .. }
        | ValidationError::DegenerateTorusEscalated { .. }
        | ValidationError::NonpositiveTorusTube { .. }
        | ValidationError::ApproxCertification { .. }
        | ValidationError::ApproxLaneUnsupported { .. }
        | ValidationError::EdgeCertification { .. }
        | ValidationError::DescriptionNotAdjacent { .. }
        | ValidationError::PlanarFaceResidual { .. }
        | ValidationError::PlanarFaceEscalated { .. }
        | ValidationError::PlanarBoundaryResidual { .. }
        | ValidationError::PlanarBoundaryEscalated { .. }
        | ValidationError::SliverDihedral { .. }
        | ValidationError::TransverseNotIntrinsic { .. }
        | ValidationError::TangentNotIntrinsic { .. }
        | ValidationError::ScaffoldAtRest { .. }
        | ValidationError::LoopRoleInverted { .. }
        | ValidationError::CurvedSenseInverted { .. }
        | ValidationError::NegativeVolume
        | ValidationError::VolumeUncomputable { .. }
        | ValidationError::Pcurve { .. }
        | ValidationError::RingMeetsOuter { .. }
        | ValidationError::RingContactEscalated { .. }
        | ValidationError::DanglingTopology { .. }
        | ValidationError::DanglingGeometry { .. }
        | ValidationError::NextPrevMismatch { .. }
        | ValidationError::LoopCycleOverrun { .. }
        | ValidationError::ParentLoopMismatch { .. }
        | ValidationError::UnreachableHalfEdge { .. }
        | ValidationError::EdgeHalvesIdentical { .. }
        | ValidationError::EdgeSlotBackpointerMismatch { .. }
        | ValidationError::HalfEdgeUnclaimed { .. }
        | ValidationError::HalfEdgeMultiplyClaimed { .. }
        | ValidationError::EdgeNotAntiparallel { .. }
        | ValidationError::EmanatingStartMismatch { .. }
        | ValidationError::EmptyLoopVertexWithEmanating { .. }
        | ValidationError::LoneVertexWithIncidence { .. }
        | ValidationError::VertexOrbitOverrun { .. }
        | ValidationError::OrbitForeignMember { .. }
        | ValidationError::SplitVertexOrbit { .. }
        | ValidationError::OuterListedAsRing { .. }
        | ValidationError::BackPointerMismatch { .. }
        | ValidationError::OrphanEntity { .. }
        | ValidationError::MultiplyOwned { .. }
        | ValidationError::OrphanGeometry { .. }
        | ValidationError::SolidWithoutShells { .. }
        | ValidationError::ShellWithoutFaces { .. }
        | ValidationError::EdgeAcrossShells { .. }
        | ValidationError::ComponentEulerViolation { .. }
        | ValidationError::MissingProvenance { .. }
        | ValidationError::LeakedProvenance { .. }
        | ValidationError::ScaffoldingEmptyLoop { .. }
        | ValidationError::ScaffoldingStrutVertex { .. }
        | ValidationError::ShellDisconnected { .. }
        | ValidationError::NullScaffoldShared { .. }
        | ValidationError::LeakedNullFaceRecord { .. }
        | ValidationError::StaleNullFaceLoop { .. }
        | ValidationError::NullEdgeAtRest { .. }
        | ValidationError::NullFaceAtRest { .. } => Attribution::Unattributed,
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod attribution {
    //! **[`attribute`]'s classification, one row per arm.**
    //!
    //! **Two of the six arms are already driven end to end**, and the
    //! module does not stand in for those: `asm_r2b_assembly.rs`
    //! reaches `CensusEscalated` through [`assemble`] with an in-band
    //! authored gap and `CensusUnsupported` through a touching pair
    //! the certifier declines. What the rest have in common is that
    //! **no fixture provokes them**, so their label is settled by the
    //! argument at the arm or not at all — which is how one wildcard
    //! held five of them with every row green.
    //!
    //! The cost of that is measured rather than asserted, and it is
    //! why these rows exist. Relabelling `UncertifiableSurface`
    //! `Declined` promotes an [`AssemblyError::AtRest`] refusal into
    //! [`AssemblyError::Uncertified`] — an unrefuted frontier over a
    //! body the kernel refused — and **the whole of `editor-core` goes
    //! green over it**: every integration row in the crate passes, and
    //! the one failure is
    //! `a_structural_finding_on_a_declared_face_is_unattributed` here.
    //! (A count would rot on an unrelated merge; the claim is the
    //! re-derivable part, by making that arm `Declined` and running
    //! the crate.) Relabelling `CensusUnsupported` `Refuted`, by contrast,
    //! reds three rows of `asm_r2b_assembly.rs` — which is the
    //! difference between an arm a fixture reaches and an arm only an
    //! argument reaches.
    //!
    //! The keys come from a real body, so they are distinct and not
    //! invented; the body itself is dropped, because attribution is
    //! key algebra over what was minted and nothing here reads the
    //! geometry those keys describe. The findings are constructed
    //! rather than provoked, which CANNOT show that the kernel ever
    //! produces a given finding for a given configuration — those are
    //! the arguments at the arms, and the two that carry weight (a
    //! declared pair never reaches the cross-solid backstop; `mint`
    //! makes `PatchContact` and nothing else) are properties of
    //! `topo::census` and [`mint`], not of this module.

    use geom_core::predicate::{Band, MarginDiag};
    use topo::{CensusContact, DeclaredContact, EntityId, StaleDeclaration, ValidationError};

    use super::{Attribution, FaceKey, MintedDeclaration, attribute};
    use crate::mate::ContactClass;
    use crate::names::{EntityKind, RoleSeg, StableName};
    use crate::node::RecipeNodeId;
    use geom_core::Tol;

    /// A body with three unrelated faces, and one declaration over the
    /// first two — the shape every row below asks a question against:
    /// a finding about the declared pair, about the odd face, or about
    /// neither.
    fn fixture() -> (
        Vec<MintedDeclaration>,
        FaceKey,
        FaceKey,
        FaceKey,
        topo::VertexKey,
    ) {
        let mut body = topo::Body::<f64>::new();
        let mut mint_face = || {
            let created = body
                .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0))
                .expect("mvfs births a solid, shell, face and lone vertex");
            (created.face, created.vertex)
        };
        let (a, _) = mint_face();
        let (b, _) = mint_face();
        let (odd, vertex) = mint_face();
        // Two DIFFERENT references, as a mate's are: a same-instance
        // pair refuses `SelfMate` at the solve door, so a declaration
        // naming one entity twice is a state `mint` cannot produce.
        let name = |node| StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![RoleSeg::OutputBody],
        };
        let minted = vec![MintedDeclaration {
            mate: RecipeNodeId(7),
            a: name(1),
            b: name(2),
            class: ContactClass::Rest,
            faces: (a, b),
        }];
        (minted, a, b, odd, vertex)
    }

    fn escalation() -> geom_core::Indeterminate {
        geom_core::Indeterminate {
            margin: MarginDiag::Value(0.0),
            band: Band::linear(Tol::witness()).expect("the ambient tolerance builds a band"),
            predicate: None,
        }
    }

    /// A declared pair the kernel CONTRADICTED — the only `Refuted`
    /// producer reachable through [`assemble`] today, and the one arm
    /// whose lookup is `by_pair` rather than `by_face`: the
    /// declaration travels in the error, so an order-swapped
    /// declaration must still be found and a pair no mate declared
    /// must not be.
    #[test]
    fn a_contradicted_declaration_is_refuted_by_pair_in_either_order() {
        let (minted, a, b, odd, _) = fixture();
        let contradicted = |x, y| ValidationError::ContactContradicted {
            declaration: DeclaredContact {
                a: x,
                b: y,
                class: topo::ContactClass::Rest,
            },
            witness: String::new(),
            margin: escalation(),
            steer: None,
        };
        for (x, y) in [(a, b), (b, a)] {
            assert!(matches!(
                attribute(&contradicted(x, y), &minted),
                Attribution::Refuted(m) if m.mate == RecipeNodeId(7)
            ));
        }
        assert_eq!(
            attribute(&contradicted(a, odd), &minted),
            Attribution::Unattributed,
            "a pair no mate declared has no declaration to refute"
        );
    }

    /// An UNDECLARED contact is the definition of unattributed: by
    /// definition no mate authored it, which is what makes it F1's
    /// hard error. Held against a finding that names a face a mate DID
    /// declare — the case a coarser rule would attribute, and the one
    /// where attributing it would report the hard error as a frontier.
    #[test]
    fn an_undeclared_contact_is_unattributed_over_a_declared_face() {
        let (minted, a, _, _, vertex) = fixture();
        assert_eq!(
            attribute(
                &ValidationError::UndeclaredContact {
                    contact: CensusContact::VertexOnFace { vertex, face: a },
                    witness: String::new(),
                },
                &minted
            ),
            Attribution::Unattributed
        );
    }

    /// The declared direction: a face the census inventory cannot
    /// certify is a DECLINE, which is the only relation that can reach
    /// [`AssemblyError::Uncertified`].
    #[test]
    fn an_unsupported_declared_face_declines() {
        let (minted, a, _, odd, _) = fixture();
        let unsupported = |f: FaceKey| ValidationError::CensusUnsupported {
            entity: EntityId::Face(f),
        };
        assert!(matches!(
            attribute(&unsupported(a), &minted),
            Attribution::Declined(m) if m.faces == minted[0].faces
        ));
        assert_eq!(
            attribute(&unsupported(odd), &minted),
            Attribution::Unattributed,
            "a face no mate declared is nobody's decline"
        );
    }

    /// The same refusal on a NON-face entity names no declaration:
    /// `mint` mints face pairs, so its live case — the curve-record
    /// confirm pass, which names its witness edge — is a record a PART
    /// carried, not one this document minted.
    #[test]
    fn an_unsupported_non_face_entity_is_unattributed() {
        let (minted, _, _, _, vertex) = fixture();
        assert_eq!(
            attribute(
                &ValidationError::CensusUnsupported {
                    entity: EntityId::Vertex(vertex),
                },
                &minted
            ),
            Attribution::Unattributed
        );
    }

    /// **The dangerous direction, pinned.** A stale face-pair
    /// declaration is a REFUTED declaration; labelled `Declined` it
    /// would be promoted into [`AssemblyError::Uncertified`] and
    /// reported as an unrefuted frontier. Unreachable through
    /// [`assemble`] today, which is exactly why the label is asserted
    /// here.
    #[test]
    fn a_stale_face_pair_declaration_is_refuted_in_either_order() {
        let (minted, a, b, ..) = fixture();
        for (face_a, face_b) in [(a, b), (b, a)] {
            assert!(matches!(
                attribute(
                    &ValidationError::StaleContactDeclaration {
                        declaration: StaleDeclaration::Patch { face_a, face_b },
                    },
                    &minted
                ),
                Attribution::Refuted(m) if m.mate == RecipeNodeId(7)
            ));
        }
    }

    /// A VERTEX-granular stale record is not the face-pair
    /// declaration's business, even when it names one of its faces:
    /// attribution is by the record the finding names, and sharing a
    /// face is not being named.
    #[test]
    fn a_vertex_granular_stale_record_is_not_the_pairs_refutation() {
        let (minted, a, _, _, vertex) = fixture();
        assert_eq!(
            attribute(
                &ValidationError::StaleContactDeclaration {
                    declaration: StaleDeclaration::VertexOnFace { vertex, face: a },
                },
                &minted
            ),
            Attribution::Unattributed
        );
    }

    /// An escalation is indeterminate geometry at rest — a refusal,
    /// never the census declining a lane.
    ///
    /// **The weakest row here, kept deliberately.** `row4_b` of
    /// `asm_r2b_assembly.rs` already drives this arm through
    /// [`assemble`]; what it cannot separate is the classification
    /// from the rest of the verdict, which is what this asserts. And
    /// because the finding carries no entity, no lookup is possible:
    /// only a mutation that invents a declaration out of `minted` can
    /// red it. The arm's label is an ARGUMENT rather than a lookup,
    /// and this is where the argument is written down.
    #[test]
    fn an_escalated_census_predicate_is_unattributed() {
        let (minted, ..) = fixture();
        assert_eq!(
            attribute(
                &ValidationError::CensusEscalated {
                    cause: escalation()
                },
                &minted
            ),
            Attribution::Unattributed
        );
    }

    /// **The pair the census could not clear is not a decline.** The
    /// cross-solid backstop defers on a pair the records declare, so
    /// this configuration does not arise; the row pins what the
    /// classifier would say if it did, because `Declined` here would
    /// report a pair the census could not even examine as an unrefuted
    /// frontier.
    #[test]
    fn an_undecidable_pair_is_unattributed_even_when_declared() {
        let (minted, a, b, ..) = fixture();
        assert_eq!(
            attribute(
                &ValidationError::CensusUndecidable {
                    a: EntityId::Face(a),
                    b: EntityId::Face(b),
                    what: "a class the census cannot examine",
                },
                &minted
            ),
            Attribution::Unattributed
        );
    }

    /// A finding about the body's own geometry is not a verdict on any
    /// declaration, however many of its faces the declaration names.
    #[test]
    fn a_structural_finding_on_a_declared_face_is_unattributed() {
        let (minted, a, ..) = fixture();
        assert_eq!(
            attribute(&ValidationError::UncertifiableSurface { face: a }, &minted),
            Attribution::Unattributed
        );
    }
}
