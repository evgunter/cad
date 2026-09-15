---
id: direction-role-words-respell-the-operand-phrases
kind: issue
title: Four direction-role words respell an eval::phrase const plus a suffix; composing them needs the macro layer extended from words to phrases
status: open
opened: 2026-09-12
---



## Finding

Found by the full review of PR 2480 (S3), against that PR's own stated
goal. Confidence `sure`, the reviewer's.

PR 2480 gave every `expected:` phrase one home (`eval::family`,
`eval::phrase`) and stated the rule that no such phrase is a literal at
a call site. A SECOND user-visible vocabulary sits beside it and does
not follow suit: the DIRECTION-ROLE words a
`NodeErrorKind::Direction` refusal carries, which name the slot whose
vector would not normalize. Four of them open with a phrase from
`eval::phrase` and respell it:

- `FRAME_X_ROLE` = `"datum frame x axis"` — `phrase::DATUM_FRAME` + `" x axis"`
- `FRAME_Y_ROLE` = `"datum frame y axis"` — `phrase::DATUM_FRAME` + `" y axis"`
- `PLANE_NORMAL_ROLE` = `"datum plane normal"` — `phrase::DATUM_PLANE` + `" normal"`
- `DATUM_AXIS_ROLE` = `"datum axis direction"` — `phrase::DATUM_AXIS` + `" direction"`

all in `crates/editor-core/src/eval/wire.rs` beside the placement-rule
arithmetic. (PR 2480 minted the first three as consts, replacing three
literals written at their call sites, so the "one home per phrase" half
is already done. What is not done is the composition.)

`TRANSFORM_AXIS_ROLE` and `PATTERN_DIRECTION_ROLE` share no family word
with anything in `eval::phrase`, so they are not this row.

## Why it was not swept with the rest

`concat!` takes LITERALS and a `const` is not one. `phrase`'s own consts
compose because they are built from `family_word!`, a macro that expands
to a literal; a role word cannot be built from `phrase::DATUM_FRAME` the
same way. Composing them needs the macro layer extended from the WORDS
to the PHRASES — a `phrase_word!` beside `family_word!`, with `phrase`'s
consts defined from it and `eval::mod`'s
`every_family_word_has_exactly_one_const` census extended to hold the
second macro's arms and consts in step the same way.

That is a real design step, not a rename, and it also has to answer
whether the role words want their own `role` module in `eval/mod.rs`
(where the macro is textually in scope) or an exported macro so they can
stay beside the arithmetic that raises them. `mate/member.rs` reads
three of them through `crate::eval::`, so either home works for its
callers.

## What guards it today

Nothing. The four are consts, so the words are not duplicated — but the
family word inside each is a second spelling of `family::DATUM`, and the
census PR 2480 added is scoped to `expected:` and says so.
`crates/pncad-py/tests/test_document.py` asserts
`f"datum frame {axis} axis"`, so two of these are pinned end to end by
the Python suite and a re-wording would red there.
