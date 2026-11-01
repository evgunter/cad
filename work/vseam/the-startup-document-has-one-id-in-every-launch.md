---
id: the-startup-document-has-one-id-in-every-launch
kind: issue
title: the startup document carries one derived id in every launch, so two launches' plates are one part
status: closed
opened: 2026-10-31
closed: 2026-10-31
priority: P1
cost: E
refs: [new-document-derives-its-id-from-the-typed-name]
pr: 6145
branch: vseam/a-new-document-mints-a-random-id
---


Found by the sweep of `new-document-derives-its-id-from-the-typed-name`
for interactive doors that derive an identity. (VSEAM implementer)

`ViewerApp`'s startup (`crates/viewer/src/app.rs`, the
`scene::plate_with_hole(tol)` call in the boot path) opens on
`scene::plate_with_hole` (`crates/viewer/src/scene.rs`), which builds
`Doc::empty_derived("gui-0-plate", tol)`. Every launch therefore starts
on the same `DocumentId`. A user who edits the startup plate in one
launch and saves it at `plate.pncad`, then edits it differently in a
second launch and saves into the same folder at `plate.pncad`, gets a
resave of one id through `Workspace::save_at`: no refusal, and the
first drawing is overwritten. That is the case `NewDocument`'s random
id closed, reached through the startup door instead.

`plate_with_hole` is also the fixture many viewer tests and the gpu
row build on, so they want its derived id. The startup path, not the
fixture, is what should mint. One way to do that is to re-identify
the booted document under `pncad::workspace::random_document_id()`
at the boot. Another is to boot onto an empty `NewDocument`. Either
one has to surface `RandomnessUnavailable` as a startup error.
`scene.rs` is chrome/polish/vgeom ground; the boot in `app.rs` is
VSEAM's.

## Closed (2026-10-31): the startup plate mints its id

Re-banded P1 by the VSEAM orchestrator with the style reviewer: one
creating door minted and the other did not, which is a special case
that should be handled uniformly. Fixed in the same PR as
`new-document-derives-its-id-from-the-typed-name`.

`ViewerApp::assemble` now re-identifies the booted plate under
`pncad::workspace::random_document_id()`. A refused entropy source
fails startup as the new `StartupError::Identity`, like the boot's
other failures. That arm is untested because `getrandom` has no
injection seam, and the site says so. `scene::plate_with_hole`, the
shared fixture, still derives its id.

`app.rs` `two_startups_mint_two_ids_and_the_second_refuses_the_firsts_file`
pins the fix. The one test that relied on two startups sharing an id,
`a_save_beside_the_original_puts_its_choice_on_the_toolbar`, now
opens the original's file in its second app instead.
