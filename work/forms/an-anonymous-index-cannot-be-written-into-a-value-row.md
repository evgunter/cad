---
id: an-anonymous-index-cannot-be-written-into-a-value-row
kind: issue
title: A formula's text names a variable only by name, so a pattern whose index is anonymous cannot have its spacing or step typed on its row
status: open
opened: 2026-11-03
priority: P2
cost: M
---



## Finding

VR2 (`crates/editor-core/VARIABLES.md`) says the GUI proposes no name,
and `the-pattern-gesture-names-its-index` asks the pattern gesture to
leave its index anonymous. It cannot yet, because the pattern's
spacing and step are then not editable on their rows.

The placement editor's value rows edit a value as text: the field
shows `matetool::value_text` (`Doc::unparse`) and a typed edit is
`matetool::typed_value` (`parse_formula` over `Doc::var_scope`). A
row's value reads the index, `scalar(k) * 20 mm`.
`editor_core::expr::unparse` writes a reader of an unnamed variable as
`#<16 hex>`, "which the parser refuses", and `Doc::written` expands only
free and defined anonymous variables, never an index. So with the index
anonymous the slide row of a row pattern reads
`scalar(#…) * 20 mm`, re-entering that text is refused, and no text
the person can type reads the index: the spacing and the ring's step
cannot be changed, or made a formula over a variable, without first
naming the index on its variable row.

The build is a way for a value row's text to read the one index its
placement reaches without a name (or for the editor to edit only the
step and keep the index factor), after which `combine::fresh_index`
stops minting `k`, `k1`, ….
(FORMS, the split and pattern forms unit.)
