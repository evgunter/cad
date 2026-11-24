---
id: a-nodes-content-key-leaves-out-inputs-its-op-reads
kind: issue
title: A node's content key leaves out inputs its op reads: the clearance witness point, the computing twist, the sweep strategy, a part row's options, a family member's indexed read
status: closed
opened: 2026-11-22
closed: 2026-11-24
pr: 7512
---



Found while auditing what `eval::eval_node` and `wire::run_op` read
against what `eval::content_key` hashes, for the node-key unit
(`a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression`).
Found here by reading. **All five have since been executed into a
wrong answer** (the review of PR 7470, its probe
`review_node_key_stale_member_read`, and PR 7512), and the fix is
`a-family-members-read-of-another-member-is-served-from-a-prior-that-read-its-old-body`
(`boxfinish/member-read-key`). They
are at every scalar, not the symbolic one alone, and each matters to
whatever serves a node across two runs whose `EvalOptions` or box
differ: the evaluator's prior today (a prior is refused only for
another document, `evaluate_at_descent`'s `ident::mispaired`; its
`RunLane` is compared only in `RecordBinding::of`), and the store per
drive (`a-drive-keeps-no-node-result-from-one-leaf-to-the-next`).

Not hashed, and read by an op:

- **The clearance witness** (`wire::Witnessing`, `LaneEnv::witness`):
  `wire_clearance` reads the `f64` rebuild's bodies and writes the
  rebuild's parameter point (`ParamWitness`) into the verdict. The
  rebuild is the nominal run, or the box's midpoint
  (`clearance::midpoint_of`) where the run is over a box, and
  `Witnessing`'s own doc says a pair is a witness about the box only
  where the rebuild is a point of it. A clearance node whose lane
  inputs agree across two boxes is served with the other box's point.
  The one caller that passes a prior across boxes today is the
  stackup's worst case (`stackup.rs`, `LeafPrior::of` over the nominal
  box, `Interval`), which reads keys and a measure off the replay, not
  an assertion's verdict.
- **The computing twist** (`OpEnv::twist`, `EvalOptions::computing_twist`):
  `twisted()` after each construction.
- **The boolean sweep strategy** (`OpEnv::boolean_sweep`): deliberately
  unkeyed (`EvalOptions`' doc), and the value's verdict log differs by
  strategy.
- **A part row's options** (`OpEnv::parts`): the key names the part by
  `DocRef` (`feed_doc_ref`); the row is built under the run's symbols,
  twist and sweep. `eval_any` guards the instance node by
  `parts::Reached::carried_from`; no key does.
- **A family member's read of another member**: inside a member's
  evaluation `members::overlay` stands the bound member in for an
  upstream family, so the upstream key recorded is that member's; an
  indexed seat reads the whole family at its index
  (`reads_projected`, `member_at`). The index value is hashed; the key
  of the member it picks is not, unless it is the bound one.

Within one drive the twist, the strategy and the part rows are the
same in every leaf, so the store's exposure is the clearance witness
and the family read. The symbolic session's side (the parameter
brackets the decision read takes from `Sym::param_over`'s record) is on
`a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state`.

Not determined by this reading: that `PoseDef::scalars()` lists every
scalar a pose definition evaluates, and that `Doc::read_ports` /
`Doc::upstream_of` cover every operation a nested pose definition
reaches.

## Closed

PR 7512 (2026-11-24) is the fix for all five: a node's keys hash what
each read sees (the family read), the clearance's verdict citing a
point is served only where the rebuild's point is this run's, and a
prior run under another twist, sweep or symbol setting is refused at
the door, which covers a part row built under other options. What this
reading did not determine (that a pose definition's scalars and
`Doc::read_ports` cover every operation a nested pose reaches) was
probed by that PR's review (a pose of a selection of another member, at
a mirror and a measure) and held; the general form of it, that the op
and its keys walk a node's reads separately, is
`a-nodes-op-and-its-keys-walk-its-reads-separately`.
