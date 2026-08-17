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
//! # The honest boundary
//!
//! A `Rest` mate's kernel record is [`topo::PatchContact`] — face
//! granularity, the rung M9-1 landed. A `Tangent` mate's would be
//! `CurveContact`, which is keyed by the WITNESS EDGE whose carrier is
//! the contact locus; an assembly at rest has no such edge (nothing
//! zipped the instances together, which is exactly what "at rest, not
//! a boolean" means). So a Tangent mate refuses typed at the mint door
//! naming the deferral, rather than minting a record with an invented
//! witness. Where the census inventory cannot certify a carrier kind
//! it refuses typed too, and that refusal passes straight through
//! here — stated, never swallowed.

use geom_core::Decide;
use topo::{ContactRecords, FaceKey, PatchContact, PropsQuadLane, ValidationError};

use crate::doc::Doc;
use crate::eval::{Evaluation, NodeResult, ValuePayload};
use crate::mate::{ContactClass, MateSide};
use crate::names::{EntityKey, EntityKind, Entry, NameTable, StableName};
use crate::node::{Node, RecipeNodeId};
use crate::product::{Product, ProductError, product_recorded};

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

/// One at-rest refusal, attributed to its mate where the kernel's
/// finding names a declared face pair.
#[derive(Debug, Clone, PartialEq)]
pub struct AtRestFinding {
    /// The mate whose declaration the finding names, when one does.
    /// `None` for an UNDECLARED contact — by definition no mate
    /// authored it, and that is precisely the F1 hard error.
    pub mate: Option<MintedDeclaration>,
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
    /// The mate's class has no at-rest kernel record (module docs: the
    /// honest boundary).
    NoAtRestRecord {
        /// The mate.
        mate: RecipeNodeId,
        /// Its class.
        class: ContactClass,
    },
    /// The kernel's tier-3′ door refused the assembled product with
    /// its records (A5). Every finding, in the kernel's own
    /// deterministic sweep order, each carrying the mate that
    /// authored the declaration it names.
    AtRest {
        /// Every finding.
        findings: Vec<AtRestFinding>,
    },
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
                "assembly: mate {}'s {} reference {name:?} does not name a face \
                 of the product: {why:?}",
                mate.0,
                side.name()
            ),
            Self::NoAtRestRecord { mate, class } => write!(
                f,
                "assembly: mate {}'s class {} has no at-rest kernel record — a \
                 tangent contact's record is keyed by the witness edge whose \
                 carrier is the locus, and an assembly at rest has none; the \
                 record is not minted with an invented witness",
                mate.0,
                class.name()
            ),
            Self::AtRest { findings } => {
                write!(
                    f,
                    "assembly: the at-rest gate refused ({} findings)",
                    findings.len()
                )?;
                for finding in findings {
                    match &finding.mate {
                        Some(m) => write!(
                            f,
                            "; mate {} ({:?} ~ {:?}, {}): {:?}",
                            m.mate.0,
                            m.a,
                            m.b,
                            m.class.name(),
                            finding.error
                        )?,
                        None => write!(f, "; unattributed: {:?}", finding.error)?,
                    }
                }
                Ok(())
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
/// names no product face, a class with no at-rest record, and the
/// kernel's tier-3′ findings attributed back to their mates.
pub fn assemble<P, T: Decide + PropsQuadLane>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
) -> Result<Assembly<T>, AssemblyError> {
    let product =
        product_recorded(doc, evaluation).map_err(|e| AssemblyError::Product(Box::new(e)))?;
    let Product {
        body,
        names,
        mut contacts,
    } = product;
    let minted = mint(doc, evaluation, &names, &mut contacts)?;
    match topo::validate_pseudomanifold(&body, &contacts) {
        Ok(()) => Ok(Assembly {
            body,
            names,
            contacts,
            minted,
        }),
        Err(errors) => Err(AssemblyError::AtRest {
            findings: errors
                .into_iter()
                .map(|error| AtRestFinding {
                    mate: attribute(&error, &minted),
                    error,
                })
                .collect(),
        }),
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
        match class {
            // The face-granularity rung (M9-1): a planar or conformal
            // rest between two placed faces IS a `PatchContact`. Same
            // type as the boolean wrapper's records, no adapter.
            ContactClass::Rest => contacts.patches.push(PatchContact { face_a, face_b }),
            // The honest boundary (module docs).
            other => {
                return Err(AssemblyError::NoAtRestRecord {
                    mate: id,
                    class: *other,
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

/// Which mate's declaration a kernel finding names, if any.
///
/// INVARIANT: attribution is by ARENA KEY against what was minted —
/// the same lineage discipline the record carry uses. A finding whose
/// faces no declaration names is UNATTRIBUTED, and that is the honest
/// answer for the F1 case: an undeclared contact has no mate, which
/// is exactly what makes it the hard error.
fn attribute(error: &ValidationError, minted: &[MintedDeclaration]) -> Option<MintedDeclaration> {
    let by_pair = |a: FaceKey, b: FaceKey| {
        minted
            .iter()
            .find(|m| m.faces == (a, b) || m.faces == (b, a))
    };
    let by_face = |f: FaceKey| minted.iter().find(|m| m.faces.0 == f || m.faces.1 == f);
    let hit = match error {
        // A declared pair the kernel CONTRADICTED: the declaration is
        // in the error, so the pair names its mate exactly.
        ValidationError::ContactContradicted { declaration, .. } => {
            by_pair(declaration.a, declaration.b)
        }
        // A declared FACE-PAIR record the kernel could not confirm —
        // the other direction of the certification diff. The record's
        // own faces are in the error, so it names its mate exactly.
        ValidationError::StaleContactDeclaration {
            declaration:
                topo::StaleDeclaration::Patch { face_a, face_b, .. }
                | topo::StaleDeclaration::CurveLocus { face_a, face_b, .. },
        } => by_pair(*face_a, *face_b),
        // A carrier kind the census inventory cannot certify. The
        // entity is a single face; a declaration naming it is the
        // reason it was examined at all.
        ValidationError::CensusUnsupported {
            entity: topo::EntityId::Face(f),
        } => by_face(*f),
        // Everything else — undeclared contacts, vertex-granular
        // staleness, band failures — names no declared face pair.
        _ => None,
    };
    hit.cloned()
}
