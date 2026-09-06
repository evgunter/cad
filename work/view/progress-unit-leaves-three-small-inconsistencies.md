---
id: progress-unit-leaves-three-small-inconsistencies
kind: issue
title: three small inconsistencies left by #2055: app.rs's THREE-states comment, the README's three-of-six values, and the item's missing pr field
status: open
opened: 2026-09-06
---



Three separate small things, none of them worth their own file,
grouped because a fix pass would touch all three in one edit.

**1. `app.rs`'s comment counts states two ways in three lines.**

    crates/viewer/src/app.rs:1265-1268
    // THREE states, not two, because a cancel leaves a
    // fourth thing to say: the picture is older than the
    // document AND nothing is running. A spinner there
    // would be a lie about work nobody is doing.

"THREE states … a fourth thing to say" was already awkward before
this unit; it is now ambiguous as well, because the line beneath it
reads `frame::progress(self.session.outstanding(), …)` and there are
now two different threes in scope — `Outstanding`'s
`Current | Evaluating | Canceled` and the chrome's
`Evaluating | Canceled | Indexing`. The comment predates both and was
not revisited when the call under it changed shape.

**2. The README says "the three values" after listing six things.**

    crates/viewer/README.md:329-333
    `session` itself keeps `DocSession`, its `Gesture`, `Landing`,
    `AtRestBadge`, `Outstanding`, `perform` and the operation doors.
    Those are the driver and cannot leave it: … and the three values
    are what the session says about itself, minted nowhere else.

Seven items are named and "the three values" is then used without
saying which three. They are `Landing`, `AtRestBadge` and
`Outstanding`; a reader has to infer that from `DocSession`,
`Gesture`, `perform` and the doors not being values. The sentence was
`AtRestBadge`-plus-`Landing` before the unit and adding a third
member to a list is exactly when the count stops being inferable.

Second-order: *"minted nowhere else"* is a prose invariant with no
enforcement — the variants are `pub`, and
`crates/viewer/tests/frame_policy.rs:1580-1603` constructs
`Outstanding` values directly, which is necessary and fine but means
the sentence is already not literally true of the crate.

**3. The closed item does not name the PR that closed it.**

`work/view/progress-takes-three-positional-bools.md` sets
`branch: view/progress` and `closed: 2026-09-06` but no `pr:`. The
`pr:` key exists in `work.py`'s schema and the majority of closed
items in `work/view/` carry it
(`camera-fold-clears-status-line.md`,
`boundary-rule-has-no-mechanical-check.md`,
`opoutcome-superseded-has-no-production-reader.md`,
`pick-index-built-on-ui-thread.md`,
`prune-discards-the-fault-that-explains-the-supersession.md`,
`news-and-standing-facts-are-orthogonal-axes.md` …). Lint does not
require it, so this is a convention drift and not a break — but the
item file is the record that survives the branch, and without `pr:`
its done-state does not name #2055. Two other closed items in the
same directory (`blamed-mates-lost-its-exhaustive-arm.md`,
`four-badges-five-spellings.md`) have the same gap, so it is a small
class rather than one omission.
