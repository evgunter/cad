---
id: declared-flush-intersect-refuses-in-one-operand-order
kind: issue
title: A declared flush intersect refuses JoinDesync (every chord arc separates a loose scaffolding pair) in one operand order and builds in the other
status: open
opened: 2026-10-02
priority: P2
cost: M
---

## What

Found re-authoring the tour's letterforms at natural proportions
(`show/letterforms-flush-declared`). Intersection is commutative, and
with every flush pair declared (`flush::find_flush_candidates` then
`flush::declare_all`, both operands' findings) the same two operands
build in one order and refuse `Boolean(JoinDesync { what: "every
chord arc separates a loose scaffolding pair" })` in the other. The
`JoinDesync` is raised in `crates/topo/src/boolean/join.rs` (the
`desync("every chord arc separates a loose scaffolding pair")` arm),
the class `join-desync-on-the-star-fixture` closed for the star
fixture in PR 3770; these are new witnesses on main after it.

Every operand is a straight extrude of an axis-aligned polygon, all
inside the block x∈[0,2], y∈[0,3], z∈[0,3]:

- `H`: xy sketch at z=0, extruded 3 along +z, outline (x,y)
  `(0,0) (0.5,0) (0.5,1.25) (1.5,1.25) (1.5,0) (2,0) (2,3) (1.5,3)
  (1.5,1.75) (0.5,1.75) (0.5,3) (0,3)`.
- `T`: yz sketch at x=0, extruded 2 along +x, outline (y,z)
  `(1.25,0) (1.75,0) (1.75,2.5) (3,2.5) (3,3) (0,3) (0,2.5) (1.25,2.5)`.
- `C`: zx sketch at y=0, extruded 3 along +y, outline (z,x)
  `(0,0) (3,0) (3,2) (2.5,2) (2.5,0.5) (0.5,0.5) (0.5,2) (0,2)`.

Measured with `intersect_with(a, b, &declare_all(&find_flush_candidates(a, b)?), tol)`
at `Tol::witness()`, every result checked for volume and
`validate_pseudomanifold`:

| a | b | findings | outcome |
|---|---|---|---|
| `H` | `T` | 10 | builds, V = 17/4, 3′ valid (either order) |
| `H∩T` | `C` | 15 | **refuses** `JoinDesync` |
| `C` | `H∩T` | 15 | builds, V = 11/4, 3′ valid |
| `H∩C` | `T` | 17 | **refuses** `JoinDesync` |
| `T` | `H∩C` | 17 | builds, V = 11/4 |
| `T∩C` | `H` | 13 | builds, V = 11/4 (either order) |

What did not move it: which pairs are declared has no other setting
to try here (all 15 findings are `SameOriented`, so a filter by
relation is all or none); declaring none
refuses `UndeclaredCoincidence` as it should; running
`Body::merge_coplanar_faces` on `H∩T` first finds 0 groups and
refuses the same. The same split held on a second set of proportions
(the T box z∈[1/8, 25/8] with the H and T caps overshooting by 1/4:
`(H∩T)∩C` refuses, `C∩(H∩T)` builds at 11/4).

A two-operand witness, no nested boolean: the tour's `az` letters at
natural proportions. `A` is the xy outline `(0,0) (0.625,0)
(0.8125,1) (1.1875,1) (1.375,0) (2,0) (1.125,2.5) (0.875,2.5)` with
the inner loop `(0.90625,1.4375) (1.09375,1.4375) (1,2)`, extruded
z∈[0,2]; `Z` is the yz outline `(0,0) (2.5,0) (2.5,0.4375)
(0.75,0.4375) (2.5,1.5625) (2.5,2) (0,2) (0,1.5625) (1.75,1.5625)
(0,0.4375)` at x=0, extruded 2 along +x. 8 findings, all declared:
`A∩Z` builds at the exact 38627/14336, 3′ valid; `Z∩A` refuses
`JoinDesync` with the same words. Overshooting Z's extrude in x
(x∈[-1/16, 33/16] or [-1/4, 9/4]) makes both orders build.

## Before PR 3770 (reviewer's measurement)

On main just before #3770 (`a454967521^1`), the same operands refused
in more orders: `C∩(H∩T)`, `T∩(H∩C)` and `H∩(T∩C)` also refused, and
only `(T∩C)∩H` built; `Z∩A` refused too. So #3770's strut fix (the
vertex-on-face pierce run recorded as the strut `mev_null` builds)
narrowed this class from 5 of the 6 nestings to 2 — the remaining two
are very likely the same mechanism, one shape further on. A possible
relative: `whole-orbit-fan-end-has-three-spellings` (the fan end
`next(mate(last))` in three near-copies with three whole-orbit
behaviours, and strut facing with two spellings), found in the same
PR's review.

## Where it shows

Two live `walls::wall` probes panic when this closes:

- `demos/tour/src/letterforms.rs` builds `silhouette3` as `C ∩ (H∩T)`
  and pins `(H∩T) ∩ C` (silhouette3 wall 1); the scene then takes
  whichever order reads naturally.
- `demos/tour/src/az.rs` builds `A ∩ Z` and pins `Z ∩ A` (az wall 1).
