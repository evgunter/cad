---
id: no-persistent-setplacement-session-op
kind: issue
title: SessionOp vocabulary gap - no persistent SetPlacement, free-move cannot be committed as a document edit
status: open
opened: 2026-08-28
github: 1120
---

## From GitHub issue 1120

Opened 2026-08-28; 0 comments.

(GUI orchestrator) Banked from GUI-4's R1 review (n8, `review-report-r1.md` on `gui/gui-4-review-r1`): the layer-3 session vocabulary has no operation that commits an instance placement as a persistent document edit. Free-move is display-layer by ratification (G3 — never persisted), and mates supersede it — but a user who fit-probed an unconstrained instance into position has no door to KEEP that position short of authoring a mate; the shipped document layer has placement machinery (`Doc.placements`, `SetPlacement` at the editor-core level per the ASM series) that the viewer never exposes. R1's rotated-fixture row documents the workaround (author the placement through the document API directly).

Not a v1 defect — G3's scope deliberately excludes it and no ratified item demands it — but it is a real vocabulary gap a user meets on the natural path (probe → like it → want to keep it), and the review that found it is the kind of finding that evaporates without a home. A future unit (post-v1, plausibly beside GUI-6 or an ASM follow-on) decides whether the answer is exposing `SetPlacement` in the session vocabulary, a "commit this probe as a placement" affordance, or a documented no.

## Home

The session vocabulary is `crates/viewer/src/session.rs` — GUI ground, and that program is closed and may hold only closed items, so it lands under `work/issues/`.

## Un-parked — the trigger fired (2026-09-04)

`viewer-session-god-module-split` closed on 2026-09-04, so this row's
only blocker is gone and the row is dispatchable. Un-parked here, from
VIEW's PR #1857, rather than by CHROME: on Ev's ruling there, `work.py
lint` now REFUSES a `parked` row whose every blocker is closed, and a
program cannot un-park another program's rows in the PR that closes
their trigger — `work/README.md`'s one-file-one-item rule makes that a
merge conflict by design.

Whether `open` is the right status is CHROME's call, not this PR's: the
finding above argues the gap is *deliberately* out of v1 scope, which is
what the new `deferred` status is for. `open` overstates availability
and is the lesser error; re-classifying another program's row into a
status they have never used would be the greater one.

## Claimed by VIEW (VIEW orchestrator, 2026-09-04)

Claimed by `git mv` from `work/chrome/` while CHROME is dormant with
its slate landed. VIEW holds the ground on both sides of this now:
the session vocabulary (`crates/viewer/src/session/op.rs`) is this
program's after unit 1, and the item that waits on this one —
`two-hand-written-copies-of-the-g1-gesture-machine` — is this
program's too. DI5 (`docs/DOCM-IDENTITY-DESIGN.md`, ratified) is what
turns the gap into a build: the free-move commit becomes a
`DocEdit::SetPlacement`, which is exactly the door this item says the
session vocabulary lacks.

So the sequencing the G1 item states holds and now sits on one slate:
this row first, then the two gesture machines' shared rules.

Not dispatched in this wave.
