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
///
/// **UNTESTED, and labelled as such rather than left looking
/// guarded.** `getrandom::fill` has no injection seam, so this arm —
/// and the `IdentityError` it becomes in Python — is reachable only
/// on a host whose entropy source is broken. What IS pinned is the
/// class mapping (`error_classes_name_the_python_hierarchy`), the tag
/// (`workspace_error_tags_are_stable`, over the map the raise site
/// READS rather than a literal it writes), and the absence of any
/// fallback: the `?` here is the whole control flow, so there is no
/// second path for a constant to hide on.
///
/// The `variant` a caller sees is therefore whichever
/// [`WorkspaceError`] arrives, not a name chosen at the raise site.
/// That `RandomnessUnavailable` is the only arm this function can
/// produce is a fact about [`random_document_id`] — stated here, and
/// relied on nowhere.
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
