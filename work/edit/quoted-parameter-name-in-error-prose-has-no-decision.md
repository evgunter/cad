---
id: quoted-parameter-name-in-error-prose-has-no-decision
kind: issue
title: editor-core quotes a parameter name in error prose at some doors and not at others, and nothing decides which
status: closed
opened: 2026-09-16
closed: 2026-09-17
branch: edit/param-name-display
pr: 2800
---


Disclosed by `edit/error-prose`, which swept `{binding:?}` inside every
`impl Display` in `editor-core/src` and repaired the sites where the
rendered payload was a variant identifier or a struct dump. This is what
that sweep left standing, with the reason it was left and the question
that decides it.

## The residue

A `{x:?}` whose payload is a plain `String` renders the string plus
`Debug`'s quotes. It carries neither of F6's two fingerprints — no
variant identifier, no field-name punctuation — so it is not the class
the sweep repaired. It is nonetheless a `Debug` rendering in a user's
sentence, and it is spelled inconsistently across one crate:

| door | spelling |
| --- | --- |
| `EditError` (`edit.rs`) | bare — `parameter {}` on `name.0` |
| `PersistError` (`persist/mod.rs`) | quoted — `document parameter {:?}` |
| `NonFiniteSite`, `SnapshotError` (`persist/check.rs`) | quoted |
| `RangeRefusal` (`range.rs`) | quoted |
| `SplitError`, `InlineError` (`refactor.rs`, FIX's) | quoted |
| `ParseError` (`parse.rs`) | quoted |
| `EvalError` (`expr.rs`) | quoted |

`EditError`'s own `Display` header already records the split and
declines to decide it: its category prefix and its quotes were removed
deliberately, and the comment says its neighbours *"keep their spelling
until someone decides for them"*. Nothing schedules that decision, which
is what this file is.

## Why it is not simply "quote nothing"

The two spellings answer different questions. A bare name composes with
a sentence that already frames it (`parameter width is declared
length`); quotes delimit a name the reader supplied and may have
mistyped, which is why `ParseError` quotes — `{name:?} is not a
parameter this document declares` is about the exact bytes. A blanket
sweep either way would lose one of those.

`ParamName` is a `String` newtype with no `Display`, so the choice today
is made per call site with `{:?}` or `name.0`. A `Display` on
`ParamName` would move the decision to one place, which is probably the
shape of the answer; whether the quotes ride it, or ride a second door
for the did-you-type-this case, is the decision.

## The hazard that makes it more than taste

`Debug` on `String` is its prose today because `String` cannot grow a
field, and every site in the table renders either a `String` field
(`key`, `found`) or a `ParamName`'s `.0`. That is an assumption the
rendering sites make and do not state: a metadata key re-typed from
`String` to a structured key, or a `{name:?}` written over the newtype
instead of over its `.0`, starts dumping braces with no edit to the
sentence.

`crates/pncad-py/src/prose_census.rs` bounds that exposure where it can
see it — it resolves the field's declared type, so a field re-typed to a
struct reds the census. It cannot see the positional form: a bare
`{:?}`, which is how most of the table's sites are written, reaches its
`UNDECIDED` roster as `<positional>` and is decided by nobody. So the
instrument covers the named half of this residue and not the half the
crate actually uses.

## Ruled and spec'd (2026-09-17, EDIT orchestrator) — E-class, branch `edit/param-name-display`

**Ruling.** `ParamName` gains `impl Display` rendering the bare name.
The doors that frame the name in a sentence render it bare through
that `Display`; the doors that echo bytes the user typed — `ParseError`
(a name that may be mistyped) — keep `{:?}`, and say at the site that
the quotes mean "the exact bytes you wrote". Nothing else decides per
call site.

**What lands.** `impl core::fmt::Display for ParamName`; every `{:?}`
of a `ParamName` in an `impl Display` in `crates/editor-core/src`
(`persist/mod.rs`, `persist/check.rs`, `range.rs`, `refactor.rs`,
`expr.rs`) becomes `{}` except `parse.rs`'s, which keeps `{:?}` with
the sentence; `EditError`'s header comment loses "keep their spelling
until someone decides for them" and states the rule; the F6 census
rows whose expected content words carried quotes re-baseline (say
which). `refactor.rs` is FIX's ground — mechanical, disclosed.

**Row.** One row over the rendered sentences: a parameter name renders
without quotes at every door except parse, and with them there.

**Territory.** `crates/editor-core/src/{doc.rs, persist/, range.rs,
expr.rs, parse.rs}` (EDIT), `refactor.rs` (FIX, mechanical),
`crates/editor-core/tests/display_contract.rs` (TCOST/TINT). E-class:
green CI and the orchestrator's read.

## Built (2026-09-17)

**The rule has one home.** `ParamName` carries a `Display` that writes
the bare name, with the exception and its reason in its rustdoc. Every
door in `editor-core` that frames a parameter name in a sentence of its
own renders through it — including the twelve `EditError` arms that were
already bare through `name.0`, so no prose site reaches inside the
newtype any more and the hazard this row names ("a `{name:?}` written
over the newtype instead of over its `.0`") is a compile-time question
rather than a wording accident. `ParseError` keeps `{:?}` at all eight
of its placeholders, and its `Display` header now says what the quotes
mean: every one of them renders text lifted verbatim out of the
author's input.

**Twenty-four placeholders moved** across eight files: `edit.rs` (12,
`.0` -> `Display`, same bytes out), `persist/mod.rs` (2),
`persist/check.rs` (6), `range.rs` (2), `expr.rs` (2), `refactor.rs`
(2, FIX's, mechanical), `analysis.rs` (6) and `stackup.rs` (4) — the
last two PROPS's, mechanical, and **a correction to this row's own
table**, which did not list them. Two of `persist/check.rs`'s six
arrived mid-unit: `SnapshotError::PayloadUnknownDocParam` and
`PayloadDocParamDimension` landed on main with
`load-door-does-not-check-payload-expression-param-refs` (PR #2793) and
were caught by the re-sweep at this branch's merge, not by the sweep at
its base.

**The row.**
`display_contract::a_parameter_name_renders_unquoted_at_every_door_but_parse`:
thirteen doors, one arm each, asserting the rendered sentence names the
parameter and does not quote it, plus the parse door asserting it does.
Verified red before the change (`PersistError::DisplayUnit` first).
The certified-range and stackup doors are a SECOND row of their own,
`…_at_the_interval_only_doors`, because those modules compile in the
interval build alone and `check-interval-cfg-additive` refuses a
feature cfg over a block inside a shared test: the interval legs run
only the tests the feature adds, so a test present in both builds must
run identical code in both. Both rows call one predicate, so the two
lanes cannot drift into asking different questions.

**Two expectations re-baselined**, both asserting the quotes:
`tests/lib_doors_node_result.rs`'s `EvalError::UnknownParam` case
(`"\"width\""` -> `"parameter width"`) and
`tests/asm4_split_inline.rs`'s `InlineError::ParamConflict` assertion
(`msg.contains("\"L\"")` -> the sentence fragment around the name). No
F6 `assert_f6*` case carried quotes: those match content words as
substrings, so the class this row named as re-baselining turned out to
be two hand-written assertions and no census row.

**Two premises corrected.** `ParseError::UnknownParam`'s `name` is a
`String`, not a `ParamName` (it is built from `key.0` at the parse
site), so the parse door is outside the `ParamName` sweep by type as
well as by ruling. And the positional blind spot this row describes is
not where these sites landed: `prose_census.rs`'s `expression_type`
reads a tuple-struct field, so a positional `{:?}` over `name.0`
resolves to `String` and verdicts `Prose` — none of the twenty-two was
on `UNDECIDED` or `KNOWN_BRACED`, and the census stayed green across
the change. The blind spot is real for a positional `{:?}` written over
the newtype itself, which is the hazard, not the state.

**The other half of the residue is left, as a stated negative.** The
`String` payloads that are not parameter names — a metadata key
(`edit.rs` ×4, `persist/check.rs` ×2, `refactor.rs`, `meta/mod.rs`), the
header line `PersistError::HeaderId` echoes, a unit symbol
(`expr.rs`, `parse.rs`) — are quoted at every one of their sites, so
there is no second spelling to decide between, and each is text a door
was HANDED rather than a name the document holds: the reading
`parse.rs`'s header now states. They are also the named-binding half
the census can type, so a re-type to a structured key reds it. No row
filed; if a future reader wants that written beside each door rather
than argued here, it is a wording unit and not a decision.

## Closed (2026-09-17, EDIT orchestrator)

Built and merged as PR #2800 (E-class: green CI and the orchestrator's
read). `ParamName` has one `Display`, bare; the twenty-four framing
sites in eight files render through it (including PROPS's
`analysis.rs` and `stackup.rs`, which the residue table had
undercounted — placeholder-only edits, crossed by announcement, since
the rule must hold at every door for the guard row to be true); the
parse door alone keeps `{:?}` and says its quotes mean the exact bytes
the author typed; two guard rows in `display_contract.rs` hold eleven
doors plus the parse exception and the interval-only doors. Two
premises corrected by the lane: `ParseError::UnknownParam` carries a
`String`, not a `ParamName`; the prose census already typed the
positional sites as `Prose`. No F6 row moved — two hand-written
assertions that asserted the quotes did. Two CI lessons banked for
every lane (interval-feature additivity; a conflicted PR gets no run).
The remaining `{:?}`-over-`String` class (metadata keys, unit symbols,
a raw header line) is quoted at every site and is text a door was
handed, so no second spelling exists to decide.
