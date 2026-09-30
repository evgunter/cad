---
id: an-unfinished-chain-awaiting-a-binder-draws-nothing
kind: issue
title: viewer: an unfinished path whose tip awaits a binder (a radius arrival, a bound direction, the arrival after arc_fillet) draws nothing, though the steps before it replay
status: review
priority: P1
cost: M
design: true
refs: [path-preview-draws-nothing-for-a-refused-step]
opened: 2026-09-29
branch: author/binder-prefix
---


Found by AUTH-5 (`author/path-preview-prefix`).

**What happens.** `sketch::preview`'s unfinished arm (`crates/viewer/src/sketch.rs`, `preview`, the `refused.is_unfinished()` branch) retries a chain that ends without closing under ONE provisional `line_to Start` over the whole chain. When the tip is a state no `line_to` can leave, that retry is refused and the preview is `Err` (the end-of-program refusal), so the viewport draws nothing. The states measured to do this are the ones mid-authoring of a fused step or its binders, none of which admits `line_to` (`sketch::admits_at`): `RadiusArrival` (admits `at`, `angle`, `toward`), `RadiusArrivalAt` (`angle`, `toward`), `RadiusArrivalDir` (`at`), `Open` after `arc_fillet` (`at`, `angle`, `toward`, the close), and `Angle`. `ViaArrival` was not measured. Measured on the branch: `at, line_to (0.01,0), line_to (0.01,0.01), fillet_arc { radius, spec: Radius }` has tip `RadiusArrival` (`sketch::tip_state_at`), and picking `fillet_arc` with a radius arrival blanks the whole chain until both binders are written. The path_authoring row `an_unclosable_chain_reports_the_refusal_for_the_program_that_was_written` pins the `Angle` case of this behaviour.

**Why it was not fixed with AUTH-5.** A refused step now walks back to the longest prefix whose drawing its own steps fix (`sketch::prefix_loop`), and the same walk would draw these chains too. But it changes what an existing case says: the form now shows the specific end-of-program sentence ("loop 0 never closes — it ends with the tip a radius arrival still awaiting both binders…"), and a drawn open chain would show `PreviewHold::OpenChain`'s "its last step has to target the start", which is wrong for a tip whose next step has to be a binder. So the fix needs a decision on the sentence: keep the tip-state sentence beside a drawn prefix (a `PreviewHold` arm that carries the state), or say something else. AUTH-5's spec asked for existing cases to keep saying what they say.

**Owed:** decide the sentence, then draw the walked-back prefix for an unfinished chain as `LoopEnd::Unfinished`, and re-pin the `Angle` row.

Dispatched 2026-09-30 as **AUTH-11** (`docs/AUTH-11-SPEC.md`, branch `author/binder-prefix`). **The design call is decided in the spec:** keep the tip-state sentence beside the drawn prefix, carried typed and rendered through the kernel's `Display`. `OpenChain`'s "target the start" is false for a tip whose next step must be a binder. This is not a fork: one sentence is true and the other is not.

## Built (AUTH-11, `author/binder-prefix`)

The census comes from `sketch::admits_at`, over every state some verb has a row at. Ten tip states refuse `line_to`: `Entry`, `Open`, `Angle`, `DirectedPlain`, `DirectedIncoming`, `RadiusArrival`, `RadiusArrivalAt`, `RadiusArrivalDir`, `ViaArrival` and `ViaArrivalStart`. The row had measured five of them. Every one except `Entry` (which no step precedes) now draws the prefix `sketch::prefix_loop` walks back to, the same walk a refused step takes. That prefix is `LoopEnd::Unfinished { awaiting: Some(refusal), closes }`. The form shows that refusal's own sentence through `PreviewHold::Awaiting`, in the refusal's own tone, which is advisory. A merely open chain still shows `PreviewHold::OpenChain`. The walk-back happens only when the close is ill-typed at the tip. A close refused on its geometry is still `Err`; that case is filed as evidence on `a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at`.

The `Angle` row (`an_unclosable_chain_reports_the_refusal_for_the_program_that_was_written`) is `at, angle`. Its tip is `DirectedPlain`, not `Angle`, and nothing before it can be drawn, so it stays `Err`. The row now pins that state, and its doc no longer calls the chain a direction with no position.
