---
id: a-review-lane-runs-another-lanes-binary
kind: issue
title: Two review lanes sharing one CARGO_TARGET_DIR run each other's test binaries, and the review dispatch instructions do not forbid it
status: open
opened: 2026-10-18
priority: P2
cost: E
refs: [5314]
---

Found at the holdout pair on INTENT PR #5314 (row
`docs/dual-review-log/rows/PR-5314.md`, the method column).

**What happened.** The two reviewers worked in separate worktrees but
built into one target directory, `/home/user/rv-pair-target`. I chose
that at dispatch to save disk. Reviewer R2 found that one of its runs
had executed a test binary built from R1's tree. Cargo names the
integration-test binary `all-<hash>`, as in `all-439b570e61d63b2b`,
and the hash comes from the crate's metadata, not from the worktree
path or the source. So both worktrees write to the same file, and
whichever lane built last decides what the other one runs. In #5314
the binary R2 ran asserted main's corpus digest, not the one in R2's
own tree. R2 noticed, rebuilt, and kept only results from its own
build. A reviewer who doesn't notice reports another lane's results as
its own. In a pair, that is also a breach of lane isolation (protocol
rule 3) that neither reviewer can see.

**It has happened before.** The TCOST-K1 dual (`docs/MODEL-AB-LOG.md`,
the K1 row, 2026-09-03) recorded "DISPATCHER FAULT: the two review
lanes shared one CARGO_TARGET_DIR". R1 executed R2's mutated binary
once, and R2's mutant matrix was flagged as possibly contaminated.

**Where the rule lives, and where it does not.**

- **Where it lives.** The rule is written for implementers and for
  orchestrators reading memory:
  - `docs/prompts/implementer-discipline.md:70-74`: "one
    `CARGO_TARGET_DIR` per worktree, never one shared with another
    lane";
  - `memories/agent-lane-operations.md:26-31`, which gives the
    `all-<hash>` mechanism.
- **Where it does not.** A review lane is briefed from three places,
  and none of them says it:
  - `docs/DUAL-REVIEW-PROTOCOL.md`: rule 2, "The pair", sets the
    concurrency, the frozen head and identical briefs, and rule 3
    covers lane isolation on the read side only;
  - `docs/REVIEW-STYLE-DISPATCH.md`;
  - `docs/prompts/reviewer-style-lane.md`.
- **The result.** A dispatcher short of disk is free to share a
  target. The only text against it is an implementer rule the
  reviewer is never pointed at.

**The work: the dispatch instructions name a private target for each
review lane.**

1. **Add a build-side isolation clause to the protocol.** It would go
   beside rule 3 in `docs/DUAL-REVIEW-PROTOCOL.md`. Each reviewer gets
   its own `CARGO_TARGET_DIR`, outside its worktree, and the brief
   names it. The row records the two paths, so "identical briefs
   modulo lane label, `CARGO_TARGET_DIR` and scratch path" is
   something a reader can check.
2. **Say what to do when disk cannot hold two targets.** Sharing is
   not the answer. The protocol already has a fallback for a pair
   that cannot run concurrently: the late trigger in rule 2, where
   the second review runs on the same frozen head afterwards. Use that
   fallback, or wait for disk. Then the pair's method is still the
   same for both reviewers, which rule 4 (fair pair) needs.
3. **Point every review brief at the rule.** Either point it at the
   protocol's clause by path, or put the one line in
   `docs/REVIEW-STYLE-DISPATCH.md`. A single, sequential review lane
   building next to the implementer's lane has the same hazard, so the
   line belongs to every review dispatch, not only the pair's.

Editing `docs/DUAL-REVIEW-PROTOCOL.md` is Ev's call (its own header:
"Changing the protocol is Ev's call"). The same goes for any
`docs/prompts/` or `memories/` line. So this item asks rather than
edits.

