---
id: preview-error-picks-its-tone-by-hand-in-a-comment
kind: issue
title: The profile pane's PreviewError render states the actionable rule in a comment and picks the colour by hand
status: open
opened: 2026-09-19
priority: P1
cost: E
---



**Filed here because the file has no owner.** `crates/viewer/src/pane/
profile.rs` is claimed by CHROME (dormant) and VIEW (not dispatching)
and by none of VIEW's four successors — the gap
`work/view/viewer-src-files-no-successor-claims` names. Route it when
that gap closes.

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
