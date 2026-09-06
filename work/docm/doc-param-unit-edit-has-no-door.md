---
id: doc-param-unit-edit-has-no-door
kind: issue
title: No editor-core door changes a document parameter's display unit — SetDocParam would drop the distribution
status: open
opened: 2026-09-04
refs: [1776]
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
