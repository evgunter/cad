---
id: prose-digits-are-records-nothing-reconciles
kind: issue
title: ci.yml's env block argues against itself about restating a pin in prose
status: open
opened: 2026-09-11
refs: [pinned-version-named-in-present-tense-prose, 2327]
---

Raised by the style review of PR 2327 as a challenge to that unit's repair,
and filed rather than argued away: **the orchestrator ruled (2026-09-11) that
PR 2327's edits stand** — the item's position that the fix is a tense, and that
no checker can tell a record from an assertion, is ratified, and the
correctness lane verified all seven rewrites falsified no record. What the
challenge identifies is a gap that survives the ruling.

**The objection.** Rewriting *"measured against **the pinned** 0.9.140"* to
*"measured against **cargo-nextest** 0.9.140"* does not remove the restated
digit; it changes the digit's grammar. The old spelling read as visibly false
the day the pin moved, which at least invited a reader to check. The new one
reads as a durable record, so a reader will not think to.

**The finding is not any one site — it is the file arguing against itself.**
Inside one 40-line block of `.github/workflows/ci.yml`:

- `:316` states the convention outright, for `SCCACHE_VERSION`: *"The version
  is named once, here, rather than restated in the sentence about it."* Both
  reviewers flagged this line independently.
- `:219`, `:322`, `:328` and `:334` each restate it anyway — the nextest
  archive-vehicle paragraph, and the maturin / `ty` / ruff adoption records.

So after PR 2327 the tree's position is: *a prose digit is a record, records
must not track a bump, and one comment in the source-of-truth file says
records should not carry the digit at all.* Those cannot all be the rule.

**What it would take to settle it**, and why it is not a lane's call:

1. Ratify `:316`'s convention and apply it — every adoption annotation loses
   its digit and keeps its date and its argument. That is a real loss (the
   reader who wants "which version was this argued about" must read a git log)
   and it is the only shape a checker could ever enforce, because "no digit in
   a comment adjacent to the pin" is mechanical where "record or assertion" is
   not.
2. Ratify the other side — digits stay, records are records — and delete
   `:316`'s sentence, which is then the one line out of step.
3. Neither, with this file as the record that the tension was seen.

(1) and (2) are both edits to how this repo writes about its pins, which makes
it a convention question rather than a defect: it wants a sitting decision, not
a sweep.
