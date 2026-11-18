# Design fork `embed-price` — the problem, as handed to every designer

Read `docs/prompts/designer.md` in full before you start; it is binding.

**The row:** `work/embed/the-at-rest-gate-certifies-embedding.md` (P0), its
`## Census` section on the branch `embed/gate`
(`git show origin/embed/gate:work/embed/the-at-rest-gate-certifies-embedding.md`).

**What needs deciding.** What a finished body's embedding certificate
costs the work done with the kernel, now that it has been measured over
every suite, and what should be true of the kernel so that the promise
Ev ruled (a finished body's boundary is embedded, certified at the
at-rest gate, at every scalar with certification rights) is one the
suites, CI and an editing session can afford.

**Why it is a question.**
- Ev ruled on PR 6301 that embedding is part of what a finished body
  promises and is certified inside the gate (`docs/DESIGN.md`
  `[topology.tiers]`), having seen the price per body: 4 to 25 times
  tier 3 on planar bodies at f64, up to 9.2 s on the tour's worst
  curved body, 92 s over the tour walk against tier 3's 31 s (the
  row's `## Measured`). The ruling named one residual, dragging a loft
  that a Boolean or shell consumes, as "a D5 question of its own" if
  it bit.
- Ev ruled on PR 6326 that the scalar's `finish` is the one
  constructor of a finished body, that every door that returns a body
  returns its finished type, and that no consumer re-gates.
- The gate has now been built on a branch (`embed/gate`) and every
  suite run with it asking. Measured at main `362cd72fd2`, on a
  4-core box, CI's build configuration:
  - the workspace's tests make **247,480 asks**, costing **27,344 s**
    of the gate (15,171 s on bodies that hold, 12,173 s on refusals),
    in a suite whose wall time was 8,786 s;
  - the gate is about **90% of `sweep`'s test time** (20,255 s of
    22,758 s), about 75% of `editor-core`'s (4,404 of 5,883), and most
    of `viewer`'s and `mesh`'s;
  - two rows of one review file take 50 minutes each, nearly all of it
    the gate;
  - the lane's reading of why: **every intermediate finish asks**, not
    only a door's final result. A chain of operations finishes each
    intermediate body, and each pays in full. The spec's lane D (the
    verdict kept once per node by content key) would not cover it.
- The repo's bars (`memories/orchestration-model.md`, "Friction is a
  finding"): hosted CI takes 15 minutes at most, 25 for a diff that
  seeds `topo`; so does any local development step.
- Of those asks, 1,056 tests' worth refuse valid bodies today (engine
  readings still missing) and about 750 trip on contacts (a separate
  fork, PR 7068). Those will change the count. They do not change the
  cost of an ask that holds, which is the larger half.

**Why it matters now.** Lane C cannot land a gate that multiplies the
suite's time several times over without that being decided, and the
answer may change what lane C, lane D and the engine lanes build.

**Where to look.**
- `work/embed/the-at-rest-gate-certifies-embedding.md`: `## Measured`
  (the per-body price), `## Spec` (lanes C and D), and the census on
  `embed/gate`.
- `crates/topo/src/validate.rs` (`AtRestBody`, `validate`, `moved_by`),
  `crates/topo/src/props.rs` (`AtRestPolicy::finish`, the leaf switch),
  `crates/topo/src/finish.rs` (`take`, `FinishedBody::moved`),
  `crates/topo/src/boolean/ops.rs` (`gate`), `crates/topo/src/shell.rs`,
  the sweep doors, `crates/editor-core/src/eval/` (how a document's
  nodes finish and what the memo keys on).
- `crates/topo/src/clearance.rs` (`embedded`, `Sweep::run`) and
  `crates/topo/src/injective.rs`: what an ask does and what it reads.
- `docs/DESIGN.md`: `[topology.tiers]`, D5 (latency), `[one-way.readings]`,
  `[simple.checks]`, and whatever governs what a verdict is a fact of
  (an isometry keeps a finished body's verdict).
- The rulings: PR 6301, PR 6326, PR 3870 ("once, at the door that
  built it"), and the two open `[ev]` PRs on this track (7068: the
  certificate asked per solid and the finished body holding its
  records; 7095: the sliver).

**What is not known.** The lane is measuring three more things, which
will be on the row: the same run's time without the gate; where the
asks come from (which finish sites; how many are of a body an earlier
ask in the same test already certified in substance, as the same body,
an isometric copy, or a body most of whose faces are carried unchanged
from a finished operand); and the distribution of asks and seconds by
body size and verdict. If your answer turns on a number that does not
exist, say which.

A cold build of `topo` here takes 16 to 20 minutes and the box holds
one at a time, so do not build: read, and name any measurement you
need in `## For the orchestrator`.
