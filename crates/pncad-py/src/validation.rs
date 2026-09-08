//! **What one validator finding says**, projected into the words a
//! Python caller branches on.
//!
//! The validator doors are the one place in this binding where a
//! single raise reports MANY refusals: `Body::validate*` answers
//! `Result<(), Vec<ValidationError>>`, and the exception carries the
//! whole vector. So the discriminant every other refusal projects as a
//! scalar attribute is a SEQUENCE here — one [`Finding`] per element,
//! in the kernel's own deterministic report order — and
//! `failure_count` is that sequence's length.
//!
//! Python-independent on purpose, like the rest of this crate's
//! payload projection: the words are minted by `crate::tags`, whose
//! matches are exhaustive and whose drift alarm therefore fires on the
//! default no-interpreter build, and this module is the shape they are
//! assembled into.
//!
//! **Every field is present on every finding**, `None` where the arm
//! carries nothing to fill it — the "no `getattr` trap" rule the
//! projected doors already keep.

use pncad::topo::{CensusContact, CensusSubject, EntityId, ValidationError};

use crate::tags::{census_contact_tag, census_subject_tag, entity_id_tag, validation_error_tag};

/// One validator finding, as words.
///
/// The arena KEYS are not here and cannot be: Python holds an opaque
/// `Body` handle and no key crosses it. What crosses is the
/// discriminant a caller acts on — which arm refused, what it was
/// about, which coincidence it found — and the kernel's own `Display`
/// prose on the joined message carries the rest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Finding {
    /// Which `ValidationError` arm refused.
    pub variant: &'static str,
    /// What a census refusal was ABOUT: `"entity"` or `"face_pair"`.
    /// `None` on every arm that carries no census subject.
    pub subject_kind: Option<&'static str>,
    /// The entity kind of an `"entity"` subject. `None` for a
    /// `"face_pair"` subject — whose two sides are faces by
    /// construction — and for every arm with no subject.
    pub entity_kind: Option<&'static str>,
    /// Which coincidence the tier-3′ census found. `None` on every arm
    /// that reports no census contact.
    pub contact_kind: Option<&'static str>,
}

/// Read one kernel finding into the words Python receives.
pub fn project(err: &ValidationError) -> Finding {
    let subject = census_subject(err);
    Finding {
        variant: validation_error_tag(err),
        subject_kind: subject.map(census_subject_tag),
        entity_kind: subject.and_then(subject_entity).map(entity_id_tag),
        contact_kind: census_contact(err).map(census_contact_tag),
    }
}

/// The census subject an arm carries, if it carries one.
///
/// The wildcard is the kernel enum's own EXTRACT licence: a site that
/// picks one variant out and answers `None` to the rest is asking a
/// question, not mapping the enum onto a smaller vocabulary, and the
/// `_` arm IS the question. The site that classifies is
/// [`validation_error_tag`], which is exhaustive — so a kernel arm
/// added with a subject to carry stops this crate compiling there, in
/// front of the person who then decides what it projects here.
fn census_subject(err: &ValidationError) -> Option<&CensusSubject> {
    match err {
        ValidationError::CensusUnsupported { subject }
        | ValidationError::CensusLaneUnsupported { subject } => Some(subject),
        _ => None,
    }
}

/// The entity a subject names, when the subject is one entity.
fn subject_entity(subject: &CensusSubject) -> Option<&EntityId> {
    match subject {
        CensusSubject::Entity(entity) => Some(entity),
        CensusSubject::FacePair(_, _) => None,
    }
}

/// The coincidence an arm reports, if it reports one.
///
/// [`census_subject`]'s reading, at the other payload.
fn census_contact(err: &ValidationError) -> Option<&CensusContact> {
    match err {
        ValidationError::UndeclaredContact { contact, .. } => Some(contact),
        _ => None,
    }
}
