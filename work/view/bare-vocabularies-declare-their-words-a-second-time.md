---
id: bare-vocabularies-declare-their-words-a-second-time
kind: issue
title: the four bare vocabularies list their words a second time in a label match
status: open
opened: 2026-09-06
---


Filed by the `const-all` unit (PR 2046, finding S7). Membership is now
declared once for all nine closed vocabularies; the ordered WORD list
is still declared twice in the four biggest.

## The sites

| Vocabulary | The `ALL` | The second list |
|---|---|---|
| `PathVerb` | `crates/viewer/src/forms.rs` (17 variants) | `PathVerb::label`, 17 arms |
| `Seat` | `crates/viewer/src/seats.rs` (9) | `Seat::name`, 9 arms |
| `ToolKind` | `crates/viewer/src/tools.rs` (7) | `ToolKind::label`, 7 arms |
| `ArcMode` | `crates/viewer/src/forms.rs` (6) | `ArcMode::label`, 6 arms |

## This is NOT the old defect

A `match` is exhaustiveness-forced: a variant with no arm fails to
compile, which is exactly what `PathVerb::label`'s own doc claims for
itself ("a verb with no label is a compile error rather than a `?` on
somebody's screen"). Nothing here can silently go missing. What is
duplicated is the ORDER and the reading: two lists of the same
vocabulary, side by side, that a human checks against each other by
eye.

## The rule that put them in different arms, and the question it leaves

`crates/viewer/README.md`'s **Closed vocabularies are declared once**
now states the rule the unit was applying without saying so: a word
goes in the TABLE when the row that iterates the table is its only
reader, and in a METHOD when anything asks a single value for its
word — a method can be called on one value, a table can only be
iterated. That is why these four are bare and the other five labelled,
and it is checkable: `PathVerb::label` is called on the combo's
*current* verb (`pane/create.rs:719`), `ArcMode::label` on the picker's
current mode (`widgets.rs:297`), `ToolKind::label` inside a sentence
(`tools.rs:102`), `Seat::name` inside four refusal sentences
(`seats.rs`, `session/refuse.rs:395`).

**The question this item is:** the labelled arm could hold these four
too — a labelled vocabulary can still have a `label()` method that
returns its own table entry, so the "asked one at a time" requirement
is not what forces the split. If it can, the rule above collapses to
"labelled always" and the second list goes. Costs to weigh:

- A seventeen-entry declaration carrying both the variant docs and the
  words is a wide block, and rustfmt does not format it
  (`vocabulary-macro-bodies-are-outside-rustfmt`), so legibility is a
  real cost rather than a taste one.
- `label()` derived from the table becomes a lookup rather than a
  match. A `const fn` over an array is fine; the failure mode to check
  is whether it stays const-evaluable and whether any caller depends on
  the match's inlining.
- `Seat::name`'s words compose refusal sentences, so its wording is
  read against `session::refuse`'s discipline, not only against the
  radio row that has none.

Take it with the rule, not instead of it: if the answer is "labelled
always", the README's two-shape rule is deleted rather than amended.
