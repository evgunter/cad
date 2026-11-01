---
id: preview-error-picks-its-tone-by-hand-in-a-comment
kind: issue
title: The profile pane's PreviewError render states the actionable rule in a comment and picks the colour by hand
status: closed
opened: 2026-09-19
branch: vnews/preview-error-reads-its-tone
closed: 2026-09-25
priority: P1
cost: E
pr: 3391
---



**Owner: VNEWS.** Filed first under `work/issues/`, when
`crates/viewer/src/pane/profile.rs` had no dispatching owner (the gap
`work/view/viewer-src-files-no-successor-claims` names); it is
VNEWS-claimed today, and the row moved here on 2026-09-25 (below).

Found by the sweep of `work/vnews/tone-is-a-value-in-frame-and-a-
comment-in-two-panes`, as the **re-derivation of that row's third
citation**. The row cites `crates/viewer/src/pane/create.rs:582-586`;
those lines are the `ShapeKind::Path` notation block today. The subject
moved: `49897008d` (2026-09-19) lifted the preview render into
`pane::profile`, and at the row's filing sha (`8cceb9b6ce`) the same
`PreviewError` arm stood at `create.rs:572-587`. So the third copy is
real, it is one file over, and it is NOT the same rule as the row's
other two.

**`preview_verdict`, the `Some(Err(error))` arm** (~`:173-186`) picks
its loudness by hand from a typed state, with the rule stated in the
comment above it:

> **Unfinished is not wrong.** The end-of-program arm says only that
> the chain has no closing verb yet … Every OTHER refusal blames a step
> somebody actually wrote, and keeps the colour that says so.

and draws `ui.weak(error.to_string())` for
`PreviewError::Transition { verb: None, .. }` against
`ui.colored_label(chrome(theme.unresolved), …)` for every other arm.
The `Some(Ok(drawn))` arm above it (~`:159-170`) is the same shape: a
`drawn.invalid` takes the colour, the loop count is weak.

**It is the actionable-or-not rule and it is a genuinely different
population**: the tree row's split is over a node's own refusal versus
someone else's, this one is over a refusal that blames a step the
reader wrote versus a state every chain passes through while being
written. Both are `Advisory` against `Actionable`; neither classifier
derives from the other. A fix reads `frame::Tone` from `PreviewError`
— which is `crate::sketch`'s, in the viewer — rather than from the
pane, and then this comment states the argument for the split instead
of standing in for the code.

## Re-derived (2026-09-25), and routed

Re-derived by the sweep of `resolution-and-standing-pick-their-tone-by-
hand`, which settled the neighbouring cases. **The shape has moved and
the defect has not.** `pane::profile::preview_verdict` now draws every
arm through `widgets::message_toned`, but hands it a `frame::Tone`
literal chosen per arm — `Advisory` for an open chain and for
`PreviewError::Transition { verb: None, .. }`, `Actionable` for
`drawn.invalid` and every other `PreviewError` — so the rule is still
the pane's, spelled as a `Tone` rather than as a colour. The fix is the
one that row took for `Standing`: the value the viewer already owns
(`crate::sketch::PreviewError`, and the open-chain state on
`ProfilePreview`) states the tone, and the pane reads it.

**Moved here from `work/issues/`**: `crates/viewer/src/pane/profile.rs`
is VNEWS-claimed today (`work.py territory`), and the rule is this
program's charter. `crates/viewer/src/sketch.rs`, where the tone would
live, is AUTHOR's, CHROME's and VGEOM's — a crossing to announce.

## Closed (2026-09-25): the tone lives on the preview's values

**A refusal: `PreviewError::is_unfinished()` and `PreviewError::tone()`**
(`crates/viewer/src/sketch.rs`). `is_unfinished` is the one spelling
of "a chain that ended without closing" — exhaustive over the enum, so
an arm the preview grows has to answer it — and its doc is the one
home of the argument ("unfinished is not wrong"). `preview` reads it
to decide which refusals to retry under a provisional close, and
`tone` reads it: unfinished is `Advisory`, every other refusal
`Actionable`. An unfinished chain reaches the editor as a refusal
whenever that provisional close is itself refused (a close that would
enclose nothing, as a one- or two-point chain's does; a tip with a
direction and no position; an arc arrival waiting for a binder).

**A drawn preview: `ProfilePreview::hold()` → `Option<PreviewHold>`.**
One partition of the value: an open chain first (`OpenChain`), then a
failed validation (`Invalid`), else `None`. `PreviewHold` carries its
own sentence (`Display`) and its own tone (`OpenChain` `Advisory`,
`Invalid` `Actionable`) — the shape `pane::create::Held` has with
`words`/`tone`. A value that is open AND invalid, which `preview`
never builds but whose fields are public, is the open chain: quiet,
in the open chain's words. A valid drawing has no hold and no tone.

**The pane.** `pane::profile::preview_verdict` only draws: the
sentence and tone off `drawn.hold()` or off the refusal, through
`widgets::message_toned`. The loop count under a valid preview stays a
`ui.weak` label — state, not a verdict, as
`a-verdict-drawn-outside-a-tone-has-no-value-to-read` records it, and
now nothing on the value claims a tone for it either.

**Crossing.** `sketch.rs` is AUTHOR's, CHROME's and VGEOM's; the change
there is the new `PreviewHold`, three methods, a restructured retry arm
in `preview` reading `is_unfinished` instead of re-spelling the match,
and one `use crate::frame::Tone` — the dependency `parts.rs` and
`session/refuse.rs` (also vocabulary modules) already carry.

**Receipts.** `sketch::tests` holds the mappings beside the methods
from planted values: `PreviewHold` for all four combinations of open ×
invalid (open-and-invalid included), and one `PreviewError` per arm
against a fixed `Tone` and `is_unfinished`. `pane::profile::tests`
reads the ink `preview_verdict` painted (`pane::headless::Landed::ink`)
against fixed `Voices`: previews built by `sketch::preview` (a drawn
open chain, a one-point chain, an ill-typed `tangent`, crossing
circles, a closed square), a planted `Geometry`, and a planted
open-and-invalid preview, each with whether it holds the commit. The
mutation runs are in the batch PR body.

**Sweep.** Every `Tone::Advisory` / `Tone::Actionable` literal and every
hand-drawn `theme.unresolved` / `.weak(` under `crates/viewer/src` at
this branch's base: no other site picks between the two tones per arm of
a typed value. The single-voice literals and the bare `ui.weak` counts
and states are the populations `resolution-and-standing-pick-their-tone-
by-hand` and `a-verdict-drawn-outside-a-tone-has-no-value-to-read`
already dispose of. Blind spot: a tone chosen by drawing through
`widgets::message` (the body colour) versus `message_toned` per arm —
`widgets::message(` sites, checked: every one is a tool's prompt, a
fixed instruction or a label, except the status line and the checks
window's findings, which are already items 1 and 2 of that row.
