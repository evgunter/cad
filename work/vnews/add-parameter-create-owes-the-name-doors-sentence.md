---
id: add-parameter-create-owes-the-name-doors-sentence
kind: issue
title: The add-parameter Create button's name conjunct is silent while ParamName::new has a sentence for it
status: open
opened: 2026-09-24
refs: [a-disabled-control-says-why-in-four-shapes, three-spellings-say-a-parameter-is-not-declared]
priority: P3
cost: E
---

Filed by EDIT's `no-door-refuses-a-blank-parameter-name` lane (branch
`edit/param-name-door`), the hand-off its spec's row 4 names. An
announced crossing: that lane touched `crates/viewer/src/pane/properties.rs`
only as far as the type change forced it, and this row is the rest.

## The door now has the sentence

`ParamName` (`crates/editor-core/src/doc.rs`) is admissible by
construction: `ParamName::new(text)` is the one door, and it refuses a
text the expression parser would not read back as a reference to that
parameter — blank, whitespace, a leading digit, an embedded operator,
padding, a character outside the alphabet — with a typed
`ParamNameFault` whose `Display` is one sentence quoting the text:

> parameter name "1 2" opens with "1" at byte 0, which is not an
> identifier — a parameter name is one identifier, so an expression
> can refer to it

`a-disabled-control-says-why-in-four-shapes` classified the
add-parameter Create button's `!name.is_empty()` conjunct as a correct
silence *given the doors as they were*: there was no refusal for a
blank name, so no sentence for the control to be a second copy of.
That premise is gone.

## The site

`crates/viewer/src/pane/properties.rs`, the add-parameter panel
(`add_param_ui`, where the `Create` button is drawn). Today it reads

```rust
let name = ParamName::new(self.drafts.new_param_name.trim()).ok();
…
let ready = name.is_some() && self.drafts.new_param_dimension.is_some();
let create = ui.add_enabled(ready, egui::Button::new("Create"));
let create = if self.drafts.new_param_dimension.is_none() {
    create.on_disabled_hover_text("pick a dimension first")
} else {
    create
};
```

The dimension conjunct has its hover text; the name conjunct folds the
constructor's answer to a `bool` with `.ok()` and says nothing. By the
census's rule — *a control a reader cannot use owes the sentence a
click would have been answered with, when there is such a sentence* —
it is now an ordinary hit: keep the `Err(fault)` and draw
`fault.to_string()` as the disabled hover text, the way the dimension
conjunct does, so the control shows the door's own words rather than
a chrome paraphrase of them.

## What it is not

Not a change to the door: `Session::create_param`
(`crates/viewer/src/session.rs`) takes a `ParamName` and therefore
never sees an inadmissible one; `Refusal::ParamExists` stays its one
refusal. Not a wording decision either — the sentence is the type's,
and `three-spellings-say-a-parameter-is-not-declared` is the row that
decides how a chrome paraphrase and a session refusal are held in
step. This row is only the hover text.
