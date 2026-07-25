//! Persistence, schema v1 (M4 PR 6; F3 + F8 ratified).
//!
//! # Format (spec D1)
//!
//! A save is TEXT: a `schema: <integer>` header line, then a JSON body
//! `{ "snapshot": <Doc>, "edits": [<DocEdit>…] }` — the full document
//! snapshot plus the edit log since that snapshot. JSON via
//! `serde_json` is the ratified shape's PR-spec aesthetic choice
//! (REPORTED): floats serialize through ryu (shortest round-trip —
//! exactly D2's contract), parse errors carry line/column (D6.3's
//! typed position info), map keys with integer newtypes work natively,
//! and the format is universally diffable/greppable. Structural map
//! keys (the appearance store's [`StableName`]s) serialize as pair
//! LISTS ([`pairs`]) — structural, not stringified.
//!
//! # What persists (spec D3)
//!
//! The recipe IS the save: nodes, parameters, expressions, witness
//! bytes (hex, bit-exact), the appearance store (records incl. D7
//! metadata), recorded ε, the schema version, and the edit log.
//! Deliberately NOT persisted: evaluations, name tables,
//! memo/content/naming keys, arena anything — all of it re-derives on
//! replay, and the save/load/replay-identity CI row pins that the
//! re-derivation is bit-identical.
//!
//! # Doors (fail loud, D2/D6.3)
//!
//! - Save refuses non-finite floats with a typed error naming the
//!   site ([`NonFiniteSite`]); `-0.0` is data and round-trips.
//! - Load walks the header → migration chain → typed deserialize →
//!   structural validation ([`SnapshotError`]) → expression REBUILD
//!   through the dimension-checking constructors ([`wire`]) → edit
//!   replay through [`crate::edit::apply`]'s doors → ε reconciliation
//!   (D4). No silent best-effort loads, ever.
//!
//! # ε wiring (spec D4)
//!
//! The document records its ε. On load, [`load`] commits the recorded
//! ε as the process tolerance if the process has not committed one
//! (the document wins over an unread environment; K still resolves
//! from the environment — [`Tolerance::init_document_eps`]); if the
//! process HAS committed (bootstrap `get()` or an earlier document),
//! a bit-different recorded ε refuses loudly
//! ([`PersistError::ToleranceConflict`]) — one process, one ε.
//! [`crate::eval::evaluate`] enforces the same invariant per run.

pub mod hexbytes;
pub(crate) mod pairs;
mod check;
mod wire;

use geom_core::tolerance::{Tolerance, ToleranceError};

use crate::edit::{Applied, DocEdit, EditError, EditRecord, apply};
use crate::profile_desc::{ProfileDesc, ProfileDoc};

pub use check::{NonFiniteSite, SnapshotError};

/// The current schema version. Version 1 froze at M4 PR 6 (F8: the
/// persisted file IS in M4). Bump ONLY with a ratified format change
/// plus its [`migrate`] step.
pub const SCHEMA_VERSION: u32 = 1;

/// The serialized body under the header: snapshot + edit log (D1).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FileBody {
    /// The full document snapshot.
    snapshot: ProfileDoc,
    /// The recorded edits since the snapshot, replayed on load.
    edits: Vec<DocEdit<ProfileDesc>>,
}

/// A loaded document: the parsed snapshot, the parsed edit log, and
/// the REPLAYED result (snapshot + edits through [`apply`]'s doors —
/// the document's current state).
#[derive(Debug)]
pub struct Loaded {
    /// The snapshot as saved.
    pub snapshot: ProfileDoc,
    /// The edit log as saved.
    pub edits: Vec<DocEdit<ProfileDesc>>,
    /// The current document: snapshot with every edit replayed.
    pub doc: ProfileDoc,
    /// The replay's edit records (minted ids etc.), one per edit.
    pub records: Vec<EditRecord>,
}

/// Typed persistence refusal (spec D2/D6.3 — never a silent
/// best-effort load, never a stringly error at the API).
#[derive(Debug, Clone, PartialEq)]
pub enum PersistError {
    /// A non-finite float at save time, naming the site (D2: the
    /// kernel never legitimately produces NaN/inf — this is a
    /// surfaced bug, not data).
    NonFinite {
        /// Where the non-finite value sits.
        site: NonFiniteSite,
    },
    /// The serializer itself failed (I/O-free here, so effectively
    /// unreachable; surfaced rather than swallowed).
    Serialize {
        /// The serializer's message.
        message: String,
    },
    /// The file has no parseable `schema: <integer>` header line.
    Header {
        /// What the first line looked like (truncated).
        found: String,
    },
    /// The header names a schema this build does not know (D6.3:
    /// refuse typed; migrations only run FORWARD from older versions).
    UnknownSchema {
        /// The version in the file.
        found: u64,
        /// The newest version this build reads.
        newest: u32,
    },
    /// A migration step failed or is missing.
    Migration(MigrationError),
    /// The body is not valid JSON, or not the typed shape — with the
    /// serde_json position (1-based line/column into the BODY, i.e.
    /// after the header line).
    Parse {
        /// Line within the body.
        line: usize,
        /// Column within the line.
        column: usize,
        /// The parser's message.
        message: String,
    },
    /// The snapshot parsed but violates a document invariant.
    Snapshot(SnapshotError),
    /// An edit in the log refused during replay (the [`apply`] door).
    EditReplay {
        /// The refusing edit's index in the log.
        index: usize,
        /// The typed refusal.
        error: EditError,
    },
    /// The document's recorded ε conflicts with the ε this process
    /// already committed (D4: one process = one ε; refuse loudly).
    ToleranceConflict {
        /// The process's committed ε.
        process: f64,
        /// The document's recorded ε.
        document: f64,
    },
    /// The document's recorded ε is not a valid tolerance.
    ToleranceInvalid {
        /// The recorded value.
        value: f64,
    },
}

/// A failed or unavailable migration step (spec D1: migrations are
/// explicit version-to-version functions from v1 onward).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationError {
    /// The version the failing step was migrating FROM.
    pub from: u32,
    /// Why.
    pub reason: String,
}

/// One migration step: `from_version` → `from_version + 1`, on the
/// raw JSON body (spec D1's ratified mechanism — the loader walks
/// this chain from the file's version up to [`SCHEMA_VERSION`], then
/// deserializes typed). v1 is current, so no step exists yet; the
/// first format change adds `1 => Ok(...)` here and bumps
/// [`SCHEMA_VERSION`].
fn migrate(from_version: u32, _value: serde_json::Value) -> Result<serde_json::Value, MigrationError> {
    Err(MigrationError {
        from: from_version,
        reason: format!("no migration step from schema v{from_version}"),
    })
}

/// Serializes `snapshot` + `edits` into the schema-v1 text format.
///
/// # Errors
///
/// [`PersistError::NonFinite`] naming the site of any NaN/inf in the
/// document or edit log (D2); [`PersistError::Serialize`] if the JSON
/// writer itself fails.
pub fn save(
    snapshot: &ProfileDoc,
    edits: &[DocEdit<ProfileDesc>],
) -> Result<String, PersistError> {
    if let Some(site) = check::first_non_finite(snapshot, edits) {
        return Err(PersistError::NonFinite { site });
    }
    let body = SerBody { snapshot, edits };
    let json = serde_json::to_string_pretty(&body).map_err(|e| PersistError::Serialize {
        message: e.to_string(),
    })?;
    Ok(format!("schema: {SCHEMA_VERSION}\n{json}\n"))
}

/// The borrowing twin of [`FileBody`] (save side).
#[derive(serde::Serialize)]
struct SerBody<'a> {
    snapshot: &'a ProfileDoc,
    edits: &'a [DocEdit<ProfileDesc>],
}

/// Parses, migrates, validates, replays, and ε-reconciles a schema-v1
/// (or migratable older) save. See the module docs for the door
/// sequence; every failure is a typed [`PersistError`].
///
/// # Errors
///
/// Every arm of [`PersistError`] except `NonFinite`/`Serialize`.
pub fn load(text: &str) -> Result<Loaded, PersistError> {
    let (version, body_text) = parse_header(text)?;
    // Migration chain (D1): walk explicit steps up to the current
    // version, then deserialize typed.
    let body: FileBody = if version == SCHEMA_VERSION {
        parse_body(body_text)?
    } else {
        let mut value: serde_json::Value = serde_json::from_str(body_text).map_err(parse_err)?;
        let mut at = version;
        while at < SCHEMA_VERSION {
            value = migrate(at, value).map_err(PersistError::Migration)?;
            at += 1;
        }
        serde_json::from_value(value).map_err(parse_err)?
    };
    check::validate_snapshot(&body.snapshot).map_err(PersistError::Snapshot)?;
    // Replay through apply's doors: the loaded current state is the
    // replayed state, never trusted bytes.
    let mut doc = body.snapshot.clone();
    let mut records = Vec::with_capacity(body.edits.len());
    for (index, edit) in body.edits.iter().enumerate() {
        let Applied { doc: next, record } =
            apply(&doc, edit).map_err(|error| PersistError::EditReplay { index, error })?;
        doc = next;
        records.push(record);
    }
    reconcile_epsilon(doc.epsilon())?;
    Ok(Loaded {
        snapshot: body.snapshot,
        edits: body.edits,
        doc,
        records,
    })
}

/// D4's ε wiring: commit the document's recorded ε as the process
/// tolerance, or verify it matches the one already committed.
fn reconcile_epsilon(eps: f64) -> Result<(), PersistError> {
    match Tolerance::init_document_eps(eps) {
        Ok(()) => Ok(()),
        Err(ToleranceError::AlreadyInitialized { current, .. }) => {
            if current.eps.to_bits() == eps.to_bits() {
                Ok(())
            } else {
                Err(PersistError::ToleranceConflict {
                    process: current.eps,
                    document: eps,
                })
            }
        }
        Err(_) => Err(PersistError::ToleranceInvalid { value: eps }),
    }
}

/// Splits the `schema: <integer>` header from the body and validates
/// the version window.
fn parse_header(text: &str) -> Result<(u32, &str), PersistError> {
    let (first, rest) = text.split_once('\n').unwrap_or((text, ""));
    let found = || first.chars().take(80).collect::<String>();
    let Some(version_text) = first.strip_prefix("schema:") else {
        return Err(PersistError::Header { found: found() });
    };
    let Ok(version) = version_text.trim().parse::<u64>() else {
        return Err(PersistError::Header { found: found() });
    };
    if version == 0 || version > u64::from(SCHEMA_VERSION) {
        return Err(PersistError::UnknownSchema {
            found: version,
            newest: SCHEMA_VERSION,
        });
    }
    // The window check above keeps this cast exact.
    Ok((version as u32, rest))
}

fn parse_body(body_text: &str) -> Result<FileBody, PersistError> {
    serde_json::from_str(body_text).map_err(parse_err)
}

fn parse_err(e: serde_json::Error) -> PersistError {
    PersistError::Parse {
        line: e.line(),
        column: e.column(),
        message: e.to_string(),
    }
}
