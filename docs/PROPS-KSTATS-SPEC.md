# PROPS k-stats — the verdict log as a bracket with a stack, and the escalation channel beside it

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Verdict
recording; the item is `work/props/k-stats-escalation-channel-and-redo.md`
— read it in full; difficulty logged at spec: **L**, STRUCTURAL). Read
`docs/prompts/implementer-discipline.md` in full. Branch
`props/kstats-bracket`, cut from `main`.

## The decision (the item's "shape to decide", answered)

`geom_core::k_stats` delivers a production value — the per-node verdict
log — by thread-local side effect (`VERDICTS`, `start_verdict_log` /
`take_verdict_log`, `k_stats.rs:108-295`), and its module doc puts the
mechanism on notice: `start_verdict_log` overwrites an installed log
unconditionally, so a nested evaluation destroys its parent's (measured:
an `InstantiatePart` node records 0 verdicts where the same geometry
evaluated directly records 722). The obligation names two remedies —
verdicts as a returned value, or the mechanism made structurally safe
(an RAII bracket, re-entry handled, thread confinement enforced) — and
asks that whichever is not taken be argued unaffordable IN WRITING.

**Ruling: the bracket, with a stack; the returned value measured and
declined in writing.** Reasons, to be verified and written into the PR
body and the module doc, not restated from here:

- `classify` is called from inside kernel ops (`geom-brep`, `sweep`,
  `topo`, `profile`) that carry no collector parameter; a returned
  value means threading a sink through every predicate site and every
  signature between an op's door and its predicates. MEASURE that
  first: count the `classify` call sites and the public signatures on
  the paths from `wire::run_op` to them; put the number in the body.
  That number is the "proven unaffordable in writing" the obligation
  asks for — or, if it is small, the ruling flips and you say so and
  stop for the orchestrator.
- A bracket with a STACK fixes the nesting bug by construction: a
  nested evaluation pushes its own frame, records into the top, and
  pops it into its own `NodeValue`; the parent's frame is untouched and
  receives only its own op's verdicts (which is what "the verdicts a
  node's op produced" means — the inner node has its own `NodeValue`).
  Re-entry is therefore not refused; it is the case the stack exists
  for. Pin the `InstantiatePart` 0-vs-722 measurement as the red-first
  row: the same geometry evaluated directly and through the part
  records the same log at the part's node (or the outer node's
  documented subset — decide and state which).
- Thread confinement enforced by the TYPE: the bracket guard is
  `!Send` (`PhantomData<*const ()>`), so it cannot cross a thread; the
  eval's idiom-1 parallelism (whole nodes on one worker each) keeps one
  frame per worker. The thread-local stays; what changes is that the
  bracket's correctness is a type, not a comment. Say in the module doc
  what a bracket dropped without popping does (the `Drop` impl pops it;
  a frame left behind by a panic is popped by the guard's drop during
  unwinding — state it and pin it).

## The escalation channel (the item's first half)

Beside each verdict, the bracket records every INDETERMINATE outcome
`classify` produces: an `Escalation { predicate: &'static str, source:
Indeterminate }` (whatever payload `geom_core::Indeterminate` carries
today — read `k_stats.rs` and `drive.rs:1514 sliver(...)`), in decision
order, in the same frame. `NodeValue` gains `escalations:
Arc<Vec<Escalation>>` beside `verdicts` (`eval/mod.rs:257`; not
serialized, like `verdicts` — say so at the field and at
`vdiff`'s persisted-shape sentence). The acceptance is the item's:

- a leaf evaluation can answer "did any predicate escalate, and what was
  the `Indeterminate`" without matching on op error enums;
- `editor_core::drive`'s `classify` (`drive.rs:1383-1415`) reads the
  node's escalations, so `RefusalReason::SliverTerminal` fires on an
  escalation wrapped in a kernel error enum — the fixture in
  `m10_3_driver_interval.rs` that reads `Budget` where `SliverTerminal`
  is the true class flips (name it; it is the red-first row for this
  half);
- the ~40 `Indeterminate`-carrying error variants across `sweep`,
  `topo`, `profile`, `geom-brep`, `editor-core` are NOT touched: the
  channel makes matching on them unnecessary, it does not delete them
  (a sweep that deletes arms is a different unit — file it).

## Fence

`crates/geom-core/src/k_stats.rs` (PROPS'; **M10-8 on
`origin/m10/m10-8-arc-family` edits its `probe`-gated `SampleOutcome`
half — read that diff first and keep your hunks off it; the verdict
half is separate);
`crates/editor-core/src/eval/mod.rs` — the one production bracket
(`:2448-2475`) and `NodeValue` — LIB/SEAT ground by M10's keep-out,
announced by this spec (the orchestrator posts the seam in
`work/seat/log.md`); `crates/editor-core/src/drive.rs` `classify`
(M10's, by the announced seam); `crates/editor-core/src/resolve/vdiff.rs`
one doc sentence; every test that calls `start_verdict_log` /
`take_verdict_log` (the census: `k_stats.rs` unit tests, `geom-core/tests/k_stats_doors.rs`,
two `geom-brep` tests, nine `sweep` tests, one each in `topo`,
`step-import`, `profile` — mechanical: the bracket replaces the pair).
Do not deepen any dependency on the retired pair: delete
`start_verdict_log`/`take_verdict_log` outright once every caller is
on the bracket.

## Posture

- Red-first: the nesting row (0 vs 722) and the `Budget`→`SliverTerminal`
  flip, both red against `main` first and quoted.
- ε posture: none (say so). No `CI-Config:` trailer. Bit identity of
  every recorded verdict log on the existing corpora (the `vdiff`
  goldens, the drive goldens' `witness_vector` keys) — a moved key is a
  finding, not a re-baseline, unless the nesting fix is its stated cause
  (an `InstantiatePart` document's key MAY move — measure which and say
  why).
- D2-addendum: the retired pair (row 0 — the unbracketed state is
  unrepresentable); a bracket dropped mid-op (state what happens).
- Sweep obligation: other thread-local production channels in
  `geom-core` (`SINK`, `VERDICTS`, anything under `k_stats`'s
  `thread_local!`) — which are measurement (probe-gated) and which are
  production; hit list; what reading cannot match.
- Review: standard v6 dual (block PROPS-B1 slot 2; ordinal claims at
  review dispatch). Reviewers' first target: the nesting fix under
  memo reuse and cancellation (a cancelled prefix, a memo hit that never
  runs the op — what frame receives what); second: the escalation
  channel's completeness — an `Indeterminate` produced by a predicate
  that never reaches `classify`.
- Landing: the item gets `pr:` and `status: review`; the spec is deleted
  at merge; no `Co-Authored-By`; push early to `props/kstats-bracket`.

## Acceptance

The pair is gone; one bracket type with a stack, `!Send`, `Drop`-popped;
the nesting row green; escalations recorded beside verdicts and read by
`drive::classify` so the named fixture reads `SliverTerminal`; the
returned-value cost measured and written; hosted CI green on the full
matrix.
