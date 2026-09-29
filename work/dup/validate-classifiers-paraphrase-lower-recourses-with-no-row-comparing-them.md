---
id: validate-classifiers-paraphrase-lower-recourses-with-no-row-comparing-them
kind: issue
title: topo: validate.rs's classify_* functions paraphrase lower crates' Display recourses by hand, and no row compares the two
status: closed
closed: 2026-09-29
pr: 3387
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

## Re-homed

2026-09-27, from `work/atrest/` at ATREST's close: the finding is one text spelled by hand in two places (the `classify_*` paraphrases and the lower crates' `Display` recourses), which is S-DUP's charter (CENSUS fits too, but stood at 46.5 of 30). The fix edits `validate.rs`, RESTFRONT's ground; the landing PR announces the seam.

## The question for Ev, and the recommendation

Two designers weighed it independently and recommend the same end state. It generalises what PR 3351 built for certification.

- **One table per type.** Every nested refusal type a classifier renders gets `ending(Reading)`, the sole table of that decision's recourses across the three readings (Build, AtRest, Adopt).
- **Each reader asks for its own reading.**
  - Its `Display` renders the payload only.
  - Build-route wrappers call `render(Reading::Build)`.
  - `validate`'s classifiers keep their short reason and read `ending(Reading::AtRest)`, owning a recourse only for arms that belong to no decision.
- **`Reading` moves to `geom-core`,** so `geom-brep`, `topo`, `sweep` and `step-import` can all name it.

**Why not a row comparing the two texts.** The two texts are not meant to agree. Many arms differ on purpose, because the lever differs by reading: at rest, a re-mint or a finite-offset request is a kernel or file defect. So an equality row is wrong where readings differ and unwritable where they agree, and a hand table of expected texts would be a third copy.

**Drift measured.**
- `classify_contain`'s `ArcLoopUnsupported` has shown wrong advice at rest since 2026-09-26: the variant was redefined and the classifier was not updated. That one arm is fixed now, separately, by #3388.
- About ten more arms classify their refusal differently from the type's own text (defect, not-yet, or a lever).
- Both designers note that the correct text for those arms needs a per-arm ruling, which the follow-up records once in the table.

**Ratified text.** D4 ¶1 (i) (PR 3352) already asks for this: the recourse is an exhaustive match at the decision's site. The one binding sentence that would follow is D4 naming `Reading` as kernel vocabulary.

**Cost.** `Reading` moves into `geom-core`. The follow-up touches `geom-brep`'s `OffsetFitError`, `PropsError` and `PcurveCertifyError`, `topo`'s six own enums, and about 15–20 build-route wrappers. It is reversible one type per PR.

## Closed

Ev ruled on PR 3387 (2026-09-29) against the recommendation. The texts
stay where they are. Ev asked whether the drift risk was probable. It
was measured: 25 commits in five days changed a lower refusal's
wording, and two stale at-rest texts reached main. Ev then asked
whether the change is justified by code quality, not by drift. It is
not:
- the at-rest text belongs to the checks window, which reads it;
- moving it into the lower crates teaches them about a situation they
  do not otherwise know;
- the classifiers already match exhaustively, so a new variant already
  fails to compile in `validate.rs`.
Ev agreed (*"sounds good!"*) to this instead:
- fix the two stale texts: `ArcLoopUnsupported` (#3388) and
  `classify_offset_fit`'s `Band` arm (#3403);
- correct `classify_offset_fit`'s header, which claimed its recourses
  are copies of the fit's (#3403);
- file one correctness audit for the arms where the two texts classify
  the refusal differently:
  `work/restfront/validate-classifiers-and-lower-displays-classify-refusals-differently.md`.

`Reading` stays in `geom-brep`, and D4 gains no sentence.
`CertifyError::ending` (#3351) stays as it is: there, the ending is
the band decision's own. The design-fork record is row 9 of
`docs/DESIGN-FORK-LOG.md`.
