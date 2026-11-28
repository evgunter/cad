---
id: a-sym-posed-reading-carries-a-value-its-node-does-not-compute
kind: issue
title: Sym's posed readings (difference_dot, harmonic_belly) carry a value their DAG node does not compute, so revalue drifts on every plane offset
status: open
opened: 2026-11-23
priority: P1
cost: M
design: true
refs: [main-red-revalue-corpus-carrier-and-witness-on-surface-drift-since-7411]
---


## The finding

(Filed by GREEN's U19 lane, 2026-11-23.) `Sym<T>`'s two posed readings
(`crates/geom-core/src/sym.rs`, `impl Real for Sym<T>`: `fn difference_dot`
and `fn harmonic_belly`) build their node from the plain formula
(`plain_difference_dot`, `plain_harmonic_belly`) and take their value
from `T`'s posed reading: at `Sym<f64>`, Ogita–Rump–Oishi `Dot2` and the
exact-numerator belly. The node is the same real function, but it is not
the computation of the value. `sym::revalue::revalue` re-values a node
"op by op as `Sym<f64>` performs it, so a DAG that IS the computation
re-values to the `f64` build's own bits" (its doc). These two nodes
re-value to the plain formula's rounding, which differs from the value
in the last bits wherever the posed reading recovers digits.

`geom::plane_offset` reads `T::difference_dot` since PR 7411
(`germ/plane-pose-far-frame`, `3547fe5`), so every plane offset a `Sym`
lane decides on drifts. The two rows this reds:

- **editor-core `revalue_corpus`**: ten rows fail their `drifted == 0`
  check (`holds` and `holds_until_parting`, "a decision's DAG re-valued
  at p0 is not the p0 build's own bits"): `revalue_a_chart_on_a_cap_tilted_at_zero`,
  `revalue_boss_union`, `revalue_boss_union_apart_to_its_parting`,
  `revalue_cut_cylinder`, `revalue_declared_tangency`, `revalue_die_chamfer`,
  `revalue_heat_sink_fins`, `revalue_measured_web`, `revalue_plate_param`,
  `revalue_plate_param_apart_to_its_parting`. The laundered predicates are
  `carrier_on_surface_{1,2}` and `witness_on_surface_{1,2}` everywhere,
  plus `tangent_*` on declared_tangency and `curve_face_pose_belly_graze`
  and its neighbours on cut_cylinder (the belly, PR 7553). Drift counts
  at default ε on `a433ff83d6`: 50, 244, 228, 289, 172, 322, 1120, 39,
  662, 466.
- **editor-core `m10_p_lift::at_sym_a_fixed_box_is_the_build`**: its
  "a margin of the held run moved with a parameter symbol" count is a
  re-valuation of the same nodes (662 at default ε), so it reads the
  drift as a bound symbol.

Bisected on hosted CI (GREEN U19, probe run 45312428995): every row
above green at `1883da3` (PR 7437) and red at `3547fe5` (PR 7411), at
the default ε; probe run 45308109008 has them red on `a433ff83d6` at
default, 1e-6 and 1e-12 alike. They are not held: the GREEN row
`main-red-revalue-corpus-carrier-and-witness-on-surface-drift-since-7411`
points here.

## Which side is right

The instrument is. A `Sym` value whose node computes something else is
the shape E12's guard exists to see (a value that did not come from the
DAG), even though here the value is the better rounding of the same real.
The f64 value is not wrong either: PR 7411's far-frame decisions need it.
So neither the expectation nor the posed reading should be relaxed.

## The fork (why `design: true`)

- **A node that is the computation**: a `SymOp` for the compensated
  difference-dot (and one for the belly), re-valued through
  `f64::difference_dot` / `f64::harmonic_belly`. `SymNode` holds three
  kids, and the difference-dot reads nine scalars, so this is a node
  shape question (a chain of `Vec3`-valued nodes, a side table, or a
  wider node).
- **A plain value at `Sym`**: drop the two overrides, so `Sym<f64>`'s
  value is the plain formula's rounding. The DAG is then the
  computation, but the `Sym` lane decides far-frame offsets on the
  digits PR 7411 removed from the `f64` lane, and the two lanes can
  answer one decision differently.
- **The instrument knows the pose**: `revalue` re-values a recognised
  plain-formula subtree through the posed reading. This is fragile,
  because it pattern-matches a DAG shape.

The class sweep: in `sym.rs`, the shape is a `Sym { value: <T's own
reading>, node: <another formula's node> }`. It hits exactly the two
above. The sweep cannot see a scalar that launders a value through
`from_f64`, which is what the instrument itself reports.

## 2026-11-24: the same two rows, filed again from f2815a23 (GREEN U21)

GREEN's P0 `main-red-two-editor-core-slow-rows-no-row-holds-at-f2815a23`
listed `revalue_die_chamfer` (`drifted: 322`) and `revalue_measured_web`
(`drifted: 39`) as unheld. They are two of this row's ten, with the same
counts and the same laundered names, so the cause is the same. A hosted
bisect agrees (probe run 45369489326, default ε): both pass at
`9f15dcb96c` and `1883da3554`, and fail at `3547fe5764` (PR 7411) and at
every later point probed, through `341d62e9a6`. They stay red and
unheld, as this row decided.
