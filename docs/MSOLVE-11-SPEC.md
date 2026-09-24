# MSOLVE-11 — The solve's decisions reach a mate's own log (spec)

Unit of the `msolve` program. Item `work/msolve/MSOLVE-11.md`. Answers
`work/msolve/mate-lane-escalations-reach-no-nodes-log.md` (PROPS) and
`work/msolve/placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate.md`
(CHROME). Read both in full, including what the first says it does
NOT claim. **Track:** a kernel change to where the solve's funnel
decisions are recorded and where two refusals are sited. No verdict
moves. One review, full: style questions plus the claims below. No A/B
row. Dispatches from main after MSOLVE-9, which rewrites the same
functions.

## What the tree says now

1. **The solve records into whatever frame is open.** The evaluator
   calls `mate::solve_with_env` once per run, in the pre-pass that
   builds the nominal environment (`eval/mod.rs`, before any node's
   `Bracket` opens). Every funnel decision inside it — `mate_coset`'s
   frame ladder, `coset.rs`'s `parallel` and `perpendicular`, the
   rider's `mate_clocking_redundant`, the fold's intersections — lands
   on the caller's frame, or on none. A node's escalation log
   (`NodeValue::escalations`, `NodeError::escalations`) therefore never
   holds a mate escalation, even on the mate the solve refused for it.
   `asm_r2a_mate_solve::row7e_a_mate_solve_escalation_is_on_no_nodes_log_but_visible_in_an_outer_frame`
   pins that today.

2. **`coset::parallel` mints an `Indeterminate` by hand.** Its
   `UnitVec3Error::NonFiniteLength` arm builds `Indeterminate { margin:
   Invalid, band, predicate: Some("mate_axes_parallel") }` itself, so
   that escalation is on no frame even when one is open. The input
   that reaches the arm is `u × v · arm` with `u`, `v` unit witnesses.
   Its length is at most `arm`, so the arm fires only when the ARM is
   not finite. A non-finite arm is a wider fault than one predicate:
   every levered margin multiplies by it, and an infinite margin is
   maximally definite to `Decide`, so `perpendicular` and the rider's
   roll decide confidently on it. The arm is a sum of authored lengths
   and parts' reach, each finite, whose sum or whose squared norm can
   still overflow.

3. **`check_reference` sites a `Part`'s own index at the pattern
   below it.** In the `Part`-agreement loop (`mate/member.rs`) the
   `Part`'s index expression is evaluated as `count_of(level.node,
   index, SlotId::Instance)`, and its refusal is sited at
   `level.node`, the PATTERN. The expression is the `Part`'s own
   `Instance` slot; the evaluation fails the `Part` and evaluates the
   pattern `Ok`. The mate row then says "node P refuses" about a
   healthy pattern, and since CHROME's PR 3100 the tree's link takes
   the user to it. The `MissingInput { input: part }` arm above is
   sited the same way. This is the class MSOLVE-7 closed for the axis
   operand: one condition, two seats.

## What the unit builds

**1. One home per decision.** The solve records each funnel decision
on the log of exactly ONE mate: the mate whose answer the decision
decided.
- A decision about one mate's own datum lands on that mate. That
  covers the walk's checks (`check_references`), its frame ladder, its
  coset row, and its rider.
- A decision the fold makes while adding a mate to a pair's
  intersection lands on the mate being added.
- A decision about the PAIR after its mates are folded (the
  determination check, the pair's placement) lands on the mate the
  solve already names for the pair's own verdicts. `Under` names
  `mates[0]`; use the same mate, and say so where it is chosen.

`k_stats::detached` is the door: the solve runs each unit of work
under it and keeps the `Detached` recording per mate. `SolvedPoses`
carries them, keyed by mate. A `Detached` is deliberately not
`Clone`, which is why each decision has one home. The mate node's
evaluation `splice`s its own recording into its frame, so the
escalation is on that mate's log whether the mate evaluates `Ok` or
`Failed`. Instance nodes receive nothing: the pose they read is
upstream of them, by the same rule that keeps an instantiated part's
decisions on the part's own nodes. The parts' evaluations under the
reach keep their own shielding brackets, unchanged.

The recording must survive the memo. Measure what a reused mate node
carries (`NodeValue::escalations` on a reused value; the evaluator's
reuse check compares escalation prefixes). Make a reused mate's log
equal a fresh one's for the same solve answer, and pin it with a row.

**2. The lever is a finite length by construction.** The lever is
formed at one site per caller: the fold's `parts + lever_arm` and the
door's lever closure. Form it once, through one checked door that
refuses a non-finite sum, typed, with the range recourse
(`geom_core::predicate::RANGE_RECOURSE`). The refusal's name and home
are yours, under `MateFault`'s vocabulary: a `LeverRefusal` arm or a
sibling. The value the predicates receive then carries its
finiteness: a newtype minted only by that door is the preferred
shape. `parallel`'s `NonFiniteLength` arm becomes unreachable. Make
it fail loud with the invariant stated (`unreachable!` naming the
door that guarantees it), not a minted escalation. Sweep every other
levered predicate in `coset.rs` and `solve.rs` for the same input,
and name what the sweep could not match.

**3. The `Part`'s index is refused at the `Part`.** Site `count_of`'s
refusal and the `MissingInput` arm in the `Part`-agreement loop at
the `Part` node, whose slot the expression is. Construct the refusal
the finding could not: a `Count` expression in a `Part`'s index that
fails at the document's bindings. Pin that the mate's
`PlacerRefused { placer }` equals the node the evaluation itself fails
(`ev.nodes[&placer]` is `Failed`, and the pattern is `Ok`). If the
`MissingInput` arm is unreachable, as the finding believes, state that
at the arm. Sweep `member.rs` for every other `count_of` / `refused`
call: does each seat at the node whose slot it evaluates? Say so in
the PR, with the blind spot and a second pass shaped at it
(discipline §5).

## Acceptance

- **A1** The row that pinned the gap flips.
  `row7e_…_is_on_no_nodes_log_but_visible_in_an_outer_frame` becomes
  a row where the `mate_axes_parallel` escalation is on the refused
  mate's own log and NOT in an outer frame. Its name moves with its
  claim.
- **A2** One home. Over the mate suites' documents, every escalation
  the solve records appears on exactly one mate's log: the union over
  mates equals what an outer frame received before this unit, in
  decision order per mate. A row pins it over a corpus.
- **A3** Memo: a reused mate node carries the same log as a fresh one.
- **A4** A non-finite lever refuses typed at its one door, through
  the edit door and through the solve. No predicate receives a
  non-finite arm. `parallel`'s minted `Indeterminate` is gone.
- **A5** The `Part`-index refusal names the `Part`, which is the node
  the evaluation fails.
- **A6** Both items closed with `## Closed` sections citing the rows.
  `work.py lint` clean.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record. Poll it in the foreground, and read
  `mergeable_state` before concluding anything from a run that has not
  appeared.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools. Private `CARGO_TARGET_DIR` outside the worktree. `git status`
  before every `git add`; never `git add -A`.
- Fence: `crates/editor-core/src/mate/solve.rs`, `mate/coset.rs`,
  `mate/member.rs`, `mate.rs` (the refusal's arm and its docs),
  `eval/mod.rs` (the mate node's splice, and nothing else there),
  `pncad-py` if a new `MateFault`/`LeverRefusal` arm crosses
  (tag, payload, `.pyi`, census), tests, the items. `geom-core` is
  outside: `detached`, `splice` and `RANGE_RECOURSE` are taken as they
  are. If the unit needs a door there, STOP and say what door.
- Comments state the invariant (discipline §4).
- **Stop clause.** STOP, write what you measured into the PR as a
  draft, and end your turn if any of these holds:
  - attributing a decision to one mate needs a decision to be made
    twice;
  - a reused mate's log cannot equal a fresh one's without a second
    content channel in the memo key;
  - a verdict moves anywhere in the mate suites.

## Out of scope

Whether `drive::classify_replay`'s error-enum arms are load-bearing
(`work/props/should-classify-replays-error-enum-arms-be-deleted.md`,
PROPS's question). The edit door's own admission (`admit_mate`)
records into the caller's frame, as every door does; this unit
changes the evaluator's pre-pass, not the doors.

## Review

One review, full, claims to falsify:

- **C1** A1 and A2: the escalation is on exactly one mate's log, the
  one named by the rule in §1, and no longer in an outer frame.
- **C2** A3: a reused mate's log equals a fresh one's.
- **C3** A4: no levered predicate can receive a non-finite arm through
  any door. `parallel`'s arm is unreachable, and the reason is stated.
- **C4** A5: for every `Part`-index refusal the tree can produce, the
  named `placer` is the node the evaluation fails.
- **C5** No verdict moved: the mate suites pass unchanged except the
  rows the PR names as moved, each with where it meets its answer now.
