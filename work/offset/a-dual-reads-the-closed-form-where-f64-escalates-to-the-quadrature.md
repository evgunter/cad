---
id: a-dual-reads-the-closed-form-where-f64-escalates-to-the-quadrature
kind: issue
title: A dual decides every sign as f64 does, but f64's construction-time escalation to the certified quadrature has no dual twin: a dual pass can refuse where the f64 build passed
status: open
opened: 2026-10-26
priority: P2
cost: M
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
---



Filed by conic unit 1's implementer lane
(`closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them`,
branch `offset/limb2-for-every-carrier`), answering the unit's first
check: does a dual escalate an undecided sign as `f64` does?

**At the predicate, yes, exactly.** `impl Decide for Dual<T>`
(`crates/geom-core/src/dual.rs` `sign_within`, line 863) returns its
value channel's verdict, and `impl Decide for f64`
(`crates/geom-core/src/predicate.rs` `sign_within`, line 2936) does not
escalate on its own: a margin between `zero` and `escalate` is an
`Indeterminate` at `f64` and the same `Indeterminate` at `Dual<f64>`, and
the `k_stats` log records it alike. So no dual pass refuses at a single
predicate the `f64` pass decided.

**The escalation `f64` has is a lane, and a dual holds none.** Where a
construction-time decision cannot be read off the closed form, `f64`
reads it through the certified quadrature (`AtRestPolicy::quad_lane`,
`Some` at every certifying scalar, `None` at `Dual`):

- `crates/topo/src/boolean/solid_contain.rs` line 3902: a containment
  whose closed-form volume is `VolumeUncertified` is re-read through
  `Measure::Quadrature` only `if T::quad_lane().is_some()`; at a dual the
  closed form's refusal stands.
- `crates/topo/src/loop_winding.rs` line 593: a loop's certified winding
  is `Unwound::Uncertified` with no lane.
- `crates/topo/src/shell.rs` line 1435 and `crates/topo/src/pieces.rs`
  (through `sort_into_pieces`, `crates/topo/src/boolean/ops.rs` line 5599
  and `crates/topo/src/splitting/mod.rs` line 1161): a shell's role read
  with no lane reads the closed form only, and a face that needs the
  quadrature leaves the role unread.

So a dual pass can refuse where the `f64` build passed, at exactly the
nodes whose decision needed the quadrature. Under DL3 the report that
reads a sensitivity binds the dual pass to the `f64` evaluation by build
identity, so such a pass is a refused sensitivity, not a wrong one; the
question is whether DL3's "a dual keeps its values ... the sensitivities
are measurements taken in dual arithmetic over the dual body" means the
dual should construct where `f64` constructed (a dual twin of the
escalation: the decision read at `T` on the value channel, which is
DL1's no-certify line), or refuse there (today).

**Rows affected: none measured.** The dual corpus
(`cargo nextest run --release -p editor-core -p sweep -E 'test(/dual/) | test(/Dual/)'`,
63 rows) passes in full on the branch, so no corpus document reaches one
of these sites with a face the closed form cannot read. A witness owes a
dual evaluation of a document whose boolean, split or shell meets a
spline or `Approx` face where containment or a shell role is read; none
was built here.
