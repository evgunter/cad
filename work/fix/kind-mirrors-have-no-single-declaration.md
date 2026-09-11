---
id: kind-mirrors-have-no-single-declaration
kind: issue
title: a fieldless kind enum is a hand-mirror of its error enum, and only a source-scan row sees the phantom direction
status: open
opened: 2026-09-04
---


Filed by the `boolean-error-has-no-fieldless-kind` lane at the moment
it disclosed the residue; PR 1490's review named the direction and
this is its durable home.

**The shape.** A fieldless kind enum beside a payload-carrying error
(`profile::PathError`/`PathErrorKind`, `Attr`/`AttrKind`,
`topo::BooleanError`/`BooleanErrorKind`) is TWO hand-written
declarations plus a hand-written projection. The compiler sees one
direction only: an arm added to the error enum reds the exhaustive
`kind()`. A variant added to the KIND enum alone is a phantom —
nothing constructs it, so no test reaches it and the only build it
reds is some downstream exhaustive map, in another crate, if one
happens to exist. Nothing at all objects to an arm projected to the
WRONG kind, which type-checks.

**What exists today — and the phantom direction is mostly CLOSED,
against the first draft of this item.** The tree's idiom is a
compile-time exhaustive visit over the KIND, which names a phantom by
variant at `error[E0004]` rather than scanning anything:

- `PathErrorKind` IS guarded — `pncad-py/src/tags.rs:88`,
  `path_error_tag`, an exhaustive match with zero `_` arms whose own
  doc says that is why it exists. `PathErrorKind`'s doc names it.
- `VerbKind::ALL` (`crates/verbs/src/verb.rs:151`) is the same trick as
  a census, explicitly because the list is hand-written rather than
  macro-generated.
- `BooleanErrorKind` now carries one in its owning crate
  (`crates/topo/src/boolean/mod.rs`,
  `each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind`),
  which is the better home than a downstream tag map: the red lands
  where both enums live.
- `AttrKind` has NO such consumer. Measured, not assumed:
  `grep -rn "AttrKind::[A-Za-z]* *=>" --include=*.rs crates/` returns
  zero arms tree-wide, so nothing anywhere matches on it exhaustively.

**What is actually open.** Two things, and neither is the phantom
direction on the pairs above:

1. **The pairing direction, everywhere.** Nothing objects to an arm
   projected to the WRONG kind — `Self::Merge(_) => Kind::Join`
   type-checks. `BooleanErrorKind` closes this only for the arms a test
   can cheaply construct (payload = keys, spans, `&'static str`); an
   arm nesting another crate's error is unchecked, and no other pair
   checks it at all.
2. **Every pair pays for its own guard by hand**, and a new pair
   arrives unguarded by default — which is how this item came to assert
   the opposite of the truth about `PathErrorKind` in its first draft.

**A guard that scans source text is NOT the answer, and this item
should not be read as asking for one.** The first `BooleanErrorKind`
guard did exactly that and was replaced: it read an ordinary
`/** ... */` doc comment as a variant named `Nothing` and truncated
silently on an unbalanced `{` inside an ordinary `/* */`, and in both
cases failed with a message telling the author to delete a phantom
variant that did not exist. That is
`work/issues/source-scanning-censuses-are-a-tripwire-on-ordinary-rust.md`
— its third instance, and the first to produce a plausible false
accusation rather than an obvious parser panic.

**The fix shape.** A `transition_table!`-style single declaration: one
table generating the error enum, the kind enum and the projection, so
neither direction can drift and no pair needs a hand-written visit at
all. That closes the pairing direction — which no guard in the tree
closes in general — and makes the phantom direction structural rather
than something each pair remembers to buy. It is worth deciding once,
for every pair, rather than adding a fourth hand-written census.
`NodeErrorKind` (SMELL-UV's §D row) and `AttrKind` inherit the
decision; `AttrKind` is the one that would gain a guard it does not
have today rather than a cheaper version of one it has.

## Fourth mirror, and what PR 2344 established (2026-09-11)

`editor_core::product::ProductErrorKind` (10 fieldless variants) joins
`BooleanErrorKind` (41), `PathErrorKind` (28) and `AttrKind`. Four
facts from landing it, each of which changes what this row is asking
for:

**1. The guard shape has now been hand-copied three times.** 1806 wrote
the two-part pair (an exhaustive visit in the owning crate for the
phantom direction; built arms compared against the `Debug` variant name
for the pairing direction); 2344 re-derived it from scratch. Both also
wrote a `label(kind) -> &'static str` table whose *only* purpose is to
make exhaustiveness observable to a test.

**2. The pairing direction is closable by a derive and by nothing
else.** That is the sharp form of this row's ask, and it was not stated
before. A derive would delete the kind enum, the `kind()` projection
and both halves of the guard — four hand-written artefacts per error
type, replaced by one attribute. Every alternative considered so far
(a census, a roster, a source scan) either restates the mirror or reads
source text, and reading source text is
`work/tint/source-scanning-censuses-are-a-tripwire-on-ordinary-rust.md`.

**3. A complete census isolates the residual hole cleanly.**
`ProductErrorKind` is the first mirror whose census covers **every**
arm (10/10 constructible, against 1806's 34 of 41). With the sampling
gap gone, exactly one hole remains and it is now nameable: *an arm
added to the error later, projected onto an existing kind, and omitted
from the sample list.* No test can reach it; a derive makes it
unrepresentable. The 2344 lane wrote and then **deleted** a
`KINDS: [_; 10]` const meant to close it — correctly, since a third
hand-written mirror beside the two is the defect rather than the fix.

**4. `NodeErrorKind` is NOT a missing sibling**, against what this
row's earlier text implies. It exists
(`crates/editor-core/src/eval/mod.rs:675`) and is payload-carrying;
what it lacks is a **fieldless projection**, which is why three doors
render a node refusal to prose. That is a fifth instance and has its
own file, `work/fix/node-error-kind-has-no-fieldless-projection.md` —
which also asks the question that should be answered before this row
mints anything: whether those doors want a fieldless mirror at all, or
simply the enum they already have.
