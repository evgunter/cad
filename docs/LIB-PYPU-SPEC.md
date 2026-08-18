# LIB-PYPU spec — PlacedUnion's Python/audit slice (binding)

Mandate: execute the slice deferred at PLACEDUNION's merge (#571;
"the Python/audit slice queued", MODEL-AB-LOG row): the group
boolean crosses to Python, the G8 audit rows get their honest
re-diagnosis, and the heatsink corpus payoff becomes a Python
scene. Read first: crates/editor-core/src/node.rs:562-593 +
314-379 + 958-1021 (PlacedUnion, PatternKind, the shared
placement_rule_fault door), crates/editor-core/src/placement.rs
(Frame), crates/pncad-py/src/py/doc.rs (the Node constructor
pattern; `literal` helper at :82; loft's Expr::count precedent at
:686-694), crates/pncad-py/src/tags.rs:139-197 (the refusal tags
— ALREADY crossed), docs/guide/north-star-audit.md rows 32-34 +
:312 (G8) + :317-320 (tallies),
crates/editor-core/tests/corpus/heatsink_union.rs (incl. the
numbered finding at :16-24),
crates/editor-core/tests/lib_placedunion.rs (the behavioural rows
to mirror).

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding (foreground builds
one at a time, build slots, no parking + kill-your-own-waiter,
commit+push per chunk, NO Co-Authored-By, no model names,
merge-main-before-open + re-merge on movement, checks STARTED,
cold clippy CI scope both lanes, k-lint discipline, comments
state the INVARIANT). ADDITION (the SEAL lesson, #601): pncad-py
sits behind its `python` feature — verify with
`cargo check -p pncad-py --features python --all-targets` and
`clippy` the same lane, both slot-wrapped; the default workspace
lanes compile none of your crate.

## 1. Deliverables, in dependency order

1. **`Frame` value class** (pncad-py): frozen pyclass over
   `d::Frame` (pncad::document already re-exports it —
   document.rs:124). Constructors `translation(v)`,
   `rotate_then_translate(axis, angle: Angle, v)`; accessors
   mirroring `columns`/`translation`; `__eq__`/`__hash__`
   BIT-EXACT (the SketchPlane precedent). Lengths cross as
   `Length`, angles as `Angle` (§L4); raw floats only where the
   Rust side is a bare direction/scalar.
   **Rider (adopt unless it drags in plumbing; report either
   way)**: the U4b trio as classmethods — `point_at`,
   `path_start_frame`, `mirror_across_plane`
   (geom_core::linalg::frame; the banked PYG23A rider). Typed
   refusals per FrameError; if the error plumbing is not
   mechanical, SKIP and record — the trio is a rider, not the
   unit.
2. **`PatternKind` value class**: three staticmethods —
   `linear(direction, spacing: Length)`,
   `circular(axis: NodeId, step: Angle)`,
   `explicit(frames: list[Frame])`. No other surface.
3. **The node doors**, mirroring Rust's two-door split exactly
   (one-door would re-create the two-sources-of-truth state
   node.rs:958 refuses): `Node.placed_union(input, count: int,
   kind)` (count crosses as int per the loft/Expr::count
   precedent — restate the §L4 structural-int exception in the
   docstring) and `Node.placed_union_at(input,
   frames: list[Frame])`. Refusals need NO new tags — tags.rs
   already carries all four PlacementRuleFault arms +
   placements_uncertified; prove it with executed refusal rows
   (empty explicit list; a det<0 frame; an overlapping pair →
   placements_uncertified).
4. **The count-param edit, narrowed**: a door for the corpus
   payoff's `SetStructuralParam { slot: Count, expr:
   Expr::param(name, Count) }` WITHOUT binding Expr — e.g.
   `DocEdit.bind_count_param(node, name: ParamName)` (your
   naming; state reasoning). FENCE: no general Expr class, no
   general SetStructuralParam surface — the Count slot only.
5. **The Python heatsink scene**: re-author the audit's heatsink
   family on the new doors — ONE PlacedUnion(Linear) node for
   the fins + the count bound to a doc param via §1.4; fins=5/7/9
   by ONE SetDocParam edit each; volume oracle per the existing
   dyadic pins. The BASE stays un-unioned — the kernel's
   single-solid combine wall (heatsink_union.rs:16-24) is NOT
   yours to cross; say so where the scene documents itself.
6. **The audit page, re-diagnosed honestly**: rows 32-34 flip
   only as far as the truth allows — the pattern-node +
   structural-param halves of the gap are now bound, the
   fused-base residual remains G8's named kernel gap. Rewrite
   row marks + the G8 entry (:312) + the tallies (:317-320) from
   EXECUTED evidence; test_north_star.py's
   `test_a_plural_payload_cannot_feed_a_boolean` docstring
   conclusion is now stale — rewrite it against the shipped
   state (its split→boolean refusal row stays true; assert
   PlacedUnion emits Body not Instances as the contrast).
   `Node.pattern` stays UNBOUND and stays in the
   named-gaps-absent list (binding it flips no row — the
   recorded doctrine); `placed_union` must NOT appear there.
7. **Tests**: stub parity (test_stubs.py is a hard gate —
   pncad.pyi entries for every new class/method); ty fixtures —
   legal rows using the new surface + illegal rows that each
   draw a diagnostic (bare float where Length spacing is
   required; a Frame where PatternKind is required); Python
   mirrors of the load-bearing Rust rows (fin group ==
   transform-union chain oracle; recompute-on-edit; empty-list
   and overlap refusals; one-Instance-segment naming through
   read-back if the selector surface reaches it — check, do not
   assume).

## 2. Fence

- NO kernel/editor-core/topo changes; the multi-solid boolean
  operand gap (G8 residual) stays a named kernel gap.
- NO `Node.pattern` binding; NO general `Expr`/`SlotId` surface
  (§1.4's narrowed door only); NO schema claim (bindings only —
  verify main's live SCHEMA_VERSION by eye at final re-merge
  anyway, standing discipline).
- NO crates/profile or demos/tour changes (SEAL fix pass and
  ONARC own those trees).
- die_tool's Python re-authoring is OUT (Revolve-of-half-disc +
  datum axis makes it the heavier lift; record as a banked
  candidate in the report).

## 3. Acceptance

1. Hosted matrix green incl. python-suite; suite grows (state
   from/to counts); test_stubs + ty green.
2. Executed refusal rows for all four fault arms +
   placements_uncertified from Python.
3. The heatsink scene's oracle green at fins=5/7/9 with the
   param-bump edit; the audit re-diagnosis defended row by row
   in the PR body.
4. `cargo check/clippy -p pncad-py --features python
   --all-targets` rc=0, stated as executed.
5. Report ≤150 lines to
   ~/.local/share/cad-work/lib-pypu-report.md: deviations
   enumerated, door-shape choices with reasoning, rider
   disposition, banked findings.

## 4. PR discipline

One PR, branch `lib/pypu`. Merge-main-before-open; re-merge on
movement (SEAL's fix pass and ONARC may land mid-unit — absorb
at routine re-merges; your tree is disjoint). Checks STARTED
before handoff.
