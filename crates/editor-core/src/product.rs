//! **The whole-document product** (ASSEMBLY-DESIGN A10, D10
//! `[operations]`): the body the document's product slot reads
//! ([`crate::Doc::product`], set by [`crate::DocEdit::SetProduct`]),
//! gathered into ONE aggregate [`Body`]. Where the slot reads a `⊔`
//! ([`crate::Node::Assemble`]), each of its members is one source, in
//! member order; any other read is one source.
//!
//! This is what A2's "an assembly's evaluation is a body" means for a
//! part document. Nothing joins the product by being placed: the slot
//! says what it is, and a measure, an assertion or a mate reads no
//! product, so none of them gates the gather. A slot that reads
//! nothing is a valid document with no product, and a door needing a
//! product refuses [`ProductError::EmptyProduct`]. A slot whose body
//! is gone is a stranded reader, refused
//! [`ProductError::StrandedProduct`].
//!
//! Each copy is its own output and carries its source's names
//! verbatim; a `⊔` keys each member's names by the member's read
//! ([`crate::names::RoleSeg::From`]), so two placements of one body
//! carry two sets of names in the product and none collide.
//!
//! # The posture the gather copies
//!
//! Bodies arrive through [`topo::graft_disjoint_all_keyed`] — the
//! disjoint half of the boolean pipeline's combine door — one call per
//! source BODY, exactly as `step-import` materializes a STEP
//! assembly's instances. A source body carrying several solids (an
//! instantiated sub-assembly) arrives as several solids of the product
//! in its own solid order: that door is the N-solid one. Nothing is
//! fused and no seam is implied; provenance rides through
//! verbatim, and the `Instance(i)` names a pattern minted keep
//! addressing its instances through the evaluation's own name tables
//! (the gather writes no table of the evaluation's — it builds one
//! aggregate table of its own, and the per-node tables it reads are
//! untouched).
//!
//! Validation is the kernel's F8/D7 shape, gated once: the aggregate
//! is gated at rest, and only when it refuses is each source body
//! gated on its own, in gather order, so the refusal names every piece
//! and output index whose body fails ([`ProductError::PieceInvalid`]).
//! Attribution is paid for on the refusing path alone, so a successful
//! gather runs the gate once (held by the source row in
//! `tests/product_gate_attribution.rs`). Why a refusal re-gates rather
//! than reading the aggregate's findings is `attribute_at_rest`'s doc.
//! An aggregate refusal with every source clean is a graft defect
//! ([`ProductError::ProductInvalid`]). Both
//! gates go through the SCALAR'S at-rest
//! policy ([`topo::AtRestPolicy`], `docs/DUAL-DESIGN.md` DL3):
//! certifying scalars run [`topo::validate_geometric`] verbatim; at a
//! dual the gates are structurally absent, and their success arm SAYS
//! so ([`topo::AtRestOutcome::NotRunAtThisScalar`]). The PAIRING
//! OBLIGATION rides with that: at a non-certifying scalar a gathered
//! product is NOT a validated product — the base-scalar evaluation
//! beside it, whose value channel is bit-identical, is the validation
//! of record. Disjoint multi-solid bodies are tier-3 legal.
//! Know what the aggregate gate proves: tier 3 is a LOCAL battery in
//! what it CHECKS (per-face, per-edge, per-edge–face-pair, plus each
//! solid's signed volume, read on that solid's own faces), so no check
//! compares one solid against another and solids that OVERLAP pass THIS
//! call undetected — inter-solid interference is not among its checks.
//! It is not local in what it REPORTS (`attribute_at_rest`). Contact
//! and overlap between copies are decided by the tier-3′ census
//! ([`topo::validate_pseudomanifold`]), and the gather runs it too,
//! through the census's recording door ([`topo::record_at_rest`], each
//! copy its own group): a touch between two copies no record backs is
//! a `CensusAtRest` row of the product and a record citing it (D1 (iii),
//! D10), and every other finding is kept on the product
//! ([`Product::census`]) for the at-rest check ([`crate::checks::at_rest`]),
//! which reads it and runs no census again.
//!
//! **The gather does not refuse on the census**: what the census finds
//! is kept, not raised, because a caller gathering on every edit must
//! still be handed the product — the corpus heat sink at 160 fins (161
//! solids / 991 faces), whose fins overlap the base, is handed over
//! with its census's findings, which the at-rest check turns into
//! interference findings. The census costs the same order as the gather itself, and
//! more than once where a touch between copies is unmated (the door
//! reruns to confirm what it recorded, at most four passes), so every
//! gather door pays it, and the
//! gather, not the registry above it, is what a landing pays for, which
//! is why a caller gathers ONCE. The terms are measured over that heat
//! sink, not stated here: the `registry split` row of
//! `crates/editor-core/tests/m4_pr8_latency.rs` appends
//! `registry_split.gather_ms` (the census included), `checks_ms` (the
//! registry over a subject already gathered) and `census_ms` (one census
//! pass over the gathered product and its records) to
//! `docs/perf-data/rebuild-latency/`, on a nightly cron gated on `main`
//! having moved. What the gather
//! DOES owe — [`topo::graft_disjoint_all_keyed`] asserts nothing about
//! its operands, so every caller of it must establish disjointness —
//! is discharged by the census it runs, whose overlaps between copies
//! the at-rest check reports. It reports rather than refuses, because a
//! viewer must keep drawing a document it can diagnose.

use std::sync::Arc;

use geom_core::Decide;
use topo::{AtRestPolicy, Body, ContactRecords, ValidationError};

use crate::doc::Doc;
use crate::eval::{BooleanValue, Evaluation, NodeStanding, NodeValue, SplitSide, ValuePayload};
use crate::names::{CarriedRows, EntityKey, NameTable, SplitHalf};
use crate::node::RecipeNodeId;
use crate::sentence::{Labelled, Labels, Staged};
use crate::spoken::{Said, Say, Speaker};
use geom_core::Tol;

/// Why [`product`] refused. Fail-loud and typed: a product is all of
/// its pieces or none of them — there are no partial products.
#[derive(Debug)]
pub enum ProductError {
    /// The evaluation is an evaluation of ANOTHER document (DI3).
    /// Raised before the first piece is read: node ids are minted per
    /// document, so a foreign evaluation can carry entries for these
    /// very ids and gather a product out of another document's
    /// geometry without a single lookup missing.
    EvaluationOfAnotherDocument {
        /// The document whose product was asked for.
        expected: crate::ident::DocumentId,
        /// The document the handed evaluation is of.
        found: crate::ident::DocumentId,
    },
    /// The body the product slot reads has no value in this evaluation
    /// (its node, a `⊔` itself rather than one of its pieces), and its
    /// standing says why and where the repair is: no entry (the run stopped
    /// before it, or the id is not the document's), failed, or poisoned
    /// through its nearest failed ancestor.
    Read(NodeStanding),
    /// **The product slot reads nothing** (D10 `[operations]`): there
    /// is no product for a door that needs one, and nothing has gone
    /// wrong. The reading is [`ProductErrorKind::means_no_body`]'s.
    EmptyProduct,
    /// **A product read whose body is gone** (D10): the slot reads what
    /// no live operation defines, which the gather refuses rather than
    /// skip.
    StrandedProduct {
        /// The read.
        read: crate::VarId,
    },
    /// **A product read of a body outside the world** (D10
    /// `[operations]`): an edit moved the body the slot reads into
    /// another space ([`crate::Maintenance::ProductOutsideWorld`]).
    ProductOutsideWorld {
        /// The read.
        read: crate::VarId,
    },
    /// **A source could not be read into the frame the gather computes
    /// in** (D10 `[placement.frame]`): its relative map overflowed, or
    /// the kernel refused the mapped body.
    Frame {
        /// The piece whose body was being read.
        node: RecipeNodeId,
        /// The refusal, as an operation reading the same operand would
        /// raise it.
        kind: Box<crate::eval::NodeErrorKind>,
    },
    /// The kernel's disjoint-graft door refused a source body.
    Graft {
        /// The piece whose body was being grafted.
        node: RecipeNodeId,
        /// The kernel's own refusal.
        source: Box<topo::BooleanError>,
    },
    /// The gathered product failed the at-rest validity gate, and these
    /// pieces' bodies fail it on their own: one finding per failing
    /// source body, in gather order, each carrying the validator's
    /// findings about that body in that body's own keys.
    ///
    /// Every grafted source is re-gated when the aggregate refuses, so
    /// the list names EVERY failing piece, not only the one whose defect
    /// the aggregate's list happened to report (`attribute_at_rest`
    /// says why those differ). Non-empty. A per-entity local
    /// verdict; inter-solid overlap is not among its checks (module
    /// docs, issue #382).
    PieceInvalid {
        /// Every failing source body, in gather order.
        findings: Vec<SourceFinding>,
    },
    /// The gathered product failed the at-rest validity gate while
    /// EVERY source body passes it on its own — a graft defect.
    ///
    /// The battery is local in what it checks, and the graft
    /// transplants each source verbatim, so every finding about the
    /// aggregate is about entities some source holds unchanged; with
    /// every source clean, the transplant is what changed them. The
    /// findings are the aggregate's, in the aggregate's keys.
    ProductInvalid {
        /// Every failure the validator found on the aggregate.
        errors: Vec<ValidationError>,
    },
    /// A source's declared contact record names an entity the graft's
    /// descendant map has no image for (ASM-R2b D-1). The bridge is
    /// total over what it grafted, so this is a bridge bug surfaced —
    /// never a quietly dropped declaration.
    ContactLineage {
        /// The piece whose records were being carried.
        node: RecipeNodeId,
        /// Which entity kind had no descendant.
        what: &'static str,
    },
}

/// **The pairing predicate's finding, in this door's vocabulary.**
///
/// A2a's rule is one predicate (`ident::mispaired`) and one arm per
/// error type over it. The projection lives HERE, at the type that
/// owns the arm, so a door that runs the predicate writes `?` or
/// `m.into()` and no site re-spells which field goes where.
impl From<crate::ident::Mispaired> for ProductError {
    fn from(m: crate::ident::Mispaired) -> Self {
        Self::EvaluationOfAnotherDocument {
            expected: m.expected,
            found: m.found,
        }
    }
}

/// One source body that fails the at-rest gate on its own, as
/// [`ProductError::PieceInvalid`] lists it.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceFinding {
    /// The piece that contributed the body.
    pub node: RecipeNodeId,
    /// The body's output index within that piece's value — the index
    /// the piece's name table keys its rows by.
    pub output: u32,
    /// Every failure the validator found on that body, in its own keys.
    /// Non-empty.
    pub errors: Vec<ValidationError>,
}

// One validity finding about one source, through the layer's one sink
// ([`crate::finding`]): the piece and output are the subject, the
// kernel's finding is the story, FORWARDED verbatim, and the recourse
// is `""` because the kernel's message ends in its own.
struct SourceLine<'a> {
    source: &'a SourceFinding,
    error: &'a ValidationError,
    by: Speaker<'a>,
}

impl crate::finding::Finding for SourceLine<'_> {
    fn subject(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.source.say(f, self.by)
    }

    fn story(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.error)
    }

    fn recourse(&self) -> &str {
        ""
    }
}

/// The source as a finding's subject names it: its piece and output.
impl Say for SourceFinding {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} output {}",
            by.node_as(self.node, "piece"),
            self.output
        )
    }
}

/// The source where no document is at hand: its piece by tag.
impl core::fmt::Display for SourceFinding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Say::say(self, f, Speaker::TAG)
    }
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM and FORWARDS its payload's own `Display` — the kernel's
// refusals and validity findings both carry one, so no arm re-states
// them (and none Debug-dumps them). A validity-finding list renders
// one kernel finding per indented line through the finding sink
// ([`crate::finding`]): the aggregate's bare findings through
// `render_lines`, a per-piece list through `render_list`, each line
// composed with its piece and output as the subject. Every node the
// sentence names is in the gathered document's ids, said by the
// speaker; a name is said as its kind and minting node.
impl core::fmt::Display for ProductError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Labelled(self, Labels::Kept))
    }
}

/// The refusal under its stage word, each node said by `by`.
impl Say for ProductError {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        Labelled::<Self>::open(f, Labels::Kept)?;
        self.fmt_said(f, Labels::Kept, by)
    }
}

// The gather's labels are its stage word and the `piece N output M`
// subject each listed finding opens with, legitimate where the gather's
// refusal is drawn under its own name. The sentence a carrier that
// names the stage renders ([`crate::PartFault::PartProduct`]'s) carries
// neither, and names the pieces in its header.
impl Staged for ProductError {
    const STAGE: &'static str = "product";

    fn fmt_labelled(&self, f: &mut core::fmt::Formatter<'_>, labels: Labels) -> core::fmt::Result {
        self.fmt_said(f, labels, Speaker::TAG)
    }
}

/// [`ProductError`]'s sentence without its labels, each node said by a
/// speaker: what a carrier that names the stage draws.
pub(crate) struct BareSentence<'a>(&'a ProductError, Speaker<'a>);

impl core::fmt::Display for BareSentence<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt_said(f, Labels::Stripped, self.1)
    }
}

impl ProductError {
    /// The sentence without its labels, each node said by `by`.
    pub(crate) fn bare_said<'a>(&'a self, by: Speaker<'a>) -> BareSentence<'a> {
        BareSentence(self, by)
    }

    /// **The refusal as the frame holding the gathered document says
    /// it**: each node as `doc` holds it now ([`crate::Doc::spoken`]).
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &Doc<P>) -> String {
        crate::spoken::spoken_by(self, doc)
    }

    /// The sentence at `labels`, after the stage word when they are
    /// kept, each node said by `by`.
    fn fmt_said(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        labels: Labels,
        by: Speaker<'_>,
    ) -> core::fmt::Result {
        let list = crate::finding::render_lines::<&ValidationError, _>;
        let piece = |id: RecipeNodeId| by.node_as(id, "piece");
        match self {
            Self::EvaluationOfAnotherDocument { expected, found } => write!(
                f,
                "the evaluation is of document {found}, not of \
                 document {expected}",
            ),
            Self::Read(standing) => write!(f, "{}", Said(&standing.of_read(), by)),
            Self::EmptyProduct => write!(
                f,
                "the product slot reads nothing, so the document has no product. {}",
                crate::sentence::Recourse("set the product to a body in the world")
            ),
            // The read is of an output whose operation is gone, so it
            // has nothing to be spoken by.
            Self::StrandedProduct { .. } => write!(
                f,
                "the product reads a body that is gone. {}",
                crate::sentence::Recourse("set the product to a live body")
            ),
            Self::ProductOutsideWorld { .. } => write!(
                f,
                "the product reads a body outside the world. {}",
                crate::sentence::Recourse("set the product to a body in the world")
            ),
            Self::Frame { node, kind } => write!(
                f,
                "{}'s body could not be read into the frame the product computes in: {kind}",
                piece(*node)
            ),
            Self::Graft { node, source } => write!(
                f,
                "the kernel could not graft {}'s body: {source}",
                piece(*node)
            ),
            Self::PieceInvalid { findings } => {
                // Findings arrive in gather order, so one piece's outputs
                // are adjacent and `dedup` counts pieces.
                let mut pieces: Vec<RecipeNodeId> = findings.iter().map(|s| s.node).collect();
                pieces.dedup();
                match labels {
                    Labels::Kept => {
                        let noun = if pieces.len() == 1 { "piece" } else { "pieces" };
                        write!(f, "{} {noun} not valid at rest:", pieces.len())?;
                        let lines: Vec<SourceLine<'_>> = findings
                            .iter()
                            .flat_map(|source| {
                                source.errors.iter().map(move |error| SourceLine {
                                    source,
                                    error,
                                    by,
                                })
                            })
                            .collect();
                        crate::finding::render_list(f, &lines)
                    }
                    Labels::Stripped => {
                        if let [one] = pieces.as_slice() {
                            write!(f, "{} is not valid at rest:", piece(*one))?;
                        } else {
                            let named: Vec<String> =
                                pieces.iter().map(|id| piece(*id).to_string()).collect();
                            write!(f, "{} are not valid at rest:", named.join(", "))?;
                        }
                        crate::finding::render_lines(
                            f,
                            findings.iter().flat_map(|source| &source.errors),
                        )
                    }
                }
            }
            Self::ProductInvalid { errors } => {
                write!(
                    f,
                    "the gathered product is not valid at rest though every copy \
                     is on its own — a graft defect ({} finding(s)):",
                    errors.len()
                )?;
                list(f, errors)
            }
            Self::ContactLineage { node, what } => write!(
                f,
                "{}'s declared contact names {} {what} the \
                 graft's descendant map has no image for — the key bridge is \
                 incomplete; declarations are never dropped to make a gather \
                 succeed",
                piece(*node),
                crate::sentence::article(what)
            ),
        }
    }
}

impl core::error::Error for ProductError {}

/// **A gather refusal, shared** ([`crate::Refusal`]): what
/// [`crate::PartFault::PartProduct`] and [`crate::ChecksError::Product`]
/// hold, its ids bare for the frame that hands it out to say.
pub type ProductRefusal = crate::refusal::Refusal<ProductError>;

impl ProductRefusal {
    /// The refusal, as the gather typed it.
    #[must_use]
    pub fn error(&self) -> &ProductError {
        self.get()
    }

    /// Which arm refused ([`ProductError::kind`]).
    #[must_use]
    pub fn kind(&self) -> ProductErrorKind {
        self.get().kind()
    }
}

/// Which arm of [`ProductError`] refused, without the payload.
///
/// A consumer that records or compares the refusal holds it whole, as
/// a [`ProductRefusal`]; this projection is the class alone, for one
/// that branches on it, hashes it or asserts it.
///
/// One variant per [`ProductError`] arm, except [`ProductError::Read`],
/// whose standing decides which refusal it is to a caller: one variant
/// per state the feature tree draws that node in — no entry, failed,
/// poisoned. [`ProductError::kind`] matches exhaustively, the standing
/// included, so an arm added to the error or a state added to
/// [`NodeStanding`] reds `kind` itself, here in this crate.
///
/// A variant HERE with no arm behind it is a phantom: nothing
/// constructs it, so no test can reach it. This module's tests
/// therefore carry the visit that reds one — an exhaustive match over
/// this enum, which names the phantom at compile time. The fix at that
/// red is to delete the phantom, never to give it a label: a name
/// minted for a phantom publishes a class no refusal can ever carry.
///
/// Deliberately NOT `Ord`. The declaration order mirrors
/// [`ProductError`]'s for reading, and nothing depends on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductErrorKind {
    /// [`ProductError::EvaluationOfAnotherDocument`].
    EvaluationOfAnotherDocument,
    /// [`ProductError::Read`] with no entry for the read node:
    /// [`NodeStanding::NotEvaluated`] or [`NodeStanding::NotInDocument`].
    UnknownNode,
    /// [`ProductError::Read`] carrying [`NodeStanding::Failed`].
    ReadFailed,
    /// [`ProductError::Read`] carrying [`NodeStanding::Poisoned`].
    ReadPoisoned,
    /// [`ProductError::EmptyProduct`].
    EmptyProduct,
    /// [`ProductError::StrandedProduct`].
    StrandedProduct,
    /// [`ProductError::ProductOutsideWorld`].
    ProductOutsideWorld,
    /// [`ProductError::Frame`].
    Frame,
    /// [`ProductError::Graft`].
    Graft,
    /// [`ProductError::PieceInvalid`].
    PieceInvalid,
    /// [`ProductError::ProductInvalid`].
    ProductInvalid,
    /// [`ProductError::ContactLineage`].
    ContactLineage,
}

impl ProductErrorKind {
    /// Whether this class means the DOCUMENT denotes no body at all —
    /// nothing to gather, rather than something gone wrong.
    ///
    /// **The one home of that reading.** Every consumer of a gather
    /// refusal draws this line before it can act on one, and it is the
    /// one line all of them draw the same way, so it is drawn here and
    /// cited rather than re-argued at each site. Consumers that badge
    /// or report call it the empty-document reading; the class itself
    /// is about the world, which is why this is not named for it.
    ///
    /// [`ProductErrorKind::EmptyProduct`] is the only class where there
    /// is nothing to gather rather than something wrong: a document
    /// whose world holds no placement — a fresh one, one holding only
    /// sketches, poses and unplaced bodies.
    /// Every other class is a refusal, with a cause the error carries.
    ///
    /// **What a consumer does with either answer stays the consumer's,
    /// in both directions.** `true` says there is nothing to gather; it
    /// does not say nothing is wrong where the CALLER stands, and a
    /// caller that needed a body may be entitled to fail over its
    /// absence. `eval::parts`'s `product_fault` is the live case:
    /// instantiating a body-less part document is a fault of the
    /// instantiate node, so it renders this class as a part fault like
    /// any other. `false` says the class IS a refusal, not that this
    /// consumer is the one to report it — a consumer whose other
    /// channels already carry some of those classes still decides that
    /// for itself.
    ///
    /// Exhaustive over [`ProductErrorKind`], so a class cannot be added
    /// without being classified here. **That is the whole of what the
    /// compiler buys**: a [`ProductError`] arm added under an EXISTING
    /// class inherits that class's answer silently, and nothing reds.
    #[must_use]
    pub fn means_no_body(self) -> bool {
        match self {
            Self::EmptyProduct => true,
            Self::StrandedProduct
            | Self::ProductOutsideWorld
            | Self::EvaluationOfAnotherDocument
            | Self::UnknownNode
            | Self::ReadFailed
            | Self::ReadPoisoned
            | Self::Frame
            | Self::Graft
            | Self::PieceInvalid
            | Self::ProductInvalid
            | Self::ContactLineage => false,
        }
    }
}

impl ProductError {
    /// Which arm refused, without the payload.
    ///
    /// Exhaustive over [`ProductError`]: adding an arm there is a
    /// compile error here and in every consumer that maps this enum.
    #[must_use]
    pub fn kind(&self) -> ProductErrorKind {
        match self {
            Self::EvaluationOfAnotherDocument { .. } => {
                ProductErrorKind::EvaluationOfAnotherDocument
            }
            Self::Read(standing) => match standing {
                NodeStanding::NotEvaluated { .. } | NodeStanding::NotInDocument { .. } => {
                    ProductErrorKind::UnknownNode
                }
                NodeStanding::Failed { .. } => ProductErrorKind::ReadFailed,
                NodeStanding::Poisoned { .. } => ProductErrorKind::ReadPoisoned,
            },
            Self::EmptyProduct => ProductErrorKind::EmptyProduct,
            Self::StrandedProduct { .. } => ProductErrorKind::StrandedProduct,
            Self::ProductOutsideWorld { .. } => ProductErrorKind::ProductOutsideWorld,
            Self::Frame { .. } => ProductErrorKind::Frame,
            Self::Graft { .. } => ProductErrorKind::Graft,
            Self::PieceInvalid { .. } => ProductErrorKind::PieceInvalid,
            Self::ProductInvalid { .. } => ProductErrorKind::ProductInvalid,
            Self::ContactLineage { .. } => ProductErrorKind::ContactLineage,
        }
    }
}

/// The body-denoting sources one piece contributes, in gather order,
/// each tagged with the OUTPUT-BODY INDEX it occupies in the piece's own
/// value (module docs). That index is what a piece's name table keys its
/// rows by, so carrying it here is what lets [`product_named`] find the
/// rows belonging to each grafted body. `None` for a piece that denotes
/// no body at all.
///
/// Each source also carries the DECLARED CONTACT RECORDS keyed in that
/// body's arena (ASM-R2b D-1), as its value's carry holds them
/// ([`crate::Carry::contacts`]).
///
/// A placement reads one `Body`, so the product of a document whose one
/// placement reads a split's `above` port is that one half, and one
/// reading a `Part` of a pattern is that one copy.
pub(crate) fn sources_of<T: Decide>(value: &NodeValue<T>) -> Option<Vec<Source0<T>>> {
    // The records and findings a body carries are keyed in its arena:
    // each member of a family, and each half of a split, brings its own
    // ([`crate::Carry::member`]).
    let carried = || Arc::clone(&value.carry.contacts);
    let rows = || Arc::clone(&value.carry.findings);
    let norows = || Arc::new(crate::checks::at_rest::CarriedFindings::default());
    match &value.payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            Some(vec![(0, body.clone(), carried(), rows())])
        }
        ValuePayload::Boolean(BooleanValue::Empty) => Some(Vec::new()),
        // Cannot refuse: an instance list is minted by the pattern op,
        // which refuses any index `names::output_body` does not fit, or
        // by `Placeable::map`, which rebuilds an existing list one body
        // for one.
        ValuePayload::Instances(bodies) => Some(
            bodies
                .iter()
                .enumerate()
                .map(|(i, body)| {
                    let ix = crate::names::output_body(i).unwrap_or_else(|e| {
                        unreachable!("instance {i} outlived its minting op's index refusal: {e}")
                    });
                    let member = value.carry.member(i);
                    (
                        ix,
                        body.clone(),
                        Arc::clone(&member.contacts),
                        Arc::clone(&member.findings),
                    )
                })
                .collect(),
        ),
        // Split's halves are output bodies by `SplitHalf::output_body`
        // (the one definition of that mapping); an EMPTY half
        // contributes nothing but does not shift the other half's
        // index — the index is the value's layout, not a position in
        // this list.
        ValuePayload::Split { above, below } => Some(
            [(SplitHalf::Above, above), (SplitHalf::Below, below)]
                .into_iter()
                .filter_map(|(half, side)| match side {
                    SplitSide::Body(body) => {
                        let at = usize::try_from(half.output_body()).unwrap_or(usize::MAX);
                        let records = Arc::clone(&value.carry.member(at).contacts);
                        Some((half.output_body(), body.clone(), records, norows()))
                    }
                    SplitSide::Empty => None,
                })
                .collect(),
        ),
        // The gather IGNORES a mate — it denotes no body, and "ignored
        // by the gather" is exactly this arm. E3/E10's two
        // sinks join it for the same reason: a measured quantity and an
        // an assertion's verdict denote no material, so a document
        // whose only addition is an assertion has the same product it
        // had without one.
        ValuePayload::Profile(_)
        | ValuePayload::Measure { .. }
        | ValuePayload::MeasureUnavailable { .. }
        | ValuePayload::Assertion(_)
        | ValuePayload::Family => None,
    }
}

// Gathers this thread has performed, the debug-only witness of the
// one-gather-per-landing invariant. Thread-local rather than global: a
// witness two tests running in one process can both read is a witness
// neither can trust.
#[cfg(debug_assertions)]
thread_local! {
    static GATHERS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// How many times [`product_recorded`] has run on THIS thread — every
/// call, refusals included, since a refused gather is still a gather
/// paid for.
///
/// `cfg(debug_assertions)`-gated. **That is not the same as "absent
/// from a release build" here**: this workspace's `[profile.release]` sets
/// `debug-assertions = true` deliberately (and says so, and says it
/// comes out before publish), so every build this repo produces today
/// carries the counter and the increment. What the gate buys is that
/// cargo's OWN release defaults strip both, which is what a consumer
/// building this crate normally gets — and that the day the stanza
/// comes out, nothing here has to change.
///
/// It counts, and a caller reads a DIFFERENCE across the operation it
/// is asking about; the absolute value means nothing.
#[cfg(debug_assertions)]
#[must_use]
pub fn gathers_on_this_thread() -> u64 {
    GATHERS.with(std::cell::Cell::get)
}

/// The document's product: the body its product slot reads, gathered
/// into one [`Body`] (module docs).
///
/// The result is a pure function of (`doc.product()`, `evaluation`)
/// — no ambient state, so two evaluations of an edit that moves no
/// member yield the same solid order (D9).
///
/// # Errors
///
/// Every arm of [`ProductError`]: an evaluation of another document
/// ([`ProductError::EvaluationOfAnotherDocument`]); a piece that
/// failed, was poisoned, or is absent from this evaluation; a slot
/// that reads nothing ([`ProductError::EmptyProduct`]) or a body that
/// is gone ([`ProductError::StrandedProduct`]); the kernel's graft and at-rest validity refusals.
pub fn product<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<Body<T>, ProductError> {
    product_recorded(doc, evaluation, tol).map(|p| p.body.into_body())
}

/// The whole-document product, with everything the gather knows about
/// it: the aggregate body, its name table, and its DECLARED CONTACT
/// RECORDS (ASM-R2b D-1).
///
/// Three doors, one implementation — [`product`] and [`product_named`]
/// are this function with fields dropped — because a gather that named
/// or recorded its entities differently from the gather that shipped
/// them would be a second truth about what a document's product is.
#[derive(Debug)]
pub struct Product<T: Decide> {
    /// The document this product is OF (DI3).
    ///
    /// A gather is a statement about one document, and the doors that
    /// take a gathered product rather than gathering for themselves
    /// have no other way to check that it is the one they were asked
    /// about — `crate::run_checks_on` refuses a product from another
    /// document exactly as this gather refuses a foreign evaluation.
    /// Written from `doc.id()` after the pairing door below, so it is
    /// the identity BOTH arguments agreed on.
    pub document: crate::ident::DocumentId,
    /// The gathered aggregate, with the at-rest gate's verdict on it
    /// kept ([`topo::AtRestBody`]). The gather runs tier 3′'s census
    /// over it once ([`Self::census`]); the at-rest check reads it and
    /// runs no census again.
    pub body: topo::AtRestBody<T>,
    /// **The frame [`Self::body`] is in** (D10 `[placement.frame]`): the
    /// frame the gather computed in, its first source in its space's
    /// coordinates, else its first source's. Export, the façade's
    /// read-outs and the viewer compose it into the world through
    /// [`crate::mate::in_space`] alone.
    pub(crate) frame: crate::eval::Frame<T>,
    /// Its stable names, re-keyed onto the aggregate ([`product_named`]).
    pub names: NameTable,
    /// Its contact records: each source's, re-keyed onto the aggregate
    /// through the graft's own descendant map, citing that source's
    /// record ([`Self::cited_inputs`]); and each the census recorded for
    /// a touch between copies, mated or not, citing its own row of
    /// [`Self::coincidences`] ([`topo::record_at_rest`]).
    pub contacts: ContactRecords,
    /// **The at-rest census's decisions** ([`topo::DecisionSite::CensusAtRest`]):
    /// the contacts between copies no carried record backs, then the
    /// flush pairs between copies, each in the census's order, each cell
    /// named as its copy's placement reads it. The `unproven-coincidence`
    /// check reads them ([`crate::checks`]).
    pub coincidences: Vec<crate::checks::at_rest::AtRestRow>,
    /// The product's inputs a carried record cites, by position: the
    /// gathered sources, in gather order.
    pub cited_inputs: Vec<crate::coincide::CitedInput>,
    /// **The tier-3′ census's findings over the product and its
    /// records**, run once, by the gather, in the census's order: every
    /// finding but the contacts between copies it recorded as rows
    /// ([`Self::coincidences`]). The at-rest check reads them and runs
    /// no census of its own ([`crate::checks::at_rest`]).
    pub census: Vec<ValidationError>,
    /// Which piece contributed each of the aggregate's solids, in
    /// gather order.
    ///
    /// Read off the GRAFT's own minted-key list, exactly as the name
    /// and contact carries are — never re-derived by looking at the
    /// gathered geometry.
    pub solid_copies: Vec<SolidOrigin>,
    /// Every copy the gather grafted, in gather order: its placed body
    /// and the bridge from that body's arena to the aggregate's. The
    /// at-rest check intersects two of them to localize an interference
    /// between them ([`crate::checks::at_rest`]).
    pub copies: Vec<GatheredCopy<T>>,
    /// The holding assertions over a `Gap` between faces of two of
    /// `copies`, whose admitted values are all negative: the only
    /// assertions that can quiet an interference
    /// ([`crate::checks::at_rest::GapAssertion`]).
    pub gap_assertions: Vec<crate::checks::at_rest::GapAssertion>,
    /// The overlaps documents below this one reported between their own
    /// copies ([`crate::CarriedInterference`]), carried verbatim, in
    /// gather order: reported by the at-rest check, never refused.
    pub carried_interference: Vec<crate::checks::at_rest::CarriedInterference>,
    /// What documents below this one could not look at
    /// ([`crate::CarriedAtRestFinding`]), carried verbatim, in gather
    /// order: reported by the at-rest check, never refused.
    pub carried_at_rest: Vec<crate::checks::at_rest::CarriedAtRestFinding>,
    /// **Each `⊔` the product does not read, gathered by itself**
    /// ([`assemble_gathers`]), in document order, kept beside the
    /// product so the at-rest check reads every `⊔` from the one product
    /// it is handed ([`crate::checks::at_rest`]). Empty on a `⊔`'s own
    /// gather.
    pub assembles: Vec<AssembleGather<T>>,
    /// The pieces the product body holds ([`crate::eval::Pieces`]): what
    /// an instance of this document carries, so a pick reads one piece
    /// of it.
    pub pieces: Option<Arc<crate::eval::Pieces<T>>>,
    /// How many parts the product is ([`crate::Carry::parts`]).
    pub parts: usize,
}

impl<T: Decide> Product<T> {
    /// A copy of this verdict a reader owns: every field but
    /// [`Self::assembles`], which only the product slot's gather fills.
    #[must_use]
    pub fn fresh(&self) -> Self {
        // Destructured, so a field added to `Product` fails to compile
        // here until this copy says what it does with it.
        let Self {
            document,
            body,
            frame,
            names,
            contacts,
            coincidences,
            cited_inputs,
            census,
            solid_copies,
            copies,
            gap_assertions,
            carried_interference,
            carried_at_rest,
            assembles: _,
            pieces,
            parts,
        } = self;
        Self {
            document: *document,
            body: body.clone(),
            frame: frame.clone(),
            names: names.clone(),
            contacts: contacts.clone(),
            coincidences: coincidences.clone(),
            cited_inputs: cited_inputs.clone(),
            census: census.clone(),
            solid_copies: solid_copies.clone(),
            copies: copies.clone(),
            gap_assertions: gap_assertions.clone(),
            carried_interference: carried_interference.clone(),
            carried_at_rest: carried_at_rest.clone(),
            assembles: Vec::new(),
            pieces: pieces.clone(),
            parts: *parts,
        }
    }
}

/// **One copy as the gather grafted it** ([`Product::copies`]).
#[derive(Debug, Clone)]
pub struct GatheredCopy<T: Decide> {
    /// The piece that defines it.
    pub node: RecipeNodeId,
    /// The piece's output variable: the copy as a selection reads
    /// it.
    pub var: crate::VarId,
    /// The output-body index within the piece's value, which is
    /// also its name rows' index.
    pub output: u32,
    /// The placed body, in the frame the product computed in
    /// ([`Product::body`]'s).
    pub body: Arc<Body<T>>,
    /// The bridge from `body`'s arena to the aggregate's.
    pub keys: Arc<topo::GraftKeys>,
}

/// One gathered solid's origin: the piece that contributed it, that
/// piece's output-body index, and the solid's key in the aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolidOrigin {
    /// The product piece the solid came from.
    pub node: RecipeNodeId,
    /// The output-body index within that piece's value.
    pub output: u32,
    /// The solid's key in the gathered aggregate.
    pub solid: topo::SolidKey,
}

/// The document's product, with the product's own NAME TABLE: every
/// gathered piece's stable names, re-keyed onto the aggregate's entities
/// (ASM-2A D-4).
///
/// One implementation serves all three doors — this is
/// [`product_recorded`] with the contacts dropped.
///
/// The table carries FACE, EDGE and VERTEX rows. Body-kind rows are
/// deliberately absent: a piece's body name denotes THAT PIECE's body,
/// and the product is not any piece's body — it is the document's, a
/// distinct entity whose naming belongs to whoever mints it (an
/// instantiate node names its own placed body at itself).
///
/// # Errors
///
/// Every arm of [`ProductError`], including
/// [`ProductError::EvaluationOfAnotherDocument`] when `evaluation` is
/// not an evaluation of `doc`.
pub fn product_named<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<(Body<T>, NameTable), ProductError> {
    product_recorded(doc, evaluation, tol).map(|p| (p.body.into_body(), p.names))
}

/// The document's product with its name table AND its contact
/// records — the widest of the three doors, and the one the other two
/// are defined by ([`Product`]).
///
/// # The contacts carry (ASM-R2b D-1)
///
/// A source body's records move onto the aggregate through the GRAFT's
/// own descendant map, exactly as its name rows do — the lineage rule
/// the boolean pipeline's substitution door (`carry`) states: a record's new key
/// is the key the graft says its old entity BECAME, never a key
/// re-derived by looking at the gathered geometry. Re-derivation is
/// the scan-to-bless move F1 bans. The touches the census finds between
/// copies are recorded beside the carried records, never in place of
/// them: each is a row of its own decision ([`topo::record_at_rest`]),
/// and a carried record is never re-derived from one.
///
/// # Errors
///
/// Every arm of [`ProductError`], including
/// [`ProductError::EvaluationOfAnotherDocument`] — `evaluation` must
/// be an evaluation OF `doc`, and this is the door all three read it
/// through — and [`ProductError::ContactLineage`] when the graft's
/// bridge has no image for a record's entity.
pub fn product_recorded<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<Product<T>, ProductError> {
    // The pairing door (DI3), before the slot is read: this gather is a
    // statement about `doc`, and an evaluation of another document
    // answers about other geometry — silently, whenever the two
    // documents' node ids overlap, which two documents built from one
    // recipe always do.
    if let Some(m) = crate::ident::mispaired(doc.id(), evaluation.document) {
        return Err(m.into());
    }
    let Some(var) = doc.product() else {
        // The product door asked, and nothing was there to gather: a
        // gather all the same, for the counters that read one.
        #[cfg(debug_assertions)]
        GATHERS.with(|gathers| gathers.set(gathers.get().saturating_add(1)));
        return Err(ProductError::EmptyProduct);
    };
    if doc.operation_of(var).is_some() && doc.space_of(var) != Some(crate::VarId::WORLD) {
        return Err(ProductError::ProductOutsideWorld { read: var });
    }
    let mut product = gather(doc, evaluation, tol, var)?;
    product.assembles = assemble_gathers(doc, evaluation, tol, Some(var));
    Ok(product)
}

/// **Every census the product slot does not read, each as its door
/// kept it** ([`AssembleGather`]), in document order: each `⊔`'s, and
/// each cut's or meet's that kept a `⊔`'s pieces, read off its own
/// value ([`Census`]) and so run once however many readers ask. What
/// [`product_recorded`] carries beside the product
/// ([`Product::assembles`]), so every door's census is checked whether
/// or not the product reads it.
pub fn assemble_gathers<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
    product: Option<crate::VarId>,
) -> Vec<AssembleGather<T>> {
    doc.ids()
        .into_iter()
        .filter_map(|id| Some((id, doc.output(id, 0)?, evaluation.value(id)?)))
        .flat_map(|(id, var, value)| {
            // A door's own census, not one a copy carries from its
            // source's. A family of doors is a door per member, each
            // censused at its own; the product's gather reads each
            // member as one arrival, so it reads none of theirs.
            let own = |carry: &'_ crate::Carry<T>| {
                carry
                    .census
                    .as_ref()
                    .filter(|census| census.node == id)
                    .cloned()
            };
            let censuses: Vec<Arc<Census<T>>> = match &value.carry.members {
                Some(members) => members.iter().filter_map(own).collect(),
                None => own(&value.carry)
                    .filter(|_| Some(var) != product)
                    .into_iter()
                    .collect(),
            };
            censuses.into_iter().map(move |census| (var, census))
        })
        .map(|(var, census)| AssembleGather {
            var,
            gather: census.verdict(doc, tol).map_err(Into::into).map(|verdict| {
                let mut verdict = verdict.fresh();
                verdict.document = doc.id();
                verdict.gap_assertions = crate::checks::at_rest::gap_assertions(
                    doc,
                    evaluation,
                    &verdict.body,
                    &verdict.names,
                    &verdict.copies,
                    tol,
                );
                Box::new(verdict)
            }),
        })
        .collect()
}

/// **One census the product does not read, as its door kept it** (D10
/// `[operations.operators]`): a `⊔`'s operands, grafted and censused
/// with nothing outside them.
#[derive(Debug)]
pub struct AssembleGather<T: Decide> {
    /// The door's output.
    pub var: crate::VarId,
    /// Its verdict, kept rather than raised: a refusal is the at-rest
    /// check's could-not-look finding against this `⊔`
    /// ([`crate::AtRestEvidence::NotGathered`]).
    pub gather: Result<Box<Product<T>>, ProductRefusal>,
}

/// **The census at a door that assembled pieces**, kept on its value
/// ([`crate::Carry::census`]; D10 `[assertions]`, "The census is
/// each `⊔`'s door"): its pieces grafted side by side, gated, and
/// censused one group per arrival ([`topo::record_at_rest`],
/// `flush_at_rest`), each piece's records carried and each touch
/// between two arrivals recorded. A copy or a pick of the door's body
/// carries the same census, so every reader of the door's meetings
/// reads one verdict ([`Self::verdict`]): the product's gather, the
/// at-rest check, a boolean reading the body, an outer `⊔`.
///
/// The census is the door's, but it does not run at the door: the
/// wire only sets it on the value, and it runs lazily, once, the first
/// time a reader asks, so an evaluation that no reader asks pays
/// nothing for it.
///
/// The verdict reads the document only inside the `⊔`'s cone: what
/// each member read resolves to, an instance's reference through it,
/// and the records its pieces' own censuses keep. The gather reads the
/// document's id and its assertions afresh. So an evaluation that
/// reuses the `⊔`'s value after an edit outside its cone reuses a
/// verdict still true of it (`intent_dprime3_assemble_product_slot`'s
/// `a_kept_verdict_is_reused_only_with_its_cone`).
pub struct Census<T: Decide> {
    /// The node whose door it is.
    pub node: RecipeNodeId,
    pieces: Arc<crate::eval::Pieces<T>>,
    verdict: std::sync::OnceLock<Product<T>>,
}

impl<T: Decide> std::fmt::Debug for Census<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            node,
            pieces: _,
            verdict,
        } = self;
        f.debug_struct("Census")
            .field("node", node)
            .field("run", &verdict.get().is_some())
            .finish_non_exhaustive()
    }
}

impl<T: Decide + AtRestPolicy> Census<T> {
    /// The census of `node`'s door over `pieces`, not yet run.
    #[must_use]
    pub fn new(node: RecipeNodeId, pieces: Arc<crate::eval::Pieces<T>>) -> Self {
        Self {
            node,
            pieces,
            verdict: std::sync::OnceLock::new(),
        }
    }

    /// **The door's verdict**: run the first time it is asked for and
    /// kept. A refusal is not kept: it is run again where it is asked
    /// for again, and refuses again.
    ///
    /// # Errors
    ///
    /// [`ProductError`]'s graft, at-rest and lineage refusals over the
    /// door's pieces.
    pub fn verdict<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        tol: Tol,
    ) -> Result<&Product<T>, ProductError> {
        if let Some(verdict) = self.verdict.get() {
            return Ok(verdict);
        }
        let verdict = census_of(doc, self.node, &self.pieces, tol)?;
        Ok(self.verdict.get_or_init(|| verdict))
    }

    /// The records the door kept: each piece's own, and each touch it
    /// recorded between two of them, keyed in the door's body (and so in
    /// any copy of it, which re-mints no key). `None` where the census
    /// refuses: a reader then has the pieces' own records alone.
    pub(crate) fn records<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        tol: Tol,
    ) -> Option<Arc<ContactRecords>> {
        self.verdict(doc, tol)
            .ok()
            .map(|verdict| Arc::new(verdict.contacts.clone()))
    }
}

/// **A door's census over its pieces** ([`Census`]): each piece one
/// arrival (each member of a family piece its own), named as the door
/// keys it by the piece's read, carrying the records its own door kept
/// where one did.
fn census_of<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    pieces: &Arc<crate::eval::Pieces<T>>,
    tol: Tol,
) -> Result<Product<T>, ProductError> {
    #[cfg(debug_assertions)]
    GATHERS.with(|gathers| gathers.set(gathers.get().saturating_add(1)));
    // A read assembled twice keys its pieces' names alike, so each such
    // name is a tie over those pieces, as the door's own table names it.
    let repeated = |read: crate::VarId| pieces.pieces.iter().filter(|p| p.read == read).count() > 1;
    let mut next = std::collections::BTreeMap::new();
    let mut sources: Vec<Source<T>> = Vec::new();
    for piece in &pieces.pieces {
        let at = doc.read_operation(piece.read).unwrap_or(node);
        // Each piece's names as the door keys them, by its read.
        let keyed = if repeated(piece.read) {
            crate::names::repeated_member_view(pieces.by, piece.read, &piece.table, &mut next)
        } else {
            crate::names::member_view(pieces.by, piece.read, &piece.table)
        };
        let keyed = Arc::new(keyed.unwrap_or_else(|e| {
            unreachable!("a member's table keyed by its read stays a table: {e:?}")
        }));
        for (i, body) in piece.bodies.iter().enumerate() {
            // A piece whose own door censused it brings that door's
            // records, the touches inside it among them; each member of
            // a family brings its own.
            let carry = piece.carry.member(i);
            let contacts = carry
                .census
                .as_ref()
                .and_then(|census| census.records(doc, tol))
                .unwrap_or_else(|| Arc::clone(&carry.contacts));
            let ix = if piece.family {
                crate::names::output_body(i).unwrap_or_else(|e| {
                    unreachable!("piece {i} outlived its minting op's index refusal: {e}")
                })
            } else {
                0
            };
            sources.push(Source {
                node: at,
                var: piece.read,
                ix,
                body: body.clone(),
                table: Arc::clone(&keyed),
                contacts: Arc::clone(&contacts),
                rows: Arc::clone(&carry.findings),
                censused: carry.censused,
            });
        }
    }
    let mut product = gather_sources(doc, tol, sources)?;
    product.pieces = Some(Arc::clone(pieces));
    product.parts = pieces.pieces.iter().map(|piece| piece.carry.parts).sum();
    Ok(product)
}

/// **The gather of the body `var` reads** (D10 `[operations]`): for a
/// door that assembled pieces, its census, as the door kept it
/// ([`Census`]); for any other body, that body as the one source,
/// censused alone.
fn gather<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
    var: crate::VarId,
) -> Result<Product<T>, ProductError> {
    #[cfg(debug_assertions)]
    GATHERS.with(|gathers| gathers.set(gathers.get().saturating_add(1)));
    if let Some(m) = crate::ident::mispaired(doc.id(), evaluation.document) {
        return Err(m.into());
    }
    // The recipe's own refusal, before any value is read: a read whose
    // body is gone is absent, which is never skipped.
    let Some((node, port)) = doc
        .defined_by(var)
        .filter(|&(node, _)| doc.node(node).is_some())
    else {
        return Err(ProductError::StrandedProduct { read: var });
    };
    // A read in no space reads a body whose space is gone.
    if doc.space_of(var).is_none() {
        return Err(ProductError::StrandedProduct { read: var });
    }
    let value = evaluation.usable(node).map_err(ProductError::Read)?;
    let carry = &value.carry;
    let mut product = match carry.census.as_ref().filter(|census| census.node == node) {
        // A door that assembled pieces: its own census, as it kept it.
        Some(census) => census.verdict(doc, tol)?.fresh(),
        None => {
            let Some(bodies) = sources_of(value) else {
                return Err(ProductError::Read(NodeStanding::NotEvaluated { node }));
            };
            // A split's port is its own half.
            let half = matches!(doc.node(node), Some(crate::node::Node::Split { .. }))
                .then(|| SplitHalf::of_output_body(u32::from(port)))
                .flatten()
                .map(SplitHalf::output_body);
            // A copy of a door's body brings the records that door kept.
            let kept = carry
                .census
                .as_ref()
                .and_then(|census| census.records(doc, tol));
            let table = evaluation
                .shared_table_at(var)
                .unwrap_or_else(crate::names::empty);
            // A family's member brings the records its own door kept.
            let sources = bodies
                .into_iter()
                .enumerate()
                .filter(|(_, (ix, ..))| half.is_none_or(|h| h == *ix))
                .map(|(i, (ix, body, contacts, rows))| {
                    let member = carry.member(i);
                    let kept = match &carry.members {
                        Some(_) => member
                            .census
                            .as_ref()
                            .and_then(|census| census.records(doc, tol)),
                        None => kept.clone(),
                    };
                    Source {
                        node,
                        var,
                        ix,
                        body,
                        table: Arc::clone(&table),
                        contacts: kept.unwrap_or(contacts),
                        rows,
                        censused: member.censused,
                    }
                })
                .collect();
            let mut product = gather_sources(doc, tol, sources)?;
            product.pieces = carry.pieces.clone();
            product.parts = carry.parts;
            product
        }
    };
    product.document = doc.id();
    product.gap_assertions = crate::checks::at_rest::gap_assertions(
        doc,
        evaluation,
        &product.body,
        &product.names,
        &product.copies,
        tol,
    );
    Ok(product)
}

/// One grafted source: the source, its graft's key bridge, and its body
/// read into the gather's computing frame.
type Grafted<'s, T> = (&'s Source<T>, Arc<topo::GraftKeys>, Arc<Body<T>>);

/// [`gather`] over its sources: grafted, gated, carried and
/// censused, each source its own group.
fn gather_sources<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    tol: Tol,
    sources: Vec<Source<T>>,
) -> Result<Product<T>, ProductError> {
    // Pass 2: the graft, one call per SOURCE BODY, in list order. A
    // source holding N solids goes through as one call: the keyed graft
    // is the N-solid door (#381), and its per-entity bridge is total
    // over the source however many solids it spans, so the carries
    // below are the same code for N as for 1.
    let mut aggregate = Body::new();
    // The gather computes in its first non-empty source's frame whose
    // frame is its space's (`Own`), else its first's
    // ([`crate::eval::Framed::computing_among`]), and reads every other
    // into it through `relative`.
    let mut computing: Option<&crate::eval::Framed<T>> = crate::eval::Framed::computing_among(
        sources
            .iter()
            .map(|source| &source.body)
            .filter(|body| body.body().solids().next().is_some()),
    );
    let mut grafted: Vec<Grafted<'_, T>> = Vec::with_capacity(sources.len());
    for source in &sources {
        let Source { node, body, .. } = source;
        // An empty source contributes nothing; the graft door refuses a
        // solidless body, so the skip is here rather than there.
        if body.body().solids().next().is_none() {
            continue;
        }
        let read = computing
            .get_or_insert(body)
            .read_in(body, tol)
            .map_err(|kind| ProductError::Frame {
                node: *node,
                kind: Box::new(kind),
            })?;
        let keys =
            topo::graft_disjoint_all_keyed(&mut aggregate, read.as_ref()).map_err(|source| {
                ProductError::Graft {
                    node: *node,
                    source: Box::new(source),
                }
            })?;
        grafted.push((source, Arc::new(keys), read));
    }
    let frame = computing.map_or(crate::eval::Frame::Own, |first| first.frame().clone());

    // The at-rest gate, ONCE, on the aggregate — before any name is
    // carried, so a product that is not a valid body refuses as that
    // before any naming collision is read. Only a refusal pays for
    // attribution (`attribute_at_rest`). The verdict is kept with the
    // body, so the at-rest check over this product pays the census
    // alone.
    let aggregate = match T::gate_at_rest_kept(aggregate, tol) {
        Ok(gated) => gated,
        Err(errors) => {
            return Err(attribute_at_rest(
                grafted.iter().map(|(source, ..)| *source),
                errors,
                tol,
            ));
        }
    };

    // Pass 3: the carries, per grafted source, each across the key
    // bridge its graft minted: the source's name rows, its contact
    // records, and the findings below it.
    let mut names = NameTable::new();
    let mut contacts = ContactRecords::default();
    let mut cited_inputs: Vec<crate::coincide::CitedInput> = Vec::new();
    let mut solid_copies: Vec<SolidOrigin> = Vec::new();
    let mut carried_interference: Vec<crate::checks::at_rest::CarriedInterference> = Vec::new();
    let mut carried_at_rest: Vec<crate::checks::at_rest::CarriedAtRestFinding> = Vec::new();
    let mut tie_rows = CarriedRows::default();
    let mut copies: Vec<GatheredCopy<T>> = Vec::with_capacity(grafted.len());
    for (
        Source {
            node,
            var,
            ix,
            body: _,
            table,
            contacts: records,
            rows,
            censused: _,
        },
        keys,
        body,
    ) in &grafted
    {
        solid_copies.extend(keys.solids().iter().map(|&solid| SolidOrigin {
            node: *node,
            output: *ix,
            solid,
        }));
        copies.push(GatheredCopy {
            node: *node,
            var: *var,
            output: *ix,
            body: Arc::clone(body),
            keys: Arc::clone(keys),
        });
        carry_names(&mut tie_rows, table, *node, *ix, keys);
        // Each source's records cite the source by its position among
        // the product's inputs, which names the read it entered through:
        // the copy's records are its body's, in their order.
        let input = u32::try_from(cited_inputs.len()).unwrap_or(u32::MAX);
        cited_inputs.push(match doc.node(*node) {
            Some(crate::node::Node::InstantiatePart { doc_ref, .. }) => {
                crate::coincide::CitedInput::Part(*doc_ref)
            }
            _ => crate::coincide::CitedInput::Read(*var),
        });
        carry_contacts(
            &aggregate,
            &mut contacts,
            &records.carried_from(input),
            keys,
        )
        .map_err(|what| ProductError::ContactLineage { node: *node, what })?;
        // A part's census findings carry verbatim too, spelled in its
        // own document, each reported once per part row, by the first
        // route in gather order (`once_per_part_row`).
        crate::checks::at_rest::once_per_part_row(
            &mut carried_interference,
            rows.interference.iter().cloned(),
        );
        crate::checks::at_rest::once_per_part_row(
            &mut carried_at_rest,
            rows.at_rest.iter().cloned(),
        );
    }
    // The name carry's second half, after the last source: every
    // carried row, narrowed ONCE over all of them (`carry_names`). A tie
    // whose candidates the document separated into different SOURCES is
    // one tie of the product — the product holds both faces — and this
    // is where that is written, because no single source can see it.
    //
    // A refusal here names the node that MINTED the colliding name
    // rather than a piece: every repeated pair refused in the carry
    // above, so what is left belongs to no one piece.
    if let Err(e) = tie_rows.finish(&mut names) {
        unreachable!(
            "two copies carried one name, {e:?}: a copy's names are qualified by its placement"
        )
    }
    // Each touch between copies names its cells as the census reads
    // them: in its copy, as that copy's placement output names it.
    let groups = crate::checks::at_rest::copy_groups(&copies);
    // The tier-3′ census over the product and its records, here and
    // nowhere else: each touch it finds between two copies is a row of
    // the product and a record citing it (`topo::record_at_rest`, D1
    // (iii)), mated or not, which the `unproven-coincidence` check reads
    // and the door proves by its mate where one rests the pair; the rest
    // is what the at-rest check reports ([`Product::census`]).
    let recorded = topo::record_at_rest(
        &aggregate,
        &contacts,
        |records| {
            T::gate_at_rest_declared(&aggregate, records, tol)
                .err()
                .unwrap_or_default()
        },
        &|solid| groups.get(&solid).copied(),
        0,
    );
    // The flush pairs between copies are rows of their own, which no
    // record cites (`topo::AtRestPolicy::flush_at_rest`).
    let flush = match geom_core::Band::linear(tol) {
        Ok(band) => T::flush_at_rest(&aggregate, &|solid| groups.get(&solid).copied(), band),
        Err(error) => topo::FlushAtRest {
            findings: vec![ValidationError::Band { error }],
            ..topo::FlushAtRest::default()
        },
    };
    contacts.extend(recorded.records.rows().iter().cloned());
    let coincidences = crate::checks::at_rest::copy_rows(
        &aggregate,
        &names,
        &copies,
        recorded.rows.into_iter().chain(flush.rows).collect(),
    );
    let mut census = recorded.findings;
    census.extend(flush.findings);
    // A meeting inside one arrival a door already decided is that
    // door's (D10 `[assertions]`): a `⊔`'s pieces were censused at its
    // own door, and an instance's copy at its part's. The kernel's
    // census has no group scope, so it still computes such a meeting;
    // this door discards what it found there.
    let decided: std::collections::BTreeSet<u32> = grafted
        .iter()
        .zip(0u32..)
        .filter(|((source, ..), _)| source.censused)
        .map(|(_, group)| group)
        .collect();
    if !decided.is_empty() {
        census.retain(|finding| {
            crate::checks::at_rest::finding_group(&aggregate, &groups, finding)
                .is_none_or(|group| !decided.contains(&group))
        });
    }
    Ok(Product {
        document: doc.id(),
        body: aggregate,
        frame,
        names,
        contacts,
        coincidences,
        cited_inputs,
        census,
        solid_copies,
        copies,
        gap_assertions: Vec::new(),
        carried_interference,
        carried_at_rest,
        assembles: Vec::new(),
        pieces: None,
        parts: 0,
    })
}

/// One body the gather will graft: the operation that contributed it,
/// the read it entered through, the output-body index it occupies in
/// that read's value (the index its name rows are keyed by), the body,
/// its name table, its contact records, the at-rest findings it
/// carries from the documents below, and whether its meetings are
/// already decided.
struct Source<T: Decide> {
    node: RecipeNodeId,
    var: crate::VarId,
    ix: u32,
    body: crate::eval::Framed<T>,
    table: Arc<NameTable>,
    contacts: Arc<ContactRecords>,
    rows: Arc<crate::checks::at_rest::CarriedFindings>,
    /// Whether a door already decided the meetings inside it
    /// ([`crate::Carry::censused`]).
    censused: bool,
}

/// **Which pieces the aggregate's at-rest refusal is about**: every
/// source body re-gated on its own, in gather order, and each that
/// fails named with its own findings ([`ProductError::PieceInvalid`]).
/// When every source passes, the refusal is the aggregate's
/// ([`ProductError::ProductInvalid`], a graft defect).
///
/// Re-gated rather than read off `aggregate`'s findings, for two
/// reasons. Many [`ValidationError`] arms carry no key a solid can be
/// recovered from, so a finding cannot always be mapped to the piece
/// that caused it. And the battery's early stops are body-wide, so the
/// aggregate's list can omit one piece's defect entirely: a structural
/// finding anywhere stops every tier-3 check, and a finding of checks
/// 1–6, 8 or 9 on any solid stops check 7 for every solid, so an
/// inside-out solid beside it goes unreported. The battery is local in
/// what it CHECKS, not in what it REPORTS. Each source gated alone
/// reports what that source would have reported as the only body.
///
/// `grafted` is exactly the sources the graft put into the aggregate,
/// in gather order.
fn attribute_at_rest<'a, T: Decide + AtRestPolicy + 'a>(
    grafted: impl Iterator<Item = &'a Source<T>>,
    aggregate: Vec<ValidationError>,
    tol: Tol,
) -> ProductError {
    let findings: Vec<SourceFinding> = grafted
        .filter_map(|Source { node, ix, body, .. }| {
            T::gate_at_rest(body.body(), tol)
                .err()
                .map(|errors| SourceFinding {
                    node: *node,
                    output: *ix,
                    errors,
                })
        })
        .collect();
    if findings.is_empty() {
        ProductError::ProductInvalid { errors: aggregate }
    } else {
        ProductError::PieceInvalid { findings }
    }
}

/// One body-denoting source as [`sources_of`] hands it back: output
/// index, body, the records keyed in that body's arena, and the
/// declaration rows keyed in the same arena.
pub(crate) type Source0<T> = (
    u32,
    crate::eval::Framed<T>,
    Arc<ContactRecords>,
    Arc<crate::checks::at_rest::CarriedFindings>,
);

/// Re-keys one grafted body's contact records onto the aggregate,
/// through the graft's DESCENDANT MAP (`product_recorded`'s contacts
/// carry). INVARIANT: every key here is `keys.<kind>(old)` — the
/// lineage the graft recorded. Nothing is looked up by position, by
/// name, or by re-measuring the aggregate.
///
/// The bridge is total and incidence-keeping over a source it grafted,
/// so a missing image, or a pair the bridge made one cell or incident
/// in the aggregate, is a bridge bug, not a dropped declaration: it refuses typed rather
/// than silently weakening the at-rest gate the records feed.
///
/// [`ProductError::ContactLineage`] is therefore DEFENSIVE and has no
/// test: no public API can reach it without first breaking the graft's
/// own key-bridge contract. Stated rather than left to be discovered —
/// an arm that cannot be exercised is worth knowing about, and the
/// alternative (dropping the record) is the failure this guards.
pub(crate) fn carry_contacts<T: geom_core::Real>(
    body: &Body<T>,
    into: &mut ContactRecords,
    from: &ContactRecords,
    keys: &topo::GraftKeys,
) -> Result<(), &'static str> {
    use topo::Cell;
    let moved = from
        .rekeyed(body, |cell| match cell {
            Cell::Vertex(v) => keys.vertex(v).map(Cell::Vertex),
            Cell::Edge(e) => keys.edge(e).map(Cell::Edge),
            Cell::Face(f) => keys.face(f).map(Cell::Face),
        })
        .map_err(|refusal| match refusal {
            topo::CarryRefusal::Unmapped {
                cell: Cell::Vertex(_),
                ..
            } => "vertex",
            topo::CarryRefusal::Unmapped {
                cell: Cell::Edge(_),
                ..
            } => "edge",
            topo::CarryRefusal::Unmapped {
                cell: Cell::Face(_),
                ..
            } => "face",
            topo::CarryRefusal::Pair { .. }
            | topo::CarryRefusal::Parted { .. }
            | topo::CarryRefusal::Unread { .. }
            | topo::CarryRefusal::Stale { .. }
            | topo::CarryRefusal::MintedMeeting { .. } => "cell pair",
        })?;
    into.extend(moved.rows().iter().cloned());
    Ok(())
}

/// Re-keys one grafted body's name rows onto the aggregate: the same
/// stable names, pointing at the entities the graft minted. Body index
/// on the product side is 0 — the product is ONE body.
///
/// The KEY MAP is this function's own — it is the graft's descendant
/// map, plus the product's rule that a piece body-row does not carry
/// (see `product_named`: the product's own body is nobody's piece
/// body). What a carried row may collide with is not: that is
/// [`crate::names::CarriedRows`], the accumulate-then-narrow mechanism
/// the emitters share, so the aggregate table narrows a tie by the one
/// rule every other table narrows by.
fn carry_names(
    rows: &mut CarriedRows,
    from: &NameTable,
    node: RecipeNodeId,
    ix: u32,
    keys: &topo::GraftKeys,
) {
    let carried = rows.carry(from, ix, |key| match key {
        EntityKey::Body => None,
        EntityKey::Face(f) => keys.face(f).map(EntityKey::Face),
        EntityKey::Edge(e) => keys.edge(e).map(EntityKey::Edge),
        EntityKey::Vertex(v) => keys.vertex(v).map(EntityKey::Vertex),
    });
    if let Err(e) = carried {
        unreachable!(
            "{node:?} carried a name twice, {e:?}: a copy's names are qualified by its placement"
        )
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{ProductError, ProductErrorKind, SourceFinding};
    use crate::eval::NodeStanding;
    use crate::node::RecipeNodeId;
    use crate::sentence::Staged;

    /// One error per [`ProductError`] arm, and one per standing under
    /// [`ProductError::Read`] — a CENSUS, not a sample: every payload
    /// this error carries is constructible from here (keys, ids,
    /// `&'static str`, empty finding lists, and one unit arm of the
    /// kernel's own refusal), so no arm is left unbuilt.
    fn every_arm() -> Vec<ProductError> {
        let node = RecipeNodeId::new(0, test_utils::refusal::tagged(3));
        let through = RecipeNodeId::new(0, test_utils::refusal::tagged(1));
        vec![
            ProductError::EvaluationOfAnotherDocument {
                expected: crate::ident::DocumentId::derive("expected"),
                found: crate::ident::DocumentId::derive("found"),
            },
            ProductError::Read(NodeStanding::NotEvaluated { node }),
            ProductError::Read(NodeStanding::NotInDocument { node }),
            ProductError::Read(NodeStanding::Failed { node }),
            ProductError::Read(NodeStanding::Poisoned { node, through }),
            ProductError::EmptyProduct,
            ProductError::StrandedProduct {
                read: crate::VarId::new(0, test_utils::refusal::tagged(1)),
            },
            ProductError::ProductOutsideWorld {
                read: crate::VarId::new(0, test_utils::refusal::tagged(1)),
            },
            ProductError::Frame {
                node,
                kind: Box::new(crate::eval::NodeErrorKind::Motion(
                    geom_core::RigidMotionError::NonFinite(geom_core::MotionPart::Product),
                )),
            },
            ProductError::Graft {
                node,
                source: Box::new(topo::BooleanError::UnrepresentableResult),
            },
            ProductError::PieceInvalid {
                findings: vec![SourceFinding {
                    node,
                    output: 0,
                    errors: Vec::new(),
                }],
            },
            ProductError::ProductInvalid { errors: Vec::new() },
            ProductError::ContactLineage { node, what: "face" },
        ]
    }

    /// **The phantom direction, closed by the compiler; the pairing
    /// direction, closed by construction.**
    ///
    /// [`ProductError::kind`] is exhaustive over the ERROR, so an arm
    /// added there reds this crate. `label` below is exhaustive over
    /// the KIND, so a variant added to [`ProductErrorKind`] alone reds
    /// HERE, by name, in the crate that owns both — rather than in
    /// whatever downstream crate next maps the enum.
    ///
    /// Neither exhaustiveness objects to an arm PROJECTED to the wrong
    /// kind, which type-checks. That is what the errors below are for:
    /// each is built, projected, and its kind's name compared with the
    /// class written beside it in [`every_arm`]'s order. The expected
    /// classes are literals, not a reading of the error: a census that
    /// derived them would restate the projection it checks. Two arms
    /// projected onto one kind fail it too, because the literals differ.
    ///
    /// The census is complete as measured: every arm of
    /// [`ProductError`] and every standing is built here, so no
    /// projection is unchecked today. What no guard closes is an arm
    /// added to the error LATER, projected onto an existing kind and
    /// left out of [`every_arm`] — neither exhaustiveness reds on that,
    /// and this row accuses no author of anything it has not measured.
    #[test]
    fn each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind() {
        fn label(kind: ProductErrorKind) -> &'static str {
            match kind {
                ProductErrorKind::EvaluationOfAnotherDocument => "EvaluationOfAnotherDocument",
                ProductErrorKind::UnknownNode => "UnknownNode",
                ProductErrorKind::ReadFailed => "ReadFailed",
                ProductErrorKind::ReadPoisoned => "ReadPoisoned",
                ProductErrorKind::EmptyProduct => "EmptyProduct",
                ProductErrorKind::StrandedProduct => "StrandedProduct",
                ProductErrorKind::ProductOutsideWorld => "ProductOutsideWorld",
                ProductErrorKind::Frame => "Frame",
                ProductErrorKind::Graft => "Graft",
                ProductErrorKind::PieceInvalid => "PieceInvalid",
                ProductErrorKind::ProductInvalid => "ProductInvalid",
                ProductErrorKind::ContactLineage => "ContactLineage",
            }
        }
        let expected = [
            "EvaluationOfAnotherDocument",
            "UnknownNode",
            "UnknownNode",
            "ReadFailed",
            "ReadPoisoned",
            "EmptyProduct",
            "StrandedProduct",
            "ProductOutsideWorld",
            "Frame",
            "Graft",
            "PieceInvalid",
            "ProductInvalid",
            "ContactLineage",
        ];
        let arms = every_arm();
        assert_eq!(
            arms.len(),
            expected.len(),
            "one expected class per built arm"
        );
        for (err, class) in arms.iter().zip(expected) {
            assert_eq!(
                label(err.kind()),
                class,
                "kind() projects each arm to its own kind: {err:?}"
            );
        }
    }

    /// **What this reads is the SET, and the set is what a bug moves.**
    ///
    /// [`ProductErrorKind::means_no_body`] is exhaustive over the KIND,
    /// so its answer for any one class is fixed the moment it compiles
    /// and asserting that answer alone names nothing. What is not fixed
    /// is which of the arms [`every_arm`] builds answer `true`. That
    /// set moves when an existing class is re-classified, and when
    /// [`ProductError::kind`] re-projects a built arm onto a class
    /// carrying the other answer — both red here, naming the arms that
    /// came back.
    ///
    /// **It is a floor, not a bijection, and the gap is the census's
    /// own.** A [`ProductError`] arm added under an EXISTING kind and
    /// left out of [`every_arm`] would read as no-body with nothing
    /// red anywhere: not here, because the roster never sees it; not at
    /// the predicate, which is exhaustive over the kind and not over
    /// the error; and not at [`ProductError::kind`], where projecting a
    /// new arm onto an existing kind compiles. That is exactly the hole
    /// `each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind`
    /// states above, and no guard closes it: the roster is hand-written
    /// because stable Rust cannot enumerate an enum's variants, so a
    /// bijection is not available to be asserted. This row states what
    /// it has rather than what would be better.
    #[test]
    fn exactly_one_arm_reads_as_no_body() {
        let reads_no_body: Vec<String> = every_arm()
            .iter()
            .filter(|err| err.kind().means_no_body())
            .map(|err| format!("{err:?}"))
            .collect();
        assert_eq!(
            reads_no_body,
            vec![format!("{:?}", ProductError::EmptyProduct)],
            "an empty slot is the only gather refusal that is an absence \
             rather than a fault"
        );
    }

    /// **The stage word is written once.** Each arm's sentence is the
    /// gather's refusal without it, so a carrier that names the stage
    /// itself draws it bare, and the gather's own rendering opens with
    /// it exactly once.
    #[test]
    fn the_stage_word_opens_the_rendering_and_no_sentence() {
        for err in every_arm() {
            let sentence = err.sentence().to_string();
            assert!(
                !sentence.starts_with("product"),
                "the sentence is the refusal without its stage word: {sentence}"
            );
            assert_eq!(
                err.to_string().matches("product:").count(),
                1,
                "the rendering opens with the stage word once: {err}"
            );
        }
    }

    /// **A read with no value is refused with its standing**, under the
    /// gather's subject: the standing names the node, its state and the
    /// repair, so the gather adds only that the slot reads it.
    #[test]
    fn a_read_without_a_value_renders_its_standing() {
        let standing = NodeStanding::Poisoned {
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(4)),
            through: RecipeNodeId::new(0, test_utils::refusal::tagged(2)),
        };
        assert_eq!(
            ProductError::Read(standing).to_string(),
            format!("product: the body the slot reads: {standing}")
        );
    }

    /// **The bare sentence names every failing piece in its header**,
    /// each as a piece, so a carrier that draws no per-line subject
    /// still says which pieces failed.
    #[test]
    fn the_bare_piece_invalid_header_names_each_piece() {
        let source = |node: u64| SourceFinding {
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(node)),
            output: 0,
            errors: vec![topo::ValidationError::NegativeVolume {
                solid: topo::SolidKey::default(),
            }],
        };
        let two = ProductError::PieceInvalid {
            findings: vec![source(3), source(5)],
        };
        assert!(
            two.sentence()
                .to_string()
                .starts_with("piece 000000000003, piece 000000000005 are not valid at rest:\n  "),
            "{}",
            two.sentence()
        );
        let one = ProductError::PieceInvalid {
            findings: vec![source(3)],
        };
        assert!(
            one.sentence()
                .to_string()
                .starts_with("piece 000000000003 is not valid at rest:\n  "),
            "{}",
            one.sentence()
        );
    }
}
