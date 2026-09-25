---
id: hand-maintained-counts-in-frame-rs-prose-have-no-guard
kind: issue
title: Five prose counts in frame.rs are maintained by hand, one went stale in a day, and nothing reds for any of them
status: closed
opened: 2026-09-20
priority: P4
cost: E
rides_with: frame-rs-says-the-per-subject-line-is-a-question-for-ev
closed: 2026-09-24
---


Filed 2026-09-20 by VNEWS's frame.rs-cluster adjudication, after it
caught one of these stale and was about to schedule a rewrite that
updated the number instead of removing it.

## The trigger

`crates/viewer/src/frame.rs:1345` says *"what [`tool_news`] buys for
its twelve sites by having one door"*. There are **fourteen** —
`crates/viewer/src/pane/create.rs` ten, `crates/viewer/src/app.rs` two,
and `crates/viewer/src/pane/profile.rs` two (`:110`, `:120`), the last
pair landed by `49897008d` on **2026-09-19**. The sentence was true for
a fortnight and false for a day before anyone read it, and nothing went
red.

**Updating it to fourteen re-mints the defect**, which is why this row
exists rather than a line in the rewrite: the next `tool_news` site
falsifies it again, on the same silence.

## The population

**Every prose count in `crates/viewer/src/frame.rs` that states the
size of a population the file does not derive.** Run over the module
header and the doc comments:

| site | the count | what it counts |
|---|---|---|
| `crates/viewer/src/frame.rs:72` | *"eighteen writers"* | writers that can put a sentence on the line |
| `crates/viewer/src/frame.rs:140` | *"Seventeen of the eighteen"* | the same population, through the ranking |
| `crates/viewer/src/frame.rs:156` | *"The eighteenth"* | the same population's residue |
| `crates/viewer/src/frame.rs:1345` | *"its twelve sites"* | `tool_news` call sites — **stale today** |
| `crates/viewer/src/frame.rs:1519` | *"the three places it could"* | where a file dialog opens |

The first three are one count spelled three times, so a nineteenth
writer falsifies three sentences at one stroke — the count-fixed-in-one-
place hazard `work/view/plan.md` records, pre-loaded.

`crates/viewer/README.md` mirrors the eighteen/seventeen pair at
`:737` and `:743`. That file is VDOC's by `work/vnews/program.md`'s
`keep_out` and is filed there, never fixed from here.

## What the answer is, and what it is not

`work/view/plan.md`'s rule is *a universal in prose owes the sweep rule
that produces its population, written at the sentence*. The rule this
row adds is the next step for a NUMBER: **a sentence that states a
population size either carries the rule that derives it — so a reader
can re-run it — or states no number at all.** *"Every writer that can
put a sentence on the line, which is every `push` onto
`ViewerBehavior::notices` plus the startup initializer"* is falsifiable
by grep and never goes stale; *"eighteen"* is neither.

Not proposed: a test that pins each number. `frame_policy.rs`'s
`the_readme_counts_its_two_populations_correctly` already shows what
that costs and what it is worth — it scans return types rather than
names, and it exists because the two populations it guards are
structural. A `grep`-shaped count of call sites in four files is not,
and a guard that has to be hand-updated beside the number it guards is
the same defect wearing a `#[test]`.

So the deliverable is per-site: drop the number and state the rule, or
keep the number and write the derivation beside it, decided by reading
each of the five. Whichever way each goes, **say at the site why it
cannot be mechanically guarded**, because the next reader's first
instinct is the test.

## Home

VNEWS's: `crates/viewer/src/frame.rs`, serialized. It rides
`work/vnews/frame-rs-says-the-per-subject-line-is-a-question-for-ev`,
group A of `work/vnews/plan.md` §Order 7, which carries `:1345`; the
other four are the same shape and are re-read in the same pass.

Same shape as `work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`
— a claim in prose that no mechanism holds true — and the two are worth
reading together, but they are different populations and neither
subsumes the other.

## Closed 2026-09-24 (`vnews/frame-rs-prose-pass`)

Every number the row lists was re-read against the tree, and each one
was dropped rather than updated; where a count was worth keeping, the
thing that holds it is named instead. Line numbers below are the merge
base's and will rot; the symbols will not.

**The row's five, plus one its table missed.**

| site | what it said | what it says now |
|---|---|---|
| module header, *"Held state"* paragraph | *"the sweep that sorted eighteen writers"* | what sorting the line's writers needs; no number |
| module header, *"The line: news, ranked"* | *"Seventeen of the eighteen writers"* | *every writer but one*, with the derivation (a write to `ViewerApp`'s `notices`, which `ViewerBehavior` lends the panes) and, at the site, that **nothing enforces the "but one"**: `ViewerBehavior` lends the panes `status` too, so a pane could `apply` a `Show` and nothing would go red; a grep-count row like `frame_policy.rs`'s `the_readme_counts_its_two_populations_correctly` would red at every new writer, not at one that bypassed the ranking |
| same, next paragraph | *"The eighteenth"* | *"The exception"* |
| `SeamSubject`'s doc | *"its twelve sites"* — **fourteen** today (`pane/create.rs` ten, `app.rs` two, `pane/profile.rs` two) | *"its call sites"* |
| `tool_news`'s own doc — **not in the table** | *"its twelve sites"*, *"all twelve"* twice | the derivation (every `frame::tool_news` call under `crates/viewer/src`), and at the site why a grep-count row would buy nothing: it reds at every new site, and a new site is not wrong. Its *"the one door here that a type does not pin"* — a phrase-shaped count, false beside `startup_notices`' *"Not type-pinned"* — is now *"a door a type does not pin, like `startup_notices`"*, and the subject-doors header's *"`tool_news` is the exception"* likewise |
| `dialog_dir`'s doc | *"the three places it could"* | *"the candidates its signature takes"*, then the three by name |

**The table's instrument could not see `tool_news`'s own doc**: it
listed the one `twelve` it met, and the same number sat three more
times in the function it counts. A population of counts is found by
grepping for number words, not by following the one that went stale.

**The sweep, and what it found beyond the row.** A grep over every
`//` line of `frame.rs` for the spelled numbers one to twenty and for
ordinals, each hit read for *does this count a population the file does
not derive*. Four more members, one of them already false:

- **`Withdrawal`'s doc, *"three of the four say instance
  N"*** (`AdmissionFault`'s arms) — **false**: two say *instance N*
  and two, `NoSuchNode` and `NotAnInstance`, say *node N*. Now names
  the arms.
- the `Withdrawal` `Display` comment's *"four sentences … a fifth"* —
  number dropped; it names the `Display` match as what makes a new arm
  an edit there.
- `SeamSubject`'s *"the scene, the δ field and the pick index are three
  seams"* — contradicted `SCENE_SEAM`'s own doc, which says the δ field
  IS the scene seam. Now names the two constants.
- `startup_notices`'s *"Three of `prefs::Notice`'s four arms"* and
  *"two of those four arms"* — now names the arms (`WrongType`,
  `UnknownTheme`, `UnknownPreset`; `UnknownKey`, `WrongType`), which
  stay true when an arm is added. Its *"from three types"* was also
  short: `ViewerApp::new` writes a fourth source itself (the
  launch-directory sentence, which carries `; ` too). The doc now says
  the door takes `&[String]`, so nothing bounds the sources, and the
  separator guarantee rests on the door, not on the list.

**Not members, and why:** *"at four call sites"* (module header),
*"four badges each picked"* and *"The family was four members"*
(`Badge`'s docs) are past-tense counts of a history and cannot go
stale; *"the three display seams"* (module header), *"four more on
it"* (`joined_subject`), *"The four this does not word"*
(`outcome_notices`, destructured, so a new field is E0027) are each
followed by the named list that derives them; *"Three of the five
subjects"* counts this file's own enum; *"exactly three non-`Ok`
states"* (`product_badge`) names its guard,
`the_tree_still_has_exactly_the_three_states_this_policy_pairs_with`.

**What the grep could not match, and the second pass shaped at it:**
digits, and count words outside one-to-twenty (*dozen*, *pair*,
*sole*). A second grep over comment lines for digits and those words
found one more, **`// # The subject-assigning doors`' *"The dozen
writers that assign `ViewerApp::status` directly"*** — stale twice
over, since no writer assigns the field directly any more. The count
and the stale verb are gone, and the sentence is now the counterfactual
it argues from: a writer that chose its subject at its own site would
choose it where no headless row can see. What neither pass can see is a count spelled as a phrase
(*"every one of the …"*, *"the one … here"*, *"the exception"*) with
no number word at all; that is a read, not a grep. Review found two
such phrases in sentences this pass had edited (the `tool_news` pair
above), which is the blind spot landing exactly where it was named.

**Filed across the fence:** `crates/viewer/README.md`'s mirror of the
eighteen/seventeen pair. This row said it *"is filed there"*; no VDOC
row carried it, so it is filed now as
`work/vdoc/viewer-readme-counts-the-status-line-writers-by-hand`.
