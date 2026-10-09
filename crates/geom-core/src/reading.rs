//! **Where a refusal is read** (D4 ¶1 (i)): the door that reports it,
//! which decides who may have made what it refuses, and so the ending.

/// Where a refusal is read: the door that reports it, which decides the
/// ending (D4 ¶1 (i)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reading {
    /// At the operation that built the geometry: it came from the
    /// kernel's own construction.
    Build,
    /// Over a body at rest, which a damaged file reaches as surely as a
    /// defective operation does. Certification at STEP adoption reads
    /// here too (D4 ¶1); the import door's own ε_in words are the
    /// margin's ([`crate::MarginDiag::sized_recourse_in_file`],
    /// [`crate::FileCoincidence::miss_recourse_in_file`]).
    AtRest,
}
