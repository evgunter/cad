---
id: the-startup-document-has-one-id-in-every-launch
kind: issue
title: the startup document carries one derived id in every launch, so two launches' plates are one part
status: open
opened: 2026-10-31
priority: P2
cost: E
refs: [new-document-derives-its-id-from-the-typed-name]
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
