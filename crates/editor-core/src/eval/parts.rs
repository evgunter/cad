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

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use geom_core::Decide;
use topo::Body;

use crate::ident::DocRef;
use crate::names::NameTable;
use crate::part::{PartResolver, ResolveFault};

/// How deep instantiation may nest before evaluation refuses. Content
/// pins make a reference cycle unconstructible (a document would have
/// to contain its own hash), so this bound is not a cycle detector —
/// it is the guard that keeps a MISBEHAVING resolver, which may hand
/// back any document it likes, from recursing without end.
pub(crate) const MAX_DEPTH: u32 = 32;

/// A resolved part: the referenced document's product, and the product
/// entities' part-local stable names.
pub(crate) struct PartValue<T: Decide> {
    pub body: Arc<Body<T>>,
    pub names: Arc<NameTable>,
}

impl<T: Decide> Clone for PartValue<T> {
    fn clone(&self) -> Self {
        Self {
            body: Arc::clone(&self.body),
            names: Arc::clone(&self.names),
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
    /// The referenced document evaluated, but its own product refused
    /// (a failed root, an invalid gather).
    PartProduct {
        /// The product door's diagnosis.
        message: String,
    },
    /// The referenced document's product holds N ≠ 1 solids.
    /// **ASM-2b** is the flip condition: it owns multi-solid
    /// instantiation, including the name bridge for it. Partial
    /// support here would be a second, narrower truth about what
    /// instantiating a document means.
    MultiSolid {
        /// How many solids the product holds.
        solids: usize,
    },
    /// Instantiation nested past [`MAX_DEPTH`].
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
            Self::PartProduct { message } => {
                write!(f, "the referenced document has no product: {message}")
            }
            Self::MultiSolid { solids } => write!(
                f,
                "the referenced document's product holds {solids} solids; single-solid parts are \
                 this door's scope and multi-solid instantiation is ASM-2b's"
            ),
            Self::DepthExceeded => {
                write!(f, "instantiation nested deeper than {MAX_DEPTH} documents")
            }
        }
    }
}

/// One cache row: the resolved part, or the typed reason there is
/// none. Keyed by `(DocRef, ambient ε bits)` — the module docs' key.
type Rows<T> = BTreeMap<(DocRef, u64), Result<PartValue<T>, PartFault>>;

/// The cache. One per `evaluate` call; shared across its nodes.
pub(crate) struct PartCache<'a, T: Decide> {
    resolver: Option<&'a Arc<dyn PartResolver>>,
    /// This evaluation's nesting depth (0 at the outermost call).
    depth: u32,
    /// The ambient ε these entries were produced at, by bits — half
    /// the key, hoisted because it is constant within one evaluation.
    eps_bits: u64,
    entries: Mutex<Rows<T>>,
    /// How many times a referenced document was actually evaluated —
    /// the D-3 sharing evidence. A counter, not a timing claim.
    evaluations: AtomicUsize,
}

impl<'a, T: Decide> PartCache<'a, T> {
    pub(crate) fn new(resolver: Option<&'a Arc<dyn PartResolver>>, depth: u32) -> Self {
        Self {
            resolver,
            depth,
            eps_bits: geom_core::Tolerance::get().eps.to_bits(),
            entries: Mutex::new(BTreeMap::new()),
            evaluations: AtomicUsize::new(0),
        }
    }

    /// How many referenced-document evaluations this cache ran.
    pub(crate) fn evaluations(&self) -> usize {
        self.evaluations.load(Ordering::Relaxed)
    }
}

impl<T> PartCache<'_, T>
where
    T: Decide + super::ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane,
{
    /// The part `doc_ref` denotes, evaluated at most once per key.
    ///
    /// The lock is held across the miss path on purpose: it is what
    /// makes "evaluated ONCE" true when two instances of one part race,
    /// and a nested evaluation builds its own cache, so the lock is
    /// never re-entered.
    pub(crate) fn get(&self, doc_ref: &DocRef) -> Result<PartValue<T>, PartFault> {
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
        let value = self.resolve_and_evaluate(doc_ref);
        entries.insert(key, value.clone());
        value
    }

    fn resolve_and_evaluate(&self, doc_ref: &DocRef) -> Result<PartValue<T>, PartFault> {
        let resolver = self.resolver.ok_or(PartFault::NoResolver)?;
        if self.depth >= MAX_DEPTH {
            return Err(PartFault::DepthExceeded);
        }
        let doc = resolver
            .resolve(doc_ref)
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
            boolean_sweep: topo::SweepStrategy::Realized,
            resolver: self.resolver.map(Arc::clone),
        };
        let evaluation =
            super::evaluate_nested::<T>(&doc, &super::CancelToken::new(), &opts, self.depth + 1);
        // A2's uniformity: what a document MEANS is its product, one
        // rule everywhere. A failed node inside the part surfaces
        // through the product door's own typed refusal.
        let (body, names) = crate::product::product_named(&doc, &evaluation).map_err(|e| {
            PartFault::PartProduct {
                message: e.to_string(),
            }
        })?;
        let solids = body.solids().count();
        if solids != 1 {
            return Err(PartFault::MultiSolid { solids });
        }
        Ok(PartValue {
            body: Arc::new(body),
            names: Arc::new(names),
        })
    }
}
