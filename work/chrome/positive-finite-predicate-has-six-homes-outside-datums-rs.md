---
id: positive-finite-predicate-has-six-homes-outside-datums-rs
kind: issue
title: Both of datums.rs's finiteness doors have hand-spelled siblings across crates/viewer
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

`camera.rs`'s perspective-divide guard was first listed here as a
seventh, near-miss site. It is not a member and it is not a
duplication finding: it is a live substitution defect behind a `pub`
door, and it has its own row —
`camera-project-answers-with-a-screen-position-for-a-projection-that-overflowed`.

## The other door's shape, swept second

The unit that filed this row built TWO doors and this row was first
filed for one of them. `all_finite`'s shape — *are these numbers*,
with no sign question — has its own population in the same crate,
found by the same grep and listed here so the two are taken together:

| site | spelling |
| --- | --- |
| `camera.rs`, the bounds guard | `lo.iter().chain(hi.iter()).any(\|v\| !v.is_finite())` — `all_finite` over a slice, negated |
| `sketch.rs`, the drawable-point test | `point[0].is_finite() && point[1].is_finite()` — `all_finite([a, b])` verbatim |
| `sketch.rs`, the arc door | `!(radius.is_finite() && theta.is_finite() && chord.is_finite())` — the same over three |
| `props.rs`, `display.rs`, `scene.rs` (`delta * MM_PER_METRE`) | single-value `!x.is_finite()` guards — members only if the door is worth routing one value through |
| `bounds.rs`, the integral seed | `seed.is_finite() && seed != 0.0` — a DIFFERENT question (finite and non-zero, no sign), listed so it is not swept in as one of the above |
| `scene.rs`, the diagonal | `if diagonal.is_finite() { diagonal } else { 0.0 }` — not this class at all: a SUBSTITUTION, and evidence on `viewer-substituted-value-class-is-crate-wide` |

A lane taking this owes a decision on the single-value guards before
it starts: routing one value through an array door may be worse than
the `is_finite` call it replaces, and if so the population is the
three multi-value sites and the row should say so.

## Why it is a row and not a line in a PR body

`datums.rs`'s row was filed because a sweep that closed two defects
had just ADDED two spellings to it, and the prediction was that the
next lane would add a fifth. The same pressure is on the rest of the
crate and nothing in it has a door yet. The `f32` site is the one that
makes this more than a tidy-up: `datums.rs`'s `positive_length` takes
`f64`, so a crate-wide door is either generic or is two doors, and
that is a design call rather than a find-and-replace.

## Fence

`crates/viewer/src/{input,app,sketch,scene,camera,props,display,bounds}.rs`. Every one of
them is claimed by chrome and by at least one of view, vgeom, fit,
vseam and author (`python3 scripts/work.py territory --files -`), so a
lane taking this owes an awareness pass rather than an exclusive.

`crates/viewer/src/datums.rs` is NOT in scope: it has its door.
