---
id: datums-unit-helper-normalizes-with-a-silent-x-fallback
kind: issue
title: viewer datums.rs's local unit helper normalizes without the length question and falls back to +x silently
status: open
opened: 2026-09-15
---



## Where this came from

The fix pass of `unit-vector-witness-in-geom-core` (SCALAR). That unit
converted `crates/viewer/src/datums.rs`'s `basis` to take
`geom_core::UnitVec3<f64>` — the normal it spans is unit as a property
of its type — and the very same `basis` then calls a local `unit`
helper on both cross products it builds.

## The site

`crates/viewer/src/datums.rs`, `unit` (`:743`): a hand-rolled
normalize — `(x² + y² + z²).sqrt()`, then divide — with a silent
fallback to `+x` when the length is not `> 0.0`. It asks no
finiteness question (an overflowed norm divides every component to
zero, the fallback never fires, and the zero vector goes to the paint
path as a basis axis), and an underflowed norm reads as `0.0` and
takes the fallback for a vector with a perfectly good direction. Both
are the class
`work/fix/normalize-without-the-length-question-two-more-sites.md`
names — a length divided by without the format questions the
kernel's own door (`geom_core::UnitVec3::new`, over
`decide_unit_direction`) asks first.

The doc argues the fallback is unreachable from `basis` (a unit normal
crossed with the world axis it is least aligned with has length at
least `1/√3`), and that argument is sound for a finite unit normal —
which `basis`'s parameter now is, by type. What stays open is the
helper's own shape: two call sites (`basis`, twice), a fallback that
is prose-unreachable rather than type-unreachable, and no decision
recorded. The witness's own door, `UnitVec3::orthonormal_basis`,
produces exactly what `basis` builds — a right-handed pair completing
a unit normal — with no local normalize at all; whether `basis`
becomes that door, or keeps its own seed rule for display continuity,
is VIEW's call.

## Not folded by the sweep

Display scaffolding outside the kernel's decision surface (the module
says its vectors never reach a predicate); filed rather than changed
because replacing the seed rule could move which axis a datum's grid is
drawn along, which is a rendering decision this program owns.
