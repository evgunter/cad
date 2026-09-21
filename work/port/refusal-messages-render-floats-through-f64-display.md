---
id: refusal-messages-render-floats-through-f64-display
kind: issue
title: Refusal messages render f64 knots and weights through Display, so 1e308 prints as 309 digits
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## Finding

`f64`'s `Display` prints a large-magnitude value in full positional
notation: `1e308` comes out as a 309-digit decimal. Every typed refusal
in the spline stack that names a knot, a weight or a parameter
interpolates it with a bare `{}`, so a refusal about a domain near the
`f64` ceiling renders as a wall of digits rather than as a number a
reader can see.

The class, as of this filing:

- `crates/geom-core/src/spline/algebra.rs`, `KnotAlgebraError`'s
  `Display`: `{u}` at four arms (parameter not strictly inside the
  domain; insertion exceeding the interior budget; not an interior
  knot; removal exceeding multiplicity).
- `crates/geom-core/src/spline/knots.rs`, `SplineError`'s `Display`:
  `{weight}` at `NonPositiveWeight` and `NonFiniteWeight`.
- `crates/geom/src/surfaces/nurbs.rs`, `KnotMirrorError`'s `Display`:
  `{lo}`, `{hi}`, `{knot}`, `{mirror_knot}` — `ReflectionNotFinite` on
  a `[1e308, 1.5e308]` domain is the worst case in the tree today,
  since that variant EXISTS to report a domain at the ceiling.

## Why it is filed rather than fixed

VREV's fix pass (PR 2627) was asked to make `KnotMirrorError` follow
the crate's convention for rendering floats in a refusal. Grepping the
two neighbouring error types found the convention IS the bare `{}`, so
matching it and fixing it are the same edit at three sites in two
crates — a class, not a variant. `KnotMirrorError` was left matching
its neighbours.

## What the fix looks like

One rendering helper in `geom-core` (`{:e}` above some magnitude, plain
below, so ordinary knots in `[0, 1]` read exactly as they do now) and
the three `Display` impls through it. The rows that assert on message
text (`KnotAlgebraError`'s and `KnotMirrorError`'s) move with it, and
the new spelling is what they then pin.

## Was

Filed by SCALAR's VREV fix pass (PR 2627).
