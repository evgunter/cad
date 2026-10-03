---
id: boolean-declares-no-touching-between-copies-of-one-operand-vertex
kind: issue
title: The boolean emits contact records only for cross-operand vertex pairs; its own copies of one operand vertex that touch in a result are declared by nobody
status: closed
opened: 2026-10-02
closed: 2026-10-02
priority: P2
cost: M
refs: [3856]
---


## What

`crates/topo/src/boolean/reduce.rs` pushes `VvContact`s for cross-operand pairs and carried pairs only. Where the boolean's `mev_null` copies one operand vertex and two copies land touching in the result (the same shape split's pinch halves have), no record covers them, so the result can refuse at rest as an undeclared `VertexVertex`. Whether it is reached in the corpus is unmeasured. Its fix shape depends on the open pinch-contacts question on TQUERY (records vs a shared point).

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## Closed (2026-10-02, branch `tquery/split-pinch-shared-point`)

Closed by construction and measured. Ev ruled the shared point (PR
3813), and the build gives every `mev_null` copy the original vertex's
`PointKey`. The boolean mints its copies through that door
(`boolean/insert.rs`, `boolean/vtxfac.rs`). So copies of one operand
vertex that touch in a result sit on one point, and the census clears
them as rung-1 structural sharing (`census.rs`, `Geo::same_point`). No
record is needed.

Measured with the release suites of topo, sweep and editor-core,
instrumented locally and not committed. The probe ran
`gate_at_rest_kept` + `gate_at_rest_declared` on every `Body` result of
`boolean_op_with`, against that result's own `ContactRecords`, and
counted vertex pairs on one point:
- 22,138 results on the branch; none holds two vertices on one point.
  The copies are zipped or killed before rest in every corpus result,
  so the case this item names is never reached.
- Refusals per test are the same on main and on the branch, within
  interleaved-log noise of one line. No result flips either way.


## Reachability, measured (2026-10-02)

An independent re-measure on main after the shared-point change
(branch `cleave/vv-copies-measure`). It confirms the closure above.

**Mechanism, as the tree reads now.** The boolean mints its copies
through `Body::mev_null`: `boolean/insert.rs` (the fan insertion) and
`boolean/vtxfac.rs` (the strut and pierced-side mints). `mev_null`
(`null.rs`) passes the fan plan's `point` to `mev_fan_execute`, so the
copy shares the original vertex's `PointKey`. The census's
`pair_vertex_vertex` (`census.rs`) skips a zero-gap pair when
`Geo::same_point` holds, before it looks up any record. `reduce.rs`
still pushes `VvContact`s only for cross-operand pairs, but no record
is needed for copies, so the claim no longer holds: copies of one
operand vertex cannot refuse as an undeclared `VertexVertex`.

**(a) Instrumented corpus.** The probe was never committed. It wrapped
`boolean_op_with` and, for every `Body` result, ran
`gate_at_rest_kept` and then `gate_at_rest_declared` against that
result's own `ContactRecords`. It also counted vertex pairs on one
point. Run: `cargo nextest run --release -p topo -p sweep
--no-capture`, which passed 4,027 tests and gave 2,743 probed results.
- Results with two vertices on one point: **0**. The copies are fused
  or killed before rest in every result, so this item's shape is not
  reached in the corpus.
- Undeclared `VertexVertex` refusals: 2, both from
  `m3_pr6_tier3prime::closure_kiss_vs_mover` (∪ and ∖). Their vertices
  sit on distinct points from two different operand bricks, so they are
  not `mev_null` copies. They are that file's documented closure gap:
  a kiss contact inside an operand is not carried into the result.
  That test already pins the refusal, so this is not a new finding.
- Other census findings: 18 results. Gate-kept refusals: 53 results.
  The census at rest (`gate_at_rest_kept`) already refuses 53 results
  before the declared pass runs, so those were not checked for this
  shape. Neither group involves two vertices on one point.

**(b) Public-door constructions**, run at f64 and Interval, each
checked with `validate_closed` and `validate_pseudomanifold` against
the result's own records:
- a reflex edge split by a second notch, over the full height and over
  z in [0.5, 1];
- a wedge along a cube's convex edge, over the full height and over the
  upper part only;
- a cube kissing an L's reflex vertex;
- a ridge prism on a face;
- notch fills of an L.

All of them pass, and none holds two vertices on one point. At each
pinch the op leaves one vertex per point, or two cross-operand vertices
with a `VvContact` (the reflex-vertex kiss).

**Reproducer: none.** The pinch shapes are kept as regression rows in
`crates/topo/tests/boolean_pinch_copies.rs`:
`pinch_copies_do_not_reach_rest` (f64: tier 2, tier 3′ and one vertex
per pinch point) and `pinch_copies_do_not_reach_rest_interval`.

**Blind spots.** The probe sees only `boolean_op_with` results, at the
scalars the suites run. Editor-core and the demos were not re-run here;
the closure above covers editor-core. No public construction kept two
copies apart at rest, so the `same_point` rung was never exercised on a
boolean result. Its only row is split's
`notched_block_halves_pass_the_pseudomanifold_door_with_no_records`.
Recommendation: the item stays closed.
