---
id: an-unfinished-chain-awaiting-a-binder-draws-nothing
kind: issue
title: viewer: an unfinished path whose tip awaits a binder (a radius arrival, a bound direction, the arrival after arc_fillet) draws nothing, though the steps before it replay
status: review
priority: P1
cost: M
refs: [path-preview-draws-nothing-for-a-refused-step]
opened: 2026-09-29
branch: author/binder-prefix
pr: 3563
---


Found by AUTH-5 (`author/path-preview-prefix`).

**What happens.** `sketch::preview`'s unfinished arm (`crates/viewer/src/sketch.rs`, `preview`, the `refused.is_unfinished()` branch) retries a chain that ends without closing under ONE provisional `line_to Start` over the whole chain. When the tip is a state no `line_to` can leave, that retry is refused and the preview is `Err` (the end-of-program refusal), so the viewport draws nothing. The states measured to do this are the ones mid-authoring of a fused step or its binders, none of which admits `line_to` (`sketch::admits_at`): `RadiusArrival` (admits `at`, `angle`, `toward`), `RadiusArrivalAt` (`angle`, `toward`), `RadiusArrivalDir` (`at`), `Open` after `arc_fillet` (`at`, `angle`, `toward`, the close), and `Angle`. `ViaArrival` was not measured. Measured on the branch: `at, line_to (0.01,0), line_to (0.01,0.01), fillet_arc { radius, spec: Radius }` has tip `RadiusArrival` (`sketch::tip_state_at`), and picking `fillet_arc` with a radius arrival blanks the whole chain until both binders are written. The path_authoring row `an_unclosable_chain_reports_the_refusal_for_the_program_that_was_written` pins the `Angle` case of this behaviour.

**Why it was not fixed with AUTH-5.** A refused step now walks back to the longest prefix whose drawing its own steps fix (`sketch::prefix_loop`), and the same walk would draw these chains too. But it changes what an existing case says: the form now shows the specific end-of-program sentence ("loop 0 never closes — it ends with the tip a radius arrival still awaiting both binders…"), and a drawn open chain would show `PreviewHold::OpenChain`'s "its last step has to target the start", which is wrong for a tip whose next step has to be a binder. So the fix needs a decision on the sentence: keep the tip-state sentence beside a drawn prefix (a `PreviewHold` arm that carries the state), or say something else. AUTH-5's spec asked for existing cases to keep saying what they say.

**Owed:** decide the sentence, then draw the walked-back prefix for an unfinished chain as `LoopEnd::Unfinished`, and re-pin the `Angle` row.

Dispatched 2026-09-30 as **AUTH-11** (`docs/AUTH-11-SPEC.md`, branch `author/binder-prefix`). **The design call is decided in the spec:** keep the tip-state sentence beside the drawn prefix, carried typed and rendered through the kernel's `Display`. `OpenChain`'s "target the start" is false for a tip whose next step must be a binder. This is not a fork: one sentence is true and the other is not.

## Built (AUTH-11, `author/binder-prefix`, PR 3563)

The predicate is an **unclosable** tip: one no `line_to` leaves, so the provisional close is ill-typed there. "Awaiting a binder" was too narrow: `DirectedPlain` and `DirectedIncoming` have their binders and await a leg. The census comes from `sketch::admits_at` over the kernel's census of states (`profile::test_support::every_state`, less the finished `Closed`). Ten states are unclosable: `Entry`, `Open`, `Angle`, `DirectedPlain`, `DirectedIncoming`, `RadiusArrival`, `RadiusArrivalAt`, `RadiusArrivalDir`, `ViaArrival` and `ViaArrivalStart`. The row had measured five.

An unfinished chain at such a tip draws its prefix when one exists, through the same `prefix_loop` call a refused step takes, and ends `LoopEnd::Unfinished(Some(Cut))`. `Cut { refusal, closes }` is shared with `LoopEnd::Refused(Cut)`. The form says that tip's end-of-program refusal through `PreviewHold::Refusal` (renamed from `Refused`), in the refusal's own tone, which is advisory. A merely open chain still says `PreviewHold::OpenChain`. `Entry`, and a chain like `at, angle` with nothing drawable before its tip, stay `Err`.

A close refused on its **geometry** is still `Err`. That class is filed as `a-close-refused-on-its-geometry-draws-nothing` and held by `sketch::tests::a_close_refused_on_its_geometry_draws_nothing`.

The ways into each state are the kernel's own. `profile::test_support::way_in` now serves both censuses; `arc_spec_census.rs`'s `prefix` is rebuilt from it, and every program is byte-identical.

The `Angle` row (`an_unclosable_chain_reports_the_refusal_for_the_program_that_was_written`) is `at, angle`. Its tip is `DirectedPlain`, not `Angle`, and nothing before it can be drawn, so it stays `Err`. The row now pins that state.
