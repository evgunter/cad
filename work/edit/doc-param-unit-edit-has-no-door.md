---
id: doc-param-unit-edit-has-no-door
kind: unit
title: No editor-core door changes a document parameter's display unit — SetDocParam would drop the distribution
status: spec
opened: 2026-09-04
refs: [1776]
branch: edit/doc-param-unit
---


## Finding

**Nothing in `editor-core`'s edit vocabulary can change a document
parameter's display unit, so no panel can offer a unit picker on a
parameter row.** `git grep SetDocParamUnit` matches only prose.

`DocEdit` carries exactly two parameter doors and neither is
unit-only:

- `SetDocParam` (`crates/editor-core/src/edit.rs:78-84`) is
  create-or-replace. It takes a whole `DocParam`, so a caller changing
  only the notation has to reassemble the declaration — and the
  natural spelling, `DocParam::continuous(dim, value)`
  (`crates/editor-core/src/doc.rs:176`), silently drops any
  `Distribution` the parameter carried. That is the exact trap the
  binding census's `B-DISTRIBUTIONS` charter documents and the reason
  `SetDocParamValue` exists at all (`edit.rs:89-100`).
- `SetDocParamValue` (`edit.rs:101-107`) writes a number into a
  standing declaration and says in its own rustdoc that the unit
  beside it is deliberately not its business
  (`crates/editor-core/src/doc.rs:56-61`).

The slot trick does not transfer. A slot's whole state is one `Expr`,
so `SessionOp::SetSlotUnit` can rebuild the literal and lose nothing
(`crates/viewer/src/props.rs`'s `slot_unit_edit`). A parameter's unit
sits beside `distribution` on the DECLARATION
(`crates/editor-core/src/doc.rs:39-80`), so the same rebuild through
create-or-replace must carry the annotation by hand, and no authoring
door can express that pairing. The only spelling that can is the raw
`DocParam::Continuous { .. }` struct literal, which is the
`B-DISTRIBUTIONS` trap itself.

## What is needed

Either shape closes it; both are `crates/editor-core/`, outside
CHROME's `paths`:

1. **`DocEdit::SetDocParamUnit { name, unit }`**, refusing typed on an
   undeclared name, on a `Count` (a count is a number, not a quantity,
   and names no notation), and on a unit that does not measure the
   declared `dim` — the pairing `persist::check` validates
   (`crates/editor-core/src/doc.rs:63-70`) and that `written_length` /
   `written_angle` (`doc.rs:152`, `:164`) make unreachable by
   construction.
2. **`DocParam::with_display_unit`**, the carry-forward mirror of
   `DocParam::with_value` (`doc.rs:218-240`) — same "the whole
   declaration rides through untouched" argument, over the other
   field. `apply` would then have a total door to route a unit-only
   edit through, exactly as `edit.rs:1426` routes a value-only one.

## The design question it carries

`with_value`'s rustdoc argues that changing a parameter's KIND is a
redeclaration and must not happen through a carry-forward door. Is
changing its NOTATION the same class of thing? The document says no —
`DocParam::bit_eq` excludes `display_unit` as presentation metadata
(`doc.rs:246-255`), the same ruling `Expr::bit_eq` makes — which is
the argument for a narrow door rather than a redeclaration. Whoever
takes this should state that reading rather than assume it.

## Why it is filed rather than taken

Disclosed as residue 5 of
`work/chrome/doc-params-carry-no-display-unit.md` (CHROME unit 8, PR
1776), which closed the panel half. It needs announcing on the away
channel the way PR 1748's `mate.rs` crossing did.

## Home

`work/issues/` — the door is in `crates/editor-core/src/edit.rs` and
`doc.rs`, which no open program owns and which CHROME's `paths`
exclude.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/docm/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Ev (in chat, 2026-09-16) agreed this is a unit, not a design fork.
Branch `edit/doc-param-unit`. Both shapes the row offers, because
each is half of one door:

1. `DocParam::with_display_unit(&self, unit) -> Option<Self>` beside
   `with_value` (`crates/editor-core/src/doc.rs`): the carry-forward
   mirror over the other field — value and distribution ride through
   untouched; `None` for a `Count` (a count names no notation) and for
   a unit that does not measure the declared `dim` (the pairing
   `persist::check` validates and `written_length`/`written_angle`
   make unreachable by construction — reuse that predicate, do not
   restate it). Exhaustive on both arms as `with_value` is.
2. `DocEdit::SetDocParamUnit { name, unit }`: refuses typed on an
   undeclared name, on a `Count`, and on a dimension mismatch — each
   its own `EditError` arm or a reuse of an existing one where the
   sentence is the same; `apply` routes it through (1) exactly as the
   value-only edit routes through `with_value`. Persisted and replayed
   like every `DocEdit` (`persist/wire.rs` gains its arm; the format
   has no schema version and the corpus regenerates if anything
   committed carries an edit log — say what moved). The `pncad-py`
   façade enumerates `DocEdit` (`py/doc.rs`): add the door there too,
   LIB's file, mechanical, said in the PR.
3. **State the reading, in the door's doc and the PR body**: changing
   a parameter's KIND is a redeclaration (`with_value`'s argument);
   changing its NOTATION is not, because `DocParam::bit_eq` already
   excludes `display_unit` as presentation metadata, the same ruling
   `Expr::bit_eq` makes. A unit edit therefore changes nothing
   `bit_eq` sees — and a row pins that.
4. Rows that go red: the distribution survives a unit edit (RED
   today through `SetDocParam` with `DocParam::continuous` — write
   that trap as the first row, then the door that avoids it); each
   refusal by name; `bit_eq` unchanged across the edit; replay and
   save/load round-trip the edit; the `SessionOp` side is CHROME's
   and is filed on their slate, not built.
