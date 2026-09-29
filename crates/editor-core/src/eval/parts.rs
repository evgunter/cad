//! The per-evaluation instantiated-part memo (ASM-2A D-3).
//!
//! A2 says evaluation MATERIALIZES: an instantiate node evaluates the
//! document it pins and takes that document's A10 product. N instances
//! of one part must not evaluate it N times, so one cache per
//! evaluation holds each resolved part's product.
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
//! The cache is LAZY: a memo-hit instantiate node never asks, so a
//! re-evaluation that changed nothing across the seam does no
//! cross-document work at all.
//!
//! # Cycles are decided, not waited out
//!
//! The cache also carries the DESCENT CHAIN — the references this
//! evaluation was reached through. A reference already in the chain is
//! a cycle by A4's own rule (same id, same pin ⇒ same content), so it
//! refuses immediately, NAMING the loop. `MAX_DEPTH` is what is left
//! over once that is handled: runaway insurance for acyclic descent,
//! diagnosing nothing.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use geom_core::Decide;
use topo::Body;

use crate::ident::DocRef;
use crate::names::NameTable;
use crate::node::RecipeNodeId;
use crate::part::{PartResolver, ResolveFault};
use geom_core::Tol;

/// Pure runaway insurance: the depth at which instantiation gives up,
/// having ruled out the reason it would ordinarily run away.
///
/// CYCLES are caught structurally, one level up, by the descent chain
/// (see [`PartCache`]) — a revisited reference refuses NAMING the
/// cycle, which is the diagnosis an author can act on. This constant
/// is what remains after that: a bound on genuinely deep, genuinely
/// acyclic nesting, high enough that no real assembly meets it and
/// finite so that nothing recurses without end. It diagnoses nothing;
/// reaching it means the descent chain was long, not that it looped.
pub(crate) const MAX_DEPTH: usize = 1024;

/// A resolved part: the referenced document's product, the product
/// entities' part-local stable names, the product's DECLARED CONTACT
/// RECORDS (ASM-R2b D-1 — a part's own declarations cross the document
/// seam with its geometry, in the same keys), and the MATE BOOKKEEPING
/// that says whose declaration each record is and which of the
/// document's mates it could not mint at all.
pub(crate) struct PartValue<T: Decide> {
    pub body: Arc<Body<T>>,
    pub names: Arc<NameTable>,
    pub contacts: Arc<topo::ContactRecords>,
    /// The referenced document's OWN minted declarations — which of
    /// its mates authored which of those records, keyed in the same
    /// arena the records are. The records already crossed the seam;
    /// without these rows a finding against one names nobody.
    pub minted: Arc<Vec<crate::assembly::MintedDeclaration>>,
    /// The referenced document's own MINT REFUSALS: mates it could not
    /// mint at all. Carried because inner mint health is the outermost
    /// gate's business — a part with an unverifiable contact is a
    /// broken part, and the document that instantiates it is not at
    /// rest over it.
    pub unminted: Arc<Vec<crate::assembly::MintRefusal>>,
    /// What the referenced document itself carried up from ITS parts,
    /// route and all. Instantiation extends the route; nothing below
    /// is re-read.
    pub carried: Arc<Vec<crate::assembly::CarriedDeclaration>>,
    /// The same for the refusals it carried up.
    pub carried_unminted: Arc<Vec<crate::assembly::CarriedRefusal>>,
}

impl<T: Decide> Clone for PartValue<T> {
    fn clone(&self) -> Self {
        Self {
            body: Arc::clone(&self.body),
            names: Arc::clone(&self.names),
            contacts: Arc::clone(&self.contacts),
            minted: Arc::clone(&self.minted),
            unminted: Arc::clone(&self.unminted),
            carried: Arc::clone(&self.carried),
            carried_unminted: Arc::clone(&self.carried_unminted),
        }
    }
}

/// Why an instantiation could not produce a part body. Cloneable and
/// self-contained: the cache stores one of these per reference, so
/// every instance of a broken part reports the same typed cause.
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
    /// The referenced document evaluated, but one of its PRODUCT ROOTS
    /// failed — the ordinary "my part is broken, why?" path.
    ///
    /// The nested [`super::Evaluation`] dies with the resolution that
    /// produced it, so a caller can never be sent to look at it: the
    /// root's own refusal travels here instead, typed and unaltered. A
    /// root that is itself an instantiate node carries
    /// [`super::NodeErrorKind::Part`] in turn, so a part inside a part
    /// is a chain of these however many documents deep, and every
    /// level keeps the reference it crossed.
    ///
    /// The sentence is the instance's own and never renders `refusal`:
    /// that is another node's refusal, in another document, with its
    /// own recourse, and it is drawn as its own line (the exception
    /// written over [`super::NodeErrorKind`]'s `Display`).
    PartRootFailed {
        /// The failing root, in the REFERENCED document's id space.
        node: RecipeNodeId,
        /// That root's own refusal.
        refusal: super::NodeRefusal,
    },
    /// The referenced document evaluated, but one of its PRODUCT ROOTS
    /// never ran: a node upstream of it inside the part failed. The
    /// part's typical broken shape — a transform or a boolean over the
    /// node that failed.
    ///
    /// `through` is the node the author repairs, so its refusal is the
    /// one carried, as [`Self::PartRootFailed`] carries a failed
    /// root's; `root` says which of the part's roots it cost.
    PartRootPoisoned {
        /// The poisoned root, in the REFERENCED document's id space.
        root: RecipeNodeId,
        /// Its nearest failed ancestor, in the same id space.
        through: RecipeNodeId,
        /// That ancestor's own refusal.
        refusal: super::NodeRefusal,
    },
    /// The product door named a node as failed — a failed root, or a
    /// poisoned root's failed ancestor — and the evaluation it read
    /// holds no failure there. The two disagree about one node, so this
    /// is a kernel bug, reported typed rather than as a failure with no
    /// cause.
    RootFailureUnrecorded {
        /// The node the product door named as failed, in the
        /// REFERENCED document's id space.
        node: RecipeNodeId,
    },
    /// The referenced document has no product for a reason that is not
    /// a failed or poisoned root (no body-denoting root, an invalid
    /// gather, a name collision).
    ///
    /// The refusal crosses in both halves, the
    /// [`crate::checks::ChecksError::Product`] shape: `kind` is the
    /// class a consumer branches on, `message` the gather's own
    /// sentence a reader reads — it carries the node ids and finding
    /// lists the class drops. Neither half is a substring hunt through
    /// the other, and both come off ONE [`crate::product::ProductError`].
    PartProduct {
        /// Which arm of the product door refused.
        kind: crate::product::ProductErrorKind,
        /// The product door's diagnosis without the gather's labels
        /// (its stage word, and the `root N output M` subject of a
        /// listed finding): this sentence names the stage itself.
        message: String,
    },
    /// The reference CHAIN returned to a document it had already
    /// entered — the same (id, pin), so the same content: descending
    /// further would repeat forever.
    ///
    /// A4 makes this unconstructible through an honest store (a
    /// document would have to contain its own hash), so the fault names
    /// a broken RESOLVER or a hand-built cycle. It carries the loop
    /// itself, first repeated reference through last, because the loop
    /// is the diagnosis.
    ReferenceCycle {
        /// The cycle, starting and ending at the repeated reference.
        cycle: Vec<DocRef>,
    },
    /// Instantiation nested past [`MAX_DEPTH`] without repeating a
    /// reference — runaway insurance, not a cycle diagnosis.
    DepthExceeded,
}

impl PartFault {
    /// **The refusal this fault carries, when it carries one**: a
    /// failed root, or a poisoned root's failed ancestor, and that
    /// node's own refusal, in the referenced document's id space ([`super::NodeErrorKind::carried`]). Exhaustive, for the
    /// reason [`crate::MateFault::carried`] states.
    #[must_use]
    pub fn carried(&self) -> Option<(RecipeNodeId, &super::NodeRefusal)> {
        match self {
            Self::PartRootFailed { node, refusal } => Some((*node, refusal)),
            Self::PartRootPoisoned {
                through, refusal, ..
            } => Some((*through, refusal)),
            Self::NoResolver
            | Self::Unresolved { .. }
            | Self::RootFailureUnrecorded { .. }
            | Self::PartProduct { .. }
            | Self::ReferenceCycle { .. }
            | Self::DepthExceeded => None,
        }
    }
}

impl core::fmt::Display for PartFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // Every door that evaluates, solves or edits over parts
            // takes one.
            Self::NoResolver => write!(
                f,
                "this evaluation carries no part resolver, so a referenced document cannot be \
                 reached. Recourse: give it a resolver over the store that holds the part"
            ),
            // The resolver knows what went wrong in its store, so its
            // message states the recourse of a pin or a lookup; the ε
            // seam's is the same whatever the store, since a document
            // keeps the ε it was written at and a process holds one.
            Self::Unresolved { fault, message } => match fault {
                ResolveFault::PinMismatch => {
                    write!(f, "the reference's pin does not hold: {message}")
                }
                ResolveFault::EpsilonSeam => write!(
                    f,
                    "the referenced document's recorded tolerance disagrees with this process's: \
                     {message}. Recourse: author the part again in a process at this tolerance"
                ),
                ResolveFault::Unresolved => write!(f, "the reference did not resolve: {message}"),
            },
            Self::PartRootFailed { node, .. } => write!(
                f,
                "the part's node {} failed, so the part has no body. {}",
                node.0,
                InThePart(format_args!("repair node {}", node.0)),
            ),
            Self::PartRootPoisoned { root, through, .. } => write!(
                f,
                "the part's node {} failed and poisoned its root, node {}, so the part has \
                 no body. {}",
                through.0,
                root.0,
                InThePart(format_args!("repair node {}", through.0)),
            ),
            Self::RootFailureUnrecorded { node } => write!(
                f,
                "the part's product names its node {} as failed and the part's evaluation holds \
                 no failure there; the two disagree, so this is a kernel bug. {}",
                node.0,
                InThePart(format_args!(
                    "see node {} as it evaluates, then report it with the part's file",
                    node.0
                )),
            ),
            Self::PartProduct { kind, message } => {
                write!(f, "the part has no product: {message}")?;
                match product_recourse(*kind) {
                    ProductRecourse::InThePart(action) => write!(f, ". {}", InThePart(action)),
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
                 reference. Recourse: flatten the assembly so its parts nest fewer documents \
                 deep"
            ),
        }
    }
}

/// The recourse of a repair the author makes inside the part: open it,
/// and act there.
struct InThePart<A>(A);

impl<A: core::fmt::Display> core::fmt::Display for InThePart<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Recourse: open the part and {}", self.0)
    }
}

/// What a part with no product states after the gather's own sentence.
enum ProductRecourse {
    /// The repair is in the part, and the gather's sentence does not
    /// say what it is.
    InThePart(&'static str),
    /// The gather's sentence already states the one recourse: its own,
    /// or the kernel refusal it forwards.
    Carried,
    /// A class a nested evaluation cannot reach.
    KernelDefect,
}

/// [`ProductRecourse`] by the gather's class. A nested evaluation runs
/// its own document to completion, and the failed and poisoned roots
/// cross as their own arms, so the classes it cannot reach end in the
/// kernel-defect recourse.
fn product_recourse(kind: crate::product::ProductErrorKind) -> ProductRecourse {
    use crate::product::ProductErrorKind as K;
    match kind {
        K::NoBodyRoots => ProductRecourse::InThePart("give it a root that denotes a body"),
        K::Naming => ProductRecourse::InThePart("repair it there"),
        K::PlacedUnderTwoRoots | K::Graft | K::RootInvalid | K::ProductInvalid => {
            ProductRecourse::Carried
        }
        K::ContactLineage
        | K::EvaluationOfAnotherDocument
        | K::UnknownNode
        | K::RootFailed
        | K::RootPoisoned => ProductRecourse::KernelDefect,
    }
}

/// One cache row: the resolved part, or the typed reason there is
/// none. Keyed by `(DocRef, ambient ε bits)` — the module docs' key.
type Rows<T> = BTreeMap<(DocRef, u64), Result<PartValue<T>, PartFault>>;

/// The cache. One per `evaluate` call; shared across its nodes.
pub(crate) struct PartCache<'a, T: Decide> {
    resolver: Option<&'a Arc<dyn PartResolver>>,
    /// The DESCENT CHAIN: every reference this evaluation was reached
    /// through, outermost first — empty at the top-level call.
    ///
    /// A4 makes `(id, pin)` say WHICH CONTENT, so a reference already
    /// in the chain would re-enter a document whose evaluation is the
    /// one currently in progress: the loop is structural, and it is
    /// decided here rather than waited out by a depth counter. The
    /// chain doubles as the nesting depth (its length).
    chain: &'a [DocRef],
    /// The ambient ε these entries were produced at, by bits — half
    /// the key, hoisted because it is constant within one evaluation.
    eps_bits: u64,
    /// The candidate-generation strategy the CALLER chose. Inherited
    /// across the seam so a differential run exercises the parts'
    /// booleans too — the strategy is a property of the run, not of
    /// the document, and results are bit-identical either way.
    boolean_sweep: topo::SweepStrategy,
    /// Where profile geometry comes from, inherited across the seam
    /// for the same reason `boolean_sweep` is: it is a property of the
    /// RUN, so a referenced document must be elaborated the way its
    /// instantiator is being elaborated.
    profile_lift: super::ProfileLift,
    entries: Mutex<Rows<T>>,
    /// How many referenced-document evaluations happened at or BELOW
    /// this level — the D-3 sharing evidence. A counter, not a timing
    /// claim. Nested crossings fold in (see `resolve_and_evaluate`), so
    /// the outermost evaluation reports every crossing the run made.
    evaluations: AtomicUsize,
}

impl<'a, T: Decide> PartCache<'a, T> {
    pub(crate) fn new(
        resolver: Option<&'a Arc<dyn PartResolver>>,
        chain: &'a [DocRef],
        boolean_sweep: topo::SweepStrategy,
        profile_lift: super::ProfileLift,
        tol: Tol,
    ) -> Self {
        Self {
            resolver,
            chain,
            eps_bits: tol.eps().to_bits(),
            boolean_sweep,
            profile_lift,
            entries: Mutex::new(BTreeMap::new()),
            evaluations: AtomicUsize::new(0),
        }
    }

    /// The descent chain this evaluation was reached through — empty
    /// at the top level, ending in this document's own reference
    /// below it. What `param_source::ParamScope::of` reads.
    pub(crate) fn chain(&self) -> &'a [DocRef] {
        self.chain
    }

    /// How many referenced-document evaluations ran at or below this
    /// level.
    pub(crate) fn evaluations(&self) -> usize {
        self.evaluations.load(Ordering::Relaxed)
    }
}

impl<T: super::EvalScalar> PartCache<'_, T> {
    /// The part `doc_ref` denotes, evaluated at most once per key.
    ///
    /// The lock is held across the miss path on purpose: it is what
    /// makes "evaluated ONCE" true when two instances of one part race,
    /// and a nested evaluation builds its own cache, so the lock is
    /// never re-entered.
    ///
    /// **The miss path runs inside a shielding verdict bracket.** The
    /// instantiate node's log is the decisions its op made about its
    /// own content, the same on a hit and on a miss (same content key
    /// ⇒ same decisions, D9), so the part's unbracketed decisions —
    /// its mate solve, its profile pre-passes, the gather of its
    /// product — must land on neither instance. The shield sits HERE
    /// rather than at the op door because the miss path is the one
    /// thing a hit does not run: shielding the whole op would also
    /// hide the placement and validation the op does on every path,
    /// which ARE the node's decisions.
    pub(crate) fn get(&self, doc_ref: &DocRef, tol: Tol) -> Result<PartValue<T>, PartFault> {
        let key = (*doc_ref, self.eps_bits);
        let mut entries = match self.entries.lock() {
            Ok(g) => g,
            // A poisoned lock means another instance's resolution
            // panicked. This crate has no panic paths, so treat it as
            // the same refusal rather than propagating a panic.
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(hit) = entries.get(&key) {
            return hit.clone();
        }
        let _shield = geom_core::k_stats::Bracket::open();
        let value = self.resolve_and_evaluate(doc_ref, tol);
        entries.insert(key, value.clone());
        value
    }

    fn resolve_and_evaluate(&self, doc_ref: &DocRef, tol: Tol) -> Result<PartValue<T>, PartFault> {
        let resolver = self.resolver.ok_or(PartFault::NoResolver)?;
        // The cycle, decided structurally and named: the loop runs from
        // the reference's earlier appearance to this repeat of it.
        if let Some(at) = self.chain.iter().position(|r| r == doc_ref) {
            let mut cycle = self.chain[at..].to_vec();
            cycle.push(*doc_ref);
            return Err(PartFault::ReferenceCycle { cycle });
        }
        if self.chain.len() >= MAX_DEPTH {
            return Err(PartFault::DepthExceeded);
        }
        let doc = resolver
            .resolve(doc_ref, tol)
            .map_err(|e| PartFault::Unresolved {
                fault: e.fault,
                message: e.message,
            })?;
        self.evaluations.fetch_add(1, Ordering::Relaxed);
        // AQ4: the referenced document evaluates at its OWN parameters
        // — v1 instantiation takes no arguments. Sequentially, with a
        // fresh epoch: this run's identity is its own, and nesting a
        // rayon scope under a held lock buys nothing.
        let opts = super::EvalOptions {
            epoch: super::Epoch::mint(),
            parallel: false,
            boolean_sweep: self.boolean_sweep,
            resolver: self.resolver.map(Arc::clone),
            profile_lift: self.profile_lift,
            // NOT inherited, unlike the two rows above: a parameter box
            // is a set of THIS document's parameter names, and a
            // referenced document is a different document with its own
            // names (AQ4 — v1 instantiation takes no arguments). A box
            // that crossed the seam would either name nothing there or,
            // worse, collide by name with an unrelated parameter.
            param_box: None,
            // NOT inherited, by the same argument: a seed is a name of
            // THIS document's parameters. A part's geometry is constant
            // with respect to them (AQ4 — v1 instantiation takes no
            // arguments), which the unseeded nested run states exactly:
            // every tangent it carries is zero.
            seed: None,
        };
        let mut chain = self.chain.to_vec();
        chain.push(*doc_ref);
        let evaluation =
            super::evaluate_nested::<T>(&doc, &super::CancelToken::new(), &opts, &chain, tol);
        // The nested run's own crossings are crossings of THIS run:
        // fold them in, so the outermost counter is the whole run's
        // evidence rather than one level's.
        self.evaluations
            .fetch_add(evaluation.part_evaluations, Ordering::Relaxed);
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
        let product = match crate::product::product_recorded(&doc, &evaluation, tol) {
            Ok(product) => product,
            Err(e) => return Err(product_fault(&e, evaluation)),
        };
        // The whole product crosses the seam, not a slice of it: what
        // a document MEANS is its product, and its mates' identity and
        // mint health are as much part of that as its records are. The
        // `Arc`s are the cache's, so every instance of one part shares
        // one row set.
        Ok(PartValue {
            body: Arc::new(product.body.into_body()),
            names: Arc::new(product.names),
            contacts: Arc::new(product.contacts),
            minted: Arc::new(product.minted),
            unminted: Arc::new(product.unminted),
            carried: Arc::new(product.carried),
            carried_unminted: Arc::new(product.carried_unminted),
        })
    }
}

/// Turns the referenced document's product refusal into a fault that
/// still NAMES its cause.
///
/// A failed root, and a poisoned root's failed ancestor, hold their
/// refusal in the nested evaluation, which is local to the resolution
/// and cannot be reached from the caller. So the evaluation is taken
/// here by value and the failure is moved out of it: the fault carries
/// the very refusal the part's evaluation raised, a seam fault any
/// number of documents down included.
///
/// Every OTHER refusal crosses as its class beside its sentence, both
/// read off the one error — the pairing
/// [`PartFault::PartProduct`] states.
fn product_fault<T: Decide>(
    error: &crate::product::ProductError,
    mut evaluation: super::Evaluation<T>,
) -> PartFault {
    use super::NodeStanding;
    let mut refusal_at = |failed: RecipeNodeId| match evaluation.nodes.remove(&failed) {
        Some(super::NodeResult::Failed(failure)) => Ok(super::NodeRefusal::from(failure.kind)),
        _ => Err(PartFault::RootFailureUnrecorded { node: failed }),
    };
    let carried = match *error {
        crate::product::ProductError::Root(NodeStanding::Failed { node }) => {
            refusal_at(node).map(|refusal| PartFault::PartRootFailed { node, refusal })
        }
        crate::product::ProductError::Root(NodeStanding::Poisoned { node, through }) => {
            refusal_at(through).map(|refusal| PartFault::PartRootPoisoned {
                root: node,
                through,
                refusal,
            })
        }
        _ => Ok(PartFault::PartProduct {
            kind: error.kind(),
            message: error.sentence().to_string(),
        }),
    };
    carried.unwrap_or_else(|unrecorded| unrecorded)
}
