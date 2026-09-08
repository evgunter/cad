---
id: bracket-scope-is-run-op-not-the-node
kind: issue
title: The verdict bracket's scope is run_op, not the node: the profile pre-pass and the mate solve decide before any bracket opens
status: review
pr: 2176
opened: 2026-09-05
refs: [1969]
branch: eval/7-node-bracket
---

## What

The verdict bracket in `eval_node` (`crates/editor-core/src/eval/mod.rs`)
opens around `wire::run_op` and nothing else. Two decision-making
passes run BEFORE it and record on no node:

- the profile pre-pass — `profile_pre` / `lane_program`
  (`crates/editor-core/src/eval/mod.rs:2380` and the lift's second
  pass above it), computed before the memo lookup;
- the whole-document mate solve — `mate::solve_document`
  (`crates/editor-core/src/eval/mod.rs:2058`), run once per
  evaluation before any node.

At top level those decisions land in no frame. Measured (R2, pinned by
`kstats_bracket_rows::the_decisions_outside_every_node_bracket_are_the_pre_pass_ones`):
the one-solid part fixture records **724** verdicts on its nodes and
**75** outside every bracket (`chord_side` 28, `line_span` 8, …); the
two-instance assembly records **0** outside (the part's are shielded on
the cache's miss path). Before PR #1969 the same decisions landed on
whichever instantiate node's frame enclosed the nested run; the PR
shields them (`PartCache::get`) rather than widening the bracket,
because widening moves every Profile node's log and the verdict-log
goldens with it.

## Why it matters

`resolve::vdiff` and the driver's verdict vector see a Profile node's
op decisions but not its pre-pass ones; a flip in the pre-pass is
invisible to both. A mate-solve escalation reaches the driver only as
`NodeErrorKind::Mate` (see
`work/props/escalation-channel-misses-op-minted-indeterminates.md`).

## Home

LIB/SEAT ground (`eval/mod.rs`), so no program: the bracket's scope is
a design choice about what a node's log means, to be made with the
per-node profile pre-pass's owner.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/eval/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). `eval/mod.rs`'s verdict bracket is EVAL's; the decision about what a node's log means is made with DOCM (the profile pre-pass's owner) on an `[ev]` PR.

## Question for Ev (EVAL, with PROPS and M10; announced on DOCM's board, 2026-09-08)

**What is a node's verdict log: the decisions its OP made, or every
decision made on the node's behalf?** Today it is the former by
construction. `eval_node` (`crates/editor-core/src/eval/mod.rs`) resolves
the profile program at the lane scalar (`lane_program`, M10-P's second
pass) BEFORE the content key, because that resolution feeds the key —
and its decisions (`chord_side`, `line_span`, …: 75 on the one-solid
part fixture, against 724 on the nodes) land in whatever frame encloses
the run, since the node's `Bracket` opens only around `wire::run_op`.
The whole-document mate solve (`mate::solve_document`, once per
evaluation before the schedule) is the same shape one level up. Pinned:
`kstats_bracket_rows::the_decisions_outside_every_node_bracket_are_the_pre_pass_ones`
(the counts), and PROPS's
`escalation-channel-misses-op-minted-indeterminates` family 3 (the mate
solve's escalations reach the driver only as `NodeErrorKind::Mate`).

Why it matters: `resolve::vdiff` and the driver's verdict vector read a
Profile node's op decisions and not its pre-pass ones, so a pre-pass
flip is invisible to both — and `VerdictVectorKey`, which certification
gates on (M10's `golden/m10_6_certifying_keys.txt`), hashes a log that
omits decisions the node's value depends on.

Three answers, with a recommendation.

**1. A node's log is every decision made on its behalf; the mate solve
gets the document's frame (RECOMMENDED).** The node's bracket opens at
the top of `eval_node`, before payload and lane resolution, and closes
after `run_op` as now. On a memo hit the pre-pass has already run
(identically, D9) inside the fresh frame; the hit returns the prior
value, whose log already holds those same verdicts, and the fresh
frame is finished and dropped — bit-consistent, and the hit path stays
a hit. The mate solve is a whole-document computation (A11) and its
decisions are the document's: `Evaluation` gains a document-level log
(`verdicts`/`escalations` beside `nodes`), which is the shape PROPS's
item already schedules, and `vdiff` gains a document row. **What
moves, stated:** every Profile node's log grows by its pre-pass
verdicts (the part fixture: 75 move from outside to the node), the R2
pin's outside count goes to zero, every verdict-log golden and
certification key over a document with a profile re-baselines, and
the re-baseline is the point — a certification that cannot see a
pre-pass flip is certifying less than the node computes. Cost beyond
the goldens: one `Bracket::open()` moves ~200 lines up; the
`profile_pre` validation between the two points is inside the frame
too (right: its decisions are the node's).

**2. Two logs per node — the op's and the pre-pass's — stored
separately on `NodeValue`.** Keeps every existing golden byte-stable
and makes the pre-pass visible. Rejected on `memories/
output-stability-as-justification.md`: stability is the only argument
for it, and it doubles the substrate `vdiff`, the verdict vector and
certification all read, for a distinction (before the key / after the
key) that is the evaluator's plumbing, not the node's meaning.

**3. Ratify the fence: a node's log is its op's; the pre-pass and the
mate solve share a document-level frame.** Coherent and cheap (it is
answer 1's mate half applied to the pre-pass too), and it keeps the
per-node goldens. Against it: the pre-pass decisions ARE per node —
they are made from one Profile node's program and nothing else — so
recording them on the document loses attribution the driver wants
("which node's flip") for no reason but where the code happens to
run; and it makes the per-node certification key blind to a class of
flip by ratification rather than by accident.

**What this asks of you:** answer 1 or 3 (2 is recorded so it is not
re-derived). Under 1, EVAL builds the bracket move as a unit with a
correctness arm (the memo-hit consistency claim, the pin flip, the
re-baselines named in the PR), PROPS builds the document frame for the
mate solve on its own item, and M10 is told its certification keys
re-baseline; under 3, EVAL builds the pre-pass half of the document
frame beside PROPS's mate half and closes this row as ratified.

## Ruled (Ev, PR 2138, 2026-09-08)

Answer 1: **a node's verdict log is every decision made on its
behalf; the mate solve gets the document's frame.** This row becomes
the unit for the node half: `docs/EVAL-7-SPEC.md`, branch
`eval/7-node-bracket`, correctness arm on (every Profile node's log,
the R2 pin, the certification keys and accounting goldens re-baseline
and the PR names each). The document-level frame for
`mate::solve_document` is PROPS's build on its escalation item, family
3, announced there; M10 is told its certification keys move.

## Closed

Built as EVAL-7 (`docs/EVAL-7-SPEC.md`). `eval_node`'s bracket opens
at the top of the function and closes after `run_op`, so a Profile
node's log opens with its pre-pass — the plane's axes, the program's
replay, the f64 validation — ahead of its op's decisions; a memo hit
finishes and drops the fresh frame and returns the prior value, whose
log already opens with the same decisions (asserted at the hit site,
driven by `kstats_bracket_rows` at f64 and Interval); a pre-key
refusal carries its frame's escalations. On the one-solid part: 799 on
the nodes (frame 2, profile 144 = 75 + 69, extrude 653), 0 outside;
the assembly half unchanged (466 per instance). Every moved value is a
Profile node's own log or a document key over one — 126 node rows and
48 document keys over the corpus, and the two M10-6 certification keys
— listed in the PR body; no non-Profile log moved. The mate solve's
document frame is PROPS's build (its escalation item, family 3), not
this row's. Residue filed beside this row:
`profile-node-log-holds-the-f64-validation-twice-under-the-pinned-lift`
(the doubled validation the log now shows) and
`interval-content-key-hashes-bits-the-pre-pass-does-not-read` (the
review's finding: at Interval the key does not hold the nominal the
precompute reads, so the hit site's prefix identity is an f64
argument).
