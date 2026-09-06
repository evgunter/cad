---
id: refusal-has-no-all-to-walk
kind: issue
title: Refusal has no ALL value, so every property over the vocabulary is a hand-maintained list
status: closed
opened: 2026-09-05
closed: 2026-09-06
refs: [refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake, session-clearing-walk-is-hand-maintained-three-times, viewer-const-all-tables-have-no-exhaustiveness-guard, dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum, verb-and-dimension-render-through-debug, prose-census-cannot-see-a-bypassed-prose-renderer, viewer-preview-names-a-verb-by-its-variant-identifier]
pr: 2053
branch: view/refusal-all
---


Named by `refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake` as
"its own small design question" and left there; filed here because the
unit that closed that item ran straight into it.

## What happens

`crates/viewer/tests/panel_edits.rs`'s `refusals_render_as_sentences`
asserted a property over the whole `Refusal` vocabulary — renders as
prose, never as a debug dump — while exercising ONE arm. It stayed
green for as long as `Refusal::Edit` rendered a `{:?}`-quoted parameter
name into the status line, because the arm it walked was `Io`.

That row now walks five arms, each through a real op. It is still a
HAND-MAINTAINED list: `Refusal` has no `ALL`, so a new arm joins the
property by someone remembering to add it, which is the same shape as
the clearing walk `session-clearing-walk-is-hand-maintained-three-times`
closed one layer down.

## The tree already answers half of this, and not with a roster

**A source census, not samples.** `crates/pncad-py/src/prose_census.rs`
reads SITES across the whole tree — every `{binding:?}` in every format
string inside every `impl Display`, resolved to the field type the
binding is declared at — including `crates/viewer/`. Its own header
states why a roster cannot do this job: *"a roster that picks its own
samples excludes the failing mode by construction… what decides the
rendering is the variant of the PAYLOAD, one level down."* That is
precisely `Refusal::Edit`, which forwards ~50 sub-variants.

So the `{`-and-variant-name half of the property has a real guard, and
a third spelling of it exists as well:
`crates/editor-core/tests/display_contract.rs:27-45`'s `assert_f6`,
which asserts `!contains('{')`, no `node:`/`name:` punctuation, and
that the rendering differs from `Debug`.

**What nothing covers is the quoting half.** No census asks whether a
user-facing sentence contains a `"`. It is false today of
`EditError`'s metadata arms (`MetaNotSet`, `MetaNonFinite`,
`MetaUnversioned`, `RebindMetadataCollision`), whose `key: String`
renders `{key:?}` — and `MetaUnversioned` embeds a literal `\"v\"`
besides. `panel_edits::refusals_render_as_sentences` asserts
`!contains('"')` over five sampled renderings and cannot see any of
them, which is the same blindness `prose_census` was written against.

**So the ask is not a third home.** It is: extend the existing source
census to the quoting question (a `{binding:?}` whose field type is
`String` renders quotes in prose), and let the sampled row stop
claiming what a census answers.

## Why it is not simply "add an ALL"

Every other vocabulary in this crate that has an exhaustiveness guard
gets it from a `match` the compiler checks —
`SessionOp::permitted_during_value_gesture` is the model: a fortieth
operation cannot be added without answering for it, because the table
is an exhaustive match rather than a list. For the part a census cannot answer — that every arm
renders SOMETHING, and names its subject — `Refusal`'s arms carry
PAYLOADS (a node id, a name, a boxed `EditError`), so an `ALL` would
have to mint a representative value per arm, and a representative
payload is a fixture decision, not a fact about the type.

Two shapes worth costing before either is built:

- an exhaustive `match` in the test that maps each arm to a sample —
  the compiler then refuses a new arm until it is sampled, and the
  samples stay where the property is asserted;
- a `Refusal::sample_of_each()` behind `cfg(test)` or a test-support
  feature, which puts the same match in the crate and lets more than
  one suite walk it.

The first is cheaper and keeps fixtures out of the shipped type; the
second is what `viewer-const-all-tables-have-no-exhaustiveness-guard`
is about more generally, so the two should be decided together.

## Closed

**There is no `Refusal::ALL` because the compiler is already the
instrument an `ALL` was wanted for, and because no `ALL` of this
vocabulary can exist.**

Both halves matter, and the second is why this cannot be reopened as a
build:

* A value of `Refusal` needs a payload and a payload needs a document
  — `DrivenByExpression { node, slot, params, current }`,
  `NoSuchSlot { node, slot }`, `Edit(Box<EditError>)`. `ALL` over this
  type would have to MINT a representative per arm, and a
  representative payload is a fixture decision, not a fact about the
  type. `crates/viewer/src/vocab.rs`'s `vocabulary!` projects an `ALL`
  from a declaration for **fieldless variants only**, stated in its own
  docs, so the obvious move is blocked by construction rather than by
  effort.
* What an `ALL` was wanted for here is the compile-time obligation:
  *a new arm must answer for itself.* `Refusal` has that.
  `impl Display for Refusal` and `Refusal::rank` are exhaustive matches
  with no wildcard arm (`crates/viewer/src/session/refuse.rs`), so a
  nineteenth arm reds the crate until it is given a sentence and a
  rank. Nothing can be added to this vocabulary and reach a user
  UNRENDERED.

  **That reach stops at `Refusal`'s own arms, and it used to stop one
  level too early.** `rank` carried `Self::Display(_) => 1` beside two
  `DisplayFault` arms hand-listed at rank 2, so an eighth display fault
  took a rank nobody chose — the closure would have overstated itself
  on the one arm whose rank is a per-payload decision. The style review
  of PR 2053 found it
  (`refusal-rank-wildcards-the-display-fault-payload`) and it is fixed
  here rather than caveated: that arm now matches `DisplayFault`
  exhaustively. `Edit` and `SlotUnit` still take one rank each for a
  whole vocabulary, which is a default and is now labelled as one —
  every condition either raises names a real failure.

So the property left over is not "every arm renders" but "every arm
renders as PROSE", and that decomposes into three, of which one was
live and broken:

**Brace fingerprint — guarded, with a hole this item did not name.**
`crates/pncad-py/src/prose_census.rs` reads every `{binding:?}` in
every format string inside every `impl Display` in the tree, resolved
to the field type the binding is declared at. Its scan set is literally
`impl … Display for X { … }` bodies. This vocabulary composes three of
its sentences OUTSIDE that impl, by ratified design — `Refusal::
affordance`, `exists_wording` and `offer_wording` are each "its one
home" precisely so a pre-click surface and the status line cannot
drift — and `Display` then delegates through a bare `{}`. **A wording
composed in an inherent `impl` is invisible to the census.**

**Variant identifier — guarded nowhere, and one was reaching the
user.** `Refusal::exists_wording` rendered `({dimension:?})`, so the
status line and the add-parameter form both said *"parameter width
already exists (Length) — edit it instead?"*. That is a violation of a
rule with a written home: `Dimension`'s `Display` in
`crates/editor-core/src/expr.rs` declares itself "the one home of the
dimension-in-prose rule … refusal prose renders it as the common noun a
person would say … and never as the variant identifier. That holds
wherever a dimension reaches a user." Three sibling leaks of the same
shape stood in the properties panel
(`crates/viewer/src/pane/properties.rs`, the parameter header and the
two dimension tags). All four are fixed here and the refusal one is
pinned by `refusals_render_as_sentences`, which now walks
`ParamExists`.

It got past both instruments for two independent reasons, and that is
the answer to "name it concretely": the census could not SEE the site
(inherent impl), and had it seen it the verdict would still have been
`Verdict::Prose` — a fieldless enum carries no `" { "`, so the census's
question, which is about braces, does not ask whether a `Debug` here
spells an identifier a person then reads.

**Quotation marks — the item's own ask, answered NO.** This item asked
to extend the census so that a `{binding:?}` over a `String` reds,
because `panel_edits`'s row asserted `!contains('"')` over five samples
while `EditError`'s metadata arms render a quoted key. That extension
would be enforcing an unratified rule against deliberate prose. The
ratified `Display` contract is F6
(`crates/editor-core/tests/display_contract.rs`): no brace, no
`node:`/`name:` punctuation, no variant identifier, never simply the
dump. A quotation mark is not a `Debug` fingerprint — *no metadata
"colour" is set on the face minted by node 7* quotes a user's key on
purpose, and `MetaUnversioned` names the D7 `"v"` field by writing it.
So a CENSUS extended to a blanket quote ban would red correct prose.

**But the row's own clause was dropped on that argument and should not
have been**, which the style review caught
(`refusals-render-row-dropped-the-clause-that-caught-its-own-defect`).
The metadata arms are not arms this row walks; all six sampled arms are
quote-free, so the clause was green. And it is the only clause that
catches the case the row exists for: a `{:?}` over a `String` or a
`ParamName` renders `"width"` — no brace, no field punctuation, and the
identifier it leaks is the PAYLOAD's rather than the arm's, so
`contains(arm)` misses it and the whole-string `assert_ne!` against
`Debug` cannot see a fragment inside prose. It is restored, beside
F6's three. The distinction that survives is the one this whole item
turns on: **a tripwire over named samples may be stricter than the
ratified contract; a claim over a vocabulary may not.** The row's doc
comment carries it, along with which half is the compiler's and which
the census's.

**Residue.** Inside this program's fence there is one, and it was
already on the slate:
`dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum` carries
the add-parameter form's hand-written `Dimension` table AND its
capitalized labels, which its last section reads against the same
dimension-in-prose clause this unit enforced. The three sites fixed
here are not that row and it stays open. Outside the fence, the two
census gaps above — `prose_census` reads `impl Display` bodies only,
and its verdict asks about braces rather than about identifiers — are
LIB's ground (`crates/pncad-py/*`), so they are filed as
`work/issues/prose-census-cannot-see-a-bypassed-prose-renderer`, which
is where a finding no single program owns goes; a unit branch does not
file onto LIB's slate.

**The coordination miss.** This unit's `Dimension` sweep read the tree
and not the board.
`work/fix/verb-and-dimension-render-through-debug` (open, FIX's slate)
already enumerated all four sites fixed here, at their pre-split paths,
under "`Dimension` renders through `Debug` in four UI labels" — so the
tracker held the complete hit list before the grep did. It also holds a
FIFTH instance of the same class this sweep missed, because the pattern
was scoped to the type and not to the defect:
`profile::path::Verb` rendered `{verb:?}` at
`crates/viewer/src/sketch.rs:663` and put on screen by
`pane/create.rs`. That half is deliberately untouched — its fix is
`impl Display for Verb` in `crates/profile`, not this program's
territory, and forwarding it from the viewer would mint a fourth
spelling of the word list. The reviewer's
`viewer-preview-names-a-verb-by-its-variant-identifier` is its viewer
half and now records the state of both halves of the FIX item.

Closed as ANSWERED-and-fixed: the instrument asked for cannot exist,
the compile-time obligation it stood in for already does, and the one
property genuinely unguarded turned out to be a live defect rather than
a missing roster.
