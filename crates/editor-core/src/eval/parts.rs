//! The instantiated parts an evaluation reads (ASM-2A D-3).
//!
//! A2 says evaluation MATERIALIZES: an instantiate node evaluates the
//! document it pins and takes that document's A10 product. N instances
//! of one part must not evaluate it N times, so each evaluation enters
//! every part it instantiates once, into one table its nodes read, and
//! carries the successful rows to the run that names it as its prior.
//!
//! # Three hash vocabularies, deliberately not unified
//!
//! The key here is `(DocRef, ambient ε)` — the reference's own pin plus
//! the tolerance every predicate below it decided at. It is NOT the
//! [`crate::eval::ContentKey`] memo key (a process-internal FNV digest
//! over resolved slot values, deliberately not collision-resistant),
//! and it is NOT the [`crate::ident::ContentPin`] itself (a SHA-256 of
//! authored canonical bytes, which says nothing about the tolerance an
//! evaluation ran at). Each answers a different question, and unifying
//! any two would answer one of them wrongly.
//!
//! # A document's parts are entered before it evaluates
//!
//! At every depth, an evaluation reads its parts from a [`Reached`]
//! table filled before its mate solve and its node map run, by `&`,
//! with no lock: a node map on rayon workers reads it as the serial
//! walk does. Nothing evaluates a part on a miss; a miss is the typed
//! kernel defect [`PartFault::NotEntered`], an ask that escaped
//! [`instantiated`], the one census of the asks.
//!
//! The top fills its table in one serial pass
//! ([`PartStore::enter_census`]) over that census, in document order:
//! each distinct key evaluated at most once, under a shielding verdict
//! bracket ([`PartStore::row`]), so its decisions land on no node and
//! its `probe` samples reach the caller's sink ahead of every node's,
//! whatever schedule the node map then runs. Each successful row is
//! carried on the [`super::Evaluation`] ([`Carried`]) and reused by the
//! run that names that evaluation as its prior — the node memo's
//! argument: the pin says which content (A4), and the same content at
//! the same ε evaluates to the same product (D9). So an unchanged
//! assembly, or one whose edit moved only a pose, evaluates no part. A
//! faulted row is not carried: the next run asks again.
//!
//! # The descent runs on the heap
//!
//! Below the top, the descent is bottom-up on an explicit stack
//! (`PartStore::resolve_and_evaluate`): a referenced document is
//! evaluated once every part it instantiates has its row, and its
//! table starts with those rows. So one nested evaluation is on the
//! thread's stack at a time, how deep an assembly nests costs heap and
//! never stack, and every depth either evaluates or refuses typed.
//!
//! **Every part a document instantiates is evaluated, whether or not
//! its instance asks.** The instance that declines to ask is one whose
//! placement refused (a mate fault), or a memo hit; its part's row,
//! failed or not, reaches no node, product or refusal. What it does
//! reach is what counts or records work as it happens:
//! `part_evaluations` counts it, and a shape report, a symbolic
//! session's counts or a sample sink installed around the evaluation
//! sees its decisions — and sees every nested document's decisions
//! bottom-up, a part's before its instantiator's.
//!
//! # Cycles are decided, not waited out
//!
//! The descent also carries its CHAIN — the references each document
//! on it was reached through. A reference already in the chain is
//! a cycle by A4's own rule (same id, same pin ⇒ same content), so it
//! refuses immediately, NAMING the loop, and that check runs before
//! the depth check, so a loop that closes at [`MAX_DEPTH`] is still
//! named as a loop.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::Decide;

use crate::ProfileDoc;
use crate::checks::at_rest::PartRow;
use crate::ident::DocRef;
use crate::names::NameTable;
use crate::node::RecipeNodeId;
use crate::part::{PartResolver, ResolveFault};
use crate::sentence::{PASS_A_RESOLVER, Recourse};
use crate::spoken::{HeldNodes, Say, Speaker};
use geom_core::Tol;

/// **How deep an assembly may nest**: a document `MAX_DEPTH` documents
/// below the top evaluates, and one more refuses with
/// [`PartFault::DepthExceeded`], whose sentence states this number and
/// whose recourse is to flatten the assembly.
///
/// An author reaches it with a file: any acyclic chain of references
/// that long. CYCLES are decided before it, structurally, by the
/// descent chain (see [`PartStore`]) — a revisited reference refuses
/// NAMING the cycle — so reaching the bound means the chain was long,
/// not that it looped.
pub(crate) const MAX_DEPTH: usize = 1024;

/// A resolved part: the referenced document's product body and its
/// part-local stable names, what that body carries
/// ([`super::Carry`]: its CONTACT RECORDS, ASM-R2b D-1, in the same keys
/// as its geometry, and its pieces), and the part's rows, which cross
/// the document seam routed through the instance copying it
/// ([`Self::carry_through`]).
///
/// Each row list (`interference`, `at_rest`, `door_rows`) is the
/// referenced document's OWN rows first, then what it carried up from
/// ITS parts, each as a [`PartRow`] that already holds the document's
/// nodes it names. Instantiation adds its own instance to the route;
/// nothing below is re-read.
pub(crate) struct PartValue<T: Decide> {
    /// The product body, in the frame the part's product computed in.
    pub body: super::Framed<T>,
    pub names: Arc<NameTable>,
    /// What the product body carries, its rows aside: the census that
    /// decided the meetings inside it is the part's.
    pub carry: super::Carry<T>,
    /// **The part's own at-rest census verdict**, its own rows first and
    /// then those it carried up: the overlaps between its copies,
    /// reported and never refused, as its own at-rest check reads them
    /// (`checks::at_rest::reading`).
    pub interference: Arc<
        Vec<PartRow<crate::checks::at_rest::Pinned<crate::checks::at_rest::InterferenceFinding>>>,
    >,
    /// The census findings its gate refuses, its own first: inner
    /// at-rest health is the outermost gate's business.
    pub at_rest:
        Arc<Vec<PartRow<crate::checks::at_rest::Pinned<crate::checks::at_rest::AtRestEvidence>>>>,
    /// **The rows its coincidence door owes an answer about**, its own
    /// first and then those it carried up, each with the document it is
    /// of: unproven, since the door's replay runs at rest, over that
    /// document, when a report asks (`checks::DoorJob`).
    pub door_rows:
        Arc<Vec<PartRow<crate::checks::at_rest::Pinned<crate::checks::at_rest::DocRows>>>>,
}

impl<T: Decide> PartValue<T> {
    /// **What the copy instance `through` defines carries**: the
    /// product body's carry, and the part's rows, each the part's,
    /// routed through `through` ([`PartRow::through`]). The door rows
    /// ride unjudged: they are the part's, named in its document, and
    /// judged over it where a report asks.
    pub(crate) fn carry_through(&self, through: RecipeNodeId) -> super::Carry<T> {
        use crate::checks::at_rest as a;
        let findings = a::CarriedFindings {
            interference: self
                .interference
                .iter()
                .map(|row| {
                    let (route, pinned, _) = row.through(through);
                    a::CarriedInterference {
                        route,
                        pin: pinned.pin,
                        finding: pinned.finding,
                    }
                })
                .collect(),
            at_rest: self
                .at_rest
                .iter()
                .map(|row| {
                    let (route, pinned, held) = row.through(through);
                    a::CarriedAtRestFinding {
                        route,
                        pin: pinned.pin,
                        finding: pinned.finding,
                        held,
                    }
                })
                .collect(),
            coincidences: self
                .door_rows
                .iter()
                .map(|row| {
                    let (route, pinned, _) = row.through(through);
                    (route, pinned).into()
                })
                .collect(),
        };
        super::Carry {
            findings: Arc::new(findings),
            ..self.carry.clone()
        }
    }
}

impl<T: Decide> Clone for PartValue<T> {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            names: Arc::clone(&self.names),
            carry: self.carry.clone(),
            interference: Arc::clone(&self.interference),
            at_rest: Arc::clone(&self.at_rest),
            door_rows: Arc::clone(&self.door_rows),
        }
    }
}

/// Why an instantiation could not produce a part body. Cloneable and
/// self-contained: the table holds one of these per reference, so
/// every instance of a broken part reports the same typed cause.
///
/// An arm that names the part's nodes keeps them as the pinned part
/// holds them (`held`), so every frame says them with the part's labels
/// (DESIGN.md Band 1, "Node labels"). Equality compares those labels
/// too, deliberately: two faults with equal ids from parts labelled apart
/// (two pins, since a label is in the pin) are unequal, here and in the
/// refusals that hold a `PartFault` (`ReachRefusal`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartFault {
    /// The evaluation carries no resolver, so the document seam cannot
    /// be crossed at all (D-3).
    NoResolver,
    /// The resolver refused, with its own classification (A2's ε seam
    /// and A4's pin gate among them).
    Unresolved {
        /// Which seam rule failed.
        fault: ResolveFault,
        /// The resolver's diagnosis.
        message: String,
    },
    /// The referenced document evaluated, but a node its PRODUCT reads
    /// failed — the ordinary "my part is broken, why?" path.
    ///
    /// The nested [`super::Evaluation`] dies with the resolution that
    /// produced it, so a caller can never be sent to look at it: the
    /// node's own refusal travels here instead, typed and unaltered. A
    /// node that is itself an instantiate node carries
    /// [`super::NodeErrorKind::Part`] in turn, so a part inside a part
    /// is a chain of these however many documents deep, and every
    /// level keeps the reference it crossed.
    ///
    /// The sentence is the instance's own and never renders `refusal`:
    /// that is another node's refusal, in another document, with its
    /// own recourse, and it is drawn as its own line (the exception
    /// written over [`super::NodeErrorKind`]'s `Display`).
    PartProductFailed {
        /// The failing node, in the REFERENCED document's id space.
        node: RecipeNodeId,
        /// That node's own refusal.
        refusal: super::NodeRefusal,
        /// The part's nodes this fault names, as the pinned part holds
        /// them ([`PartFault::held`]).
        held: Arc<HeldNodes>,
    },
    /// The referenced document evaluated, but a node its PRODUCT reads
    /// never ran: a node upstream of it inside the part failed. The
    /// part's typical broken shape — a transform or a boolean over the
    /// node that failed.
    ///
    /// `through` is the node the author repairs, so its refusal is the
    /// one carried, as [`Self::PartProductFailed`] carries a failed
    /// node's; `node` says which node the part's product reads it cost.
    PartProductPoisoned {
        /// The poisoned node, in the REFERENCED document's id space.
        node: RecipeNodeId,
        /// Its nearest failed ancestor, in the same id space.
        through: RecipeNodeId,
        /// That ancestor's own refusal.
        refusal: super::NodeRefusal,
        /// The part's nodes this fault names, as the pinned part holds
        /// them ([`PartFault::held`]).
        held: Arc<HeldNodes>,
    },
    /// The product door named a node as failed — a failed node, or a
    /// poisoned node's failed ancestor — and the evaluation it read
    /// holds no failure there. The two disagree about one node, so this
    /// is a kernel bug, reported typed rather than as a failure with no
    /// cause.
    PartProductFailureUnrecorded {
        /// The node the product door named as failed, in the
        /// REFERENCED document's id space.
        node: RecipeNodeId,
        /// The part's nodes this fault names, as the pinned part holds
        /// them ([`PartFault::held`]).
        held: Arc<HeldNodes>,
    },
    /// The referenced document has no product for a reason that is not
    /// a failed or poisoned node (a product with no body, an invalid
    /// gather, a name collision).
    ///
    /// The gather's refusal crosses whole, its ids the part's: its
    /// class ([`crate::ProductRefusal::kind`]) is what a consumer
    /// branches on.
    PartProduct {
        /// The product door's refusal, in the REFERENCED document's id
        /// space.
        refusal: crate::product::ProductRefusal,
        /// The part's nodes this fault names, as the pinned part holds
        /// them ([`PartFault::held`]).
        held: Arc<HeldNodes>,
    },
    /// The reference CHAIN returned to a document it had already
    /// entered — the same (id, pin), so the same content: descending
    /// further would repeat forever.
    ///
    /// A4 makes this unconstructible through an honest store (a
    /// document would have to contain its own hash), so the fault names
    /// a resolver that checks no pins. Every shipped resolver checks
    /// them, so it renders as a kernel defect. It carries the loop
    /// itself, first repeated reference through last, because the loop
    /// is the diagnosis.
    ReferenceCycle {
        /// The cycle, starting and ending at the repeated reference.
        cycle: Vec<DocRef>,
    },
    /// Instantiation nested past [`MAX_DEPTH`] without repeating a
    /// reference: the assembly nests deeper than the bound allows.
    DepthExceeded,
    /// An evaluation asked for a part that was not entered before it
    /// evaluated. Every evaluation's parts are entered from
    /// [`instantiated`]'s census, which names every reference an
    /// evaluation can ask for, so this is a kernel defect: an ask that
    /// escaped that census.
    NotEntered,
}

impl PartFault {
    /// **The refusal this fault carries, when it carries one**: a
    /// failed node, or a poisoned node's failed ancestor, and that
    /// node's own refusal, in the referenced document's id space ([`super::NodeErrorKind::carried`]). Exhaustive, for the
    /// reason [`crate::MateFault::carried`] states.
    #[must_use]
    pub fn carried(&self) -> Option<(RecipeNodeId, &super::NodeRefusal)> {
        self.carried_held()
            .map(|(node, refusal, _)| (node, refusal))
    }

    /// [`PartFault::carried`], with the part's nodes the fault holds.
    pub(crate) fn carried_held(&self) -> Option<(RecipeNodeId, &super::NodeRefusal, &HeldNodes)> {
        match self {
            Self::PartProductFailed {
                node,
                refusal,
                held,
            } => Some((*node, refusal, &**held)),
            Self::PartProductPoisoned {
                through,
                refusal,
                held,
                ..
            } => Some((*through, refusal, &**held)),
            Self::NoResolver
            | Self::Unresolved { .. }
            | Self::PartProductFailureUnrecorded { .. }
            | Self::PartProduct { .. }
            | Self::ReferenceCycle { .. }
            | Self::DepthExceeded
            | Self::NotEntered => None,
        }
    }
}

impl PartFault {
    /// **The part's nodes this fault names**, as the version its
    /// reference pins holds them: the nodes of its own sentence and of
    /// each level it carries in the part ([`super::CarriedLevel`]).
    /// `None` for an arm that names no node of the part.
    #[must_use]
    pub fn held(&self) -> Option<&HeldNodes> {
        match self {
            Self::PartProductFailed { held, .. }
            | Self::PartProductPoisoned { held, .. }
            | Self::PartProductFailureUnrecorded { held, .. }
            | Self::PartProduct { held, .. } => Some(&**held),
            Self::NoResolver
            | Self::Unresolved { .. }
            | Self::ReferenceCycle { .. }
            | Self::DepthExceeded
            | Self::NotEntered => None,
        }
    }

    /// **Who says this fault's nodes**: the part's, as it holds them
    /// ([`Speaker::held`]). Its ids are numbered in the part, so no
    /// other document says them.
    pub(crate) fn speaker(&self) -> Speaker<'_> {
        self.held().map_or(Speaker::TAG, Speaker::held)
    }

    /// The sentence, each of the part's nodes said by `by`: the fault's
    /// [`speaker`](Self::speaker), or the part itself while
    /// [`product_fault`] records them.
    fn say_by(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            // Raised only at an API door, each of which takes a
            // resolver; the viewer always carries one of its own.
            Self::NoResolver => write!(
                f,
                "no part resolver was given, so a referenced document cannot be reached. {}",
                Recourse(PASS_A_RESOLVER)
            ),
            // The resolver knows what went wrong in its store, so its
            // message states the recourse of a pin or a lookup. The ε
            // seam's is the same whatever the store.
            Self::Unresolved { fault, message } => match fault {
                ResolveFault::PinMismatch => {
                    write!(f, "the reference's pin does not hold: {message}")
                }
                ResolveFault::EpsilonSeam => write!(
                    f,
                    "the referenced document's recorded tolerance disagrees with this process's: \
                     {message}. {}",
                    Recourse(crate::part::EPSILON_SEAM_RECOURSE)
                ),
                ResolveFault::Unresolved => write!(f, "the reference did not resolve: {message}"),
            },
            Self::PartProductFailed { node, .. } => {
                let node = by.node(*node);
                write!(
                    f,
                    "the part's {node} failed, so the part has no body. {}",
                    InThePart(format_args!("repair {node}")),
                )
            }
            Self::PartProductPoisoned { node, through, .. } => {
                let through = by.node(*through);
                write!(
                    f,
                    "the part's {through} failed and poisoned {}, which its product reads, so \
                     the part has no body. {}",
                    by.node(*node),
                    InThePart(format_args!("repair {through}")),
                )
            }
            Self::PartProductFailureUnrecorded { node, .. } => {
                let node = by.node(*node);
                write!(
                    f,
                    "the part's product names its {node} as failed and the part's evaluation \
                     holds no failure there; the two disagree, so this is a kernel bug. {}",
                    InThePart(format_args!(
                        "see {node} as it evaluates, then report it with the part's file"
                    )),
                )
            }
            Self::PartProduct { refusal, .. } => {
                write!(
                    f,
                    "the part has no product: {}",
                    refusal.error().bare_said(by)
                )?;
                match product_recourse(refusal.kind()) {
                    ProductRecourse::KernelDefect => {
                        write!(f, ". {}", geom_core::KERNEL_DEFECT_ENDING)
                    }
                    ProductRecourse::Carried => Ok(()),
                }
            }
            Self::ReferenceCycle { cycle } => {
                write!(
                    f,
                    "the reference chain returns to a document it already entered: "
                )?;
                for (i, r) in cycle.iter().enumerate() {
                    if i > 0 {
                        write!(f, " -> ")?;
                    }
                    write!(f, "{r}")?;
                }
                // A pin is its document's hash, so a store that checks
                // pins cannot hold a loop: every door resolves through
                // one, and a loop is a resolver's defect.
                write!(f, ". {}", geom_core::KERNEL_DEFECT_ENDING)
            }
            Self::DepthExceeded => write!(
                f,
                "instantiation nested deeper than {MAX_DEPTH} documents without repeating a \
                 reference. {}",
                Recourse("flatten the assembly so its parts nest fewer documents deep")
            ),
            Self::NotEntered => write!(
                f,
                "the part was asked for by a document that evaluated before entering it, so \
                 the part has no body. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
        }
    }
}

/// The fault, each of the part's nodes as the pinned part holds it.
impl core::fmt::Display for PartFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say_by(f, self.speaker())
    }
}

/// **Every sentence a fault says in its part**: its own, then each
/// carried level in the part — the levels [`product_fault`] records the
/// nodes of. A level in a part below is that part's, and its own fault
/// holds its nodes.
struct InPart<'a>(&'a PartFault);

impl Say for InPart<'_> {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        self.0.say_by(f, by)?;
        let levels =
            super::CarriedChain::from_first(self.0.carried(), super::CarriedIn::ThisDocument);
        for level in
            levels.take_while(|level| matches!(level.document, super::CarriedIn::ThisDocument))
        {
            write!(f, "\n{}", level.refusal.line_at(level.node, by))?;
        }
        Ok(())
    }
}

/// The recourse of a repair the author makes inside the part: open it,
/// and act there.
struct InThePart<A>(A);

impl<A: core::fmt::Display> core::fmt::Display for InThePart<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            Recourse(format_args!("open the part and {}", self.0))
        )
    }
}

/// What a part with no product states after the gather's own sentence.
enum ProductRecourse {
    /// The gather's sentence already states the one recourse: its own,
    /// or the kernel refusal it forwards.
    Carried,
    /// A class a nested evaluation cannot reach.
    KernelDefect,
}

/// [`ProductRecourse`] by the gather's class. A nested evaluation runs
/// its own document to completion, and the failed and poisoned nodes
/// cross as their own arms, so the classes it cannot reach end in the
/// kernel-defect recourse.
fn product_recourse(kind: crate::product::ProductErrorKind) -> ProductRecourse {
    use crate::product::ProductErrorKind as K;
    match kind {
        K::EmptyProduct
        | K::StrandedProduct
        | K::ProductOutsideWorld
        | K::Graft
        | K::PieceInvalid
        | K::ProductInvalid => ProductRecourse::Carried,
        K::ContactLineage
        | K::Frame
        | K::EvaluationOfAnotherDocument
        | K::UnknownNode
        | K::ReadFailed
        | K::ReadPoisoned => ProductRecourse::KernelDefect,
    }
}

/// One cache row: the resolved part, or the typed reason there is
/// none. Keyed by `(DocRef, ambient ε bits)` — the module docs' key.
type Rows<T> = BTreeMap<(DocRef, u64), Result<PartValue<T>, PartFault>>;

/// The references whose evaluation ran out of member budget
/// ([`super::EvalOutcome::Exhausted`]), by a row's key. Such a reference
/// has no row: running out is the run's, not the part's, so it is
/// neither a value nor a fault, and the evaluation reading it stops at
/// admission at its instance node instead
/// ([`Reached::exhausted`]).
type Exhausted = std::collections::BTreeSet<(DocRef, u64)>;

/// A reference's evaluation: its row, or its run out of member budget.
enum Evaluated<T: Decide> {
    Row(Result<PartValue<T>, PartFault>),
    Exhausted,
}

/// **The serial part store**: rows filled one at a time, by `&mut`,
/// outside any node map — the top evaluation's census pass
/// ([`Self::enter_census`]). A running evaluation reads the [`Reached`]
/// table the pass hands over, never this.
pub(crate) struct PartStore<'a, T: Decide> {
    resolver: Option<&'a Arc<dyn PartResolver>>,
    /// The ambient ε these rows were produced at, by bits — half the
    /// key, hoisted because it is constant within one store.
    eps_bits: u64,
    /// The candidate-generation strategy the CALLER chose. Inherited
    /// across the seam so a differential run exercises the parts'
    /// booleans too — the strategy is a property of the run, not of
    /// the document, and results are bit-identical either way.
    boolean_sweep: topo::SweepStrategy,
    /// Whether the run binds every variable a symbol
    /// ([`super::EvalOptions::symbols`]), inherited for the same reason.
    symbols: bool,
    /// The second computing frame ([`super::EvalOptions::computing_twist`]),
    /// inherited so a part's constructions turn too (each by its own
    /// node's [`super::ComputingTwist::for_node`]).
    twist: Option<super::ComputingTwist>,
    /// The run's member budget ([`super::EvalOptions::member_budget`]),
    /// inherited: each part's evaluation is a run of its own under it.
    member_budget: u64,
    rows: Rows<T>,
    exhausted: Exhausted,
    /// How many referenced documents this store's descents evaluated,
    /// at every depth — the D-3 sharing evidence. A counter, not a
    /// timing claim. Every document below the top is evaluated by the
    /// top's descent, so the top's count is every crossing the run
    /// made.
    evaluations: usize,
}

impl<'a, T: Decide> PartStore<'a, T> {
    pub(crate) fn new(
        resolver: Option<&'a Arc<dyn PartResolver>>,
        boolean_sweep: topo::SweepStrategy,
        tol: Tol,
    ) -> Self {
        Self {
            resolver,
            eps_bits: tol.eps().to_bits(),
            boolean_sweep,
            symbols: false,
            twist: None,
            member_budget: super::DEFAULT_MEMBER_BUDGET,
            rows: BTreeMap::new(),
            exhausted: Exhausted::new(),
            evaluations: 0,
        }
    }

    /// The store, each part evaluated under `member_budget`.
    pub(crate) fn with_member_budget(self, member_budget: u64) -> Self {
        Self {
            member_budget,
            ..self
        }
    }

    /// This store, its descents binding every variable a symbol where
    /// `symbols`.
    pub(crate) fn with_symbols(self, symbols: bool) -> Self {
        Self { symbols, ..self }
    }

    /// The store, its parts evaluated in the run's second computing
    /// frame ([`super::EvalOptions::computing_twist`]).
    pub(crate) fn with_twist(self, twist: Option<super::ComputingTwist>) -> Self {
        Self { twist, ..self }
    }
}

/// **The top's parts, entered** ([`PartStore::enter_census`]).
pub(crate) struct Entry<T: Decide> {
    /// What the evaluation's mate solve and node map read.
    pub reached: Reached<T>,
    /// What the evaluation carries for the run after it.
    pub carried: Carried<T>,
    /// How many referenced documents the pass evaluated, at every
    /// depth.
    pub evaluations: usize,
}

impl<T: super::EvalScalar> PartStore<'_, T> {
    /// The part `doc_ref` denotes, evaluated on its first ask and kept.
    ///
    /// **The evaluation runs inside a shielding verdict bracket.** An
    /// instantiate node's log is the decisions its op made about its
    /// own content, the same whether or not its part was evaluated in
    /// this run (same content key ⇒ same decisions, D9), so the part's
    /// unbracketed decisions — its mate solve, its profile pre-passes,
    /// the gather of its product — must land on no instance. Its
    /// `probe` samples go to whatever sink the caller installed.
    fn row(&mut self, doc_ref: &DocRef, tol: Tol, cancel: &super::CancelToken) -> Evaluated<T> {
        let key = (*doc_ref, self.eps_bits);
        if let Some(hit) = self.rows.get(&key) {
            return Evaluated::Row(hit.clone());
        }
        if self.exhausted.contains(&key) {
            return Evaluated::Exhausted;
        }
        let value = {
            let _shield = geom_core::k_stats::Bracket::open();
            self.resolve_and_evaluate(doc_ref, tol, cancel)
        };
        match &value {
            Evaluated::Row(row) => {
                self.rows.insert(key, row.clone());
            }
            Evaluated::Exhausted => {
                self.exhausted.insert(key);
            }
        }
        value
    }

    /// **The top's census, entered**: one row per distinct reference
    /// `doc` instantiates ([`instantiated`]), in document order, each
    /// taken from `prior`'s carried rows or evaluated once
    /// ([`Self::row`]). Serial, and run before the evaluation's node
    /// map, so which part is evaluated, and where its
    /// samples land, is a property of the document and not of a
    /// schedule. The store's rows ARE the table it hands over, so it is
    /// consumed, and is called on a store fresh from [`Self::new`].
    ///
    /// A cancel stops the pass before its next evaluation; the rows it
    /// did not enter refuse [`PartFault::NotEntered`], and the
    /// evaluation, canceled, runs no node to ask. A row evaluated while
    /// `cancel` fired is not carried, nor is a faulted one.
    pub(crate) fn enter_census<P>(
        mut self,
        doc: &crate::Doc<P>,
        prior: Option<&Carried<T>>,
        cancel: &super::CancelToken,
        tol: Tol,
    ) -> Entry<T> {
        let mut carried = BTreeMap::new();
        for doc_ref in doc.ids().iter().filter_map(|&id| instantiated(doc, id)) {
            let key = (doc_ref, self.eps_bits);
            if self.rows.contains_key(&key) || self.exhausted.contains(&key) {
                continue;
            }
            if let Some(value) = prior.and_then(|p| p.0.get(&key)) {
                carried.insert(key, value.clone());
                self.rows.insert(key, Ok(value.clone()));
                continue;
            }
            if cancel.is_canceled() {
                break;
            }
            if let Evaluated::Row(Ok(value)) = self.row(&doc_ref, tol, cancel)
                && !cancel.is_canceled()
            {
                carried.insert(key, value);
            }
        }
        Entry {
            reached: Reached {
                rows: self.rows,
                exhausted: self.exhausted,
            },
            carried: Carried(carried),
            evaluations: self.evaluations,
        }
    }

    /// The descent below `doc_ref`, run BOTTOM-UP on an explicit stack.
    ///
    /// Every document on the way down is resolved and entered before
    /// any is evaluated, and each is evaluated once every part it
    /// instantiates has its row, which its evaluation then finds in its
    /// own cache (a [`Reached`]) instead of descending for. So the
    /// thread's stack holds one nested evaluation however deep the
    /// assembly nests, and [`MAX_DEPTH`] is the one bound on nesting.
    ///
    /// A document's rows are [`instantiated`]'s census of it, each
    /// decided against that document's own chain. So every row is the
    /// one a descent at the ask would produce: the same cycle and depth
    /// decisions, the same sharing within a document and none across
    /// two. Rows no instance asks for are evaluated too (the module
    /// docs say what that reaches).
    fn resolve_and_evaluate(
        &mut self,
        doc_ref: &DocRef,
        tol: Tol,
        cancel: &super::CancelToken,
    ) -> Evaluated<T> {
        let Some(resolver) = self.resolver else {
            return Evaluated::Row(Err(PartFault::NoResolver));
        };
        let mut path = Vec::new();
        let mut current = match Entered::enter(resolver, &path, doc_ref, tol) {
            Ok(entered) => entered,
            Err(fault) => return Evaluated::Row(Err(fault)),
        };
        path.push(*doc_ref);
        let mut waiting: Vec<Entered<T>> = Vec::new();
        loop {
            if let Some(child) = current.next_unreached(self.eps_bits) {
                match Entered::enter(resolver, &path, &child, tol) {
                    Ok(entered) => {
                        path.push(child);
                        waiting.push(core::mem::replace(&mut current, entered));
                    }
                    Err(fault) => {
                        current.reached.insert((child, self.eps_bits), Err(fault));
                    }
                }
                continue;
            }
            let reached = Reached {
                rows: core::mem::take(&mut current.reached),
                exhausted: core::mem::take(&mut current.exhausted),
            };
            let value = self.evaluate_entered(&current.doc, current.doc_ref, reached, cancel, tol);
            path.pop();
            let Some(parent) = waiting.pop() else {
                return value;
            };
            let done = core::mem::replace(&mut current, parent);
            let key = (done.doc_ref, self.eps_bits);
            match value {
                Evaluated::Row(row) => {
                    current.reached.insert(key, row);
                }
                Evaluated::Exhausted => {
                    current.exhausted.insert(key);
                }
            }
        }
    }

    /// One entered document's evaluation, over the parts it
    /// instantiates, reached.
    ///
    /// The rows its value hands up hold the nodes they name in `doc`
    /// ([`PartRow`]): this is the one place the version `doc_ref` pins
    /// is in hand, and the pin is in every instance's key, so no later
    /// label reaches them.
    fn evaluate_entered(
        &mut self,
        doc: &Arc<ProfileDoc>,
        doc_ref: DocRef,
        reached: Reached<T>,
        cancel: &super::CancelToken,
        tol: Tol,
    ) -> Evaluated<T> {
        self.evaluations += 1;
        // AQ4: the referenced document evaluates at its OWN parameters
        // — v1 instantiation takes no arguments. Sequentially, with a
        // fresh epoch: this run's identity is its own.
        let opts = super::EvalOptions {
            epoch: super::Epoch::mint(),
            parallel: false,
            boolean_sweep: self.boolean_sweep,
            resolver: self.resolver.map(Arc::clone),
            // NOT inherited, unlike the two rows above: a parameter box
            // is keyed by THIS document's variable ids, and a referenced
            // document is a different document with its own variables
            // (AQ4 — v1 instantiation takes no arguments). A box that
            // crossed the seam would name nothing there: an id is minted
            // by one document's chain and read by no other.
            param_box: None,
            // NOT inherited, by the same argument: a seed is one of THIS
            // document's variable ids. A part's geometry is constant
            // with respect to them (AQ4 — v1 instantiation takes no
            // arguments), which the unseeded nested run states exactly:
            // every tangent it carries is zero.
            seed: None,
            // Inherited, unlike the box and the seed: a symbolic run
            // binds EVERY variable a symbol (D10), a part's too, each
            // keyed by its own document as well as its id
            // (`analysis::var_env_symbolic`), so two documents' ids never
            // name one symbol.
            symbols: self.symbols,
            // The reference's own pin already says which version this is.
            pin: false,
            // Inherited: a part's constructions compute in frames of
            // their own too, each its node's variation of the twist
            // (`ComputingTwist::for_node`), so its product is gathered
            // over operands turned apart.
            computing_twist: self.twist,
            member_budget: self.member_budget,
        };
        let evaluation =
            super::evaluate_nested::<T>(doc, cancel, &opts, reached, &doc_ref.pin, tol);
        if let super::EvalOutcome::Exhausted { .. } = evaluation.outcome {
            return Evaluated::Exhausted;
        }
        // A2's uniformity: what a document MEANS is its product, one
        // rule everywhere. A failed node inside the part surfaces
        // through the product door's own typed refusal — and its
        // refusal travels with it, because the evaluation holding it
        // does not outlive this call.
        // A part is its PRODUCT, however many solids that product
        // holds. The count is not consulted here at all — the
        // placing path maps all N as one body and the gather grafts
        // them as N, so a narrower rule at this door would be a second
        // truth about what instantiating a document means.
        let product = match crate::product::product_recorded(doc, &evaluation, tol) {
            Ok(product) => product,
            Err(e) => return Evaluated::Row(Err(product_fault(e, evaluation, doc))),
        };
        // The whole product crosses the seam, not a slice of it: what
        // a document MEANS is its product, and its mates' identity and
        // mint health are as much part of that as its records are. The
        // `Arc`s are the row's, so every instance of one part shares
        // one row set.
        let parts = product.parts;
        let pieces = product.pieces.clone();
        // Its door rows, quiet as its own assertions quiet them, and
        // those its parts carried up; proven where a report asks.
        let own = crate::checks::door_rows(
            &**doc,
            &evaluation,
            &crate::checks::Subject::Product(&product),
            tol,
        );
        let door_rows = (!own.is_empty())
            .then(|| {
                PartRow::unspoken(
                    doc_ref.id,
                    crate::checks::at_rest::Pinned {
                        pin: doc_ref.pin,
                        finding: crate::checks::at_rest::DocRows {
                            doc: crate::checks::at_rest::PinnedDoc::new(Arc::clone(doc)),
                            sets: own.into(),
                        },
                    },
                )
            })
            .into_iter()
            .chain(
                crate::checks::carried_coincidences(&evaluation)
                    .into_iter()
                    .map(|row| {
                        PartRow::below(
                            &**doc,
                            row.route,
                            crate::checks::at_rest::Pinned {
                                pin: row.pin,
                                finding: row.rows,
                            },
                            Arc::default(),
                        )
                    }),
            )
            .collect();
        // The part's own verdict on what its censuses found, the
        // product's and each door's, read as its own at-rest check reads
        // them, so the document instantiating it reports each finding
        // once, as the part's.
        let crate::checks::at_rest::WholeReading {
            reading: seam,
            carried_interference,
            carried_at_rest,
        } = crate::checks::at_rest::whole_reading(&product, tol);
        Evaluated::Row(Ok(PartValue {
            interference: Arc::new(
                seam.interference
                    .into_iter()
                    .map(|finding| {
                        PartRow::unspoken(
                            doc_ref.id,
                            crate::checks::at_rest::Pinned {
                                pin: doc_ref.pin,
                                finding,
                            },
                        )
                    })
                    .chain(carried_interference.into_iter().map(|row| {
                        PartRow::below(
                            doc,
                            row.route,
                            crate::checks::at_rest::Pinned {
                                pin: row.pin,
                                finding: row.finding,
                            },
                            Arc::default(),
                        )
                    }))
                    .collect(),
            ),
            at_rest: Arc::new(
                seam.could_not_look
                    .into_iter()
                    .map(|finding| {
                        PartRow::own(
                            doc,
                            doc_ref.id,
                            crate::checks::at_rest::Pinned {
                                pin: doc_ref.pin,
                                finding,
                            },
                        )
                    })
                    .chain(carried_at_rest.into_iter().map(|row| {
                        PartRow::below(
                            doc,
                            row.route,
                            crate::checks::at_rest::Pinned {
                                pin: row.pin,
                                finding: row.finding,
                            },
                            row.held,
                        )
                    }))
                    .collect(),
            ),
            door_rows: Arc::new(door_rows),
            body: super::Framed::in_frame(Arc::new(product.body.into_body()), product.frame),
            names: Arc::new(product.names),
            carry: super::Carry {
                contacts: Arc::new(product.contacts),
                parts,
                pieces,
                censused: true,
                ..super::Carry::default()
            },
        }))
    }
}

/// **The parts an evaluation instantiates, already evaluated**: what
/// its mate solve and node map read, by `&`, from any thread.
pub(crate) struct Reached<T: Decide> {
    rows: Rows<T>,
    exhausted: Exhausted,
}

impl<T: Decide> Reached<T> {
    /// Whether `doc_ref`'s evaluation at `tol`'s ε ran out of member
    /// budget: an instance of it is where the evaluation reading it
    /// runs out too.
    pub(crate) fn exhausted(&self, doc_ref: &DocRef, tol: Tol) -> bool {
        self.exhausted.contains(&(*doc_ref, tol.eps().to_bits()))
    }

    /// The row `doc_ref` has at `tol`'s ε. A reference with none was
    /// not entered: [`PartFault::NotEntered`].
    pub(crate) fn get(&self, doc_ref: &DocRef, tol: Tol) -> Result<PartValue<T>, PartFault> {
        self.rows
            .get(&(*doc_ref, tol.eps().to_bits()))
            .cloned()
            .unwrap_or(Err(PartFault::NotEntered))
    }
}

/// **The successful rows an evaluation carries for the next**
/// ([`PartStore::enter_census`]): the next run that names it as its
/// prior takes each row its own census names from here instead of
/// evaluating the part again.
pub(crate) struct Carried<T: Decide>(BTreeMap<(DocRef, u64), PartValue<T>>);

impl<T: Decide> Carried<T> {
    /// No row carried: a nested evaluation, or one that entered none.
    pub(crate) fn none() -> Self {
        Self(BTreeMap::new())
    }

    /// The carried rows' keys, in order.
    pub(crate) fn keys(&self) -> impl Iterator<Item = (DocRef, u64)> + '_ {
        self.0.keys().copied()
    }
}

impl<T: Decide> core::fmt::Debug for Carried<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set()
            .entries(self.0.keys().map(|(r, _)| r))
            .finish()
    }
}

/// **The one census of what a document's evaluation can ask its part
/// table for**: the reference `id` instantiates, when `id` is an
/// instantiate node. Both askers read it: the instantiate node's own op
/// (`wire::wire_instantiate_part`) and the mate solve's reach over a
/// member's instance (`mate::solve`'s `part_of`, which the lever's
/// `pair_reach` and a face-based side's face pose both ask through).
/// Every reference it names is entered before the document evaluates,
/// and the table refuses any other ask ([`PartFault::NotEntered`]).
pub(crate) fn instantiated<P>(doc: &crate::Doc<P>, id: RecipeNodeId) -> Option<DocRef> {
    match doc.node(id) {
        Some(crate::node::Node::InstantiatePart { doc_ref, .. }) => Some(*doc_ref),
        _ => None,
    }
}

/// A document on the descent stack: resolved, and waiting for the
/// parts it instantiates.
struct Entered<T: Decide> {
    doc_ref: DocRef,
    doc: Arc<ProfileDoc>,
    /// Its `InstantiatePart` references in document order, repeats
    /// included; `next` is how far the descent has read them.
    refs: Vec<DocRef>,
    next: usize,
    reached: Rows<T>,
    exhausted: Exhausted,
}

impl<T: Decide> Entered<T> {
    /// Enters `doc_ref` from the document at the end of `path`, or says
    /// why it cannot be entered.
    fn enter(
        resolver: &Arc<dyn PartResolver>,
        path: &[DocRef],
        doc_ref: &DocRef,
        tol: Tol,
    ) -> Result<Self, PartFault> {
        // The cycle, decided structurally and named: the loop runs from
        // the reference's earlier appearance to this repeat of it.
        if let Some(at) = path.iter().position(|r| r == doc_ref) {
            let mut cycle = path[at..].to_vec();
            cycle.push(*doc_ref);
            return Err(PartFault::ReferenceCycle { cycle });
        }
        if path.len() >= MAX_DEPTH {
            return Err(PartFault::DepthExceeded);
        }
        let doc = resolver
            .resolve(doc_ref, tol)
            .map_err(|e| PartFault::Unresolved {
                fault: e.fault,
                message: e.message,
            })?;
        let refs = if super::recorded_at_process_eps(&doc, tol) {
            doc.ids()
                .iter()
                .filter_map(|&id| instantiated(&doc, id))
                .collect()
        } else {
            Vec::new()
        };
        Ok(Self {
            doc_ref: *doc_ref,
            doc: Arc::new(doc),
            refs,
            next: 0,
            reached: BTreeMap::new(),
            exhausted: Exhausted::new(),
        })
    }

    /// The next reference this document instantiates that has no row
    /// yet.
    fn next_unreached(&mut self, eps_bits: u64) -> Option<DocRef> {
        while let Some(r) = self.refs.get(self.next).copied() {
            self.next += 1;
            if !self.reached.contains_key(&(r, eps_bits))
                && !self.exhausted.contains(&(r, eps_bits))
            {
                return Some(r);
            }
        }
        None
    }
}

/// Turns the referenced document's product refusal into a fault that
/// still NAMES its cause.
///
/// A failed node, and a poisoned node's failed ancestor, hold their
/// refusal in the nested evaluation, which is local to the resolution
/// and cannot be reached from the caller. So the evaluation is taken
/// here by value and the failure is moved out of it: the fault carries
/// the very refusal the part's evaluation raised, a seam fault any
/// number of documents down included.
///
/// Every OTHER refusal crosses whole ([`PartFault::PartProduct`]).
///
/// `part` is the document evaluated, at the version its reference pins:
/// the fault keeps the nodes it names as `part` holds them
/// ([`PartFault::held`]), the one place the pinned part is in hand.
fn product_fault<T: Decide>(
    error: crate::product::ProductError,
    mut evaluation: super::Evaluation<T>,
    part: &ProfileDoc,
) -> PartFault {
    use super::NodeStanding;
    // Each arm is built from its held nodes: once to say its sentence
    // over `part`, recording what it names, and once holding them.
    type Arm = Box<dyn Fn(Arc<HeldNodes>) -> PartFault>;
    let unrecorded = |node: RecipeNodeId| -> Arm {
        Box::new(move |held| PartFault::PartProductFailureUnrecorded { node, held })
    };
    let mut refusal_at = |failed: RecipeNodeId| match evaluation.nodes.remove(&failed) {
        Some(super::NodeResult::Failed(failure)) => Ok(super::NodeRefusal::from(failure.kind)),
        _ => Err(failed),
    };
    let arm: Arm = match error {
        crate::product::ProductError::Read(NodeStanding::Failed { node }) => {
            match refusal_at(node) {
                Ok(refusal) => Box::new(move |held| PartFault::PartProductFailed {
                    node,
                    refusal: refusal.clone(),
                    held,
                }),
                Err(failed) => unrecorded(failed),
            }
        }
        crate::product::ProductError::Read(NodeStanding::Poisoned { node, through }) => {
            match refusal_at(through) {
                Ok(refusal) => Box::new(move |held| PartFault::PartProductPoisoned {
                    node,
                    through,
                    refusal: refusal.clone(),
                    held,
                }),
                Err(failed) => unrecorded(failed),
            }
        }
        _ => {
            let refusal: crate::product::ProductRefusal = error.into();
            Box::new(move |held| PartFault::PartProduct {
                refusal: refusal.clone(),
                held,
            })
        }
    };
    let held = crate::spoken::held_by(&InPart(&arm(Arc::default())), part);
    arm(Arc::new(held))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{PartFault, PartStore, Reached};
    use crate::ProfileDoc;
    use crate::ident::{ContentPin, DocRef, DocumentId};
    use crate::part::{PartResolver, ResolveFailure, ResolveFault};
    use geom_core::Tol;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A resolver that counts its calls and resolves nothing.
    #[derive(Debug, Default)]
    struct Counting(AtomicUsize);

    impl PartResolver for Counting {
        fn resolve(&self, _: &DocRef, _: Tol) -> Result<ProfileDoc, ResolveFailure> {
            self.0.fetch_add(1, Ordering::Relaxed);
            Err(ResolveFailure {
                fault: ResolveFault::Unresolved,
                message: "the counting resolver holds no document".into(),
            })
        }
    }

    fn doc_ref(seed: &str) -> DocRef {
        DocRef {
            id: DocumentId::derive(seed),
            pin: ContentPin([1; 32]),
        }
    }

    /// **A table never descends, and the store asks once per key.** An
    /// ask the census did not enter refuses `NotEntered`; the store's
    /// row resolves on its first ask and keeps even a faulted answer,
    /// so a second ask in one run reaches no resolver. Goes red if a
    /// table miss evaluates, or if the store re-resolves within a run.
    #[test]
    fn a_table_miss_refuses_not_entered_and_a_store_asks_once() {
        let counting = Arc::new(Counting::default());
        let resolver: Arc<dyn PartResolver> = counting.clone();
        let asked = doc_ref("parts-unit-asked");

        let table = Reached::<f64> {
            rows: BTreeMap::new(),
            exhausted: super::Exhausted::new(),
        };
        assert!(
            matches!(
                table.get(&asked, Tol::witness()),
                Err(PartFault::NotEntered)
            ),
            "a table's miss is the kernel defect"
        );

        let defaults = super::super::EvalOptions::default();
        let mut store =
            PartStore::<f64>::new(Some(&resolver), defaults.boolean_sweep, Tol::witness());
        let cancel = super::super::CancelToken::new();
        for ask in 1..=2 {
            assert!(
                matches!(
                    store.row(&asked, Tol::witness(), &cancel),
                    super::Evaluated::Row(Err(PartFault::Unresolved { .. }))
                ),
                "ask {ask}: this resolver refuses the reference"
            );
        }
        assert_eq!(
            counting.0.load(Ordering::Relaxed),
            1,
            "two asks in one store resolve once"
        );
        assert_eq!(
            store.evaluations, 0,
            "a refused resolution evaluates nothing"
        );
    }
}
