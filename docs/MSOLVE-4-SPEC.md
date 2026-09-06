# MSOLVE-4 — A mate's memo key carries the solve's answer (spec)

**Program:** MSOLVE (`work/msolve/plan.md`), unit `MSOLVE-4`
(`work/msolve/MSOLVE-4.md`). **Finding of record:**
`work/msolve/mate-memo-key-does-not-carry-the-solve.md` (CHROME's
badge-attribution lane, 2026-09-04) — read it in full. **Track:**
kernel change — one style review plus a correctness arm (§Review). No
A/B row. **Dispatches after MSOLVE-1 merges** (both touch the mate arm
of the content key in `eval/mod.rs`).

## The defect, located

A mate node's value IS the solve's answer for it: `wire.rs`'s `Mate`
arm returns `Err(NodeErrorKind::Mate(fault))` when
`env.poses.fault(id)` is `Some`, else `Ok(ValuePayload::Mate(role))`.
The solve is re-run on every evaluation (`eval/mod.rs`,
`solve_document` before the schedule), so the answer is fresh — but the
mate's content key (`eval/mod.rs`, the `Node::Mate` arm of
`content_key`) feeds only the two references, the class and the
alignment. The memo reuses a prior `NodeResult::Ok` whose content and
naming keys match (the reuse rule is the `if let Some(NodeResult::Ok(v))
= prior…` block after the key is computed; failures are never served
from memo). So an unedited mate that was `Ok` last time stays `Ok` when
this evaluation's solve faults it, and a mate whose role changed
(declaring ↔ determining, as edits join or split its pair) keeps the
stale role. The instance node does not have this defect: its key feeds
`op_env.poses.placement(doc, id).ok()`, so a faulted instance (`None`)
keys differently from a placed one and re-runs.

CHROME carries a viewer-side guard for it (`crates/viewer/src/tree.rs`,
the blame corroboration in the row builder near `blamed_mates`): take
the first blamed mate the run agrees is `Failed`, else keep the row's
own failure. The guard exists because the kernel's evaluation is
internally inconsistent; the fix makes it unnecessary.

## What the unit builds

**1. The mate's key feeds the solve's answer.** In `content_key`'s
`Node::Mate` arm, after the references, class and alignment: the role
the solve assigned (`env.poses.role(id)`, one tag per `MateRole`
variant) and whether the solve faulted the mate (one tag). Route it the
way the instance's placement is routed — an argument the caller reads
off `op_env.poses` beside `placement`, not a global — so `content_key`
stays a pure function of what it is handed. The fault's CONTENT does
not feed the key, and the doc at the arm says why: a faulted mate
evaluates to `Err`, and the memo never serves an `Err`, so two
different faults on one mate can never be confused through reuse; the
tag exists for the `Ok → Err` direction only. If you find that
argument false on the tree (a path that memoizes a failed mate), the
content feeds too, through a `feed_mate_fault` over the enum's fields —
never a `Debug` rendering.

**2. The witness, kernel-side.** Rows in a new
`crates/editor-core/tests/msolve4_mate_memo.rs`, through ordinary
doors (`DocEdit::InsertNode`, `evaluate` with a prior):

- author mate A, evaluate; author mate B that contradicts A on the
  same pair (the finding's first shape); evaluate WITH the first
  evaluation as prior: A's result is `Failed` with the
  `Contradictory` fault naming A and B, not `Ok` — and the fault the
  solve records against A (`SolvedPoses::fault(A)`) equals the fault
  A's row carries (the consistency the finding asks for: blame and
  row agree);
- the reverse: a faulted mate whose contradiction is deleted evaluates
  `Ok` on the next evaluation with the faulted one as prior;
- a role change on an unedited mate: a second mate added to a pair
  turns the first from determining to declaring (or the reverse, as
  the tree selects) — the unedited mate's value carries the NEW role
  when evaluated with the old evaluation as prior, and its content key
  differs;
- a mate whose references, class, alignment and solve answer are all
  unchanged is REUSED (`reused == true` on its step, or the memo's own
  witness), so the key is not merely a nonce.

**3. The viewer's guard retired.** With the kernel consistent, the
corroboration in `tree.rs` is dead weight that hides the next
inconsistency: remove it, read the blame directly, and let the viewer
row that motivated it (CHROME's badge-attribution rows in
`crates/viewer/tests/tree_badges.rs`, or wherever the guard's row
lives — find it by the guard's own citation) pass on the kernel's
answer alone. If a viewer row needs the guard to pass after the kernel
change, that row has found a second inconsistency: STOP and report it
rather than keeping the guard.

**4. The docs.** The `Node::Mate` variant's doc and the content-key
arm say what a mate's key is: its recipe payload AND the solve's
answer, because the value is the answer. Present tense.

## Acceptance

- **A1** The finding's two observed shapes (a contradiction added
  around an unedited mate; two mates on one pair, "3 and 4 cannot
  both hold" with 3 reading `Ok`) both evaluate the blamed mates as
  `Failed` with the solve's fault, using the earlier evaluation as
  prior.
- **A2** Role changes on an unedited mate are reflected through the
  memo.
- **A3** An unchanged mate is still reused (no spurious re-run).
- **A4** The viewer guard is gone and CHROME's attribution rows pass
  unchanged.
- **A5** Every existing suite passes unchanged; a document with no
  mates keys bit-for-bit as before (the mate arm is the only arm
  touched).

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted
  CI is the verification of record; poll it in the foreground; never
  end a turn with background work active.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools (no `gh`). Private `CARGO_TARGET_DIR` and scratch outside the
  worktree; `git status` before every `git add`; never `git add -A`.
- Fence: `crates/editor-core/src/eval/mod.rs` (the mate key arm and
  the call site that hands it the solve's answer), the `Node::Mate`
  doc in `node.rs`, `crates/viewer/src/tree.rs` (the guard only),
  tests. Nothing in `mate/*`, `eval/wire.rs`, `eval/memo.rs`'s hasher.
- Do not feed a `Debug` string into any key; do not add a nonce; do
  not change what the memo reuses (only `Ok` priors) — if the fix
  needs that rule to change, STOP and say why.
- The sibling the finding names — the memo not keyed against the
  resolver (`crates/viewer/tests/review_gui4_r1.rs`, "a changed store
  is not re-read") — is a different subject (the part store, DOCM's
  memo-admission ground) and is NOT this unit's; leave its row alone.

## Review

One style review plus a correctness arm, claims verbatim:

- **C1** A1 on a document the implementer did not build; the blame in
  `SolvedPoses::fault` and the row's own result agree for every mate
  the solve names.
- **C2** The key feeds the answer, not a nonce: A3's reuse holds, and
  the "content need not feed" argument is true on the tree (grep for a
  memoized `Err`).
- **C3** The viewer guard is gone and nothing else corroborates the
  blame against the run; CHROME's rows pass on the kernel's answer.
- **C4** Nothing outside the fence moved; no-mate documents key
  bit-for-bit.
