---
id: tour-scenes-lift-componentwise-not-through-map
kind: issue
title: The tour's other scenes lift f64 literals componentwise (az, letterforms, bool_bodies, cutaway, curvedcut, twopeg, paths, bossplate, bodies) — lily.rs is the worked example of the layer rule; demos/tour/Cargo.lock is stale on main
status: open
opened: 2026-09-05
---


(PROPS orchestrator) From the lily-vec3 lane (PR #1954), outside its
fence (`demos/tour/src/lily.rs` only). The layer rule the lily's header
now states — scene data composed at `f64` in the kernel's vector types,
lifted once at the door through `map(S::from_f64)` — is unfixed
elsewhere in the tour: componentwise `S::from_f64` lifts in `az.rs`
(6), `letterforms.rs` (9), `bool_bodies.rs` (3), `cutaway.rs` (2),
`curvedcut.rs` (2), `twopeg.rs`, `paths.rs`, `bossplate.rs`,
`bodies.rs` (1 each); `diechamfer.rs:100` reads points back into
tuples only because `Point3` is not `Ord`. A style sweep (Track X
ground, `demos/`), `lily.rs` the pattern to copy.

Separately: `demos/tour/Cargo.lock` is stale on `main` — any tour build
adds a `profile` edge to the `pncad` entry; CI does not build the tour
with `--locked`, so nothing catches it. Needs an owner (CIW or the
program that last touched the tour's dependencies).
