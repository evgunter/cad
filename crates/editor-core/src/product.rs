//! **The whole-document product** (ASSEMBLY-DESIGN A10): the
//! deterministic gather, in the document order of the world
//! placements, of every copy a [`crate::Node::PlaceInWorld`] defines
//! into ONE aggregate [`Body`].
//!
//! This is what A2's "an assembly's evaluation is a body" means for a
//! part document. The product is the world: a body appears only if a
//! placement names it, whatever reads it, and nothing else places. A
//! measure, an assertion or a mate reads no placement, so none of them
//! gates the gather. An empty world is a valid document whose product
//! is empty, and a door needing a product refuses
//! [`ProductError::EmptyProduct`], naming the unplaced bodies. A
//! placement whose body is gone is a stranded reader, refused
//! [`ProductError::StrandedPlacement`].
//!
//! Each copy is its own output, and its names are the body's qualified
//! by the copy ([`crate::names::RoleSeg::Placed`]), so two placements
//! of one body carry two sets of names and none collide.
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
//! verbatim, so a pattern instance's `GeomSource::placed(node, i)`
//! survives into the product, and the `Instance(i)` names the pattern
//! minted keep addressing it through the evaluation's own name tables
//! (the gather writes no table of the evaluation's — it builds one
//! aggregate table of its own, and the per-node tables it reads are
//! untouched).
//!
//! Validation is the kernel's F8/D7 shape, gated once: the aggregate
//! is gated at rest, and only when it refuses is each source body
//! gated on its own, in gather order, so the refusal names every root
//! and output index whose body fails ([`ProductError::RootInvalid`]).
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
//! It is not local in what it REPORTS (`attribute_at_rest`). Undeclared
//! cross-instance contact is A5's hard error and interference fits are
//! C6's recorded-gate-skips territory; both are decided by the tier-3′
//! door ([`topo::validate_pseudomanifold`]), which the ASSEMBLY gate
//! runs over this gather's output ([`crate::assemble`]) and which this
//! function does not. The aggregate gate's verdict rides on the
//! product's body ([`topo::AtRestBody`]), so that gate pays tier 3′'s
//! census over it and not the local battery a second time.
//!
//! **That is a division of labour, not a gap**: the census reaches the
//! touching/overlap space and nothing in it validates silently. The
//! gather stays on the local battery because the census refuses what a
//! caller gathering on every edit must still be handed — the corpus
//! heat sink at 160 fins (161 solids / 991 faces), whose fins meet the
//! base, refuses there with 125 findings — and because it costs the
//! same order as the gather itself. The gather, not the registry above
//! it, is what a landing pays for, which is why a caller gathers ONCE.
//! The terms are measured over that heat sink, not stated here: the
//! `registry split` row of `crates/editor-core/tests/m4_pr8_latency.rs`
//! appends `registry_split.gather_ms`, `checks_ms` (the registry over a
//! subject already gathered) and `census_ms` (the assembly gate over
//! the kept verdict) to `docs/perf-data/rebuild-latency/`, on a nightly
//! cron gated on `main` having moved. What the gather
//! DOES owe — [`topo::graft_disjoint_all_keyed`] asserts nothing about
//! its operands, so every caller of it must establish disjointness —
//! is discharged by [`crate::checks`]'s separation resident, which
//! reads this gather's `solid_copies` and holds every cross-root solid
//! pair to the box-level certificate. It reports rather than refuses,
//! because a viewer must keep drawing a document it can diagnose.

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
/// the placements or none of them — there are no partial products.
#[derive(Debug)]
pub enum ProductError {
    /// The evaluation is an evaluation of ANOTHER document (DI3).
    /// Raised before the first root is read: node ids are minted per
    /// document, so a foreign evaluation can carry entries for these
    /// very ids and gather a product out of another document's
    /// geometry without a single lookup missing.
    EvaluationOfAnotherDocument {
        /// The document whose product was asked for.
        expected: crate::ident::DocumentId,
        /// The document the handed evaluation is of.
        found: crate::ident::DocumentId,
    },
    /// A placement has no value in this evaluation, and its standing
    /// says why and where the repair is: no entry (the run stopped
    /// before it, or the id is not the document's), failed, or poisoned
    /// through its nearest failed ancestor.
    Root(NodeStanding),
    /// **The world holds no placement** (A10): there is no product
    /// for a door that needs one, and nothing has gone wrong. The
    /// reading is [`ProductErrorKind::means_no_body`]'s.
    EmptyProduct {
        /// Every live body no placement reads, in document order
        /// ([`Doc::unplaced`]): what placing would put in the world.
        unplaced: Vec<crate::VarId>,
    },
    /// **A placement whose body is gone** (D10, A10): its read is
    /// unresolved, and the copy it defined is absent, which the gather
    /// refuses rather than skip.
    StrandedPlacement {
        /// The placement.
        placement: RecipeNodeId,
    },
    /// **No placement's copy lives in the world, and the placed
    /// material lives in the own spaces of unplaced groups** (A9,
    /// A11 (2)): the world product is empty because nothing places that
    /// material, which placing it repairs.
    Unplaced {
        /// Every unplaced group, by its root, with why nothing places
        /// it, in document order.
        groups: Vec<(RecipeNodeId, crate::mate::Unplaced)>,
    },
    /// The kernel's disjoint-graft door refused a source body.
    Graft {
        /// The placement whose copy was being grafted.
        node: RecipeNodeId,
        /// The kernel's own refusal.
        source: Box<topo::BooleanError>,
    },
    /// The gathered product failed the at-rest validity gate, and these
    /// roots' bodies fail it on their own: one finding per failing
    /// source body, in gather order, each carrying the validator's
    /// findings about that body in that body's own keys.
    ///
    /// Every grafted source is re-gated when the aggregate refuses, so
    /// the list names EVERY failing root, not only the one whose defect
    /// the aggregate's list happened to report (`attribute_at_rest`
    /// says why those differ). Non-empty. A per-entity local
    /// verdict; inter-solid overlap is not among its checks (module
    /// docs, issue #382).
    RootInvalid {
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
        /// The placement whose records were being carried.
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
/// [`ProductError::RootInvalid`] lists it.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceFinding {
    /// The placement that contributed the body.
    pub node: RecipeNodeId,
    /// The body's output index within that root's value — the index
    /// the root's name table keys its rows by.
    pub output: u32,
    /// Every failure the validator found on that body, in its own keys.
    /// Non-empty.
    pub errors: Vec<ValidationError>,
}

// One validity finding about one source, through the layer's one sink
// ([`crate::finding`]): the root and output are the subject, the
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

/// The source as a finding's subject names it: its root and output.
impl Say for SourceFinding {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} output {}",
            by.node_as(self.node, "placement"),
            self.output
        )
    }
}

/// The source where no document is at hand: its root by tag.
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
// `render_lines`, a per-root list through `render_list`, each line
// composed with its root and output as the subject. Every node the
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

// The gather's labels are its stage word and the `root N output M`
// subject each listed finding opens with, legitimate where the gather's
// refusal is drawn under its own name. The sentence a carrier that
// names the stage renders ([`crate::PartFault::PartProduct`]'s) carries
// neither, and names the roots in its header.
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
        let root = |id: RecipeNodeId| by.node_as(id, "placement");
        match self {
            Self::EvaluationOfAnotherDocument { expected, found } => write!(
                f,
                "the evaluation is of document {found}, not of \
                 document {expected}",
            ),
            Self::Root(standing) => write!(f, "{}", Said(&standing.of_placement(), by)),
            Self::EmptyProduct { unplaced } => {
                f.write_str("nothing is placed in the world, so the document has no product")?;
                if !unplaced.is_empty() {
                    let said: Vec<String> = unplaced.iter().map(|&v| by.output(v)).collect();
                    write!(f, " — unplaced: {}", said.join(", "))?;
                }
                write!(
                    f,
                    ". {}",
                    crate::sentence::Recourse("place a body in the world")
                )
            }
            Self::StrandedPlacement { placement } => write!(
                f,
                "{} reads a body that is gone, so its copy is absent. {}",
                root(*placement),
                crate::sentence::Recourse("re-point it at a body, or delete it")
            ),
            Self::Unplaced { groups } => {
                f.write_str(
                    "no placement's copy lives in the world: the placed material lives in \
                     the own spaces of unplaced groups —",
                )?;
                for (i, (group, cause)) in groups.iter().enumerate() {
                    let sep = if i == 0 { "" } else { ";" };
                    write!(
                        f,
                        "{sep} the group rooted at {}, because {}",
                        by.node(*group),
                        crate::spoken::Said(cause, by)
                    )?;
                }
                write!(
                    f,
                    ". {}",
                    crate::sentence::Recourse(crate::mate::UNPLACED_RECOURSE)
                )
            }
            Self::Graft { node, source } => write!(
                f,
                "the kernel could not graft {}'s body: {source}",
                root(*node)
            ),
            Self::RootInvalid { findings } => {
                // Findings arrive in gather order, so one root's outputs
                // are adjacent and `dedup` counts placements.
                let mut roots: Vec<RecipeNodeId> = findings.iter().map(|s| s.node).collect();
                roots.dedup();
                match labels {
                    Labels::Kept => {
                        let noun = if roots.len() == 1 {
                            "placement"
                        } else {
                            "placements"
                        };
                        write!(f, "{} {noun} not valid at rest:", roots.len())?;
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
                        if let [one] = roots.as_slice() {
                            write!(f, "{} is not valid at rest:", root(*one))?;
                        } else {
                            let named: Vec<String> =
                                roots.iter().map(|id| root(*id).to_string()).collect();
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
                root(*node),
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
/// One variant per [`ProductError`] arm, except [`ProductError::Root`],
/// whose standing decides which refusal it is to a caller: one variant
/// per state the feature tree draws a root in — no entry, failed,
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
    /// [`ProductError::Root`] with no entry for the root:
    /// [`NodeStanding::NotEvaluated`] or [`NodeStanding::NotInDocument`].
    UnknownNode,
    /// [`ProductError::Root`] carrying [`NodeStanding::Failed`].
    RootFailed,
    /// [`ProductError::Root`] carrying [`NodeStanding::Poisoned`].
    RootPoisoned,
    /// [`ProductError::EmptyProduct`].
    EmptyProduct,
    /// [`ProductError::StrandedPlacement`].
    StrandedPlacement,
    /// [`ProductError::Unplaced`].
    Unplaced,
    /// [`ProductError::Graft`].
    Graft,
    /// [`ProductError::RootInvalid`].
    RootInvalid,
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
    /// sketches, datums and unplaced bodies.
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
            Self::Unplaced
            | Self::StrandedPlacement
            | Self::EvaluationOfAnotherDocument
            | Self::UnknownNode
            | Self::RootFailed
            | Self::RootPoisoned
            | Self::Graft
            | Self::RootInvalid
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
            Self::Root(standing) => match standing {
                NodeStanding::NotEvaluated { .. } | NodeStanding::NotInDocument { .. } => {
                    ProductErrorKind::UnknownNode
                }
                NodeStanding::Failed { .. } => ProductErrorKind::RootFailed,
                NodeStanding::Poisoned { .. } => ProductErrorKind::RootPoisoned,
            },
            Self::EmptyProduct { .. } => ProductErrorKind::EmptyProduct,
            Self::StrandedPlacement { .. } => ProductErrorKind::StrandedPlacement,
            Self::Unplaced { .. } => ProductErrorKind::Unplaced,
            Self::Graft { .. } => ProductErrorKind::Graft,
            Self::RootInvalid { .. } => ProductErrorKind::RootInvalid,
            Self::ProductInvalid { .. } => ProductErrorKind::ProductInvalid,
            Self::ContactLineage { .. } => ProductErrorKind::ContactLineage,
        }
    }
}

/// The body-denoting sources one root contributes, in gather order,
/// each tagged with the OUTPUT-BODY INDEX it occupies in the root's own
/// value (module docs). That index is what a root's name table keys its
/// rows by, so carrying it here is what lets [`product_named`] find the
/// rows belonging to each grafted body. `None` for a root that denotes
/// no body at all.
///
/// Each source also carries the DECLARED CONTACT RECORDS keyed in that
/// body's arena (ASM-R2b D-1). This function is the ONE place the
/// channel's two homes reconcile: a boolean's records ride its payload
/// (the `BooleanBody` contract, which predates the channel), every
/// other op's ride [`crate::eval::NodeValue::contacts`]. Downstream
/// therefore never asks which op put records where.
///
/// A [`crate::Node::Part`] is a `Body` value and contributes exactly
/// the body it selected. The half or instance it did NOT select is in
/// no product through it: a split or a pattern consumed by a Part is
/// no longer a sink, so it is no longer a root, and the product of a
/// document whose only root is a `Part(Above)` is that one half.
pub(crate) fn sources_of<T: Decide>(value: &NodeValue<T>) -> Option<Vec<Source0<T>>> {
    let carried = || Arc::clone(&value.contacts);
    let none = || Arc::new(ContactRecords::default());
    // The DECLARATION rows ride the value's own channel, filled at the
    // instantiate op alone. They are keyed in the same arena as
    // `value.contacts`, so they travel with exactly the arm that
    // travels those records — and nowhere else, because a body 0 that
    // is not the instantiated one has no declaration to carry.
    let rows = || Arc::clone(&value.carried);
    let norows = || Arc::new(crate::assembly::CarriedDeclarations::default());
    match &value.payload {
        ValuePayload::Body(body) => Some(vec![(0, Arc::clone(body), carried(), rows())]),
        ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) => {
            Some(vec![(0, Arc::clone(body), Arc::clone(contacts), norows())])
        }
        ValuePayload::Boolean(BooleanValue::Empty) => Some(Vec::new()),
        // Multi-output ops carry no records (the `OpOut` invariant):
        // "output body 0" names nothing here, so there is no home to
        // read from and none is invented.
        //
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
                    (ix, Arc::clone(body), none(), norows())
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
                        Some((half.output_body(), Arc::clone(body), none(), norows()))
                    }
                    SplitSide::Empty => None,
                })
                .collect(),
        ),
        // A12: the gather IGNORES a mate — it is a non-body root, and
        // "ignored by the gather" is exactly this arm. E3/E10's two
        // sinks join it for the same reason: a measured quantity and an
        // an assertion's verdict denote no material, so a document
        // whose only addition is an assertion has the same product it
        // had without one.
        ValuePayload::Datum(_)
        | ValuePayload::Profile(_)
        | ValuePayload::Mate(_)
        | ValuePayload::Gauge
        | ValuePayload::Measure { .. }
        | ValuePayload::MeasureUnavailable { .. }
        | ValuePayload::Assertion(_) => None,
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
/// `cfg(debug_assertions)`-gated, the shape `topo::source`'s bit
/// witnesses use. **That is not the same as "absent from a release
/// build" here**: this workspace's `[profile.release]` sets
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

/// The document's product: every body-denoting root's solids gathered,
/// in root-list order, into one [`Body`] (module docs). A root that
/// lives in an unplaced group's own space is not part of it (A9).
///
/// The result is a pure function of (`doc.roots()`, `evaluation`) — no
/// ambient state, so two evaluations of a root-neutral edit yield the
/// same solid order (D9).
///
/// # Errors
///
/// Every arm of [`ProductError`]: an evaluation of another document
/// ([`ProductError::EvaluationOfAnotherDocument`]); a root that
/// failed, was poisoned, or is absent from this evaluation; a document
/// whose roots denote no body ([`ProductError::NoBodyRoots`]); the
/// kernel's graft and at-rest validity refusals.
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
    /// kept ([`topo::AtRestBody`]): [`crate::assemble_gathered`] reads
    /// it and pays tier 3′'s census alone.
    pub body: topo::AtRestBody<T>,
    /// Its stable names, re-keyed onto the aggregate ([`product_named`]).
    pub names: NameTable,
    /// Its declared contacts, re-keyed onto the aggregate through the
    /// graft's own descendant map.
    pub contacts: ContactRecords,
    /// Which product ROOT contributed each of the aggregate's solids,
    /// in gather order (`crate::checks`'s separation resident is the
    /// consumer: it turns a kernel finding about two solid keys into a
    /// sentence about two roots).
    ///
    /// Read off the GRAFT's own minted-key list, exactly as the name
    /// and contact carries are — never re-derived by looking at the
    /// gathered geometry.
    pub solid_copies: Vec<SolidOrigin>,
    /// One row per mate of THIS document whose declaration the gather
    /// minted into `contacts`, in document order.
    ///
    /// The rows are the attribution channel and nothing else: the
    /// records themselves carry arena keys, so a finding against one
    /// needs this list to say which mate authored it. A declaration
    /// that arrived from a sub-assembly has no row HERE — its mate
    /// belongs to another document, so it rides `carried` below, which
    /// keeps that document and the route with it.
    pub minted: Vec<crate::assembly::MintedDeclaration>,
    /// One row per live mate the gather could NOT mint, in document
    /// order ([`crate::MintRefusal`]).
    ///
    /// Recorded rather than refused: neither refusal changes what
    /// material the document denotes, so the product stands and
    /// [`crate::assemble`] is the door that turns the first row into
    /// its typed refusal.
    pub unminted: Vec<crate::assembly::MintRefusal>,
    /// One row per declaration a document BELOW this one minted,
    /// re-keyed onto the aggregate through the graft's descendant map
    /// and tagged with the route it arrived by
    /// ([`crate::CarriedDeclaration`]).
    ///
    /// The records themselves already crossed the seam on `contacts`;
    /// these are the rows that say WHOSE mate authored each of them,
    /// so a finding against a carried record names a mate and a file
    /// instead of nobody. A sub-assembly's own carried rows carry up
    /// again, their route extended.
    pub carried: Vec<crate::assembly::CarriedDeclaration>,
    /// One row per mate a document below this one could NOT mint
    /// ([`crate::CarriedRefusal`]), carried verbatim: inner mint
    /// health is what the outermost gate refuses over, and the gather
    /// is where it arrives.
    pub carried_unminted: Vec<crate::assembly::CarriedRefusal>,
    /// **Each unplaced group's own space, gathered by itself** (A9,
    /// A11 (2)), in document order: what the world leaves out, kept beside
    /// it so the at-rest gate checks every space of the document from
    /// the one product it is handed ([`crate::assemble_gathered`]).
    /// Empty on a space's own gather, and on a document every group of
    /// which is placed.
    pub spaces: Vec<OwnSpace<T>>,
}

/// **One unplaced group's own space, gathered by itself** (A9,
/// A11 (2)): the roots that live in it, gathered and minted with
/// nothing outside the group.
#[derive(Debug)]
pub struct OwnSpace<T: Decide> {
    /// The group, by its root.
    pub group: RecipeNodeId,
    /// Why nothing places it.
    pub cause: crate::mate::Unplaced,
    /// The space's gather, kept rather than raised: a space whose
    /// roots denote no body ([`ProductError::NoBodyRoots`], an
    /// instance only a measure in the group reads) holds nothing to
    /// check, and any other refusal is the gate's to raise
    /// ([`crate::AssemblyError::Space`]).
    pub gather: Result<Box<Product<T>>, ProductError>,
}

/// **Every unplaced group's own space** in `evaluation`, each gathered
/// by itself ([`OwnSpace`]), in document order: what [`product_recorded`]
/// carries beside the world ([`Product::spaces`]).
pub fn own_spaces<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Vec<OwnSpace<T>> {
    evaluation
        .unplaced_groups()
        .into_iter()
        .map(|(group, cause)| OwnSpace {
            group,
            cause,
            gather: product_in(
                doc,
                evaluation,
                tol,
                crate::mate::Space::Own { group, cause },
            )
            .map(Box::new),
        })
        .collect()
}

/// One gathered solid's origin: the root that contributed it, that
/// root's output-body index, and the solid's key in the aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolidOrigin {
    /// The product root the solid came from.
    pub node: RecipeNodeId,
    /// The output-body index within that root's value.
    pub output: u32,
    /// The solid's key in the gathered aggregate.
    pub solid: topo::SolidKey,
}

/// The document's product, with the product's own NAME TABLE: every
/// gathered root's stable names, re-keyed onto the aggregate's entities
/// (ASM-2A D-4).
///
/// One implementation serves all three doors — this is
/// [`product_recorded`] with the contacts dropped.
///
/// The table carries FACE, EDGE and VERTEX rows. Body-kind rows are
/// deliberately absent: a root's body name denotes THAT ROOT's body,
/// and the product is not any root's body — it is the document's, a
/// distinct entity whose naming belongs to whoever mints it (an
/// instantiate node names its own placed body at itself).
///
/// # Errors
///
/// Every arm of [`ProductError`], including
/// [`ProductError::EvaluationOfAnotherDocument`] when `evaluation` is
/// not an evaluation of `doc`; [`ProductError::PlacedUnderTwoRoots`],
/// from the recipe before any root is read, when two roots place one
/// body through transforms and part selections; and
/// [`ProductError::Naming`] when two roots' rows would still name one
/// aggregate entity or collide on one name — never resolved silently.
pub fn product_named<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<(Body<T>, NameTable), ProductError> {
    product_recorded(doc, evaluation, tol).map(|p| (p.body.into_body(), p.names))
}

/// The document's product with its name table AND its declared contact
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
/// the scan-to-bless move F1 bans; there is no second opinion here
/// about which faces touch.
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
    product_in(doc, evaluation, tol, crate::mate::Space::World)
}

/// **The gather of one space** (A9, A11 (2)): the world's — the
/// document's product — or the own space of an unplaced group, which
/// the at-rest gate checks by itself ([`crate::assemble_gathered`]). A
/// root gathers into the space its value lives in
/// ([`Evaluation::space`]), and only the mates whose
/// two members both live in this space are minted: nothing outside an
/// unplaced group is compared with it.
///
/// # Errors
///
/// [`product_recorded`]'s.
pub(crate) fn product_in<P: crate::ProfilePayload, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
    space: crate::mate::Space,
) -> Result<Product<T>, ProductError> {
    #[cfg(debug_assertions)]
    GATHERS.with(|gathers| gathers.set(gathers.get().saturating_add(1)));
    // The pairing door (DI3), before the first root is read: this
    // gather is a statement about `doc`, and an evaluation of another
    // document answers about other geometry — silently, whenever the
    // two documents' node ids overlap, which two documents built from
    // one recipe always do.
    if let Some(m) = crate::ident::mispaired(doc.id(), evaluation.document) {
        return Err(m.into());
    }
    // The recipe's own refusal, before any value is read: a placement
    // whose body is gone defines no copy, and the copy it defined is
    // absent, which is never skipped.
    let placements = doc.placements();
    for &placement in &placements {
        if let Some(crate::node::Node::PlaceInWorld { body, .. }) = doc.node(placement)
            && doc.operation_of(*body).is_none()
        {
            return Err(ProductError::StrandedPlacement { placement });
        }
    }
    // Pass 1: every placement's copy, refused whole. "No partial
    // products" means a FAILED placement refuses even when a later one
    // would have supplied a body, so the whole list is read before
    // anything is grafted.
    let mut sources: Vec<Source<T>> = Vec::new();
    let mut any_placed = false;
    for &node in &placements {
        if evaluation.space(node) != space {
            continue;
        }
        any_placed = true;
        let value = evaluation.usable(node).map_err(ProductError::Root)?;
        let Some(bodies) = sources_of(value) else {
            unreachable!("a placement's copy is a body")
        };
        sources.extend(bodies.into_iter().map(|(ix, body, contacts, rows)| {
            (
                node,
                ix,
                body,
                Arc::clone(&value.name_table),
                contacts,
                rows,
            )
        }));
    }
    if !any_placed {
        let groups = evaluation.unplaced_groups();
        return Err(
            if space == crate::mate::Space::World && !placements.is_empty() && !groups.is_empty() {
                ProductError::Unplaced { groups }
            } else {
                ProductError::EmptyProduct {
                    unplaced: doc.unplaced(),
                }
            },
        );
    }

    // Pass 2: the graft, one call per SOURCE BODY, in list order. A
    // source holding N solids goes through as one call: the keyed graft
    // is the N-solid door (#381), and its per-entity bridge is total
    // over the source however many solids it spans, so the carries
    // below are the same code for N as for 1.
    let mut aggregate = Body::new();
    let mut grafted: Vec<(&Source<T>, topo::GraftKeys)> = Vec::with_capacity(sources.len());
    for source in &sources {
        let (node, _, body, _, _, _) = source;
        // An empty source contributes nothing; the graft door refuses a
        // solidless body, so the skip is here rather than there.
        if body.solids().next().is_none() {
            continue;
        }
        let keys =
            topo::graft_disjoint_all_keyed(&mut aggregate, body.as_ref()).map_err(|source| {
                ProductError::Graft {
                    node: *node,
                    source: Box::new(source),
                }
            })?;
        grafted.push((source, keys));
    }

    // The at-rest gate, ONCE, on the aggregate — before any name is
    // carried, so a product that is not a valid body refuses as that
    // before any naming collision is read. Only a refusal pays for
    // attribution (`attribute_at_rest`). The verdict is kept with the
    // body, so the assembly gate over this product pays the census
    // alone.
    let aggregate = match T::gate_at_rest_kept(aggregate, tol) {
        Ok(gated) => gated,
        Err(errors) => {
            return Err(attribute_at_rest(
                grafted.iter().map(|(source, _)| *source),
                errors,
                tol,
            ));
        }
    };

    // Pass 3: the carries, per grafted source, each across the key
    // bridge its graft minted: the source's name rows, its contact
    // records, and the declarations below it.
    let mut names = NameTable::new();
    let mut contacts = ContactRecords::default();
    let mut solid_copies: Vec<SolidOrigin> = Vec::new();
    let mut carried: Vec<crate::assembly::CarriedDeclaration> = Vec::new();
    let mut carried_unminted: Vec<crate::assembly::CarriedRefusal> = Vec::new();
    let mut tie_rows = CarriedRows::default();
    for ((node, ix, _, table, records, rows), keys) in &grafted {
        solid_copies.extend(keys.solids().iter().map(|&solid| SolidOrigin {
            node: *node,
            output: *ix,
            solid,
        }));
        carry_names(&mut tie_rows, table, *node, *ix, keys);
        carry_contacts(&mut contacts, records, keys)
            .map_err(|what| ProductError::ContactLineage { node: *node, what })?;
        carry_declarations(&mut carried, &rows.minted, keys)
            .map_err(|what| ProductError::ContactLineage { node: *node, what })?;
        // A refusal names no entity — it is a mate that produced NO
        // record — so it carries with nothing to re-key.
        carried_unminted.extend(rows.unminted.iter().cloned());
    }
    // The name carry's second half, after the last source: every
    // carried row, narrowed ONCE over all of them (`carry_names`). A tie
    // whose candidates the document separated into different SOURCES is
    // one tie of the product — the product holds both faces — and this
    // is where that is written, because no single source can see it.
    //
    // A refusal here names the node that MINTED the colliding name
    // rather than a root: every repeated pair refused in the carry
    // above, so what is left belongs to no one root.
    if let Err(e) = tie_rows.finish(&mut names) {
        unreachable!(
            "two copies carried one name, {:?}: a copy's names are qualified by its placement",
            e.name
        )
    }
    // Pass 4: MINTING (A3's "Declaration minting"). Every evaluated
    // product carries its own mates' declarations, so what a document
    // MEANS includes what its mates say about the material — which is
    // what lets the instantiation seam compose: a sub-assembly's
    // declarations ride up on the contacts channel above, keyed by the
    // graft, rather than being lost because the consuming document
    // never ran the mate loop.
    //
    // Last, and after the gate: minting resolves references against
    // the FINISHED name table, and the aggregate's own at-rest verdict
    // is about geometry, so a product that is not a body at all
    // refuses before any mate is read.
    let (minted, unminted) = crate::assembly::mint(doc, evaluation, &names, &mut contacts, space);
    let spaces = match space {
        crate::mate::Space::World => own_spaces(doc, evaluation, tol),
        crate::mate::Space::Own { .. } => Vec::new(),
    };
    Ok(Product {
        document: doc.id(),
        body: aggregate,
        names,
        contacts,
        solid_copies,
        minted,
        unminted,
        carried,
        carried_unminted,
        spaces,
    })
}

/// One body the gather will graft: which root contributed it, which
/// OUTPUT-BODY index it occupies in that root's value (the index its
/// name rows are keyed by), the body, the root's name table, the
/// body's declared contact records, and the declaration rows that say
/// whose mate authored each carried record.
type Source<T> = (
    RecipeNodeId,
    u32,
    Arc<Body<T>>,
    Arc<NameTable>,
    Arc<ContactRecords>,
    Arc<crate::assembly::CarriedDeclarations>,
);

/// **Which roots the aggregate's at-rest refusal is about**: every
/// source body re-gated on its own, in gather order, and each that
/// fails named with its own findings ([`ProductError::RootInvalid`]).
/// When every source passes, the refusal is the aggregate's
/// ([`ProductError::ProductInvalid`], a graft defect).
///
/// Re-gated rather than read off `aggregate`'s findings, for two
/// reasons. Many [`ValidationError`] arms carry no key a solid can be
/// recovered from, so a finding cannot always be mapped to the root
/// that caused it. And the battery's early stops are body-wide, so the
/// aggregate's list can omit one root's defect entirely: a structural
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
        .filter_map(|(node, output, body, _, _, _)| {
            T::gate_at_rest(body.as_ref(), tol)
                .err()
                .map(|errors| SourceFinding {
                    node: *node,
                    output: *output,
                    errors,
                })
        })
        .collect();
    if findings.is_empty() {
        ProductError::ProductInvalid { errors: aggregate }
    } else {
        ProductError::RootInvalid { findings }
    }
}

/// One body-denoting source as [`sources_of`] hands it back: output
/// index, body, the records keyed in that body's arena, and the
/// declaration rows keyed in the same arena.
pub(crate) type Source0<T> = (
    u32,
    Arc<Body<T>>,
    Arc<ContactRecords>,
    Arc<crate::assembly::CarriedDeclarations>,
);

/// Re-keys one grafted body's contact records onto the aggregate,
/// through the graft's DESCENDANT MAP (`product_recorded`'s contacts
/// carry). INVARIANT: every key here is `keys.<kind>(old)` — the
/// lineage the graft recorded. Nothing is looked up by position, by
/// name, or by re-measuring the aggregate.
///
/// The bridge is total over a source it grafted, so a missing image is
/// a bridge bug, not a dropped declaration: it refuses typed rather
/// than silently weakening the at-rest gate the records feed.
///
/// [`ProductError::ContactLineage`] is therefore DEFENSIVE and has no
/// test: no public API can reach it without first breaking the graft's
/// own key-bridge contract. Stated rather than left to be discovered —
/// an arm that cannot be exercised is worth knowing about, and the
/// alternative (dropping the record) is the failure this guards.
fn carry_contacts(
    into: &mut ContactRecords,
    from: &ContactRecords,
    keys: &topo::GraftKeys,
) -> Result<(), &'static str> {
    use topo::Cell;
    let moved = from
        .rekeyed(|cell| match cell {
            Cell::Vertex(v) => keys.vertex(v).map(Cell::Vertex),
            Cell::Edge(e) => keys.edge(e).map(Cell::Edge),
            Cell::Face(f) => keys.face(f).map(Cell::Face),
        })
        .map_err(|cell| match cell {
            Cell::Vertex(_) => "vertex",
            Cell::Edge(_) => "edge",
            Cell::Face(_) => "face",
        })?;
    // Every field, by name: a new record kind fails to compile here
    // until it is carried.
    let ContactRecords {
        vv,
        a_on_b,
        b_on_a,
        ve,
        ee,
        curves,
        patches,
    } = moved;
    into.vv.extend(vv);
    into.a_on_b.extend(a_on_b);
    into.b_on_a.extend(b_on_a);
    into.ve.extend(ve);
    into.ee.extend(ee);
    into.curves.extend(curves);
    into.patches.extend(patches);
    Ok(())
}

/// Re-keys one grafted body's carried DECLARATION rows onto the
/// aggregate — [`carry_contacts`]'s twin, under its lineage rule and
/// its refusal, for the bookkeeping that says whose mate authored each
/// record. Dropping a row here would leave its RECORD in the set with
/// nothing to attribute it to, which is the anonymity this channel
/// exists to end.
fn carry_declarations(
    into: &mut Vec<crate::assembly::CarriedDeclaration>,
    from: &[crate::assembly::CarriedDeclaration],
    keys: &topo::GraftKeys,
) -> Result<(), &'static str> {
    for row in from {
        let (a, b) = row.declaration.faces;
        into.push(crate::assembly::CarriedDeclaration {
            declaration: crate::assembly::MintedDeclaration {
                faces: (keys.face(a).ok_or("face")?, keys.face(b).ok_or("face")?),
                ..row.declaration.clone()
            },
            ..row.clone()
        });
    }
    Ok(())
}

/// Re-keys one grafted body's name rows onto the aggregate: the same
/// stable names, pointing at the entities the graft minted. Body index
/// on the product side is 0 — the product is ONE body.
///
/// The KEY MAP is this function's own — it is the graft's descendant
/// map, plus the product's rule that a root body-row does not carry
/// (see `product_named`: the product's own body is nobody's root
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
            "{node:?} carried {:?} twice: a copy's names are qualified by its placement",
            e.name
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
    /// [`ProductError::Root`] — a CENSUS, not a sample: every payload
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
            ProductError::Root(NodeStanding::NotEvaluated { node }),
            ProductError::Root(NodeStanding::NotInDocument { node }),
            ProductError::Root(NodeStanding::Failed { node }),
            ProductError::Root(NodeStanding::Poisoned { node, through }),
            ProductError::EmptyProduct {
                unplaced: vec![crate::VarId::new(0, test_utils::refusal::tagged(1))],
            },
            ProductError::StrandedPlacement { placement: node },
            ProductError::Unplaced {
                groups: vec![(node, crate::mate::Unplaced::NoOffset)],
            },
            ProductError::Graft {
                node,
                source: Box::new(topo::BooleanError::UnrepresentableResult),
            },
            ProductError::RootInvalid {
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
                ProductErrorKind::RootFailed => "RootFailed",
                ProductErrorKind::RootPoisoned => "RootPoisoned",
                ProductErrorKind::EmptyProduct => "EmptyProduct",
                ProductErrorKind::StrandedPlacement => "StrandedPlacement",
                ProductErrorKind::Unplaced => "Unplaced",
                ProductErrorKind::Graft => "Graft",
                ProductErrorKind::RootInvalid => "RootInvalid",
                ProductErrorKind::ProductInvalid => "ProductInvalid",
                ProductErrorKind::ContactLineage => "ContactLineage",
            }
        }
        let expected = [
            "EvaluationOfAnotherDocument",
            "UnknownNode",
            "UnknownNode",
            "RootFailed",
            "RootPoisoned",
            "EmptyProduct",
            "StrandedPlacement",
            "Unplaced",
            "Graft",
            "RootInvalid",
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
            vec![format!(
                "{:?}",
                ProductError::EmptyProduct {
                    unplaced: vec![crate::VarId::new(0, test_utils::refusal::tagged(1))],
                }
            )],
            "an empty world is the only gather refusal that is an absence \
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

    /// **A root with no value is refused with its standing**, under the
    /// gather's subject: the standing names the node, its state and the
    /// repair, so the gather adds only that the node is a root.
    #[test]
    fn a_root_without_a_value_renders_its_standing() {
        let standing = NodeStanding::Poisoned {
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(4)),
            through: RecipeNodeId::new(0, test_utils::refusal::tagged(2)),
        };
        assert_eq!(
            ProductError::Root(standing).to_string(),
            format!("product: placement {standing}")
        );
    }

    /// **The bare sentence names every failing root in its header**,
    /// each as a root, so a carrier that draws no per-line subject
    /// still says which roots failed.
    #[test]
    fn the_bare_root_invalid_header_names_each_root() {
        let source = |node: u64| SourceFinding {
            node: RecipeNodeId::new(0, test_utils::refusal::tagged(node)),
            output: 0,
            errors: vec![topo::ValidationError::NegativeVolume {
                solid: topo::SolidKey::default(),
            }],
        };
        let two = ProductError::RootInvalid {
            findings: vec![source(3), source(5)],
        };
        assert!(
            two.sentence().to_string().starts_with(
                "placement 000000000003, placement 000000000005 are not valid at rest:\n  "
            ),
            "{}",
            two.sentence()
        );
        let one = ProductError::RootInvalid {
            findings: vec![source(3)],
        };
        assert!(
            one.sentence()
                .to_string()
                .starts_with("placement 000000000003 is not valid at rest:\n  "),
            "{}",
            one.sentence()
        );
    }
}
