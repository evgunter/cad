//! Persistence, schema v5 (M4 PR 6's format; four ratified clean
//! breaks since — see [`SCHEMA_VERSION`]).
//!
//! # Schema history
//!
//! - **v1** (M4 PR 6) — the ratified text format below.
//! - **v2** (M5 PR 10) — the same text format carrying the grown node
//!   vocabulary (`Loft`, `Sweep`). Ratified as a **clean break**: no
//!   `migrate` step is written, v1 refuses typed
//!   ([`PersistError::SchemaTooOld`], naming [`REGENERATE_RECOURSE`]),
//!   and the repo's own v1 goldens were regenerated once, in that PR.
//!   The kernel is unreleased; the only v1 files that ever existed are
//!   the repo's, and every one of them replays from source.
//!
//!   **What the break does NOT do** (M5 PR 10 review NOTE): v2 changed
//!   the recipe VOCABULARY, not the wire format, so a v1 body is still
//!   valid v2 JSON — hand-edit a v1 file's header to `schema: 2` and
//!   it loads. That is inherent to a version break with no format
//!   change and no door can close it: the header is the only place the
//!   version is recorded, so an edited header IS a v2 file by
//!   definition. It costs nothing (a v1 body carries no construct v2
//!   rejects) and it is not a gap in the version door, which refuses
//!   every file that still SAYS v1. Pinned, executed, in
//!   `tests/review_m5_pr10_schema.rs`.
//!
//! # Format (spec D1)
//!
//! A save is TEXT: a `schema: <integer>` header line, an
//! `id: <32 lowercase hex>` line naming the document's identity (v5,
//! ASM-1 D-6 — the workspace scan reads it without parsing the
//! body), then a JSON body
//! `{ "snapshot": <Doc>, "edits": [<DocEdit>…] }` — the full document
//! snapshot plus the edit log since that snapshot. JSON via
//! `serde_json` is the ratified shape's PR-spec aesthetic choice
//! (REPORTED): floats serialize through ryu (shortest round-trip —
//! exactly D2's contract), parse errors carry line/column (D6.3's
//! typed position info), map keys with integer newtypes work natively,
//! and the format is universally diffable/greppable. Structural map
//! keys (the appearance store's [`crate::StableName`]s) serialize as pair
//! LISTS ([`pairs`]) — structural, not stringified.
//!
//! # What persists (spec D3)
//!
//! The recipe IS the save: the document id, nodes, parameters,
//! expressions, witness bytes (hex, bit-exact), the appearance store
//! (records incl. D7 metadata), recorded ε, the schema version, and
//! the edit log.
//! Deliberately NOT persisted: evaluations, name tables,
//! memo/content/naming keys, arena anything — and, since v4, the
//! profile programs' REPLAYED SEGMENTS (vertices/bulges/joints are
//! replay products of the stored programs; V3: caches live in the
//! evaluation memo, never on disk) — all of it re-derives on replay,
//! and the save/load/replay-identity CI row pins that the
//! re-derivation is bit-identical.
//!
//! # Doors (fail loud, D2/D6.3; DESIGN engineering convention 2)
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
//! - Load walks the header → migration chain → typed deserialize
//!   (expression REBUILD through the dimension-checking constructors
//!   and the wire-only canonical-set rule, [`wire`]) → the shared
//!   validator → edit replay through [`crate::edit::apply`]'s doors →
//!   ε reconciliation (D4). No silent best-effort loads, ever.
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
pub(crate) mod pairs;
pub(crate) mod strict;
mod wire;

use geom_core::tolerance::{Tolerance, ToleranceError};

use crate::edit::{Applied, DocEdit, EditError, EditRecord, apply};
use crate::ident::DocumentId;
use crate::program::{ProfileDoc, ProfileProgram};

pub use canon::{canonical_bytes, content_pin};
pub use check::{NonFiniteSite, ProgramFault, SnapshotError};

/// The current schema version.
///
/// Version 1 froze at M4 PR 6 (F8: the persisted file IS in M4).
/// Version 2 is M5 PR 10's **clean break** (spec §4, ratified by Evan
/// on #148): the recipe vocabulary grew [`crate::node::Node::Loft`]
/// and [`crate::node::Node::Sweep`], and rather than carry live
/// compatibility code for a format nobody outside this repo has ever
/// written, v1 refuses TYPED ([`PersistError::SchemaTooOld`]) with the
/// regenerate recourse. No `migrate` step exists for 1 → 2, on
/// purpose; the chain machinery ([`migration_step`]) stays, carrying
/// no steps.
///
/// Version 3 is M6-5's **clean break** on the same terms (ruled by
/// Evan on #217): [`crate::node::Node::Fillet`] grew a required
/// `selection` field. A v2 fillet meant "every edge", which the new
/// vocabulary cannot express as data — the equivalent selection
/// depends on an evaluation the file does not carry — so there is no
/// honest default to migrate to and none is invented. A v2 file
/// refuses TYPED with the regenerate recourse, exactly as v1 does.
///
/// Version 4 is the **profiles-as-programs clean break** (LIB-SWITCH
/// §4h; PROFILES-V2 ratified #242, LQ7a): `Node::Profile`'s payload
/// switched from the opaque vertex/bulge description to the
/// [`crate::ProfileProgram`] (Expr-bearing step lists; the program IS
/// the definition, derived segments are unpersisted replay products),
/// and expression literals gained the optional display-unit field
/// (U8b, §4g). No v3 form survives verbatim — the in-repo corpora
/// re-authored program-form — so v3 refuses TYPED with the regenerate
/// recourse, exactly as v1/v2 do; the migration table stays empty.
///
/// Version 5 is the **document-identity clean break** (ASM-1 spec
/// D-6, same ratified terms): [`crate::Doc`] gained the required
/// [`DocumentId`] field (ASSEMBLY-DESIGN A4 — identity ≠ pin), and
/// the text format gained the `id:` header line so a workspace scan
/// reads identity without parsing the body. Identity is AUTHORED data
/// with no honest default a migration could invent, so v4 refuses
/// TYPED with the regenerate recourse, exactly as v1–v3 do; the
/// migration table stays empty. (A future ASM-ROOTS root list takes
/// its own bump when it lands — noted, not decided here.)
///
/// Version 6 is that bump: the **product-roots clean break**
/// (ASM-ROOTS spec D-1, ASSEMBLY-DESIGN A10). [`crate::Doc`] gained
/// the ordered `roots` list, and a v5 file carries none. A migration
/// COULD compute today's sink set — but not its ORDER, which is
/// product-solid order and therefore semantic (it moves the content
/// pin), so the migrated document's product would be an invented
/// answer to a question the file never recorded. There is no honest
/// default; v5 refuses TYPED with the regenerate recourse, exactly as
/// v1–v4 do, and the migration table stays empty.
///
/// Version 7 is the **instantiate-part clean break** (ASM-2A spec
/// D-6, ASSEMBLY-DESIGN A2/A3/A11): [`crate::Node`] gained the
/// `InstantiatePart` variant and [`crate::Doc`] the A11 `placements`
/// registry. A v6 file carries neither, and neither has an honest
/// invented default — an absent registry is exactly the all-identity
/// state a v6 document already means, but the NODE variant is the
/// break: a v6 reader and a v7 reader disagree about what a document
/// can contain at all. v6 refuses TYPED with the regenerate recourse,
/// exactly as v1–v5 do, and the migration table stays empty.
///
/// Version 8 is **vocabulary growth** on the standing terms (LIB-LBRET;
/// PATHS-DESIGN §2b's LB10 route 3, ratified on #386): the chain step
/// vocabulary gained `ProgramStep::AtToward` (since retired by v9), the straight
/// fillet arrival off an arc-carrier departure. The addition is
/// forward-additive — a v7 file contains no `AtToward` and would load
/// — but the reverse is what the version gate is FOR: a v8 file handed
/// to a v7 reader must refuse at the gate, typed, instead of reaching
/// serde and dying on an unknown variant. That is the same call v2 and
/// v3 made for exactly this shape (new node vocabulary, new required
/// field), so a v7 file refuses TYPED with the regenerate recourse and
/// the migration table stays empty.
///
/// **7 and 8 were claimed twice, and this is the resolution.** ASM-2A
/// (#414) and LIB-LBRET (#413) each concluded 7 was theirs, because
/// each re-merged main before the other's bump landed. ASM-2A merged
/// first, so v7 is InstantiatePart and LBRET takes the later number —
/// two vocabulary changes never share one version, which is the whole
/// point of the gate. The reason it needed a human eye: the constant
/// is ONE LINE, so the second merge resolves it CLEANLY to the same
/// text while the two meanings silently collapse.
///
/// Version 9 is the **§2c fillet-family re-spell** (PATHS-DESIGN §2c,
/// ratified on #419; LIB-RESPELL PR-1): the chain step vocabulary
/// RE-SPELLED — `AtOn`/`AtToward`/`CloseToOn`/`ArcVia`/`ArcCenter`
/// retired, `ArcTo` re-shaped onto the one unified arc-spec record,
/// and the fused verbs (`FilletArc`/`ArcFillet`/`ArcFilletArc`)
/// added. A pre-release clean break, both directions: a v8 file's
/// retired variants would die in serde under today's types, and a v9
/// file's fused variants are unknown to a v8 reader — so v8 refuses
/// TYPED at the gate with the regenerate recourse (the v3 precedent),
/// and the migration table stays empty.
///
/// Bump ONLY with a ratified format change — plus its
/// [`migration_step`] entry, or a ratified break like these eight.
pub const SCHEMA_VERSION: u32 = 9;

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

/// Typed persistence refusal (spec D2/D6.3 — never a silent
/// best-effort load, never a stringly error at the API).
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
    /// The file has no parseable `id: <32 lowercase hex>` header line
    /// (required since v5 — the workspace scan reads identity from the
    /// header without parsing the body; canonical spelling only, like
    /// the schema line).
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
    /// The header names a schema this build does not know (D6.3:
    /// refuse typed; migrations only run FORWARD from older versions).
    UnknownSchema {
        /// The version in the file.
        found: u64,
        /// The newest version this build reads.
        newest: u32,
    },
    /// The header names an OLDER schema this build cannot reach: the
    /// migration chain has no step for `missing` (M5 PR 10 §4 — the
    /// 1 → 2 clean break deliberately writes none). Typed refusal, not
    /// a best-effort load; the recourse is to REGENERATE the file from
    /// its source recipe with a current build (every file this kernel
    /// has ever written replays from source).
    ///
    /// Version comparison is exact integer arithmetic, so this arm has
    /// no in-band twin — the two-tolerance discipline does not apply
    /// (spec §4, stated so the omission reads as a decision).
    SchemaTooOld {
        /// The version in the file.
        found: u32,
        /// The version this build reads and writes.
        supported: u32,
        /// The version whose forward migration step is missing
        /// (`found` for a single-step gap).
        missing: u32,
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

/// The one recourse sentence a [`PersistError::SchemaTooOld`] ends on
/// — composed EXACTLY once per message (the shared-recourse-carrier
/// discipline, D4 ¶1 addendum). Public so callers can assert on it
/// without restating prose.
pub const REGENERATE_RECOURSE: &str = "regenerate the file from its source recipe with a current build \
     (every saved document replays from source; this kernel is \
     unreleased and writes no old-format files)";

impl core::fmt::Display for PersistError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFinite { site } => write!(f, "persist: non-finite float at {site:?}"),
            Self::ProfileProgram { node, fault } => write!(
                f,
                "persist: profile program fault at node {node:?}: {fault:?}"
            ),
            Self::Serialize { message } => write!(f, "persist: serializer failed: {message}"),
            Self::Header { found } => {
                write!(
                    f,
                    "persist: no `schema: <integer>` header (first line: {found:?})"
                )
            }
            Self::HeaderId { found } => {
                write!(
                    f,
                    "persist: no `id: <32 lowercase hex>` header line (found: {found:?})"
                )
            }
            Self::IdMismatch { header, snapshot } => write!(
                f,
                "persist: header id {header} disagrees with the snapshot's id {snapshot} — \
                 tampered or hand-assembled file"
            ),
            Self::UnknownSchema { found, newest } => write!(
                f,
                "persist: schema v{found} is newer than this build reads (newest v{newest}) — \
                 migrations only run forward; use a newer build"
            ),
            Self::SchemaTooOld {
                found,
                supported,
                missing,
            } => write!(
                f,
                "persist: schema v{found} is older than this build reads (supported v{supported}) \
                 and no migration step exists from v{missing} — {REGENERATE_RECOURSE}"
            ),
            Self::Migration(e) => write!(
                f,
                "persist: migration from schema v{} failed: {}",
                e.from, e.reason
            ),
            Self::Parse {
                line,
                column,
                message,
            } => write!(f, "persist: body line {line} column {column}: {message}"),
            Self::Snapshot(e) => write!(f, "persist: invalid snapshot: {e:?}"),
            Self::EditReplay { index, error } => {
                write!(f, "persist: edit {index} refused on replay: {error:?}")
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

/// A failed or unavailable migration step (spec D1: migrations are
/// explicit version-to-version functions from v1 onward).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationError {
    /// The version the failing step was migrating FROM.
    pub from: u32,
    /// Why.
    pub reason: String,
}

/// A migration step: `from_version` → `from_version + 1`, on the raw
/// JSON body (spec D1's ratified mechanism).
pub type MigrationStep = fn(serde_json::Value) -> Result<serde_json::Value, MigrationError>;

/// The forward migration chain's step table (spec D1: migrations are
/// explicit version-to-version functions, and they only run FORWARD —
/// D6.3). The loader walks this from the file's version up to
/// [`SCHEMA_VERSION`], then deserializes typed.
///
/// `None` means NO step exists for that version — the loader turns
/// that into [`PersistError::SchemaTooOld`] BEFORE it touches the
/// body, so a too-old file's diagnostics name the version problem
/// rather than whatever the stale body happens to parse as.
///
/// **The table is empty, on purpose**: 1 → 2 (M5 PR 10 §4), 2 → 3
/// (M6-5, ruled #217), 3 → 4 (LIB-SWITCH §4h — profiles as programs,
/// ratified LQ7a clean break), 4 → 5 (ASM-1 D-6 — document identity)
/// 5 → 6 (ASM-ROOTS D-1 — product roots) and 6 → 7 (ASM-2A D-6 —
/// the instantiate node + A11 placements) were all ratified clean
/// breaks. The mechanism stays because it costs nothing and D6.3's
/// forward-only rule is unchanged; a future format change that is NOT
/// a break adds its `n => Some(step_n)` arm here.
fn migration_step(from_version: u32) -> Option<MigrationStep> {
    /// `(from_version, step)` pairs — the whole chain, one line each.
    const TABLE: &[(u32, MigrationStep)] = &[];
    TABLE
        .iter()
        .find(|(from, _)| *from == from_version)
        .map(|(_, step)| *step)
}

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
) -> Result<String, PersistError> {
    check::validate_document(snapshot, edits)?;
    // Save/load symmetry for the LOG: load replays the edits through
    // apply's doors, so a log that refuses there must refuse HERE —
    // never a file that saves clean and cannot load. (Pure and
    // document-scale cheap; the replayed value is discarded.)
    let mut replay = snapshot.clone();
    for (index, edit) in edits.iter().enumerate() {
        replay = apply(&replay, edit)
            .map_err(|error| PersistError::EditReplay { index, error })?
            .doc;
    }
    drop(replay);
    let body = SerBody { snapshot, edits };
    let json = serde_json::to_string_pretty(&body).map_err(|e| PersistError::Serialize {
        message: e.to_string(),
    })?;
    // The `id:` header line duplicates the snapshot's id (ASM-1 D-6)
    // so a workspace scan reads identity without parsing the body;
    // load verifies the two agree.
    Ok(format!(
        "schema: {SCHEMA_VERSION}\nid: {}\n{json}\n",
        snapshot.id()
    ))
}

/// The borrowing twin of [`FileBody`] (save side).
#[derive(serde::Serialize)]
struct SerBody<'a> {
    snapshot: &'a ProfileDoc,
    edits: &'a [DocEdit<ProfileProgram>],
}

/// Parses, migrates, validates, replays, and ε-reconciles a saved
/// document. See the module docs for the door sequence; every failure
/// is a typed [`PersistError`].
///
/// # Errors
///
/// Every arm of [`PersistError`] except `Serialize` (`NonFinite` is
/// guarded by the shared validator but unreachable post-parse — JSON
/// carries no non-finite tokens, so those bytes refuse as `Parse`).
pub fn load(text: &str) -> Result<Loaded, PersistError> {
    let (version, rest) = parse_header(text)?;
    // Migration chain (D1): walk explicit steps up to the current
    // version, then deserialize typed.
    let (header_id, body): (Option<DocumentId>, FileBody) = if version == SCHEMA_VERSION {
        // The v5 header carries the document's id (ASM-1 D-6); parse
        // it before the body so a malformed header refuses in header
        // terms, then verify it against the snapshot below.
        let (id, body_text) = parse_id_line(rest)?;
        (Some(id), parse_body(body_text)?)
    } else {
        let body_text = rest;
        // Walk the chain for AVAILABILITY first, before a byte of the
        // body is parsed: a file this build cannot reach must say so
        // in version terms (§4's clean break), not report whatever the
        // old body's JSON looks like under today's types.
        let steps: Vec<MigrationStep> = (version..SCHEMA_VERSION)
            .map(|at| {
                migration_step(at).ok_or(PersistError::SchemaTooOld {
                    found: version,
                    supported: SCHEMA_VERSION,
                    missing: at,
                })
            })
            .collect::<Result<_, _>>()?;
        let mut value: serde_json::Value = serde_json::from_str(body_text).map_err(parse_err)?;
        for step in steps {
            value = step(value).map_err(PersistError::Migration)?;
        }
        (None, serde_json::from_value(value).map_err(parse_err)?)
    };
    // Header/snapshot id agreement (ASM-1 D-6): the save door writes
    // the snapshot's own id, so disagreement is tampering, refused.
    if let Some(header) = header_id
        && header != body.snapshot.id()
    {
        return Err(PersistError::IdMismatch {
            header,
            snapshot: body.snapshot.id(),
        });
    }
    // The ONE shared validator — the same call the save door makes
    // (convention 2): a parsed document passes exactly the checks an
    // in-memory document must pass to be saved.
    check::validate_document(&body.snapshot, &body.edits)?;
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
    // Canonical spelling ONLY (review NOTE-5): exactly "schema: "
    // then plain decimal digits — no signs, no extra whitespace, no
    // leading zeros. The write side emits exactly this; any other
    // spelling is a tampered or foreign file and refuses typed.
    let Some(version_text) = first.strip_prefix("schema: ") else {
        return Err(PersistError::Header { found: found() });
    };
    let canonical_digits = !version_text.is_empty()
        && version_text.bytes().all(|b| b.is_ascii_digit())
        && (version_text == "0" || !version_text.starts_with('0'));
    if !canonical_digits {
        return Err(PersistError::Header { found: found() });
    }
    let Ok(version) = version_text.parse::<u64>() else {
        // Only reachable on > u64::MAX digit strings.
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

/// Splits the `id: <32 lowercase hex>` line (v5's second header line)
/// from the body. Canonical spelling ONLY, same discipline as the
/// schema line: exactly `id: ` then exactly 32 lowercase hex digits.
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

/// The document id named by a save's header lines, WITHOUT parsing
/// the body — the workspace scan's cheap read (ASM-1 D-5/D-6). Walks
/// the same doors as [`load`]'s header phase: version window, then
/// the v5 `id:` line; an older schema refuses [`PersistError::SchemaTooOld`]
/// exactly as a full load would (the migration table is empty).
///
/// # Errors
///
/// [`PersistError::Header`], [`PersistError::UnknownSchema`],
/// [`PersistError::SchemaTooOld`], [`PersistError::HeaderId`].
pub fn header_document_id(text: &str) -> Result<DocumentId, PersistError> {
    let (version, rest) = parse_header(text)?;
    if version != SCHEMA_VERSION {
        // Pre-v5 headers carry no id line; the file would refuse at
        // load for the same reason (empty migration table).
        return Err(PersistError::SchemaTooOld {
            found: version,
            supported: SCHEMA_VERSION,
            missing: version,
        });
    }
    let (id, _body) = parse_id_line(rest)?;
    Ok(id)
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
