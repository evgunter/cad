---
id: the-whole-app-harness-paints-before-the-evaluation-lands
kind: issue
title: The whole-app headless harness paints two frames and never waits for the evaluation, so a row can catch it still running
status: open
opened: 2026-09-28
priority: P2
cost: E
---

Seen by the `preview-error-picks-its-tone-by-hand-in-a-comment` lane
(2026-09-28): `app::properties_pane_tests::an_undeclared_parameter_is_said_once_in_the_pane`
failed under a full parallel `cargo test -p viewer --features app --lib`
and passed alone. The lane read the failing frame as one that caught an
evaluation still running.

## Why it can

`properties_pane_tests::painted_with` (`crates/viewer/src/app.rs:3187`)
assembles a `ViewerApp`, performs one batch, and paints **exactly two
frames** (`for _ in 0..2`, `:3194`), keeping the second frame's text.
Nothing in the loop waits for the evaluation that the batch started to
land. Whether the second frame shows the landed document or the
in-flight one is therefore up to the scheduler. Under load, `with` and
`without` can be painted in different states, and the row's equality
fails for a reason that has nothing to do with its subject.

That makes it a harness defect, not a defect of the one row: every row
built on `painted_with` inherits it (the harness came in with #3230).

## What a fix has to decide

Paint until the application reports nothing outstanding, with a bounded
number of frames and a loud failure when the bound is hit, rather than
a fixed two. The session already knows when an evaluation is outstanding
(`frame::Progress` / `session::Outstanding`), so the harness can read
that rather than sleep.

## A second copy of the frame body (2026-09-28)

The unit-picker unit (`vnews/the-unit-picker-reads-its-refusal`) adds a
`Driven` harness to the same test module whose `Driven::frame` repeats
`painted_with`'s frame body. Its full review found that the new rows
inherit this race: `gained_hovering` diffs a quiet frame against a
hovered frame over whole-app text, with no wait for evaluation, after
edits that request one. The race's direction is spurious reds, not
false greens, because appearing text breaks the equality and vanishing
text is filtered. The fix pass on that unit factors the two frame
bodies into one helper, so the fix this row owes lands in one place;
until then, a fix scoped to `painted_with` alone is half a fix.

## The wait already exists in one helper (2026-09-28)

The seats unit (`vnews/one-seat-line`) added `painted_with_tool` to the
same module. It paints until `app.session.landed_pair()` is `Some`,
bounded at 3000 frames with a 10 ms sleep, and asserts that the startup
document landed. That is this row's fix, for one helper. On
`vnews/batch-1` all three helpers draw through `app_frame`; what is
left is to give `painted_with` and the `Driven` harness the same
bounded wait, ideally as one function they all call, rather than a
third copy of the loop.

## What the two frames showed (2026-09-28)

The ranked-verdict unit's lane (`vnews/a-ranked-verdict-is-its-own-type`)
hit the same row failing under full-suite load, twice, and recorded the
difference: one frame painted `indexing…`, the other `evaluating…`,
`Cancel` and three `—` readouts. So the two frames are in different
phases of the startup evaluation, as this row predicts. It had filed a
second row for this (`an-undeclared-parameter-pane-row-compares-two-frames-across-an-async-state`);
that row was deleted in review as a duplicate of this one, and its one
alternative fix, comparing only the properties pane's own region, is
the one-row half-fix this row warns against.

## A fourth frame body (2026-09-28)

The add-profile unit (`vnews/the-frame-prompt-comes-first`) adds
`painted_adding_a_profile` to the same module with its own frame body,
at a 1600 by 4000 window and a set clock (`time`), so the "Add feature"
section fits and its opening animation is past. `app_frame` fixes the
window at 1600 by 1000, so this helper could not route through it as it
stands; the fix this row owes should give `app_frame` the window size
as an argument and fold this copy in with the wait.
