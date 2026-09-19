---
id: datums-unit-helper-normalizes-with-a-silent-x-fallback
kind: issue
title: viewer datums.rs's local unit helper normalizes without the length question and falls back to +x silently
status: closed
opened: 2026-09-15
closed: 2026-09-16
branch: view/datums-basis
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

## Closed

Deleted, with its two call sites: `basis` is
`UnitVec3::orthonormal_basis`, which computes
`r = 1 / (1 + |n.z|) ∈ [1/2, 1]` for a unit normal and divides by no
length at all. Both failure shapes are gone by construction rather
than by argument, and the `+x` fallback with them.

**The premise this row was filed under is half right, and the half
that is wrong matters.** Transcribed verbatim and executed, the helper
does exactly what the row says: `[1e200; 3]` gives `[0.0, 0.0, 0.0]`
(the norm overflows, the fallback never fires, a zero vector is
returned as a direction) and `[1e-200; 3]` gives `[1.0, 0.0, 0.0]`
(the norm underflows, and a perfectly good `(1,1,1)` direction takes
the fallback). **Neither input can reach it on this tree.** `unit` had
exactly two call sites, both in `basis`, whose only parameter is
`UnitVec3<f64>`; that type's only mints are a decided normalize, exact
negation and exact axis/cross doors, so `n` is unit and both crosses
have components at most 2. So the zero vector never reached the paint
path — this was a defect in the helper, not in the drawing, and the
row's own body says so where the dispatch that carried it did not.

That is the reason to take the door rather than to patch the helper:
the repair a live defect would justify is a finiteness question, and
the repair an unreachable one justifies is deleting the code that
needs the argument.

**What the tests pin, and what they cannot.** The two shapes above
have no observable form once the helper is gone, so no landed row
asserts them; what lands is
`no_normal_makes_a_datum_draw_something_that_is_not_a_drawing`, which
reds when a zero basis axis reaches the paint path — checked by
planting one. It reds through the AXIS tick only: `rule_patch`'s
`span > 0.0` arm already refuses a ruling whose extent collapsed, so a
plane cannot see a zero axis and the row would be a false receipt if
it drew only planes.
