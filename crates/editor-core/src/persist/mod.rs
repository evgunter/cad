//! Persistence: the ratified text format of M4 PR 6, at the schema
//! version [`SCHEMA_VERSION`] names. Every bump since v1 is a ratified
//! CLEAN BREAK — an older file refuses typed, naming
//! [`REGENERATE_RECOURSE`], and the migration table stays empty
//! because LQ7a rules it so ([`migration_step`]), not because no one
//! has filled it in yet.
//!
//! Every version from v2 on carries an entry on [`SCHEMA_VERSION`],
//! enforced by `tests/schema_ledger.rs` rather than left to
//! discipline: the version is ONE LINE, so two units
//! claiming the same number merge CLEANLY, and those entries are where
//! their reasoning can be compared — and equally where one of them can
//! be dropped to resolve a conflict.
//!
//! # Schema history
//!
//! v1 and v2 only, because the format TEXT below is theirs; every
//! version's own entry is on [`SCHEMA_VERSION`].
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
/// Version 10 is **edit vocabulary growth** (ASSEMBLY-DESIGN A13,
/// ratified #544; ASM-UPD D-1): [`crate::DocEdit`] gained the
/// `UpdateReference` arm, the recorded per-reference pin move. The
/// edit log is FILE data — a saved document carries its unreplayed
/// edits — so a new arm is a new wire shape, exactly the case v8
/// bumped for one level over in the vocabulary. Forward-additive
/// again (a v9 file contains no `UpdateReference`), and again the
/// gate buys the other direction: a v10 file handed to a v9 reader
/// must refuse at the version door rather than reach serde and die on
/// an unknown variant. A v9 file refuses TYPED with the regenerate
/// recourse and the migration table stays empty.
///
/// **9 was claimed twice, and this is the resolution** — the v7/v8
/// collision again, and caught the same way. LIB-RESPELL (#531) and
/// ASM-UPD (#549) each concluded 9 was theirs, each having re-merged
/// main before the other's bump landed. RESPELL merged first, so v9
/// is the re-spell and the `UpdateReference` arm takes 10. What made
/// it visible this time rather than silent: the pattern is now in
/// this ledger, so ASM-UPD flagged the hazard in its own PR body
/// BEFORE the collision fired and re-checked the constant by eye at
/// the merge — which is the discipline the v7/v8 entry asks for.
///
///
/// **The persistence boundary for contact data, stated once** (C4,
/// D9; the seam ASM-R2-SPEC-DRAFT:41-58 negotiates). DECLARATIONS
/// persist — they are recipe data on the consuming node, exactly like
/// any other authored payload, and that is what this version's break
/// is about. RECORDS never persist: a body's verified contact records
/// are re-derived by replay (D9), so nothing here writes a
/// `ContactRecords`. ASM-4's interface record stores DECLARATIONS for
/// the same reason — crossing declarations ARE the seam, so the split
/// populates them and the re-verification gate re-checks them against
/// solved geometry; it does not store the records that gate produces.
///
/// Version 11 is the **declaration-class clean break** (M9-1 spec
/// PR-2; CONTACT-DESIGN C4, ratified #178): [`crate::Node::Declare`]'s
/// pairs each gained the [`topo::ContactClass`] they assert, so a
/// declaration now says WHAT kind of contact it claims instead of
/// leaving the consuming boolean to assume the conformal one. With
/// `deny_unknown_fields` and a changed tuple arity, a v10 pair is not
/// a v11 pair at the wire, either direction.
///
/// A migration COULD write `rest` into every v10 pair — that is what
/// they meant — but it would be inventing the one datum the break
/// exists to stop being assumed, and it would do so silently on files
/// whose author never made the choice. C4's invariant is that no path
/// exists from "the numbers look equal" to a glued contact without a
/// structural or declared rung; a migration that authors the rung on
/// the user's behalf is that path with extra steps. So v10 and below
/// refuse TYPED with the regenerate recourse, exactly as v1–v9 do, and
/// the migration table stays empty. v10 is the version a real document
/// in this lineage can now carry, so it is the fixture the refusal
/// suite pins.
///
/// **Why 11: the race, run twice, resolved consciously both times.**
/// This unit claimed 10 at dispatch because LIB-RESPELL (#531) was
/// OPEN holding 9 — claim PAST the open holders rather than race them,
/// the standing resolution of the 7/8 double-claim above. Then #531
/// merged with 9, and ASM-UPD (#549) merged with 10 while this branch
/// was still open, so the claim moved again, to 11.
///
/// Both shifts cost one conscious resolve each and nothing else, which
/// is the whole argument for the discipline. Note what the SECOND
/// re-merge did on its own: the constant is one line and both sides
/// had already written `10`, so git merged it CLEANLY — the paragraph
/// conflict above is the only thing that stopped two breaks sharing a
/// version in silence. That is the 7/8 failure mode reproduced
/// exactly, and caught only because the ledger prose is long enough to
/// collide. All three meanings survive here: 9 the fillet-family
/// re-spell, 10 the `UpdateReference` arm, 11 the declaration class.
///
/// Version 12 is the **group boolean's vocabulary** (GROUP-BOOLEAN-
/// DESIGN, ratified A′; LIB-PLACEDUNION): the node vocabulary gained
/// `Node::PlacedUnion` — one prototype, a placement rule, ONE fused
/// body out — and the rule vocabulary gained `PatternKind::Explicit`,
/// a listed set of absolute frames. ONE vocabulary change, one version
/// (the one-meaning-per-version rule): the node kind and the rule kind
/// ship together because neither is expressible without the other at
/// the die tour's twenty-one-pip site that motivated both.
///
/// A pre-release clean break, both directions, on the v3/v9 precedent:
/// a v12 file's new variants are unknown to a v11 reader, and this
/// reader has no v11-shaped meaning to migrate from, so v11 and below
/// refuse TYPED with the regenerate recourse and the migration table
/// stays empty.
///
/// Version 13 is **node vocabulary growth** (ASSEMBLY-DESIGN
/// A3/A12, ratified #522; ASM-R2a D-1): [`crate::Node`] gained the
/// `Mate` variant — the mate's two instance-qualified references, its
/// declared [`topo::ContactClass`], and its alignment datum, all file
/// data. A new node arm is the case v7 bumped for, and v2/v3/v8 before
/// it: forward-additive (a v12 file contains no `Mate`), while the
/// gate buys the direction that fails badly — a v13 file handed to a
/// v12 reader must refuse at the version door rather than reach serde
/// and die on an unknown variant. A v12 file refuses TYPED with the
/// regenerate recourse and the migration table stays empty.
///
/// The mate's class rides the SAME stable spellings v11 gave
/// `Declare`'s pairs (`declare_pairs_wire`'s table, reused rather
/// than re-spelled): one contact vocabulary, one wire spelling of it.
/// A spelling this build has no name for refuses at that door, in both
/// directions. That is a WIRE refusal about a tag, not the v1 class
/// policy — how far an admitted class then gets is
/// [`crate::mate::class_admission`].
///
/// The A11 placement registry did NOT force this bump, and that is
/// worth stating: its keys generalized from per-instance to
/// per-cluster-REPRESENTATIVE, a change of MEANING with no change of
/// shape — a mate-less document's registry is the same map with the
/// same keys, because every singleton cluster's gauge is its own
/// instance. The bump is the node arm's alone.
///
/// **Why 13: the same-number race, run TWICE on one branch, and what
/// it costs.** This unit claimed 11 at its own bump, moved to 12 when
/// M9-1 PR-2 merged with 11, and moved to 13 when LIB-PLACEDUNION
/// merged with 12. BOTH shifts were caught the same way and only that
/// way: an explicit read of main's constant at the re-merge
/// (`git show origin/main:crates/editor-core/src/persist/mod.rs |
/// grep SCHEMA_VERSION`). Neither produced a merge conflict on the
/// constant — both sides had written the identical line, so git
/// merged it silently, exactly as the v11 and v12 entries above
/// predicted. Three consecutive units have now reproduced that
/// failure mode. The claim lives as prose in `docs/MODEL-AB-LOG.md`,
/// where it collides, and the number was re-read by eye at the
/// re-merge.
/// A gap in the sequence would cost nothing; a collision costs a
/// human eye.
///
/// Version 14 is the **interface record inhabited** (ASM-R2b D-4,
/// discharging the obligation ASM-4 wrote down at
/// [`crate::InterfaceCrossing`]): the enum that was UNINHABITED — so
/// that every [`crate::InterfaceRecord`] was provably empty, absent
/// from the wire, and fed no content key — gained its
/// `Mate { mate, class, outer, inner }` variant. A split that a mate
/// crosses now writes a non-empty record onto the remainder's
/// instantiate node, and that record is file data.
///
/// The claim reasoning, stated because a schema number is the one
/// thing in this repo that two units can silently agree on: this is a
/// FORMAT change, not merely a new value of an existing field. Before
/// v14 the instantiate node's `interface` key could not appear on the
/// wire at all (no `InterfaceCrossing` value exists to put in it), so
/// a v13 reader handed a v14 file with a populated record reaches
/// serde and dies on an unknown shape — exactly the direction the
/// version gate exists to fail cleanly. Forward-additive as ever (a
/// v13 file has no crossings), and the migration table stays empty:
/// a v13 file refuses TYPED with the regenerate recourse.
///
/// The record's `class` rides the SAME `kernel_wire` spelling v11 gave
/// `Declare`'s pairs and v13 gave `Node::Mate` — one contact
/// vocabulary, one wire spelling of it, third consumer, still not
/// re-spelled.
///
/// This number was taken by an explicit by-eye read of main's
/// constant at the final re-merge, and the claim also lives as prose
/// in `docs/MODEL-AB-LOG.md`, where a second claimant collides
/// instead of merging clean.
///
/// Version 15 is **distributions in the document** (ERROR-DESIGN
/// E1/E2, ratified; M10-1): [`crate::DocParam::Continuous`] gained an
/// optional [`crate::Distribution`] — the parameter's uncertainty as
/// offsets from its own nominal, in its own dimension. Document
/// metadata read only by [`crate::analysis`]; it enters no evaluation
/// and no content key.
///
/// The claim reasoning, stated because a schema number is the one
/// thing in this repo that two units can silently agree on: the field
/// is `skip_serializing_if = "Option::is_none"`, so a document that
/// declares no distribution writes the v14 bytes exactly — that is the
/// DEGENERATE CARRY, not the format claim. The format claim is the
/// populated key: a v14 reader handed a param carrying
/// `"distribution"` meets a field its `deny_unknown_fields` document
/// types have no name for and dies inside serde rather than at the
/// version door, which is exactly the direction the gate exists to
/// fail cleanly. Forward-additive as ever (a v14 file declares no
/// distributions), and the migration table stays empty: a v14 file
/// refuses TYPED with the regenerate recourse.
///
/// [`crate::DocParam::Count`] parameters gained nothing, deliberately:
/// structural parameters are fixed under any error analysis, which
/// comes out unrepresentable rather than as a refusal.
///
/// **The same break also carries a new edit arm**, and it rides here
/// rather than taking a number of its own because it is part of the
/// same change: an optional annotation is worth nothing if the
/// ordinary way to move a parameter's value deletes it.
/// [`crate::DocEdit::SetDocParamValue`] writes a new value into an
/// already-declared parameter and carries the declaration — dimension
/// AND distribution — forward, where [`crate::DocEdit::SetDocParam`]
/// is create-or-replace and a caller who rebuilt a `DocParam` from
/// `(dim, value)` silently dropped the annotation. The edit log is
/// file data, so a v14 reader handed a log containing the new arm
/// meets a variant its `deny_unknown_fields` edit type has no name for
/// and dies inside serde — the same direction, failed the same way, by
/// the same gate. A v15 file whose log contains no value edits is the
/// degenerate carry, exactly as an all-`None` distribution is.
///
/// Version 16 is **the chamfer recipe node** (RECIPE-DOORS D2,
/// issue #918).
///
/// The fifteenth break, and the node vocabulary's own kind of break:
/// [`crate::Node`] gains a `Chamfer` variant (RECIPE-DOORS D2, issue
/// #918) carrying `{ target, distance, selection }`, and [`crate::SlotId`]
/// gains `ChamferDistance` for its size. Both types are
/// `deny_unknown_fields`, so the direction that fails is the usual
/// one: a v15 reader handed a file containing a chamfer node — or a
/// slot binding naming its distance — meets a variant it has no name
/// for and dies inside serde. A v16 file with no chamfer in it is the
/// degenerate carry.
///
/// The recourse is the standing one for a vocabulary break with no
/// migration machinery (LQ7a): regenerate the file from its own
/// recipe. Nothing in the wire shape of any existing node moved, so a
/// file that never mentions a chamfer differs from its v15 self only
/// in the header number.
///
/// This number was taken by an explicit by-eye read of main's constant
/// at the final re-merge (`git show
/// origin/main:crates/editor-core/src/persist/mod.rs | grep
/// SCHEMA_VERSION`), the only thing that has ever caught the
/// same-number race, and the claim also lives as prose in
/// `docs/MODEL-AB-LOG.md`, where a second claimant collides instead of
/// merging clean.
///
/// Version 17 is **the measurement vocabulary** (ERROR-DESIGN E3/E10,
/// CONTACT-DESIGN C5; M10-2): [`crate::Node`] gained TWO variants —
/// `Measure`, carrying a measured expression over a frozen
/// [`crate::names::StableName`] reference list, and `Assertion`,
/// carrying a measure's node id, a bound expression and a direction.
/// Both are file data, and the measured expression is a NEW wire
/// vocabulary (`WireMeasureExpr`) beside the existing one.
///
/// The claim reasoning, stated because a schema number is the one
/// thing in this repo that two units can silently agree on: this is
/// the case v7, v13 and v2/v3/v8 bumped for — a new node arm.
/// Forward-additive (a v16 file contains no measure), while the gate
/// buys the direction that fails badly: a v17 file handed to a v16
/// reader meets a variant its `deny_unknown_fields` node enum has no
/// name for and dies inside serde rather than at the version door.
/// The migration table stays empty and a v16 file refuses TYPED with
/// the regenerate recourse.
///
/// A document with neither node writes the v16 bytes exactly — the
/// degenerate carry, not the format claim, exactly as v15's all-`None`
/// distributions were.
///
/// Taken by the same by-eye read of main's constant at the re-merge
/// that every entry above describes, and not re-described here.
/// What IS specific to this number: **it moved, and this is the
/// record of the collision resolving.** This unit claimed 16 and said
/// in its own ledger entry that LIB-G16's `Node::Chamfer` claimed it
/// too, and that the rule is order of merge. LIB-G16 merged first
/// (`a0427344`) and kept 16, so this unit took 17 and repaired what
/// the rule says it owes: this entry, the `assert_eq!(SCHEMA_VERSION,
/// ..)` rows, the golden filename, and the `plate_param`,
/// bench-corpus and `gallery_ring` fixtures. The by-eye read at the
/// re-merge is what caught it — the constant merged CLEAN at 16
/// against 16, exactly as every entry above predicts.
///
/// Version 18 is **the rim-support name vocabulary re-spelled onto
/// the carve's ROLES** (issue #961): [`crate::names::RimSupport`]'s
/// variants are `Host`/`Mate` where they were `Plane`/`Curved`, so
/// the persisted [`crate::RoleSeg::BandTrim`] segment names WHICH
/// SUPPORT a band trimline lies on by the role the annulus surgery
/// resolved rather than by a kind it guessed. A rim between two cones
/// has no planar side and no distinguishing kind at all; the roles are
/// what the surgery decides and what survives a parameter edit.
///
/// A stable name is FILE data — a node's frozen selection is recipe,
/// and a selection can name a band trimline — so the spelling is on
/// the wire, and the break is not additive in EITHER direction, which
/// is why it takes a number rather than riding one: a v17 file whose
/// selection spells `"Plane"` meets a `deny_unknown_fields` role enum
/// with no such variant and dies inside serde, and a v18 file's
/// `"Host"` dies the same way in a v17 reader. So v17 and below refuse
/// TYPED at the version door with the regenerate recourse and the
/// migration table stays empty, on the standing LQ7a disposition.
///
/// (The appearance store is NOT the carrier: its door restricts
/// attributes to face and body names, and a band trimline is an edge.)
///
/// A migration COULD rewrite `Plane` to `Host` and `Curved` to `Mate`
/// — the mapping is total and meaning-preserving, because `Host` is
/// the planar support wherever a rim has one, which is exactly what
/// the old spelling claimed on every rim it could name honestly. What
/// stops it being written is the standing rule, not the mapping: no
/// migration machinery exists, the kernel is unreleased, and every
/// file in this lineage replays from its own recipe. The mapping is
/// recorded here because it is the CONTENT of the break — a v17
/// document's rim selections mean the same thing under v18, spelled
/// differently — and `blend5_schema_v18.rs` executes both halves.
///
/// Taken by the same by-eye read of main's constant at the re-merge
/// (`git show origin/main:crates/editor-core/src/persist/mod.rs | grep
/// SCHEMA_VERSION`) that every entry above describes.
///
/// Bump ONLY with a ratified format change — plus its
/// [`migration_step`] entry, or a ratified break like these seventeen.
pub const SCHEMA_VERSION: u32 = 18;

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
            Self::NonFinite { site } => write!(f, "persist: non-finite float at {site}"),
            Self::ProfileProgram { node, fault } => write!(
                f,
                "persist: profile program fault at node {}: {fault}",
                node.0
            ),
            Self::Distribution { name, fault } => {
                write!(f, "persist: document parameter {:?}: {fault}", name.0)
            }
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

/// A failed or unavailable migration step (spec D1: migrations are
/// explicit version-to-version functions from v1 onward).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationError {
    /// The version the failing step was migrating FROM.
    pub from: u32,
    /// Why.
    pub reason: String,
}

// The human-readable rendering (LIB-DOORS F6 shape): the step names
// the version it was migrating FROM and forwards the step's own
// reason. The chain runs forward only and a step is written for one
// version pair, so which pair failed is the fact a reader needs; the
// reason is the step's words and is not re-stated here.
impl core::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { from, reason } = self;
        write!(
            f,
            "persist: the migration step from schema version {from} to {} failed: {reason}",
            from + 1
        )
    }
}

impl core::error::Error for MigrationError {}

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
/// **The table is empty by RULING, not by omission.** LQ7a
/// (`docs/LIBRARY-DESIGN.md`) bans backwards-compatibility machinery
/// of any kind before release — no migration chains, no deprecation
/// shims — so every bump is a clean break and none of them writes a
/// step: 1 → 2, 2 → 3 (ruled #217), 3 → 4 (profiles as programs),
/// 4 → 5 (document identity), 5 → 6 (product roots), 6 → 7 (the
/// instantiate node + the placements registry), 7 → 8 (the `AtToward`
/// chain step), 8 → 9 (the fillet-family re-spell), 9 → 10 (the
/// `UpdateReference` edit arm), 10 → 11 (the declaration class),
/// 11 → 12 (the group boolean's vocabulary), 12 → 13 (`Node::Mate`)
/// and 13 → 14 (the inhabited interface record).
///
/// The MECHANISM stays by the same ruling's other half: D6.3's
/// forward-only rule is unchanged, and the first post-release format
/// change that is not a break adds its `n => Some(step_n)` arm here.
/// An empty table is the ratified state of this door, not scaffolding
/// waiting to be finished.
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
pub fn load(text: &str, tol: Tol) -> Result<Loaded, PersistError> {
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
