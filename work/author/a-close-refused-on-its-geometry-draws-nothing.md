---
id: a-close-refused-on-its-geometry-draws-nothing
kind: issue
title: viewer: an unfinished chain whose provisional close is refused on its geometry draws nothing, not even its authored legs
status: review
opened: 2026-09-30
priority: P2
cost: M
refs: [a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at, an-unfinished-chain-awaiting-a-binder-draws-nothing]
branch: author/geometry-close
---


Found by AUTH-11's sweep and its two reviews (`author/binder-prefix`, PR 3563).

**What happens.** `sketch::preview` retries an unfinished chain under one provisional `line_to Start`. When that retry is refused as a lattice violation (an unclosable tip), AUTH-11 walks back through `sketch::prefix_loop`. When it is refused on its **geometry** (the `Some(Err(_)) => None` arm of the unfinished match in `preview`), the tip does take a `line_to`, nothing walks back, the preview is `Err` ("loop 0 never closes …"), and the viewport draws none of the authored legs. Every member below was measured on the branch; each is held as `Err` by `sketch::tests::a_close_refused_on_its_geometry_draws_nothing`, which is the row to re-pin when this is fixed.

- **A last leg onto the start point**: `at (0,0), line_to (0.01,0), line_to (0.01,0.01), line_to (0,0)`. The tip (a leg end) sits on the start, so the close is zero-length and refused. This is the unfinished-arm face of `a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at`, which covers the refused-step walk-back; the two want one answer to what a last leg onto the start means.
- **A last leg the close continues straight on from**: `at (0,0), line_to (0.01,0.01), line_to (0.02,0), line_to (0.01,0)`. The close runs on along the last leg: `JunctionTangent { margin: 0 }`.
- **A pending fillet resolved against the close**: `at, line_to (0.01,0), line_to (0.01,0.01), fillet 0.002, at (-0.005,0.02)`. The tip is a plain point, and the close resolves the fillet against a carrier nobody wrote, which does not meet the incoming ray: `NoCornerForFillet`.
- **The same with `arc_fillet`** (`sketch::fresh_step_at(ArcFillet, DirectedPoint)` in place of the fillet): `NoCornerForFillet { reason: CarriersDoNotMeet }`.

**Measured, not a member.** The collinear back-track `at, line_to (0.01,0), line_to (0.02,0)` is refused at its own step 2 (`JunctionTangent`), so it is a refused written step, not a refused close.

**Shape of a fix.** For a close refused on its geometry, walk back as a refused step does (`prefix_loop`). The walk-back already declines a prefix whose close completes a pending fillet (`drew_only_its_leg`), and it already reads a last leg onto the start as the close (`closed_on_start`). What is open is the sentence and the tip mark. The chain is unfinished rather than refused, and its end-of-program sentence says only that it does not close yet. What stops the close is its geometry (a zero-length close, a tangent junction, a fillet with no corner), which the close's own refusal names and the author's steps did not write.

Dispatched 2026-09-30 with its sibling as **AUTH-13** (`docs/AUTH-13-SPEC.md`, branch `author/geometry-close`). Both rows say they want one answer to what a last leg onto the start means, so they are one unit.

## Built (AUTH-13, `author/geometry-close`, PR 3579)

An unfinished chain whose close is refused now walks back through `sketch::prefix_loop` for any refusal, not only an ill-typed one. That is the same single call a refused step and an unclosable tip take. It ends `LoopEnd::Unfinished(Some(Cut))`. The form says the cut's refusal through `PreviewHold::Unfinished`, which is advisory whatever it carries, because the tone is read off the arm (`LoopEnd::unfinished_refusal`). Which refusal the cut carries:
- **the close's own**, when the close is refused on its geometry: `PreviewError::Geometry`, now carrying the typed `PathErrorKind` beside the driver's words, at step one past the chain's last;
- **the end-of-program refusal**, when the close is ill-typed (AUTH-11's unclosable tip), or when the author's own last leg closes the loop and only its spelling is unfinished ("the last verb has to target the start").

Per member, measured on the branch (census `test_support::geometry_refused_closes`, re-pinned from `…draws_nothing` as `sketch::tests::a_close_refused_on_its_geometry_draws_the_legs_written`):
- **A last leg onto the start point**, after an `at` entry (a triangle and a square) or a direction entry: drawn closed, saying the end-of-program refusal.
- **A last leg the close continues straight on from** is not a refused close. The provisional close (`sketch::replay_provisionally_closed`) is re-spelled the lattice's way for each decided tangency, until it replays: `continue_to` for `JunctionTangent`, and a start declared tangent for `SeamTangent`. A close that is both takes both (`continue_to` the start declared tangent). The chain is simply open (`Unfinished(None)`), every leg drawn.
- **A pending `fillet` or `arc_fillet`**: the two legs before it, saying `NoCornerOfPair` / `NoCornerForFillet`. `prefix_loop`'s guards are not what answers these: the close from every prefix ending on the pending step is itself refused. Disabling `drew_only_its_leg`'s piece check reds only AUTH-5's fillet row.

Not drawn, and filed as `work/author/a-last-leg-no-close-can-follow-is-dropped`:
- a last leg no close can follow: onto the start by a step that does not name it, reversed by every close (a cusp), or whose `line_to Start` escalates inside the ambiguity band ("too close to call", said and not re-spelled, even where `continue_to Start` would pass);
- a chain that passes through its start and goes on, which is drawn as the loop its own steps closed and says the cusp that stops its tip closing.
