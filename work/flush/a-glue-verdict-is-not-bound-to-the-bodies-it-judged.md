---
id: a-glue-verdict-is-not-bound-to-the-bodies-it-judged
kind: issue
title: A GlueVerdict is not bound to the bodies it judged: a caller can re-key its A face or hand it to another pair
status: closed
opened: 2026-10-13
priority: P2
cost: H
branch: flush/nary-union-door
pr: 5772
closed: 2026-10-27
---

The residue of `boolean-given-pairs-glue-unmeasured-for-any-rust-caller`
(`flush/given-gate`, PR 5002), banded at its parent's P2 because the
harm it leaves is the parent's own: a glue no record states.

## What

A given pair (`GluedPair`) is made only by `GlueVerdict::given_at`,
and a `GlueVerdict` only by the kernel from its own face-pair rows
(`BooleanBody::verdicts`), so a given pair's class is always one some
boolean decided. What the verdict does not carry is which bodies it
judged:

- `given_at(held, a)` takes the A face from the caller, and since
  PR 5303 also `held`, which side of the verdict that face stands for
  (a union's judged pair may have the joining member as operand A).
  The union fold
  passes the accumulation faces whose lineage holds the judged face
  (`given_pairs` in `crates/editor-core/src/eval/wire.rs`), and the
  kernel cannot check that claim: the lineage lives in
  `editor_core::names` (`Fold::member_faces`).
- A verdict's B face is a bare arena key, so a Rust caller can take a
  verdict from one boolean and hand it to a step over two other
  bodies, where the key names whatever face holds it there. The door
  checks only that the keys are live and planar.

## Measured

PR 5002's review reproduced the parent's three measurements on the
gated branch through reuse (probes in
`crates/topo/tests/given_gate_review_probes.rs` on
`flush/given-gate-review` @ `e1abf0a4dd`):

- a `Continuation` verdict re-keyed onto an opposed rest touch builds,
  with 0 coincidence rows;
- a verdict reused on faces 0.5 apart builds;
- unrelated pairs refuse only at the door's unglued-coincidence
  backstop.

Over 648 runs (6 verdicts × 6 A faces × 3 ops × 6 B placements) and a
small-gap sweep, no forged step built a body that differs from the
decided run. What is lost is the record: a forged step glues with no
row, so D10's one door where a value-decided coincidence is recorded,
and the `unproven-coincidence` lint that reads it, never see the glue.

## Owed

Bind a verdict to what it judged without measuring again: either the
kernel carries the fold's lineage itself (the fold's step moved behind
a kernel door that holds the members and advances its own lineage from
each step's naming), or a verdict carries an identity of the judged
bodies that the door can compare. The façade never sets `given`, and
`pncad-py` does not expose it, so the route is a Rust caller of
`boolean_op_with`.

## Design

Ev adopted this on PR 5737 (2026-10-26, "sounds good!"), after asking whether an honest fold would be better. Two designers weighed the row and agree on the final state,
and both reject its framing: a verdict is not a token to bind to its
bodies. `given` is a decision handed to the kernel from outside, and it
exists because DM4's n-ary union and intersect have no kernel door. The
kernel exposes pair booleans, so `editor-core` rebuilds the operator
(judge every meeting member pair once, fold in the author's order, read
each verdict through face lineage) around them and hands the kernel's
own verdicts back in through a public channel it cannot check. The
lineage is already a function of kernel output alone (`names::Fold`
builds it from `topo::BooleanNaming` through `FaceDescent`).

The answer: `topo` gets an n-ary union and intersect over the members in
the author's order. It judges the pairs, folds, advances its own lineage
and carries each step's records, and returns each judgement's and each
step's body with the lineage `editor-core` names from.
`BooleanInputs::given`, `GlueVerdict::given_at` and `GluedPair` become
crate-private, so no public kernel input is a decision and a glue no
record states cannot be written. The fold's half of
`work/contacthold/a-booleans-records-are-complete-only-relative-to-the-records-passed-in.md`
and `work/contacthold/a-fold-steps-restated-tangency-takes-one-of-several-verdicts-margins.md`
close with it.

Rejected, on Ev's question: an honest fold, each step deciding its own
coincidences at the accumulation's extent with no pass. It makes the
union's refusals and records follow member order. On kernel probes a
covered touch is recorded in only some orders, a covered 3ε gap refuses
in 2 of 6 orders, and a 0.4ε tilt refuses in 2 of 3 orders where it is
read over the merged face. `[operations.operators]` calls an order that
refuses where another builds a kernel defect.

Rejected: a verdict carrying an identity of the bodies it judged (a stamp
the kernel carries, which `[coincidence.door]` rules out, and it leaves
the channel public); re-checking the given pair at the step (ruled out by
the one-verdict-per-carrier-pair rule PR 4323 ratified, and it records
nothing); leaving it documented (a sequencing choice, not a design).

Open within the answer, settled at dispatch: which member is a judgement's operand A. Today
`editor-core` picks the lesser read (PR 5303). The kernel door either
takes a per-member rank, or judges in list order with the margin made
symmetric where the asymmetry starts.

Settled at dispatch (the orchestrator's ruling, 2026-10-26): the door
takes a per-member rank, and `editor-core` passes the order it uses
today (the lesser read is operand A, PR 5303), so the move changes no
behaviour. The other option, list order with the margin made symmetric
where the asymmetry starts, is a possible follow-up.

## Measured (2026-10-26, on main `c2c75756dd`)

- **The union and intersect corpus through `editor-core`.** A temporary
  hook (not committed) fingerprinted every union and intersect node
  evaluation in `editor-core`'s whole suite, default profile with the
  slow set: its body, its name tables, its rows, its records and their
  cited inputs, or its refusal. 3009 of the suite's 3020 tests ran on
  both sides (the 11 left out are debug-build interval and range tests
  that run past 300 s, none a union test). 458 tests evaluate a union
  or an intersect: 11,427 evaluations, 11,350 bodies and 77 refusals.
- **The batteries.** `corner_pairs_battery`, `join1_delta_arc_battery`,
  `four_pairs_battery`, `pinch_runs_battery` and `pierce_runs_battery`
  (release, ignored): 32,959 outcome lines.
- **On main, red before this change** (default profile, not this
  unit's): 18 `editor-core` tests, among them
  `intent_s2_c_world::the_migrated_world_is_the_pre_c_product`
  (cut_cylinder's product moved), `m4_pr3_names_ci::die_name_tables_are_golden`,
  seven `revalue_corpus::revalue_*` rows, `node_standing`'s reader census
  and the interval pins.

## Fix

`crates/topo/src/boolean/nary.rs`: `topo::nary_boolean` takes the
members in the author's order, each with its records and rank
(`MemberInputs`). It reads each member's box, judges every pair whose
boxes meet as a pair boolean (operand A the lesser rank, the listed
first on a tie), and folds in list order. Each step is given the
judgements' verdicts on the member it joins, read through the face
lineage the door advances from each step's naming (`MemberLineage`,
`FaceDescent`, which moved from `editor-core`), and is handed the
previous step's records and the member's. It returns each judgement's
and each step's `BooleanBody`, each step's lineage, and for each
restated row the judged rows it stands for (`NaryStep::restates`). A
refusal says where it was raised (`NaryRefusal`).

`BooleanInputs` lost `given` and is records only; `GlueVerdict::given_at`
and `GluedPair` are crate-private, and a step's given pairs reach the
pair pipeline through the crate-private `boolean_op_given`.
`verbs::Members` is the n-ary verb (`run_members`), of kind
`VerbKind::Boolean(op)`, so its content tag is unchanged.
`editor-core`'s `wire_combine` (`crates/editor-core/src/eval/wire.rs`)
runs it and names what it returns; `judge_pairwise_contact`,
`PairVerdict`, `pair_verdicts`, `given_pairs`, `restated_given`,
`member_lineage` and `debug_assert_given` are gone, and `names::Fold`
reads the kernel's lineage instead of building its own.

**CONTACTHOLD's m2 stays open**, though this row's `## Design` says it
closes with the door
(`work/contacthold/a-fold-steps-restated-tangency-takes-one-of-several-verdicts-margins.md`).
The door keeps the moved `given_pairs` logic, one given pair per
(accumulation face, member face) under the first verdict, because
closing it changes which margin a step's restated row carries, a
decision this move does not take: it is CONTACTHOLD's. The door makes
the fix local to `nary.rs`, and the defect is pinned as it stands
(`crates/sweep/tests/face_face_touch.rs`,
`contacthold_m2_a_restated_tangency_carries_the_first_of_two_verdicts_margins`).

Re-measured on the branch, and again by both reviewers on the frozen
head after main moved under it (462 and 464 tests, 11,527 and 11,529
evaluations, as each filtered them): every evaluation gives the same
body, names, records, cited inputs and refusal. Rows are identical
except in one test
(`coincidence_door::an_unproven_fold_row_is_reported_under_the_judged_row_decided_at_its_point`),
where a fold row's margin carries a different deferred-entry index
(`ZeroWord`, the place of a Zero on the deferring symbolic session's
list; one reviewer counts 8 indices that move): all of a union's
booleans now run before its steps are named, so the session's list
fills in another order. `ZeroWord` is read only as that session's
handle, so nothing reads the move. Value, relation, cells and site are
equal. The batteries' lines are identical, and main's failing set is
the same on both sides.

## Resolution

Closed by PR 5772 (2026-10-27). `topo::nary_boolean` is the kernel's
n-ary union and intersect: it judges every pair whose boxes meet (operand
A the lesser rank), folds in list order, hands each step the judgements'
verdicts on the member it joins through the lineage it advances, and
returns every judgement, step, lineage and restated row with a refusal
that says where it was raised. The given channel (`GluedPair`,
`GlueVerdict::given_at`, `boolean_op_given`) is crate-private, and
`BooleanInputs` is records only. `editor-core` names from the door's
output. No body, name, record, citation or refusal moved.

Dual review (concurrent pair, one frozen head): both APPROVE-WITH-FIXES,
no MAJOR. The fix pass restored the evaluation-order precedence of
refusals (a judgement's naming defect had lost to a later refusal, and
on an intersect to a later empty step, so an `Err` read as `Ok(Empty)`),
pinned through a test seam; made one read of a result key back to its
operand; and pinned the given channel's privacy by compile-fail
doctests. Both delta checks approved; a last pass took their notes
(each doctest fails only for the item it names, the `Judgement` doc,
`pairwise` read once, a drop guard on the seam).

CONTACTHOLD's m2 stays open: a restated tangency still carries the
first of several verdicts' margins, now in `nary.rs`, pinned as it
stands. One reviewer showed it reachable (two plates and a rod at
dy = 1e-11 restate margins 0.0 and 1.0e-11, and the step carries 0.0).
It is CONTACTHOLD's decision, on
`work/contacthold/a-fold-steps-restated-tangency-takes-one-of-several-verdicts-margins.md`.
Style notes not taken: `Fold::step`'s obstacle closure re-derives the
member-lineage rule, the restated-row lookup is by structural equality,
and the records' order across the crates is held by a comment.
