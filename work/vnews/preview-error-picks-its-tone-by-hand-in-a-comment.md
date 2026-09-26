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

**`PreviewError::tone()`** and **`ProfilePreview::tone()`**
(`crates/viewer/src/sketch.rs`), total like `Standing::tone`.
`PreviewError::tone` is exhaustive over the enum with no wildcard, so an
arm the preview grows has to answer it: `Transition { verb: None, .. }`
— a chain that ended without closing, the state every chain passes
through while it is written — is `Advisory`; every other arm blames
something somebody wrote and is `Actionable`. `ProfilePreview::tone` is
`Actionable` exactly when `invalid` is `Some`, and `Advisory` for an
open chain and a valid drawing. The argument the pane's comment carried
("unfinished is not wrong") is now the doc of the method that decides
it; the two tones agree that a chain being written is quiet whether or
not it could be drawn.

**The pane.** `pane::profile::preview_verdict` picks the SENTENCE per
arm and reads the tone off the value it drew it from (`drawn.tone()`,
`error.tone()`), then draws once through `widgets::message_toned`. The
loop count under a valid preview stays a `ui.weak` label — a count, not
a verdict, as `a-verdict-drawn-outside-a-tone-has-no-value-to-read`
records it.

**Crossing.** `sketch.rs` is AUTHOR's, CHROME's and VGEOM's; the change
there is two additive methods and one `use crate::frame::Tone`, the
same dependency `parts.rs` and `session/refuse.rs` (also vocabulary
modules) already carry.

**Receipts.** `pane::profile::verdict_tests`: three rows read the ink
`preview_verdict` painted (`pane::headless::Landed::ink`) against fixed
`Voices`, over previews produced by `sketch::preview` itself (a drawn
open chain, a one-point chain, an ill-typed `tangent`, two crossing
circles, a closed square) plus a planted `Geometry`, and check whether
each holds the commit; a fourth plants one `PreviewError` per arm and
holds `tone()` against a fixed `Tone`. The mutation runs are in the
batch PR body.

**Sweep.** Every `Tone::Advisory` / `Tone::Actionable` literal and every
hand-drawn `theme.unresolved` / `.weak(` under `crates/viewer/src` at
this branch's base: no other site picks between the two tones per arm of
a typed value. The single-voice literals and the bare `ui.weak` counts
and states are the populations `resolution-and-standing-pick-their-tone-
by-hand` and `a-verdict-drawn-outside-a-tone-has-no-value-to-read`
already dispose of. Blind spot: a tone chosen by drawing through
`widgets::message` (the body colour) versus `message_toned` per arm —
grep `widgets::message(` sites, checked: every one is a tool's prompt, a
fixed instruction or a label, except the status line and the checks
window's findings, which are already items 1 and 2 of that row.
