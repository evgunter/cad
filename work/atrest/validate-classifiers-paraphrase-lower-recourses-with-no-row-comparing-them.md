---
id: validate-classifiers-paraphrase-lower-recourses-with-no-row-comparing-them
kind: issue
title: topo: validate.rs's classify_* functions paraphrase lower crates' Display recourses by hand, and no row compares the two
status: open
opened: 2026-09-26
priority: P3
cost: D
refs: [3269, 3294]
---

## The finding

`crates/topo/src/validate.rs`'s rendered-reason classifiers map a
nested refusal onto a short reason and a recourse, and the recourse is
written by hand as a paraphrase of the one the lower crate's own
`Display` names for the same arm. `classify_offset_fit` says so in
its own comment ("Each recourse is the one the fit's own message names
for the same arm (`geom_brep::OffsetFitError`'s `Display`), without its
numbers"). The section header above the classifiers ("The
rendered-reason CLASSIFIERS below …") states the same arrangement for
all of them: the nested sentence rides whole in the payload, and the
classifier restates its recourse.

Nothing checks that the two agree. When a lower crate's recourse
changes, the classifier keeps the old one silently. PRs 3269 and 3294
both changed `OffsetFitError`'s recourses (3294 split the budget
face's recourse on `LastRound`) and both had to find and hand-edit
`classify_offset_fit` to match. No test would have failed if they had
not.

## The class

Every `classify_*` in `validate.rs` whose arm returns a recourse
paraphrasing a lower `Display`: `classify_certify`,
`classify_offset_fit`, `classify_mass_props`, `classify_pcurve`,
`classify_contain`, `classify_chart_region`, `classify_contact_lane`,
`classify_census_cause`. (`classify_band` returns a reason only.)

## What a fix needs to decide

One of two things: a row per classifier that renders each sample arm
(`topo::test_support_samples` already enumerates them) and asserts
that the classifier's recourse is the lower message's recourse with
its numbers stripped; or a single source for the recourse, such as a
`recourse()` method on each lower error that both `Display` and the
classifier read. The second removes the paraphrase, but it reaches
into every lower crate's error type.

Not investigated beyond `classify_offset_fit`: whether the other
classifiers have already drifted from their lower `Display`s is
this row's first measurement.
