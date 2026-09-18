---
id: the-unreachable-eighth-combination-lost-its-only-assertion
kind: issue
title: #2055 deleted the only executable statement about !busy && running and moved the case into an unasserted branch of outstanding()
status: closed
opened: 2026-09-06
closed: 2026-09-06
pr: 2055
branch: view/progress
refs: [evalservice-coalescing-rule-is-prose-no-implementor-is-held-to]
---



PR #2055 says the eighth combination *"stops being expressible rather
than staying documented"* (PR body, `work/view/log.md`, and the item's
Closed section all put it that way). It stops being expressible at
`frame::progress`'s signature. It does not stop existing: it moved one
level down into `DocSession::outstanding`, where it is answered by the
first arm of an if/else and is now asserted by nothing.

    crates/viewer/src/session.rs:740-748 (`outstanding()` at 740)
    pub fn outstanding(&self) -> Outstanding {
        if !self.busy() { Outstanding::Current }
        else if self.running() { Outstanding::Evaluating }
        else { Outstanding::Canceled }
    }

`!busy() && running()` reaches the first arm and returns `Current`.
The only place that answer is written down is the doc comment three
lines above it (`crates/viewer/src/session.rs:735-739`).

What the diff deleted was the tree's only executable statement about
that case — `crates/viewer/tests/frame_policy.rs`, removed at
`1606-1615` of the old file:

    for indexing in [false, true] {
        assert_eq!(frame::progress(false, true, indexing),
                   frame::progress(false, false, indexing),
                   "with the picture current, a running evaluation the session
                    cannot report changes nothing");
    }

So the net movement on this specific point is from *documented and
asserted at two points* to *documented only*. That is a coverage
regression disclosed as the opposite, which is the part worth
recording; the mapping is total either way, so nothing is unsound.

## The unreachability claim itself

Checked and it holds for both shipped seams, for a reason neither the
comment nor the PR gives: `request_eval` bumps `self.generation` on
**every** submit (`crates/viewer/src/session.rs:1818`), including
`Reevaluate`'s, so `!busy()` means the newest submitted generation has
landed; and `EvalService::poll` hands a result up only when nothing is
queued behind it (`InlineEvaluator` at `evalseam.rs:344-353`,
`ThreadEvaluator` at `evalseam.rs:693-716`), so a landed newest
generation implies an idle seam. The comment's own reason — *"a seam
with work outstanding always has a generation the picture has not
caught up to"* — is a restatement of the conclusion rather than the
argument, and the argument is the two mechanisms above.

## And it rests on a seam contract nothing enforces

`DocSession` holds `Box<dyn EvalService>` (`session.rs:176`), so the
claim is a claim about every implementor, not about the two shipped
ones. The coalescing rule it depends on lives in `evalseam`'s module
prose (*"at most one request is ever outstanding, and a submit while
one is outstanding REPLACES the waiting request"*,
`crates/viewer/src/evalseam.rs:34-38`) — and one in-tree implementor
already departs from it: `HeldEvaluator`
(`crates/viewer/tests/review_gui2_r2.rs:1272-1288`) queues submits
instead of replacing them and makes `cancel` a no-op. Nothing in the
trait, and no test, holds an implementor to the rule the doc comment
on `outstanding()` now leans on.

## Closed (2026-09-06)

The finding is accepted in full: the claim was false, and the diff
that made it deleted the tree's only executable statement about the
case. #2055's fix pass took all three parts.

**The assertion is restored, one level down where the case now lives.**
`a_current_picture_reads_current_even_when_the_seam_claims_work`
(`crates/viewer/tests/eval_seam.rs`) hands `DocSession::new` a
`NeverIdle` seam — an `InlineEvaluator` whose `busy()` is always true —
lands the first result, and asserts `!busy() && running()` really holds
and that `outstanding()` answers `Current`. It is a stronger row than
the one that was deleted: the old one asserted at `frame::progress`,
which no longer takes the pair, and this asserts at the function that
now decides.

**The claim is struck.** "Stops being expressible rather than staying
documented" is gone from the PR body, the log entry and the item's
Closed section. What is true is narrower and is what those now say: the
combination stops being expressible at `frame::progress`'s signature,
moves into `outstanding()`'s first arm, and is asserted there.

**The reason is rewritten to the actual argument**, both mechanisms as
this item states them — `request_eval` bumping the generation on every
submit, and both shipped seams handing a result up only with nothing
queued behind it — and the doc comment now says out loud that the
second is a property of the two implementations and not of
`EvalService`.

That last point is the residue and it has its own file:
`work/view/evalservice-coalescing-rule-is-prose-no-implementor-is-held-to.md`,
carrying the `HeldEvaluator` departure and the missing conformance row.
