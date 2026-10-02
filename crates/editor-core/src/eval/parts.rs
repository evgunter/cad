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
//! The top-level cache is LAZY: a memo-hit instantiate node never
//! asks, so a re-evaluation that changed nothing across the seam does
//! no cross-document work at all.
//!
//! # The descent runs on the heap
//!
//! Below the top, the descent is bottom-up on an explicit stack
//! (`PartCache::resolve_and_evaluate`): a referenced document is
//! evaluated once every part it instantiates has its row, and its cache
//! starts with those rows. So one nested evaluation is on the thread's
//! stack at a time, how deep an assembly nests costs heap and never
//! stack, and every depth either evaluates or refuses typed on whatever
//! thread the evaluation runs on.
//!
//! A nested cache never descends: every reference a nested evaluation
//! can ask for is entered first ([`instantiated`], the one census of
//! the asks), and a miss below the top is the typed kernel defect
//! [`PartFault::NotEntered`], never a recursion.
//!
//! **Below the top, every part a document instantiates is evaluated,
//! whether or not its instance asks.** The instance that declines to
//! ask is one whose placement refused (a mate fault); its part's row,
//! failed or not, sits in a cache that dies with the nested evaluation
//! and reaches no node, product or refusal. What it does reach is
//! what counts or records work as it happens: `part_evaluations`
//! counts it, and a shape report, a symbolic session's counts or a
//! sample sink installed around the evaluation sees its decisions —
//! and sees every nested document's decisions bottom-up, a part's
//! before its instantiator's.
//!
//! # Cycles are decided, not waited out
//!
//! The cache also carries the DESCENT CHAIN — the references this
//! evaluation was reached through. A reference already in the chain is
//! a cycle by A4's own rule (same id, same pin ⇒ same content), so it
//! refuses immediately, NAMING the loop, and that check runs before
//! the depth check, so a loop that closes at [`MAX_DEPTH`] is still
//! named as a loop.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use geom_core::Decide;
use topo::Body;

use crate::ProfileDoc;
use crate::ident::DocRef;
use crate::names::NameTable;
use crate::node::RecipeNodeId;
use crate::part::{PartResolver, ResolveFault};
use crate::sentence::{PASS_A_RESOLVER, Recourse, Staged};
use geom_core::Tol;

/// **How deep an assembly may nest**: a document `MAX_DEPTH` documents
/// below the top evaluates, and one more refuses with
/// [`PartFault::DepthExceeded`], whose sentence states this number and
/// whose recourse is to flatten the assembly.
///
/// An author reaches it with a file: any acyclic chain of references
/// that long. CYCLES are decided before it, structurally, by the
/// descent chain (see [`PartCache`]) — a revisited reference refuses
/// NAMING the cycle — so reaching the bound means the chain was long,
/// not that it looped.
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
    /// The referenced document's own UNPLACED GROUPS, by root, with
    /// their causes: material its world product leaves out (A9), which
    /// the instantiating document must still be able to name.
    pub unplaced: Arc<Vec<(RecipeNodeId, crate::mate::Unplaced)>>,
    /// The same for the unplaced groups it carried up from its parts.
    pub carried_unplaced: Arc<Vec<crate::assembly::CarriedUnplaced>>,
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
            unplaced: Arc::clone(&self.unplaced),
            carried_unplaced: Arc::clone(&self.carried_unplaced),
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
    /// A document below the top of the descent asked for a part the
    /// descent had not entered before evaluating it. The descent enters
    /// every reference [`instantiated`] names, which is every reference
    /// a nested evaluation can ask for, so this is a kernel defect: an
    /// ask that escaped that census.
    NotEntered,
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
            | Self::DepthExceeded
            | Self::NotEntered => None,
        }
    }
}

// Every node the fault names is numbered in the PART: a frame that does
// not hold the part says them by tag (`Display`), and one that holds the
// resolved part says them from it ([`PartFault::spoken`]).
impl crate::spoken::Say for PartFault {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
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
            // seam's is the same whatever the store: a document keeps
            // the ε it was written at, a process holds one, and the
            // recorded-ε edit moves a part onto another while it keeps
            // its id, whether that was minted or derived.
            Self::Unresolved { fault, message } => match fault {
                ResolveFault::PinMismatch => {
                    write!(f, "the reference's pin does not hold: {message}")
                }
                ResolveFault::EpsilonSeam => write!(
                    f,
                    "the referenced document's recorded tolerance disagrees with this process's: \
                     {message}. {}",
                    Recourse(
                        "open the part in a process at its own tolerance, record the edit that \
                         sets this process's tolerance, save it over its file, then accept its \
                         updated version here"
                    )
                ),
                ResolveFault::Unresolved => write!(f, "the reference did not resolve: {message}"),
            },
            Self::PartRootFailed { node, .. } => {
                let node = by.node(*node);
                write!(
                    f,
                    "the part's {node} failed, so the part has no body. {}",
                    InThePart(format_args!("repair {node}")),
                )
            }
            Self::PartRootPoisoned { root, through, .. } => {
                let through = by.node(*through);
                write!(
                    f,
                    "the part's {through} failed and poisoned its root, {}, so the part has \
                     no body. {}",
                    by.node(*root),
                    InThePart(format_args!("repair {through}")),
                )
            }
            Self::RootFailureUnrecorded { node } => {
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
                 reference. {}",
                Recourse("flatten the assembly so its parts nest fewer documents deep")
            ),
            Self::NotEntered => write!(
                f,
                "the part was asked for by a nested document the descent evaluated before \
                 entering it, so the part has no body. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
        }
    }
}

/// The fault where the part is not in hand: each node by its tag.
impl core::fmt::Display for PartFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl PartFault {
    /// **The fault as a frame holding the resolved part says it**: each
    /// node as `part` holds it now. Its node ids are the part's, so
    /// `part` is the document the instance's reference names, at the
    /// version it pins; no other document can say them.
    ///
    /// # Panics
    ///
    /// When `part` is not the document `doc_ref` names.
    #[must_use]
    pub fn spoken<P>(&self, doc_ref: &DocRef, part: &crate::doc::Doc<P>) -> String {
        crate::spoken::assert_taken_of("the part fault", doc_ref.id, part);
        crate::spoken::spoken_by(self, part)
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
        K::Unplaced | K::PlacedUnderTwoRoots | K::Graft | K::RootInvalid | K::ProductInvalid => {
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
    /// How many referenced documents this cache's descents evaluated,
    /// at every depth — the D-3 sharing evidence. A counter, not a
    /// timing claim. Only a top-level cache descends
    /// (`resolve_and_evaluate` evaluates every document below it on
    /// this cache), so the top's count is every crossing the run made
    /// and a nested cache's is zero.
    evaluations: AtomicUsize,
}

impl<'a, T: Decide> PartCache<'a, T> {
    pub(crate) fn new(
        resolver: Option<&'a Arc<dyn PartResolver>>,
        chain: &'a [DocRef],
        reached: Reached<T>,
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
            entries: Mutex::new(reached.0),
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
    /// Only the top of a descent (an empty chain) evaluates on a miss.
    /// A nested cache starts with every row its document can ask for
    /// ([`instantiated`]), so a miss there is an ask that escaped that
    /// census: it refuses [`PartFault::NotEntered`] rather than
    /// descending from inside a nested evaluation, which would put a
    /// second evaluation on the thread's stack per level.
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
        if !self.chain.is_empty() {
            return Err(PartFault::NotEntered);
        }
        let _shield = geom_core::k_stats::Bracket::open();
        let value = self.resolve_and_evaluate(doc_ref, tol);
        entries.insert(key, value.clone());
        value
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
    fn resolve_and_evaluate(&self, doc_ref: &DocRef, tol: Tol) -> Result<PartValue<T>, PartFault> {
        let resolver = self.resolver.ok_or(PartFault::NoResolver)?;
        let mut path = Vec::new();
        let mut current = Entered::enter(resolver, &path, doc_ref, tol)?;
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
            let reached = core::mem::take(&mut current.reached);
            let value = self.evaluate_entered(&current.doc, &path, reached, tol);
            path.pop();
            let Some(parent) = waiting.pop() else {
                return value;
            };
            let done = core::mem::replace(&mut current, parent);
            current.reached.insert((done.doc_ref, self.eps_bits), value);
        }
    }

    /// One entered document's evaluation, at the end of `chain` (its
    /// own reference last), over the parts it instantiates, reached.
    fn evaluate_entered(
        &self,
        doc: &ProfileDoc,
        chain: &[DocRef],
        reached: Rows<T>,
        tol: Tol,
    ) -> Result<PartValue<T>, PartFault> {
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
        let evaluation = super::evaluate_nested::<T>(
            doc,
            &super::CancelToken::new(),
            &opts,
            chain,
            Reached(reached),
            tol,
        );
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
            Err(e) => return Err(product_fault(&e, evaluation)),
        };
        // The part's unplaced groups are not in its product (A9), so
        // they cross beside it: its own, and those its parts carried up
        // to it, read off the evaluation rather than the product so a
        // group below an instance no root gathers is named too.
        let unplaced = Arc::new(
            evaluation
                .unplaced
                .values()
                .copied()
                .collect::<BTreeMap<_, _>>()
                .into_iter()
                .collect(),
        );
        let carried_unplaced = Arc::new(evaluation.all_unplaced_below());
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
            unplaced,
            carried_unplaced,
        })
    }
}

/// **The parts a nested evaluation instantiates, already evaluated** —
/// the rows its cache starts from.
pub(crate) struct Reached<T: Decide>(Rows<T>);

impl<T: Decide> Reached<T> {
    /// No part reached: the top of a descent, whose cache asks lazily.
    pub(crate) fn none() -> Self {
        Self(BTreeMap::new())
    }
}

/// **The one census of what a document's evaluation can ask its part
/// cache for**: the reference `id` instantiates, when `id` is an
/// instantiate node. Both askers read it: the instantiate node's own op
/// (`wire::wire_instantiate_part`) and the mate solve's reach over a
/// member's instance (`mate::solve`'s `part_of`, which the lever's
/// `pair_reach` and a `FromFace` side's face pose both ask through).
/// The descent enters every reference it names before the document
/// evaluates, and a nested cache refuses any other ask
/// ([`PartFault::NotEntered`]).
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
    doc: ProfileDoc,
    /// Its `InstantiatePart` references in document order, repeats
    /// included; `next` is how far the descent has read them.
    refs: Vec<DocRef>,
    next: usize,
    reached: Rows<T>,
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
            doc.order()
                .iter()
                .filter_map(|&id| instantiated(&doc, id))
                .collect()
        } else {
            Vec::new()
        };
        Ok(Self {
            doc_ref: *doc_ref,
            doc,
            refs,
            next: 0,
            reached: BTreeMap::new(),
        })
    }

    /// The next reference this document instantiates that has no row
    /// yet.
    fn next_unreached(&mut self, eps_bits: u64) -> Option<DocRef> {
        while let Some(r) = self.refs.get(self.next).copied() {
            self.next += 1;
            if !self.reached.contains_key(&(r, eps_bits)) {
                return Some(r);
            }
        }
        None
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{PartCache, PartFault, Reached};
    use crate::ProfileDoc;
    use crate::ident::{ContentPin, DocRef, DocumentId};
    use crate::part::{PartResolver, ResolveFailure, ResolveFault};
    use geom_core::Tol;
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

    /// **A nested cache never descends.** Below the top, an ask the
    /// descent did not enter refuses `NotEntered` without reaching the
    /// resolver; at the top the same ask resolves. Goes red if a miss
    /// below the top descends again, which would put one more nested
    /// evaluation on the thread's stack per level.
    #[test]
    fn a_miss_below_the_top_refuses_not_entered_and_never_resolves() {
        let counting = Arc::new(Counting::default());
        let resolver: Arc<dyn PartResolver> = counting.clone();
        let chain = [doc_ref("parts-unit-above")];
        let defaults = super::super::EvalOptions::default();
        let cache = |chain| {
            PartCache::<f64>::new(
                Some(&resolver),
                chain,
                Reached::none(),
                defaults.boolean_sweep,
                defaults.profile_lift,
                Tol::witness(),
            )
        };
        let asked = doc_ref("parts-unit-asked");

        let nested = cache(&chain);
        assert!(
            matches!(
                nested.get(&asked, Tol::witness()),
                Err(PartFault::NotEntered)
            ),
            "a nested cache's miss is the kernel defect"
        );
        assert_eq!(
            counting.0.load(Ordering::Relaxed),
            0,
            "a nested cache's miss reaches no resolver"
        );
        assert_eq!(nested.evaluations(), 0, "and evaluates nothing");

        let top = cache(&[]);
        assert!(
            matches!(
                top.get(&asked, Tol::witness()),
                Err(PartFault::Unresolved { .. })
            ),
            "the top's miss descends, and this resolver refuses it"
        );
        assert_eq!(
            counting.0.load(Ordering::Relaxed),
            1,
            "the top's miss resolves once"
        );
    }
}
