---
id: the-strut-cover-on-cylinder-pairs
kind: issue
title: The strut source of the one-sided cover admits plane x cylinder only; the cylinder x cylinder row has a measured witness and waits on INTENT stage 4, and sphere x cylinder stops at later doors
status: parked
opened: 2026-10-02
priority: P1
cost: M
blocked_on: [intent-stage4-is-built]
---


## What

`tangency_certifies_side` (`crates/topo/src/boolean/mod.rs`) is the one
table for which tangencies along a curve give the one-sided cover a
global side. Its seam column admits plane × cylinder, two cylinders,
sphere × cylinder, and sphere or plane × torus. Its strut column (an
operand edge described `TangentIntersection`) admits plane × cylinder
only.

PR 3849's first fix pass widened the strut column to the whole table.
Its delta review restored the old strut table and measured the effect:
zero outcomes changed over 3,992 tests. The reviewer's two probes, a
strut between two cylinder walls and a strut between a sphere and a
cylinder (the one-profile capsule's joint), refuse with and without the
widening. So the widening was unproven and unexercised, and it was
reverted (fix pass 2).

## The cylinder × cylinder witness (measured, PR 4400)

Two plates stacked `z ∈ [0, 1]` and `[1, 2]`, with every flush finding
declared (the mating plane `Rest`, the walls continuations), as
`reach_continuation`'s rounded stack is. The lower outline is a big arc
(r 2 about the origin, −60° to 60°) running into a small arc (r 0.5)
tangent to it. The small arc is authored as the big one's tangent
continuation (`.tangent().tangent_arc_to`), so the ruling between the
two walls is `TangentIntersection`. There are three stacks:

| Stack | Main, declared, both orders | With the row | Closed-form volume |
|---|---|---|---|
| small arc inside the big circle, on itself | `CurvedPierceUnsupported` | builds, tier 3/3′ | 18.30553352911589 |
| small arc outside it (an S), on itself | `CurvedPierceUnsupported` | builds, tier 3/3′ | 21.234236980856195 |
| inside joint under a plate cut back to the big arc (no strut of its own) | `CurvedPierceUnsupported` | builds, tier 3/3′ | 18.17168398826653 |

- Each build matches its closed form to 1e-12 relative, at ε default,
  1e-6 and 1e-12. It also meets about 900 point probes per stack and
  order: the winding number of its watertight mesh, checked against the
  outline's own polyline.
- Subtract and intersect: `CurvedPierceUnsupported` on main. With the
  row they give `FallbackExtentUnsupported`, the rounded stack's door
  (`reachhold/rounded-stack-subtract-and-intersect-refuse-fallback-extent`).
- Undeclared, every stack refuses `UndeclaredCoincidence`
  (`carrier_cyl_axis_parallel`) before any cover is built.
- The cut-back stack needs both directions the row certifies. Each
  direction dropped alone reds it. The stacks of one outline build on
  either direction alone, because each plate's strut covers the other's.

`crates/sweep/tests/strut_cover_on_cylinder_pairs.rs` pins main's
outcome (`the_arc_joint_stacks_refuse_at_the_crossing_layer`). It also
holds the witness as `#[ignore]`d rows, which pass with this diff
applied:

```diff
             TangencySource::Strut => {
-                matches!((parent, partner), (Plane, Cylinder) | (Cylinder, Plane))
+                matches!(
+                    (parent, partner),
+                    (Plane, Cylinder) | (Cylinder, Plane | Cylinder)
+                )
             }
```

The same change rewords the table's doc, its unit row
(`only_tangencies_that_are_a_global_side_certify_one`: the strut list
gains `(Cylinder, Cylinder)`) and `crates/topo/README.md`'s "Which
tangencies count". With the row applied, the pin goes red and the
ignored rows pass: `cargo nextest run -p sweep --run-ignored all -E
'test(strut_cover_on_cylinder_pairs)'`.

## Why it waits (D10)

- The strut certificate itself reads no declaration: `tangent_struts`
  reads an edge's `TangentIntersection` description and its faces'
  kinds.
- But `DeclaredPairs::build` consults the strut column only for a
  partner face in `verified.one_carrier`. Only
  `verify_declared_contacts` fills that set, from a declared `Rest` or
  continuation pair.
- So the row would widen what the declared path builds, and the D10
  hold forbids that. The existing plane × cylinder row predates the hold
  and does not license extending it (TANG orchestrator's ruling on PR
  4400).
- Stage 4 replaces the declared seat with Zero glue
  (`booleans-glue-on-zero`, `declared-pairs-retire`). The row lands with
  or after it, read from a value-decided one-carrier verdict, and the
  pin flips to the build.

## Where the sphere × cylinder probes stop

The one-profile capsule (tube `z ∈ [0, 2]`, radius 0.5, half ball on
it, the joint authored as the side's tangent continuation), unioned with
a coaxial rod of its radius whose wall is declared a continuation of
the capsule's, in both member orders:

| Rod | Main | Strut column with the sphere row |
|---|---|---|
| `z ∈ [−1, 2]`, ending on the joint | `CurvedPierceUnsupported` | `Escalated { Coincidence(Sectors, Moot) }` (capsule first), `CurvedPairUnsupported { site: InteriorLoopGuard }` cylinder × sphere (rod first) |
| `z ∈ [−1, 2.25]`, into the half ball | `CurvedPierceUnsupported` | `CurvedBooleanUnsupported` (the join's lane dispatch) |
| `z ∈ [1, 3]`, over the half ball | `CurvedPierceUnsupported` | `CurvedBooleanUnsupported` |
| `z ∈ [2, 3]`, from the joint | `CurvedPierceUnsupported` | `CurvedBooleanUnsupported` |
| `z ∈ [−1, 1]`, short of the joint | builds | builds |

- Undeclared, each refuses `UndeclaredCoincidence`.
- `CurvedBooleanUnsupported` is
  `join/cylinder-sphere-germ-pair-has-no-join-lane`'s door.
- Pinned by `the_capsules_strut_waits_at_the_crossing_layer`. The torus
  rows were not probed.

## The work

Once stage 4 lands: apply the diff above, un-ignore the witness rows and
retire the pin. For the sphere row, find a union that needs the strut
cover on sphere × cylinder once the doors above move, and widen the row
with it as its witness.
