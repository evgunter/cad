# EVAL-7 — a node's verdict log is every decision made on its behalf: the bracket opens at the top of `eval_node` (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 7 — the build of the ruling on
PR 2138). **Item:** `work/eval/bracket-scope-is-run-op-not-the-node.md`.
**Track:** E with a **correctness arm**: the unit changes what every
Profile node's log holds and therefore every verdict-log golden and
certification key over a document with a profile; one style review AND one
correctness review, fix pass, record-at-merge. No A/B draw.
**Branch:** `eval/7-node-bracket`. **Difficulty:** M (small diff, wide
re-baseline; the argument is the deliverable).

## The ruling (Ev, PR 2138, 2026-09-08: "sure 1 seems fine")

**A node's log is every decision made on its behalf; the whole-document
mate solve gets the document's frame.** This unit is the node half. The
document frame for `mate::solve_document` is PROPS's build, on
`work/props/escalation-channel-misses-op-minted-indeterminates.md`
family 3 (announced there by the orchestrator); nothing in this unit
touches the mate solve, and the R2 pin's assembly half keeps asserting
what it asserts today.

## What lands

1. **The bracket moves up.** `geom_core::k_stats::Bracket::open()` in
   `eval_node` (`crates/editor-core/src/eval/mod.rs`, today at `:2642`
   immediately before `wire::run_op`) opens at the top of the function,
   before the first thing that can decide — the poison walk decides
   nothing, but the slot evaluation, `resolved_program`, `profile_pre`
   (the profile's validation) and `lane_program` (M10-P's second pass)
   all can — and closes where it closes now, after `run_op`. One frame
   per node, exactly as now; only its opening line moves. The comment at
   the site is rewritten to say what the frame holds: every decision
   made evaluating THIS node, pre-key and op alike, with the nesting
   argument for instantiated parts unchanged.
2. **The memo hit returns the prior's log and drops the fresh frame.** On
   a content-and-naming hit the pre-pass has already run inside the
   fresh frame; the hit path calls `bracket.finish()` and discards the
   result, returning the prior value whose `verdicts`/`escalations`
   already hold the identical decisions (D9: the pre-pass is a pure
   function of `(program, environment)`, so the fresh frame's verdicts
   equal the prior's pre-pass verdicts bit for bit). **That equality is
   a CLAIM the correctness arm falsifies** (below), not an assumption:
   a row evaluates a document twice with `prior`, asserts the second
   run's Profile node is `reused`, and asserts the dropped frame's
   verdicts equal the prefix of the reused value's log that the
   pre-pass wrote. Do not thread the fresh frame's verdicts into the
   reused value — the reused value IS the record.
3. **A pre-op failure now carries its frame.** Today `fail(..)` builds a
   `NodeError` with empty `escalations` because "no bracket was open".
   With the bracket open, a failure between the opening and `run_op`
   (an `Expr` refusal, a profile validation refusal) finishes the frame
   and carries `recorded.escalations` on the error, exactly as an op
   failure does. `kstats_bracket_rows::a_pre_op_failure_has_empty_escalations`
   is re-read, not deleted: the D4 process-ε door it exercises refuses
   before any decision is made, so its assertion still holds; the row's
   doc says why (no decision, not no frame), and a SECOND row asserts
   the other case — a pre-pass that decides and then fails carries those
   escalations.
4. **The pin flips.**
   `kstats_bracket_rows::the_decisions_outside_every_node_bracket_are_the_pre_pass_ones`
   becomes the row that says the part decides NOTHING outside its nodes'
   brackets (75 → 0 outside; 724 → 799 on the nodes, with the Profile
   node's log carrying the 75 — assert the per-node number, not just the
   sum) and keeps its assembly half. Rename it for what it now asserts.
5. **The goldens re-baseline, and the PR says what moved.** Every
   committed record hashed from a Profile node's log moves:
   `crates/editor-core/tests/golden/m10_6_certifying_keys.txt` and the
   `m10_6_accounting_*.txt` rows (re-bless with `M10_6_BLESS_ACCOUNTING=1`
   per `m10_6_ci_rows_interval.rs:647`, inspect the diff, commit it WITH
   the change), plus any verdict-count or `VerdictVectorKey` pin in the
   tests that grep `verdicts` (the sweep). The PR body lists each moved
   value with its before/after and the one-line reason (the Profile
   node's pre-pass verdicts are now in its log). A moved certification
   key here is the point of the ruling, not a cost weighed against it
   (`memories/output-stability-as-justification.md`); a moved key on a
   node that is NOT a Profile node and has no Profile upstream is a
   MAJOR — nothing else may move.
6. **`NodeValue::verdicts`'s doc** states the new invariant in one
   sentence: the log holds every decision made evaluating the node,
   including those made before its content key, in the order made.
   `resolve::vdiff` and `drive` need no change; they read the substrate.

## Correctness claims (the correctness arm)

1. **Hit-path consistency** (item 2): the dropped frame's verdicts equal
   the reused log's pre-pass prefix, on the part fixture, at every
   lane/eps point.
2. **Nothing but Profile-bearing logs move**: the reviewer diffs every
   verdict-log golden and certification key at base and head and
   confirms each moved row has a Profile node in its dependency cone.
3. **The count identity**: on the part fixture, verdicts on nodes at head
   = verdicts on nodes at base + verdicts outside at base (799 = 724 +
   75), and outside at head = 0.
4. **Order**: within the Profile node's log the pre-pass verdicts
   PRECEDE the op's (the bracket is one frame; the order is the order
   made), pinned by a row that reads the first pre-pass predicate name
   and the first op predicate name.
5. **Determinism across `parallel`**: the existing cancel/re-run rows in
   `kstats_bracket_rows.rs` stay green with the wider frame under both
   scheduler modes.

## Sweep

`rg -n 'verdicts\.len\(\)|VerdictVectorKey|certifying=' crates/editor-core/tests crates/pncad-py/tests` —
every pinned count or key is a candidate to move; hit list with
before/after or "unchanged, no Profile in cone" per hit. Blind spot: a
count pinned through a helper whose name does not say `verdicts`.

## Review

Style lane per `docs/prompts/reviewer-style-lane.md`; correctness lane
takes claims 1–5 as MAJOR-class. Emphasis: **a re-baseline is not a
verification** — the reviewer reproduces at least one moved certification
key independently from the new log rather than accepting the blessed
file; and Q2 on the rewritten bracket comment (it must not argue at
length for a two-line move).

## Records at merge

`work/eval/log.md` entry with the PROPS and M10 announcements; the item
`closed` with `pr:`; this spec deleted per `docs/DOC-LEDGER.md`.
