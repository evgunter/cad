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

Separately (RESOLVED 2026-09-05 by the PROPS orchestrator's sync):
`demos/tour/Cargo.lock` was stale on `main` — along with `benches/` and
`demos/wild/`, a `profile` edge missing on the `verbs` entry since
`410d1d6cc`; every lane running `scripts/doc-gate.sh` dirtied all three.
Regenerated with cargo. The class gap stands: CI does not build the
excluded roots with `--locked`, so nothing catches the next one — CIW's.

(2026-09-05, from the vec3-doors lane, PR 1977.) `lily.rs` is this
sweep's too, on three counts, and these sites are the consumers the
doors that PR minted were minted for: `demos/tour/src/lily.rs:822` and
`:1155` build a `SketchPlane<f64>` only to read `.placement` — now
`Affine3::from_frame(origin, u, v)` directly; `:402`, `:482`, `:651`,
`:1894` and `:2102` lift a frame at the `from_frame` boundary three
times, component by component, where the choice is now per site
between `SketchPlane::map(S::from_f64)` (the `f64` normal lifted as a
value) and constructing at `S` (the normal's cross product at the
target scalar — the spellings and their difference are written at
`SketchPlane::map`); and the three struct-literal constants
`LEAF_A_BASE` / `LEAF_A_DIR` / `LEAF_A_UP` (`:697`, `:703`, `:709`) can
be `Point3::new` / `Vec3::new` now that the constructors are `const fn`.
