# BOOL-13 — the schema demolition: no pre-release schema version

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **S/M, recorded numeric M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The ruling is the primary specification — Evan, in-chat,
2026-09-01, option C in its strong form, ratified at PR #1540:
`docs/DESIGN.md`'s Band-4 roadmap line and the BOOL-13 slate entry
in `docs/S-BOOL-PLAN.md`. Read both verbatim; they bound the unit.

## Situation

`crates/editor-core/src/persist` carries a hand-maintained
`SCHEMA_VERSION` (20), a version door that refuses older files
`SchemaTooOld` with the regenerate recourse, a migration chain that
is ratified EMPTY, thirteen per-version break-fixture test files
each pinning `SCHEMA_VERSION == 20` and a checked-in old-version
document, and a bump-coordination convention. Nothing is released
and no document exists outside this repo; every checked-in document
is a regenerable artifact. The version machinery exists to reassure
agents that format changes are tracked — it protects nothing. It
goes.

## Deliverables

1. **The door that stays, verified FIRST and reported before the
   build**: a file this build cannot read must refuse TYPED with the
   regenerate recourse, carried by the deserializer's own rejection
   of unknown or missing vocabulary. Verify that `serde_json`'s
   rejection of an unknown enum variant and of a missing required
   field is typed and NAME-CARRYING through this crate's error
   plumbing (the variant / field name reaches the message). If it
   is, build on it; if it is not, STOP and report — the fallback is
   the one-tag-plus-one-generic-row shape, and Evan picks.
2. **The demolition**: `SCHEMA_VERSION`, the `schema: N` header line
   and its parser, the version door and `PersistError::SchemaTooOld`,
   `MigrationStep` / `migration_step` / the empty table, the thirteen
   `*schema_v*.rs` test files with their checked-in fixtures and
   their `schema_version_is_current` pins, and every prose site
   describing the bump convention (the persist module header, the
   `D6.3` citations, `docs/DISCIPLINES-DESIGN.md` if it names it).
   The `id:` header line stays (document identity is ASM-1's, not
   versioning's). Every consumer swept — at minimum:
   `persist/{mod,wire}.rs`, `node.rs`, `lib.rs`, `pncad`'s
   `document.rs`/`workspace.rs` and tests, `profile/src/lift.rs`,
   the editor-core probe and pin suites that name the constant, and
   the Python surface census if it lists `SchemaTooOld`.
3. **The one refusal**: one `PersistError` variant (name yours —
   "unreadable by this build" is the fact) wrapping the
   deserializer's typed failure with the offending name and
   `REGENERATE_RECOURSE`. ONE generic row family replaces thirteen
   files: an unknown variant refuses naming it; a missing
   now-required field refuses naming it; and — the property the
   ruling is FOR — an older-shaped document that lacks a
   newly-grown variant LOADS (additive growth invalidates nothing).
   Red-first each.
4. **The persist module header rewritten**, saying in as many words
   (Evan's ask): schema breaks are not at all a problem because this
   is not released yet; no document exists outside the repo; every
   checked-in document is a regenerable artifact; a format change
   needs no version, no migration and no coordination — regenerate
   the corpus and move on; versioning returns as Band-4 work the day
   a document ships to someone (DESIGN.md's roadmap line).
5. **Corpus regeneration**, once — the header format changes, so
   every checked-in `.pncad` (the tour document, the bench corpus,
   the pncad plate, the viewer gallery) regenerates through the
   release tour build; the `.v20` fixture names lose their version
   suffix. This is the LAST such regeneration a format change forces
   on anyone.
6. **D9 / behavior**: no geometry moves — state it; the document
   round-trip suites and the corpus load rows are the gate.
7. **Class sweep** (discipline §5): every other site that pins a
   version number or a bump convention in `crates/` — dispositioned
   (in scope: persistence; NOT in scope: the content-key tag space in
   `editor-core/eval/mod.rs`, which is a different mechanism with a
   different reason, record and leave).

## Coordination (orchestrator's, stated so you plan for it)

Schema is contended ground — S-MATE and S-SEAT bumped versions
today. Report READINESS (green head, corpus regenerated) and STOP;
the orchestrator announces on the away channel and merges after a
short window. Expect one conflict against any in-flight bump; the
resolution is always "the version is gone."

## Acceptance

- The one refusal red-first three ways incl. the additive-growth
  LOAD row; the thirteen files gone; the corpus regenerated and
  loading; hosted CI green; gate record per head; the readiness
  report.

## Hard rules

- NO `Co-Authored-By`, no model names. No closing keywords; no issue
  closes here (the ruling has no issue).
- Scope fence: `crates/editor-core/src/persist/**`, the schema test
  files and the consumers named in deliverable 2, the persist
  header prose and its citations, the regenerated corpus files.
  NOT: the `Verb`/`ProgramStep`/`WireStep` VOCABULARY itself
  (BOOL-12 grows it), the content-key hashers, `docs/MODEL-AB-LOG.md`
  / `docs/S-BOOL-*.md` / SMELL edits. Track V's `crates/editor-core`
  fence: `persist/` is not among its rows' subjects — if a row's
  file is reached, disclose it.
- Re-merge main before opening the PR.
