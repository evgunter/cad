//! Canonical bytes and the content pin (ASM-1 D-2/D-3, as amended
//! 2026-08-10: include-by-default).
//!
//! [`canonical_bytes`] serializes the REPLAYED document — the same
//! deterministic machinery the save body uses (BTreeMaps, pair-list
//! structural keys, ryu shortest-round-trip floats) — with exactly
//! TWO exclusions, each structurally justified:
//!
//! - the **edit log** — history is not state: two edit paths to one
//!   snapshot are the same version, and undo must not move pins
//!   (the log never appears here because the input is a [`ProfileDoc`]
//!   value, not a save file; callers holding a (snapshot, log) pair
//!   pin the replayed state, [`super::Loaded::doc`] — the same
//!   replay-before-trust discipline as [`super::save`]/[`super::load`] —
//!   so pin stability across no-op saves holds by construction);
//! - the **[`DocumentId`]** — A4: the id answers "which part", the
//!   pin "which version"; a document copied under a fresh id is
//!   detectably the same content.
//!
//! Everything else is INCLUDED — nodes, order, params, recorded ε,
//! witnesses, metadata, appearance, and every FUTURE `Doc` field,
//! automatically: the preimage is the document's own serde form with
//! the `id` key removed, not a curated field list, so a grown field
//! pins by default instead of relying on someone remembering to
//! classify it (the amendment's silent-omission rationale). New
//! exclusions are explicit carve-outs earned by a demonstrated
//! problem, and they ride a schema seam.
//!
//! Stated consequence (spec, honest): an appearance-only edit moves
//! the pin — a consuming assembly sees an update whose
//! re-verification passes trivially. Accepted v1 noise.
//!
//! [`DocumentId`]: crate::ident::DocumentId

use crate::ident::ContentPin;
use crate::program::ProfileDoc;

use super::{PersistError, check};
use geom_core::Tol;

/// The canonical bytes of the CURRENT document state (module docs:
/// the full serde form minus the `id` key; the edit log is absent
/// because the input is the replayed value). Runs the shared
/// validator first (with an empty log), so a document that could not
/// be saved cannot be pinned. Deterministic: object keys land in
/// serde_json's sorted map order, floats through ryu — bit-exact.
///
/// # Errors
///
/// The shared validator's arms ([`PersistError::NonFinite`],
/// [`PersistError::ProfileProgram`], [`PersistError::Snapshot`]), and
/// [`PersistError::Serialize`] if the JSON writer itself fails or
/// the serialized document is not the object shape this build wrote
/// (unreachable short of a serde-impl bug; surfaced, not swallowed).
pub fn canonical_bytes(doc: &ProfileDoc, tol: Tol) -> Result<Vec<u8>, PersistError> {
    check::validate_document(doc, &[], tol)?;
    let mut value = serde_json::to_value(doc).map_err(|e| PersistError::Serialize {
        message: e.to_string(),
    })?;
    let Some(object) = value.as_object_mut() else {
        return Err(PersistError::Serialize {
            message: "canonical projection: the document did not serialize as an object".into(),
        });
    };
    // The ONE field carve-out (D-3 as amended): identity is not
    // content. The key is structurally present (`Doc::id` has no
    // skip attribute); its absence is a serde-impl bug, surfaced.
    if object.remove("id").is_none() {
        return Err(PersistError::Serialize {
            message: "canonical projection: the document object carries no `id` key".into(),
        });
    }
    let json = serde_json::to_string(&value).map_err(|e| PersistError::Serialize {
        message: e.to_string(),
    })?;
    Ok(json.into_bytes())
}

/// The document's content pin: SHA-256 of [`canonical_bytes`] (ASM-1
/// D-2; ASSEMBLY-DESIGN A4 — the pin IS version identity).
///
/// # Errors
///
/// Exactly [`canonical_bytes`]'s.
pub fn content_pin(doc: &ProfileDoc, tol: Tol) -> Result<ContentPin, PersistError> {
    Ok(ContentPin::of_bytes(&canonical_bytes(doc, tol)?))
}
