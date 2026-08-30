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
/// entities' part-local stable names, and the product's DECLARED
/// CONTACT RECORDS (ASM-R2b D-1 — a part's own declarations cross the
/// document seam with its geometry, in the same keys).
pub(crate) struct PartValue<T: Decide> {
    pub body: Arc<Body<T>>,
    pub names: Arc<NameTable>,
    pub contacts: Arc<topo::ContactRecords>,
}

impl<T: Decide> Clone for PartValue<T> {
    fn clone(&self) -> Self {
        Self {
            body: Arc::clone(&self.body),
            names: Arc::clone(&self.names),
            contacts: Arc::clone(&self.contacts),
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
    /// produced it, so a caller can never be sent to look at it. The
    /// cause therefore travels: `cause` when the failing root was
    /// itself an instantiate node (the fault CHAINS, typed, however
    /// many documents deep the real reason lies — a nesting-depth
    /// refusal at the bottom of a cycle arrives here intact), and
    /// `message` otherwise, carrying that node error's own rendering.
    PartRootFailed {
        /// The failing root, in the REFERENCED document's id space.
        node: RecipeNodeId,
        /// The seam fault that root reported, when it had one.
        cause: Option<Box<PartFault>>,
        /// Otherwise the failing node error's rendering, kind included.
        message: String,
    },
    /// The referenced document has no product for a reason that is not
    /// a failing root (no body-denoting root, an invalid gather, a
    /// name collision).
    PartProduct {
        /// The product door's diagnosis.
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

impl core::fmt::Display for PartFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoResolver => write!(
                f,
                "this evaluation carries no part resolver, so a referenced document cannot be \
                 reached"
            ),
            Self::Unresolved { fault, message } => match fault {
                ResolveFault::PinMismatch => {
                    write!(f, "the reference's pin does not hold: {message}")
                }
                ResolveFault::EpsilonSeam => write!(
                    f,
                    "the referenced document's recorded tolerance disagrees with this process's: \
                     {message}"
                ),
                ResolveFault::Unresolved => write!(f, "the reference did not resolve: {message}"),
            },
            Self::PartRootFailed {
                node,
                cause,
                message,
            } => {
                write!(
                    f,
                    "the referenced document's product root {} failed: ",
                    node.0
                )?;
                match cause {
                    Some(cause) => write!(f, "{cause}"),
                    None => write!(f, "{message}"),
                }
            }
            Self::PartProduct { message } => {
                write!(f, "the referenced document has no product: {message}")
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
                Ok(())
            }
            Self::DepthExceeded => write!(
                f,
                "instantiation nested deeper than {MAX_DEPTH} documents without repeating a \
                 reference"
            ),
        }
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
        // through the product door's own typed refusal — and its cause
        // travels with it, because the evaluation holding that cause
        // does not outlive this call.
        // A part is its PRODUCT, however many solids that product
        // holds. The count is not consulted here at all — the
        // placing path maps all N as one body and the gather grafts
        // them as N, so a narrower rule at this door would be a second
        // truth about what instantiating a document means.
        let product = crate::product::product_recorded(&doc, &evaluation, tol)
            .map_err(|e| product_fault(&e, &evaluation))?;
        Ok(PartValue {
            body: Arc::new(product.body),
            names: Arc::new(product.names),
            contacts: Arc::new(product.contacts),
        })
    }
}

/// Turns the referenced document's product refusal into a fault that
/// still NAMES its cause (review MAJOR-1).
///
/// A `RootFailed` refusal's own text points at
/// `Evaluation::node_error` — an object the caller cannot reach, since
/// the nested evaluation is local to the resolution. So the cause is
/// read here, while it still exists: typed and chained when the
/// failing root was itself an instantiate node, rendered otherwise.
fn product_fault<T: Decide>(
    error: &crate::product::ProductError,
    evaluation: &super::Evaluation<T>,
) -> PartFault {
    let crate::product::ProductError::RootFailed { node } = error else {
        return PartFault::PartProduct {
            message: error.to_string(),
        };
    };
    let Some(failure) = evaluation.node_error(*node) else {
        return PartFault::PartRootFailed {
            node: *node,
            cause: None,
            message: "the evaluation records no cause for it".to_string(),
        };
    };
    // A nested seam fault chains VERBATIM: a depth refusal, a pin
    // mismatch or an ε disagreement any number of documents down
    // arrives at the top typed, not as prose about prose.
    let cause = match &failure.kind {
        super::NodeErrorKind::Part { fault, .. } => Some(Box::new(fault.clone())),
        _ => None,
    };
    PartFault::PartRootFailed {
        node: *node,
        cause,
        message: failure.kind.to_string(),
    }
}
