---
id: bare-vocabularies-declare-their-words-a-second-time
kind: issue
title: the four bare vocabularies list their words a second time in a label match
status: closed
opened: 2026-09-06
closed: 2026-09-08
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

## Closed — two of the four convert, and the rule is corrected (Ev, 2026-09-08)

**Neither answer this item framed.** The rule is not deleted for
"labelled always", and it is not kept: it tested the wrong thing.
"Is there a single-value reader?" sends `PathVerb` and `ArcMode` to the
bare arm even though a production loop walks their table and wants a
word per entry, which is what the labelled arm exists for. The test
that sorts this tree is **does anything walk the table for its WORDS?**
— a question about whether the words are table data, and one a sweep
can answer, where a count of readers is neither.

**The sweep that produces the population**, over `crates/viewer/src`:
every loop over a vocabulary's `ALL`, read for what it asks each entry
for. There are two. `crates/viewer/src/pane/create.rs:727` draws an
option per verb and puts that verb's word on it (`:738`, `:748`), and
`crates/viewer/src/widgets.rs:300` does the same per mode (`:301`).
Both vocabularies also name a single value's word — the combo's closed
face at `create.rs:719` and `widgets.rs:297` — and that is served by a
`label()` the macro now projects from the same list, so it is not what
decides the shape.

- **`PathVerb` (17) and `ArcMode` (6) are now LABELLED**, each
  declaring `pub(crate) fn label;` under its `ALL`. The seventeen and
  the six words are in the declaration once; the loops read
  `(option, label)` pairs.
- **`ToolKind` (7) and `Seat` (9) stay BARE.** Nothing under `src/`
  walks their lists at all. `ToolKind::label` composes a sentence
  (`crates/viewer/src/tools.rs:102`) and `Seat::name` composes the
  refusal sentences `seats` and `session::refuse` write
  (`crates/viewer/src/seats.rs:181`, `:208`, `:357`, `:358`;
  `crates/viewer/src/session/refuse.rs:416`) — wording read against
  that discipline, not against a row of buttons. The suites do walk
  both lists, for the VALUES: `crates/viewer/tests/combine_ops.rs:1290`
  maps kinds to booleans and `:1422` drives one op per seat, where the
  seat's word reaches only an assertion message about the single seat
  that failed (`:1471`, `:1478`). A walk that would still do its job if
  the words did not exist is not a reader of them.

**The three costs.** (1) The seventeen-variant block is hand-formatted,
because rustfmt does not reach inside the invocation
(`vocabulary-macro-bodies-are-outside-rustfmt`); each variant keeps one
doc line above one `Variant = "word",` line, which is the shape the
five already-labelled vocabularies have. (2) `label()` is **not** a
lookup: the macro projects a `const fn` MATCH from the same tokens the
array is built from, so it stays exhaustive by construction, costs no
scan, and is const-evaluable — nothing needed that, and declaring it
`const fn` is what checks it. (3) The accessor is **opt-in**: a
labelled vocabulary declares `fn <name>;` to get one. The five labelled
vocabularies that never ask for a single value's word declare none and
so carry no dead code, and no `#[allow(dead_code)]` blankets the arm —
which would have silenced the report that an accessor had lost its last
reader.

**Every word and its position is unchanged**, proved by running rather
than reading: a throwaway unit test printed `index, variant, word` for
all 23 entries on the merge base (words from the `label` match) and
again after the conversion (words from `ALL`, with the projected
`label()` asserted equal to each entry's word). The two outputs are
byte-identical — same 23 rows, same md5 — and the receipt is in the
PR body.

**This item's own citation was stale**: `session/refuse.rs:395` names
nothing; `Seat::name` is called at `:416`, and `:393` is a different
type's `slot.label()`.

`crates/viewer/README.md`'s **Closed vocabularies are declared once**
carries the corrected test, the population it produces and the sweep
rule behind it.
