---
id: opoutcome-superseded-has-no-production-reader
kind: issue
title: OpOutcome::superseded has no production reader — a discarded free-move probe is silent in the GUI
status: open
opened: 2026-09-04
refs: [viewer-session-god-module-split]
---


Found by the whole-file read that opened
`viewer-session-god-module-split` (2026-09-04).

## What happens

`OpOutcome` carries four fields (`crates/viewer/src/session.rs:1469`).
The application reads exactly one of them: `app.rs:1745` takes
`.refusal` and nothing else. `committed` is read by 21 test files;
`previewed` likewise; **`superseded` has no reader in `src/` at all**
— its only observers are `crates/viewer/tests/assembly_display.rs:607`
and `crates/viewer/tests/assembly_walk.rs:212`.

It is set on the paths where a free-move probe is discarded — an undo
(`session.rs:2700`) and a commit (`session.rs:3081`). So the one thing
the field exists to report, that the user's in-flight probe was thrown
away by something else they did, reaches the tests and never reaches
the user.

## Why this is a finding and not a nit

D-level fail-loud is the project's standing posture, and this is a
value the session computes correctly, hands to the GUI, and the GUI
drops. The status line is right there and already has a vocabulary for
it (`frame::StatusUpdate`). Whether a supersession deserves the line,
a badge, or nothing is a real question — but "nothing, undocumented,
while the type still promises it" is the one answer that cannot be
right, because the promise is what makes the next reader trust it.

It also constrains any split that moves `OpOutcome`: `refused` is
private and `Default` is derived, so a test cannot construct a refusal
outcome and must go through `perform`. That is a good property and the
split must not lose it.

## Home

VIEW's: `crates/viewer/src/session.rs` and `app.rs`. Rides unit 1's
ground; not unit 1's fix, since deciding what a supersession is worth
is a chrome question rather than a module-boundary one.
