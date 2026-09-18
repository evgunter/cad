---
id: composed-expected-phrases-are-hand-copied-across-sites
kind: issue
title: The composed expected: phrases PR 2376 licensed to stay prose are themselves hand-copied: datum frame three times, datum axis three, datum plane twice
status: closed
opened: 2026-09-11
refs: [2376]
pr: 2480
closed: 2026-09-13
---


## Finding

Raised by the style review of PR 2376 against that PR's own disposition,
and it is the right objection. Confidence `sure`, the reviewer's.
Accurate at `af8bbca`. **This row exists because PR 2376 closed
`wire-expected-phrases-spell-family-words-as-literals` with it
disclosed** — `work/README.md`: disclosing a residue is not scheduling
it.

PR 2376 argued, correctly, that the composed phrases are **not family
words** and gain nothing from a `family::` const: `"datum frame"` names a
variant *within* a family, and the `found:` beside it answers
`kind_name()` = `"datum"` on purpose, so a `family::DATUM_FRAME` would
spell `"datum"` a second time one level up. That answers *"do these
belong in `family`?"*

It does not answer the question the item's class actually asks — **one
vocabulary, several spellings** — because the composed phrases are
duplicated among themselves:

- `"datum frame"` ×3 — `wire.rs:978`, `:1019`, `:1080`
- `"datum axis"` ×3 — `wire.rs:1707`, `:3964`, `mate/member.rs:716`
- `"datum plane"` ×2 — `verbs/split.rs:157`, and `:218`'s `assert_eq!`
  pinning it, which is the one copy that is a guard rather than a twin

Three hand-written copies of one phrase is the class one level up, and
"stays prose" leaves it untouched.

**Two sites the PR's stated licensing rule does not actually cover**, and
this is where the rule needs restating rather than the code changing:

- `wire.rs:694` `"body or instances"` is **wider** than a family, not
  narrower — it is `family::BODY` and `family::INSTANCES` with `" or "`
  between them, and nothing else. PR 2376's leg 1 ("they are not family
  words") is simply false of this one site; only leg 2 (the tree has no
  compile-time string concatenation) does any work there, which the PR
  did not say.
- `wire.rs:1603` `"an axis in a sketch frame (Datum::AxisInPlane)"` is a
  sentence, licensed by neither leg.

## What a taker owes

A rule that covers all three cases — narrower than a family, wider than
one, and a sentence — or a home for the repeated phrases, or an argued
statement that two copies of a two-word phrase are cheaper than any
mechanism that would unify them. The last is a real answer; it has just
never been written down. The reviewer also names a composer route the
PR's survey missed and did not rule on: a `macro_rules!` per family word
expanding to its literal, so `concat!` can compose them at compile time
with no dependency (`unsure` whether it is an improvement).

Read beside `frame-plane-lane-and-axis-frame-are-one-door`: two of
`"datum frame"`'s three copies are that one duplicated door, so unifying
it retires them as a side effect.


## Closed 2026-09-13 (PR 2480)

**The licensing rule was written, and it covers all three cases the row
said it had to.** An `expected:` comes from a const — `family` when it
is exactly a value family, `phrase` otherwise — and no `expected:` is a
literal at a call site. Narrower-than-a-family, wider-than-one
(`"body or instances"`) and the whole-sentence case are all inside it,
because the rule is about where the word lives rather than about what
shape the phrase has.

Implemented with the reviewer's own unruled suggestion: a
`macro_rules!` per family word, with `family`'s consts defined **from**
it and `phrase`'s composed by `concat!` at compile time. One literal per
word, no new public name.

**And the rule is guarded**, which is the part that took two review
rounds. The first cut stated it in prose with nothing behind it; the
delta round then broke the first guard by hoisting a const to a call
site, which is the exact move the same commit had made three times.
The census now asserts the rule it states rather than a proxy for it,
and the reviewer's breaking mutation is the first row of its mutant
table.

**One residue, filed rather than disclosed**: four direction role words
respell a phrase const instead of composing it, because `concat!` takes
literals and a `const` is not one, so composition needs the macro layer
extended from words to phrases —
`work/wire/direction-role-words-respell-the-operand-phrases.md`. The
sweep was attempted and failed to compile; the three that were literals
at call sites are named consts now.
