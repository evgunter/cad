---
id: check-9-nesting-is-line-bounded-only
kind: issue
title: check 9's nesting half reaches only planar faces whose outer loop is a line-carrier polygon: an arc-bounded rim (every shelled vessel of revolution) still accepts a ring outside its outer loop
status: open
opened: 2026-09-13
---



Raised by `tier3-accepts-a-ring-outside-its-outer-loop`, which gave
check 9 a nesting half. That half asks the crate's one trilean
containment walk — `splitting::containment::point_in_loop` — whether
each ring vertex lies inside the region the face's outer loop bounds.
The walk's stated contract is the planar POLYGON through a loop's
vertices, so the arm gates on it: `validate::outer_loop_is_a_polygon`
admits a face only when its surface is a `Plane` and its outer loop is
a cycle of three or more half-edges carried entirely by certified
`Line`s. Outside that gate the arm is silent, and silence is the
correct direction there — the polygon through an arc-bearing loop's
vertices is not that loop's region (a disc-class loop's has zero
area), so answering from it would REFUSE valid bodies.

What the silence costs, concretely: the shape the parent item came
from. `shell::shell_open`'s rim glue picks host and guest by the
sealed arm's shell list, and on a planar line-bounded rim an inverted
pick now reds at the verb's closing `validate_geometric`. On a vessel
of revolution the rim is an annulus between two CIRCLES, so the arm's
gate is shut and the inverted pick still validates. The comment at
`shell.rs`'s `(host, guest)` assignment says exactly this and names
this row's subject as what is left.

The widening already exists one crate-module over and is not a new
walk: `boolean::contain`'s `loop_shape` classifies a loop as
`Disc`/`Parity`/`NoWalk` and its `disc_side` decides the disc class
EXACTLY (one radial margin, one decide). Check 9's nesting arm wants
`loop_shape` + `disc_side`, both private to `boolean::contain` today,
and the `NoWalk` class wants a typed silence rather than an answer —
the same posture `contfp` takes. That is a SEAM: `crates/topo/src/
boolean/` is S-BOOL's ground and `validate.rs` is this program's, so
the visibility change is announced on S-BOOL's board before it lands.

The general arc-aware parity walk (#1076) subsumes the `NoWalk` third
of this; the disc third does not wait on it.
