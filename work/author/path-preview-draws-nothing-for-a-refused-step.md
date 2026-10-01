---
id: path-preview-draws-nothing-for-a-refused-step
kind: issue
title: viewer: the add-profile path preview draws nothing once any authored step refuses, so the author cannot see what to fix
status: closed
opened: 2026-09-18
priority: P0
cost: M
branch: author/path-preview-prefix
pr: 3440
closed: 2026-09-30
---

Reported by Ev from the viewer (2026-09-18): "arc_fillet_arc and other such paths should try to display something even when invalid, so the user can figure out how to fix them."

**What happens.** `sketch::preview` (`crates/viewer/src/sketch.rs`) draws a chain only when it replays: fully, or, for a chain that has not closed yet (`Transition { verb: None }`), under a provisional `line_to Start`. Every other refusal returns `Err(refusal(..))` for the whole profile, and the viewport shows nothing. A fused step's refusal (`arc_fillet_arc`: a fillet radius that does not fit, an arc spec whose numbers name no arc, a corner that does not resolve) blanks every step before it as well, and those are the steps the author needs to see to judge what the failing numbers should be.

**What is owed.** When a replay refuses at step `k`, draw the longest prefix that does replay, which is `steps[..k]` under the same provisional close the unfinished-chain arm already uses, marked open. Keep the refusal sentence beside it, and mark the drawn tip as the place step `k` failed. For a fused step, the parts that DID resolve (the incoming arc, the arrival carrier) are what would show why the fillet does not fit. Drawing those needs a hook from the driver (the carriers a step bound before it refused), so it is a second, larger step. It is split out as the design item `work/round/refused-fused-step-reports-no-partial-geometry.md` and waits on that item's ruling. This item's prefix drawing does not. As with the provisional close, nothing about the lattice is re-implemented: the prefix goes through the same `replay`.

Dispatched 2026-09-29 as **AUTH-5** (`docs/AUTH-5-SPEC.md`, branch `author/path-preview-prefix`): the prefix half only. The split-out design item moved from `work/paths/` to `work/round/`; the citation above is corrected.

## Ev's ruling on the edit door (2026-09-30)

Review found that a refused edit of a COMMITTED profile flipped on something the author cannot see: refused at step 1 (nothing drawable, an `Err`) it kept the committed drawing, and refused after a leg (a prefix, `Ok`) it hid the committed loop and drew the leg. Ev chose to hide the committed shape during a refused edit, consistently: `Drafts::edited_in_place` now leaves the node out whenever the edit door took a preview, so a refusal with no prefix hides it too and the form says the refusal. The faint "ghost" of the committed shape behind the edit is the follow-up row `work/vgeom/an-edited-profile-shows-no-ghost-of-its-committed-shape.md`.

## Closed 2026-09-30 — PR 3440 merged (`5255964e`)

When a step refuses, the preview draws the longest prefix fixed by the
authored steps, a cross at the tip, and the refusal's own sentence —
Ev's `arc_fillet_arc` case draws its legs instead of nothing. `replay`
and `ReplayError` are untouched; the fused step's own carriers wait on
`work/round/refused-fused-step-reports-no-partial-geometry`.

What review changed: a pending `fillet` had been drawn resolved against
the provisional close, whatever angle the author typed; a prefix is now
accepted only when the close drew nothing but its own leg, read off the
driver's own replay record. The multi-loop `Err` ranking was made to
follow `hold`'s rule. Per Ev's ruling, a refused edit hides the
committed shape whether or not a prefix exists.

Residue: `a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at`
(the C4 claim held only for `at`-first loops),
`an-unfinished-chain-awaiting-a-binder-draws-nothing`, and the ghost on
`work/vgeom/an-edited-profile-shows-no-ghost-of-its-committed-shape`.
