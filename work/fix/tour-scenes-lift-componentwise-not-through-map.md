---
id: tour-scenes-lift-componentwise-not-through-map
kind: issue
title: The tour's other scenes lift f64 literals componentwise (az, letterforms, bool_bodies, cutaway, curvedcut, twopeg, paths, bossplate, bodies) — lily.rs is the worked example of the layer rule; demos/tour/Cargo.lock is stale on main
status: review
opened: 2026-09-05
branch: fix/tour-scenes-lift
pr: 2341
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

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/code-quality/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). A Track X style sweep over `demos/tour/src/*` (the body says so); X's ground is code-quality's own to dispatch and its rows live there, so this row takes `track: X` and waits for a lane the way `D403` does. Not enough on X today to open a program for it (three rows, one parked on `L2`).

## Claimed by FIX (2026-09-11)

`demos/tour/src/*` is in no open program's `paths`, and this row is a
one-PR sweep whose fix is written (`lily.rs` is the worked example) —
FIX's charter exactly. Moved here from `work/code-quality/`, id
unchanged; the `track: X` key leaves with the directory, since it is a
code-quality-only field and the row is no longer waiting on a Track X
lane. `demos/tour/Cargo.lock`'s staleness is already resolved above and
is not part of this unit; the `--locked` gap it names stays CIW's.

## What landed

The sweep, re-measured on the merge base rather than taken from the
filing. The population is **31 componentwise constructor lifts in 9
files**, not the 26 in 8 the title lists: the filing's grep was
single-line, so it missed every `Point3`/`Vec3`/`Point2`/`Vec2::new`
rustfmt had broken across lines — three more in `twopeg.rs`, one more
each in `bossplate.rs` and `curvedcut.rs`, and the whole of
`crosslap.rs`. `paths.rs` does not exist and never did; the title's
list is wrong about it.

All 31 now go through `pncad::authoring`'s `p2`/`v2`/`p3`/`v3`, which
are the same expression the sites spelled by hand. `curvedcut.rs` also
carried a surviving per-scene `p2` closure of the kind
`crates/pncad/src/authoring.rs`'s header says were deleted; it is gone
in favour of the seam's own.

`lily.rs`'s three counts: the two `SketchPlane::from_frame(..).placement`
sites are `Affine3::from_frame` directly; the two frame lifts at the
`from_frame` boundary are `SketchPlane::from_frame(..).map(S::from_f64)`
— the frame lifted once as a value, which is what this file's own layer
rule says an already-composed `f64` frame does; and the three
`LEAF_A_*` struct literals are `Point3::new`/`Vec3::new`, which are
`const fn` (`crates/geom-core/src/linalg/point.rs:152`,
`vec.rs:144`).

`main.rs`'s crate doc now states the layer rule and points at `lily`'s
full statement of it, because the rule holds corpus-wide and a reader
of `az.rs` had no pointer to it.

Nothing moved: the release render's whole output tree is byte-identical
to the merge base's (every STL, STEP, `scenes.json` and `uv/` chart),
and so is the k-probe sweep's 1 590 255-sample CSV.

Not fixed, and why: `diechamfer.rs:100` still reads points into tuples.
`Point3<f64>` has no `PartialOrd`, so there is no key to sort a point
set by, and `Vec3` has no sup-norm door for the Chebyshev gap
`feet_agreement` measures. Both are library gaps; the site stays until
one of them closes.
