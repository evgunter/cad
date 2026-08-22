//! The document seam (ASSEMBLY-DESIGN A2/A4; ASM-2A D-3): the door an
//! instantiate node reaches another document through.
//!
//! Evaluation is a KERNEL-layer service and knows nothing about files,
//! directories or workspaces. What it needs at the seam is exactly one
//! capability — turn a [`DocRef`] into the document it pins — so that
//! is the whole trait. The document layer implements it (`Workspace`);
//! kernel-layer tests implement it with a stub. An evaluation carrying
//! NO resolver refuses instantiate nodes typed rather than pretending
//! a part is empty.
//!
//! **Verification lives behind the door, and its VERDICT comes back
//! through it.** A11/A4 make the pin the version: a resolver hands back
//! a document only when the bytes hash to the pin the reference names.
//! The kernel does not re-check that (it would be the same computation,
//! done twice, in the layer that owns neither the file nor the store) —
//! it classifies the refusal, which is why [`ResolveFault`] exists: the
//! three seam failures ASSEMBLY-DESIGN singles out are separately
//! observable without the kernel importing the document layer's error
//! type.

use crate::ident::DocRef;
use crate::program::ProfileDoc;
use geom_core::Tol;

/// Which seam rule a resolution failed (A2/A4). The classification is
/// the RESOLVER's — it is the layer that knows why — and it exists so
/// the two ratified seam refusals stay distinguishable from ordinary
/// "no such document" at the point a caller catches them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResolveFault {
    /// A4's version gate: the document resolved, and its content pin
    /// disagrees with the reference's. References are never retargeted
    /// silently; moving the pin is its own recorded edit.
    PinMismatch,
    /// A2's ε seam: the referenced document's recorded ε disagrees with
    /// the committed process ε, so every predicate below it would
    /// decide at the wrong tolerance.
    EpsilonSeam,
    /// Anything else the store refuses: unknown id, unreadable file,
    /// unparseable or invalid document.
    Unresolved,
}

/// A typed resolution failure: the classified fault, plus the
/// resolver's own rendering (which carries the store-level detail and
/// recourse the kernel deliberately does not model).
#[derive(Debug, Clone)]
pub struct ResolveFailure {
    /// Which seam rule failed.
    pub fault: ResolveFault,
    /// The resolver's human-readable diagnosis.
    pub message: String,
}

impl core::fmt::Display for ResolveFailure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// The document seam: [`DocRef`] → the document it pins.
///
/// `Send + Sync` because evaluation may run its nodes under rayon;
/// `Debug` because [`crate::EvalOptions`] carries one and every option
/// struct in this crate is inspectable.
pub trait PartResolver: core::fmt::Debug + Send + Sync {
    /// Resolves a reference to the document it pins.
    ///
    /// # Errors
    ///
    /// [`ResolveFailure`] — the implementation's own refusal,
    /// classified by [`ResolveFault`]. A document is returned ONLY if
    /// it hashes to the reference's pin.
    fn resolve(&self, doc_ref: &DocRef, tol: Tol) -> Result<ProfileDoc, ResolveFailure>;
}
