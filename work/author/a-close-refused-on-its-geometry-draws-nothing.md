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

An unfinished chain whose close is refused on its geometry now walks back through `sketch::prefix_loop`, the same call a refused step and an unclosable tip take. It ends `LoopEnd::Unfinished(Some(Cut))`. The cut carries the close's own refusal as a new `PreviewError::Close { loop_, step, rendered }`: the driver's words, rendered through the `Display` it shares with `Geometry` (`loop N step M: …`), where `step` is one past the last, the row the close would go in. It is advisory (`PreviewError::is_unfinished`): nothing the author wrote is refused, and the close it names is one nobody has written.

Per member, measured on the branch:
- **A last leg onto the start point** is drawn as the close it is (`closed_on_start`), all legs, `closes: true`. It says the close's refusal, which is `SeamTangent`. Those words are wrong for a close of no length, so they are filed on the kernel's slate: `work/paths/a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`.
- **A last leg the close continues straight on from** is not a refused close any more. The provisional close is `line_to Start`, or `continue_to Start` where the close runs straight on from the last leg (`sketch::provisionally_closed`), so the chain is simply open (`Unfinished(None)`, the open chain's sentence) with every leg drawn.
- **A pending `fillet` or `arc_fillet`** draws the two legs before it and says `NoCornerOfPair` / `NoCornerForFillet`. `prefix_loop`'s guards were not what answered these: the close from each prefix ending on the pending step is itself refused (geometry, or ill-typed at `Open`). Removing `drew_only_its_leg`'s piece check turns none of these rows red; AUTH-5's fillet row still catches it.

`sketch::tests::a_close_refused_on_its_geometry_draws_nothing` is re-pinned as `a_close_refused_on_its_geometry_draws_the_legs_written`, over the member census `test_support::geometry_refused_closes`. The viewport and the profile pane each drive the same census.
