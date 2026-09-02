//! Persistence: the ratified text format of M4 PR 6.
//!
//! # No schema version, on purpose
//!
//! This format carries NO hand-maintained schema version, no migration
//! chain and no bump-coordination convention. Schema breaks are not at
//! all a problem, because this is not released yet: no document exists
//! outside this repository, and every checked-in document is a
//! regenerable artifact (the tour writes the corpus; the fixtures
//! re-bless from their authoring functions). A format change therefore
//! needs no version, no migration and no coordination — regenerate the
//! corpus and move on. Versioning returns as Band-4 work the day a
//! document ships to someone (`docs/DESIGN.md`, the Band 4 roadmap
//! line).
//!
//! What stays is ONE door: a file this build cannot read refuses TYPED
//! ([`PersistError::Unreadable`]) with [`REGENERATE_RECOURSE`], carried
//! by the deserializer's own rejection of unknown or missing
//! vocabulary — serde_json names the variant or field it could not
//! place, and the refusal forwards that name. Both directions of
//! growth follow from that one door. An OLDER document lacking
//! vocabulary this build has since grown (a new node arm, a new
//! optional field) never names it, so it loads — additive growth
//! invalidates nothing. A NEWER document carrying a field this build
//! lacks refuses (every wire type is `deny_unknown_fields`): a stale
//! reader must not silently drop data. A BREAKING change — a field
//! made required, a spelling retired — refuses naming the field. The
//! recourse is the one sentence it always was, and it is also on
//! [`PersistError::HeaderId`], because a document from before the
//! `id:` line is a file this build cannot read too.
//!
//! Which arm a refusal lands on is decided by serde_json's own
//! classification of its failure and by nothing else — the one
//! statement of that seam, with its executed edges, is on
//! [`parse_err`].
//!
//! # Format (spec D1)
//!
//! A save is TEXT: an `id: <32 lowercase hex>` header line naming the
//! document's identity (ASM-1 D-6 — the workspace scan reads it
//! without parsing the body), then a JSON body
//! `{ "snapshot": <Doc>, "edits": [<DocEdit>…] }` — the full document
//! snapshot plus the edit log since that snapshot. (One known hatch,
//! pinned rather than closed: serde's derived struct visitor also
//! accepts the two fields POSITIONALLY, so a body spelled
//! `[<snapshot>, <edits>]` loads. No writer produces it, the loaded
//! document still walks every validator, and closing it would mean a
//! hand-written visitor for a struct that exists only to be derived —
//! `tests/bool13_r1_probes.rs::a_positional_array_body_loads` is the
//! pin.) JSON via
//! `serde_json` is the ratified shape's PR-spec aesthetic choice
//! (REPORTED): floats serialize through ryu (shortest round-trip —
//! exactly D2's contract), parse errors carry line/column (typed
//! position info), map keys with integer newtypes work natively, and
//! the format is universally diffable/greppable. Structural map keys
//! (the appearance store's [`crate::StableName`]s) serialize as pair
//! LISTS ([`pairs`]) — structural, not stringified.
//!
//! # What persists (spec D3)
//!
//! The recipe IS the save: the document id, nodes, parameters,
//! expressions, witness bytes (hex, bit-exact), the appearance store
//! (records incl. D7 metadata), recorded ε, and the edit log.
//! Deliberately NOT persisted: evaluations, name tables,
//! memo/content/naming keys, arena anything — and the profile
//! programs' REPLAYED SEGMENTS (vertices/bulges/joints are replay
//! products of the stored programs; V3: caches live in the evaluation
//! memo, never on disk) — all of it re-derives on replay, and the
//! save/load/replay-identity CI row pins that the re-derivation is
//! bit-identical.
//!
//! # Doors (fail loud, D2; DESIGN engineering convention 2)
//!
//! Every direction-independent document check lives in ONE shared
//! validator ([`check`]'s `validate_document`: non-finite floats
//! ([`NonFiniteSite`]), profile-program structure faults
//! ([`ProgramFault`]), the structural document invariants
//! ([`SnapshotError`])), invoked by BOTH doors — a document that
//! would refuse to load cannot be saved, by construction rather than
//! by mirrored sweeps.
//!
//! - Save runs the shared validator on the in-memory document, then
//!   verifies the log replays — everything the LOAD side would
//!   refuse, refused before a byte is written (never an unloadable
//!   file); `-0.0` is data and round-trips.
//! - Load walks the header → typed deserialize (expression REBUILD
//!   through the dimension-checking constructors and the wire-only
//!   canonical-set rule, [`wire`]) → the shared validator → edit
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

mod canon;
mod check;
pub mod hexbytes;
/// The bytes of kernel types, described from above the layering
/// boundary — see the module's own docs for the rules a new one
/// follows.
pub(crate) mod kernel_wire;
pub(crate) mod pairs;
pub(crate) mod strict;
mod wire;

use geom_core::tolerance::{Tolerance, ToleranceError};

use crate::edit::{Applied, DocEdit, EditError, EditRecord, apply};
use crate::ident::DocumentId;
use crate::program::{ProfileDoc, ProfileProgram};
use geom_core::Tol;

pub use canon::{canonical_bytes, content_pin};
pub use check::{NonFiniteSite, ProgramFault, SnapshotError};

/// The serialized body under the header: snapshot + edit log (D1).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FileBody {
    /// The full document snapshot.
    snapshot: ProfileDoc,
    /// The recorded edits since the snapshot, replayed on load.
    edits: Vec<DocEdit<ProfileProgram>>,
}

/// A loaded document: the parsed snapshot, the parsed edit log, and
/// the REPLAYED result (snapshot + edits through [`apply`]'s doors —
/// the document's current state).
#[derive(Debug)]
pub struct Loaded {
    /// The snapshot as saved.
    pub snapshot: ProfileDoc,
    /// The edit log as saved.
    pub edits: Vec<DocEdit<ProfileProgram>>,
    /// The current document: snapshot with every edit replayed.
    pub doc: ProfileDoc,
    /// The replay's edit records (minted ids etc.), one per edit.
    pub records: Vec<EditRecord>,
}

/// Typed persistence refusal (spec D2 — never a silent best-effort
/// load, never a stringly error at the API).
#[derive(Debug, Clone, PartialEq)]
pub enum PersistError {
    /// A non-finite float in the document or edit log, naming the
    /// site (D2: the kernel never legitimately produces NaN/inf —
    /// this is a surfaced bug, not data). Shared-validator check; in
    /// practice a save-door refusal, since JSON's lack of non-finite
    /// tokens makes it unreachable post-parse.
    NonFinite {
        /// Where the non-finite value sits.
        site: NonFiniteSite,
    },
    /// A profile PROGRAM structure fault (LIB-SWITCH §4h; the retired
    /// stored-joint refusal's successor): a wrong-dimension argument
    /// role, or a lattice-violating step order caught by the replay
    /// probe. The payload is `pub`, so an in-crate bug can build one
    /// without passing an edit door, and a parsed file can carry the
    /// same corruption. Shared-validator check: save refuses before a
    /// byte is written, load refuses with the SAME diagnostics.
    ProfileProgram {
        /// The profile node carrying the fault.
        node: crate::node::RecipeNodeId,
        /// The typed fault.
        fault: check::ProgramFault,
    },
    /// A document parameter's distribution breaks an E2 invariant
    /// (ERROR-DESIGN E2; finiteness is [`Self::NonFinite`]'s half).
    /// Shared-validator check, by the same `Distribution::check` the
    /// edit door runs: save refuses before a byte is written, and a
    /// hand-written file refuses at LOAD with the same diagnostics —
    /// never a best-effort load.
    Distribution {
        /// The parameter carrying the fault.
        name: crate::doc::ParamName,
        /// The invariant that failed.
        fault: crate::distribution::DistributionFault,
    },
    /// A document parameter's authored display unit does not MEASURE
    /// the dimension it was declared with — `mm` on an `Angle`
    /// parameter (LIB-SWITCH §4g's `DisplayUnitMismatch`, at the
    /// document-parameter carrier rather than at a literal).
    ///
    /// Reachable two ways, which is why it is a validator walk and not
    /// a door check: the `DocParam` payload is `pub`, and a file can
    /// pair any dimension with any table symbol. The AUTHORING doors
    /// ([`crate::DocParam::written_length`] /
    /// [`crate::DocParam::written_angle`]) cannot produce one — they
    /// take a typed carrier whose unit already agrees. Shared-validator
    /// check, so save refuses before a byte is written and load refuses
    /// with the same diagnostics.
    ///
    /// An OFF-TABLE symbol is a different fault and refuses earlier, at
    /// the token, in `UnitSym`'s `Deserialize` — this walk sees only
    /// units that are rows of the table.
    DisplayUnit {
        /// The parameter carrying the fault.
        name: crate::doc::ParamName,
        /// The dimension the unit measures.
        unit: crate::expr::Dimension,
        /// The dimension the parameter was declared with.
        declared: crate::expr::Dimension,
    },
    /// The serializer itself failed (I/O-free here, so effectively
    /// unreachable; surfaced rather than swallowed).
    Serialize {
        /// The serializer's message.
        message: String,
    },
    /// The file has no parseable `id: <32 lowercase hex>` header line
    /// (the workspace scan reads identity from the header without
    /// parsing the body; canonical spelling only). Carries
    /// [`REGENERATE_RECOURSE`]: a document written before the `id:`
    /// line existed lands here, and it is a file this build cannot
    /// read exactly as an [`Self::Unreadable`] body is.
    HeaderId {
        /// What the id line looked like (truncated).
        found: String,
    },
    /// The header's `id:` line and the snapshot's own id field
    /// disagree — a tampered or hand-assembled file (the save door
    /// writes the snapshot's id, so the two agree by construction).
    IdMismatch {
        /// The id the header line names.
        header: DocumentId,
        /// The id the snapshot carries.
        snapshot: DocumentId,
    },
    /// The body was rejected by the JSON reader BEFORE this build's
    /// types were consulted — serde_json's `Syntax`/`Eof` classes: a
    /// syntax error, a truncated file, a non-finite token, a numeric
    /// literal outside `f64` (see [`parse_err`] for the executed edges).
    /// Position only, no recourse: these bytes are not a stale
    /// document, and "regenerate" is not the diagnosis. Position is
    /// serde_json's (1-based line/column into the BODY, i.e. after the
    /// header line).
    Parse {
        /// Line within the body.
        line: usize,
        /// Column within the line.
        column: usize,
        /// The parser's message.
        message: String,
    },
    /// The body passed the JSON reader and this build's TYPES rejected
    /// it — serde_json's `Data` class (the seam is stated once, on
    /// [`parse_err`]): a variant or field this build has no name for,
    /// a required field it lacks, a wrong type, a rebuild refusal.
    /// `detail` is the deserializer's own words and the offending name
    /// is in there. Both directions of growth meet this arm or none
    /// (module docs): an OLDER document that merely lacks vocabulary
    /// grown since does NOT land here — it loads; an OLDER document
    /// missing a field since made required lands here naming it; a
    /// NEWER document carrying a field this build lacks lands here
    /// naming it (`deny_unknown_fields` — a stale reader must not
    /// silently drop data). The recourse is [`REGENERATE_RECOURSE`].
    Unreadable {
        /// Line within the body (serde_json's 1-based position).
        line: usize,
        /// Column within the line.
        column: usize,
        /// The deserializer's message, naming the vocabulary it could
        /// not place.
        detail: String,
    },
    /// The snapshot violates a document invariant — a parsed one on
    /// load, or an in-memory one at save (which would have written an
    /// unloadable file; shared-validator check).
    Snapshot(SnapshotError),
    /// An edit in the log refused through the [`apply`] door — on
    /// LOAD replay, or at SAVE by the symmetric log-verification pass
    /// (a log that cannot replay would make an unloadable file; save
    /// refuses first).
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

/// The one recourse sentence a [`PersistError::Unreadable`] and a
/// [`PersistError::HeaderId`] end on — composed EXACTLY once per
/// message (the shared-recourse-carrier discipline, D4 ¶1 addendum).
/// Public so callers can assert on it without restating prose.
pub const REGENERATE_RECOURSE: &str = "regenerate the file from its source recipe with a current build \
     (every saved document replays from source; this kernel is \
     unreleased and writes no old-format files)";

impl core::fmt::Display for PersistError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFinite { site } => write!(f, "persist: non-finite float at {site}"),
            Self::ProfileProgram { node, fault } => write!(
                f,
                "persist: profile program fault at node {}: {fault}",
                node.0
            ),
            Self::Distribution { name, fault } => {
                write!(f, "persist: document parameter {:?}: {fault}", name.0)
            }
            Self::DisplayUnit {
                name,
                unit,
                declared,
            } => write!(
                f,
                "persist: document parameter {:?} is declared {declared:?} but its display \
                 unit measures {unit:?}",
                name.0
            ),
            Self::Serialize { message } => write!(f, "persist: serializer failed: {message}"),
            Self::HeaderId { found } => {
                write!(
                    f,
                    "persist: no `id: <32 lowercase hex>` header line (found: {found:?}) — \
                     {REGENERATE_RECOURSE}"
                )
            }
            Self::IdMismatch { header, snapshot } => write!(
                f,
                "persist: header id {header} disagrees with the snapshot's id {snapshot} — \
                 tampered or hand-assembled file"
            ),
            Self::Parse {
                line,
                column,
                message,
            } => write!(f, "persist: body line {line} column {column}: {message}"),
            Self::Unreadable {
                line,
                column,
                detail,
            } => write!(
                f,
                "persist: this build cannot read the document (body line {line} column \
                 {column}: {detail}) — {REGENERATE_RECOURSE}"
            ),
            Self::Snapshot(e) => write!(f, "persist: invalid snapshot: {e}"),
            Self::EditReplay { index, error } => {
                write!(f, "persist: edit {index} refused on replay: {error}")
            }
            Self::ToleranceConflict { process, document } => write!(
                f,
                "persist: document ε {document:e} conflicts with the process ε {process:e} \
                 (one process, one ε)"
            ),
            Self::ToleranceInvalid { value } => {
                write!(f, "persist: recorded ε {value:e} is not a valid tolerance")
            }
        }
    }
}

impl core::error::Error for PersistError {}

/// Serializes `snapshot` + `edits` into the current text format.
///
/// # Errors
///
/// Every arm of the shared validator (module docs — the same checks
/// load runs): [`PersistError::NonFinite`] naming the site of any
/// NaN/inf in the document or edit log (D2),
/// [`PersistError::ProfileProgram`], and
/// [`PersistError::Snapshot`] for a document whose structural
/// invariants are broken (an unloadable file, refused before it
/// exists). Plus [`PersistError::EditReplay`] for a log that cannot
/// replay, and [`PersistError::Serialize`] if the JSON writer itself
/// fails.
pub fn save(
    snapshot: &ProfileDoc,
    edits: &[DocEdit<ProfileProgram>],
    tol: Tol,
) -> Result<String, PersistError> {
    check::validate_document(snapshot, edits, tol)?;
    // Save/load symmetry for the LOG: load replays the edits through
    // apply's doors, so a log that refuses there must refuse HERE —
    // never a file that saves clean and cannot load. (Pure and
    // document-scale cheap; the replayed value is discarded.)
    let mut replay = snapshot.clone();
    for (index, edit) in edits.iter().enumerate() {
        replay = apply(&replay, edit, tol)
            .map_err(|error| PersistError::EditReplay { index, error })?
            .doc;
    }
    let body = SerBody { snapshot, edits };
    let json = serde_json::to_string_pretty(&body).map_err(|e| PersistError::Serialize {
        message: e.to_string(),
    })?;
    // The `id:` header line duplicates the snapshot's id (ASM-1 D-6)
    // so a workspace scan reads identity without parsing the body;
    // load verifies the two agree.
    Ok(format!("id: {}\n{json}\n", snapshot.id()))
}

/// The borrowing twin of [`FileBody`] (save side).
#[derive(serde::Serialize)]
struct SerBody<'a> {
    snapshot: &'a ProfileDoc,
    edits: &'a [DocEdit<ProfileProgram>],
}

/// Parses, validates, replays, and ε-reconciles a saved document. See
/// the module docs for the door sequence; every failure is a typed
/// [`PersistError`].
///
/// # Errors
///
/// Every arm of [`PersistError`] except `Serialize` (`NonFinite` is
/// guarded by the shared validator but unreachable post-parse — JSON
/// carries no non-finite tokens, so those bytes refuse as `Parse`).
pub fn load(text: &str, tol: Tol) -> Result<Loaded, PersistError> {
    // The header carries the document's id (ASM-1 D-6); it is parsed
    // before the body so a malformed header refuses in header terms,
    // then verified against the snapshot below.
    let (header_id, body_text) = parse_id_line(text)?;
    let body = parse_body(body_text)?;
    // Header/snapshot id agreement (ASM-1 D-6): the save door writes
    // the snapshot's own id, so disagreement is tampering, refused.
    if header_id != body.snapshot.id() {
        return Err(PersistError::IdMismatch {
            header: header_id,
            snapshot: body.snapshot.id(),
        });
    }
    // The ONE shared validator — the same call the save door makes
    // (convention 2): a parsed document passes exactly the checks an
    // in-memory document must pass to be saved.
    check::validate_document(&body.snapshot, &body.edits, tol)?;
    // Replay through apply's doors: the loaded current state is the
    // replayed state, never trusted bytes.
    let mut doc = body.snapshot.clone();
    let mut records = Vec::with_capacity(body.edits.len());
    for (index, edit) in body.edits.iter().enumerate() {
        let Applied {
            doc: next, record, ..
        } = apply(&doc, edit, tol).map_err(|error| PersistError::EditReplay { index, error })?;
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

/// Splits the `id: <32 lowercase hex>` header line from the body.
/// Canonical spelling ONLY: exactly `id: ` then exactly 32 lowercase
/// hex digits — no other whitespace, case or width. The write side
/// emits exactly this; any other spelling is a tampered or foreign
/// file and refuses typed.
fn parse_id_line(text: &str) -> Result<(DocumentId, &str), PersistError> {
    let (first, rest) = text.split_once('\n').unwrap_or((text, ""));
    let found = || first.chars().take(80).collect::<String>();
    let Some(id_text) = first.strip_prefix("id: ") else {
        return Err(PersistError::HeaderId { found: found() });
    };
    let Some(id) = DocumentId::parse_hex(id_text) else {
        return Err(PersistError::HeaderId { found: found() });
    };
    Ok((id, rest))
}

/// The document id named by a save's header line, WITHOUT parsing the
/// body — the workspace scan's cheap read (ASM-1 D-5/D-6). The same
/// door as [`load`]'s header phase.
///
/// # Errors
///
/// [`PersistError::HeaderId`].
pub fn header_document_id(text: &str) -> Result<DocumentId, PersistError> {
    let (id, _body) = parse_id_line(text)?;
    Ok(id)
}

fn parse_body(body_text: &str) -> Result<FileBody, PersistError> {
    serde_json::from_str(body_text).map_err(parse_err)
}

/// THE seam, stated once (the variant docs and the module header point
/// here): which arm a body refusal lands on is serde_json's own
/// classification of its failure — [`serde_json::error::Category`] —
/// and nothing else. No message is inspected.
///
/// `Data` → [`PersistError::Unreadable`] with the recourse: the JSON
/// reader accepted the bytes and this build's TYPES rejected them.
/// `Syntax` / `Eof` → [`PersistError::Parse`] without it: the reader
/// rejected the bytes before any type was consulted. `Io` cannot arise
/// from a `&str` source and is grouped with the reader's classes
/// rather than left to a wildcard.
///
/// The executed edges, so nobody has to guess where the line falls
/// (`tests/bool13_r1_probes.rs`, `tests/bool13r2_probes.rs`):
/// unknown variant, unknown field, missing field, duplicate field, a
/// wrong type at any depth, a body that is `null` / `5` / `[]` / a
/// string, a nesting bomb (the typed visitor fails at depth three
/// before the reader's recursion limit), and the crate's own rebuild
/// refusals (duplicate strict-map key, ill-dimensioned expression,
/// unknown display unit) are all `Data` → `Unreadable`. A syntax error,
/// truncation, an empty body, trailing bytes after the value, a `NaN`
/// or `Infinity` token, and a decimal literal outside `f64` (`1e999`,
/// "number out of range" — serde_json rejects it at the TOKEN, so it is
/// `Syntax` although the bytes are grammatical JSON) are all
/// → `Parse`. That last edge is the one place the two descriptions
/// "not JSON" and "reader-rejected" part company, and the reader's
/// class is the one this door follows.
fn parse_err(e: serde_json::Error) -> PersistError {
    use serde_json::error::Category;
    let (line, column, message) = (e.line(), e.column(), e.to_string());
    match e.classify() {
        Category::Data => PersistError::Unreadable {
            line,
            column,
            detail: message,
        },
        Category::Syntax | Category::Eof | Category::Io => PersistError::Parse {
            line,
            column,
            message,
        },
    }
}
