//! Document identity for Python-authored documents.
//!
//! A `DocumentId` answers "which part". The workspace store's
//! uniqueness invariant is keyed on it, and so are `DocRef`/
//! `ContentPin` cross-document references — so two documents sharing
//! an id are the same part, and a store refuses to hold both.
//!
//! Python is the INTERACTIVE authoring surface, so a document
//! authored with no identity argument gets a fresh random id
//! ([`interactive`]) — the façade's own interactive door, and the
//! reason two `Doc()`s can coexist in one workspace. Reproducibility
//! is the LABELLED spelling ([`derived`]): same label, same id, on
//! every platform, which is what corpus and regeneration callers
//! need.
//!
//! This module is deliberately free of `pyo3`: it compiles and is
//! tested on the default (no-Python) build path.

use pncad::document::ProfileDoc;
use pncad::workspace::{WorkspaceError, random_document_id};

/// An empty document under a FRESH random identity — the interactive
/// spelling, and the default one from Python.
///
/// # Errors
///
/// [`WorkspaceError::RandomnessUnavailable`] if the OS entropy source
/// refuses. Surfaced, never papered over with a weaker source or with
/// a constant.
pub fn interactive() -> Result<ProfileDoc, WorkspaceError> {
    Ok(ProfileDoc::empty(random_document_id()?))
}

/// An empty document whose identity is derived from `label` — the
/// deterministic spelling, for callers whose saves must reproduce
/// byte for byte.
///
/// Two documents authored under the SAME label are the same part, by
/// construction. That is the point of the door, and it is why it is
/// not the default.
pub fn derived(label: &str) -> ProfileDoc {
    ProfileDoc::empty_derived(label)
}
