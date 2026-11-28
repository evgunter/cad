---
id: whether-a-formulas-source-spells-its-numbers-positionally
kind: issue
title: Is a formula's source text a field over a written value: should a literal in it spell positionally?
status: open
opened: 2026-11-25
priority: P2
cost: E
design: true
---

Found by the review of PR 7775 (`vgeom/field-seed`).

PR 7775 brought every editable field over a written value under the
ratified rule: the value's exact spelling, positional at every
magnitude (`crates/geom-core/README.md`, "A number told to a person").
That covers the mate tool's value and offset fields, whose typed
number `crates/viewer/src/matetool.rs` `value_text` now spells through
`props::FieldNumber`.

A formula's SOURCE text still spells its literals through
`editor_core` `Doc::unparse` → `expr.rs` `write_quantity` →
`quantity::fmt_length`/`fmt_angle` (`fmt.rs` `fmt_in`). That path
prints `Decimal`'s `Display`, which is scientific outside
`[1e-4, 1e16)`. Places that show it:

- A driven slot's keyboard edit opens on its source
  (`crates/viewer/src/props.rs`, `field_source`).
- A defined variable's field (`pane/properties.rs`, `defined_showing`).
- A mate value that is an expression (`value_text`'s fallback).

Example: a slot driven by `2 * 0.00004 mm` re-opens its edit as
`2 * 4e-5 mm`. A digit typed at the end of `4e-5` lands in the
exponent, which is the reason the rule gives for never using exponent
form in a field.

The question: is a formula's source text "a field over a written
value"? The rule does not settle it.

- If it is, `unparse` (or `write_quantity`) spells its literals
  positionally.
- If it is not, the source is a serialized form, which is exact and
  may be scientific, and the rule should say so.

PR 7775 deliberately leaves `unparse`'s output unchanged.
