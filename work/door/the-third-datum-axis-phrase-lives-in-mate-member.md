---
id: the-third-datum-axis-phrase-lives-in-mate-member
kind: issue
title: mate/member.rs hand-writes the third copy of "datum axis", which eval::phrase::DATUM_AXIS is now the home for
status: closed
opened: 2026-09-12
priority: P1
cost: E
branch: door/datum-axis-phrase-home
pr: 2984
closed: 2026-09-21
---



## Finding

Found by WIRE's operand-door unit while closing
`composed-expected-phrases-are-hand-copied-across-sites`. The path is
DOCM's and MSOLVE's, so the unit announced it rather than editing it.
Accurate at the merge base of that unit's PR.

`crates/editor-core/src/mate/member.rs`'s `axis_datum` writes
`expected: "datum axis"` as a literal. That phrase now has a home —
`crate::eval::phrase::DATUM_AXIS`, composed at compile time from the
family word `"datum"` so the phrase and the `found:` word beside it
cannot drift — and WIRE's two copies (`tube_args` and `stepped_map`'s
circular arm, both in `eval/wire.rs`) were retired onto it. This is the
third and last copy in the tree.

The rule the const's module states is *"no `expected:` is a literal
written at a call site"*, and this is the one site left that breaks it.
The change is one word for one path: swap the literal for
`crate::eval::phrase::DATUM_AXIS`. The refusal text does not move, so
`crates/editor-core/tests/msolve3_placer_refused.rs`'s
`kind.contains("datum axis")` stays green by construction.

## What is NOT owed here

`axis_datum`'s `found:` is already right: it comes from
`crate::eval::node_value_kind(doc, other)`, the recipe-side reading of
the family. WIRE's `node_operand` in `eval/wire.rs` is the same shape
given a home, and `axis_datum` is its natural second consumer — but
that is a refactor across a fence, not this row. This row is the
literal.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/door/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the fix is written in the row and it is one PR on a file another program owns, which is DOOR's test. Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Closed (2026-09-21) — PR 2984

One line: `axis_datum`'s `expected:` is `crate::eval::phrase::DATUM_AXIS`.
Byte-identity verified rather than assumed — `DATUM_AXIS` is
`concat!(family_word!(datum), " axis")` and `family_word!(datum)`
expands to `"datum"`, so the const IS `"datum axis"` and no refusal text
moves. The lane did not widen into `node_operand`, which the row had
fenced off.

**"Third and last" now holds as a measurement, not an inherited claim.**
Four patterns at the merge base, including one for the rustfmt-wrapped
spelling (zero hits) and a read of every `NodeErrorKind::WrongOperand`
construction in `editor-core/src`. One hit, and it is this one. Blind
spot stated: an `expected:` fed from a const declared at the call site
(`DATUM_AXIS_ROLE`'s shape), or built by `format!`/`concat!`.

## The pin, and a trap this change sets for the next reader

Instruction 3's answer here is **yes**, which is the first time in two
waves. Measured by mutation rather than read: setting the literal to
`"datum axis DRIFTED"` turned **4 of `member.rs`'s 8 rows red**, because
those rows build their expectation from the const while the source held
a literal. That gap is what they were discriminating.

**This change closes that gap, so those four rows no longer discriminate
the phrase's value** — both sides are now the same const, and changing
`DATUM_AXIS` moves them together. That is correct and is the point of
one home; there is no drift left to catch.

**What still pins the user-visible text is two literals in
`crates/editor-core/tests/`**, and they must stay literals:

- `lib_tube_node.rs` matches `NodeErrorKind::WrongOperand { expected: "datum axis", .. }`
- `msolve3_placer_refused.rs` asserts `kind.contains("datum axis")`

The lane left both, giving a mechanical reason (an integration test
cannot name a `pub(crate)` const). **The real reason is stronger: a test
that named the const could never catch a change to the const's
expansion.** After this unit those two literals are the only thing in
the tree asserting what a user actually reads. A later lane that sees
`expected: "datum axis"` in a test file and folds it onto the const to
be tidy would silently delete the last pin on that sentence — so the
rule the `phrase` module states (*"no `expected:` is a literal written
at a call site"*) is about CONSTRUCTION sites and does not reach test
assertions.

**Fence:** `crates/editor-core/src/mate/member.rs` is MSOLVE's, crossed
by announcement and posted on `work/msolve/log.md`.

**Filed:** `work/wire/node-operand-has-one-consumer-where-axis-datum-is-the-same-door.md`
— and it is a finding rather than a restatement of this row's fenced-off
half: `node_operand` DROPS the seat `node_value_kind` answers with
(`.map_err(|seated| seated.1)`), while `axis_datum` CARRIES it, as
MSOLVE-7 settled for the recipe road. The two doors disagree about the
seat, so sharing them is a decision and not a rename.
