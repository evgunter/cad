---
id: hand-maintained-counts-in-frame-rs-prose-have-no-guard
kind: issue
title: Five prose counts in frame.rs are maintained by hand, one went stale in a day, and nothing reds for any of them
status: open
opened: 2026-09-20
priority: P4
cost: E
rides_with: frame-rs-says-the-per-subject-line-is-a-question-for-ev
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
