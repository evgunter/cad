---
id: mate-lane-escalations-reach-no-nodes-log
kind: issue
title: The mate lane's escalations reach no node's log: the whole-document solve, and coset's own mint
status: closed
opened: 2026-09-20
priority: P1
cost: D
parent: MSOLVE-11
closed: 2026-10-01
---



## What

Two escalations the mate lane produces reach a consumer without ever
passing through a node's escalation log. Found by the PROPS
escalation-channel unit (PR 2928), which closed the op-minted family in
`geom-brep` and measured that closing it does NOT close these.

1. **The whole-document solve runs before any node's bracket opens.**
   `editor_core::eval`'s nominal pass calls `mate::solve_with_env`
   (`eval/mod.rs`, in the pre-pass that builds the nominal environment,
   ~:2960) outside any node's `Bracket`, so the funnel escalations
   inside `mate::solve` land on whatever frame the caller happened to
   leave open rather than on a node's. Pinned today by
   `asm_r2a_mate_solve::row7e_a_mate_solve_escalation_is_on_no_nodes_log_but_visible_in_an_outer_frame`.
2. **`mate::coset::parallel` mints an `Indeterminate` of its own** after
   `UnitVec3::new` answers — the `UnitVec3Error::NonFiniteLength` arm
   builds `Indeterminate { margin: Invalid, band, predicate:
   Some("mate_axes_parallel") }` by hand, so the escalation its caller
   receives is on no frame's log even when a bracket IS open. Exactly
   the shape PR 2928 retired in `geom-brep`, one crate over.

## What this does NOT claim, and the correction that made that worth
saying

An earlier draft of this file said, in the same breath, that a mate
escalation reaches `drive::classify_replay` as
`NodeErrorKind::Mate(MateFault::Indeterminate)` — which takes the
`_ => Bisect` fall-through — AND that `classify_replay`'s two
error-enum arms are load-bearing for this path. **Both cannot be true**,
and it cited PR 2928's comment in `drive.rs` for the second while that
comment cited this file: a circular citation with no measurement at
either end. It is removed. What is true is only (1) and (2) above:
these escalations are on no node's log. Whether anything downstream
should change because of that is
`work/verdict/should-classify-replays-error-enum-arms-be-deleted.md`'s
question, and it is open.

## Shape

(2) is **two lines** with the doors PR 2928 added: the hand-built
payload becomes a `geom_core::k_stats::gate_measured` call under the
same name, so the funnel records what it produces. A `UnitVec3` door
that escalates through the funnel rather than through a payload its
caller assembles is the better fix and the one worth arguing; the
two-line version is available immediately if the better one is not.

(1) is the structural half: either a document-level frame the solve
records into, spliced onto the nodes it constrains, or the solve
recorded per mate node.

## Refs

Split out of `work/props/escalation-channel-misses-op-minted-indeterminates.md`
family 3, whose family 1 PR 2928 closed. `MateFault::Indeterminate`
and `FoldStop::Indeterminate` are two of the five variants that fall
outside that unit's sweep pattern — both `Box<Indeterminate>` — which is
recorded in `work/props/indeterminate-error-arms-sweep.md`.

## Closed — each solve decision is on its own mate's log, and the lever is finite by construction (PR 3680)

Both halves are closed in `editor-core`, and neither needed a new door
in `geom-core`.

**(1), the whole-document solve.** The evaluation's solve
(`mate::solve::solve_with_env`) runs every unit of its work under
`k_stats::detached` (`mate/solve.rs`, `Record`). It keeps each
recording on `SolvedPoses` for the one mate whose answer the unit
decided:

- a mate's own datum (its reference checks, frames, face poses, coset
  row and rider) is recorded on that mate;
- the fold's work adding a mate to its pair's intersection is recorded
  on the mate being added;
- the pair's determination and left factor are recorded on
  `mates[0]`, the mate an UNDER names.

The mate node splices its own recording into its frame before its key
(`eval/mod.rs`, `eval_node`). `solve_document` still records into the
caller's frame, as every door does.

A reused mate carries THIS run's recording. The solve's decisions about
a fold-mate read the mates folded before it, and the mate's key does
not, so the memo's prefix assert would otherwise panic. The row below
reproduces that panic with the special case removed.

Rows:

- A1: `asm_r2a_mate_solve::row7e_a_mate_solve_escalation_is_on_the_refused_mates_own_log_and_not_in_an_outer_frame`
  is the old `…_is_on_no_nodes_log_but_visible_in_an_outer_frame`,
  renamed and inverted.
- A2: `fixture::solve_decisions_have_one_home` runs inside
  `fixture::run` on every mate document a suite evaluates. Its
  reference is `solve_document` under a bracket. It checks that
  nothing lands outside the nodes, that each mate's escalation and
  verdict log is in order, and that the logs together are the
  reference. A sabotage that drops one recording reds the mate suites' rows.
  Which mate a decision lands on is pinned by
  `msolve11_mate_log::each_decision_is_on_the_log_of_the_mate_whose_answer_it_decided`.
- A3: `msolve11_mate_log::a_reused_mate_carries_the_log_a_fresh_one_does`.

**(2), coset's own mint.** The lever is a `mate::coset::Arm`, minted
only by `Arm::of`. That door refuses, as `LeverRefusal::OutOfRange`
with the range recourse, any arm whose sixteenfold square overflows.
So `parallel`'s `NonFiniteLength` arm is `unreachable!`, with the
invariant stated, and the hand-built `Indeterminate` is gone.

Rows for (2):

- `msolve11_mate_log::a_lever_out_of_range_refuses_typed_through_the_solve`
- `msolve11_mate_log::a_lever_out_of_range_refuses_typed_at_the_edit_door`
