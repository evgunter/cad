---
id: a-plane-poses-foot-outgrows-the-coincidence-doors-budget
kind: issue
title: A split's tool plane read at its foot outgrows the coincidence door's symbolic budget
status: open
opened: 2026-10-23
priority: P1
cost: M
refs: [a-plane-poses-in-plane-origin-enters-the-mate-coset]
---

Found by the implementer of `a-plane-poses-in-plane-origin-enters-the-mate-coset` (branch `msolve/plane-pose-holds-its-invariants`), measured.

A plane pose's value is now `{ normal, offset }`, and a split or a mirror reads its point as the plane's foot, `offset · normal` (`PoseValue::plane_foot` in `crates/editor-core/src/pose.rs`, read by `plane_of` in `crates/editor-core/src/verbs/split.rs` and `wire_mirror` in `crates/editor-core/src/eval/wire.rs`). That is what the row's Decided section rules.

At `Sym` the foot is `n (n · p)` for a face read off a carrier with stored point `p`, where the representative it replaces was `p` itself. The carrier's stored normal is a computed form there (its `x` and `y` are not literal zeros, so `mate::solve::on_its_axis` cannot tighten it), and the tier cannot fold `|n|² = 1`. So every margin that reads the section face's carrier origin carries one more factor of the normal.

Measured on `coincidence_door::a_reunited_splits_section_caps_are_one_construction` (`crates/editor-core/tests/coincidence_door.rs`): the reunion's four `VertexFusion` / `OnCarrier` rows, each a DAG of 281 nodes, no longer discharge at the door's budget (`coincide/replay.rs`'s `BUDGET`, 256 terms). They still fail at 300, 384 and 512 terms, and discharge at 2048 in about 7 ms each. So the row reads `Unproven { recourse: Budget }` where it read `Structural`, and `run_checks` would report an unproven coincidence on a plain split-and-reunite. Main proves the row.

**Re-measured over PR 5616** (merged at `de020583`): unchanged. The reunion's section-cap row (`SameOpposite`, `PlaneLadder`) proves at rung 1, `Structural(SameConstruction)`, as it did before. Its two caps read the one tool, a slide of a face plane, so 5616's reading of `Plane { face }` as the face's construction does not apply to it. What fails is only the four crossing-vertex rows (`CrossingVertex { … Enters }` against `… Leaves` on one lateral edge). They are two minted constructions, so they reach the `Sym` replay, and their `OnCarrier` margins exceed the 256-term budget.

The options, none of which this unit decides:

- **Topo takes a plane equation.** `SplitPlane` and `Reflection` take `(normal, offset)`, and the kernel's margins read `p · n − c`. This is cheaper than either spelling. The section face's carrier still stores an origin, though, so the `OnCarrier` margin would need the carrier built from a point the split already holds.
- **The tier learns `|n|² = 1`.** A unit vector's construction states it, but `register_equal` aliases nodes only (`geom-core/src/sym.rs`), and the margin expands `n_x² + n_y² + n_z²` without ever building that node.
- **A kernel-only witness point.** The value keeps a point that only the kernel's own constructions read. The Decided section rejected this kind of representative, because a reader can then see the freedom.
- **Accept the weaker proof** and re-state the row.

A fifth option opened by 5616: the walk reads the two crossing vertices of one edge cut by one tool as one construction, so they never reach the replay. That is COINCIDE's ground.

**Done when** the reunion row proves structural again at the door's budget with the canonical value, or Ev has ruled one of the above.
