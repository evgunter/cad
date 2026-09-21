---
id: positive-finite-predicate-has-six-homes-outside-datums-rs
kind: issue
title: The positive-finite predicate has six more hand-spellings across crates/viewer
status: open
opened: 2026-09-21
priority: P2
cost: E
---


## Finding

Found by the sweep that closed
`four-spellings-of-one-finiteness-predicate-in-datums-rs`, which gave
`crates/viewer/src/datums.rs` a `positive_length` door and routed its
three sites through it. **The same question is asked in five more
files, in five more spellings**, none sharing a home with each other
or with the new one:

| site | spelling |
| --- | --- |
| `input.rs`, `Orbit::world_per_px` | `!(viewport.height_px.is_finite() && viewport.height_px > 0.0)` |
| `app.rs`, `features_fraction` | `!wanted.is_finite() \|\| !(stack.is_finite() && stack > 0.0)` — on `f32`, and the two arguments get different questions |
| `sketch.rs`, the 2-D unit-vector door | `(length.is_finite() && length > 0.0).then(…)` |
| `scene.rs`, `DisplayTolerance::new` | `!delta.is_finite() \|\| delta <= 0.0` — the negation, spelled the other way round |
| `scene.rs`, the normal normalizer | `len > 0.0 && len.is_finite()` — the operands reversed |
| `camera.rs`, the direction-length guard | `!(len.is_finite() && len > 0.0)` |

A seventh site asks a NEARBY question and is listed so a sweep does
not read it as a member: `camera.rs`'s perspective-divide guard is
`out[3].is_nan() || out[3] <= 0.0`, which ADMITS an infinite `w` where
the six above refuse one. Whether that is deliberate is the reading a
lane owes before it routes the site anywhere.

## Why it is a row and not a line in a PR body

`datums.rs`'s row was filed because a sweep that closed two defects
had just ADDED two spellings to it, and the prediction was that the
next lane would add a fifth. The same pressure is on the rest of the
crate and nothing in it has a door yet. The `f32` site is the one that
makes this more than a tidy-up: `datums.rs`'s `positive_length` takes
`f64`, so a crate-wide door is either generic or is two doors, and
that is a design call rather than a find-and-replace.

## Fence

`crates/viewer/src/{input,app,sketch,scene,camera}.rs`. Every one of
them is claimed by chrome and by at least one of view, vgeom, fit,
vseam and author (`python3 scripts/work.py territory --files -`), so a
lane taking this owes an awareness pass rather than an exclusive.

`crates/viewer/src/datums.rs` is NOT in scope: it has its door.
