---
id: a-stated-sweep-blind-spot-is-never-swept
kind: issue
title: Discipline SS5 asks a lane to STATE what its sweep could not match and never to cover it, and the stated gap is where the finding is
status: open
opened: 2026-09-21
priority: P2
cost: E
needs_ev: true
---


## What

`docs/prompts/implementer-discipline.md` §5:

> If your unit fixes an instance of a class, say what pattern you swept
> with and **what that pattern could not match**. A sweep whose blind
> spot is unstated is an unverified claim, not a negative result.

The rule asks for the blind spot to be **stated**. It does not ask for
a second pass shaped to cover it. So a lane can comply perfectly,
write an honest and specific blind-spot paragraph, and ship the
finding inside it.

## The evidence: two units, one sitting, two reviewers, same shape

Both AUTHOR units dispatched on 2026-09-21 wrote §5 receipts of
unusual quality — a hit table, a disposition per hit, and a named
blind-spot paragraph. **In both, the stated blind spot is exactly
where a real finding was, and in both it took a REVIEWER to look
there.**

- **AUTH-1** (PR 2955) swept `ui.weak("` across `crates/viewer/src`,
  disposed all 17 hits, and wrote: *"`ui.weak(` cannot see a sentence
  built with `format!`, one held in a `const`, or one drawn with
  `ui.label`/`on_disabled_hover_text`."* The style reviewer ran
  `ui.label("` over the same file: 25 sites, three of them per-kind
  sentences inside a `DatumKindChoice` match arm — the exact class the
  unit existed to close, and **one of the three was added by that
  unit's own diff**.

- **AUTH-2** (PR 2957) swept unit-symbol literals, `factor()`,
  `unit_by_symbol`/`UNITS` and the `written_*` doors across
  `crates/viewer/src`, and wrote as blind spot (b): *"a unit vocabulary
  reached through a `match` on `Dimension` with no symbol text in
  it."* The style reviewer's verdict: *"The sweep's declared blind spot
  (b) is real and is where the duplication actually is"* — six such
  `Dimension` ladders in the crate, three of them added by that unit's
  own diff.

Two lanes, two different reviewers, no contact between them. Each
receipt's counts were re-derived by its reviewer and **held**; it is
not the sweeps that were sloppy. The rule got what it asked for.

## Why this is not the row beside it

`work/meta/an-items-stated-sweep-pattern-may-not-match-its-own-instance.md`
is a different defect: a pattern that fails to match the instance it
was derived FROM, so the next lane looks with the wrong instrument.
Here the pattern matches its instance correctly and the lane says
truthfully what else it cannot see. And
`work/vdoc/sweep-blind-spots-the-precheck-sweep-could-not-see.md`
preserves one sweep's blind spots as a file so they outlive a
directory — a record, not a rule.

## The proposed amendment (for Ev)

One sentence in §5, carried on the `[ev]` PR this row is flagged for.
The argument for it: **a blind spot a lane can name is a blind spot a
lane can grep**, because naming it is the hard half. `ui.label` and
"a `match` on `Dimension`" are each one command. The rule already
makes a lane do the expensive thinking and then stops one line short
of the cheap check.

The argument against, which Ev should weigh: it makes every sweep at
least two passes, and a blind spot that is genuinely unsearchable
("anything outside `crates/viewer/src`") gets a sentence saying so
rather than a pass — so the rule has to permit that answer without
letting it become the default.

## What is verified

Both instances, from two independent reviewer reports, with the
receipts re-derived. **Not verified**: whether this holds outside these
two units, or whether the sweeps' quality here is representative — two
instances in one sitting on one program is a narrow base, and the
sample is the two units I dispatched.
