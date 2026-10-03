---
id: the-mint-reader-hosts-three-lexer-operations-of-its-own
kind: issue
title: the errors mint reader hosts three lexer operations inside a consumer, one of them the inverse of a shared one
status: open
opened: 2026-09-15
priority: P3
cost: D
---


Filed by CENSUS-ARRIVAL-RESIDUE (2026-09-15), disclosing what it took
rather than leaving it in a PR body. The unit's spec said: *"It is not
a widening of `test_utils::source`'s grammar. If you find you want
one, file it rather than taking it."* This is that filing.

## The three

`crates/pncad-py/src/tests.rs` hosts three lexer operations inside the
census that consumes them:

* `balanced_open` — the offset of the `(` that opens the round bracket
  a text ends with. It is `test_utils::source::balanced_end`'s inverse,
  same precondition, read the other way round.
* `item_start` — where the item whose keyword is at an offset begins,
  walking back over `pub`/`pub(…)`/`unsafe`/`async`/`extern`/`const`,
  and `None` where the keyword opens no item. The sibling census
  `crates/test-utils/tests/hand_written_impl_census.rs` has the same
  question and answers it with `boundary_before` alone, which admits
  an `impl` in type position.
* `strip_modifier` — a trailing whole-identifier suffix, which is
  `boundary_before`'s predicate at the other end of a word.

CENSUS-ERRORS-ARRIVAL's close-out moved four operations out of this
same file on the argument that *"a reader hosted inside one of its
consumers is how the tree got its drift"*. Two of these three were
added by that same unit and named in neither its item file's *what
this unit did* nor its PR; `item_start` is this unit's, grown out of
the `starts_an_item` it inherited.

## Why they were not moved

The spec ruled it: `crates/test-utils/*` is TCOST's and TINT's, and a
widening of `source`'s grammar is a filed row rather than a taken one.
That is the right call for a fix pass and it is not an argument that
they belong here — the sibling census wants `item_start`'s answer and
does not have it.

## What closing it looks like

`balanced_open` and `strip_modifier` are small and have an obvious home
beside `balanced_end` and `boundary_before`. `item_start` is the one
with a design question in it: its allow-list of modifiers is the part
that has to be right, and a shared version has to serve a caller that
walks `impl` only as well as one that walks six keywords.

Territory: `crates/test-utils/*` is TCOST's and TINT's; the file the
operations sit in today is LIB's fence and this program's `keep_out`
announces its pncad-py rows there.
